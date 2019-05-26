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
use rocket::http::RawStr;
use rocket::config::{ConfigBuilder, Environment, Value};
use std::collections::btree_map::BTreeMap;
use url::percent_encoding::{utf8_percent_encode, DEFAULT_ENCODE_SET};

#[database("mysql_conn_pool")]
pub struct MysqlConnPool(MysqlConnection);

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
        .extra(
            "databases",
            [
                (
                    "mysql_conn_pool".to_string(),
                    [
                        ("url".to_string(), Value::String(utf8_percent_encode(config.database.url.as_str(), DEFAULT_ENCODE_SET).to_string())),
                        ("pool_size".to_string(), Value::Integer(config.database.pool_size)),
                    ].iter().cloned().collect::<BTreeMap<String, Value>>()
                ),
            ].iter().cloned().collect::<BTreeMap<String, BTreeMap<String, Value>>>(),
        )
        .finalize() {
        Ok(conf) => {
            rocket::custom(conf)
                .attach(MysqlConnPool::fairing())
                .mount("/", routes![req_yao, find_user])
                .launch();
        }
        Err(error) => println!("service launch failed: {:?}", error)
    };
}

#[get("/yao")]
fn req_yao() -> &'static str {
    "ming"
}

#[get("/user/<user_id>")]
fn find_user(mysql_conn_pool: MysqlConnPool, user_id: &RawStr) -> String {
    "not ready".to_string()
}

fn read_config<T>(config_file: &str) -> T where T: DeserializeOwned {
    let mut config_string = String::new();
    File::open(config_file)
        .expect(&format!("{} file not found", config_file))
        .read_to_string(&mut config_string)
        .expect(&format!("something went wrong when reading the {} file", config_file));
    serde_yaml::from_str(&config_string).unwrap()
}
