// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

#[macro_use]
extern crate log;

mod addressing;
pub mod handler;
mod payload;
mod state;
mod wasm_executor;
//mod protocol;
//mod protos;

pub use sabre_sdk::protocol::{ADMINISTRATORS_SETTING_ADDRESS, ADMINISTRATORS_SETTING_KEY};
//pub use crate::protocol::{ADMINISTRATORS_SETTING_ADDRESS, ADMINISTRATORS_SETTING_KEY};
