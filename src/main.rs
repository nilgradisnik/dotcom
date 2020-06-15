extern crate minifier;

#[macro_use]
extern crate log;

use std::convert::Infallible;
use std::net::SocketAddr;

use hyper::service::{make_service_fn, service_fn};
use hyper::Server;

mod html;
mod serve;

#[tokio::main]
async fn main() {
    env_logger::init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    info!("Server started on http://{}", addr);

    if let Err(error) = Server::bind(&addr)
        .serve(make_service_fn(|_conn| async {
            Ok::<_, Infallible>(service_fn(serve::index))
        }))
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c()
                .await
                .expect("Failed to install CTRL+C signal handler");

            warn!("Shutting down...");
        })
        .await
    {
        error!("Server error: {}", error);
    }
}
