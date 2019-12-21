// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

#[macro_export]
macro_rules! log {
    ($lvl:path, $($arg:tt)+) => ({
        let lvl = $lvl;
        if ::smart_sdk::log_enabled(lvl) {
            let x = format_args!($($arg)*).to_string();
            ::smart_sdk::log_message(lvl, x);
        }
    })
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) =>(log!(::smart_sdk::LogLevel::Trace, $($arg)*))
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) =>(log!(::smart_sdk::LogLevel::Debug, $($arg)*))
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) =>(log!(::smart_sdk::LogLevel::Info, $($arg)*))
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) =>(log!(::smart_sdk::LogLevel::Warn, $($arg)*))
}
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) =>(log!(::smart_sdk::LogLevel::Error, $($arg)*))
}
