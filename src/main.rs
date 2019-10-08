#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;

use rocket_contrib::json::Json;
use serde_json::{Value, json};

mod hero;
use hero::{Hero, NewHero, IgnoreNoneFieldsUpdateHero};

pub mod schema;
pub mod db;

#[post("/", data = "<hero>")]
fn create(hero: Json<NewHero>, connection: db::Connection) -> Json<Hero> {
    let insert = NewHero { ..hero.into_inner() };
    Json(Hero::create(insert, &connection))
}

#[get("/")]
fn read(connection: db::Connection) -> Json<Value> {
    Json(json!(Hero::read(&connection)))
}

#[put("/<id>", data = "<hero>")]
fn update(id: i32, hero: Json<IgnoreNoneFieldsUpdateHero>, connection: db::Connection) -> Json<Value> {
    let update = IgnoreNoneFieldsUpdateHero { ..hero.into_inner() };
    Json(json!({
        "success": Hero::update(id, update, &connection)
    }))
}

#[delete("/<id>")]
fn delete(id: i32, connection: db::Connection) -> Json<Value> {
    Json(json!({
        "success": Hero::delete(id, &connection)
    }))
}

fn main() {
    rocket::ignite()
        .mount("/hero", routes![create, update, delete])
        .mount("/heroes", routes![read])
        .manage(db::connect())
        .launch();
}