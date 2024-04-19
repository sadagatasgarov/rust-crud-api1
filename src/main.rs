#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

#[macro_use]
extern crate rocket_sync_db_pools;

mod auth;
mod models;
mod repositories;
mod schema;

use auth::BasicAuth;
use models::{Baza, NewBaza};
use rocket::http::Status;
use rocket::response::status::{self, Custom};
use rocket::serde::json::{json, Json, Value};

use crate::repositories::BazaRepository;

// #[get("/")]
// fn hello() -> Value {
//     json!("Hello, world!")
// }

#[database("sqlite")]
struct DbConn(diesel::SqliteConnection);

#[get("/baza")]
async fn get_baza(_auth: BasicAuth, db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        BazaRepository::find_multiple(c, 100)
        .map(|baza| json!(baza)).map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}

#[get("/baza/<id>")]
async fn view_baza(id: i32, _auth: BasicAuth, db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        BazaRepository::find(c, id)
        .map(|baza| json!(baza)).map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[post("/baza", format = "json", data = "<new_baza>")]
async fn create_baza(_auth: BasicAuth, db: DbConn, new_baza: Json<NewBaza>) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        BazaRepository::create(c, new_baza.into_inner())
        .map(|baza| json!(baza)).map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[put("/baza/<id>", format = "json", data = "<baz>")]
async fn update_baza(id: i32, _auth: BasicAuth, db: DbConn, baz: Json<Baza>) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        BazaRepository::save(c, id, baz.into_inner())
        .map(|baza| json!(baza)).map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[delete("/baza/<id>")]
async fn delete_baza(id: i32, _auth: BasicAuth, db: DbConn) -> Result<status::NoContent, Custom<Value>> {
    db.run(move |c| {
        BazaRepository::delete(c, id)
        .map(|_| status::NoContent)
        .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[catch(404)]
fn not_found() -> Value {
    json!("Not found")
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            routes![
                // hello,
                get_baza,
                view_baza,
                create_baza,
                update_baza,
                delete_baza
            ],
        )
        .register("/", catchers![not_found])
        .attach(DbConn::fairing())
        .launch()
        .await;
}
