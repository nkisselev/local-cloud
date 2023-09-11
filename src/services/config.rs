use std::io::Read;
use serde::{Deserialize, Serialize};
use crate::models::result_code::ResultCode;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub docker_file: String,
    pub docker_container_name: String,
    pub docker_image_name: String,
    pub source_home: String,
}

// Can be supported in the future
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct NginxSettings {
    pub config_file: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudSettings {
    pub projects: Vec<Project>,
}

pub fn get_settings() -> Result<CloudSettings, ResultCode> {
    let path = std::env::var("LOCAL_CLOUD_CFG");
    if path.is_err() {
        return Err(ResultCode::ConfigFilePathIsNotSet);
    }
    let path = path.unwrap();
    let file = std::fs::File::open(path);
    if file.is_err() {
        return Err(ResultCode::ReadConfigFileError);
    }
    let file = file.unwrap();
    let mut buf_reader = std::io::BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let s: CloudSettings = serde_yaml::from_str(&contents).unwrap();
    return Ok(s);
}