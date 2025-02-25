#[macro_use] 
extern crate rocket;
#[macro_use]
extern crate lazy_static;

mod schema;
mod models;
mod routes;
mod db;

use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};
use rocket_sync_db_pools::database;

#[database("sqlite_database")]
pub struct DbConn(diesel::SqliteConnection);

#[launch]
fn rocket() -> _ {
    dotenv::dotenv().ok();
    
    // Configuração CORS para permitir todas as origens
    let cors = CorsOptions {
        allowed_origins: AllowedOrigins::all(), // Permite qualquer origem
        allowed_methods: vec![Method::Get, Method::Post, Method::Put, Method::Delete]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::all(), // Permite todos os headers
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("Erro ao configurar CORS");
    
    let rocket = rocket::build()
        .attach(DbConn::fairing())
        .attach(cors) // Adiciona o CORS como um Fairing
        .mount("/", routes![
            routes::get_by_cpf,
            routes::get_by_email,
            routes::get_by_telefone,
            routes::get_by_nome,
            routes::get_cpfs_by_cep
        ]);

    rocket
}