use back_end::{apps::axum::routes::start,config};

#[tokio::main]
async fn main() {
    //let config= config::Config::load("config.yml").unwrap();
    // debug
    let config= config::Config::load("D:\\PROJECT\\RUST\\rust-base-2025\\back-end\\config.yml").unwrap();
    start(config).await;
}
