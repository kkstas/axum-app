use crate::{Error, Result};
use axum::{http::Request, middleware::Next, response::Response};
// use lazy_regex::regex_captures;
use tower_cookies::Cookies;

use crate::web::AUTH_TOKEN;

pub async fn mw_require_auth<B>(
    cookies: Cookies,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    // TODO: auth-token parsing & validation
    auth_token.ok_or(Error::AuthFailNoAuthTokenCookie)?;

    Ok(next.run(req).await)
}

// fn parse_token(token: String) -> Result<(u64, String, String)> {
//     let (_whole, user_id, exp, sign) = regex_captures!(r#"^user-(\d+)\.(.+)\.(.+)"#, &token)
//         .ok_or(Error::AuthFailTokenWrongFormat)?;
//     todo!()
// }
