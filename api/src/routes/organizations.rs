// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

extern crate rocket;

use rocket_contrib::json::Json;
use guard::db_conn::DbConn;

use pike_db as db;
use pike_db::models::Organization;

#[get("/organization/<id>")]
fn get_org(conn: DbConn, id: String) -> Option<Json<Organization>> {
    if let Ok(org) = db::get_org(&conn, &id) {
        Some(Json(org))
    } else {
        None
    }
}

#[get("/organization")]
fn get_orgs(conn: DbConn) -> Json<Vec<Organization>> {
    if let Ok(orgs) = db::get_orgs(&conn) {
        Json(orgs)
    } else {
        Json(vec![])
    }
}
