use crate::ctx::Context;
use crate::io::parse_num;
use crate::middleware::X_REQUEST_ID;
use actix_web::dev::ServiceResponse;
use actix_web::{Error, HttpRequest, HttpResponse};
use futures::StreamExt;
use hoas_conf::conf::Route;
use hoas_conf::conf::{Operator, QueryParamKind};
use http::StatusCode;
use mongodb::{
    bson::{doc, Bson, Document},
    options::FindOptions,
    Cursor,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Default)]
struct OpenResponse<T> {
    code: u16,
    data: Vec<T>,
}

impl<T> OpenResponse<T> {
    pub fn new(code: u16, data: Vec<T>) -> Self {
        OpenResponse { code, data }
    }
}

#[derive(Serialize, Deserialize, Default)]
struct ErrorResponse<'a> {
    code: u16,
    message: &'a str,
}

impl<'a> ErrorResponse<'a> {
    pub fn new(code: u16, message: &'a str) -> Self {
        ErrorResponse { code, message }
    }
}

const DEFAULT_PAGE_SIZE: i32 = 25;
const DEFAULT_PAGE: i32 = 1;

pub async fn handle(
    ctx: &'static Context,
    req: HttpRequest,
    route: &Route,
) -> Result<HttpResponse, Error> {
    let mut filter = doc! {};
    let params = req
        .query_string()
        .split("&")
        .filter(|e| e.len() > 0)
        .collect::<Vec<&str>>();
    let mut query_maps = HashMap::new();
    params.iter().for_each(|p| {
        let pair = p.split("=").filter(|e| e.len() > 0).collect::<Vec<&str>>();
        if pair.len() > 0 {
            query_maps.insert(pair[0], pair[1]);
        }
    });
    route.filters.iter().for_each(|q| {
        if let Some(pv) = query_maps.get(q.name.as_str()) {
            mongo_syntax!(q, filter, pv)
        }
    });
    let mut opt = FindOptions::default();
    if route.pagination {
        let (page, page_size) = build_paging(&query_maps);
        debug!("[page]:{},[page_size]:{}", page, page_size);
        opt.skip = Some(((page - 1) * page_size) as u64);
        opt.limit = Some(page_size as i64);
    }
    debug!(
        "Request id:{:?},filter is :{:?}",
        req.headers().get(X_REQUEST_ID).unwrap(),
        filter
    );
    let fetched = ctx
        .mongo
        .collection(route.collection.as_str())
        .find(filter, opt)
        .await;
    Ok(get_respond(fetched).await)
}

async fn get_respond(fetched: mongodb::error::Result<Cursor<Document>>) -> HttpResponse {
    match fetched {
        Ok(mut cursor) => {
            let mut documents = vec![];
            while let Some(result) = cursor.next().await {
                match result {
                    Ok(document) => {
                        documents.push(process_doc(document));
                    }
                    Err(e) => {
                        error!("fetch data cursor error:{:?}", e);
                        return error_response(
                            StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                            "fetch data cursor error",
                        );
                    }
                };
            }
            HttpResponse::Ok().json(OpenResponse::new(StatusCode::OK.as_u16(), documents))
        }
        Err(e) => {
            error!("fetch data error:{:?}", e);
            error_response(
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                "fetch data error",
            )
        }
    }
}

fn process_doc(mut doc: Document) -> Document {
    let mut keys = vec![];
    for x in doc.keys() {
        keys.push(x.clone());
    }
    for x in keys {
        if let Some(bson) = doc.get_mut(x.as_str()) {
            match bson {
                Bson::Double(v) => *bson = Bson::Int64(*v as i64),
                Bson::DateTime(t) => *bson = Bson::Int64(t.timestamp_millis()),
                Bson::ObjectId(object_id) => *bson = Bson::String(object_id.to_string()),
                _ => {}
            }
        }
    }
    doc
}

fn build_paging(query_maps: &HashMap<&str, &str>) -> (i32, i32) {
    let mut page_size = DEFAULT_PAGE_SIZE;
    let mut page = DEFAULT_PAGE;
    if let Some(v) = query_maps.get("per_page") {
        if let Ok(size) = v.parse::<i32>() {
            page_size = size;
        }
    }

    if let Some(v) = query_maps.get("page") {
        if let Ok(page_num) = v.parse::<i32>() {
            page = page_num;
        }
    }

    (page, page_size)
}

fn error_response(code: u16, message: &str) -> HttpResponse {
    HttpResponse::InternalServerError().json(ErrorResponse::new(code, message))
}
