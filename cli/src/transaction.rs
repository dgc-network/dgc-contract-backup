// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

//! Contains functions which assist with the creation of Sabre Batches and
//! Transactions

use std::time::Instant;

use crypto::digest::Digest;
use crypto::sha2::Sha512;
use protobuf;
use protobuf::Message;
use sabre_sdk::protocol::payload::{Action, SabrePayload};
use sabre_sdk::protocol::ADMINISTRATORS_SETTING_ADDRESS;
use sabre_sdk::protos::IntoBytes;
use sawtooth_sdk::messages::batch::Batch;
use sawtooth_sdk::messages::batch::BatchHeader;
use sawtooth_sdk::messages::batch::BatchList;
use sawtooth_sdk::messages::transaction::Transaction;
use sawtooth_sdk::messages::transaction::TransactionHeader;
use sawtooth_sdk::signing::Signer;

use crate::error::CliError;

/// The Sawtooth Sabre transaction family name (sabre)
const SABRE_FAMILY_NAME: &str = "sabre";

/// The Sawtooth Sabre transaction family version (0.4)
const SABRE_FAMILY_VERSION: &str = "0.4";

/// The namespace registry prefix for global state (00ec00)
const NAMESPACE_REGISTRY_PREFIX: &str = "00ec00";

/// The contract registry prefix for global state (00ec01)
const CONTRACT_REGISTRY_PREFIX: &str = "00ec01";

/// The contract prefix for global state (00ec02)
const CONTRACT_PREFIX: &str = "00ec02";

/// The smart permission prefix for global state (00ec03)
const SMART_PERMISSION_PREFIX: &str = "00ec03";

const DGC_ACCOUNT_PREFIX: &str = "cad11d00";

const DGC_ORGANIZATION_PREFIX: &str = "cad11d01";

/// Creates a nonce appropriate for a TransactionHeader
fn create_nonce() -> String {
    let elapsed = Instant::now().elapsed();
    format!("{}{}", elapsed.as_secs(), elapsed.subsec_nanos())
}

/// Returns a hex string representation of the supplied bytes
///
/// # Arguments
///
/// * `b` - input bytes
fn bytes_to_hex_str(b: &[u8]) -> String {
    b.iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<_>>()
        .join("")
}

/// Returns a state address for a given namespace registry
///
/// # Arguments
///
/// * `namespace` - the address prefix for this namespace
fn compute_namespace_registry_address(namespace: &str) -> Result<String, CliError> {
    let prefix = match namespace.get(..6) {
        Some(x) => x,
        None => {
            return Err(CliError::UserError(format!(
                "Namespace must be at least 6 characters long: {}",
                namespace
            )));
        }
    };

    let hash: &mut [u8] = &mut [0; 64];

    let mut sha = Sha512::new();
    sha.input(prefix.as_bytes());
    sha.result(hash);

    Ok(String::from(NAMESPACE_REGISTRY_PREFIX) + &bytes_to_hex_str(hash)[..64])
}

/// Returns a state address for a given contract registry
///
/// # Arguments
///
/// * `name` - the name of the contract registry
fn compute_contract_registry_address(name: &str) -> String {
    let hash: &mut [u8] = &mut [0; 64];

    let mut sha = Sha512::new();
    sha.input(name.as_bytes());
    sha.result(hash);

    String::from(CONTRACT_REGISTRY_PREFIX) + &bytes_to_hex_str(hash)[..64]
}

/// Returns a state address for a given contract
///
/// # Arguments
///
/// * `name` - the name of the contract
/// * `version` - the version of the contract
fn compute_contract_address(name: &str, version: &str) -> String {
    let hash: &mut [u8] = &mut [0; 64];

    let s = String::from(name) + "," + version;

    let mut sha = Sha512::new();
    sha.input(s.as_bytes());
    sha.result(hash);

    String::from(CONTRACT_PREFIX) + &bytes_to_hex_str(hash)[..64]
}
/*
/// Returns a state address for a given agent name
///
/// # Arguments
///
/// * `name` - the agent's name
fn compute_agent_address(name: &str) -> String {
    let hash: &mut [u8] = &mut [0; 64];

    let mut sha = Sha512::new();
    sha.input(name.as_bytes());
    sha.result(hash);

    String::from(DGC_ACCOUNT_PREFIX) + &bytes_to_hex_str(hash)[..62]
}

/// Returns a state address for a given organization id
///
/// # Arguments
///
/// * `id` - the organization's id
fn compute_org_address(id: &str) -> String {
    let hash: &mut [u8] = &mut [0; 64];

    let mut sha = Sha512::new();
    sha.input(id.as_bytes());
    sha.result(hash);

    String::from(DGC_ORGANIZATION_PREFIX) + &bytes_to_hex_str(hash)[..62]
}
*/
/// Returns a state address for a given smart permission
///
/// # Arguments
///
/// * `org_id` - the organization's id
/// * `name` - smart permission name
fn compute_smart_permission_address(org_id: &str, name: &str) -> String {
    let mut sha_org_id = Sha512::new();
    sha_org_id.input(org_id.as_bytes());

    let mut sha_name = Sha512::new();
    sha_name.input(name.as_bytes());

    String::from(SMART_PERMISSION_PREFIX)
        + &sha_org_id.result_str()[..6].to_string()
        + &sha_name.result_str()[..58].to_string()
}

/// Returns a state address for a given account name
///
/// # Arguments
///
/// * `name` - the account's name
fn compute_account_address(name: &str) -> String {
    let hash: &mut [u8] = &mut [0; 64];

    let mut sha = Sha512::new();
    sha.input(name.as_bytes());
    sha.result(hash);

    String::from(DGC_ACCOUNT_PREFIX)
        + &bytes_to_hex_str(hash)[..62]
}

/// Returns a state address for a given organization id
///
/// # Arguments
///
/// * `id` - the organization's id
fn compute_org_address(id: &str) -> String {
    let hash: &mut [u8] = &mut [0; 64];

    let mut sha = Sha512::new();
    sha.input(id.as_bytes());
    sha.result(hash);

    String::from(DGC_ORGANIZATION_PREFIX)
        + &bytes_to_hex_str(hash)[..62]
}

/// Returns a Transaction for the given Payload and Signer
///
/// # Arguments
///
/// * `payload` - a fully populated identity payload
/// * `signer` - the signer to be used to sign the transaction
/// * `public_key` - the public key associated with the signer
///
/// # Errors
///
/// If an error occurs during serialization of the provided payload or
/// internally created `TransactionHeader`, a `CliError::ProtobufError` is
/// returned.
///
/// If a signing error occurs, a `CliError::SigningError` is returned.
pub fn create_transaction(
    payload: SabrePayload,
    signer: &Signer,
    public_key: &str,
) -> Result<Transaction, CliError> {
    let mut txn = Transaction::new();
    let mut txn_header = TransactionHeader::new();

    txn_header.set_family_name(String::from(SABRE_FAMILY_NAME));
    txn_header.set_family_version(String::from(SABRE_FAMILY_VERSION));
    txn_header.set_nonce(create_nonce());
    txn_header.set_signer_public_key(public_key.to_string());
    txn_header.set_batcher_public_key(public_key.to_string());

    let (input_addresses, output_addresses) = match payload.action() {
        Action::CreateContract(create_contract) => {
            let name = create_contract.get_name();
            let version = create_contract.get_version();

            let addresses = vec![
                compute_contract_registry_address(name),
                compute_contract_address(name, version),
            ];

            (addresses.clone(), addresses)
        }
        Action::DeleteContract(delete_contract) => {
            let name = delete_contract.get_name();
            let version = delete_contract.get_version();

            let addresses = vec![
                compute_contract_registry_address(name),
                compute_contract_address(name, version),
            ];

            (addresses.clone(), addresses)
        }
        Action::ExecuteContract(execute_contract) => {
            let name = execute_contract.get_name();
            let version = execute_contract.get_version();

            let mut input_addresses = vec![
                compute_contract_registry_address(name),
                compute_contract_address(name, version),
            ];
            for input in execute_contract.get_inputs() {
                let namespace = match input.get(..6) {
                    Some(namespace) => namespace,
                    None => {
                        return Err(CliError::UserError(format!(
                            "Input must be at least 6 characters long: {}",
                            input
                        )));
                    }
                };

                input_addresses.push(compute_namespace_registry_address(namespace)?);
            }
            input_addresses.append(&mut execute_contract.get_inputs().to_vec());

            let mut output_addresses = vec![
                compute_contract_registry_address(name),
                compute_contract_address(name, version),
            ];

            for output in execute_contract.get_outputs() {
                let namespace = match output.get(..6) {
                    Some(namespace) => namespace,
                    None => {
                        return Err(CliError::UserError(format!(
                            "Output must be at least 6 characters long: {}",
                            output
                        )));
                    }
                };

                output_addresses.push(compute_namespace_registry_address(namespace)?);
            }
            output_addresses.append(&mut execute_contract.get_outputs().to_vec());

            (input_addresses, output_addresses)
        }
        Action::CreateContractRegistry(create_contract_registry) => {
            let name = create_contract_registry.get_name();
            let addresses = vec![
                compute_contract_registry_address(name),
                ADMINISTRATORS_SETTING_ADDRESS.into(),
            ];
            (addresses.clone(), addresses)
        }
        Action::DeleteContractRegistry(delete_contract_registry) => {
            let name = delete_contract_registry.get_name();
            let addresses = vec![
                compute_contract_registry_address(name),
                ADMINISTRATORS_SETTING_ADDRESS.into(),
            ];
            (addresses.clone(), addresses)
        }
        Action::UpdateContractRegistryOwners(update_contract_registry_owners) => {
            let name = update_contract_registry_owners.get_name();
            let addresses = vec![
                compute_contract_registry_address(name),
                ADMINISTRATORS_SETTING_ADDRESS.into(),
            ];
            (addresses.clone(), addresses)
        }
        Action::CreateNamespaceRegistry(create_namespace_registry) => {
            let namespace = create_namespace_registry.get_namespace();
            let addresses = vec![
                compute_namespace_registry_address(namespace)?,
                ADMINISTRATORS_SETTING_ADDRESS.into(),
            ];
            (addresses.clone(), addresses)
        }
        Action::DeleteNamespaceRegistry(delete_namespace_registry) => {
            let namespace = delete_namespace_registry.get_namespace();
            let addresses = vec![
                compute_namespace_registry_address(namespace)?,
                ADMINISTRATORS_SETTING_ADDRESS.into(),
            ];
            (addresses.clone(), addresses)
        }
        Action::UpdateNamespaceRegistryOwners(update_namespace_registry_owners) => {
            let namespace = update_namespace_registry_owners.get_namespace();
            let addresses = vec![
                compute_namespace_registry_address(namespace)?,
                ADMINISTRATORS_SETTING_ADDRESS.into(),
            ];
            (addresses.clone(), addresses)
        }
        Action::CreateNamespaceRegistryPermission(create_namespace_registry_permission) => {
            let namespace = create_namespace_registry_permission.get_namespace();
            let addresses = vec![
                compute_namespace_registry_address(namespace)?,
                ADMINISTRATORS_SETTING_ADDRESS.into(),
            ];
            (addresses.clone(), addresses)
        }
        Action::DeleteNamespaceRegistryPermission(delete_namespace_registry_permission) => {
            let namespace = delete_namespace_registry_permission.get_namespace();
            let addresses = vec![
                compute_namespace_registry_address(namespace)?,
                ADMINISTRATORS_SETTING_ADDRESS.into(),
            ];
            (addresses.clone(), addresses)
        }
        Action::CreateSmartPermission(create_smart_permission) => {
            let org_id = create_smart_permission.get_org_id();
            let name = create_smart_permission.get_name();
            let addresses = vec![
                compute_smart_permission_address(org_id, name),
                compute_org_address(org_id),
                compute_account_address(public_key),
            ];

            (addresses.clone(), addresses)
        }
        Action::UpdateSmartPermission(update_smart_permission) => {
            let org_id = update_smart_permission.get_org_id();
            let name = update_smart_permission.get_name();
            let addresses = vec![
                compute_smart_permission_address(org_id, name),
                compute_org_address(org_id),
                compute_account_address(public_key),
            ];

            (addresses.clone(), addresses)
        }
        Action::DeleteSmartPermission(delete_smart_permission) => {
            let org_id = delete_smart_permission.get_org_id();
            let name = delete_smart_permission.get_name();
            let addresses = vec![
                compute_smart_permission_address(org_id, name),
                compute_org_address(org_id),
                compute_account_address(public_key),
            ];

            (addresses.clone(), addresses)
        }
        Action::CreateAccount(create_account) => {
            let org_id = create_account.get_org_id();
            let account_public_key = create_account.get_public_key();
            let addresses = vec![
                compute_account_address(org_id),
                compute_account_address(account_public_key),
                compute_account_address(public_key),
            ];

            (addresses.clone(), addresses)
        }
        Action::UpdateAccount(update_account) => {
            let org_id = update_account.get_org_id();
            let account_public_key = update_account.get_public_key();
            let addresses = vec![
                compute_account_address(org_id),
                compute_account_address(account_public_key),
                compute_account_address(public_key),
            ];

            (addresses.clone(), addresses)
        }
        Action::CreateOrganization(create_organization) => {
            let org_id = create_organization.get_id();
            let addresses = vec![
                compute_org_address(org_id),
                compute_account_address(public_key),
            ];

            (addresses.clone(), addresses)
        }
        Action::UpdateOrganization(update_organization) => {
            let org_id = update_organization.get_id();
            let addresses = vec![
                compute_org_address(org_id),
                compute_account_address(public_key),
            ];

            (addresses.clone(), addresses)
        }
    };

    txn_header.set_inputs(protobuf::RepeatedField::from_vec(input_addresses));
    txn_header.set_outputs(protobuf::RepeatedField::from_vec(output_addresses));

    let payload_bytes = payload.into_bytes()?;
    let mut sha = Sha512::new();
    sha.input(&payload_bytes);
    let hash: &mut [u8] = &mut [0; 64];
    sha.result(hash);
    txn_header.set_payload_sha512(bytes_to_hex_str(hash));
    txn.set_payload(payload_bytes);

    let txn_header_bytes = txn_header.write_to_bytes()?;
    txn.set_header(txn_header_bytes.clone());

    let b: &[u8] = &txn_header_bytes;
    txn.set_header_signature(signer.sign(b)?);

    Ok(txn)
}

/// Returns a Batch for the given Transaction and Signer
///
/// # Arguments
///
/// * `txn` - a Transaction
/// * `signer` - the signer to be used to sign the transaction
/// * `public_key` - the public key associated with the signer
///
/// # Errors
///
/// If an error occurs during serialization of the provided Transaction or
/// internally created `BatchHeader`, a `CliError::ProtobufError` is
/// returned.
///
/// If a signing error occurs, a `CliError::SigningError` is returned.
pub fn create_batch(
    txn: Transaction,
    signer: &Signer,
    public_key: &str,
) -> Result<Batch, CliError> {
    let mut batch = Batch::new();
    let mut batch_header = BatchHeader::new();

    batch_header.set_transaction_ids(protobuf::RepeatedField::from_vec(vec![txn
        .header_signature
        .clone()]));
    batch_header.set_signer_public_key(public_key.to_string());
    batch.set_transactions(protobuf::RepeatedField::from_vec(vec![txn]));

    let batch_header_bytes = batch_header.write_to_bytes()?;
    batch.set_header(batch_header_bytes.clone());

    let b: &[u8] = &batch_header_bytes;
    batch.set_header_signature(signer.sign(b)?);

    Ok(batch)
}

/// Returns a BatchList containing the provided Batch
///
/// # Arguments
///
/// * `batch` - a Batch
pub fn create_batch_list_from_one(batch: Batch) -> BatchList {
    let mut batch_list = BatchList::new();
    batch_list.set_batches(protobuf::RepeatedField::from_vec(vec![batch]));
    batch_list
}
