#![feature(assert_matches)]
#![feature(box_patterns)]
#![feature(error_generic_member_access)]
#![feature(hash_extract_if)]
#![feature(option_get_or_insert_default)]
#![feature(once_cell_try)]
#![feature(panic_info_message)]
#![feature(try_blocks)]
#![feature(impl_trait_in_assoc_type)]
#![deny(clippy::all)]
// Clippy's needless mut lint is buggy: https://github.com/rust-lang/rust-clippy/issues/11299
#![allow(clippy::needless_pass_by_ref_mut)]
#![allow(dead_code)]

mod child;
mod cli;
mod commands;
mod config;
mod daemon;
mod diagnostics;
mod engine;

mod framework;
mod gitignore;
mod global_deps_package_change_mapper;
pub(crate) mod globwatcher;
mod hash;
mod opts;
mod package_changes_watcher;
mod panic_handler;
mod process;
mod rewrite_json;
mod run;
mod shim;
mod signal;
mod task_graph;
mod task_hash;
mod tracing;
mod turbo_json;
mod unescape;

pub use crate::{
    child::spawn_child,
    cli::Args,
    daemon::{DaemonClient, DaemonConnector, Paths as DaemonPaths},
    panic_handler::panic_handler,
    run::package_discovery::DaemonPackageDiscovery,
};

pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub fn main() -> Result<i32, shim::Error> {
    shim::run()
}

#[cfg(all(feature = "native-tls", feature = "rustls-tls"))]
compile_error!("You can't enable both the `native-tls` and `rustls-tls` feature.");

#[cfg(all(not(feature = "native-tls"), not(feature = "rustls-tls")))]
compile_error!("You have to enable one of the TLS features: `native-tls` or `rustls-tls`");
