// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

use protobuf::Message;
use protobuf::RepeatedField;

use std::error::Error as StdError;

use crate::protos;
use crate::protos::{
    FromBytes, FromNative, FromProto, IntoBytes, IntoNative, IntoProto, ProtoConversionError,
};

/// Native implementation for Version
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Version {
    version: String,
    contract_sha512: String,
    creator: String,
}

impl Version {
    pub fn get_version(&self) -> &String {
        &self.version
    }

    pub fn get_contract_sha512(&self) -> &String {
        &self.contract_sha512
    }

    pub fn get_creator(&self) -> &String {
        &self.creator
    }

    pub fn into_builder(self) -> VersionBuilder {
        VersionBuilder::new()
            .set_version(self.version)
            .set_contract_sha512(self.contract_sha512)
            .set_creator(self.creator)
    }
}

impl FromProto<protos::contract_registry::ContractRegistry_Version> for Version {
    fn from_proto(
        proto: protos::contract_registry::ContractRegistry_Version,
    ) -> Result<Self, ProtoConversionError> {
        Ok(Version {
            version: proto.get_version().to_string(),
            contract_sha512: proto.get_contract_sha512().to_string(),
            creator: proto.get_creator().to_string(),
        })
    }
}

impl FromNative<Version> for protos::contract_registry::ContractRegistry_Version {
    fn from_native(native: Version) -> Result<Self, ProtoConversionError> {
        let mut proto = protos::contract_registry::ContractRegistry_Version::new();

        proto.set_version(native.get_version().to_string());
        proto.set_contract_sha512(native.get_contract_sha512().to_string());
        proto.set_creator(native.get_creator().to_string());

        Ok(proto)
    }
}

impl IntoProto<protos::contract_registry::ContractRegistry_Version> for Version {}
impl IntoNative<Version> for protos::contract_registry::ContractRegistry_Version {}

#[derive(Debug)]
pub enum VersionBuildError {
    MissingField(String),
}

impl StdError for VersionBuildError {
    fn description(&self) -> &str {
        match *self {
            VersionBuildError::MissingField(ref msg) => msg,
        }
    }
}

impl std::fmt::Display for VersionBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            VersionBuildError::MissingField(ref s) => write!(f, "MissingField: {}", s),
        }
    }
}

/// Builder used to create a Version
#[derive(Default, Clone)]
pub struct VersionBuilder {
    version: Option<String>,
    contract_sha512: Option<String>,
    creator: Option<String>,
}

impl VersionBuilder {
    pub fn new() -> Self {
        VersionBuilder::default()
    }

    pub fn set_version(mut self, version: String) -> VersionBuilder {
        self.version = Some(version);
        self
    }

    pub fn set_contract_sha512(mut self, contract_sha512: String) -> VersionBuilder {
        self.contract_sha512 = Some(contract_sha512);
        self
    }

    pub fn set_creator(mut self, creator: String) -> VersionBuilder {
        self.creator = Some(creator);
        self
    }

    pub fn build(self) -> Result<Version, VersionBuildError> {
        let version = self.version.ok_or_else(|| {
            VersionBuildError::MissingField("'versions' field is required".to_string())
        })?;

        let contract_sha512 = self.contract_sha512.ok_or_else(|| {
            VersionBuildError::MissingField("'contract_sha512' field is required".to_string())
        })?;

        let creator = self.creator.ok_or_else(|| {
            VersionBuildError::MissingField("'creator' field is required".to_string())
        })?;

        Ok(Version {
            version,
            contract_sha512,
            creator,
        })
    }
}

/// Native implementation for ContractRegistry
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ContractRegistry {
    name: String,
    versions: Vec<Version>,
    owners: Vec<String>,
}

impl ContractRegistry {
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_versions(&self) -> &[Version] {
        &self.versions
    }

    pub fn get_owners(&self) -> &[String] {
        &self.owners
    }

    pub fn into_builder(self) -> ContractRegistryBuilder {
        ContractRegistryBuilder::new()
            .set_name(self.name)
            .set_versions(self.versions)
            .set_owners(self.owners)
    }
}

impl FromProto<protos::contract_registry::ContractRegistry> for ContractRegistry {
    fn from_proto(
        proto: protos::contract_registry::ContractRegistry,
    ) -> Result<Self, ProtoConversionError> {
        Ok(ContractRegistry {
            name: proto.get_name().to_string(),
            versions: proto
                .get_versions()
                .to_vec()
                .into_iter()
                .map(Version::from_proto)
                .collect::<Result<Vec<Version>, ProtoConversionError>>()?,
            owners: proto.get_owners().to_vec(),
        })
    }
}

impl FromNative<ContractRegistry> for protos::contract_registry::ContractRegistry {
    fn from_native(contract_registry: ContractRegistry) -> Result<Self, ProtoConversionError> {
        let mut proto = protos::contract_registry::ContractRegistry::new();
        proto.set_name(contract_registry.get_name().to_string());
        proto.set_versions(RepeatedField::from_vec(
            contract_registry
                .get_versions()
                .to_vec()
                .into_iter()
                .map(Version::into_proto)
                .collect::<Result<
                    Vec<protos::contract_registry::ContractRegistry_Version>,
                    ProtoConversionError,
                >>()?,
        ));
        proto.set_owners(RepeatedField::from_vec(contract_registry.get_owners().to_vec()));

        Ok(proto)
    }
}

impl FromBytes<ContractRegistry> for ContractRegistry {
    fn from_bytes(bytes: &[u8]) -> Result<ContractRegistry, ProtoConversionError> {
        let proto: protos::contract_registry::ContractRegistry = protobuf::parse_from_bytes(bytes)
            .map_err(|_| {
                ProtoConversionError::SerializationError(
                    "Unable to get ContractRegistry from bytes".to_string(),
                )
            })?;
        proto.into_native()
    }
}

impl IntoBytes for ContractRegistry {
    fn into_bytes(self) -> Result<Vec<u8>, ProtoConversionError> {
        let proto = self.into_proto()?;
        let bytes = proto.write_to_bytes().map_err(|_| {
            ProtoConversionError::SerializationError(
                "Unable to get bytes from ContractRegistry".to_string(),
            )
        })?;
        Ok(bytes)
    }
}

impl IntoProto<protos::contract_registry::ContractRegistry> for ContractRegistry {}
impl IntoNative<ContractRegistry> for protos::contract_registry::ContractRegistry {}

#[derive(Debug)]
pub enum ContractRegistryBuildError {
    MissingField(String),
}

impl StdError for ContractRegistryBuildError {
    fn description(&self) -> &str {
        match *self {
            ContractRegistryBuildError::MissingField(ref msg) => msg,
        }
    }
}

impl std::fmt::Display for ContractRegistryBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ContractRegistryBuildError::MissingField(ref s) => write!(f, "MissingField: {}", s),
        }
    }
}

/// Builder used to create a ContractRegistry
#[derive(Default, Clone)]
pub struct ContractRegistryBuilder {
    name: Option<String>,
    versions: Vec<Version>,
    owners: Vec<String>,
}

impl ContractRegistryBuilder {
    pub fn new() -> Self {
        ContractRegistryBuilder::default()
    }

    pub fn set_name(mut self, name: String) -> ContractRegistryBuilder {
        self.name = Some(name);
        self
    }

    pub fn set_versions(mut self, versions: Vec<Version>) -> ContractRegistryBuilder {
        self.versions = versions;
        self
    }

    pub fn set_owners(mut self, owners: Vec<String>) -> ContractRegistryBuilder {
        self.owners = owners;
        self
    }

    pub fn build(self) -> Result<ContractRegistry, ContractRegistryBuildError> {
        let name = self.name.ok_or_else(|| {
            ContractRegistryBuildError::MissingField("'name' field is required".to_string())
        })?;

        let versions = self.versions;

        let owners = {
            if !self.owners.is_empty() {
                self.owners
            } else {
                return Err(ContractRegistryBuildError::MissingField(
                    "'owners' field is required".to_string(),
                ));
            }
        };

        Ok(ContractRegistry {
            name,
            versions,
            owners,
        })
    }
}

/// Native implementation for ContractRegistryList
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ContractRegistryList {
    registries: Vec<ContractRegistry>,
}

impl ContractRegistryList {
    pub fn get_registries(&self) -> &[ContractRegistry] {
        &self.registries
    }
}

impl FromProto<protos::contract_registry::ContractRegistryList> for ContractRegistryList {
    fn from_proto(
        proto: protos::contract_registry::ContractRegistryList,
    ) -> Result<Self, ProtoConversionError> {
        Ok(ContractRegistryList {
            registries: proto
                .get_registries()
                .to_vec()
                .into_iter()
                .map(ContractRegistry::from_proto)
                .collect::<Result<Vec<ContractRegistry>, ProtoConversionError>>()?,
        })
    }
}

impl FromNative<ContractRegistryList> for protos::contract_registry::ContractRegistryList {
    fn from_native(
        contract_registry_list: ContractRegistryList,
    ) -> Result<Self, ProtoConversionError> {
        let mut proto = protos::contract_registry::ContractRegistryList::new();
        proto.set_registries(
            RepeatedField::from_vec(
                contract_registry_list
                    .get_registries()
                    .to_vec()
                    .into_iter()
                    .map(ContractRegistry::into_proto)
                    .collect::<Result<
                        Vec<protos::contract_registry::ContractRegistry>,
                        ProtoConversionError,
                    >>()?,
            ),
        );

        Ok(proto)
    }
}

impl FromBytes<ContractRegistryList> for ContractRegistryList {
    fn from_bytes(bytes: &[u8]) -> Result<ContractRegistryList, ProtoConversionError> {
        let proto: protos::contract_registry::ContractRegistryList =
            protobuf::parse_from_bytes(bytes).map_err(|_| {
                ProtoConversionError::SerializationError(
                    "Unable to get ContractRegistryList from bytes".to_string(),
                )
            })?;
        proto.into_native()
    }
}

impl IntoBytes for ContractRegistryList {
    fn into_bytes(self) -> Result<Vec<u8>, ProtoConversionError> {
        let proto = self.into_proto()?;
        let bytes = proto.write_to_bytes().map_err(|_| {
            ProtoConversionError::SerializationError(
                "Unable to get bytes from ContractRegistryList".to_string(),
            )
        })?;
        Ok(bytes)
    }
}

impl IntoProto<protos::contract_registry::ContractRegistryList> for ContractRegistryList {}
impl IntoNative<ContractRegistryList> for protos::contract_registry::ContractRegistryList {}

#[derive(Debug)]
pub enum ContractRegistryListBuildError {
    MissingField(String),
}

impl StdError for ContractRegistryListBuildError {
    fn description(&self) -> &str {
        match *self {
            ContractRegistryListBuildError::MissingField(ref msg) => msg,
        }
    }
}

impl std::fmt::Display for ContractRegistryListBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ContractRegistryListBuildError::MissingField(ref s) => write!(f, "MissingField: {}", s),
        }
    }
}

/// Builder used to create a ContractRegistryList
#[derive(Default, Clone)]
pub struct ContractRegistryListBuilder {
    registries: Vec<ContractRegistry>,
}

impl ContractRegistryListBuilder {
    pub fn new() -> Self {
        ContractRegistryListBuilder::default()
    }

    pub fn set_registries(
        mut self,
        registries: Vec<ContractRegistry>,
    ) -> ContractRegistryListBuilder {
        self.registries = registries;
        self
    }

    pub fn build(self) -> Result<ContractRegistryList, ContractRegistryListBuildError> {
        let registries = self.registries;

        Ok(ContractRegistryList { registries })
    }
}

/// Native implementation for Permission
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Permission {
    contract_name: String,
    read: bool,
    write: bool,
}

impl Permission {
    pub fn get_contract_name(&self) -> &String {
        &self.contract_name
    }

    pub fn get_read(&self) -> bool {
        self.read
    }

    pub fn get_write(&self) -> bool {
        self.write
    }

    pub fn into_builder(self) -> PermissionBuilder {
        PermissionBuilder::new()
            .set_contract_name(self.contract_name)
            .set_read(self.read)
            .set_write(self.write)
    }
}

impl FromProto<protos::namespace_registry::NamespaceRegistry_Permission> for Permission {
    fn from_proto(
        proto: protos::namespace_registry::NamespaceRegistry_Permission,
    ) -> Result<Self, ProtoConversionError> {
        Ok(Permission {
            contract_name: proto.get_contract_name().to_string(),
            read: proto.get_read(),
            write: proto.get_write(),
        })
    }
}

impl FromNative<Permission> for protos::namespace_registry::NamespaceRegistry_Permission {
    fn from_native(native: Permission) -> Result<Self, ProtoConversionError> {
        let mut proto = protos::namespace_registry::NamespaceRegistry_Permission::new();

        proto.set_contract_name(native.get_contract_name().to_string());
        proto.set_read(native.get_read());
        proto.set_write(native.get_write());

        Ok(proto)
    }
}

impl IntoProto<protos::namespace_registry::NamespaceRegistry_Permission> for Permission {}
impl IntoNative<Permission> for protos::namespace_registry::NamespaceRegistry_Permission {}

#[derive(Debug)]
pub enum PermissionBuildError {
    MissingField(String),
}

impl StdError for PermissionBuildError {
    fn description(&self) -> &str {
        match *self {
            PermissionBuildError::MissingField(ref msg) => msg,
        }
    }
}

impl std::fmt::Display for PermissionBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            PermissionBuildError::MissingField(ref s) => write!(f, "MissingField: {}", s),
        }
    }
}

/// Builder used to create a Permission
#[derive(Default, Clone)]
pub struct PermissionBuilder {
    contract_name: Option<String>,
    read: Option<bool>,
    write: Option<bool>,
}

impl PermissionBuilder {
    pub fn new() -> Self {
        PermissionBuilder::default()
    }

    pub fn set_contract_name(mut self, contract_name: String) -> PermissionBuilder {
        self.contract_name = Some(contract_name);
        self
    }

    pub fn set_read(mut self, read: bool) -> PermissionBuilder {
        self.read = Some(read);
        self
    }

    pub fn set_write(mut self, write: bool) -> PermissionBuilder {
        self.write = Some(write);
        self
    }

    pub fn build(self) -> Result<Permission, PermissionBuildError> {
        let contract_name = self.contract_name.ok_or_else(|| {
            PermissionBuildError::MissingField("'contract_name' field is required".to_string())
        })?;

        let read = self.read.unwrap_or_default();

        let write = self.write.unwrap_or_default();

        Ok(Permission {
            contract_name,
            read,
            write,
        })
    }
}

/// Native implementation for NamespaceRegistry
#[derive(Default, Debug, Clone, PartialEq)]
pub struct NamespaceRegistry {
    namespace: String,
    owners: Vec<String>,
    permissions: Vec<Permission>,
}

impl NamespaceRegistry {
    pub fn get_namespace(&self) -> &String {
        &self.namespace
    }

    pub fn get_owners(&self) -> &[String] {
        &self.owners
    }

    pub fn get_permissions(&self) -> &[Permission] {
        &self.permissions
    }

    pub fn into_builder(self) -> NamespaceRegistryBuilder {
        NamespaceRegistryBuilder::new()
            .set_namespace(self.namespace)
            .set_owners(self.owners)
            .set_permissions(self.permissions)
    }
}

impl FromProto<protos::namespace_registry::NamespaceRegistry> for NamespaceRegistry {
    fn from_proto(
        proto: protos::namespace_registry::NamespaceRegistry,
    ) -> Result<Self, ProtoConversionError> {
        Ok(NamespaceRegistry {
            namespace: proto.get_namespace().to_string(),
            owners: proto.get_owners().to_vec(),
            permissions: proto
                .get_permissions()
                .to_vec()
                .into_iter()
                .map(Permission::from_proto)
                .collect::<Result<Vec<Permission>, ProtoConversionError>>()?,
        })
    }
}

impl FromNative<NamespaceRegistry> for protos::namespace_registry::NamespaceRegistry {
    fn from_native(native: NamespaceRegistry) -> Result<Self, ProtoConversionError> {
        let mut proto = protos::namespace_registry::NamespaceRegistry::new();
        proto.set_namespace(native.get_namespace().to_string());
        proto.set_owners(RepeatedField::from_vec(native.get_owners().to_vec()));
        proto.set_permissions(RepeatedField::from_vec(
            native
                .get_permissions()
                .to_vec()
                .into_iter()
                .map(Permission::into_proto)
                .collect::<Result<
                    Vec<protos::namespace_registry::NamespaceRegistry_Permission>,
                    ProtoConversionError,
                >>()?,
        ));

        Ok(proto)
    }
}

impl FromBytes<NamespaceRegistry> for NamespaceRegistry {
    fn from_bytes(bytes: &[u8]) -> Result<NamespaceRegistry, ProtoConversionError> {
        let proto: protos::namespace_registry::NamespaceRegistry =
            protobuf::parse_from_bytes(bytes).map_err(|_| {
                ProtoConversionError::SerializationError(
                    "Unable to get NamespaceRegistry from bytes".to_string(),
                )
            })?;
        proto.into_native()
    }
}

impl IntoBytes for NamespaceRegistry {
    fn into_bytes(self) -> Result<Vec<u8>, ProtoConversionError> {
        let proto = self.into_proto()?;
        let bytes = proto.write_to_bytes().map_err(|_| {
            ProtoConversionError::SerializationError(
                "Unable to get bytes from NamespaceRegistry".to_string(),
            )
        })?;
        Ok(bytes)
    }
}

impl IntoProto<protos::namespace_registry::NamespaceRegistry> for NamespaceRegistry {}
impl IntoNative<NamespaceRegistry> for protos::namespace_registry::NamespaceRegistry {}

#[derive(Debug)]
pub enum NamespaceRegistryBuildError {
    MissingField(String),
}

impl StdError for NamespaceRegistryBuildError {
    fn description(&self) -> &str {
        match *self {
            NamespaceRegistryBuildError::MissingField(ref msg) => msg,
        }
    }
}

impl std::fmt::Display for NamespaceRegistryBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            NamespaceRegistryBuildError::MissingField(ref s) => write!(f, "MissingField: {}", s),
        }
    }
}

/// Builder used to create a NamespaceRegistry
#[derive(Default, Clone)]
pub struct NamespaceRegistryBuilder {
    namespace: Option<String>,
    owners: Vec<String>,
    permissions: Vec<Permission>,
}

impl NamespaceRegistryBuilder {
    pub fn new() -> Self {
        NamespaceRegistryBuilder::default()
    }

    pub fn set_namespace(mut self, namespace: String) -> NamespaceRegistryBuilder {
        self.namespace = Some(namespace);
        self
    }

    pub fn set_owners(mut self, owners: Vec<String>) -> NamespaceRegistryBuilder {
        self.owners = owners;
        self
    }

    pub fn set_permissions(mut self, permissions: Vec<Permission>) -> NamespaceRegistryBuilder {
        self.permissions = permissions;
        self
    }

    pub fn build(self) -> Result<NamespaceRegistry, NamespaceRegistryBuildError> {
        let namespace = self.namespace.ok_or_else(|| {
            NamespaceRegistryBuildError::MissingField("'namespace' field is required".to_string())
        })?;

        let owners = {
            if !self.owners.is_empty() {
                self.owners
            } else {
                return Err(NamespaceRegistryBuildError::MissingField(
                    "'owners' field is required".to_string(),
                ));
            }
        };

        let permissions = self.permissions;

        Ok(NamespaceRegistry {
            namespace,
            owners,
            permissions,
        })
    }
}

// Native implementation for NamespaceRegistryList
#[derive(Default, Debug, Clone, PartialEq)]
pub struct NamespaceRegistryList {
    registries: Vec<NamespaceRegistry>,
}

impl NamespaceRegistryList {
    pub fn get_registries(&self) -> &[NamespaceRegistry] {
        &self.registries
    }
}

impl FromProto<protos::namespace_registry::NamespaceRegistryList> for NamespaceRegistryList {
    fn from_proto(
        proto: protos::namespace_registry::NamespaceRegistryList,
    ) -> Result<Self, ProtoConversionError> {
        Ok(NamespaceRegistryList {
            registries: proto
                .get_registries()
                .to_vec()
                .into_iter()
                .map(NamespaceRegistry::from_proto)
                .collect::<Result<Vec<NamespaceRegistry>, ProtoConversionError>>()?,
        })
    }
}

impl FromNative<NamespaceRegistryList> for protos::namespace_registry::NamespaceRegistryList {
    fn from_native(
        namespace_registry_list: NamespaceRegistryList,
    ) -> Result<Self, ProtoConversionError> {
        let mut proto = protos::namespace_registry::NamespaceRegistryList::new();
        proto.set_registries(
            RepeatedField::from_vec(
                namespace_registry_list
                    .get_registries()
                    .to_vec()
                    .into_iter()
                    .map(NamespaceRegistry::into_proto)
                    .collect::<Result<
                        Vec<protos::namespace_registry::NamespaceRegistry>,
                        ProtoConversionError,
                    >>()?,
            ),
        );

        Ok(proto)
    }
}

impl FromBytes<NamespaceRegistryList> for NamespaceRegistryList {
    fn from_bytes(bytes: &[u8]) -> Result<NamespaceRegistryList, ProtoConversionError> {
        let proto: protos::namespace_registry::NamespaceRegistryList =
            protobuf::parse_from_bytes(bytes).map_err(|_| {
                ProtoConversionError::SerializationError(
                    "Unable to get NamespaceRegistryList from bytes".to_string(),
                )
            })?;
        proto.into_native()
    }
}

impl IntoBytes for NamespaceRegistryList {
    fn into_bytes(self) -> Result<Vec<u8>, ProtoConversionError> {
        let proto = self.into_proto()?;
        let bytes = proto.write_to_bytes().map_err(|_| {
            ProtoConversionError::SerializationError(
                "Unable to get bytes from NamespaceRegistryList".to_string(),
            )
        })?;
        Ok(bytes)
    }
}

impl IntoProto<protos::namespace_registry::NamespaceRegistryList> for NamespaceRegistryList {}
impl IntoNative<NamespaceRegistryList> for protos::namespace_registry::NamespaceRegistryList {}

#[derive(Debug)]
pub enum NamespaceRegistryListBuildError {
    MissingField(String),
}

impl StdError for NamespaceRegistryListBuildError {
    fn description(&self) -> &str {
        match *self {
            NamespaceRegistryListBuildError::MissingField(ref msg) => msg,
        }
    }
}

impl std::fmt::Display for NamespaceRegistryListBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            NamespaceRegistryListBuildError::MissingField(ref s) => {
                write!(f, "MissingField: {}", s)
            }
        }
    }
}

/// Builder used to create a NamespaceRegistryList
#[derive(Default, Clone)]
pub struct NamespaceRegistryListBuilder {
    registries: Vec<NamespaceRegistry>,
}

impl NamespaceRegistryListBuilder {
    pub fn new() -> Self {
        NamespaceRegistryListBuilder::default()
    }

    pub fn set_registries(
        mut self,
        registries: Vec<NamespaceRegistry>,
    ) -> NamespaceRegistryListBuilder {
        self.registries = registries;
        self
    }

    pub fn build(self) -> Result<NamespaceRegistryList, NamespaceRegistryListBuildError> {
        let registries = self.registries;

        Ok(NamespaceRegistryList { registries })
    }
}

/// Native implementation for Contract
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Contract {
    name: String,
    version: String,
    inputs: Vec<String>,
    outputs: Vec<String>,
    creator: String,
    contract: Vec<u8>,
}

impl Contract {
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_version(&self) -> &String {
        &self.version
    }

    pub fn get_inputs(&self) -> &[String] {
        &self.inputs
    }

    pub fn get_outputs(&self) -> &[String] {
        &self.outputs
    }

    pub fn get_creator(&self) -> &String {
        &self.creator
    }

    pub fn get_contract(&self) -> &[u8] {
        &self.contract
    }

    pub fn into_builder(self) -> ContractBuilder {
        ContractBuilder::new()
            .set_name(self.name)
            .set_version(self.version)
            .set_inputs(self.inputs)
            .set_outputs(self.outputs)
            .set_creator(self.creator)
            .set_contract(self.contract)
    }
}

impl FromProto<protos::contract::Contract> for Contract {
    fn from_proto(proto: protos::contract::Contract) -> Result<Self, ProtoConversionError> {
        Ok(Contract {
            name: proto.get_name().to_string(),
            version: proto.get_version().to_string(),
            inputs: proto.get_inputs().to_vec(),
            outputs: proto.get_outputs().to_vec(),
            creator: proto.get_creator().to_string(),
            contract: proto.get_contract().to_vec(),
        })
    }
}

impl FromNative<Contract> for protos::contract::Contract {
    fn from_native(contract: Contract) -> Result<Self, ProtoConversionError> {
        let mut proto = protos::contract::Contract::new();

        proto.set_name(contract.get_name().to_string());
        proto.set_version(contract.get_version().to_string());
        proto.set_inputs(RepeatedField::from_vec(contract.get_inputs().to_vec()));
        proto.set_outputs(RepeatedField::from_vec(contract.get_outputs().to_vec()));
        proto.set_creator(contract.get_creator().to_string());
        proto.set_contract(contract.get_contract().to_vec());

        Ok(proto)
    }
}

impl FromBytes<Contract> for Contract {
    fn from_bytes(bytes: &[u8]) -> Result<Contract, ProtoConversionError> {
        let proto: protos::contract::Contract =
            protobuf::parse_from_bytes(bytes).map_err(|_| {
                ProtoConversionError::SerializationError(
                    "Unable to get Contract from bytes".to_string(),
                )
            })?;
        proto.into_native()
    }
}

impl IntoBytes for Contract {
    fn into_bytes(self) -> Result<Vec<u8>, ProtoConversionError> {
        let proto = self.into_proto()?;
        let bytes = proto.write_to_bytes().map_err(|_| {
            ProtoConversionError::SerializationError(
                "Unable to get bytes from Contract".to_string(),
            )
        })?;
        Ok(bytes)
    }
}

impl IntoProto<protos::contract::Contract> for Contract {}
impl IntoNative<Contract> for protos::contract::Contract {}

#[derive(Debug)]
pub enum ContractBuildError {
    MissingField(String),
}

impl StdError for ContractBuildError {
    fn description(&self) -> &str {
        match *self {
            ContractBuildError::MissingField(ref msg) => msg,
        }
    }
}

impl std::fmt::Display for ContractBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ContractBuildError::MissingField(ref s) => write!(f, "MissingField: {}", s),
        }
    }
}

/// Builder used to create a Contract
#[derive(Default, Clone)]
pub struct ContractBuilder {
    name: Option<String>,
    version: Option<String>,
    inputs: Vec<String>,
    outputs: Vec<String>,
    creator: Option<String>,
    contract: Vec<u8>,
}

impl ContractBuilder {
    pub fn new() -> Self {
        ContractBuilder::default()
    }

    pub fn set_name(mut self, name: String) -> ContractBuilder {
        self.name = Some(name);
        self
    }

    pub fn set_version(mut self, version: String) -> ContractBuilder {
        self.version = Some(version);
        self
    }

    pub fn set_inputs(mut self, inputs: Vec<String>) -> ContractBuilder {
        self.inputs = inputs;
        self
    }

    pub fn set_outputs(mut self, outputs: Vec<String>) -> ContractBuilder {
        self.outputs = outputs;
        self
    }

    pub fn set_creator(mut self, creator: String) -> ContractBuilder {
        self.creator = Some(creator);
        self
    }

    pub fn set_contract(mut self, contract: Vec<u8>) -> ContractBuilder {
        self.contract = contract;
        self
    }

    pub fn build(self) -> Result<Contract, ContractBuildError> {
        let name = self.name.ok_or_else(|| {
            ContractBuildError::MissingField("'name' field is required".to_string())
        })?;

        let version = self.version.ok_or_else(|| {
            ContractBuildError::MissingField("'version' field is required".to_string())
        })?;

        let creator = self.creator.ok_or_else(|| {
            ContractBuildError::MissingField("'version' field is required".to_string())
        })?;

        let inputs = {
            if !self.inputs.is_empty() {
                self.inputs
            } else {
                return Err(ContractBuildError::MissingField(
                    "'inputs' field is required".to_string(),
                ));
            }
        };

        let outputs = {
            if !self.outputs.is_empty() {
                self.outputs
            } else {
                return Err(ContractBuildError::MissingField(
                    "'outputs' field is required".to_string(),
                ));
            }
        };

        let contract = {
            if !self.contract.is_empty() {
                self.contract
            } else {
                return Err(ContractBuildError::MissingField(
                    "'contract' field is required".to_string(),
                ));
            }
        };

        Ok(Contract {
            name,
            version,
            inputs,
            outputs,
            creator,
            contract,
        })
    }
}

// Native implementation for ContractList
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ContractList {
    contracts: Vec<Contract>,
}

impl ContractList {
    pub fn get_contracts(&self) -> &[Contract] {
        &self.contracts
    }
}

impl FromProto<protos::contract::ContractList> for ContractList {
    fn from_proto(proto: protos::contract::ContractList) -> Result<Self, ProtoConversionError> {
        Ok(ContractList {
            contracts: proto
                .get_contracts()
                .to_vec()
                .into_iter()
                .map(Contract::from_proto)
                .collect::<Result<Vec<Contract>, ProtoConversionError>>()?,
        })
    }
}

impl FromNative<ContractList> for protos::contract::ContractList {
    fn from_native(contract_list: ContractList) -> Result<Self, ProtoConversionError> {
        let mut proto = protos::contract::ContractList::new();
        proto.set_contracts(RepeatedField::from_vec(
            contract_list
                .get_contracts()
                .to_vec()
                .into_iter()
                .map(Contract::into_proto)
                .collect::<Result<Vec<protos::contract::Contract>, ProtoConversionError>>()?,
        ));

        Ok(proto)
    }
}

impl FromBytes<ContractList> for ContractList {
    fn from_bytes(bytes: &[u8]) -> Result<ContractList, ProtoConversionError> {
        let proto: protos::contract::ContractList =
            protobuf::parse_from_bytes(bytes).map_err(|_| {
                ProtoConversionError::SerializationError(
                    "Unable to get ContractList from bytes".to_string(),
                )
            })?;
        proto.into_native()
    }
}

impl IntoBytes for ContractList {
    fn into_bytes(self) -> Result<Vec<u8>, ProtoConversionError> {
        let proto = self.into_proto()?;
        let bytes = proto.write_to_bytes().map_err(|_| {
            ProtoConversionError::SerializationError(
                "Unable to get bytes from ContractList".to_string(),
            )
        })?;
        Ok(bytes)
    }
}

impl IntoProto<protos::contract::ContractList> for ContractList {}
impl IntoNative<ContractList> for protos::contract::ContractList {}

#[derive(Debug)]
pub enum ContractListBuildError {
    MissingField(String),
}

impl StdError for ContractListBuildError {
    fn description(&self) -> &str {
        match *self {
            ContractListBuildError::MissingField(ref msg) => msg,
        }
    }
}

impl std::fmt::Display for ContractListBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ContractListBuildError::MissingField(ref s) => write!(f, "MissingField: {}", s),
        }
    }
}

/// Builder used to create a ContractList
#[derive(Default, Clone)]
pub struct ContractListBuilder {
    contracts: Vec<Contract>,
}

impl ContractListBuilder {
    pub fn new() -> Self {
        ContractListBuilder::default()
    }

    pub fn set_contracts(mut self, contracts: Vec<Contract>) -> ContractListBuilder {
        self.contracts = contracts;
        self
    }

    pub fn build(self) -> Result<ContractList, ContractListBuildError> {
        let contracts = self.contracts;

        Ok(ContractList { contracts })
    }
}

/// Native implementation for SmartPermission
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SmartPermission {
    name: String,
    org_id: String,
    function: Vec<u8>,
}

impl SmartPermission {
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_org_id(&self) -> &String {
        &self.org_id
    }

    pub fn get_function(&self) -> &[u8] {
        &self.function
    }

    pub fn into_builder(self) -> SmartPermissionBuilder {
        SmartPermissionBuilder::new()
            .set_name(self.name)
            .set_org_id(self.org_id)
            .set_function(self.function)
    }
}

impl FromProto<protos::smart_permission::SmartPermission> for SmartPermission {
    fn from_proto(
        proto: protos::smart_permission::SmartPermission,
    ) -> Result<Self, ProtoConversionError> {
        Ok(SmartPermission {
            name: proto.get_name().to_string(),
            org_id: proto.get_org_id().to_string(),
            function: proto.get_function().to_vec(),
        })
    }
}

impl FromNative<SmartPermission> for protos::smart_permission::SmartPermission {
    fn from_native(smart_permission: SmartPermission) -> Result<Self, ProtoConversionError> {
        let mut proto = protos::smart_permission::SmartPermission::new();

        proto.set_name(smart_permission.get_name().to_string());
        proto.set_org_id(smart_permission.get_org_id().to_string());
        proto.set_function(smart_permission.get_function().to_vec());

        Ok(proto)
    }
}

impl FromBytes<SmartPermission> for SmartPermission {
    fn from_bytes(bytes: &[u8]) -> Result<SmartPermission, ProtoConversionError> {
        let proto: protos::smart_permission::SmartPermission = protobuf::parse_from_bytes(bytes)
            .map_err(|_| {
                ProtoConversionError::SerializationError(
                    "Unable to get SmartPermission from bytes".to_string(),
                )
            })?;
        proto.into_native()
    }
}

impl IntoBytes for SmartPermission {
    fn into_bytes(self) -> Result<Vec<u8>, ProtoConversionError> {
        let proto = self.into_proto()?;
        let bytes = proto.write_to_bytes().map_err(|_| {
            ProtoConversionError::SerializationError(
                "Unable to get bytes from SmartPermission".to_string(),
            )
        })?;
        Ok(bytes)
    }
}

impl IntoProto<protos::smart_permission::SmartPermission> for SmartPermission {}
impl IntoNative<SmartPermission> for protos::smart_permission::SmartPermission {}

#[derive(Debug)]
pub enum SmartPermissionBuildError {
    MissingField(String),
}

impl StdError for SmartPermissionBuildError {
    fn description(&self) -> &str {
        match *self {
            SmartPermissionBuildError::MissingField(ref msg) => msg,
        }
    }
}

impl std::fmt::Display for SmartPermissionBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            SmartPermissionBuildError::MissingField(ref s) => write!(f, "MissingField: {}", s),
        }
    }
}

/// Builder used to create a SmartPermission
#[derive(Default, Clone)]
pub struct SmartPermissionBuilder {
    name: Option<String>,
    org_id: Option<String>,
    function: Vec<u8>,
}

impl SmartPermissionBuilder {
    pub fn new() -> Self {
        SmartPermissionBuilder::default()
    }

    pub fn set_name(mut self, name: String) -> SmartPermissionBuilder {
        self.name = Some(name);
        self
    }

    pub fn set_org_id(mut self, org_id: String) -> SmartPermissionBuilder {
        self.org_id = Some(org_id);
        self
    }

    pub fn set_function(mut self, function: Vec<u8>) -> SmartPermissionBuilder {
        self.function = function;
        self
    }

    pub fn build(self) -> Result<SmartPermission, SmartPermissionBuildError> {
        let name = self.name.ok_or_else(|| {
            SmartPermissionBuildError::MissingField("'name' field is required".to_string())
        })?;

        let org_id = self.org_id.ok_or_else(|| {
            SmartPermissionBuildError::MissingField("'org_id' field is required".to_string())
        })?;

        let function = {
            if !self.function.is_empty() {
                self.function
            } else {
                return Err(SmartPermissionBuildError::MissingField(
                    "'function' field is required".to_string(),
                ));
            }
        };

        Ok(SmartPermission {
            name,
            org_id,
            function,
        })
    }
}

// Native implementation for SmartPermissionList
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SmartPermissionList {
    smart_permissions: Vec<SmartPermission>,
}

impl SmartPermissionList {
    pub fn get_smart_permissions(&self) -> &[SmartPermission] {
        &self.smart_permissions
    }
}

impl FromProto<protos::smart_permission::SmartPermissionList> for SmartPermissionList {
    fn from_proto(
        proto: protos::smart_permission::SmartPermissionList,
    ) -> Result<Self, ProtoConversionError> {
        Ok(SmartPermissionList {
            smart_permissions: proto
                .get_smart_permissions()
                .to_vec()
                .into_iter()
                .map(SmartPermission::from_proto)
                .collect::<Result<Vec<SmartPermission>, ProtoConversionError>>()?,
        })
    }
}

impl FromNative<SmartPermissionList> for protos::smart_permission::SmartPermissionList {
    fn from_native(
        smart_permissions_list: SmartPermissionList,
    ) -> Result<Self, ProtoConversionError> {
        let mut proto = protos::smart_permission::SmartPermissionList::new();
        proto.set_smart_permissions(RepeatedField::from_vec(
            smart_permissions_list
                .get_smart_permissions()
                .to_vec()
                .into_iter()
                .map(SmartPermission::into_proto)
                .collect::<Result<Vec<protos::smart_permission::SmartPermission>, ProtoConversionError>>()?,
        ));

        Ok(proto)
    }
}

impl FromBytes<SmartPermissionList> for SmartPermissionList {
    fn from_bytes(bytes: &[u8]) -> Result<SmartPermissionList, ProtoConversionError> {
        let proto: protos::smart_permission::SmartPermissionList =
            protobuf::parse_from_bytes(bytes).map_err(|_| {
                ProtoConversionError::SerializationError(
                    "Unable to get SmartPermissionList from bytes".to_string(),
                )
            })?;
        proto.into_native()
    }
}

impl IntoBytes for SmartPermissionList {
    fn into_bytes(self) -> Result<Vec<u8>, ProtoConversionError> {
        let proto = self.into_proto()?;
        let bytes = proto.write_to_bytes().map_err(|_| {
            ProtoConversionError::SerializationError(
                "Unable to get bytes from SmartPermissionList".to_string(),
            )
        })?;
        Ok(bytes)
    }
}

impl IntoProto<protos::smart_permission::SmartPermissionList> for SmartPermissionList {}
impl IntoNative<SmartPermissionList> for protos::smart_permission::SmartPermissionList {}

#[derive(Debug)]
pub enum SmartPermissionListBuildError {
    MissingField(String),
}

impl StdError for SmartPermissionListBuildError {
    fn description(&self) -> &str {
        match *self {
            SmartPermissionListBuildError::MissingField(ref msg) => msg,
        }
    }
}

impl std::fmt::Display for SmartPermissionListBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            SmartPermissionListBuildError::MissingField(ref s) => write!(f, "MissingField: {}", s),
        }
    }
}

/// Builder used to create a SmartPermissionList
#[derive(Default, Clone)]
pub struct SmartPermissionListBuilder {
    smart_permissions: Vec<SmartPermission>,
}

impl SmartPermissionListBuilder {
    pub fn new() -> Self {
        SmartPermissionListBuilder::default()
    }

    pub fn set_smart_permissions(
        mut self,
        smart_permissions: Vec<SmartPermission>,
    ) -> SmartPermissionListBuilder {
        self.smart_permissions = smart_permissions;
        self
    }

    pub fn build(self) -> Result<SmartPermissionList, SmartPermissionListBuildError> {
        let smart_permissions = self.smart_permissions;

        Ok(SmartPermissionList { smart_permissions })
    }
}

/// Native implementation for KeyValueEntry
#[derive(Debug, Clone, PartialEq)]
pub struct KeyValueEntry {
    key: String,
    value: String,
}

impl KeyValueEntry {
    pub fn get_key(&self) -> &str {
        &self.key
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }
}

impl FromProto<protos::account::KeyValueEntry> for KeyValueEntry {
    fn from_proto(
        key_value: protos::account::KeyValueEntry,
    ) -> Result<Self, ProtoConversionError> {
        Ok(KeyValueEntry {
            key: key_value.get_key().to_string(),
            value: key_value.get_value().to_string(),
        })
    }
}

impl FromNative<KeyValueEntry> for protos::account::KeyValueEntry {
    fn from_native(key_value: KeyValueEntry) -> Result<Self, ProtoConversionError> {
        let mut key_value_proto = protos::account::KeyValueEntry::new();

        key_value_proto.set_key(key_value.get_key().to_string());
        key_value_proto.set_value(key_value.get_value().to_string());

        Ok(key_value_proto)
    }
}

impl FromBytes<KeyValueEntry> for KeyValueEntry {
    fn from_bytes(bytes: &[u8]) -> Result<KeyValueEntry, ProtoConversionError> {
        let proto: protos::account::KeyValueEntry =
            protobuf::parse_from_bytes(bytes).map_err(|_| {
                ProtoConversionError::SerializationError(
                    "Unable to get KeyValueEntry from bytes".to_string(),
                )
            })?;
        proto.into_native()
    }
}

impl IntoBytes for KeyValueEntry {
    fn into_bytes(self) -> Result<Vec<u8>, ProtoConversionError> {
        let proto = self.into_proto()?;
        let bytes = proto.write_to_bytes().map_err(|_| {
            ProtoConversionError::SerializationError(
                "Unable to get bytes from KeyValueEntry".to_string(),
            )
        })?;
        Ok(bytes)
    }
}

impl IntoProto<protos::account::KeyValueEntry> for KeyValueEntry {}
impl IntoNative<KeyValueEntry> for protos::account::KeyValueEntry {}

#[derive(Debug)]
pub enum KeyValueEntryBuildError {
    MissingField(String),
}

impl StdError for KeyValueEntryBuildError {
    fn description(&self) -> &str {
        match *self {
            KeyValueEntryBuildError::MissingField(ref msg) => msg,
        }
    }
}

impl std::fmt::Display for KeyValueEntryBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            KeyValueEntryBuildError::MissingField(ref s) => write!(f, "MissingField: {}", s),
        }
    }
}

/// Builder used to create a KeyValueEntry
#[derive(Default, Clone)]
pub struct KeyValueEntryBuilder {
    pub key: Option<String>,
    pub value: Option<String>,
}

impl KeyValueEntryBuilder {
    pub fn new() -> Self {
        KeyValueEntryBuilder::default()
    }

    pub fn set_key(mut self, key: String) -> KeyValueEntryBuilder {
        self.key = Some(key);
        self
    }

    pub fn set_value(mut self, value: String) -> KeyValueEntryBuilder {
        self.value = Some(value);
        self
    }

    pub fn build(self) -> Result<KeyValueEntry, KeyValueEntryBuildError> {
        let key = self.key.ok_or_else(|| {
            KeyValueEntryBuildError::MissingField("'key' field is required".to_string())
        })?;

        let value = self.value.ok_or_else(|| {
            KeyValueEntryBuildError::MissingField("'value' field is required".to_string())
        })?;

        Ok(KeyValueEntry { key, value })
    }
}

/// Native implementation of Account
#[derive(Debug, Clone, PartialEq)]
pub struct Account {
    org_id: String,
    public_key: String,
    active: bool,
    roles: Vec<String>,
    metadata: Vec<KeyValueEntry>,
}

impl Account {
    pub fn get_org_id(&self) -> &str {
        &self.org_id
    }

    pub fn get_public_key(&self) -> &str {
        &self.public_key
    }

    pub fn get_active(&self) -> &bool {
        &self.active
    }

    pub fn get_roles(&self) -> &[String] {
        &self.roles
    }

    pub fn get_metadata(&self) -> &[KeyValueEntry] {
        &self.metadata
    }
}

impl FromProto<protos::account::Account> for Account {
    fn from_proto(account: protos::account::Account) -> Result<Self, ProtoConversionError> {
        Ok(Account {
            org_id: account.get_org_id().to_string(),
            public_key: account.get_public_key().to_string(),
            active: account.get_active(),
            roles: account.get_roles().to_vec(),
            metadata: account
                .get_metadata()
                .to_vec()
                .into_iter()
                .map(KeyValueEntry::from_proto)
                .collect::<Result<Vec<KeyValueEntry>, ProtoConversionError>>()?,
        })
    }
}

impl FromNative<Account> for protos::account::Account {
    fn from_native(account: Account) -> Result<Self, ProtoConversionError> {
        let mut account_proto = protos::account::Account::new();

        account_proto.set_org_id(account.get_org_id().to_string());
        account_proto.set_public_key(account.get_public_key().to_string());
        account_proto.set_active(account.get_active().clone());
        account_proto.set_org_id(account.get_org_id().to_string());
        account_proto.set_roles(RepeatedField::from_vec(account.get_roles().to_vec()));
        account_proto.set_metadata(RepeatedField::from_vec(
            account
                .get_metadata()
                .to_vec()
                .into_iter()
                .map(KeyValueEntry::into_proto)
                .collect::<Result<Vec<protos::account::KeyValueEntry>, ProtoConversionError>>(
                )?,
        ));

        Ok(account_proto)
    }
}

impl FromBytes<Account> for Account {
    fn from_bytes(bytes: &[u8]) -> Result<Account, ProtoConversionError> {
        let proto: protos::account::Account = protobuf::parse_from_bytes(bytes).map_err(|_| {
            ProtoConversionError::SerializationError("Unable to get Account from bytes".to_string())
        })?;
        proto.into_native()
    }
}

impl IntoBytes for Account {
    fn into_bytes(self) -> Result<Vec<u8>, ProtoConversionError> {
        let proto = self.into_proto()?;
        let bytes = proto.write_to_bytes().map_err(|_| {
            ProtoConversionError::SerializationError("Unable to get bytes from Account".to_string())
        })?;
        Ok(bytes)
    }
}

impl IntoProto<protos::account::Account> for Account {}
impl IntoNative<Account> for protos::account::Account {}

#[derive(Debug)]
pub enum AccountBuildError {
    MissingField(String),
}

impl StdError for AccountBuildError {
    fn description(&self) -> &str {
        match *self {
            AccountBuildError::MissingField(ref msg) => msg,
        }
    }
}

impl std::fmt::Display for AccountBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            AccountBuildError::MissingField(ref s) => write!(f, "MissingField: {}", s),
        }
    }
}

/// Builder used to create an Account
#[derive(Default, Clone)]
pub struct AccountBuilder {
    pub org_id: Option<String>,
    pub public_key: Option<String>,
    pub active: Option<bool>,
    pub roles: Vec<String>,
    pub metadata: Vec<KeyValueEntry>,
}

impl AccountBuilder {
    pub fn new() -> Self {
        AccountBuilder::default()
    }

    pub fn set_org_id(mut self, org_id: String) -> AccountBuilder {
        self.org_id = Some(org_id);
        self
    }

    pub fn set_public_key(mut self, public_key: String) -> AccountBuilder {
        self.public_key = Some(public_key);
        self
    }

    pub fn set_active(mut self, active: bool) -> AccountBuilder {
        self.active = Some(active);
        self
    }

    pub fn set_roles(mut self, roles: Vec<String>) -> AccountBuilder {
        self.roles = roles;
        self
    }

    pub fn set_metadata(mut self, metadata: Vec<KeyValueEntry>) -> AccountBuilder {
        self.metadata = metadata;
        self
    }

    pub fn build(self) -> Result<Account, AccountBuildError> {
        let org_id = self.org_id.ok_or_else(|| {
            AccountBuildError::MissingField("'org_id' field is required".to_string())
        })?;

        let public_key = self.public_key.ok_or_else(|| {
            AccountBuildError::MissingField("'public_key' field is required".to_string())
        })?;

        let active = self.active.unwrap_or_default();
        let roles = self.roles;
        let metadata = self.metadata;

        Ok(Account {
            org_id,
            public_key,
            active,
            roles,
            metadata,
        })
    }
}

/// Native implementation of AccountList
#[derive(Debug, Clone, PartialEq)]
pub struct AccountList {
    accounts: Vec<Account>,
}

impl AccountList {
    pub fn get_accounts(&self) -> &[Account] {
        &self.accounts
    }
}

impl FromProto<protos::account::AccountList> for AccountList {
    fn from_proto(account_list: protos::account::AccountList) -> Result<Self, ProtoConversionError> {
        Ok(AccountList {
            accounts: account_list
                .get_accounts()
                .to_vec()
                .into_iter()
                .map(Account::from_proto)
                .collect::<Result<Vec<Account>, ProtoConversionError>>()?,
        })
    }
}

impl FromNative<AccountList> for protos::account::AccountList {
    fn from_native(account_list: AccountList) -> Result<Self, ProtoConversionError> {
        let mut account_list_proto = protos::account::AccountList::new();

        account_list_proto.set_accounts(RepeatedField::from_vec(
            account_list
                .get_accounts()
                .to_vec()
                .into_iter()
                .map(Account::into_proto)
                .collect::<Result<Vec<protos::account::Account>, ProtoConversionError>>()?,
        ));

        Ok(account_list_proto)
    }
}

impl FromBytes<AccountList> for AccountList {
    fn from_bytes(bytes: &[u8]) -> Result<AccountList, ProtoConversionError> {
        let proto: protos::account::AccountList =
            protobuf::parse_from_bytes(bytes).map_err(|_| {
                ProtoConversionError::SerializationError(
                    "Unable to get AccountList from bytes".to_string(),
                )
            })?;
        proto.into_native()
    }
}

impl IntoBytes for AccountList {
    fn into_bytes(self) -> Result<Vec<u8>, ProtoConversionError> {
        let proto = self.into_proto()?;
        let bytes = proto.write_to_bytes().map_err(|_| {
            ProtoConversionError::SerializationError(
                "Unable to get bytes from AccountList".to_string(),
            )
        })?;
        Ok(bytes)
    }
}

impl IntoProto<protos::account::AccountList> for AccountList {}
impl IntoNative<AccountList> for protos::account::AccountList {}

#[derive(Debug)]
pub enum AccountListBuildError {
    MissingField(String),
}

impl StdError for AccountListBuildError {
    fn description(&self) -> &str {
        match *self {
            AccountListBuildError::MissingField(ref msg) => msg,
        }
    }
}

impl std::fmt::Display for AccountListBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            AccountListBuildError::MissingField(ref s) => write!(f, "MissingField: {}", s),
        }
    }
}

/// Builder used to create an AccountList
#[derive(Default, Clone)]
pub struct AccountListBuilder {
    pub accounts: Vec<Account>,
}

impl AccountListBuilder {
    pub fn new() -> Self {
        AccountListBuilder::default()
    }

    pub fn set_accounts(mut self, accounts: Vec<Account>) -> AccountListBuilder {
        self.accounts = accounts;
        self
    }

    pub fn build(self) -> Result<AccountList, AccountListBuildError> {
        let accounts = {
            if self.accounts.is_empty() {
                return Err(AccountListBuildError::MissingField(
                    "'accounts' cannot be empty".to_string(),
                ));
            } else {
                self.accounts
            }
        };

        Ok(AccountList { accounts })
    }
}

/// Native implementation for Organization
#[derive(Debug, Clone, PartialEq)]
pub struct Organization {
    org_id: String,
    name: String,
    address: String,
    metadata: Vec<KeyValueEntry>,
}

impl Organization {
    pub fn get_org_id(&self) -> &str {
        &self.org_id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_address(&self) -> &str {
        &self.address
    }

    pub fn get_metadata(&self) -> &[KeyValueEntry] {
        &self.metadata
    }
}

impl FromProto<protos::account::Organization> for Organization {
    fn from_proto(org: protos::account::Organization) -> Result<Self, ProtoConversionError> {
        Ok(Organization {
            org_id: org.get_org_id().to_string(),
            name: org.get_name().to_string(),
            address: org.get_address().to_string(),
            metadata: org
                .get_metadata()
                .to_vec()
                .into_iter()
                .map(KeyValueEntry::from_proto)
                .collect::<Result<Vec<KeyValueEntry>, ProtoConversionError>>()?,
        })
    }
}

impl FromNative<Organization> for protos::account::Organization {
    fn from_native(org: Organization) -> Result<Self, ProtoConversionError> {
        let mut org_proto = protos::account::Organization::new();

        org_proto.set_org_id(org.get_org_id().to_string());
        org_proto.set_name(org.get_name().to_string());
        org_proto.set_address(org.get_address().to_string());
        org_proto.set_metadata(RepeatedField::from_vec(
            org.get_metadata()
                .to_vec()
                .into_iter()
                .map(KeyValueEntry::into_proto)
                .collect::<Result<Vec<protos::account::KeyValueEntry>, ProtoConversionError>>(
                )?,
        ));

        Ok(org_proto)
    }
}

impl FromBytes<Organization> for Organization {
    fn from_bytes(bytes: &[u8]) -> Result<Organization, ProtoConversionError> {
        let proto: protos::account::Organization =
            protobuf::parse_from_bytes(bytes).map_err(|_| {
                ProtoConversionError::SerializationError(
                    "Unable to get Organization from bytes".to_string(),
                )
            })?;
        proto.into_native()
    }
}

impl IntoBytes for Organization {
    fn into_bytes(self) -> Result<Vec<u8>, ProtoConversionError> {
        let proto = self.into_proto()?;
        let bytes = proto.write_to_bytes().map_err(|_| {
            ProtoConversionError::SerializationError(
                "Unable to get bytes from Organization".to_string(),
            )
        })?;
        Ok(bytes)
    }
}

impl IntoProto<protos::account::Organization> for Organization {}
impl IntoNative<Organization> for protos::account::Organization {}

#[derive(Debug)]
pub enum OrganizationBuildError {
    MissingField(String),
}

impl StdError for OrganizationBuildError {
    fn description(&self) -> &str {
        match *self {
            OrganizationBuildError::MissingField(ref msg) => msg,
        }
    }
}

impl std::fmt::Display for OrganizationBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            OrganizationBuildError::MissingField(ref s) => write!(f, "MissingField: {}", s),
        }
    }
}

/// Builder used to create an Organization
#[derive(Default, Clone)]
pub struct OrganizationBuilder {
    pub org_id: Option<String>,
    pub name: Option<String>,
    pub address: Option<String>,
    pub metadata: Vec<KeyValueEntry>,
}

impl OrganizationBuilder {
    pub fn new() -> Self {
        OrganizationBuilder::default()
    }

    pub fn set_org_id(mut self, org_id: String) -> OrganizationBuilder {
        self.org_id = Some(org_id);
        self
    }

    pub fn set_name(mut self, name: String) -> OrganizationBuilder {
        self.name = Some(name);
        self
    }

    pub fn set_address(mut self, name: String) -> OrganizationBuilder {
        self.address = Some(name);
        self
    }

    pub fn set_metadata(mut self, metadata: Vec<KeyValueEntry>) -> OrganizationBuilder {
        self.metadata = metadata;
        self
    }

    pub fn build(self) -> Result<Organization, OrganizationBuildError> {
        let org_id = self.org_id.ok_or_else(|| {
            OrganizationBuildError::MissingField("'org_id' field is required".to_string())
        })?;

        let name = self.name.ok_or_else(|| {
            OrganizationBuildError::MissingField("'name' field is required".to_string())
        })?;

        let address = self.address.ok_or_else(|| {
            OrganizationBuildError::MissingField("'address' field is required".to_string())
        })?;

        let metadata = self.metadata;

        Ok(Organization {
            org_id,
            name,
            address,
            metadata,
        })
    }
}

/// Native implementation of OrganizationList
#[derive(Debug, Clone, PartialEq)]
pub struct OrganizationList {
    organizations: Vec<Organization>,
}

impl OrganizationList {
    pub fn get_organizations(&self) -> &[Organization] {
        &self.organizations
    }
}

impl FromProto<protos::account::OrganizationList> for OrganizationList {
    fn from_proto(
        organization_list: protos::account::OrganizationList,
    ) -> Result<Self, ProtoConversionError> {
        Ok(OrganizationList {
            organizations: organization_list
                .get_organizations()
                .to_vec()
                .into_iter()
                .map(Organization::from_proto)
                .collect::<Result<Vec<Organization>, ProtoConversionError>>()?,
        })
    }
}

impl FromNative<OrganizationList> for protos::account::OrganizationList {
    fn from_native(org_list: OrganizationList) -> Result<Self, ProtoConversionError> {
        let mut org_list_proto = protos::account::OrganizationList::new();

        org_list_proto.set_organizations(RepeatedField::from_vec(
            org_list
                .get_organizations()
                .to_vec()
                .into_iter()
                .map(Organization::into_proto)
                .collect::<Result<Vec<protos::account::Organization>, ProtoConversionError>>()?,
        ));

        Ok(org_list_proto)
    }
}

impl FromBytes<OrganizationList> for OrganizationList {
    fn from_bytes(bytes: &[u8]) -> Result<OrganizationList, ProtoConversionError> {
        let proto: protos::account::OrganizationList = protobuf::parse_from_bytes(bytes)
            .map_err(|_| {
                ProtoConversionError::SerializationError(
                    "Unable to get OrganizationList from bytes".to_string(),
                )
            })?;
        proto.into_native()
    }
}

impl IntoBytes for OrganizationList {
    fn into_bytes(self) -> Result<Vec<u8>, ProtoConversionError> {
        let proto = self.into_proto()?;
        let bytes = proto.write_to_bytes().map_err(|_| {
            ProtoConversionError::SerializationError(
                "Unable to get bytes from OrganizationList".to_string(),
            )
        })?;
        Ok(bytes)
    }
}

impl IntoProto<protos::account::OrganizationList> for OrganizationList {}
impl IntoNative<OrganizationList> for protos::account::OrganizationList {}

#[derive(Debug)]
pub enum OrganizationListBuildError {
    MissingField(String),
}

impl StdError for OrganizationListBuildError {
    fn description(&self) -> &str {
        match *self {
            OrganizationListBuildError::MissingField(ref msg) => msg,
        }
    }
}

impl std::fmt::Display for OrganizationListBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            OrganizationListBuildError::MissingField(ref s) => write!(f, "MissingField: {}", s),
        }
    }
}

/// Builder used to create an OrganizationList
#[derive(Default, Clone)]
pub struct OrganizationListBuilder {
    pub organizations: Vec<Organization>,
}

impl OrganizationListBuilder {
    pub fn new() -> Self {
        OrganizationListBuilder::default()
    }

    pub fn set_organizations(
        mut self,
        organizations: Vec<Organization>,
    ) -> OrganizationListBuilder {
        self.organizations = organizations;
        self
    }

    pub fn build(self) -> Result<OrganizationList, OrganizationListBuildError> {
        let organizations = {
            if self.organizations.is_empty() {
                return Err(OrganizationListBuildError::MissingField(
                    "'organization' cannot be empty".to_string(),
                ));
            } else {
                self.organizations
            }
        };

        Ok(OrganizationList { organizations })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // check that a contract registry is built correctly
    fn check_contract_registry() {
        let builder = VersionBuilder::new();
        let version = builder
            .set_version("0.0.0".to_string())
            .set_contract_sha512("sha512".to_string())
            .set_creator("The Creator".to_string())
            .build()
            .unwrap();

        let builder = ContractRegistryBuilder::new();
        let contract_registry = builder
            .set_name("Tests".to_string())
            .set_versions(vec![version.clone()])
            .set_owners(vec!["owner".to_string()])
            .build()
            .unwrap();

        assert_eq!(contract_registry.get_name(), "Tests");
        assert_eq!(contract_registry.get_versions(), [version]);
        assert_eq!(contract_registry.get_owners(), ["owner"]);
    }

    #[test]
    // check that a contract registry can be converted to bytes and back
    fn check_contract_registry_bytes() {
        let builder = VersionBuilder::new();
        let version = builder
            .set_version("0.0.0".to_string())
            .set_contract_sha512("sha512".to_string())
            .set_creator("The Creator".to_string())
            .build()
            .unwrap();

        let builder = ContractRegistryBuilder::new();
        let original = builder
            .set_name("Tests".to_string())
            .set_versions(vec![version.clone()])
            .set_owners(vec!["owner".to_string()])
            .build()
            .unwrap();

        let bytes = original.clone().into_bytes().unwrap();

        let contract_registry = ContractRegistry::from_bytes(&bytes).unwrap();
        assert_eq!(contract_registry, original);
    }

    #[test]
    // check that a contract registry can be converted into builder
    fn check_contract_registry_into_builder() {
        let builder = VersionBuilder::new();
        let version = builder
            .set_version("0.0.0".to_string())
            .set_contract_sha512("sha512".to_string())
            .set_creator("The Creator".to_string())
            .build()
            .unwrap();

        let builder = ContractRegistryBuilder::new();
        let contract_registry = builder
            .set_name("Tests".to_string())
            .set_versions(vec![version.clone()])
            .set_owners(vec!["owner".to_string()])
            .build()
            .unwrap();

        let builder = contract_registry.into_builder();

        assert_eq!(builder.name, Some("Tests".to_string()));
        assert_eq!(builder.versions, [version]);
        assert_eq!(builder.owners, ["owner"]);
    }

    #[test]
    // check that a contract registry list is built correctly
    fn check_contract_registry_list() {
        let builder = VersionBuilder::new();
        let version = builder
            .set_version("0.0.0".to_string())
            .set_contract_sha512("sha512".to_string())
            .set_creator("The Creator".to_string())
            .build()
            .unwrap();

        let builder = ContractRegistryBuilder::new();
        let contract_registry = builder
            .set_name("Tests".to_string())
            .set_versions(vec![version.clone()])
            .set_owners(vec!["owner".to_string()])
            .build()
            .unwrap();

        let build = ContractRegistryListBuilder::new();
        let contract_registry_list = build
            .set_registries(vec![contract_registry.clone()])
            .build()
            .unwrap();

        assert_eq!(contract_registry_list.get_registries(), [contract_registry]);
    }

    #[test]
    // check that a contract registry list can be converted to bytes and back
    fn check_contract_registry_bytes_list() {
        let builder = VersionBuilder::new();
        let version = builder
            .set_version("0.0.0".to_string())
            .set_contract_sha512("sha512".to_string())
            .set_creator("The Creator".to_string())
            .build()
            .unwrap();

        let builder = ContractRegistryBuilder::new();
        let contract_registry = builder
            .set_name("Tests".to_string())
            .set_versions(vec![version.clone()])
            .set_owners(vec!["owner".to_string()])
            .build()
            .unwrap();

        let build = ContractRegistryListBuilder::new();
        let original = build
            .set_registries(vec![contract_registry.clone()])
            .build()
            .unwrap();

        let bytes = original.clone().into_bytes().unwrap();

        let contract_registry_list = ContractRegistryList::from_bytes(&bytes).unwrap();
        assert_eq!(contract_registry_list, original);
    }

    #[test]
    // check that a namespace registry is built correctly
    fn check_namespace_registry() {
        let builder = PermissionBuilder::new();
        let permission = builder
            .set_contract_name("Test".to_string())
            .set_read(true)
            .set_write(true)
            .build()
            .unwrap();

        let builder = NamespaceRegistryBuilder::new();
        let namespace_registry = builder
            .set_namespace("Tests".to_string())
            .set_owners(vec!["owner".to_string()])
            .set_permissions(vec![permission.clone()])
            .build()
            .unwrap();

        assert_eq!(namespace_registry.get_namespace(), "Tests");
        assert_eq!(namespace_registry.get_permissions(), [permission]);
        assert_eq!(namespace_registry.get_owners(), ["owner"]);
    }

    #[test]
    // check that a namespace registry can be converted to bytes and back
    fn check_namespace_registry_bytes() {
        let builder = PermissionBuilder::new();
        let permission = builder
            .set_contract_name("Test".to_string())
            .set_read(true)
            .set_write(true)
            .build()
            .unwrap();

        let builder = NamespaceRegistryBuilder::new();
        let original = builder
            .set_namespace("Tests".to_string())
            .set_owners(vec!["owner".to_string()])
            .set_permissions(vec![permission.clone()])
            .build()
            .unwrap();

        let bytes = original.clone().into_bytes().unwrap();

        let namespace_registry = NamespaceRegistry::from_bytes(&bytes).unwrap();
        assert_eq!(namespace_registry, original);
    }

    #[test]
    // check that a namespace registry can be conveted into a builder
    fn check_namespace_registry_into_build() {
        let builder = PermissionBuilder::new();
        let permission = builder
            .set_contract_name("Test".to_string())
            .set_read(true)
            .set_write(true)
            .build()
            .unwrap();

        let builder = NamespaceRegistryBuilder::new();
        let namespace_registry = builder
            .set_namespace("Tests".to_string())
            .set_owners(vec!["owner".to_string()])
            .set_permissions(vec![permission.clone()])
            .build()
            .unwrap();

        let builder = namespace_registry.into_builder();

        assert_eq!(builder.namespace, Some("Tests".to_string()));
        assert_eq!(builder.permissions, [permission]);
        assert_eq!(builder.owners, ["owner"]);
    }

    #[test]
    // check that a namespace registry list is built correctly
    fn check_namespace_registry_list() {
        let builder = PermissionBuilder::new();
        let permission = builder
            .set_contract_name("Test".to_string())
            .set_read(true)
            .set_write(true)
            .build()
            .unwrap();

        let builder = NamespaceRegistryBuilder::new();
        let namespace_registry = builder
            .set_namespace("Tests".to_string())
            .set_owners(vec!["owner".to_string()])
            .set_permissions(vec![permission.clone()])
            .build()
            .unwrap();

        let build = NamespaceRegistryListBuilder::new();
        let namespace_registry_list = build
            .set_registries(vec![namespace_registry.clone()])
            .build()
            .unwrap();

        assert_eq!(namespace_registry_list.get_registries(), [namespace_registry]);
    }

    #[test]
    // check that a namespace registry list can be converted to bytes and back
    fn check_namespace_registry_bytes_list() {
        let builder = PermissionBuilder::new();
        let permission = builder
            .set_contract_name("Test".to_string())
            .set_read(true)
            .set_write(true)
            .build()
            .unwrap();

        let builder = NamespaceRegistryBuilder::new();
        let namespace_registry = builder
            .set_namespace("Tests".to_string())
            .set_owners(vec!["owner".to_string()])
            .set_permissions(vec![permission.clone()])
            .build()
            .unwrap();

        let build = NamespaceRegistryListBuilder::new();
        let original = build
            .set_registries(vec![namespace_registry.clone()])
            .build()
            .unwrap();

        let bytes = original.clone().into_bytes().unwrap();

        let namespace_registry_list = NamespaceRegistryList::from_bytes(&bytes).unwrap();
        assert_eq!(namespace_registry_list, original);
    }

    #[test]
    // check that a contract is built correctly
    fn check_contract() {
        let builder = ContractBuilder::new();
        let contract = builder
            .set_name("Tests".to_string())
            .set_version("0.0.0".to_string())
            .set_inputs(vec!["input1".to_string(), "input2".to_string()])
            .set_outputs(vec!["output1".to_string(), "output2".to_string()])
            .set_creator("The Creator".to_string())
            .set_contract(b"test_contract".to_vec())
            .build()
            .unwrap();

        assert_eq!(contract.get_name(), "Tests");
        assert_eq!(contract.get_version(), "0.0.0");
        assert_eq!(
            contract.get_inputs(),
            ["input1".to_string(), "input2".to_string()]
        );
        assert_eq!(
            contract.get_outputs(),
            ["output1".to_string(), "output2".to_string()]
        );
        assert_eq!(contract.get_creator(), "The Creator");
        assert_eq!(contract.get_contract(), b"test_contract");
    }

    #[test]
    // check that a contract can be converted to bytes and back
    fn check_contract_bytes() {
        let builder = ContractBuilder::new();
        let original = builder
            .set_name("Tests".to_string())
            .set_version("0.0.0".to_string())
            .set_inputs(vec!["input1".to_string(), "input2".to_string()])
            .set_outputs(vec!["output1".to_string(), "output2".to_string()])
            .set_creator("The Creator".to_string())
            .set_contract(b"test_contract".to_vec())
            .build()
            .unwrap();

        let bytes = original.clone().into_bytes().unwrap();

        let contract = Contract::from_bytes(&bytes).unwrap();
        assert_eq!(contract, original);
    }

    #[test]
    // check that the contract can be converted into a builder
    fn check_contract_into_builder() {
        let builder = ContractBuilder::new();
        let contract = builder
            .set_name("Tests".to_string())
            .set_version("0.0.0".to_string())
            .set_inputs(vec!["input1".to_string(), "input2".to_string()])
            .set_outputs(vec!["output1".to_string(), "output2".to_string()])
            .set_creator("The Creator".to_string())
            .set_contract(b"test_contract".to_vec())
            .build()
            .unwrap();

        let builder = contract.into_builder();

        assert_eq!(builder.name, Some("Tests".to_string()));
        assert_eq!(builder.version, Some("0.0.0".to_string()));
        assert_eq!(builder.inputs, ["input1".to_string(), "input2".to_string()]);
        assert_eq!(
            builder.outputs,
            ["output1".to_string(), "output2".to_string()]
        );
        assert_eq!(builder.creator, Some("The Creator".to_string()));
        assert_eq!(builder.contract, b"test_contract".to_vec());
    }

    #[test]
    // check that a contract list is built correctly
    fn check_contract_list() {
        let builder = ContractBuilder::new();
        let contract = builder
            .set_name("Tests".to_string())
            .set_version("0.0.0".to_string())
            .set_inputs(vec!["input1".to_string(), "input2".to_string()])
            .set_outputs(vec!["output1".to_string(), "output2".to_string()])
            .set_creator("The Creator".to_string())
            .set_contract(b"test_contract".to_vec())
            .build()
            .unwrap();

        let builder = ContractListBuilder::new();
        let contract_list = builder
            .set_contracts(vec![contract.clone()])
            .build()
            .unwrap();

        assert_eq!(contract_list.get_contracts(), [contract]);
    }

    #[test]
    // check that a contract list can be converted to bytes and back
    fn check_contract_list_bytes() {
        let builder = ContractBuilder::new();
        let contract = builder
            .set_name("Tests".to_string())
            .set_version("0.0.0".to_string())
            .set_inputs(vec!["input1".to_string(), "input2".to_string()])
            .set_outputs(vec!["output1".to_string(), "output2".to_string()])
            .set_creator("The Creator".to_string())
            .set_contract(b"test_contract".to_vec())
            .build()
            .unwrap();

        let builder = ContractListBuilder::new();
        let original = builder
            .set_contracts(vec![contract.clone()])
            .build()
            .unwrap();

        let bytes = original.clone().into_bytes().unwrap();

        let contract_list = ContractList::from_bytes(&bytes).unwrap();
        assert_eq!(contract_list, original);
    }

    #[test]
    // check that a smart permission is built correctly
    fn check_smart_permission() {
        let builder = SmartPermissionBuilder::new();
        let smart_permission = builder
            .set_name("Tests".to_string())
            .set_org_id("org_id".to_string())
            .set_function(b"test_function".to_vec())
            .build()
            .unwrap();

        assert_eq!(smart_permission.get_name(), "Tests");
        assert_eq!(smart_permission.get_org_id(), "org_id");
        assert_eq!(smart_permission.get_function(), b"test_function");
    }

    #[test]
    // check that a smart permission can be converted to bytes and back
    fn check_smart_permission_bytes() {
        let builder = SmartPermissionBuilder::new();
        let original = builder
            .set_name("Tests".to_string())
            .set_org_id("org_id".to_string())
            .set_function(b"test_function".to_vec())
            .build()
            .unwrap();

        let bytes = original.clone().into_bytes().unwrap();

        let smart_permission = SmartPermission::from_bytes(&bytes).unwrap();
        assert_eq!(smart_permission, original);
    }

    #[test]
    // check that a smart permission can be converted to builder
    fn check_smart_permission_into_builder() {
        let builder = SmartPermissionBuilder::new();
        let smart_permission = builder
            .set_name("Tests".to_string())
            .set_org_id("org_id".to_string())
            .set_function(b"test_function".to_vec())
            .build()
            .unwrap();

        let builder = smart_permission.into_builder();

        assert_eq!(builder.name, Some("Tests".to_string()));
        assert_eq!(builder.org_id, Some("org_id".to_string()));
        assert_eq!(builder.function, b"test_function".to_vec());
    }

    #[test]
    // check that a smart permission list is built correctly
    fn check_smart_permission_list() {
        let builder = SmartPermissionBuilder::new();
        let smart_permission = builder
            .set_name("Tests".to_string())
            .set_org_id("org_id".to_string())
            .set_function(b"test_function".to_vec())
            .build()
            .unwrap();

        let builder = SmartPermissionListBuilder::new();
        let smart_permission_list = builder
            .set_smart_permissions(vec![smart_permission.clone()])
            .build()
            .unwrap();

        assert_eq!(
            smart_permission_list.get_smart_permissions(),
            [smart_permission]
        );
    }

    #[test]
    // check that a smart permission list can be converted to bytes and back
    fn check_smart_permission_list_bytes() {
        let builder = SmartPermissionBuilder::new();
        let smart_permission = builder
            .set_name("Tests".to_string())
            .set_org_id("org_id".to_string())
            .set_function(b"test_function".to_vec())
            .build()
            .unwrap();

        let builder = SmartPermissionListBuilder::new();
        let original = builder
            .set_smart_permissions(vec![smart_permission.clone()])
            .build()
            .unwrap();

        let bytes = original.clone().into_bytes().unwrap();

        let smart_permission_list = SmartPermissionList::from_bytes(&bytes).unwrap();
        assert_eq!(smart_permission_list, original);
    }

    #[test]
    // check that a KeyValueEntry is built correctly
    fn check_key_value_entry_builder() {
        let builder = KeyValueEntryBuilder::new();
        let key_value = builder
            .set_key("Key".to_string())
            .set_value("Value".to_string())
            .build()
            .unwrap();

        assert_eq!(key_value.key(), "Key");
        assert_eq!(key_value.value(), "Value");
    }

    #[test]
    // check that a KeyValueEntry can be converted to bytes and back
    fn check_key_value_entry_bytes() {
        let builder = KeyValueEntryBuilder::new();
        let original = builder
            .set_key("Key".to_string())
            .set_value("Value".to_string())
            .build()
            .unwrap();

        let bytes = original.clone().into_bytes().unwrap();
        let key_value = KeyValueEntry::from_bytes(&bytes).unwrap();
        assert_eq!(key_value, original);
    }

    #[test]
    // check that an Account is built correctly
    fn check_account_builder() {
        let builder = KeyValueEntryBuilder::new();
        let key_value = builder
            .set_key("Key".to_string())
            .set_value("Value".to_string())
            .build()
            .unwrap();

        let builder = AccountBuilder::new();
        let account = builder
            .set_org_id("organization".to_string())
            .set_public_key("public_key".to_string())
            .set_active(true)
            .set_roles(vec!["Role".to_string()])
            .set_metadata(vec![key_value.clone()])
            .build()
            .unwrap();

        assert_eq!(account.get_org_id(), "organization");
        assert_eq!(account.get_public_key(), "public_key");
        assert!(account.get_active());
        assert_eq!(account.get_roles(), ["Role".to_string()]);
        assert_eq!(account.get_metadata(), [key_value]);
    }

    #[test]
    // check that an Account can be converted to bytes and back
    fn check_account_bytes() {
        let builder = KeyValueEntryBuilder::new();
        let key_value = builder
            .set_key("Key".to_string())
            .set_value("Value".to_string())
            .build()
            .unwrap();

        let builder = AccountBuilder::new();
        let original = builder
            .set_org_id("organization".to_string())
            .set_public_key("public_key".to_string())
            .set_active(true)
            .set_roles(vec!["Role".to_string()])
            .set_metadata(vec![key_value.clone()])
            .build()
            .unwrap();

        let bytes = original.clone().into_bytes().unwrap();
        let account = Account::from_bytes(&bytes).unwrap();
        assert_eq!(account, original);
    }

    #[test]
    // check that an AccountList is built correctly
    fn check_account_list_builder() {
        let builder = KeyValueEntryBuilder::new();
        let key_value = builder
            .set_key("Key".to_string())
            .set_value("Value".to_string())
            .build()
            .unwrap();

        let builder = AccountBuilder::new();
        let account = builder
            .set_org_id("organization".to_string())
            .set_public_key("public_key".to_string())
            .set_active(true)
            .set_roles(vec!["Role".to_string()])
            .set_metadata(vec![key_value.clone()])
            .build()
            .unwrap();

        let builder = AccountListBuilder::new();
        let account_list = builder.set_accounts(vec![account.clone()]).build().unwrap();

        assert_eq!(account_list.get_accounts(), [account])
    }

    #[test]
    // check that an AccountList can be converted to bytes and back
    fn check_account_list_bytes() {
        let builder = KeyValueEntryBuilder::new();
        let key_value = builder
            .set_key("Key".to_string())
            .set_value("Value".to_string())
            .build()
            .unwrap();

        let builder = AccountBuilder::new();
        let account = builder
            .set_org_id("organization".to_string())
            .set_public_key("public_key".to_string())
            .set_active(true)
            .set_roles(vec!["Role".to_string()])
            .set_metadata(vec![key_value.clone()])
            .build()
            .unwrap();

        let builder = AccountBuilder::new();
        let original = builder.set_accounts(vec![account.clone()]).build().unwrap();

        let bytes = original.clone().into_bytes().unwrap();
        let account_list = AccountList::from_bytes(&bytes).unwrap();
        assert_eq!(account_list, original);
    }

    #[test]
    // check that an Organization is built correctly
    fn check_organization_builder() {
        let builder = KeyValueEntryBuilder::new();
        let key_value = builder
            .set_key("Key".to_string())
            .set_value("Value".to_string())
            .build()
            .unwrap();

        let builder = OrganizationBuilder::new();
        let organization = builder
            .set_org_id("organization".to_string())
            .set_name("name".to_string())
            .set_address("address".to_string())
            .set_metadata(vec![key_value.clone()])
            .build()
            .unwrap();

        assert_eq!(organization.get_org_id(), "organization");
        assert_eq!(organization.get_name(), "name");
        assert_eq!(organization.get_address(), "address");
        assert_eq!(organization.get_metadata(), [key_value]);
    }

    #[test]
    // check that an Organization can be converted to bytes and back
    fn check_organization_bytes() {
        let builder = KeyValueEntryBuilder::new();
        let key_value = builder
            .set_key("Key".to_string())
            .set_value("Value".to_string())
            .build()
            .unwrap();

        let builder = OrganizationBuilder::new();
        let original = builder
            .set_org_id("organization".to_string())
            .set_name("name".to_string())
            .set_address("address".to_string())
            .set_metadata(vec![key_value.clone()])
            .build()
            .unwrap();

        let bytes = original.clone().into_bytes().unwrap();
        let org = Organization::from_bytes(&bytes).unwrap();
        assert_eq!(org, original);
    }

    #[test]
    // check that an OrganizationList is built correctly
    fn check_organization_lists_builder() {
        let builder = KeyValueEntryBuilder::new();
        let key_value = builder
            .set_key("Key".to_string())
            .set_value("Value".to_string())
            .build()
            .unwrap();

        let builder = OrganizationBuilder::new();
        let organization = builder
            .set_org_id("organization".to_string())
            .set_name("name".to_string())
            .set_address("address".to_string())
            .set_metadata(vec![key_value.clone()])
            .build()
            .unwrap();

        let builder = OrganizationListBuilder::new();
        let organization_list = builder
            .set_organizations(vec![organization.clone()])
            .build()
            .unwrap();

        assert_eq!(organization_list.get_organizations(), [organization])
    }

    #[test]
    // check that an OrganizationList can be converted to bytes and back
    fn check_organization_list_bytes() {
        let builder = KeyValueEntryBuilder::new();
        let key_value = builder
            .set_key("Key".to_string())
            .set_value("Value".to_string())
            .build()
            .unwrap();

        let builder = OrganizationBuilder::new();
        let organization = builder
            .set_org_id("organization".to_string())
            .set_name("name".to_string())
            .set_address("address".to_string())
            .set_metadata(vec![key_value.clone()])
            .build()
            .unwrap();

        let builder = OrganizationListBuilder::new();
        let original = builder
            .set_organizations(vec![organization.clone()])
            .build()
            .unwrap();

        let bytes = original.clone().into_bytes().unwrap();
        let org_list = OrganizationList::from_bytes(&bytes).unwrap();
        assert_eq!(org_list, original);
    }
}
