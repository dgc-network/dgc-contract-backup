// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

#![feature(proc_macro_hygiene, decl_macro)]
#![feature(rustc_private)]

#[macro_use] extern crate rocket;
extern crate rocket_cors;
#[macro_use] extern crate rocket_contrib;
//extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate serde_yaml;
extern crate serde_json;
//extern crate pike_db;
extern crate sawtooth_sdk;
extern crate protobuf;
extern crate uuid;

mod openapi;
mod routes;
mod guard;
mod submit;
#[cfg(test)] mod tests;

use std::env;
use rocket::http::Method;
//use rocket_cors::{AllowedOrigins, AllowedHeaders};
use rocket_cors::{AllowedOrigins, AllowedHeaders, Error};

//use routes::{accounts, organizations};
//use pike_db::pools;
//use routes::transactions;

use sawtooth_sdk::messaging::zmq_stream::ZmqMessageConnection;
/*
#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}
*/
use rocket::Request;

#[catch(500)]
fn internal_error() -> &'static str {
    "Whoops! Looks like we messed up."
}

#[catch(400)]
fn not_found(req: &Request) -> String {
    format!("I couldn't find '{}'. Try something else?", req.uri())
}
/*
fn main() {
    rocket::ignite()
        .register(catchers![internal_error, not_found])
}

#[error(404)]
fn not_found(_: &rocket::Request) -> Json {
    Json(json!({
        "message": "Not Found"
    }))
}

#[error(500)]
fn internal_server_error(_: &rocket::Request) -> Json {
    Json(json!({
        "message": "Internal Server Error"
    }))
}
*/
/*
//fn main() {
fn main() -> Result<(), Error> {

    //let (allowed_origins, failed_origins) = AllowedOrigins::some(&["http://localhost:9002"]);
    //assert!(failed_origins.is_empty());

    //let options = rocket_cors::Cors {
    let options = rocket_cors::CorsOptions {
        //allowed_origins: allowed_origins,
        allowed_origins: AllowedOrigins::all(),
        allowed_methods: vec![Method::Get, Method::Post, Method::Options].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept", "Content-Type"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors();
   
    let database_url = if let Ok(s) = env::var("DATABASE_URL") {
        s
    } else {
        "postgres://localhost:5432".into()
    };

    let validator_url = if let Ok(s) = env::var("VALIDATOR_URL") {
       s
    } else {
        "tcp://localhost:8004".into()
    };

    rocket::ignite()
        .mount("/", routes![
               hello,
               //openapi::openapi_json,
               //openapi::openapi_yaml,
               //accounts::get_account,
               //accounts::get_accounts,
               //organizations::get_org,
               //organizations::get_orgs,
               //transactions::submit_txns,
               //transactions::submit_txns_wait,
               //transactions::get_batch_status
               ])
        //.manage(pools::init_pg_pool(database_url))
        .manage(ZmqMessageConnection::new(&validator_url))
        .attach(options)
        //.catch(errors![not_found, internal_server_error])
        .register(catchers![internal_error, not_found])
        .launch();
}
*/

#[get("/")]
//fn cors<'a>() -> &'a str {
fn hello<'a>() -> &'a str {
    "Hello, world!"
}

fn main() -> Result<(), Error> {
    //let allowed_origins = AllowedOrigins::some_exact(&["https://www.acme.com"]);

    // You can also deserialize this
    let cors = rocket_cors::CorsOptions {
        //allowed_origins,
        //allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        //allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allowed_origins: AllowedOrigins::all(),
        allowed_methods: vec![Method::Get, Method::Post, Method::Options].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept", "Content-Type"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()?;

    let validator_url = if let Ok(s) = env::var("VALIDATOR_URL") {
        s
     } else {
         "tcp://localhost:8004".into()
     };
 
     rocket::ignite()
        .mount("/", routes![
            //cors
            hello
        ])
        .attach(cors)
        .manage(ZmqMessageConnection::new(&validator_url))
        //.attach(options)
        //.catch(errors![not_found, internal_server_error])
        .register(catchers![internal_error, not_found])
        .launch();

    Ok(())
}
