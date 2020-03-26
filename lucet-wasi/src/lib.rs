#![deny(bare_trait_objects)]

pub mod c_api;
mod wasi;

// Wasi-common re-exports:
pub use wasi_common::{wasi::types::Exitcode, WasiCtx, WasiCtxBuilder};

// Wasi executables export the following symbol for the entry point:
pub const START_SYMBOL: &str = "_start";

pub fn export_wasi_funcs() {
    wasi::hostcalls::init()
}

pub fn bindings() -> lucet_module::bindings::Bindings {
    lucet_wiggle_generate::bindings(&wasi_common::wasi::metadata::document())
}
