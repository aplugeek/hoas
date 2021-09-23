use crate::conf::ApiConf;
use std::fs::File;
use std::io::{BufReader, Result};

pub fn parse_rules(path: &str) -> Result<ApiConf> {
    let reader = BufReader::new(File::open(path)?);
    let api_config =
        serde_yaml::from_reader::<BufReader<File>, ApiConf>(reader).expect("parse api conf error");
    Ok(api_config)
}
