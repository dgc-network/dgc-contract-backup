// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

extern crate dgc_contract_api;
extern crate serde;
#[macro_use] extern crate rouille;

use dgc_contract_api::{Task, TaskManager};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

/// External representation of a single task.
#[derive(Serialize)]
struct GetTaskResponse<'a> {
    text: &'a str,
    done: bool,
}

impl<'a> GetTaskResponse<'a> {
    /// Constructs a new external version of a task given a task's definition.
    fn from(t: &'a Task) -> Self {
        Self { text: &t.text, done: t.done }
    }
}

/// Representation of a request to update zero or more fields of a task.
#[derive(Deserialize)]
struct UpdateTaskRequest {
    text: Option<String>,
    done: Option<bool>,
}

/// Processes REST requests for the task manager API and transforms them into
/// operations against the given backing `task_manager`.
fn route_request(request: &rouille::Request, task_manager: &Mutex<TaskManager>)
    -> rouille::Response {

    let mut task_manager = task_manager.lock().unwrap();

    router!(request,
        (GET) ["/task"] => {
            let mut response = HashMap::new();
            for (id, task) in task_manager.all().iter() {
                let path = format!("/task/{}", id);
                response.insert(path, GetTaskResponse::from(task));
            }
            rouille::Response::json(&response)
        },

        (POST) ["/task"] => {
            let body: String =
                try_or_400!(rouille::input::json_input(request));
            let id = task_manager.add(body);
            rouille::Response::json(&format!("/task/{}", id))
        },

        (GET) ["/task/{id}", id: usize] => {
            match task_manager.get(id) {
                Ok(task) =>
                    rouille::Response::json(&GetTaskResponse::from(task)),
                Err(e) => rouille::Response::json(&e).with_status_code(404),
            }
        },

        (UPDATE) ["/task/{id}", id: usize] => {
            let body: UpdateTaskRequest =
                try_or_400!(rouille::input::json_input(request));
            match task_manager.set(id, body.text, body.done) {
                Ok(()) => rouille::Response::empty_204(),
                Err(e) => rouille::Response::json(&e).with_status_code(404),
            }
        },

        (DELETE) ["/task/{id}", id: usize] => {
            match task_manager.delete(id) {
                Ok(()) => rouille::Response::empty_204(),
                Err(e) => rouille::Response::json(&e).with_status_code(404),
            }
        },

        _ => rouille::Response::empty_404()
    )
}

fn main() {
    let task_manager = Mutex::from(TaskManager::new());
    rouille::start_server(
        "localhost:3000", move |request| route_request(request, &task_manager))
}

/*
#![feature(async_await, futures_api, await_macro)]

#[macro_use]
extern crate serde_derive;

use tide::{error::ResultExt, response, App, Context, EndpointResult};
use http::status::StatusCode;
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Clone)]
struct User {
    name: String,
    height: u32,
}

#[derive(Default)]
struct Database {
    contents: Mutex<Vec<User>>,
}

impl Database {
    fn insert(&self, user: User) -> usize {
        let mut table = self.contents.lock().unwrap();
        table.push(user);
        table.len() - 1
    }

    fn get_all(&self) -> Vec<User> {
        self.contents.lock().unwrap().clone()
    }

    fn get(&self, id: usize) -> Option<User> {
        self.contents.lock().unwrap().get(id).cloned()
    }

    fn set(&self, id: usize, user: User) -> bool {
        let mut table = self.contents.lock().unwrap();

        if let Some(old_user) = table.get_mut(id) {
            *old_user = user;
            true
        } else {
            false
        }
    }

    fn delete(&self, id: usize) -> bool {
        let mut table = self.contents.lock().unwrap();

        if let Some(_user) = table.get_mut(id) {
            self.contents.lock().unwrap().remove(id);
            true
        } else {
            false
        }
    }
}

async fn handle_get_users(cx: Context<Database>) -> EndpointResult {
    Ok(response::json(cx.app_data().get_all()))
}

async fn handle_get_user(cx: Context<Database>) -> EndpointResult {
    let id = cx.param("id").client_err()?;
    if let Some(user) = cx.app_data().get(id) {
        Ok(response::json(user))
    } else {
        Err(StatusCode::NOT_FOUND)?
    }
}

async fn handle_update_user(mut cx: Context<Database>) -> EndpointResult<()> {
    let user = await!(cx.body_json()).client_err()?;
    let id = cx.param("id").client_err()?;

    if cx.app_data().set(id, user) {
        Ok(())
    } else {
        Err(StatusCode::NOT_FOUND)?
    }
}

async fn handle_create_user(mut cx: Context<Database>) -> EndpointResult<String> {
    let user = await!(cx.body_json()).client_err()?;
    Ok(cx.app_data().insert(user).to_string())
}

async fn handle_delete_user(cx: Context<Database>) -> EndpointResult<String> {
    let id = cx.param("id").client_err()?;
    Ok(cx.app_data().delete(id).to_string())
}

fn main() {
    let mut app = App::new(Database::default());
    app.at("/users")
        .post(handle_create_user)
        .get(handle_get_users);
    app.at("/users/:id")
        .get(handle_get_user)
        .patch(handle_update_user)
        .delete(handle_delete_user);

    //app.serve("127.0.0.1:8000").unwrap();
    app.serve("0.0.0.0:3000").unwrap();
}
*/

/*
//#![feature(plugin, decl_macro, custom_derive)]
#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_cors;
#[macro_use] extern crate rocket_contrib;
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
use rocket_cors::{AllowedOrigins, AllowedHeaders};
use rocket_contrib::Json;
use routes::{accounts, organizations};
//use pike_db::pools;
use routes::transactions;

use sawtooth_sdk::messaging::zmq_stream::ZmqMessageConnection;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
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

fn main() {
    let (allowed_origins, failed_origins) = AllowedOrigins::some(&["http://localhost:9002"]);
    assert!(failed_origins.is_empty());

    let options = rocket_cors::Cors {
        allowed_origins: allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Options].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept", "Content-Type"]),
        allow_credentials: true,
        ..Default::default()
    };

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
               openapi::openapi_json,
               openapi::openapi_yaml,
               //accounts::get_account,
               //accounts::get_accounts,
               //organizations::get_org,
               //organizations::get_orgs,
               transactions::submit_txns,
               transactions::submit_txns_wait,
               transactions::get_batch_status])
        .manage(pools::init_pg_pool(database_url))
        .manage(ZmqMessageConnection::new(&validator_url))
        .attach(options)
        .catch(errors![not_found, internal_server_error])
        .launch();
}
*/