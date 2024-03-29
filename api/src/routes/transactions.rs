// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

#![feature(proc_macro_hygiene, decl_macro)]

extern crate rocket;

use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;

use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::request::FromForm;
//use rocket::request::Form;

use guard::validator_conn::ValidatorConn;
use submit::{submit_batches, check_batch_status, BatchStatus};
use submit::TransactionError as error;

#[derive(FromForm)]
struct TxnQuery {
    wait: u32
}

//impl<'f> FromForm<'f> for TxnQuery {}

#[derive(FromForm)]
struct StatusQuery {
    wait: Option<u32>,
    ids: String
}

//impl<'f> FromForm<'f> for StatusQuery {}

#[post("/batches?<query>", format = "application/octet-stream", data = "<data>")]
pub fn submit_txns_wait(
    conn: ValidatorConn,
    data: Vec<u8>,
    //query: TxnQuery) -> Result<Custom<Json<Vec<BatchStatus>>>, Custom<Json>> {
    query: TxnQuery ///the trait `rocket::request::FromFormValue<'_>` is not implemented for `routes::transactions::TxnQuery`
) -> Result<Custom<Json<Vec<BatchStatus>>>, Custom<Json<String>>> {

    let batch_status_list = submit_batches(&mut conn.clone(), &data, query.wait)
        //.map_err(map_error)?;
        .map_err(map_error);

    if batch_status_list
        .iter()
        .all(|x| x.status == "COMMITTED") {
            ///unknown field
        Ok(Custom(Status::Created, Json(batch_status_list))) ///expected struct `std::vec::Vec`, found enum `std::result::Result`
    } else {
        Ok(Custom(Status::Accepted, Json(batch_status_list))) ///expected struct `std::vec::Vec`, found enum `std::result::Result`
    }
}

#[post("/batches", format = "application/octet-stream", data = "<data>")]
pub fn submit_txns(
    conn: ValidatorConn, 
    //data: Vec<u8>) -> Result<Json<Vec<BatchStatus>>, Custom<Json>> {
    data: Vec<u8>
) -> Result<Json<Vec<BatchStatus>>, Custom<Json<String>>> {
///expected `std::result::Result<rocket_contrib::json::Json<std::vec::Vec<submit::BatchStatus>>, rocket::response::status::Custom<rocket_contrib::json::Json<std::string::String>>>` because of return type
    submit_batches(&mut conn.clone(), &data, 0)
        .map_err(map_error)
        .and_then(|b| Ok(Json(b)))
}                       ///expected struct `rocket_contrib::json::Json`, found struct `rocket_contrib::json::JsonValue`
/*
#[get("/users/<id>")]
fn user(
    id: usize
) -> Json<User> {
    let user_from_id = User::from(id);
    Json(user_from_id)
}
*/
#[get("/batch_status?<query>")]
pub fn get_batch_status(
    conn: ValidatorConn,
    //query: StatusQuery) -> Result<Json<Vec<BatchStatus>>, Custom<Json>> {
    query: StatusQuery ///the trait `rocket::request::FromFormValue<'_>` is not implemented for `routes::transactions::StatusQuery`
) -> Result<Json<Vec<BatchStatus>>, Custom<Json<String>>> {
///expected `std::result::Result<rocket_contrib::json::Json<std::vec::Vec<submit::BatchStatus>>, rocket::response::status::Custom<rocket_contrib::json::Json<std::string::String>>>` because of return type
    let wait = query.wait.unwrap_or(0);
    let ids: Vec<String> = query.ids
        .split(",")
        .map(String::from)
        .collect();

    check_batch_status(&mut conn.clone(), ids, wait)
        .map_err(map_error)
        .and_then(|b| Ok(Json(b)))
}                           ///expected struct `rocket_contrib::json::Json`, found struct `rocket_contrib::json::JsonValue`

//fn map_error(err: error) -> Custom<Json> {
//fn map_error(err: error) -> Custom<Json<String>> {
fn map_error(err: error) -> Custom<JsonValue> {
    let message = Json(
        json!({
            "message": format!("{:?}", err)
        })
    );

    match err {
        error::BatchParseError(_) |
        error::InvalidBatch(_) |
        error::NoResource(_) |
        error::InvalidId(_) => Custom(Status::BadRequest, message), ///expected struct `rocket_contrib::json::JsonValue`, found struct `rocket_contrib::json::Json`
        _ => Custom(Status::InternalServerError, message) ///expected struct `rocket_contrib::json::JsonValue`, found struct `rocket_contrib::json::Json`
    }
}
