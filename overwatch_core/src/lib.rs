mod proxy;

pub fn main_entry() {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        if let Err(e) = crate::proxy::start_proxy().await {
            eprintln!("Proxy error: {e}");
        }
    });
}
