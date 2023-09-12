use std::io::Read;
use std::process::{Command, Stdio};
use actix_cors::Cors;
use actix_web::{get, HttpResponse, patch, post};
use env_logger::Builder;
use log::LevelFilter;
use crate::models::get_services::{GetProjectsListRs, ProjectDto};
use crate::services::config::{CloudSettings, get_settings, Project};
use crate::services::docker::run;

pub mod models;
pub mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    /*let settings = CloudSettings{
        projects: vec![
            Project{
                name: "ticker".to_string(),
                docker_file: "Dockerfile".to_string(),
                docker_container_name: Some("ticker".to_string()),
                docker_image_name: "ticker".to_string(),
                source_home: "/Users/linitpro/repos/Ticker/Ticker/bin/Debug/net7.0".to_string(),
                ports: Some(vec![(8080, 8080)].into_iter().collect()),
            }
        ],
    };
    let yaml = serde_yaml::to_string(&settings).unwrap();
     */
    Builder::new()
        .filter_level(LevelFilter::Info)
        .target(env_logger::Target::Stdout)
        .init();

    actix_web::HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin().allow_any_method().allow_any_header()
            .max_age(36000);
        actix_web::App::new().wrap(cors)
            .service(get_cloud_projects)
            .service(get_cloud_republish_project)
    })
        .bind(("127.0.0.1", 8088))?
        .run()
        .await
}

#[get("/cloud/projects")]
async fn get_cloud_projects() -> impl actix_web::Responder {
    let config = get_settings();
    let response = GetProjectsListRs {
        projects: config.unwrap().projects.iter().map(|p| ProjectDto{
            name: p.name.clone(),
            docker_file: p.docker_file.clone(),
            docker_container_name: p.docker_container_name.clone(),
            docker_image_name: p.docker_image_name.clone(),
            source_home: p.source_home.clone(),
        }).collect()
    };
    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&response).unwrap())
}

#[get("/cloud/projects/{name}/republish")]
async fn get_cloud_republish_project(path: actix_web::web::Path<(String)>) -> impl actix_web::Responder {
    let (name) = path.into_inner();
    let config = get_settings().unwrap();
    let project = config.projects.iter().find(|p| p.name == name).unwrap();
    // // docker build --tag baseimg --file Dockerfile .
    let mut child = Command::new("docker")
        .arg("build")
        .arg("--tag")
        .arg(&project.docker_image_name)
        .arg("--file")
        .arg(&project.docker_file)
        .arg(&project.source_home)
        .stdout(Stdio::piped())
        .output().unwrap();
    if project.docker_container_name != None {
        let containerName = project.docker_container_name.as_ref().unwrap();
        run(&project).await;
    }

    let result = format!("status: {}\nout: {}\nerr: {}",
                         child.status,
                         String::from_utf8_lossy(&child.stdout),
                         String::from_utf8_lossy(&child.stderr));
    // build project
    // run project
    HttpResponse::Ok()
        .content_type("application/json")
        .body(result)
}


#[post("/docker/containers")]
async fn add_project() -> impl actix_web::Responder {
    HttpResponse::Ok().json("ok")
}

#[patch("/docker/containers/{id}/rebuild")]
async fn edit_project() -> impl actix_web::Responder {
    // The usage is similar as with the standard library's `Command` type
    let mut child = Command::new("/Users/linitpro/repos/Ticker/Ticker/bin/Debug/net7.0/Ticker")
        .spawn()
        .expect("failed to spawn");
    let status = child.wait();
    println!("the command exited with: {}", status.unwrap());
    HttpResponse::Ok().json("ok")
}
