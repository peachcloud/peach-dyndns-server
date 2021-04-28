use futures::Future;
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use hyper::service::{make_service_fn, service_fn};


async fn handle_request(_req: Request<Body>) -> Result<Response<Body>, Infallible>{
    Ok(Response::new(Body::from("Hello, World!")))
}

pub async fn server() {
    let address = ([127, 0, 0, 1], 3000).into();

    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(handle_request))
    });

    info!("HTTP server listening for TCP on {:?}", address);

    Server::bind(&address).serve(make_svc).await;
}
