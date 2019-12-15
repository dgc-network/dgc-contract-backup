// Copyright 2018 dgc.network
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;

mod contract_registry;
mod error;
mod execute;
mod key;
mod namespace;
mod smart_permission;
mod submit;
mod transaction;
mod upload;

use std::time::Instant;

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn run() -> Result<(), error::CliError> {
    // Below, unwrap() is used on required arguments, since they will always
    // contain a value (and lack of value is should cause a panic). unwrap()
    // is also used on get_matches() because SubcommandRequiredElseHelp will
    // ensure a value.

    let matches = clap_app!(myapp =>
        (name: APP_NAME)
        (version: VERSION)
        (about: "Sawtooth Sabre CLI")
        (@setting SubcommandRequiredElseHelp)
        (@subcommand upload =>
            (about: "upload a Sabre contract")
            (@arg filename: -f --filename +required +takes_value "Path to Sabre contract definition (*.yaml)")
            (@arg key: -k --key +takes_value "Signing key name")
            (@arg url: --url +takes_value "URL to the Sawtooth REST API")
            (@arg wait: --wait +takes_value "A time in seconds to wait for batches to be committed")
            (@arg wasm: -w --wasm +takes_value "Path to compiled smart contract (*.wasm)")
        )
        (@subcommand exec =>
            (about: "execute a Sabre contract")
            (@arg contract: -C --contract +required +takes_value "Name:Version of a Sabre contract")
            (@arg payload: -p --payload +required +takes_value "Path to Sabre contract payload")
            (@arg key: -k --key +takes_value "Signing key name")
            (@arg url: --url +takes_value "URL to the Sawtooth REST API")
            (@arg inputs: --inputs +takes_value +multiple "Input addresses used by the contract")
            (@arg outputs: --outputs +takes_value +multiple "Output addresses used by the contract")
            (@arg wait: --wait +takes_value "A time in seconds to wait for batches to be committed")
        )
        (@subcommand ns =>
            (about: "create, update, or delete a Sabre namespace")
            (@group action =>
                (@arg create: -c --create "Create the namespace")
                (@arg update: -u --update "Update the namespace")
                (@arg delete: -d --delete "Delete the namespace")
            )
            (@arg namespace: +required "A global state address prefix (namespace)")
            (@arg key: -k --key +takes_value "Signing key name")
            (@arg url: -U --url +takes_value "URL to the Sawtooth REST API")
            (@arg owner: -O --owner +takes_value +multiple "Owner of this namespace")
            (@arg wait: --wait +takes_value "A time in seconds to wait for batches to be committed")
        )
        (@subcommand perm =>
            (about: "set or delete a Sabre namespace permission")
            (@arg namespace: +required "A global state address prefix (namespace)")
            (@arg contract: +required "Name of the contract")
            (@arg key: -k --key +takes_value "Signing key name")
            (@arg url: -U --url +takes_value "URL to the Sawtooth REST API")
            (@arg delete: -d --delete "Remove all permissions")
            (@arg read: -r --read conflicts_with[delete] "Set read permission")
            (@arg write: -w --write conflicts_with[delete] "Set write permission")
            (@arg wait: --wait +takes_value "A time in seconds to wait for batches to be committed")
        )
        (@subcommand cr =>
            (about: "create, update, or delete a Sabre contract registry")
            (@group action =>
                (@arg create: -c --create "Create the contract registry")
                (@arg update: -u --update "Update the contract registry")
                (@arg delete: -d --delete "Delete the contract registry")
            )
            (@arg name: +required "Name of the contracts in the registry")
            (@arg key: -k --key +takes_value "Signing key name")
            (@arg url: -U --url +takes_value "URL to the Sawtooth REST API")
            (@arg owner: -O --owner +takes_value +multiple "Owner of this contract registry")
            (@arg wait: --wait +takes_value "A time in seconds to wait for batches to be committed")
        )
        (@subcommand sp =>
          (about: "Create, update or delete smart permissions")
          (@arg url: -U --url +takes_value "URL to the Sawtooth REST API")
          (@arg wait: --wait +takes_value "A time in seconds to wait for batches to be committed")
          (@subcommand create =>
                (@arg org_id: +required "Organization ID ")
                (@arg name: +required "Name of the Smart Permission")
                (@arg filename: -f --filename +required +takes_value "Path to smart_permission")
                (@arg key: -k --key +takes_value "Signing key name")
            )
            (@subcommand update =>
                (@arg org_id: +required "Organization IDs")
                (@arg name: +required "Name of the Smart Permission")
                (@arg filename: -f --filename +required +takes_value "Path to smart_permission")
                (@arg key: -k --key +takes_value "Signing key name")
            )
            (@subcommand delete =>
                (@arg org_id: +required "Organization IDs")
                (@arg name: +required "Name of the Smart Permission")
                (@arg key: -k --key +takes_value "Signing key name")
            )
        )
    ).get_matches();

    let (batch_link, mut wait) = if let Some(upload_matches) = matches.subcommand_matches("upload")
    {
        upload(upload_matches)?
    } else if let Some(exec_matches) = matches.subcommand_matches("exec") {
        execute(exec_matches)?
    } else if let Some(ns_matches) = matches.subcommand_matches("ns") {
        namespace_registry(ns_matches)?
    } else if let Some(perm_matches) = matches.subcommand_matches("perm") {
        namespace_permission(perm_matches)?
    } else if let Some(cr_matches) = matches.subcommand_matches("cr") {
        contract_registry(cr_matches)?
    } else if let Some(sp_matches) = matches.subcommand_matches("sp") {
        smart_permission(sp_matches)?
    } else {
        return Err(error::CliError::UserError("Subcommand required".into()));
    };

    if wait > 0 {
        let response_body = loop {
            let time = Instant::now();
            let status_response = submit::wait_for_batch(&batch_link, wait)?;

            wait = wait.saturating_sub(time.elapsed().as_secs());

            if wait == 0 || status_response.is_finished() {
                break status_response;
            }
        };

        println!("Response Body:\n{}", response_body);
    }

    Ok(())
}

fn upload(upload_matches: &clap::ArgMatches) -> Result<(String, u64), error::CliError> {
    let filename = upload_matches.value_of("filename").unwrap();
    let key_name = upload_matches.value_of("key");
    let url = upload_matches
        .value_of("url")
        .unwrap_or("http://localhost:8008/");
    let wasm_name = upload_matches.value_of("wasm");

    let wait = match value_t!(upload_matches, "wait", u64) {
        Ok(wait) => wait,
        Err(err) => match err.kind {
            clap::ErrorKind::ArgumentNotFound => 0,
            _ => return Err(error::CliError::UserError("Wait must be an integer".into())),
        },
    };

    let batch_link = upload::do_upload(&filename, key_name, &url, wasm_name)?;
    Ok((batch_link, wait))
}

fn execute(exec_matches: &clap::ArgMatches) -> Result<(String, u64), error::CliError> {
    let contract = exec_matches.value_of("contract").unwrap();
    let payload = exec_matches.value_of("payload").unwrap();
    let key_name = exec_matches.value_of("key");
    let url = exec_matches
        .value_of("url")
        .unwrap_or("http://localhost:8008/");

    let wait = match value_t!(exec_matches, "wait", u64) {
        Ok(wait) => wait,
        Err(err) => match err.kind {
            clap::ErrorKind::ArgumentNotFound => 0,
            _ => return Err(error::CliError::UserError("Wait must be an integer".into())),
        },
    };

    let inputs = exec_matches
        .values_of("inputs")
        .map(|values| values.map(|v| v.into()).collect())
        .ok_or_else(|| {
            error::CliError::UserError("exec action requires one or more --inputs arguments".into())
        })?;

    let outputs = exec_matches
        .values_of("outputs")
        .map(|values| values.map(|v| v.into()).collect())
        .ok_or_else(|| {
            error::CliError::UserError(
                "exec action requires one or more --outputs arguments".into(),
            )
        })?;
    let (name, version) = match contract.split(':').collect::<Vec<_>>() {
        ref v if (v.len() == 1 || v.len() == 2) && v[0].is_empty() => Err(
            error::CliError::UserError("contract name must be specified".into()),
        ),
        ref v if v.len() == 1 || v.len() == 2 && v[1].is_empty() => Ok((v[0], "latest")),
        ref v if v.len() == 2 => Ok((v[0], v[1])),
        _ => Err(error::CliError::UserError(
            "malformed contract argument, may contain at most one ':'".into(),
        )),
    }?;

    let batch_link = execute::do_exec(&name, &version, &payload, inputs, outputs, key_name, &url)?;

    Ok((batch_link, wait))
}

fn namespace_registry(ns_matches: &clap::ArgMatches) -> Result<(String, u64), error::CliError> {
    let namespace = ns_matches.value_of("namespace").unwrap();

    let key_name = ns_matches.value_of("key");

    let url = ns_matches
        .value_of("url")
        .unwrap_or("http://localhost:8008/");

    let wait = match value_t!(ns_matches, "wait", u64) {
        Ok(wait) => wait,
        Err(err) => match err.kind {
            clap::ErrorKind::ArgumentNotFound => 0,
            _ => return Err(error::CliError::UserError("Wait must be an integer".into())),
        },
    };

    let owners = ns_matches
        .values_of("owner")
        .map(|values| values.map(|v| v.into()).collect());

    let batch_link = if ns_matches.is_present("update") {
        let o = owners.ok_or_else(|| {
            error::CliError::UserError(
                "update action requires one or more --owner arguments".into(),
            )
        })?;
        namespace::do_ns_update(key_name, &url, &namespace, o)?
    } else if ns_matches.is_present("delete") {
        if ns_matches.is_present("owner") {
            return Err(error::CliError::UserError(
                "arguments --delete and --owner conflict".into(),
            ));
        }
        namespace::do_ns_delete(key_name, &url, &namespace)?
    } else {
        let o = owners.ok_or_else(|| {
            error::CliError::UserError(
                "create action requires one or more --owner arguments".into(),
            )
        })?;
        namespace::do_ns_create(key_name, &url, &namespace, o)?
    };

    Ok((batch_link, wait))
}

fn namespace_permission(perm_matches: &clap::ArgMatches) -> Result<(String, u64), error::CliError> {
    let namespace = perm_matches.value_of("namespace").unwrap();
    let contract = perm_matches.value_of("contract").unwrap();
    let key_name = perm_matches.value_of("key");
    let url = perm_matches
        .value_of("url")
        .unwrap_or("http://localhost:8008/");

    let wait = match value_t!(perm_matches, "wait", u64) {
        Ok(wait) => wait,
        Err(err) => match err.kind {
            clap::ErrorKind::ArgumentNotFound => 0,
            _ => return Err(error::CliError::UserError("Wait must be an integer".into())),
        },
    };

    let batch_link = if perm_matches.is_present("delete") {
        namespace::do_perm_delete(key_name, &url, &namespace)?
    } else {
        let read = perm_matches.is_present("read");
        let write = perm_matches.is_present("write");

        if !(read || write) {
            return Err(error::CliError::UserError("no permissions provided".into()));
        }

        namespace::do_perm_create(key_name, &url, &namespace, &contract, read, write)?
    };

    Ok((batch_link, wait))
}

fn contract_registry(cr_matches: &clap::ArgMatches) -> Result<(String, u64), error::CliError> {
    let name = cr_matches.value_of("name").unwrap();

    let key_name = cr_matches.value_of("key");

    let url = cr_matches
        .value_of("url")
        .unwrap_or("http://localhost:8008/");

    let wait = value_t!(cr_matches, "wait", u64).unwrap_or(0);

    let owners = cr_matches
        .values_of("owner")
        .map(|values| values.map(|v| v.into()).collect());

    let batch_link = if cr_matches.is_present("update") {
        let o = owners.ok_or_else(|| {
            error::CliError::UserError(
                "update action requires one or more --owner arguments".into(),
            )
        })?;
        contract_registry::do_cr_update(key_name, &url, &name, o)?
    } else if cr_matches.is_present("delete") {
        if cr_matches.is_present("owner") {
            return Err(error::CliError::UserError(
                "arguments --delete and --owner conflict".into(),
            ));
        }
        contract_registry::do_cr_delete(key_name, &url, &name)?
    } else {
        let o = owners.ok_or_else(|| {
            error::CliError::UserError(
                "create action requires one or more --owner arguments".into(),
            )
        })?;
        contract_registry::do_cr_create(key_name, &url, &name, o)?
    };
    Ok((batch_link, wait))
}

fn smart_permission(sp_matches: &clap::ArgMatches) -> Result<(String, u64), error::CliError> {
    let url = sp_matches
        .value_of("url")
        .unwrap_or("http://localhost:8008/");

    let wait = match value_t!(sp_matches, "wait", u64) {
        Ok(wait) => wait,
        Err(err) => match err.kind {
            clap::ErrorKind::ArgumentNotFound => 0,
            _ => return Err(error::CliError::UserError("Wait must be an integer".into())),
        },
    };

    let batch_link = match sp_matches.subcommand() {
        ("create", Some(m)) => smart_permission::do_create(
            url,
            m.value_of("org_id").unwrap(),
            m.value_of("name").unwrap(),
            m.value_of("filename").unwrap(),
            m.value_of("key"),
        )?,
        ("update", Some(m)) => smart_permission::do_update(
            url,
            m.value_of("org_id").unwrap(),
            m.value_of("name").unwrap(),
            m.value_of("filename").unwrap(),
            m.value_of("key"),
        )?,
        ("delete", Some(m)) => smart_permission::do_delete(
            url,
            m.value_of("org_id").unwrap(),
            m.value_of("name").unwrap(),
            m.value_of("key"),
        )?,
        _ => {
            return Err(error::CliError::UserError(
                "Unrecognized smart permission subcommand".into(),
            ));
        }
    };

    Ok((batch_link, wait))
}

fn main() {
    if let Err(e) = run() {
        println!("{}", e);
        std::process::exit(1);
    }
}
