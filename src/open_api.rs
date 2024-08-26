use axum::Json;
use utoipa::OpenApi;
use crate::routes::*;

#[derive(OpenApi)]
#[openapi(paths(openapi))]
#[openapi(paths(json_return),components(schemas(Message)))]
pub struct ApiDoc;

#[utoipa::path(
    get,
    path = "/api-docs/openapi.json",
    responses(
        (status = 200, description = "JSON file", body = ())
    )
)]
pub async fn openapi()-> Json<utoipa::openapi::OpenApi>{
    Json(ApiDoc::openapi())
}