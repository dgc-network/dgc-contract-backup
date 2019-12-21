// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

//use smart_sdk::protocol::pike::state::{Particpant, ParticpantList, Organization, OrganizationList};
use smart_sdk::protocol::state::{
    Contract, ContractList, ContractListBuilder, ContractRegistry, ContractRegistryList,
    ContractRegistryListBuilder, NamespaceRegistry, NamespaceRegistryList,
    NamespaceRegistryListBuilder, SmartPermission, SmartPermissionList, SmartPermissionListBuilder,
    Account, AccountList, Organization, OrganizationList,
    AccountListBuilder, OrganizationListBuilder,
};
use smart_sdk::protocol::ADMINISTRATORS_SETTING_ADDRESS;
use smart_sdk::protos::{FromBytes, IntoBytes};
use sawtooth_sdk::messages::setting::Setting;
use sawtooth_sdk::processor::handler::ApplyError;
use sawtooth_sdk::processor::handler::TransactionContext;

use crate::addressing::{
    compute_account_address, compute_org_address, compute_smart_permission_address,
    make_contract_address, make_contract_registry_address, make_namespace_registry_address,
};

pub struct SmartState<'a> {
    context: &'a mut dyn TransactionContext,
}

impl<'a> SmartState<'a> {
    pub fn new(context: &'a mut dyn TransactionContext) -> SmartState {
        SmartState { context }
    }

    pub fn context(&mut self) -> &mut dyn TransactionContext {
        self.context
    }

    pub fn get_admin_setting(&mut self) -> Result<Option<Setting>, ApplyError> {
        let d = self
            .context
            .get_state_entry(ADMINISTRATORS_SETTING_ADDRESS)?;
        match d {
            Some(packed) => {
                let setting: Setting =
                    protobuf::parse_from_bytes(packed.as_slice()).map_err(|err| {
                        ApplyError::InvalidTransaction(format!(
                            "Cannot deserialize setting: {:?}",
                            err,
                        ))
                    })?;

                Ok(Some(setting))
            }
            None => Ok(None),
        }
    }

    pub fn get_contract(
        &mut self,
        name: &str,
        version: &str,
    ) -> Result<Option<Contract>, ApplyError> {
        let address = make_contract_address(name, version)?;
        let d = self.context.get_state_entry(&address)?;
        match d {
            Some(packed) => {
                let contracts = ContractList::from_bytes(packed.as_slice()).map_err(|err| {
                    ApplyError::InvalidTransaction(format!(
                        "Cannot deserialize contract list: {:?}",
                        err,
                    ))
                })?;
                Ok(contracts
                    .get_contracts()
                    .iter()
                    .find(|c| c.get_name() == name)
                    .cloned())
            }
            None => Ok(None),
        }
    }

    pub fn set_contract(
        &mut self,
        name: &str,
        version: &str,
        new_contract: Contract,
    ) -> Result<(), ApplyError> {
        let address = make_contract_address(name, version)?;
        let d = self.context.get_state_entry(&address)?;
        let mut contracts = match d {
            Some(packed) => match ContractList::from_bytes(packed.as_slice()) {
                Ok(contracts) => {
                    // remove old contract if it exists
                    contracts
                        .get_contracts()
                        .iter()
                        .filter(|c| c.get_name() != name)
                        .cloned()
                        .collect::<Vec<Contract>>()
                }
                Err(err) => {
                    return Err(ApplyError::InvalidTransaction(format!(
                        "Cannot deserialize contract list: {}",
                        err,
                    )));
                }
            },
            None => vec![],
        };
        contracts.push(new_contract);
        // sort the contracts by name
        contracts.sort_by_key(|c| c.get_name().to_string());

        // build new ContractList and set in state
        let contract_list = ContractListBuilder::new()
            .set_contracts(contracts)
            .build()
            .map_err(|_| {
                ApplyError::InvalidTransaction(String::from("Cannot build contract list"))
            })?;

        let serialized = contract_list.into_bytes().map_err(|err| {
            ApplyError::InvalidTransaction(format!("Cannot serialize contract list: {:?}", err,))
        })?;
        self.context
            .set_state_entry(address, serialized)
            .map_err(|err| ApplyError::InvalidTransaction(format!("{}", err)))?;
        Ok(())
    }

    pub fn delete_contract(&mut self, name: &str, version: &str) -> Result<(), ApplyError> {
        let address = make_contract_address(name, version)?;
        let d = self.context.delete_state_entry(&address)?;
        let deleted = match d {
            Some(deleted) => deleted,
            None => {
                return Err(ApplyError::InvalidTransaction(String::from(
                    "Cannot delete contract",
                )));
            }
        };
        if deleted != address {
            return Err(ApplyError::InvalidTransaction(String::from(
                "Cannot delete contract",
            )));
        };
        Ok(())
    }

    pub fn get_contract_registry(
        &mut self,
        name: &str,
    ) -> Result<Option<ContractRegistry>, ApplyError> {
        let address = make_contract_registry_address(name)?;
        let d = self.context.get_state_entry(&address)?;
        match d {
            Some(packed) => {
                let contract_registries = ContractRegistryList::from_bytes(packed.as_slice())
                    .map_err(|err| {
                        ApplyError::InvalidTransaction(format!(
                            "Cannot deserialize contract registry list: {:?}",
                            err,
                        ))
                    })?;

                Ok(contract_registries
                    .get_registries()
                    .iter()
                    .find(|reg| reg.get_name() == name)
                    .cloned())
            }
            None => Ok(None),
        }
    }

    pub fn set_contract_registry(
        &mut self,
        name: &str,
        new_contract_registry: ContractRegistry,
    ) -> Result<(), ApplyError> {
        let address = make_contract_registry_address(name)?;
        let d = self.context.get_state_entry(&address)?;
        let mut contract_registries = match d {
            Some(packed) => match ContractRegistryList::from_bytes(packed.as_slice()) {
                Ok(contract_registries) => {
                    // remove old contract_registry if it exists
                    contract_registries
                        .get_registries()
                        .iter()
                        .filter(|c| c.get_name() != name)
                        .cloned()
                        .collect::<Vec<ContractRegistry>>()
                }
                Err(err) => {
                    return Err(ApplyError::InvalidTransaction(format!(
                        "Cannot deserialize contract registry list: {}",
                        err,
                    )));
                }
            },
            None => vec![],
        };

        contract_registries.push(new_contract_registry);
        // sort the contract regisitries by name
        contract_registries.sort_by_key(|c| c.get_name().to_string());
        let contract_registry_list = ContractRegistryListBuilder::new()
            .set_registries(contract_registries)
            .build()
            .map_err(|_| {
                ApplyError::InvalidTransaction(String::from("Cannot build contract registry list"))
            })?;

        let serialized = contract_registry_list.into_bytes().map_err(|err| {
            ApplyError::InvalidTransaction(format!(
                "Cannot serialize contract registry list: {:?}",
                err,
            ))
        })?;
        self.context
            .set_state_entry(address, serialized)
            .map_err(|err| ApplyError::InvalidTransaction(format!("{}", err)))?;
        Ok(())
    }

    pub fn delete_contract_registry(&mut self, name: &str) -> Result<(), ApplyError> {
        let address = make_contract_registry_address(name)?;
        let d = self.context.delete_state_entry(&address)?;
        let deleted = match d {
            Some(deleted) => deleted,
            None => {
                return Err(ApplyError::InvalidTransaction(String::from(
                    "Cannot delete contract registry",
                )));
            }
        };
        if deleted != address {
            return Err(ApplyError::InvalidTransaction(String::from(
                "Cannot delete contract registry",
            )));
        };
        Ok(())
    }

    pub fn get_namespace_registry(
        &mut self,
        namespace: &str,
    ) -> Result<Option<NamespaceRegistry>, ApplyError> {
        let address = make_namespace_registry_address(namespace)?;
        let d = self.context.get_state_entry(&address)?;
        match d {
            Some(packed) => {
                let namespace_registries = NamespaceRegistryList::from_bytes(packed.as_slice())
                    .map_err(|err| {
                        ApplyError::InvalidTransaction(format!(
                            "Cannot deserialize namespace registry list: {:?}",
                            err,
                        ))
                    })?;

                Ok(namespace_registries
                    .get_registries()
                    .iter()
                    .find(|reg| reg.get_namespace() == namespace)
                    .cloned())
            }
            None => Ok(None),
        }
    }

    pub fn get_namespace_registries(
        &mut self,
        namespace: &str,
    ) -> Result<Option<NamespaceRegistryList>, ApplyError> {
        let address = make_namespace_registry_address(namespace)?;
        let d = self.context.get_state_entry(&address)?;
        match d {
            Some(packed) => {
                let namespace_registries = NamespaceRegistryList::from_bytes(packed.as_slice())
                    .map_err(|err| {
                        ApplyError::InvalidTransaction(format!(
                            "Cannot deserialize namespace registry list: {:?}",
                            err,
                        ))
                    })?;
                Ok(Some(namespace_registries))
            }
            None => Ok(None),
        }
    }

    pub fn set_namespace_registry(
        &mut self,
        namespace: &str,
        new_namespace_registry: NamespaceRegistry,
    ) -> Result<(), ApplyError> {
        let address = make_namespace_registry_address(namespace)?;
        let d = self.context.get_state_entry(&address)?;
        let mut namespace_registries = match d {
            Some(packed) => match NamespaceRegistryList::from_bytes(packed.as_slice()) {
                Ok(namespace_registries) => {
                    // remove old namespace rgistry if it exists
                    namespace_registries
                        .get_registries()
                        .iter()
                        .filter(|nr| nr.get_namespace() != namespace)
                        .cloned()
                        .collect::<Vec<NamespaceRegistry>>()
                }
                Err(err) => {
                    return Err(ApplyError::InvalidTransaction(format!(
                        "Cannot deserialize namespace registry list: {}",
                        err,
                    )));
                }
            },
            None => vec![],
        };
        namespace_registries.push(new_namespace_registry);
        // sort the namespace registries by namespace
        namespace_registries.sort_by_key(|nr| nr.get_namespace().to_string());
        let namespace_registry_list = NamespaceRegistryListBuilder::new()
            .set_registries(namespace_registries)
            .build()
            .map_err(|_| {
                ApplyError::InvalidTransaction(String::from("Cannot build namespace registry list"))
            })?;

        let serialized = namespace_registry_list.into_bytes().map_err(|err| {
            ApplyError::InvalidTransaction(format!(
                "Cannot serialize namespace registry list: {:?}",
                err,
            ))
        })?;
        self.context
            .set_state_entry(address, serialized)
            .map_err(|err| ApplyError::InvalidTransaction(format!("{}", err)))?;
        Ok(())
    }

    pub fn delete_namespace_registry(&mut self, namespace: &str) -> Result<(), ApplyError> {
        let address = make_namespace_registry_address(namespace)?;
        let d = self.context.delete_state_entry(&address)?;
        let deleted = match d {
            Some(deleted) => deleted,
            None => {
                return Err(ApplyError::InvalidTransaction(String::from(
                    "Cannot delete namespace registry",
                )));
            }
        };
        if deleted != address {
            return Err(ApplyError::InvalidTransaction(String::from(
                "Cannot delete namespace registry",
            )));
        };
        Ok(())
    }

    pub fn get_smart_permission(
        &mut self,
        org_id: &str,
        name: &str,
    ) -> Result<Option<SmartPermission>, ApplyError> {
        let address = compute_smart_permission_address(org_id, name);
        let d = self.context.get_state_entry(&address)?;
        match d {
            Some(packed) => {
                let smart_permissions = SmartPermissionList::from_bytes(packed.as_slice())
                    .map_err(|err| {
                        ApplyError::InvalidTransaction(format!(
                            "Cannot deserialize smart permissions list: {:?}",
                            err,
                        ))
                    })?;

                Ok(smart_permissions
                    .get_smart_permissions()
                    .iter()
                    .find(|sp| sp.get_name() == name)
                    .cloned())
            }
            None => Ok(None),
        }
    }

    pub fn set_smart_permission(
        &mut self,
        org_id: &str,
        name: &str,
        new_smart_permission: SmartPermission,
    ) -> Result<(), ApplyError> {
        let address = compute_smart_permission_address(org_id, name);
        let d = self.context.get_state_entry(&address)?;
        let mut smart_permissions = match d {
            Some(packed) => match SmartPermissionList::from_bytes(packed.as_slice()) {
                Ok(smart_permissions) => {
                    // remove old smart_permission if it exists
                    smart_permissions
                        .get_smart_permissions()
                        .iter()
                        .filter(|sp| sp.get_name() != name)
                        .cloned()
                        .collect::<Vec<SmartPermission>>()
                }
                Err(err) => {
                    return Err(ApplyError::InvalidTransaction(format!(
                        "Cannot deserialize smart permission list: {}",
                        err,
                    )));
                }
            },
            None => vec![],
        };

        smart_permissions.push(new_smart_permission);
        // sort the smart_permission by name
        smart_permissions.sort_by_key(|sp| sp.get_name().to_string());

        let smart_permission_list = SmartPermissionListBuilder::new()
            .set_smart_permissions(smart_permissions)
            .build()
            .map_err(|_| {
                ApplyError::InvalidTransaction(String::from("Cannot build smart permission list"))
            })?;

        let serialized = smart_permission_list.into_bytes().map_err(|err| {
            ApplyError::InvalidTransaction(format!(
                "Cannot serialize smart permission list: {:?}",
                err,
            ))
        })?;
        self.context
            .set_state_entry(address, serialized)
            .map_err(|err| ApplyError::InvalidTransaction(format!("{}", err)))?;
        Ok(())
    }

    pub fn delete_smart_permission(&mut self, org_id: &str, name: &str) -> Result<(), ApplyError> {
        let address = compute_smart_permission_address(org_id, name);
        let d = self.context.delete_state_entry(&address.clone())?;
        let deleted = match d {
            Some(deleted) => deleted,
            None => {
                return Err(ApplyError::InvalidTransaction(String::from(
                    "Cannot delete smart_permission",
                )));
            }
        };
        if deleted != address {
            return Err(ApplyError::InvalidTransaction(String::from(
                "Cannot delete smart_permission",
            )));
        };
        Ok(())
    }

    pub fn get_account(&mut self, public_key: &str) -> Result<Option<Account>, ApplyError> {
        let address = compute_account_address(public_key);
        let d = self.context.get_state_entry(&address)?;
        match d {
            Some(packed) => {
                let accounts = AccountList::from_bytes(packed.as_slice()).map_err(|err| {
                    ApplyError::InvalidTransaction(format!(
                        "Cannot deserialize account list: {:?}",
                        err,
                    ))
                })?;

                Ok(accounts
                    .get_accounts()
                    .iter()
                    .find(|account| account.get_public_key() == public_key)
                    .cloned())
            }
            None => Ok(None),
        }
    }

    pub fn set_account(
        &mut self,
        public_key: &str,
        new_account: Account,
    ) -> Result<(), ApplyError> {
        let address = compute_account_address(public_key);
        let d = self.context.get_state_entry(&address)?;
        let mut accounts = match d {
            Some(packed) => match AccountList::from_bytes(packed.as_slice()) {
                Ok(accounts) => {
                    // remove old account if it exists
                    accounts
                        .get_accounts()
                        .iter()
                        .filter(|sp| sp.get_public_key() != public_key)
                        .cloned()
                        .collect::<Vec<Account>>()
                }
                Err(err) => {
                    return Err(ApplyError::InvalidTransaction(format!(
                        "Cannot deserialize account list: {}",
                        err,
                    )));
                }
            },
            None => vec![],
        };

        accounts.push(new_account);
        // sort the account by public_key
        accounts.sort_by_key(|sp| sp.get_public_key().to_string());

        let account_list = AccountListBuilder::new()
            .set_accounts(accounts)
            .build()
            .map_err(|_| {
                ApplyError::InvalidTransaction(String::from("Cannot build account list"))
            })?;

        let serialized = account_list.into_bytes().map_err(|err| {
            ApplyError::InvalidTransaction(format!(
                "Cannot serialize account list: {:?}",
                err,
            ))
        })?;
        self.context
            .set_state_entry(address, serialized)
            .map_err(|err| ApplyError::InvalidTransaction(format!("{}", err)))?;
        Ok(())
    }

    pub fn get_organization(&mut self, org_id: &str) -> Result<Option<Organization>, ApplyError> {
        let address = compute_org_address(org_id);
        let d = self.context.get_state_entry(&address)?;
        match d {
            Some(packed) => {
                let orgs = OrganizationList::from_bytes(packed.as_slice()).map_err(|err| {
                    ApplyError::InvalidTransaction(format!(
                        "Cannot deserialize organization list: {:?}",
                        err,
                    ))
                })?;

                Ok(orgs
                    .get_organizations()
                    .iter()
                    .find(|org| org.get_org_id() == org_id)
                    .cloned())
            }
            None => Ok(None),
        }
    }

    pub fn set_organization(
        &mut self,
        org_id: &str,
        new_organization: Organization,
    ) -> Result<(), ApplyError> {
        let address = compute_org_address(org_id);
        let d = self.context.get_state_entry(&address)?;
        let mut organizations = match d {
            Some(packed) => match OrganizationList::from_bytes(packed.as_slice()) {
                Ok(organizations) => {
                    // remove old organization if it exists
                    organizations
                        .get_organizations()
                        .iter()
                        .filter(|sp| sp.get_org_id() != org_id)
                        .cloned()
                        .collect::<Vec<Organization>>()
                }
                Err(err) => {
                    return Err(ApplyError::InvalidTransaction(format!(
                        "Cannot deserialize organization list: {}",
                        err,
                    )));
                }
            },
            None => vec![],
        };

        organizations.push(new_organization);
        // sort the organization by org_id
        organizations.sort_by_key(|sp| sp.get_org_id().to_string());

        let organization_list = OrganizationListBuilder::new()
            .set_organizations(organizations)
            .build()
            .map_err(|_| {
                ApplyError::InvalidTransaction(String::from("Cannot build organization list"))
            })?;

        let serialized = organization_list.into_bytes().map_err(|err| {
            ApplyError::InvalidTransaction(format!(
                "Cannot serialize organization list: {:?}",
                err,
            ))
        })?;
        self.context
            .set_state_entry(address, serialized)
            .map_err(|err| ApplyError::InvalidTransaction(format!("{}", err)))?;
        Ok(())
    }
}
