// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

extern crate rocket;

use rocket_contrib::Json;
use guard::db_conn::DbConn;

use pike_db as db;
use pike_db::models::Agent;

#[get("/account/<publickey>")]
fn get_account(conn: DbConn, publickey: String) -> Option<Json<Agent>> {
    if let Ok(account) = db::get_account(&conn, &publickey) {
        Some(Json(account))
    } else {
        None
    }
}

#[get("/account")]
fn get_accounts(conn: DbConn) -> Json<Vec<Agent>> {
    if let Ok(accounts) = db::get_accounts(&conn) {
        Json(accounts)
    } else {
        Json(vec![])
    }
}
