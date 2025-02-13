

use std::{fs, path::Path};

use serde::{Deserialize,Serialize};
use serde_yaml::Error;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config{
    pub db: DbConfig,
    pub server: ServerConfig,
    pub redis: ResdisConfig,
    pub log: LogConfig
}
impl Config {
    pub fn load(file: impl AsRef<Path>)-> Result<Self,Error >{
        let content= fs::read_to_string(file).unwrap();
        Ok(serde_yaml::from_str(&content)?)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResdisConfig {
    pub host: String,
    pub port: u16,
    pub seq_step: i32
}

impl ResdisConfig {
    pub fn url(&self)-> String{
        format!("redis://{}:{}", self.host, self.port)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerConfig{
    pub host: String,
    pub port: u16,
    pub jwt_secret: String,
    pub oauth2: OAuth2
}
impl ServerConfig {
    pub fn url(&self, https:bool)-> String{
        match https {
            true => format!("https://{}:{}",self.host,self.port),
            false => format!("https://{}:{}",self.host,self.port),
        }
    }
    pub fn server_url(&self)-> String{
        format!("{}:{}", &self.host, self.port)
    }
    pub fn with_port(&self, port: u16)-> ServerConfig{
        ServerConfig { 
            host: self.host.clone(),
            port, 
            jwt_secret: self.jwt_secret.clone(), 
            oauth2: self.oauth2.clone() 
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OAuth2{
    pub google: OAuth2Item // google, facebook, github
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OAuth2Item{
    pub client_id: String,
    pub client_secret: String,
    pub redirect_url: String,
    pub auth_url: String,
    pub token_url: String,
    pub user_info_url: String,
    pub email_url: String
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LogConfig{
    pub level: String,
    pub output: String
}
impl LogConfig {
    pub fn level(&self)-> tracing::Level{
        match self.level.as_str() {
            "trace" => tracing::Level::TRACE,
            "debug" => tracing::Level::DEBUG,
            "info" => tracing::Level::INFO,
            "warn" => tracing::Level::WARN,
            "error" => tracing::Level::ERROR,
            _ => tracing::Level::INFO
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DbConfig{
    pub mysql: MysqlConfig
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MysqlConfig{
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
    #[serde(default ="default_conn")]
    pub max_connections: u32
}
fn default_conn()->u32{
    5
}