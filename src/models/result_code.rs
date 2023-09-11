use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResultCode{
    Ok,
    Error,
    ConfigFilePathIsNotSet,
    ReadConfigFileError,

}