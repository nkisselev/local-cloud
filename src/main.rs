use std::collections::HashMap;
use std::io::Read;
use std::process::{Command, Stdio};
use actix_web::{get, HttpResponse, patch, post};
use bollard::container::ListContainersOptions;
use bollard::Docker;
use env_logger::Builder;
use log::LevelFilter;
use crate::services::config::{CloudSettings, get_settings};
use crate::services::docker::run;

pub mod models;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    Builder::new()
        .filter_level(LevelFilter::Info)
        .target(env_logger::Target::Stdout)
        .init();

    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .service(get_services)
            .service(get_cloud_projects)
            .service(get_cloud_republish_project)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[get("/cloud/projects")]
async fn get_cloud_projects() -> impl actix_web::Responder {
    let config = get_settings();

    HttpResponse::Ok().content_type("application/json").body(serde_json::to_string(&config).unwrap())
}

#[get("/cloud/projects/{name}/republish")]
async fn get_cloud_republish_project(path: actix_web::web::Path<(String)>) -> impl actix_web::Responder {
    let (name) = path.into_inner();
    let config = get_settings().unwrap();
    let project = config.projects.iter().find(|p| p.name == name).unwrap();
    // // docker build --tag baseimg --file Dockerfile .
    let buildCommand = format!("docker build --tag {} --file {} .", project.docker_service_name, project.docker_file);
    let mut child = Command::new("docker")
        .arg("build")
        .arg("--tag")
        .arg(&project.docker_service_name)
        .arg("--file")
        .arg(&project.docker_file)
        .arg(&project.home)
        .stdout(Stdio::piped())
        .output().unwrap();
    run(&project.docker_service_name, &project.docker_service_name).await;
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

#[get("/docker/containers")]
async fn get_services() -> impl actix_web::Responder {
    HttpResponse::Ok().json("ok")
}

// build project

/*

*/