use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;

pub async fn start_proxy() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("🛡️ Overwatch listening on http://{}", addr);

    let make_svc =
        make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(forward_request)) });

    let server = Server::bind(&addr).serve(make_svc);
    server.await?;
    Ok(())
}

async fn forward_request(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let client = Client::new();

    // Modify the request URI to point to the real destination if needed
    let uri_string = format!("http://{}", req.uri().host().unwrap_or("example.com"));
    let mut new_req = Request::builder()
        .method(req.method())
        .uri(uri_string)
        .body(req.into_body())?;

    *new_req.headers_mut() = req.headers().clone();

    client.request(new_req).await
}
