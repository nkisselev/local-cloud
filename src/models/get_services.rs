use serde::{Deserialize, Serialize};
use crate::services::config::Project;

pub struct GetProjectsListRs {
    pub projects: Vec<ProjectDto>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectDto {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "dockerFile")]
    pub docker_file: String,
    #[serde(rename = "dockerServiceName")]
    pub docker_service_name: String,
    #[serde(rename = "home")]
    pub home: String,
}