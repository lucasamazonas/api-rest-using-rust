use std::io::Result;
use actix_web::{web, App, HttpServer};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

mod usuario;
mod models;
mod schema;

#[actix_web::main]
async fn main() -> Result<()> {
  HttpServer::new(|| {
    App::new()
      .service(web::scope("/api")
        .service(usuario::cadastrar_usuario)
        .service(usuario::editar_usuario)
        .service(usuario::excluir_usuario)
        .service(usuario::listar_usuario)
        .service(usuario::listar_usuarios)
      )
  })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

pub fn establish_connection() -> PgConnection {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  PgConnection::establish(&database_url)
    .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}