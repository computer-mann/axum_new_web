mod routes;
mod open_api;

use axum::{
    routing::get,
    Router,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::routes::*;
use crate::open_api::*;

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(log::Level::Debug).unwrap();
    dotenv::dotenv().ok();
    for(key,value) in std::env::vars(){
        if key.eq("MUSIC_NOW"){
            log::info!("{} , {} ",key,value);
        }
    }
    // build our application with a single route
    let app = Router::new()
        .merge(
            SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi())
        )
        .route("/tell",get(tell_me))
        .route("/json",get(json_return))
        .route("/", get(hellow_word))
        .route("/env_variable",get(env_variables))
        .route("/forbidden_response",get(return_403_with_data))
        .route("/headers_with_data",get(with_status_and_array_headers))
        ;


    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:5005").await.unwrap();
    log::info!("serving axum app at port 5005.");
    axum::serve(listener, app).await.unwrap();
    
}

