#[macro_use]
extern crate diesel;

mod models;
mod routes;
mod schema;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use diesel::r2d2::{self, ConnectionManager, Pool};
use diesel::SqliteConnection;

// pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("NOT FOUND");
    let database_pool = Pool::builder()
        .build(ConnectionManager::<SqliteConnection>::new(database_url))
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .data(database_pool.clone())
            .route("/", web::get().to(routes::root))
            .route("/users", web::post().to(routes::create_user))
            .route("/getusers", web::get().to(routes::get_users))
    })
    .bind("localhost:8080")?
    .run()
    .await
}
