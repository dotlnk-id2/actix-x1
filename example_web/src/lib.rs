pub mod api {
    use actix_web::web::ServiceConfig;


    pub fn config_api(cfg: &mut ServiceConfig) {
        test::load_test(cfg);
    }

    pub mod test {
        use actix_web::{get, post, web::{self, ServiceConfig}, HttpResponse, Responder};

        pub(crate) fn load_test(cfg: &mut ServiceConfig){
            cfg
            .route("/hello/world",web::get().to(hello))
            .route("/echo",web::post().to(echo))
            .route("/hey", web::get().to(manual_hello));

        }

        // #[get("/hello/world")]
        async fn hello() -> impl Responder {
            HttpResponse::Ok().body("Hello world!")
        }

        // #[post("/echo")]
        async fn echo(req_body: String) -> impl Responder {
            HttpResponse::Ok().body(req_body)
        }

        // #[get("/hello/world")]
        pub async fn manual_hello() -> impl Responder {
            HttpResponse::Ok().body("Hey there!")
        }
    }
}

pub mod StateMachine{

}
