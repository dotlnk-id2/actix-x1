pub mod api {
    use actix_web::web::ServiceConfig;


    pub fn config_api(cfg: &mut ServiceConfig) {
        test::load_config(cfg);
        health::load_config(cfg);
        v1::load_config(cfg);
    }

    pub mod test {
        // use actix_web::{get, post}
        use actix_web::{ web::{self, ServiceConfig}, HttpResponse, Responder};

        pub const PATH_PREFIX_V1: &str = "/test";

        pub(crate) fn load_config(cfg: &mut ServiceConfig){
            cfg.service(
                web
                    ::scope(PATH_PREFIX_V1)
                    .route("/hello/world", web::get().to(hello))
                    .route("/echo", web::post().to(echo))
                    .route("/hey", web::get().to(manual_hello))
            );
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

    pub mod health {
        use actix_web::{web::{self, ServiceConfig}, HttpResponse, Responder};

        pub(crate) fn load_config(cfg: &mut ServiceConfig){
            cfg
            .route("/health",web::get().to(health));
        }

        async fn health() -> impl Responder {
            HttpResponse::Ok().json("Hello world!")
        }

    }

    pub mod v1 {
        use actix_web::{web::{self, ServiceConfig}, HttpResponse, Responder};

        pub const PATH_PERFIX : &str = "/api/v1";

        pub(crate) fn load_config(cfg: &mut ServiceConfig){
            cfg
            .service(web::scope(PATH_PERFIX)
            .route("/goods/list/${type}",web::post().to(goods_list))
            .route("/goods/${id}",web::post().to(goods_detail)));
        }

        async fn goods_list() -> impl Responder {
            HttpResponse::Ok().json("Hello world!")
        }

        async fn goods_detail() -> impl Responder {
            HttpResponse::Ok().json("Hello world!")
        }

    }
}

pub mod state_machine{
    use std::collections::HashMap;

    use serde::{Deserialize, Serialize};


    #[derive(Debug,Deserialize,Serialize,Clone)]
    pub struct ConfFSMap{
        bind_addr:String,

        db_conf:CfgDatabase,
        dici: HashMap<String,String>,
    }

    #[derive(Debug,Deserialize,Serialize,Clone)]
    pub struct CfgDatabase{
        db_url:String,
        db_user:String,
        db_passwd:String,
    }
}


pub mod config_toml {
    use serde::{Deserialize, Serialize};
    use std::sync::Arc;

    #[derive(Debug, Default, Deserialize, Serialize)]
    pub struct AppConfig {
        pub server_addr: Arc<String>,
        // pub port: u16,
        // pub redis_addr: String,
        // pub redis_port: u16,
        pub database: Database,
        pub route_mapping: Vec<RouteMapping>,
    }

    #[derive(Deserialize, Debug, Clone, Default)]
    pub struct Database {
        pub host: String,
        pub user: String,
    }

    #[derive(Deserialize, Debug, Clone)]
    pub struct RouteMapping {
        pub prefix: String,
        pub target: String,
    }
}
