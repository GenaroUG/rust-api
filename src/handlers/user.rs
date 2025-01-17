use actix_web::{get, post, HttpResponse, Responder};

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

pub async fn jumping() -> impl Responder {
    HttpResponse::Ok().body("Jumping!")
}

pub async fn playing() -> impl Responder {
    HttpResponse::Ok().body("Playing!")
}

pub async fn manual_goodbye() -> impl Responder {
    HttpResponse::Ok().body("See you!")
}

pub async fn running() -> impl Responder {
    HttpResponse::Ok().body("Running!")
}

pub async fn dancing() -> impl Responder {
    HttpResponse::Ok().body("Dancing!")
}