#[derive(Deserialize)]
pub struct Config {
    pub server: Server,
    pub database: Database,
    pub rest_api: RestApi,
}

#[derive(Deserialize)]
pub struct Server {
    pub address: String,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct Database {
    pub url: String,
    pub pool_size: i64,
}

#[derive(Deserialize, Clone)]
pub struct RestApi {
    pub host: String,
    pub port: String,
}
