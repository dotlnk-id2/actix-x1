use actix_web::{web, App,HttpServer};

use example_web::api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let bind = ("0.0.0.0", 8080);

    let builder = HttpServer::new(|| {
        App::new()
        .configure(api::config_api)
    })
    .bind(bind);

    match builder {
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



