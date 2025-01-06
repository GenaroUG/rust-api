#[path = "../internal/mod.rs"]
mod internal;
use actix_web::{web, App, HttpServer};
use internal::infrastructure::routes; 

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(routes::hello) // Ahora solo usamos `hello`, `echo`, `manual_hello`
            .service(routes::echo)
            .route("/hey", web::get().to(routes::manual_hello))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
