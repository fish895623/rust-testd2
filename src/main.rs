use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use serde;
use std::env;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello worldaaa!")
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct MyObj {
    name: String,
    number: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Info {
    name: String,
}

#[post("/api/hello")]
async fn index_manual(name: web::Json<Info>) -> Result<impl Responder> {
    let obj = Info {
        name: name.name.clone(),
    };
    Ok(web::Json(obj))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port;
    match env::var("PORT") {
        Ok(res) => {
            port = res.to_string().parse::<u16>().unwrap();
        }
        Err(_e) => {
            port = 3000;
        }
    }
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(index_manual)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
#[cfg(test)]
mod test {
    use super::*;
    use actix_web::{http, test, App};

    #[actix_web::test]
    async fn test_index() {
        let mut app = test::init_service(App::new().service(hello)).await;
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }
}
