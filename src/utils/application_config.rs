use serde::{Deserialize, Serialize};


#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ApplicationConfig {
    pub database: PostgreSQL,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PostgreSQL {
    pub db_type: String,
    pub url: String,
    pub port: i32,
    pub database: String,
    pub username: String,
    pub password: String,
}
