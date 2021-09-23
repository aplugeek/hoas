use crate::app::App;
use hoas_conf::conf::parse_rules;
use hoas_route::RouteMap;
use mongodb::{options::ClientOptions, Client, Database};

use std::io::Result;
use std::sync::Arc;
use std::time::Duration;

pub struct Context {
    pub route_map: Arc<RouteMap>,
    pub mongo: Database,
}

impl Context {
    pub async fn init_ctx(app: App) -> Result<Self> {
        let conf = parse_rules(app.parse_path.as_str())?;
        let mut client_options = ClientOptions::parse(conf.spc.datasource.addr.as_str())
            .await
            .expect("mongo client options parse error");
        client_options.connect_timeout = Some(Duration::from_secs(10));
        client_options.heartbeat_freq = Some(Duration::from_secs(10));
        let client = Client::with_options(client_options).expect("init mongo client error");
        let db = conf.spc.datasource.dbname.clone();
        Ok(Context {
            route_map: Arc::new(RouteMap::from_conf(&conf.spc.rules)),
            mongo: client.database(db.as_str()),
        })
    }
}
