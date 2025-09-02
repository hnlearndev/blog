#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    blog::server::run().await;
}

#[cfg(not(feature = "ssr"))]
fn main() {
    // No-op main for non-ssr builds
}
