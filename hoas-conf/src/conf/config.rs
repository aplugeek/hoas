use serde::{de, Deserialize, Serialize};

#[derive(Serialize, PartialEq, Deserialize, Debug, Clone)]
pub enum Operator {
    EQ,
    NE,
    LIKE,
    GT,
    LT,
    GTE,
    LTE,
}

#[derive(Serialize, PartialEq, Deserialize, Debug, Clone)]
pub enum QueryParamKind {
    Number,
    String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ApiConf {
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    #[serde(rename = "metadata")]
    pub meta_data: Metadata,
    #[serde(rename = "spec")]
    pub spc: ApiSepc,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ApiSepc {
    #[serde(rename = "rules")]
    pub rules: Vec<Route>,
    #[serde(rename = "datasource")]
    pub datasource: Datasource,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Datasource {
    #[serde(rename = "addr")]
    pub addr: String,
    #[serde(rename = "dbname")]
    pub dbname: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Route {
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "method")]
    pub method: String,
    #[serde(rename = "collection")]
    pub collection: String,
    #[serde(rename = "filters")]
    pub filters: Vec<QueryFilter>,
    #[serde(rename = "pagination")]
    pub pagination: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct QueryFilter {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "type", deserialize_with = "de_param_kind")]
    pub value_type: QueryParamKind,
    #[serde(rename = "operator", deserialize_with = "de_operator")]
    pub operator: Operator,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(rename = "name")]
    pub name: String,
}

fn de_operator<'de, D>(deserializer: D) -> Result<Operator, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?.to_lowercase();
    let op = match s.as_str() {
        "eq" => Operator::EQ,
        "ne" => Operator::NE,
        "like" => Operator::LIKE,
        "gt" => Operator::GT,
        "lt" => Operator::LT,
        "lte" => Operator::LTE,
        "gte" => Operator::GTE,
        other => {
            return Err(de::Error::custom(format!("Invalid state '{}'", other)));
        }
    };
    Ok(op)
}

fn de_param_kind<'de, D>(deserializer: D) -> Result<QueryParamKind, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?.to_lowercase();
    let op = match s.as_str() {
        "string" => QueryParamKind::String,
        "number" => QueryParamKind::Number,
        other => {
            return Err(de::Error::custom(format!(
                "Invalid query param kind '{}'",
                other
            )));
        }
    };
    Ok(op)
}
