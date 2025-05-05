mod proxy;

#[tokio::main]
async fn main() {
    if let Err(e) = proxy::start_proxy().await {
        eprintln!("Proxy error: {e}");
    }
}
