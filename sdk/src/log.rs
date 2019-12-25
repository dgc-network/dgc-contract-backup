// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

#[macro_export]
macro_rules! log {
    ($lvl:path, $($arg:tt)+) => ({
        let lvl = $lvl;
        if ::dgc_contract_sdk::log_enabled(lvl) {
            let x = format_args!($($arg)*).to_string();
            ::dgc_contract_sdk::log_message(lvl, x);
        }
    })
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) =>(log!(::dgc_contract_sdk::LogLevel::Trace, $($arg)*))
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) =>(log!(::dgc_contract_sdk::LogLevel::Debug, $($arg)*))
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) =>(log!(::dgc_contract_sdk::LogLevel::Info, $($arg)*))
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) =>(log!(::dgc_contract_sdk::LogLevel::Warn, $($arg)*))
}
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) =>(log!(::dgc_contract_sdk::LogLevel::Error, $($arg)*))
}
