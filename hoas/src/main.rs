#[macro_use]
extern crate log;

use crate::app::App;

use crate::ctx::Context;
use crate::io::handle;
use crate::middleware::{with_print, with_trace};
use actix_web::dev::{Service, ServiceResponse};

use actix_web::{web, App as ActixApp, HttpRequest, HttpServer};
use env_logger::Env;
use futures::future::ok;
use futures::FutureExt;
use structopt::StructOpt;

mod ctx {
    pub mod ctx;

    pub use ctx::*;
}

#[macro_use]
mod middleware;
mod app;
mod io;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app = App::from_args();
    let App {
        ref level, port, ..
    } = app;
    env_logger::from_env(Env::default().default_filter_or(level)).init();
    let ctx = Context::init_ctx(app).await?;
    let sctx: &'static Context = unsafe { std::mem::transmute(&ctx) };
    HttpServer::new(move || {
        let mut app = ActixApp::new().wrap_fn(|mut req, svc| {
            // actix-web framework so weird, fix me
            let sreq = &mut req;
            let result = middleware!(sreq, with_trace, with_print);
            if let Err(e) = result {
                let (r, _) = req.into_parts();
                return ok(ServiceResponse::from_err(e, r)).boxed_local();
            }
            svc.call(req)
        });

        for (p, r) in sctx.route_map.0.iter() {
            app = app.route(
                p.as_str(),
                web::get().to(move |req: HttpRequest| handle(sctx, req, r)),
            );
        }
        app
    })
    .bind(format!("0.0.0.0:{:?}", port))?
    .run()
    .await
}
