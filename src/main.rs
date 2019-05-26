#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

mod config;

use diesel::mysql::MysqlConnection;
use reqwest::Client;
use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::Read;
use crate::config::Config;
use rocket::config::{ConfigBuilder, Environment};

#[database("db_mysql")]
pub struct DbMysqlConnection(MysqlConnection);

#[derive(Clone)]
pub struct HttpClient {
    pub reqwest_client: Client,
}

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

    let config: Config = read_config("config.yaml");
    match ConfigBuilder::new(Environment::Production)
        .address(config.server.address)
        .port(config.server.port)
        .finalize() {
        Ok(config) => { rocket::custom(config).mount("/", routes![req_yao]).launch(); }
        Err(error) => println!("service launch failed: {:?}", error)
    };
}

#[get("/yao")]
fn req_yao() -> &'static str {
    "ming"
}

fn read_config<T>(config_file: &str) -> T where T: DeserializeOwned {
    let mut config_string = String::new();
    File::open(config_file)
        .expect(&format!("{} file not found", config_file))
        .read_to_string(&mut config_string)
        .expect(&format!("something went wrong when reading the {} file", config_file));
    serde_yaml::from_str(&config_string).unwrap()
}
