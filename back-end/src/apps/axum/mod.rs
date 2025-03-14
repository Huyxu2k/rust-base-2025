pub mod middlewares;
pub mod routes;
pub mod error;
pub mod handlers;
pub mod traits;
pub mod state;


pub async fn start(config: Config){
    let state= AppState::new(config.clone()).await;
    let app= routes::app_routes(state);

    let listener= tokio::net::TcpListener::bind(&config.server.server_url())
                .await
                .unwrap();
    tracing::debug!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>()   
    )
    .await
    .unwrap();

}