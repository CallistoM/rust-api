#![feature(plugin)]
#![plugin(rocket_codegen)]
#![allow(proc_macro_derive_resolution_fallback)]

extern crate rocket;
extern crate rocket_cors;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
extern crate chrono;

use rocket_contrib::{Json, Value};
use rocket_cors::{AllowedHeaders, AllowedOrigins};

mod db;
mod schema;

mod olc;

// models
use self::olc::OlcIntern;
use self::olc::OlcExtern;
use self::olc::GPS;

use rocket::http::Method;


#[post("/", data = "<olc>")]
fn intern_create(olc: Json<OlcIntern>, connection: db::Connection) -> Json<OlcIntern> {
    let insert = OlcIntern { id: None, ..olc.into_inner() };
    Json(OlcIntern::create(insert, &connection))
}

#[get("/")]
fn intern_read(connection: db::Connection) -> Json<Value> {
    Json(json!(OlcIntern::read(&connection)))
}

#[put("/<id>", data = "<olc>")]
fn intern_update(id: i32, olc: Json<OlcIntern>, connection: db::Connection) -> Json<Value> {
    let update = OlcIntern { id: Some(id), ..olc.into_inner() };
    Json(json!({
        "success": OlcIntern::update(id, update, &connection)
    }))
}

#[delete("/<id>")]
fn intern_delete(id: i32, connection: db::Connection) -> Json<Value> {
    Json(json!({
        "success": OlcIntern::delete(id, &connection)
    }))
}

#[post("/", data = "<olc>")]
fn extern_create(olc: Json<OlcExtern>, connection: db::Connection) -> Json<OlcExtern> {
    let insert = OlcExtern { id: None, ..olc.into_inner() };
    Json(OlcExtern::create(insert, &connection))
}

#[get("/")]
fn extern_read(connection: db::Connection) -> Json<Value> {
    Json(json!(OlcExtern::read(&connection)))
}

#[put("/<id>", data = "<olc>")]
fn extern_update(id: i32, olc: Json<OlcExtern>, connection: db::Connection) -> Json<Value> {
    let update = OlcExtern { id: Some(id), ..olc.into_inner() };
    Json(json!({
        "success": OlcExtern::update(id, update, &connection)
    }))
}

#[delete("/<id>")]
fn extern_delete(id: i32, connection: db::Connection) -> Json<Value> {
    Json(json!({
        "success": OlcExtern::delete(id, &connection)
    }))
}

#[post("/", data = "<olc>")]
fn gps_create(olc: Json<GPS>, connection: db::Connection) -> Json<GPS> {
    let insert = GPS { id: None, ..olc.into_inner() };
    Json(GPS::create(insert, &connection))
}

#[get("/")]
fn gps_read(connection: db::Connection) -> Json<Value> {
    let mut vec = Vec::new();
    for tt in GPS::read(&connection) {
        let f = GPS {
            id: tt.id,
            latitude: tt.latitude,
            longitude: tt.longitude,
            altitude: tt.altitude,
            accuracy: tt.accuracy,
            satellites: tt.satellites
        };
        vec.push(f);
    }

    Json(json!(vec))
}

#[put("/<id>", data = "<olc>")]
fn gps_update(id: i32, olc: Json<GPS>, connection: db::Connection) -> Json<Value> {
    let update = GPS { id: Some(id), ..olc.into_inner() };
    Json(json!({
        "success": GPS::update(id, update, &connection)
    }))
}

#[delete("/<id>")]
fn gps_delete(id: i32, connection: db::Connection) -> Json<Value> {
    Json(json!({
        "success": GPS::delete(id, &connection)
    }))
}

fn main() {
    let (allowed_origins, failed_origins) = AllowedOrigins::some(&["http://localhost:4200"]);

    assert!(failed_origins.is_empty());

    let options = rocket_cors::Cors {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Options].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    };

    rocket::ignite()
        .manage(db::connect())
        .mount("/olc/internal_temperature", routes![intern_create, intern_update, intern_delete])
        .mount("/olcs/internal_temperature", routes![intern_read])
        .mount("/olc/external_temperature", routes![extern_create, extern_update, extern_delete])
        .mount("/olcs/external_temperature", routes![extern_read])
        .mount("/olc/gps", routes![gps_create, gps_update, gps_delete])
        .mount("/olcs/gps", routes![gps_read])
        .attach(options)
        .launch();
}