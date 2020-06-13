extern crate minifier;

use std::convert::Infallible;
use std::net::SocketAddr;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};

mod html;

async fn serve(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let body = html::index();

    Ok(Response::new(body.into()))
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");

    println!("Shutting down...");
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(serve)) });

    let server = Server::bind(&addr).serve(make_svc);

    let graceful = server.with_graceful_shutdown(shutdown_signal());

    println!("Server listening on http://{}", addr);

    if let Err(e) = graceful.await {
        eprintln!("Server error: {}", e);
    }
}
