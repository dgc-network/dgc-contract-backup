// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

#![feature(proc_macro_hygiene, decl_macro)]

//#[macro_use] extern crate rocket;
/*
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
*/
extern crate rocket;

//use rocket::request::Form;
use rocket_contrib::json::Json;
use rocket::http::Status;
use rocket::response::status::Custom;
use guard::validator_conn::ValidatorConn;
use submit::{submit_batches, check_batch_status, BatchStatus};
use submit::TransactionError as error;

#[derive(FromForm)]
struct TxnQuery {
    wait: u32
}

#[derive(FromForm)]
struct StatusQuery {
    wait: Option<u32>,
    ids: String
}
/*
// In a `GET` request and all other non-payload supporting request types, the
// preferred media type in the Accept header is matched against the `format` in
// the route attribute. Note: if this was a real application, we'd use
// `rocket_contrib`'s built-in JSON support and return a `JsonValue` instead.
#[get("/<name>/<age>", format = "json")]
fn get_hello(name: String, age: u8) -> Json<String> {
    // NOTE: In a real application, we'd use `rocket_contrib::json::Json`.
    let person = Person { name: name, age: age, };
    Json(serde_json::to_string(&person).unwrap())
}

// In a `POST` request and all other payload supporting request types, the
// content type is matched against the `format` in the route attribute.
//
// Note that `content::Json` simply sets the content-type to `application/json`.
// In a real application, we wouldn't use `serde_json` directly; instead, we'd
// use `contrib::Json` to automatically serialize a type into JSON.
#[post("/<age>", format = "plain", data = "<name_data>")]
fn post_hello(age: u8, name_data: Data) -> Result<Json<String>, Debug<io::Error>> {
    let mut name = String::with_capacity(32);
    name_data.open().take(32).read_to_string(&mut name)?;
    let person = Person { name: name, age: age, };
    // NOTE: In a real application, we'd use `rocket_contrib::json::Json`.
    Ok(Json(serde_json::to_string(&person).expect("valid JSON")))
}
*/
#[post("/batches?<query>", format = "application/octet-stream", data = "<data>")]
pub fn submit_txns_wait(
    conn: ValidatorConn,
    data: Vec<u8>,
    //query: TxnQuery) -> Result<Custom<Json<Vec<BatchStatus>>>, Custom<Json>> {
    query: TxnQuery) -> Result<Json<Vec<BatchStatus>>, Json> {

    let batch_status_list = submit_batches(&mut conn.clone(), &data, query.wait)
        .map_err(map_error)?;

    if batch_status_list
            .iter()
            .all(|x| x.status == "COMMITTED") {

        Ok(Custom(Status::Created, Json(batch_status_list)))
    } else {
        Ok(Custom(Status::Accepted, Json(batch_status_list)))
    }
}

#[post("/batches", format = "application/octet-stream", data = "<data>")]
pub fn submit_txns(
    conn: ValidatorConn, 
    data: Vec<u8>) -> Result<Json<Vec<BatchStatus>>, Custom<Json>> {

    submit_batches(&mut conn.clone(), &data, 0)
        .map_err(map_error)
        .and_then(|b| Ok(Json(b)))
}

#[get("/batch_status?<query>")]
pub fn get_batch_status(
    conn: ValidatorConn,
    query: StatusQuery) -> Result<Json<Vec<BatchStatus>>, Custom<Json>> {

    let wait = query.wait.unwrap_or(0);
    let ids: Vec<String> = query.ids
        .split(",")
        .map(String::from)
        .collect();

    check_batch_status(&mut conn.clone(), ids, wait)
        .map_err(map_error)
        .and_then(|b| Ok(Json(b)))
}

fn map_error(err: error) -> Custom<Json> {
    let message = Json(
        json!({
            "message": format!("{:?}", err)
        })
    );

    match err {
        error::BatchParseError(_) |
        error::InvalidBatch(_) |
        error::NoResource(_) |
        error::InvalidId(_) => Custom(Status::BadRequest, message),
        _ => Custom(Status::InternalServerError, message)
    }
}
