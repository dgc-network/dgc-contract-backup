// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

#[macro_use]
extern crate clap;

use log::Level;

use sawtooth_sabre::handler::SabreTransactionHandler;
use sawtooth_sdk::processor::TransactionProcessor;

fn main() {
    let matches = clap_app!(wasm_store_tp =>
        (version: crate_version!())
        (about: "Implements the Sawtooth Sabre transaction family")
        (@arg connect: -C --connect +takes_value
         "connection endpoint for validator")
        (@arg verbose: -v --verbose +multiple
         "increase output verbosity"))
    .get_matches();

    let logger = match matches.occurrences_of("verbose") {
        0 => simple_logger::init_with_level(Level::Warn),
        1 => simple_logger::init_with_level(Level::Info),
        2 => simple_logger::init_with_level(Level::Debug),
        3 | _ => simple_logger::init_with_level(Level::Trace),
    };

    logger.expect("Failed to create logger");

    let connect = matches
        .value_of("connect")
        .unwrap_or("tcp://localhost:4004");

    let handler = SabreTransactionHandler::new();
    let mut processor = TransactionProcessor::new(connect);

    processor.add_handler(&handler);
    processor.start();
}
