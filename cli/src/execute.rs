// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use sabre_sdk::protocol::payload::{
    Action, ExecuteContractActionBuilder, SabrePayload, SabrePayloadBuilder,
};
use sawtooth_sdk::signing;

use crate::error::CliError;
use crate::key;
use crate::submit::submit_batch_list;
use crate::transaction::{create_batch, create_batch_list_from_one, create_transaction};

pub fn do_exec(
    name: &str,
    version: &str,
    payload_file: &str,
    inputs: Vec<String>,
    outputs: Vec<String>,
    key_name: Option<&str>,
    url: &str,
) -> Result<String, CliError> {
    let private_key = key::load_signing_key(key_name)?;
    let context = signing::create_context("secp256k1")?;
    let public_key = context.get_public_key(&private_key)?.as_hex();
    let factory = signing::CryptoFactory::new(&*context);
    let signer = factory.new_signer(&private_key);

    let contract_payload = load_contract_payload_file(payload_file)?;

    let txn_payload = create_exec_txn_payload(name, version, inputs, outputs, contract_payload)?;

    let txn = create_transaction(txn_payload, &signer, &public_key)?;
    let batch = create_batch(txn, &signer, &public_key)?;
    let batch_list = create_batch_list_from_one(batch);

    submit_batch_list(url, &batch_list)
}

fn create_exec_txn_payload(
    name: &str,
    version: &str,
    inputs: Vec<String>,
    outputs: Vec<String>,
    contract_payload: Vec<u8>,
) -> Result<SabrePayload, CliError> {
    let exec_contract = ExecuteContractActionBuilder::new()
        .with_name(name.into())
        .with_version(version.into())
        .with_inputs(inputs)
        .with_outputs(outputs)
        .with_payload(contract_payload)
        .build()?;

    let payload = SabrePayloadBuilder::new()
        .with_action(Action::ExecuteContract(exec_contract))
        .build()?;

    Ok(payload)
}

fn load_contract_payload_file(payload_file: &str) -> Result<Vec<u8>, CliError> {
    let file = File::open(payload_file).map_err(|e| {
        CliError::UserError(format!(
            "Could not load payload \"{}\": {}",
            payload_file, e
        ))
    })?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = Vec::new();
    buf_reader.read_to_end(&mut contents).map_err(|e| {
        CliError::UserError(format!(
            "IoError while reading payload \"{}\": {}",
            payload_file, e
        ))
    })?;

    Ok(contents)
}
