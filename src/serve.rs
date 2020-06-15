use hyper::{Body, Request, Response};
use std::convert::Infallible;

use crate::html;

pub async fn index(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let body = html::index();

    Ok(Response::new(body.into()))
}
