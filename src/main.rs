#[macro_use] 
extern crate rocket;
#[macro_use]
extern crate lazy_static;

mod schema;
mod models;
mod routes;
mod db;

use rocket_sync_db_pools::database;

#[database("sqlite_database")]
pub struct DbConn(diesel::SqliteConnection);

#[launch]
fn rocket() -> _ {
    dotenv::dotenv().ok();
    let rocket = rocket::build()
        .attach(DbConn::fairing())
        .mount("/", routes![
            routes::get_by_cpf,
            routes::get_by_email,
            routes::get_by_telefone,
            routes::get_by_nome,
            routes::get_cpfs_by_cep
        ]);

    rocket
}