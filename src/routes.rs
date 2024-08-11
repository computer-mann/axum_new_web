use axum::http::{header, StatusCode};
use axum::Json;
    use axum::response::IntoResponse;
    use dotenv_codegen::dotenv;
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Value};

    pub async fn hellow_word() ->String{
        "Hello, World! from axum need to learn rust.".to_string()
    }

    pub async fn tell_me()->String{
        log::info!("tell me endpoint was called");
        "Tell me something".to_string()
    }

    pub async fn json_return()->Json<Message>{
        let data=Message{ message : "Json message".to_string()};
        log::info!("json endpoint was called");
        Json(data)
    }
    pub async fn env_variables()->Json<Value>{
        log::info!("env_variables route called");
        log::info!("{}",dotenv!("MUSIC_NOW"));
        Json(json!("{data: 2}"))
    }

    pub async fn return_403_with_data()->impl IntoResponse{

        (StatusCode::FORBIDDEN,Json(Message{message:"Forbidden".to_string()}))
    }
// `(StatusCode, headers, impl IntoResponse)` to set status and add headers
// `headers` can be either a `HeaderMap` or an array of tuples
pub async fn with_status_and_array_headers() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        [(header::CONTENT_TYPE, "text/plain")],
        "foo",
    )
}
    #[derive(Serialize,Deserialize)]
    pub struct Message{
       message: String
    }
