#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

#[macro_use] extern crate rocket_sync_db_pools;

mod auth;
mod models;
mod schema;

use auth::BasicAuth;
use diesel::prelude::*;
use models::{Baza, NewBaza};
use schema::baza;
use rocket::serde::json::{json, Json, Value};
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
async fn view_baza(id: i32, _auth: BasicAuth, db: DbConn ) -> Value {
    db.run( move |c| {
        let baz = baza::table.find(id)
        .get_result::<Baza>(c)
        .expect("bazaya baxanda problem oldu");
    json!(baz)
    }).await
}

#[post("/baza", format = "json", data = "<new_baza>")]
async fn create_baza(_auth: BasicAuth, db: DbConn, new_baza: Json<NewBaza>) -> Value {
    db.run(|c| {
        let result = diesel::insert_into(baza::table)
        .values(new_baza.into_inner())
        .execute(c)
        .expect("Elave ederken problem oldu");
        json!(result)
    }).await
}

#[put("/baza/<id>", format = "json", data = "<baz>")]
async fn update_baza(id: i32, _auth: BasicAuth, db: DbConn, baz: Json<Baza>) -> Value {
    db.run(move |c|{
        let result = diesel::update(baza::table.find(id))
        .set((
            baza::email.eq(baz.email.to_owned()),
            baza::email.eq(baz.name.to_owned())
        ))
        .execute(c)
        .expect("Update eden zaman xeta oldu");
    json!(result)
    }).await
}


#[delete("/baza/<id>")]
async fn delete_baza(id: i32, _auth: BasicAuth, db: DbConn) -> status::NoContent{
    db.run(move |c|{
        diesel::delete(baza::table.find(id))
        .execute(c)
        .expect("");
        status::NoContent
    }).await
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
