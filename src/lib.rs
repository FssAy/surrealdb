#![deny(clippy::mem_forget)]
#![forbid(unsafe_code)]

#[macro_use]
extern crate tracing;

#[macro_use]
mod mac;

pub mod cli;
mod cnf;
mod dbs;
mod env;
mod err;
#[cfg(surrealdb_unstable)]
mod gql;
mod mem;
pub mod net;
mod rpc;
mod telemetry;

pub use surrealdb_core;
