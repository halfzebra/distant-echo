use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use log::info;
use pretty_env_logger;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::time::delay_for;

async fn echo_response(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let delay_ms = 10000;
    info!("{} {}", req.method(), req.uri());
    info!("Delaying by {}ms", delay_ms);
    delay_for(Duration::from_millis(delay_ms)).await;
    Ok(Response::new(format!("{:?}", req.headers()).into()))
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let port = 3000;
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    // A `Service` is created for every connection, using `echo_response` function.
    let make_svc = make_service_fn(|_conn| {
        async {
            // service_fn converts our function into a `Service`
            Ok::<_, Infallible>(service_fn(echo_response))
        }
    });

    info!("Listening at http://localhost:{}", port);
    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
