use std::{collections::HashMap, sync::Arc};
use quinn::{Endpoint, ServerConfig};
use flate2::{Compression, write::GzDecoder};
use tokio::sync::Mutex;



#[derive(Clone)]
pub struct QuicFileService{
    endpoint: Endpoint,
    resume_map: Option<Arc<Mutex<HashMap<String,u64>>>>
}
 
impl QuicFileService {

    
    pub async fn new(server_address: String, server: bool, resume: bool,ipv6: bool)-> Result<Self, Box<std::error::Error>>{
       match server {
            true => {
                ServerConfig::new(crypto, token_key)
            },
            false =>{
                todo!()
            },
        }
    }

    //pub async fn upload_file(&self, )
}