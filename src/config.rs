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
pub struct RestApi {
    pub host: String,
    pub port: String,
}

#[derive(Deserialize)]
pub struct Database {
    pub connection_pool: ConnectionPool,
}

#[derive(Deserialize)]
pub struct ConnectionPool {
    pub url: String,
    pub pool_size: i64,
}
