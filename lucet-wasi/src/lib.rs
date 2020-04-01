#![deny(bare_trait_objects)]

// disabled for now
//pub mod c_api;

use lucet_runtime::vmctx::Vmctx;
use lucet_runtime::{lucet_hostcall, lucet_hostcall_terminate};
use wiggle::GuestPtr;

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

impl<'a> wasi_snapshot_preview1::WasiSnapshotPreview1 for LucetWasiCtx<'a> {
    fn args_get<'b>(
        &self,
        argv: &GuestPtr<'b, GuestPtr<'b, u8>>,
        argv_buf: &GuestPtr<'b, u8>,
    ) -> Result<(), types::Errno> {
        unimplemented!()
    }

    fn args_sizes_get(&self) -> Result<(types::Size, types::Size), types::Errno> {
        unimplemented!()
    }

    fn environ_get<'b>(
        &self,
        environ: &GuestPtr<'b, GuestPtr<'b, u8>>,
        environ_buf: &GuestPtr<'b, u8>,
    ) -> Result<(), types::Errno> {
        unimplemented!()
    }

    fn environ_sizes_get(&self) -> Result<(types::Size, types::Size), types::Errno> {
        unimplemented!()
    }

    fn clock_res_get(&self, id: types::Clockid) -> Result<types::Timestamp, types::Errno> {
        unimplemented!()
    }

    fn clock_time_get(
        &self,
        id: types::Clockid,
        _precision: types::Timestamp,
    ) -> Result<types::Timestamp, types::Errno> {
        unimplemented!()
    }

    fn fd_advise(
        &self,
        fd: types::Fd,
        offset: types::Filesize,
        len: types::Filesize,
        advice: types::Advice,
    ) -> Result<(), types::Errno> {
        unimplemented!()
    }

    fn fd_allocate(
        &self,
        fd: types::Fd,
        offset: types::Filesize,
        len: types::Filesize,
    ) -> Result<(), types::Errno> {
        unimplemented!()
    }

    fn fd_close(&self, fd: types::Fd) -> Result<(), types::Errno> {
        unimplemented!()
    }

    fn fd_datasync(&self, fd: types::Fd) -> Result<(), types::Errno> {
        unimplemented!()
    }

    fn fd_fdstat_get(&self, fd: types::Fd) -> Result<types::Fdstat, types::Errno> {
        unimplemented!()
    }

    fn fd_fdstat_set_flags(
        &self,
        fd: types::Fd,
        flags: types::Fdflags,
    ) -> Result<(), types::Errno> {
        unimplemented!()
    }

    fn fd_fdstat_set_rights(
        &self,
        fd: types::Fd,
        fs_rights_base: types::Rights,
        fs_rights_inheriting: types::Rights,
    ) -> Result<(), types::Errno> {
        unimplemented!()
    }

    fn fd_filestat_get(&self, fd: types::Fd) -> Result<types::Filestat, types::Errno> {
        unimplemented!()
    }

    fn fd_filestat_set_size(
        &self,
        fd: types::Fd,
        size: types::Filesize,
    ) -> Result<(), types::Errno> {
        unimplemented!()
    }

    fn fd_filestat_set_times(
        &self,
        fd: types::Fd,
        atim: types::Timestamp,
        mtim: types::Timestamp,
        fst_flags: types::Fstflags,
    ) -> Result<(), types::Errno> {
        unimplemented!()
    }

    fn fd_pread(
        &self,
        fd: types::Fd,
        iovs: &types::IovecArray<'_>,
        offset: types::Filesize,
    ) -> Result<types::Size, types::Errno> {
        unimplemented!()
    }

    fn fd_prestat_get(&self, fd: types::Fd) -> Result<types::Prestat, types::Errno> {
        unimplemented!()
    }

    fn fd_prestat_dir_name(
        &self,
        fd: types::Fd,
        path: &GuestPtr<u8>,
        path_len: types::Size,
    ) -> Result<(), types::Errno> {
        unimplemented!()
    }

    fn fd_pwrite(
        &self,
        fd: types::Fd,
        ciovs: &types::CiovecArray<'_>,
        offset: types::Filesize,
    ) -> Result<types::Size, types::Errno> {
        unimplemented!();
    }

    fn fd_read(
        &self,
        fd: types::Fd,
        iovs: &types::IovecArray<'_>,
    ) -> Result<types::Size, types::Errno> {
        unimplemented!();
    }

    fn fd_readdir(
        &self,
        fd: types::Fd,
        buf: &GuestPtr<u8>,
        buf_len: types::Size,
        cookie: types::Dircookie,
    ) -> Result<types::Size, types::Errno> {
        unimplemented!();
    }

    fn fd_renumber(&self, from: types::Fd, to: types::Fd) -> Result<(), types::Errno> {
        unimplemented!();
    }

    fn fd_seek(
        &self,
        fd: types::Fd,
        offset: types::Filedelta,
        whence: types::Whence,
    ) -> Result<types::Filesize, types::Errno> {
        unimplemented!()
    }

    fn fd_sync(&self, fd: types::Fd) -> Result<(), types::Errno> {
        unimplemented!()
    }

    fn fd_tell(&self, fd: types::Fd) -> Result<types::Filesize, types::Errno> {
        unimplemented!()
    }

    fn fd_write(
        &self,
        fd: types::Fd,
        ciovs: &types::CiovecArray<'_>,
    ) -> Result<types::Size, types::Errno> {
        unimplemented!()
    }

    fn path_create_directory(
        &self,
        dirfd: types::Fd,
        path: &GuestPtr<'_, str>,
    ) -> Result<(), types::Errno> {
        unimplemented!();
    }

    fn path_filestat_get(
        &self,
        dirfd: types::Fd,
        flags: types::Lookupflags,
        path: &GuestPtr<'_, str>,
    ) -> Result<types::Filestat, types::Errno> {
        unimplemented!();
    }

    fn path_filestat_set_times(
        &self,
        dirfd: types::Fd,
        flags: types::Lookupflags,
        path: &GuestPtr<'_, str>,
        atim: types::Timestamp,
        mtim: types::Timestamp,
        fst_flags: types::Fstflags,
    ) -> Result<(), types::Errno> {
        unimplemented!()
    }

    fn path_link(
        &self,
        old_fd: types::Fd,
        old_flags: types::Lookupflags,
        old_path: &GuestPtr<'_, str>,
        new_fd: types::Fd,
        new_path: &GuestPtr<'_, str>,
    ) -> Result<(), types::Errno> {
        unimplemented!();
    }

    fn path_open(
        &self,
        dirfd: types::Fd,
        dirflags: types::Lookupflags,
        path: &GuestPtr<'_, str>,
        oflags: types::Oflags,
        fs_rights_base: types::Rights,
        fs_rights_inheriting: types::Rights,
        fdflags: types::Fdflags,
    ) -> Result<types::Fd, types::Errno> {
        unimplemented!();
    }

    fn path_readlink(
        &self,
        dirfd: types::Fd,
        path: &GuestPtr<'_, str>,
        buf: &GuestPtr<u8>,
        buf_len: types::Size,
    ) -> Result<types::Size, types::Errno> {
        unimplemented!();
    }

    fn path_remove_directory(
        &self,
        dirfd: types::Fd,
        path: &GuestPtr<'_, str>,
    ) -> Result<(), types::Errno> {
        unimplemented!();
    }

    fn path_rename(
        &self,
        old_fd: types::Fd,
        old_path: &GuestPtr<'_, str>,
        new_fd: types::Fd,
        new_path: &GuestPtr<'_, str>,
    ) -> Result<(), types::Errno> {
        unimplemented!();
    }

    fn path_symlink(
        &self,
        old_path: &GuestPtr<'_, str>,
        dirfd: types::Fd,
        new_path: &GuestPtr<'_, str>,
    ) -> Result<(), types::Errno> {
        unimplemented!()
    }

    fn path_unlink_file(
        &self,
        dirfd: types::Fd,
        path: &GuestPtr<'_, str>,
    ) -> Result<(), types::Errno> {
        unimplemented!()
    }

    fn poll_oneoff(
        &self,
        in_: &GuestPtr<types::Subscription>,
        out: &GuestPtr<types::Event>,
        nsubscriptions: types::Size,
    ) -> Result<types::Size, types::Errno> {
        unimplemented!()
    }

    fn proc_exit(&self, rval: types::Exitcode) -> Result<(), ()> {
        unimplemented!()
    }

    fn proc_raise(&self, _sig: types::Signal) -> Result<(), types::Errno> {
        unimplemented!()
    }

    fn sched_yield(&self) -> Result<(), types::Errno> {
        unimplemented!()
    }

    fn random_get(&self, buf: &GuestPtr<u8>, buf_len: types::Size) -> Result<(), types::Errno> {
        unimplemented!();
    }

    fn sock_recv(
        &self,
        _fd: types::Fd,
        _ri_data: &types::IovecArray<'_>,
        _ri_flags: types::Riflags,
    ) -> Result<(types::Size, types::Roflags), types::Errno> {
        unimplemented!()
    }

    fn sock_send(
        &self,
        _fd: types::Fd,
        _si_data: &types::CiovecArray<'_>,
        _si_flags: types::Siflags,
    ) -> Result<types::Size, types::Errno> {
        unimplemented!()
    }

    fn sock_shutdown(&self, _fd: types::Fd, _how: types::Sdflags) -> Result<(), types::Errno> {
        unimplemented!()
    }
}
