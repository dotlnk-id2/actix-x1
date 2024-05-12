use std::time::Duration;

use actix_web::{web, App,HttpServer};

use example_web::api;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let bind = ("0.0.0.0", 8080);


    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("./ssl/key.pem", SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file("./ssl/cert.pem").unwrap();

    let server = HttpServer::new(|| {
        App::new()
        .configure(api::config_api)
    })
    .workers(8)
    .keep_alive(Duration::from_secs(30))
    .shutdown_timeout(40)
    // .bind(bind);
    .bind_openssl(bind, builder);

    match server {
        Ok(thread) => {
            tracing::info!("tcp_bind {}:{} Ok !!!",bind.0,bind.1);
            thread.run().await
        },
        Err(e) => {
            tracing::error!("service {:?} failed",bind);
            tracing::error!("{:?}",e);
            std::io::Result::Err(e)
        }
    }
}



