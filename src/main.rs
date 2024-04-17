#[macro_use] extern crate rocket;
extern crate diesel;

#[macro_use] extern crate rocket_sync_db_pools;

mod auth;
mod models;
mod schema;

use auth::BasicAuth;
use diesel::prelude::*;
use models::Baza;
use schema::baza;
use rocket::serde::json::{json, Value};
use rocket::response::status;


// #[get("/")]
// fn hello() -> Value {
//     json!("Hello, world!")
// }

#[database("sqlite")]
struct DbConn(diesel::SqliteConnection);


#[get("/baza")]
async fn get_baza(_auth: BasicAuth, db: DbConn) -> Value {
    db.run(|c|{
        let result = baza::table.limit(100).load::<Baza>(c).expect("Failed to read Baza daxil olmalari");
        json!(result)
    }).await
}

#[get("/baza/<id>")]
fn view_baza(id: i32, _auth: BasicAuth) -> Value {
    json!([
        {
            "id": id,
            "name": "Sada Asga",
            "email": "sadagatasgarov@gmil.com"
        }
    ])
}

#[post("/baza", format = "json")]
fn create_baza(_auth: BasicAuth) -> Value {
    json!([
        {
            "id": 3,
            "name": "Sada2 Asga2",
            "email": "sadagatasgarov@gmil.com"
        }
    ])
}

#[put("/baza/<id>", format = "json")]
fn update_baza(id: i32, _auth: BasicAuth) -> Value {
    json!([
        {
            "id": id,
            "name": "Sada2 Asga2",
            "email": "sadagatasgarov@gmil.com"
        }
    ])
}


#[delete("/baza/<_id>")]
fn delete_baza(_id: i32, _auth: BasicAuth) -> status::NoContent{
    status::NoContent
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
        .register("/", catchers![not_found]).attach(DbConn::fairing())
        .launch()
        .await;
}
