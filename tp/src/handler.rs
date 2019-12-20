// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

//! Provides a Sawtooth Transaction Handler for executing Sabre transactions.

use crypto::digest::Digest;
use crypto::sha2::Sha512;
use sawtooth_sdk::messages::processor::TpProcessRequest;
use sawtooth_sdk::processor::handler::ApplyError;
use sawtooth_sdk::processor::handler::TransactionContext;
use sawtooth_sdk::processor::handler::TransactionHandler;

use crate::payload::SabreRequestPayload;
use crate::state::SabreState;
use crate::wasm_executor::wasm_module::WasmModule;
//use crate::protocol::state::{
use sabre_sdk::protocol::state::{
    ContractBuilder, ContractRegistry, ContractRegistryBuilder, NamespaceRegistry,
    NamespaceRegistryBuilder, PermissionBuilder, SmartPermissionBuilder, VersionBuilder,
    AccountBuilder, OrganizationBuilder,
};
//use crate::protocol::payload::{
use sabre_sdk::protocol::payload::{
    Action, CreateContractAction, CreateContractRegistryAction, CreateNamespaceRegistryAction,
    CreateNamespaceRegistryPermissionAction, CreateSmartPermissionAction, DeleteContractAction,
    DeleteContractRegistryAction, DeleteNamespaceRegistryAction,
    DeleteNamespaceRegistryPermissionAction, DeleteSmartPermissionAction, ExecuteContractAction,
    UpdateContractRegistryOwnersAction, UpdateNamespaceRegistryOwnersAction,
    UpdateSmartPermissionAction,
    CreateAccountAction, UpdateAccountAction,
    CreateOrganizationAction, UpdateOrganizationAction,
};
//use crate::protocol::ADMINISTRATORS_SETTING_KEY;
use sabre_sdk::protocol::ADMINISTRATORS_SETTING_KEY;

/// The namespace registry prefix for global state (00ec00)
const NAMESPACE_REGISTRY_PREFIX: &str = "00ec00";

/// The contract registry prefix for global state (00ec01)
const CONTRACT_REGISTRY_PREFIX: &str = "00ec01";

/// The contract prefix for global state (00ec02)
const CONTRACT_PREFIX: &str = "00ec02";

/// Handles Sabre Transactions
///
/// This handler implements the Sawtooth TransactionHandler trait, in order to execute Sabre
/// transaction payloads.  These payloads include on-chain smart contracts executed in a
/// WebAssembly virtual machine.
///
/// WebAssembly (Wasm) is a stack-based virtual machine newly implemented in major browsers. It is
/// well-suited for the purposes of smart contract execution due to its sandboxed design, growing
/// popularity, and tool support.
pub struct SabreTransactionHandler {
    family_name: String,
    family_versions: Vec<String>,
    namespaces: Vec<String>,
}

impl SabreTransactionHandler {
    /// Constructs a new SabreTransactionHandler
    #[allow(clippy::new_without_default)]
    pub fn new() -> SabreTransactionHandler {
        SabreTransactionHandler {
            family_name: "sabre".into(),
            family_versions: vec!["0.4".into()],
            namespaces: vec![
                NAMESPACE_REGISTRY_PREFIX.into(),
                CONTRACT_REGISTRY_PREFIX.into(),
                CONTRACT_PREFIX.into(),
            ],
        }
    }
}

impl TransactionHandler for SabreTransactionHandler {
    fn family_name(&self) -> String {
        self.family_name.clone()
    }

    fn family_versions(&self) -> Vec<String> {
        self.family_versions.clone()
    }

    fn namespaces(&self) -> Vec<String> {
        self.namespaces.clone()
    }

    fn apply(
        &self,
        request: &TpProcessRequest,
        context: &mut dyn TransactionContext,
    ) -> Result<(), ApplyError> {
        let payload = SabreRequestPayload::new(request.get_payload());

        let payload = match payload {
            Err(e) => return Err(e),
            Ok(payload) => payload,
        };
        let payload = match payload {
            Some(x) => x,
            None => {
                return Err(ApplyError::InvalidTransaction(String::from(
                    "Request must contain a payload",
                )));
            }
        };

        let signer = request.get_header().get_signer_public_key();
        let mut state = SabreState::new(context);

        info!(
            "{} {:?} {:?}",
            payload.get_action(),
            request.get_header().get_inputs(),
            request.get_header().get_outputs()
        );

        match payload.get_action() {
            Action::CreateContract(create_contract_payload) => {
                create_contract(create_contract_payload, signer, &mut state)
            }
            Action::DeleteContract(delete_contract_payload) => {
                delete_contract(delete_contract_payload, signer, &mut state)
            }
            Action::ExecuteContract(execute_contract_payload) => execute_contract(
                execute_contract_payload,
                signer,
                request.get_signature(),
                &mut state,
            ),
            Action::CreateContractRegistry(create_contract_registry_payload) => {
                create_contract_registry(create_contract_registry_payload, signer, &mut state)
            }
            Action::DeleteContractRegistry(delete_contract_registry_payload) => {
                delete_contract_registry(delete_contract_registry_payload, signer, &mut state)
            }
            Action::UpdateContractRegistryOwners(update_contract_registry_owners_payload) => {
                update_contract_registry_owners(
                    update_contract_registry_owners_payload,
                    signer,
                    &mut state,
                )
            }
            Action::CreateNamespaceRegistry(create_namespace_registry_payload) => {
                create_namespace_registry(create_namespace_registry_payload, signer, &mut state)
            }
            Action::DeleteNamespaceRegistry(delete_namespace_registry_payload) => {
                delete_namespace_registry(delete_namespace_registry_payload, signer, &mut state)
            }
            Action::UpdateNamespaceRegistryOwners(update_namespace_registry_owners_payload) => {
                update_namespace_registry_owners(
                    update_namespace_registry_owners_payload,
                    signer,
                    &mut state,
                )
            }
            Action::CreateNamespaceRegistryPermission(
                create_namespace_registry_permission_payload,
            ) => create_namespace_registry_permission(
                create_namespace_registry_permission_payload,
                signer,
                &mut state,
            ),
            Action::DeleteNamespaceRegistryPermission(
                delete_namespace_registry_permission_payload,
            ) => delete_namespace_registry_permission(
                delete_namespace_registry_permission_payload,
                signer,
                &mut state,
            ),
            Action::CreateSmartPermission(payload) => {
                create_smart_permission(payload, signer, &mut state)
            }
            Action::UpdateSmartPermission(payload) => {
                update_smart_permission(payload, signer, &mut state)
            }
            Action::DeleteSmartPermission(payload) => {
                delete_smart_permission(payload, signer, &mut state)
            }
            Action::CreateAccount(payload) => {
                create_account(payload, signer, &mut state)
            }
            Action::UpdateAccount(payload) => {
                update_account(payload, signer, &mut state)
            }
            Action::CreateOrganization(payload) => {
                create_organization(payload, signer, &mut state)
            }
            Action::UpdateOrganization(payload) => {
                update_organization(payload, signer, &mut state)
            }
        }
    }
}

fn create_contract(
    payload: CreateContractAction,
    signer: &str,
    state: &mut SabreState,
) -> Result<(), ApplyError> {
    let name = payload.get_name();
    let version = payload.get_version();
    match state.get_contract(name, version) {
        Ok(None) => (),
        Ok(Some(_)) => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Contract already exists: {}, {}",
                name, version,
            )));
        }
        Err(err) => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Unable to check state: {}",
                err,
            )));
        }
    };

    // update or create the contract registry for the contract
    let contract_registry = match state.get_contract_registry(name) {
        Ok(None) => {
            return Err(ApplyError::InvalidTransaction(format!(
                "The Contract Registry does not exist: {}",
                name,
            )));
        }
        Ok(Some(contract_registry)) => contract_registry,
        Err(err) => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Unable to check state: {}",
                err,
            )));
        }
    };

    if !contract_registry.get_owners().contains(&signer.into()) {
        return Err(ApplyError::InvalidTransaction(format!(
            "Only owners can submit new versions of contracts: {}",
            signer,
        )));
    }

    let contract = ContractBuilder::new()
        .set_name(name.into())
        .set_version(version.into())
        .set_inputs(payload.get_inputs().to_vec())
        .set_outputs(payload.get_outputs().to_vec())
        .set_creator(signer.into())
        .set_contract(payload.get_contract().to_vec())
        .build()
        .map_err(|_| ApplyError::InvalidTransaction(String::from("Cannot build contract")))?;

    state.set_contract(name, version, contract)?;

    let mut sha = Sha512::new();
    sha.input(payload.get_contract());

    let contract_registry_version = VersionBuilder::new()
        .set_version(version.into())
        .set_contract_sha512(sha.result_str())
        .set_creator(signer.into())
        .build()
        .map_err(|_| {
            ApplyError::InvalidTransaction(String::from("Cannot build contract version"))
        })?;

    let mut versions = contract_registry.get_versions().to_vec();
    versions.push(contract_registry_version);

    let contract_registry = contract_registry
        .into_builder()
        .set_versions(versions)
        .build()
        .map_err(|_| {
            ApplyError::InvalidTransaction(String::from("Cannot build contract registry"))
        })?;

    state.set_contract_registry(name, contract_registry)
}

fn delete_contract(
    payload: DeleteContractAction,
    signer: &str,
    state: &mut SabreState,
) -> Result<(), ApplyError> {
    let name = payload.get_name();
    let version = payload.get_version();

    match state.get_contract(name, version) {
        Ok(Some(_)) => (),
        Ok(_) => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Contract does not exist: {}, {}",
                name, version,
            )));
        }
        Err(err) => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Unable to check state: {}",
                err,
            )));
        }
    };

    // update the contract registry for the contract
    let contract_registry = match state.get_contract_registry(name) {
        Ok(None) => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Contract Registry does not exist {}",
                name,
            )));
        }
        Ok(Some(contract_registry)) => contract_registry,
        Err(err) => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Unable to check state: {}",
                err,
            )));
        }
    };

    if !(contract_registry.get_owners().contains(&signer.into())) {
        return Err(ApplyError::InvalidTransaction(format!(
            "Signer is not an owner of this contract: {}",
            signer,
        )));
    }
    let mut versions = contract_registry.get_versions().to_vec();
    for (index, contract_registry_version) in versions.iter().enumerate() {
        if contract_registry_version.get_version() == version {
            versions.remove(index);
            break;
        }
    }

    let contract_registry = contract_registry
        .into_builder()
        .set_versions(versions)
        .build()
        .map_err(|_| {
            ApplyError::InvalidTransaction(String::from("Cannot build contract registry"))
        })?;

    state.set_contract_registry(name, contract_registry)?;
    state.delete_contract(name, version)
}

fn execute_contract(
    payload: ExecuteContractAction,
    signer: &str,
    signature: &str,
    state: &mut SabreState,
) -> Result<(), ApplyError> {
    let name = payload.get_name();
    let version = payload.get_version();

    let contract = match state.get_contract(name, version) {
        Ok(Some(contract)) => contract,
        Ok(None) => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Contract does not exist: {}, {}",
                name, version,
            )));
        }
        Err(err) => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Unable to check state: {}",
                err,
            )));
        }
    };

    for input in payload.get_inputs() {
        let namespace = match input.get(..6) {
            Some(namespace) => namespace,
            None => {
                return Err(ApplyError::InvalidTransaction(format!(
                    "Input must have at least 6 characters: {}",
                    input,
                )));
            }
        };
        let registries = match state.get_namespace_registries(namespace) {
            Ok(Some(registries)) => registries,
            Ok(None) => {
                return Err(ApplyError::InvalidTransaction(format!(
                    "Namespace Registry does not exist: {}",
                    namespace,
                )));
            }
            Err(err) => {
                return Err(ApplyError::InvalidTransaction(format!(
                    "Unable to check state: {}",
                    err,
                )));
            }
        };

        let mut namespace_registry = None;
        for registry in registries.get_registries() {
            if input.starts_with(registry.get_namespace()) {
                namespace_registry = Some(registry)
            }
        }

        let mut permissioned = false;
        match namespace_registry {
            Some(registry) => {
                for permission in registry.permissions() {
                    if name == permission.get_contract_name() && permission.get_read() {
                        permissioned = true;
                        break;
                    }
                }
                if !permissioned {
                    return Err(ApplyError::InvalidTransaction(format!(
                        "Contract does not have permission to read from state : {} {}",
                        name, input
                    )));
                }
            }
            None => {
                return Err(ApplyError::InvalidTransaction(format!(
                    "No namespace registry exists for namespace: {} input: {}",
                    namespace, input
                )));
            }
        }
    }

    for output in payload.get_outputs() {
        let namespace = match output.get(..6) {
            Some(namespace) => namespace,
            None => {
                return Err(ApplyError::InvalidTransaction(format!(
                    "Output must have at least 6 characters: {}",
                    output,
                )));
            }
        };
        let registries = match state.get_namespace_registries(namespace) {
            Ok(Some(registries)) => registries,
            Ok(None) => {
                return Err(ApplyError::InvalidTransaction(format!(
                    "Namespace Registry does not exist: {}",
                    namespace,
                )));
            }
            Err(err) => {
                return Err(ApplyError::InvalidTransaction(format!(
                    "Unable to check state: {}",
                    err,
                )));
            }
        };

        let mut namespace_registry = None;
        for registry in registries.get_registries() {
            if output.starts_with(registry.get_namespace()) {
                namespace_registry = Some(registry)
            }
        }
        let mut permissioned = false;
        match namespace_registry {
            Some(registry) => {
                for permission in registry.permissions() {
                    if name == permission.get_contract_name() && permission.get_write() {
                        permissioned = true;
                        break;
                    }
                }
                if !permissioned {
                    return Err(ApplyError::InvalidTransaction(format!(
                        "Contract does not have permission to write to state: {}, {}",
                        name, output
                    )));
                }
            }
            None => {
                return Err(ApplyError::InvalidTransaction(format!(
                    "No namespace registry exists for namespace: {} output: {}",
                    namespace, output
                )));
            }
        }
    }

    let mut module = WasmModule::new(contract.get_contract(), state.context())
        .expect("Failed to create can_add module");

    let result = module
        .entrypoint(payload.get_payload().to_vec(), signer.into(), signature.into())
        .map_err(|e| ApplyError::InvalidTransaction(format!("{:?}", e)))?;

    match result {
        None => Err(ApplyError::InvalidTransaction(format!(
            "Wasm contract did not return a result: {}, {}",
            name, version,
        ))),
        Some(1) => Ok(()),
        Some(-3) => Err(ApplyError::InvalidTransaction(format!(
            "Wasm contract returned invalid transaction: {}, {}",
            name, version,
        ))),
        Some(num) => Err(ApplyError::InternalError(format!(
            "Wasm contract returned internal error: {}",
            num
        ))),
    }
}

fn create_contract_registry(
    payload: CreateContractRegistryAction,
    signer: &str,
    state: &mut SabreState,
) -> Result<(), ApplyError> {
    let name = payload.get_name();

    match state.get_contract_registry(name) {
        Ok(None) => (),
        Ok(Some(_)) => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Contract Registry already exists: {}",
                name,
            )));
        }
        Err(err) => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Unable to check state: {}",
                err,
            )));
        }
    };

    let setting = match state.get_admin_setting() {
        Ok(Some(setting)) => setting,
        Ok(None) => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Only admins can create a contract registry: {}",
                signer,
            )));
        }
        Err(err) => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Unable to check state: {}",
                err,
            )));
        }
    };

    for entry in setting.get_entries() {
        if entry.key == ADMINISTRATORS_SETTING_KEY {
            let values = entry.value.split(',');
            let value_vec: Vec<&str> = values.collect();
            if !value_vec.contains(&signer) {
                return Err(ApplyError::InvalidTransaction(format!(
                    "Only admins can create a contract registry: {}",
                    signer,
                )));
            }
        }
    }

    let contract_registry = ContractRegistryBuilder::new()
        .set_name(name.into())
        .set_owners(payload.get_owners().to_vec())
        .build()
        .map_err(|_| {
            ApplyError::InvalidTransaction(String::from("Cannot build contract registry"))
        })?;

    state.set_contract_registry(name, contract_registry)
}

fn delete_contract_registry(
    payload: DeleteContractRegistryAction,
    signer: &str,
    state: &mut SabreState,
) -> Result<(), ApplyError> {
    let name = payload.get_name();
    let contract_registry = match state.get_contract_registry(name) {
        Ok(None) => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Contract Registry does not exist: {}",
                name,
            )));
        }
        Ok(Some(contract_registry)) => contract_registry,
        Err(err) => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Unable to check state: {}",
                err,
            )));
        }
    };

    if !contract_registry.get_versions().is_empty() {
        return Err(ApplyError::InvalidTransaction(format!(
            "Contract Registry can only be deleted if there are no versions: {}",
            name,
        )));
    }

    // Check if signer is an owner or an admin
    can_update_contract_registry(contract_registry.clone(), signer, state)?;

    state.delete_contract_registry(name)
}

fn update_contract_registry_owners(
    payload: UpdateContractRegistryOwnersAction,
    signer: &str,
    state: &mut SabreState,
) -> Result<(), ApplyError> {
    let name = payload.get_name();
    let contract_registry = match state.get_contract_registry(name) {
        Ok(None) => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Contract Registry does not exist: {}",
                name,
            )));
        }
        Ok(Some(contract_registry)) => contract_registry,
        Err(err) => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Unable to check state: {}",
                err,
            )));
        }
    };

    // Check if signer is an owner or an admin
    can_update_contract_registry(contract_registry.clone(), signer, state)?;

    let contract_registry = contract_registry
        .into_builder()
        .set_owners(payload.get_owners().to_vec())
        .build()
        .map_err(|_| {
            ApplyError::InvalidTransaction(String::from("Cannot build contract registry"))
        })?;

    state.set_contract_registry(name, contract_registry)
}

fn create_namespace_registry(
    payload: CreateNamespaceRegistryAction,
    signer: &str,
    state: &mut SabreState,
) -> Result<(), ApplyError> {
    let namespace = payload.get_namespace();

    if namespace.len() < 6 {
        return Err(ApplyError::InvalidTransaction(format!(
            "Namespace must be at least 6 characters: {}",
            namespace,
        )));
    }

    match state.get_namespace_registry(namespace) {
        Ok(None) => (),
        Ok(Some(_)) => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Namespace Registry already exists: {}",
                namespace,
            )));
        }
        Err(err) => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Unable to check state: {}",
                err,
            )));
        }
    }

    let setting = match state.get_admin_setting() {
        Ok(Some(setting)) => setting,
        Ok(None) => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Only admins can create a namespace registry: {}",
                signer,
            )));
        }
        Err(err) => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Unable to check state: {}",
                err,
            )));
        }
    };

    for entry in setting.get_entries() {
        if entry.key == ADMINISTRATORS_SETTING_KEY {
            let values = entry.value.split(',');
            let value_vec: Vec<&str> = values.collect();
            if !value_vec.contains(&signer) {
                return Err(ApplyError::InvalidTransaction(format!(
                    "Only admins can create a namespace registry: {}",
                    signer,
                )));
            }
        }
    }

    let namespace_registry = NamespaceRegistryBuilder::new()
        .set_namespace(namespace.into())
        .set_owners(payload.get_owners().to_vec())
        .build()
        .map_err(|_| {
            ApplyError::InvalidTransaction(String::from("Cannot build namespace registry"))
        })?;

    state.set_namespace_registry(namespace, namespace_registry)
}

fn delete_namespace_registry(
    payload: DeleteNamespaceRegistryAction,
    signer: &str,
    state: &mut SabreState,
) -> Result<(), ApplyError> {
    let namespace = payload.get_namespace();

    let namespace_registry = match state.get_namespace_registry(namespace) {
        Ok(None) => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Namespace Registry does not exist: {}",
                namespace,
            )));
        }
        Ok(Some(namespace_registry)) => namespace_registry,
        Err(err) => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Unable to check state: {}",
                err,
            )));
        }
    };
    can_update_namespace_registry(namespace_registry.clone(), signer, state)?;

    if !namespace_registry.get_permissions().is_empty() {
        return Err(ApplyError::InvalidTransaction(format!(
            "Namespace Registry can only be deleted if there are no permissions: {}",
            namespace,
        )));
    }
    state.delete_namespace_registry(namespace)
}

fn update_namespace_registry_owners(
    payload: UpdateNamespaceRegistryOwnersAction,
    signer: &str,
    state: &mut SabreState,
) -> Result<(), ApplyError> {
    let namespace = payload.get_namespace();

    let namespace_registry = match state.get_namespace_registry(namespace) {
        Ok(None) => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Namespace Registry does not exist: {}",
                namespace,
            )));
        }
        Ok(Some(namespace_registry)) => namespace_registry,
        Err(err) => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Unable to check state: {}",
                err,
            )));
        }
    };

    // Check if signer is an owner or an admin
    can_update_namespace_registry(namespace_registry.clone(), signer, state)?;
    let namespace_registry = namespace_registry
        .into_builder()
        .set_owners(payload.get_owners().to_vec())
        .build()
        .map_err(|_| {
            ApplyError::InvalidTransaction(String::from("Cannot build namespace registry"))
        })?;

    state.set_namespace_registry(namespace, namespace_registry)
}

fn create_namespace_registry_permission(
    payload: CreateNamespaceRegistryPermissionAction,
    signer: &str,
    state: &mut SabreState,
) -> Result<(), ApplyError> {
    let namespace = payload.get_namespace();
    let contract_name = payload.get_contract_name();
    let namespace_registry = match state.get_namespace_registry(namespace) {
        Ok(None) => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Namespace Registry does not exist: {}",
                namespace,
            )));
        }
        Ok(Some(namespace_registry)) => namespace_registry,
        Err(err) => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Unable to check state: {}",
                err,
            )));
        }
    };
    // Check if signer is an owner or an admin
    can_update_namespace_registry(namespace_registry.clone(), signer, state)?;

    let new_permission = PermissionBuilder::new()
        .set_contract_name(contract_name.into())
        .set_read(payload.get_read())
        .set_write(payload.get_write())
        .build()
        .map_err(|_| {
            ApplyError::InvalidTransaction(String::from(
                "Cannot build namespace registry permission",
            ))
        })?;

    // remove old permission for contract if one exists and replace with the new permission
    let mut permissions = namespace_registry.get_permissions().to_vec();
    let mut index = None;
    for (count, permission) in permissions.iter().enumerate() {
        if permission.get_contract_name() == contract_name {
            index = Some(count);
            break;
        }
    }

    if let Some(x) = index {
        permissions.remove(x);
    }

    permissions.push(new_permission);

    let namespace_registry = namespace_registry
        .into_builder()
        .set_permissions(permissions)
        .build()
        .map_err(|_| {
            ApplyError::InvalidTransaction(String::from("Cannot build namespace registry"))
        })?;

    state.set_namespace_registry(namespace, namespace_registry)
}

fn delete_namespace_registry_permission(
    payload: DeleteNamespaceRegistryPermissionAction,
    signer: &str,
    state: &mut SabreState,
) -> Result<(), ApplyError> {
    let namespace = payload.get_namespace();
    let contract_name = payload.get_contract_name();

    let namespace_registry = match state.get_namespace_registry(namespace) {
        Ok(None) => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Namespace Registry does not exist: {}",
                namespace,
            )));
        }
        Ok(Some(namespace_registry)) => namespace_registry,
        Err(err) => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Unable to check state: {}",
                err,
            )));
        }
    };
    // Check if signer is an owner or an admin
    can_update_namespace_registry(namespace_registry.clone(), signer, state)?;

    // remove old permission for contract
    let mut permissions = namespace_registry.get_permissions().to_vec();
    let mut index = None;
    for (count, permission) in permissions.iter().enumerate() {
        if permission.get_contract_name() == contract_name {
            index = Some(count);
            break;
        }
    }

    match index {
        Some(x) => {
            permissions.remove(x);
        }
        None => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Namespace Registry does not have a permission for : {}",
                contract_name,
            )));
        }
    };

    let namespace_registry = namespace_registry
        .into_builder()
        .set_permissions(permissions)
        .build()
        .map_err(|_| {
            ApplyError::InvalidTransaction(String::from("Cannot build namespace registry"))
        })?;
    state.set_namespace_registry(namespace, namespace_registry)
}

pub(crate) fn is_admin(
    signer: &str,
    org_id: &str,
    state: &mut SabreState,
) -> Result<(), ApplyError> {
    let admin = match state.get_account(signer) {
        Ok(None) => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Signer is not an agent: {}",
                signer,
            )));
        }
        Ok(Some(admin)) => admin,
        Err(err) => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Failed to retrieve state: {}",
                err,
            )));
        }
    };

    if admin.get_org_id() != org_id {
        return Err(ApplyError::InvalidTransaction(format!(
            "Signer is not associated with the organization: {}",
            signer,
        )));
    }
    if !admin.get_roles().contains(&"admin".to_string()) {
        return Err(ApplyError::InvalidTransaction(format!(
            "Signer is not an admin: {}",
            signer,
        )));
    };

    if !admin.get_active() {
        return Err(ApplyError::InvalidTransaction(format!(
            "Admin is not currently an active agent: {}",
            signer,
        )));
    }
    Ok(())
}

fn create_smart_permission(
    payload: CreateSmartPermissionAction,
    signer: &str,
    state: &mut SabreState,
) -> Result<(), ApplyError> {
    // verify the signer of the transaction is authorized to create smart permissions
    is_admin(signer, payload.get_org_id(), state)?;

    // Check if the smart permissions already exists
    match state.get_smart_permission(payload.get_org_id(), payload.get_name()) {
        Ok(None) => (),
        Ok(Some(_)) => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Smart Permission already exists: {} ",
                payload.get_name(),
            )));
        }
        Err(err) => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Failed to retrieve state: {}",
                err,
            )));
        }
    };

    // Check that organizations exists
    match state.get_organization(payload.get_org_id()) {
        Ok(None) => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Organization does not exist exists: {}",
                payload.get_org_id(),
            )));
        }
        Ok(Some(_)) => (),
        Err(err) => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Failed to retrieve state: {}",
                err,
            )));
        }
    };

    let smart_permission = SmartPermissionBuilder::new()
        .set_name(payload.get_name().to_string())
        .set_org_id(payload.get_org_id().to_string())
        .set_function(payload.get_function().to_vec())
        .build()
        .map_err(|_| {
            ApplyError::InvalidTransaction(String::from("Cannot build smart permission"))
        })?;

    state.set_smart_permission(payload.get_org_id(), payload.get_name(), smart_permission)
}

fn update_smart_permission(
    payload: UpdateSmartPermissionAction,
    signer: &str,
    state: &mut SabreState,
) -> Result<(), ApplyError> {
    // verify the signer of the transaction is authorized to update smart permissions
    is_admin(signer, payload.get_org_id(), state)?;

    // verify that the smart permission exists
    let smart_permission = match state.get_smart_permission(payload.get_org_id(), payload.get_name()) {
        Ok(None) => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Smart Permission does not exist: {} ",
                payload.get_name(),
            )));
        }
        Ok(Some(smart_permission)) => smart_permission,
        Err(err) => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Failed to retrieve state: {}",
                err,
            )));
        }
    };

    let smart_permission = smart_permission
        .into_builder()
        .set_function(payload.get_function().to_vec())
        .build()
        .map_err(|_| {
            ApplyError::InvalidTransaction(String::from("Cannot build smart permission"))
        })?;
    state.set_smart_permission(payload.get_org_id(), payload.get_name(), smart_permission)
}

fn delete_smart_permission(
    payload: DeleteSmartPermissionAction,
    signer: &str,
    state: &mut SabreState,
) -> Result<(), ApplyError> {
    // verify the signer of the transaction is authorized to delete smart permissions
    is_admin(signer, payload.get_org_id(), state)?;

    // verify that the smart permission exists
    match state.get_smart_permission(payload.get_org_id(), payload.get_name()) {
        Ok(None) => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Smart Permission does not exists: {} ",
                payload.get_name(),
            )));
        }
        Ok(Some(_)) => (),
        Err(err) => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Failed to retrieve state: {}",
                err,
            )));
        }
    };

    state.delete_smart_permission(payload.get_org_id(), payload.get_name())
}

fn create_account(
    payload: CreateAccountAction,
    signer: &str,
    state: &mut SabreState,
) -> Result<(), ApplyError> {
    // verify the signer of the transaction is authorized to create account
    is_admin(signer, payload.get_org_id(), state)?;

    // Check if the account already exists
    match state.get_account(payload.get_public_key()) {
        Ok(None) => (),
        Ok(Some(_)) => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Account already exists: {} ",
                payload.get_public_key(),
            )));
        }
        Err(err) => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Failed to retrieve state: {}",
                err,
            )));
        }
    };

    // Check that organizations exists
    match state.get_organization(payload.get_org_id()) {
        Ok(None) => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Organization does not exist exists: {}",
                payload.get_org_id(),
            )));
        }
        Ok(Some(_)) => (),
        Err(err) => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Failed to retrieve state: {}",
                err,
            )));
        }
    };

    let account = AccountBuilder::new()
        .set_public_key(payload.get_public_key().to_string())
        .set_org_id(payload.get_org_id().to_string())
        .set_roles(payload.get_roles().to_vec())
        .set_metadata(payload.get_metadata().to_vec())
        .build()
        .map_err(|_| {
            ApplyError::InvalidTransaction(String::from("Cannot build account"))
        })?;

    state.set_account(payload.get_public_key(), account)
}
/*
fn update_account(
    payload: UpdateAccountAction,
    signer: &str,
    state: &mut SabreState,
) -> Result<(), ApplyError> {
    // verify the signer of the transaction is authorized to update account
    is_admin(signer, payload.get_org_id(), state)?;

    // verify that the account exists
    let account = match state.get_account(payload.get_public_key()) {
        Ok(None) => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Account does not exist: {} ",
                payload.get_public_key(),
            )));
        }
        Ok(Some(account)) => account,
        Err(err) => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Failed to retrieve state: {}",
                err,
            )));
        }
    };

    let account = account
        //.into_builder()
        //.set_roles(payload.get_roles().to_vec())
        .set_metadata(payload.get_metadata().to_vec())
        .build()
        .map_err(|_| {
            ApplyError::InvalidTransaction(String::from("Cannot build account"))
        })?;
    state.set_account(payload.get_public_key(), account)
}
*/
fn update_account(
    payload: UpdateAccountAction,
    signer: &str,
    state: &mut SabreState,
) -> Result<(), ApplyError> {
    if payload.get_public_key().is_empty() {
        return Err(ApplyError::InvalidTransaction("Public key required".into()));
    }

    if payload.get_org_id().is_empty() {
        return Err(ApplyError::InvalidTransaction(
            "Organization ID required".into(),
        ));
    }
    // verify the signer of the transaction is authorized to update account
    is_admin(signer, payload.get_org_id(), state)?;

    // make sure account already exists
    let mut account = match state.get_account(payload.get_public_key()) {
        Ok(None) => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Account does not exists: {}",
                payload.get_public_key(),
            )))
        }
        Ok(Some(account)) => account,
        Err(err) => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Failed to retrieve state: {}",
                err,
            )))
        }
    };

    if !payload.get_roles().is_empty() {
        account.set_roles(protobuf::RepeatedField::from_vec(
            payload.get_roles().to_vec(),
        ));
    }

    if !payload.get_metadata().is_empty() {
        account.set_metadata(protobuf::RepeatedField::from_vec(
            payload.get_metadata().to_vec(),
        ));
    }

    if payload.get_active() != account.get_active() {
        if signer == payload.get_public_key() {
            return Err(ApplyError::InvalidTransaction(format!(
                "Admin may not deactivate themselves: {}",
                signer,
            )));
        }
        account.set_active(payload.get_active());
    }
    state
        .set_account(payload.get_public_key(), account)
        .map_err(|e| ApplyError::InternalError(format!("Failed to create account: {:?}", e)))
}

fn create_organization(
    payload: CreateOrganizationAction,
    signer: &str,
    state: &mut SabreState,
) -> Result<(), ApplyError> {
    // verify the signer of the transaction is authorized to create organization
    is_admin(signer, payload.get_id(), state)?;

    // Check if the organization already exists
    match state.get_organization(payload.get_id()) {
        Ok(None) => (),
        Ok(Some(_)) => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Organization already exists: {} ",
                payload.get_name(),
            )));
        }
        Err(err) => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Failed to retrieve state: {}",
                err,
            )));
        }
    };

    let organization = OrganizationBuilder::new()
        .set_org_id(payload.get_id().to_string())
        .set_name(payload.get_name().to_string())
        .set_address(payload.get_address().to_string())
        .build()
        .map_err(|_| {
            ApplyError::InvalidTransaction(String::from("Cannot build organization"))
        })?;

    state.set_organization(payload.get_id(), organization)
}
/*
fn update_organization(
    payload: UpdateOrganizationAction,
    signer: &str,
    state: &mut SabreState,
) -> Result<(), ApplyError> {
    // verify the signer of the transaction is authorized to update organization
    is_admin(signer, payload.get_id(), state)?;

    // verify that the organization exists
    let organization = match state.get_organization(payload.get_id()) {
        Ok(None) => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Organization does not exist: {} ",
                payload.get_name(),
            )));
        }
        Ok(Some(organization)) => organization,
        Err(err) => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Failed to retrieve state: {}",
                err,
            )));
        }
    };

    let organization = organization
        //.into_builder()
        //.set_name(payload.get_name().to_string())
        .set_address(payload.get_address().to_string())
        .build()
        .map_err(|_| {
            ApplyError::InvalidTransaction(String::from("Cannot build organization"))
        })?;
    state.set_organization(payload.get_id(), organization)
}
*/
fn update_organization(
    payload: UpdateOrganizationAction,
    signer: &str,
    state: &mut SabreState,
) -> Result<(), ApplyError> {
    if payload.get_id().is_empty() {
        return Err(ApplyError::InvalidTransaction(
            "Unique organization ID required".into(),
        ));
    }

    // verify the signer of the transaction is authorized to update organization
    is_admin(signer, payload.get_id(), state)?;

    // Make sure the organization already exists
    let mut organization = match state.get_organization(payload.get_id()) {
        Ok(None) => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Organization does not exist exists: {}",
                payload.get_id(),
            )))
        }
        Ok(Some(org)) => org,
        Err(err) => {
            return Err(ApplyError::InvalidTransaction(format!(
                "Failed to retrieve state: {}",
                err,
            )))
        }
    };

    if !payload.get_name().is_empty() {
        organization.set_name(payload.get_name().to_string());
    }
    if !payload.get_address().is_empty() {
        organization.set_address(payload.get_address().to_string());
    }
    state.set_organization(payload.get_id(), organization)
}

// helper function to check if the signer is allowed to update a namespace_registry
fn can_update_namespace_registry(
    namespace_registry: NamespaceRegistry,
    signer: &str,
    state: &mut SabreState,
) -> Result<(), ApplyError> {
    if !namespace_registry.get_owners().contains(&signer.into()) {
        let setting = match state.get_admin_setting() {
            Ok(Some(setting)) => setting,
            Ok(None) => {
                return Err(ApplyError::InvalidTransaction(format!(
                    "Only owners or admins can update or delete a namespace registry: {}",
                    signer,
                )));
            }
            Err(err) => {
                return Err(ApplyError::InvalidTransaction(format!(
                    "Unable to check state: {}",
                    err,
                )));
            }
        };

        for entry in setting.get_entries() {
            if entry.key == ADMINISTRATORS_SETTING_KEY {
                let values = entry.value.split(',');
                let value_vec: Vec<&str> = values.collect();
                if !value_vec.contains(&signer) {
                    return Err(ApplyError::InvalidTransaction(format!(
                        "Only owners or admins can update or delete a namespace registry: {}",
                        signer,
                    )));
                }
            }
        }
    }
    Ok(())
}

// helper function to check if the signer is allowed to update a contract_registry
fn can_update_contract_registry(
    contract_registry: ContractRegistry,
    signer: &str,
    state: &mut SabreState,
) -> Result<(), ApplyError> {
    if !contract_registry.get_owners().contains(&signer.into()) {
        let setting = match state.get_admin_setting() {
            Ok(Some(setting)) => setting,
            Ok(None) => {
                return Err(ApplyError::InvalidTransaction(format!(
                    "Only owners or admins can update or delete a contract registry: {}",
                    signer,
                )));
            }
            Err(err) => {
                return Err(ApplyError::InvalidTransaction(format!(
                    "Unable to check state: {}",
                    err,
                )));
            }
        };

        for entry in setting.get_entries() {
            if entry.key == ADMINISTRATORS_SETTING_KEY {
                let values = entry.value.split(',');
                let value_vec: Vec<&str> = values.collect();
                if !value_vec.contains(&signer) {
                    return Err(ApplyError::InvalidTransaction(format!(
                        "Only owners or admins can update or delete a contract registry: {}",
                        signer,
                    )));
                }
            }
        }
    }
    Ok(())
}
