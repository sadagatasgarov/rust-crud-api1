#[macro_use]
extern crate rocket;

mod auth;

use auth::BasicAuth;

use rocket::serde::json::{json, Value};
/*
use rocket::response::status;
*/

#[get("/")]
fn hello() -> Value {
    json!("Hello, world!")
}

#[get("/baza")]
fn get_baza(auth: BasicAuth) -> Value {
    json!([
        {
            "id": 1,
            "name": "Sada Asga"
        },

        {
            "id": 2,
            "name": "Sakh Asga2"
        }
    ])
}

#[get("/baza/<id>")]
fn view_baza(id: i32) -> Value {
    json!([
        {
            "id": id,
            "name": "Sada Asga",
            "email": "sadagatasgarov@gmil.com"
        }
    ])
}

#[post("/baza", format = "json")]
fn create_baza() -> Value {
    json!([
        {
            "id": 3,
            "name": "Sada2 Asga2",
            "email": "sadagatasgarov@gmil.com"
        }
    ])
}

#[put("/baza/<id>", format = "json")]
fn update_baza(id: i32) -> Value {
    json!([
        {
            "id": id,
            "name": "Sada2 Asga2",
            "email": "sadagatasgarov@gmil.com"
        }
    ])
}


#[delete("/baza/<id>")]
fn delete_baza(id: i32) -> Value {
    json!([
        {
            "id": id,
            "name": "Sada2 Asga2",
            "email": "sadagatasgarov@gmil.com"
        }
    ])
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
                hello,
                get_baza,
                view_baza,
                create_baza,
                update_baza,
                delete_baza
            ],
        )
        .register("/", catchers![not_found])
        .launch()
        .await;
}
