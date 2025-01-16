// Nuevas dependencias
pub mod schema;

use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;

pub mod api;
use api::infrastructure::routes; 
use api::infrastructure::db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	
    // Variables de entorno
	dotenv().ok();
	let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be setted");

		// Creamos el manager de la base de datos
    let connection = ConnectionManager::new(database_url);
    let pool = Pool::builder() // Creamos una pool de conexiones
        .build(connection)
        .expect("Failed to create pool.");

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
