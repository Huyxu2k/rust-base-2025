use std::net::SocketAddr;
use std::sync::Arc;

use axum::extract::State;
use axum::routing::{delete,get,post,put};
use axum::Router;
use tower::ServiceBuilder;

use super::handlers::auth::auth_router;
use super::handlers::users::user_router;
use super::middlewares::layer::{health_check, AccessTokenLayer, AuthorizationLayer, LoggingLayer, RefreshTokenLayer };
use super::middlewares::TLayer;
use crate::apps::axum::state::AppState;
use crate::config::Config;

type _AccessTokenLayer = TLayer<AccessTokenLayer>;
type _RefreshTokenLayer = TLayer<RefreshTokenLayer>;
type _LoggingLayer = TLayer<LoggingLayer>;
type _AuthorizationLayer = TLayer<AuthorizationLayer>;

fn app_routes(state: State<Arc<AppState>>)->Router{

    // init layer
    let access_token_layer= _AccessTokenLayer::new(state.clone());
    let refresh_token_layer= _RefreshTokenLayer::new(state.clone());
    let logging_layer= _LoggingLayer::new(state.clone());
    let authorization_layer= _AuthorizationLayer::new(state.clone());

    // refresh token
    let level_1=ServiceBuilder::new()
                        .layer(refresh_token_layer);

    // access token + authorzation + logger 
    let level_2  = ServiceBuilder::new()
                        .layer(access_token_layer);
                        //.layer(authorization_layer);
                        //.layer(logging_layer);
    
    // các route của các module
    let module_routes = Router::new()
                        .nest("/user", user_router(state.clone()));
                        //.route("/products", method_router);

    let auth_routes = Router::new()
            .nest("/auth",auth_router(state.clone()));
                    


    Router::new()
    .route("/health_check",get(health_check) )
    //k phải qua layer nào , nhằm yêu cầu server tạo token và refresh token khi đăng nhập lần đầu
    .nest("/api/v1/",auth_routes)
    //phải qua 1 layer kiểm tra xem refesh token có đúng không?
    //- không đúng trả ra lỗi 401 (yêu cầu đăng nhập lại)
    //- đúng thì cấp access token mới
    //.route("/token", method_router)
    //.layer(level_1)
    //phải qua 3 layer
    //- 1 layer ktra xem access token có đúng k?
    //    + không đúng trả ra lỗi 401 (yêu cầu gửi refresh token để cấp lại access token mới)
    //    + đúng thì cho phép thực hiện request
    //- 2 layer kiểm tra xem có quyền không?
    //    + không có quyền trả ra lỗi 403
    //    + có quyền thực hiện tiếp resquest
    //- 3 layer ghi lại log
    .nest("/api/v1/", module_routes)
    //.layer(level_2)
}

pub async fn start(config: Config){
    let state=Arc::new(AppState::new(config.clone()));
    let app= app_routes(State(state));

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