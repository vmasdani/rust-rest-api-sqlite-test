use crate::models::{User, UserJson, UserNew};
use crate::Pool;

use actix_web::guard::Any;
use actix_web::http::StatusCode;
use actix_web::{web, Error, HttpResponse};
use anyhow::Result;
use diesel::dsl::insert_into;
use diesel::r2d2::ConnectionManager;
use diesel::RunQueryDsl;
use diesel::{prelude::*, r2d2};

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub async fn root() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::build(StatusCode::OK).body("Hello World, Rust!"))
}

pub async fn create_user(
    pool: web::Data<DbPool>,
    item: web::Json<User>,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || new_user(pool, item))
        .await
        .map(|user| HttpResponse::Created().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn new_user(pool: web::Data<DbPool>, item: web::Json<User>) -> Result<User, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    let db_connection = pool.get().unwrap();

    match users
        .filter(name.eq(&item.name))
        .first::<User>(&db_connection)
    {
        Ok(result) => Ok(result),
        Err(_) => {
            let new_user = UserNew {
                name: &item.name,
                address: &item.address,
                date_created: &format!("{}", chrono::Local::now().naive_local()),
            };

            insert_into(users)
                .values(&new_user)
                .execute(&db_connection)
                .expect("Error");

            let result = users
                .order(id.desc())
                .first::<User>(&db_connection)
                .unwrap();
            Ok(result)
        }
    }
}

pub async fn get_users(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    Ok(list_users(pool)
        .await
        .map(|users| HttpResponse::Ok().json(users))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

async fn list_users(pool: web::Data<DbPool>) -> Result<Vec<User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    let db_connection = pool.get().unwrap();
    let result = users.load::<User>(&db_connection)?;
    Ok(result)
}
