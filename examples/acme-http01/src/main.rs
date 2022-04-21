use salvo::listener::AcmeListener;
use salvo::prelude::*;

#[fn_handler]
async fn hello_world() -> &'static str {
    "Hello World"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let mut router = Router::new().get(hello_world);
    let listener = AcmeListener::builder()
        // .directory("letsencrypt", salvo::listener::acme::LETS_ENCRYPT_STAGING)
        .cache_path("acme/letsencrypt")
        .add_domain("acme-http01.salvo.rs")
        .http01_challege(&mut router)
        .bind("0.0.0.0:443")
        .await;
    tracing::info!("Listening on https://0.0.0.0:443");
    Server::new(listener.join(TcpListener::bind("0.0.0.0:80")))
        .serve(router)
        .await;
}