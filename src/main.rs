use crate::ctx::Ctx;
use crate::log::log_request;
use axum::http::{Method, Uri};
use axum::{
    middleware,
    response::{IntoResponse, Response},
    routing::get_service,
    Json, Router,
};
use model::ModelController;
use serde_json::json;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use uuid::Uuid;

pub use self::error::{ClientError, Error, Result};

mod ctx;
mod error;
mod log;
mod model;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    let mc = ModelController::new().await?;

    let routes_apis = web::routes_tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let routes_all = Router::new()
        .nest("/api", web::routes_login::routes())
        .nest("/api", routes_apis)
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn_with_state(
            mc.clone(),
            web::mw_auth::mw_ctx_resolver,
        ))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();
    Ok(())
}

async fn main_response_mapper(
    req_method: Method,
    uri: Uri,
    ctx: Option<Ctx>,
    res: Response,
) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    let uuid = Uuid::new_v4();

    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());

    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "error":{
                "type": client_error.as_ref(),
                "req_uuid": uuid.to_string(),
            }
            });
            println!("  ->> client_error_body: {client_error_body}");
            (*status_code, Json(client_error_body)).into_response()
        });

    // -- TODO: Build and log the server log line
    let client_error = client_status_error.unzip().1;
    log_request(uuid, req_method, uri, ctx, service_error, client_error)
        .await
        .unwrap();
    println!("  ->> server log line - {uuid} - Error: {service_error:?}");

    println!();
    error_response.unwrap_or(res)
}

fn routes_static() -> Router {
    println!("->> {:<12} - static", "HANDLER");
    Router::new().nest_service("/", get_service(ServeDir::new("./static")))
}
