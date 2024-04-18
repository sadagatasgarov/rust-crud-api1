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
use rocket::response::status;
use rocket::serde::json::{json, Json, Value};

use crate::repositories::BazaRepository;

// #[get("/")]
// fn hello() -> Value {
//     json!("Hello, world!")
// }

#[database("sqlite")]
struct DbConn(diesel::SqliteConnection);

#[get("/baza")]
async fn get_baza(_auth: BasicAuth, db: DbConn) -> Value {
    db.run(|c| {
        let result =
            BazaRepository::find_multiple(c, 100).expect("Failed to read Baza daxil olmalari");
        json!(result)
    })
    .await
}

#[get("/baza/<id>")]
async fn view_baza(id: i32, _auth: BasicAuth, db: DbConn) -> Value {
    db.run(move |c| {
        let baz = BazaRepository::find(c, id).expect("bazaya baxanda problem oldu");
        json!(baz)
    })
    .await
}

#[post("/baza", format = "json", data = "<new_baza>")]
async fn create_baza(_auth: BasicAuth, db: DbConn, new_baza: Json<NewBaza>) -> Value {
    db.run(|c| {
        let result = BazaRepository::create(c, new_baza.into_inner())
            .expect("Elave ederken problem oldu");
        json!(result)
    })
    .await
}

#[put("/baza/<id>", format = "json", data = "<baz>")]
async fn update_baza(id: i32, _auth: BasicAuth, db: DbConn, baz: Json<Baza>) -> Value {
    db.run(move |c| {
        let result = BazaRepository::save(c, id, baz.into_inner())
            .expect("Update eden zaman xeta oldu");
        json!(result)
    })
    .await
}

#[delete("/baza/<id>")]
async fn delete_baza(id: i32, _auth: BasicAuth, db: DbConn) -> status::NoContent {
    db.run(move |c| {
        BazaRepository::delete(c, id).expect("Silme zamani xeta oldu");
        status::NoContent
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
