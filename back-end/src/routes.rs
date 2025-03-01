use axum::routing::{delete,get,post,put};
use axum::Router;
use tower::ServiceBuilder;

use crate::middleware::layer::{health_check, AccessTokenLayer, AuthorizationLayer, LoggingLayer, RefreshTokenLayer };
use crate::middleware::TLayer;
use crate::modules::user::route::user_router;
use crate::AppState;

type _AccessTokenLayer = TLayer<AccessTokenLayer>;
type _RefreshTokenLayer = TLayer<RefreshTokenLayer>;
type _LoggingLayer = TLayer<LoggingLayer>;
type _AuthorizationLayer = TLayer<AuthorizationLayer>;

pub(crate) fn app_routes(state: AppState)->Router{

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
                    


    Router::new()
    .route("/health_check",get(health_check) )
    //k phải qua layer nào , nhằm yêu cầu server tạo token và refresh token khi đăng nhập lần đầu
    //.route("/login", method_router)
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
    //    + không có quyền trả ra lỗi 4032
    //    + có quyền thực hiện tiếp resquest
    //- 3 layer ghi lại log
    .nest("/api", module_routes)
    //.layer(level_2)
}

