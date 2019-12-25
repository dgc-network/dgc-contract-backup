// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

use std;
use std::borrow::Borrow;
use std::error::Error as StdError;

use hyper;
use protobuf;
use dgc_contract_sdk::protocol::payload::{
    CreateContractActionBuildError, CreateContractRegistryActionBuildError,
    CreateNamespaceRegistryActionBuildError, CreateNamespaceRegistryPermissionActionBuildError,
    CreateSmartPermissionActionBuildError, DeleteContractRegistryActionBuildError,
    DeleteNamespaceRegistryActionBuildError, DeleteNamespaceRegistryPermissionActionBuildError,
    DeleteSmartPermissionActionBuildError, ExecuteContractActionBuildError, SmartPayloadBuildError,
    UpdateContractRegistryOwnersActionBuildError, UpdateNamespaceRegistryOwnersActionBuildError,
    UpdateSmartPermissionActionBuildError,
};
use dgc_contract_sdk::protos::ProtoConversionError;
use sawtooth_sdk::signing;

#[derive(Debug)]
pub enum CliError {
    /// The user has provided invalid inputs; the string by this error
    /// is appropriate for display to the user without additional context
    UserError(String),
    IoError(std::io::Error),
    SigningError(signing::Error),
    ProtobufError(protobuf::ProtobufError),
    HyperError(hyper::Error),
    ProtocolBuildError(Box<dyn StdError>),
    ProtoConversionError(ProtoConversionError),
}

impl StdError for CliError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            CliError::UserError(_) => None,
            CliError::IoError(err) => Some(err),
            CliError::SigningError(err) => Some(err),
            CliError::ProtobufError(err) => Some(err),
            CliError::HyperError(err) => Some(err),
            CliError::ProtocolBuildError(ref err) => Some(err.borrow()),
            CliError::ProtoConversionError(err) => Some(err),
        }
    }
}

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            CliError::UserError(ref s) => write!(f, "Error: {}", s),
            CliError::IoError(ref err) => write!(f, "IoError: {}", err),
            CliError::SigningError(ref err) => write!(f, "SigningError: {}", err.description()),
            CliError::ProtobufError(ref err) => write!(f, "ProtobufError: {}", err.description()),
            CliError::HyperError(ref err) => write!(f, "HyperError: {}", err.description()),
            CliError::ProtocolBuildError(ref err) => write!(f, "Protocol Error: {}", err),
            CliError::ProtoConversionError(ref err) => write!(f, "Proto Conversion Error: {}", err),
        }
    }
}

impl From<std::io::Error> for CliError {
    fn from(e: std::io::Error) -> Self {
        CliError::IoError(e)
    }
}

impl From<protobuf::ProtobufError> for CliError {
    fn from(e: protobuf::ProtobufError) -> Self {
        CliError::ProtobufError(e)
    }
}

impl From<signing::Error> for CliError {
    fn from(e: signing::Error) -> Self {
        CliError::SigningError(e)
    }
}

impl From<hyper::Error> for CliError {
    fn from(e: hyper::Error) -> Self {
        CliError::HyperError(e)
    }
}

impl From<ProtoConversionError> for CliError {
    fn from(e: ProtoConversionError) -> Self {
        CliError::ProtoConversionError(e)
    }
}

// used to convert BuildErrors from the smart sdk protocols into a CliError.
macro_rules! impl_builder_errors {
    ($($x:ty),*) => {
        $(
            impl From<$x> for CliError {
                fn from(e: $x) -> Self {
                    CliError::ProtocolBuildError(Box::new(e))
                }
            }
        )*
    };
}

impl_builder_errors!(
    CreateContractRegistryActionBuildError,
    UpdateContractRegistryOwnersActionBuildError,
    DeleteContractRegistryActionBuildError,
    SmartPayloadBuildError,
    CreateNamespaceRegistryActionBuildError,
    UpdateNamespaceRegistryOwnersActionBuildError,
    CreateNamespaceRegistryPermissionActionBuildError,
    DeleteNamespaceRegistryActionBuildError,
    ExecuteContractActionBuildError,
    DeleteNamespaceRegistryPermissionActionBuildError,
    CreateSmartPermissionActionBuildError,
    UpdateSmartPermissionActionBuildError,
    DeleteSmartPermissionActionBuildError,
    CreateContractActionBuildError
);
