use actix_web::{App, HttpServer, get, Responder, HttpResponse, web};
use diesel::prelude::*;

mod db_models;
mod schema;
mod routes;
mod db_utils;

use db_models::*;
use crate::db_utils::establish_connection;

#[get("/")]
async fn show_users() -> impl Responder{
    use schema::users::dsl::*;
    let connection = &mut establish_connection();
    let results = users
        .load::<User>(connection)
        .expect("Error loading posts");
    HttpResponse::Ok().json(results)
}

#[get("/adduser/{first}/{last}")]
async fn add_user(path: web::Path<(String, String)>) -> impl Responder{
    use schema::users::dsl::*;
    let conn = &mut establish_connection();
    let path = path.into_inner();
    let f_name = &path.0;
    let l_name = path.1;

    let new_user = NewUser{
        first_name: &f_name,
        last_name: &l_name,
    };

    diesel::insert_into(users)
        .values(&new_user)
        .get_result::<User>(conn)
        .expect("error crating post");

    HttpResponse::Ok().json("ok")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(move || {
        App::new()
            .service(show_users)
            .service(add_user)

    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
