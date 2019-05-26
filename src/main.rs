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
mod object;
mod table;

use diesel::mysql::MysqlConnection;
use reqwest::{Client, Url};
use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::Read;
use crate::config::{Config, RestApi};
use rocket::http::RawStr;
use rocket::config::{ConfigBuilder, Environment, Value};
use std::collections::btree_map::BTreeMap;
use url::percent_encoding::{utf8_percent_encode, DEFAULT_ENCODE_SET};
use crate::object::{User, UserForInsert, UserForUpdate};
use diesel::prelude::*;
use rocket::State;

#[database("mysql_conn_pool")]
pub struct MysqlConnPool(MysqlConnection);

pub struct HttpClient {
    pub reqwest_client: Client,
    pub config: RestApi,
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
                .manage(HttpClient {
                    reqwest_client: reqwest::Client::new(),
                    config: config.rest_api.clone(),
                })
                .mount("/", routes![req_yao, find_user, create_user, update_user, other_rest_api])
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
    use crate::table::user::dsl::*;

    match user_id.to_string().parse::<i32>() {
        Ok(user_id) =>
            match user.filter(id.eq(user_id))
                .first::<User>(&mysql_conn_pool.0) {
                Ok(u) => u.name,
                Err(e) => format!("user not found. id: {}, error: {}", user_id, e)
            }
        Err(_e) => format!("the user id must be number. error: {}", user_id)
    }
}

// デモしやすいために GET メソッドにした
#[get("/user/_create/<user_name>")]
fn create_user(mysql_conn_pool: MysqlConnPool, user_name: &RawStr) -> String {
    use crate::table::user::dsl::*;

    match diesel::insert_into(user)
        .values(&UserForInsert { name: user_name.to_string() })
        .execute(&mysql_conn_pool.0) {
        Ok(_) => format!("create user succeed."),
        Err(e) => format!("create user failed. name: {}, error: {}", user_name, e)
    }
}

// デモしやすいために GET メソッドにした
#[get("/user/_update/<user_id>/<user_name>")]
fn update_user(mysql_conn_pool: MysqlConnPool, user_id: &RawStr, user_name: &RawStr) -> String {
    use crate::table::user::dsl::*;

    match user_id.to_string().parse::<i32>() {
        Ok(user_id) =>
            match diesel::update(user.filter(id.eq(user_id)))
                .set(&UserForUpdate { name: user_name.to_string() })
                .execute(&mysql_conn_pool.0) {
                Ok(_) => format!("update user succeed."),
                Err(e) => format!("update user failed. id: {}, name: {}, error: {}", user_id, user_name, e)
            }
        Err(_e) => format!("the user id must be number. error: {}", user_id)
    }
}

#[get("/rest_api")]
fn other_rest_api(http_client: State<HttpClient>) -> String {
    let config = &http_client.config;
    let reqwest_client = &http_client.reqwest_client;
    let url = Url::parse(format!("http://{}:{}/{}", config.host, config.port, "yao").as_str()).unwrap();
    reqwest_client.get(url).send().unwrap().text().unwrap()
}

fn read_config<T>(config_file: &str) -> T where T: DeserializeOwned {
    let mut config_string = String::new();
    File::open(config_file)
        .expect(&format!("{} file not found", config_file))
        .read_to_string(&mut config_string)
        .expect(&format!("something went wrong when reading the {} file", config_file));
    serde_yaml::from_str(&config_string).unwrap()
}
