// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

use smart_sdk::protocol::payload::{
    Action, CreateNamespaceRegistryActionBuilder, CreateNamespaceRegistryPermissionActionBuilder,
    DeleteNamespaceRegistryActionBuilder, DeleteNamespaceRegistryPermissionActionBuilder,
    SmartPayloadBuilder, UpdateNamespaceRegistryOwnersActionBuilder,
};
use sawtooth_sdk::signing;

use crate::error::CliError;
use crate::key;
use crate::submit::submit_batch_list;
use crate::transaction::{create_batch, create_batch_list_from_one, create_transaction};

pub fn do_ns_create(
    key_name: Option<&str>,
    url: &str,
    namespace: &str,
    owners: Vec<String>,
) -> Result<String, CliError> {
    let private_key = key::load_signing_key(key_name)?;
    let context = signing::create_context("secp256k1")?;
    let public_key = context.get_public_key(&private_key)?.as_hex();
    let factory = signing::CryptoFactory::new(&*context);
    let signer = factory.new_signer(&private_key);

    let action = CreateNamespaceRegistryActionBuilder::new()
        .set_namespace(namespace.into())
        .set_owners(owners)
        .build()?;

    let payload = SmartPayloadBuilder::new()
        .set_action(Action::CreateNamespaceRegistry(action))
        .build()?;

    let txn = create_transaction(payload, &signer, &public_key)?;
    let batch = create_batch(txn, &signer, &public_key)?;
    let batch_list = create_batch_list_from_one(batch);

    submit_batch_list(url, &batch_list)
}

pub fn do_ns_update(
    key_name: Option<&str>,
    url: &str,
    namespace: &str,
    owners: Vec<String>,
) -> Result<String, CliError> {
    let private_key = key::load_signing_key(key_name)?;
    let context = signing::create_context("secp256k1")?;
    let public_key = context.get_public_key(&private_key)?.as_hex();
    let factory = signing::CryptoFactory::new(&*context);
    let signer = factory.new_signer(&private_key);

    let action = UpdateNamespaceRegistryOwnersActionBuilder::new()
        .set_namespace(namespace.into())
        .set_owners(owners)
        .build()?;

    let payload = SmartPayloadBuilder::new()
        .set_action(Action::UpdateNamespaceRegistryOwners(action))
        .build()?;

    let txn = create_transaction(payload, &signer, &public_key)?;
    let batch = create_batch(txn, &signer, &public_key)?;
    let batch_list = create_batch_list_from_one(batch);

    submit_batch_list(url, &batch_list)
}

pub fn do_ns_delete(
    key_name: Option<&str>,
    url: &str,
    namespace: &str,
) -> Result<String, CliError> {
    let private_key = key::load_signing_key(key_name)?;
    let context = signing::create_context("secp256k1")?;
    let public_key = context.get_public_key(&private_key)?.as_hex();
    let factory = signing::CryptoFactory::new(&*context);
    let signer = factory.new_signer(&private_key);

    let action = DeleteNamespaceRegistryActionBuilder::new()
        .set_namespace(namespace.into())
        .build()?;

    let payload = SmartPayloadBuilder::new()
        .set_action(Action::DeleteNamespaceRegistry(action))
        .build()?;

    let txn = create_transaction(payload, &signer, &public_key)?;
    let batch = create_batch(txn, &signer, &public_key)?;
    let batch_list = create_batch_list_from_one(batch);

    submit_batch_list(url, &batch_list)
}

pub fn do_perm_create(
    key_name: Option<&str>,
    url: &str,
    namespace: &str,
    contract: &str,
    read: bool,
    write: bool,
) -> Result<String, CliError> {
    let private_key = key::load_signing_key(key_name)?;
    let context = signing::create_context("secp256k1")?;
    let public_key = context.get_public_key(&private_key)?.as_hex();
    let factory = signing::CryptoFactory::new(&*context);
    let signer = factory.new_signer(&private_key);

    let action = CreateNamespaceRegistryPermissionActionBuilder::new()
        .set_namespace(namespace.into())
        .set_contract_name(contract.into())
        .set_read(read)
        .set_write(write)
        .build()?;

    let payload = SmartPayloadBuilder::new()
        .set_action(Action::CreateNamespaceRegistryPermission(action))
        .build()?;

    let txn = create_transaction(payload, &signer, &public_key)?;
    let batch = create_batch(txn, &signer, &public_key)?;
    let batch_list = create_batch_list_from_one(batch);

    submit_batch_list(url, &batch_list)
}
pub fn do_perm_delete(
    key_name: Option<&str>,
    url: &str,
    namespace: &str,
) -> Result<String, CliError> {
    let private_key = key::load_signing_key(key_name)?;
    let context = signing::create_context("secp256k1")?;
    let public_key = context.get_public_key(&private_key)?.as_hex();
    let factory = signing::CryptoFactory::new(&*context);
    let signer = factory.new_signer(&private_key);

    let action = DeleteNamespaceRegistryPermissionActionBuilder::new()
        .set_namespace(namespace.into())
        .build()?;

    let payload = SmartPayloadBuilder::new()
        .set_action(Action::DeleteNamespaceRegistryPermission(action))
        .build()?;

    let txn = create_transaction(payload, &signer, &public_key)?;
    let batch = create_batch(txn, &signer, &public_key)?;
    let batch_list = create_batch_list_from_one(batch);

    submit_batch_list(url, &batch_list)
}
