use actix_web::{error, get, post, web, App, Error, HttpResponse, HttpServer, Responder};
use futures::StreamExt;
use serde;
use std::env;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello worldaaa!")
}

#[derive(serde::Serialize, serde::Deserialize)]
struct MyObj {
    name: String,
    number: i32,
}

const MAX_SIZE: usize = 262_144;

#[post("/hello")]
async fn index_manual(mut payload: web::Payload) -> Result<HttpResponse, Error> {
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    let obj = serde_json::from_slice::<Vec<MyObj>>(&body)?;
    Ok(HttpResponse::Ok().json(obj))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
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
            .route("/hey", web::get().to(manual_hello))
    })
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
