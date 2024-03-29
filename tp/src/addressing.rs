// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

use crypto::digest::Digest;
use crypto::sha2::Sha512;
use sawtooth_sdk::processor::handler::ApplyError;

/// The namespace registry prefix for global state (00ec00)
const NAMESPACE_REGISTRY_PREFIX: &str = "00ec00";

/// The contract registry prefix for global state (00ec01)
const CONTRACT_REGISTRY_PREFIX: &str = "00ec01";

/// The contract prefix for global state (00ec02)
const CONTRACT_PREFIX: &str = "00ec02";

/// The smart permission prefix for global state (00ec03)
const SMART_PERMISSION_PREFIX: &str = "00ec03";

const SMART_ACCOUNT_PREFIX: &str = "cad11d00";

const SMART_ORG_PREFIX: &str = "cad11d01";

pub fn hash(to_hash: &str, num: usize) -> Result<String, ApplyError> {
    let mut sha = Sha512::new();
    sha.input_str(to_hash);
    let temp = sha.result_str().to_string();
    let hash = match temp.get(..num) {
        Some(x) => x,
        None => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Cannot hash {} to Sha512 and return String with len {}",
                to_hash, num
            )));
        }
    };
    Ok(hash.into())
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

pub fn make_contract_address(name: &str, version: &str) -> Result<String, ApplyError> {
    Ok(CONTRACT_PREFIX.to_string() + &hash(&(name.to_string() + "," + version), 64)?)
}

pub fn make_contract_registry_address(name: &str) -> Result<String, ApplyError> {
    Ok(CONTRACT_REGISTRY_PREFIX.to_string() + &hash(name, 64)?)
}

pub fn make_namespace_registry_address(namespace: &str) -> Result<String, ApplyError> {
    let prefix = match namespace.get(..6) {
        Some(x) => x,
        None => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Namespace must be at least 6 characters long: {}",
                namespace
            )));
        }
    };
    Ok(NAMESPACE_REGISTRY_PREFIX.to_string() + &hash(prefix, 64)?)
}

/// Returns a state address for a smart permission
///
/// # Arguments
///
/// * `name` - smart permission name
/// * `org_id - ID of the organization that owns the smart permission`
pub fn compute_smart_permission_address(org_id: &str, name: &str) -> String {
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
pub fn compute_account_address(name: &str) -> String {
    let hash: &mut [u8] = &mut [0; 64];

    let mut sha = Sha512::new();
    sha.input(name.as_bytes());
    sha.result(hash);

    String::from(SMART_ACCOUNT_PREFIX) + &bytes_to_hex_str(hash)[..62]
}

/// Returns a state address for a given organization id
///
/// # Arguments
///
/// * `id` - the organization's id
pub fn compute_org_address(name: &str) -> String {
    let hash: &mut [u8] = &mut [0; 64];

    let mut sha = Sha512::new();
    sha.input(name.as_bytes());
    sha.result(hash);

    String::from(SMART_ORG_PREFIX) + &bytes_to_hex_str(hash)[..62]
}
