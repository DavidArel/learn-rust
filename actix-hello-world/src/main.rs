use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct Personal {
    name: String,
    fav_num: u32,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world! This is actix response!")
}

#[get("/sum-num-query")]
async fn sum_num_query(personal: web::Query<Personal>) -> Result<String> {
    Ok(format!("Hello {}, I believe your fav number is {} right?", personal.name, personal.fav_num))
}

#[get("/sum-num-path/{my_number}")]
async fn sum_num_path(number: web::Path<u32>) -> Result<String> {
    let param = number.into_inner();
    Ok(format!("Number {}, if multiplied by 5 the number will be {}!", param, param * 5))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Manual hello from App!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(sum_num_query)
            .service(sum_num_path)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}