#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use diesel::mysql::MysqlConnection;
use reqwest::Client;

#[database("db_mysql")]
pub struct DbMysqlConnection(MysqlConnection);

#[derive(Clone)]
pub struct HttpClient {
    pub reqwest_client: Client,
}

fn main() {
    rocket::ignite().mount("/", routes![yao]).launch();
}

#[get("/yao")]
fn yao() -> &'static str {
    "ming"
}
