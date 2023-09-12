use serde::{Deserialize, Serialize};
use crate::services::config::Project;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GetProjectsListRs {
    pub projects: Vec<ProjectDto>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectDto {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "dockerFile")]
    pub docker_file: String,
    #[serde(rename = "dockerContainerName")]
    pub docker_container_name: Option<String>,
    #[serde(rename = "dockerImageName")]
    pub docker_image_name: String,
    #[serde(rename = "sourceHome")]
    pub source_home: String,
}