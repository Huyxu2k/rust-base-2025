use back_end::{start,config};

#[tokio::main]
async fn main() {
    let config= config::Config::load("config.yml").unwrap();
    start(config).await;
}
