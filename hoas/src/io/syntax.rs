use crate::middleware::CustomResponseError;
use actix_web::http::StatusCode;
use hoas_conf::conf::{Operator, QueryFilter, QueryParamKind};
use mongodb::bson::{doc, Document};

pub fn mongo_syntax(
    q: &QueryFilter,
    filter: &mut Document,
    pv: &str,
) -> Result<(), CustomResponseError> {
    match q.operator {
        Operator::EQ => match q.value_type {
            QueryParamKind::Number => {
                let v = parse_num(pv)?;
                filter.insert(q.name.as_str(), v);
                Ok(())
            }
            _ => {
                filter.insert(q.name.as_str(), pv);
                Ok(())
            }
        },
        Operator::NE => {
            let ne = match q.value_type {
                QueryParamKind::Number => {
                    let v = parse_num(pv)?;
                    let doc = doc! {
                        q.name.as_str(): { "$ne": v }
                    };
                    doc
                }
                _ => {
                    let doc = doc! {
                        q.name.as_str(): { "$ne": pv }
                    };
                    doc
                }
            };
            filter.extend(ne);
            Ok(())
        }
        Operator::LIKE => {
            let pt = doc! {q.name.as_str():mongodb::bson::Regex{
                pattern:format!(".*{:?}.*",pv),
                options:String::from("i")
            }};
            filter.extend(pt);
            Ok(())
        }
        Operator::GT => {
            let v = parse_num(pv)?;
            let gt = doc! {
                q.name.as_str(): { "$gt": v }
            };
            filter.extend(gt);
            Ok(())
        }
        Operator::LT => {
            let v = parse_num(pv)?;
            let gt = doc! {
                q.name.as_str(): { "$lt": v }
            };
            filter.extend(gt);
            Ok(())
        }
        Operator::GTE => {
            let v = parse_num(pv)?;
            let gte = doc! {
                q.name.as_str(): { "$gte": v }
            };
            filter.extend(gte);
            Ok(())
        }
        Operator::LTE => {
            let v = parse_num(pv)?;
            let lte = doc! {
                q.name.as_str(): { "$lte": v }
            };
            filter.extend(lte);
            Ok(())
        }
        Operator::IN => {
            match q.value_type {
                QueryParamKind::ArrayNumber => {
                    //  let v = $pv.split(",").map(|x| parse_num(x)).collect::<Vec<i64>>();
                    let mut v = vec![];
                    for e in pv.split(",") {
                        v.push(parse_num(e)?);
                    }
                    let include = doc! {
                       q.name.as_str(): { "$in": v }
                    };
                    filter.extend(include);
                    Ok(())
                }
                QueryParamKind::ArrayStr => {
                    let v = pv.split(",").collect::<Vec<&str>>();
                    let include = doc! {
                       q.name.as_str(): { "$in": v }
                    };
                    filter.extend(include);
                    Ok(())
                }
                _ => Err(CustomResponseError::from(
                    StatusCode::BAD_REQUEST,
                    "parse data error",
                )),
            }
        }
    }
}

pub fn parse_num(s: &str) -> Result<i64, CustomResponseError> {
    s.parse::<i64>()
        .map_err(|_| CustomResponseError::from(StatusCode::BAD_REQUEST, "parse data error"))
}
