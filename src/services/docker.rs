use std::collections::HashMap;
use std::process::{Command, Stdio};
use bollard::container::{Config, CreateContainerOptions, ListContainersOptions};
use bollard::Docker;
use tokio_postgres::types::IsNull::No;
use crate::services::config::Project;

pub fn build() {
    println!("build docker.rs");
}

pub async fn run_nginx() {

}

pub async fn run_web() {

}

pub async fn run(project : &Project) {
    let containerName = &project.docker_container_name;
    let imageName = &project.docker_image_name;
    let  mut ports: Vec<String> = Vec::new();

    if project.ports.is_some() {
        for port in project.ports.as_ref().unwrap() {
            ports.push("-p".to_string());
            ports.push(format!("{}:{}", port.0, port.1));
        }
    }

    let mut child = Command::new("docker")
        .arg("run")
        .arg("-d")
        .args(ports)
        .arg("--name")
        .arg(containerName.as_ref().unwrap())
        .arg(&imageName)
        .stdout(Stdio::piped())
        .output().unwrap();

    let result = format!("status: {}\nout: {}\nerr: {}",
                         child.status,
                         String::from_utf8_lossy(&child.stdout),
                         String::from_utf8_lossy(&child.stderr));
    println!("{}", result);
}

/*
    let r = Docker::connect_with_local_defaults();
    let docker = r.unwrap();
    let res= docker.stop_container(containerName, None).await;
    if !res.is_err() {
        println!("stop container {:?}", res.unwrap());
    }
    let res =docker.remove_container(containerName, None).await;
    if !res.is_err() {
        println!("remove container {:?}", res.unwrap());
    }
    let res = docker.create_container(Some(CreateContainerOptions{
        name: containerName,
        platform: None,
    }), Config {
        image: Some(imageName),
        ..Default::default()
    }).await.unwrap();
    println!("create container {:?}", res);*/