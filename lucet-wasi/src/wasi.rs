use lucet_runtime::vmctx::Vmctx;
use lucet_runtime::{lucet_hostcall, lucet_hostcall_terminate};

pub struct LucetWasiCtx<'a> {
    vmctx: &'a Vmctx,
}

lucet_wasi_generate::bindings!({
    ctx: LucetWasiCtx,
    constructor: { LucetWasiCtx { vmctx } }
});
