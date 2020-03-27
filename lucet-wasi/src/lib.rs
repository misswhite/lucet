#![deny(bare_trait_objects)]

// disabled for now
//pub mod c_api;

use lucet_runtime::vmctx::Vmctx;
use lucet_runtime::{lucet_hostcall, lucet_hostcall_terminate};

// Wasi-common re-exports:
pub use wasi_common::{WasiCtx, WasiCtxBuilder};

// Wasi executables export the following symbol for the entry point:
pub const START_SYMBOL: &str = "_start";

pub fn export_wasi_funcs() {
    hostcalls::init()
}

pub fn bindings() -> lucet_module::bindings::Bindings {
    lucet_wiggle_generate::bindings(&wasi_common::wasi::metadata::document())
}

pub struct LucetWasiCtx<'a> {
    vmctx: &'a Vmctx,
}

pub mod types {
    pub use wasi_common::wasi::types::*;
}

lucet_wasi_generate::bindings!({
    ctx: LucetWasiCtx,
    constructor: { LucetWasiCtx { vmctx } }
});

impl<'a> wasi_snapshot_preview1::WasiSnapshotPreview1 for LucetWasiCtx<'a> {}
