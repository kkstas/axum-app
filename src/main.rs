use axum::{middleware, response::Response, routing::get_service, Router};
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

pub use self::error::{Error, Result};

mod error;
mod model;
mod web;

#[tokio::main]
async fn main() {
    // Router utworzony osobno, aby tylko tu działało auth
    let routes_apis =
        web::routes_data::routes().route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let routes_all = Router::new()
        .merge(web::routes_login::routes())
        .nest("/api", routes_apis)
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} -  main_response_mapper", "RES_MAPPER");
    println!("");
    res
}

fn routes_static() -> Router {
    println!("->> {:<12} - static", "HANDLER");
    Router::new().nest_service("/", get_service(ServeDir::new("./static")))
}
