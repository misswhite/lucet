// re-export types that should only be used for testing
pub use lucet_runtime_internals::module::{HeapSpec, MockModuleBuilder};
#[allow(deprecated)]
pub use lucet_runtime_internals::vmctx::vmctx_from_mock_instance;

use lazy_static::lazy_static;
use lucet_runtime_internals::error::Error;
use lucet_runtime_internals::module::DlModule;
use std::env;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

lazy_static! {
    static ref EXCLUSIVE_TEST: RwLock<()> = RwLock::default();
}

/// Run a test non-exclusively with other `test_nonex` tests.
///
/// This function _must_ wrap any uses of `DlModule` or `Instance::run()`.
#[allow(dead_code)]
pub fn test_nonex<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    let lock = EXCLUSIVE_TEST.read().unwrap();
    let r = f();
    drop(lock);
    r
}

/// Run a test exclusively, so that no other `test_nonex` or `test_ex` tests will run concurrently.
///
/// This function _must_ wrap any tests that use `fork` or that set a custom `sigaction`.
#[allow(dead_code)]
pub fn test_ex<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    let lock = EXCLUSIVE_TEST.write().unwrap();
    let r = f();
    drop(lock);
    r
}
