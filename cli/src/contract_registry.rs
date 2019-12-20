// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

use sabre_sdk::protocol::payload::{
    Action, CreateContractRegistryActionBuilder, DeleteContractRegistryActionBuilder,
    SabrePayloadBuilder, UpdateContractRegistryOwnersActionBuilder,
};
use sawtooth_sdk::signing;

use crate::error::CliError;
use crate::key;
use crate::submit::submit_batch_list;
use crate::transaction::{create_batch, create_batch_list_from_one, create_transaction};

pub fn do_cr_create(
    key_name: Option<&str>,
    url: &str,
    name: &str,
    owners: Vec<String>,
) -> Result<String, CliError> {
    let private_key = key::load_signing_key(key_name)?;
    let context = signing::create_context("secp256k1")?;
    let public_key = context.get_public_key(&private_key)?.as_hex();
    let factory = signing::CryptoFactory::new(&*context);
    let signer = factory.new_signer(&private_key);

    let action = CreateContractRegistryActionBuilder::new()
        .set_name(name.into())
        .set_owners(owners)
        .build()?;

    let payload = SabrePayloadBuilder::new()
        .set_action(Action::CreateContractRegistry(action))
        .build()?;

    let txn = create_transaction(payload, &signer, &public_key)?;
    let batch = create_batch(txn, &signer, &public_key)?;
    let batch_list = create_batch_list_from_one(batch);

    submit_batch_list(url, &batch_list)
}

pub fn do_cr_update(
    key_name: Option<&str>,
    url: &str,
    name: &str,
    owners: Vec<String>,
) -> Result<String, CliError> {
    let private_key = key::load_signing_key(key_name)?;
    let context = signing::create_context("secp256k1")?;
    let public_key = context.get_public_key(&private_key)?.as_hex();
    let factory = signing::CryptoFactory::new(&*context);
    let signer = factory.new_signer(&private_key);

    let action = UpdateContractRegistryOwnersActionBuilder::new()
        .set_name(name.into())
        .set_owners(owners)
        .build()?;

    let payload = SabrePayloadBuilder::new()
        .set_action(Action::UpdateContractRegistryOwners(action))
        .build()?;

    let txn = create_transaction(payload, &signer, &public_key)?;
    let batch = create_batch(txn, &signer, &public_key)?;
    let batch_list = create_batch_list_from_one(batch);

    submit_batch_list(url, &batch_list)
}

pub fn do_cr_delete(key_name: Option<&str>, url: &str, name: &str) -> Result<String, CliError> {
    let private_key = key::load_signing_key(key_name)?;
    let context = signing::create_context("secp256k1")?;
    let public_key = context.get_public_key(&private_key)?.as_hex();
    let factory = signing::CryptoFactory::new(&*context);
    let signer = factory.new_signer(&private_key);

    let action = DeleteContractRegistryActionBuilder::new()
        .set_name(name.into())
        .build()?;

    let payload = SabrePayloadBuilder::new()
        .set_action(Action::DeleteContractRegistry(action))
        .build()?;

    let txn = create_transaction(payload, &signer, &public_key)?;
    let batch = create_batch(txn, &signer, &public_key)?;
    let batch_list = create_batch_list_from_one(batch);

    submit_batch_list(url, &batch_list)
}
