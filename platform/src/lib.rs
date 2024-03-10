#![allow(non_snake_case)]

use core::ffi::c_void;
use roc_std::{RocList, RocResult, RocStr};
use std::io::{ErrorKind, Read, Write};
use std::time::{SystemTime, UNIX_EPOCH};
mod http_client;
mod roc_app;
use roc_fn::roc_fn;
mod glue_manual;

#[no_mangle]
pub unsafe extern "C" fn roc_alloc(size: usize, _alignment: u32) -> *mut c_void {
    return libc::malloc(size);
}

#[no_mangle]
pub unsafe extern "C" fn roc_realloc(
    c_ptr: *mut c_void,
    new_size: usize,
    _old_size: usize,
    _alignment: u32,
) -> *mut c_void {
    return libc::realloc(c_ptr, new_size);
}

#[no_mangle]
pub unsafe extern "C" fn roc_dealloc(c_ptr: *mut c_void, _alignment: u32) {
    return libc::free(c_ptr);
}

#[no_mangle]
pub unsafe extern "C" fn roc_panic(msg: *mut RocStr, tag_id: u32) {
    match tag_id {
        0 => {
            eprintln!("Roc standard library hit a panic: {}", &*msg);
        }
        1 => {
            eprintln!("Application hit a panic: {}", &*msg);
        }
        _ => unreachable!(),
    }
    std::process::exit(1);
}

#[no_mangle]
pub unsafe extern "C" fn roc_dbg(loc: *mut RocStr, msg: *mut RocStr, src: *mut RocStr) {
    eprintln!("[{}] {} = {}", &*loc, &*src, &*msg);
}

#[no_mangle]
pub unsafe extern "C" fn roc_memset(dst: *mut c_void, c: i32, n: usize) -> *mut c_void {
    libc::memset(dst, c, n)
}

#[cfg(unix)]
#[no_mangle]
pub unsafe extern "C" fn roc_getppid() -> libc::pid_t {
    libc::getppid()
}

#[cfg(unix)]
#[no_mangle]
pub unsafe extern "C" fn roc_mmap(
    addr: *mut libc::c_void,
    len: libc::size_t,
    prot: libc::c_int,
    flags: libc::c_int,
    fd: libc::c_int,
    offset: libc::off_t,
) -> *mut libc::c_void {
    libc::mmap(addr, len, prot, flags, fd, offset)
}

#[cfg(unix)]
#[no_mangle]
pub unsafe extern "C" fn roc_shm_open(
    name: *const libc::c_char,
    oflag: libc::c_int,
    mode: libc::mode_t,
) -> libc::c_int {
    libc::shm_open(name, oflag, mode as libc::c_uint)
}

#[roc_fn(name = "sendRequest")]
fn send_req(roc_request: &roc_app::InternalRequest) -> roc_app::InternalResponse {
    http_client::send_req(roc_request)
}

#[roc_fn(name = "stdoutLine")]
fn stdout_line(roc_str: &RocStr) {
    print!("{}\n", roc_str.as_str());
}

#[roc_fn(name = "envVar")]
fn env_var(roc_str: &RocStr) -> RocResult<RocStr, ()> {
    match std::env::var_os(roc_str.as_str()) {
        Some(os_str) => RocResult::ok(RocStr::from(os_str.to_string_lossy().to_string().as_str())),
        None => RocResult::err(()),
    }
}

#[roc_fn(name = "envList")]
fn env_dict() -> RocList<(RocStr, RocStr)> {
    use std::borrow::Borrow;

    let mut entries = Vec::new();

    for (key, val) in std::env::vars_os() {
        let key = RocStr::from(key.to_string_lossy().borrow());
        let value = RocStr::from(val.to_string_lossy().borrow());

        entries.push((key, value));
    }

    RocList::from_slice(entries.as_slice())
}

#[roc_fn(name = "posixTime")]
fn posix_time() -> roc_std::U128 {
    // TODO in future may be able to avoid this panic by using C APIs
    let since_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards");

    roc_std::U128::from(since_epoch.as_nanos())
}

#[roc_fn(name = "fileWriteUtf8")]
fn file_write_utf8(
    roc_path: &RocList<u8>,
    roc_str: &RocStr,
) -> roc_std::RocResult<(), glue_manual::WriteErr> {
    write_slice(roc_path, roc_str.as_str().as_bytes())
}

#[roc_fn(name = "fileWriteBytes")]
fn file_write_bytes(
    roc_path: &RocList<u8>,
    roc_bytes: &RocList<u8>,
) -> roc_std::RocResult<(), glue_manual::WriteErr> {
    write_slice(roc_path, roc_bytes.as_slice())
}

fn write_slice(
    roc_path: &RocList<u8>,
    bytes: &[u8],
) -> roc_std::RocResult<(), glue_manual::WriteErr> {
    match std::fs::File::create(path_from_roc_path(roc_path)) {
        Ok(mut file) => match file.write_all(bytes) {
            Ok(()) => RocResult::ok(()),
            Err(err) => RocResult::err(to_roc_write_error(err)),
        },
        Err(err) => RocResult::err(to_roc_write_error(err)),
    }
}

fn path_from_roc_path(bytes: &RocList<u8>) -> std::borrow::Cow<'_, std::path::Path> {
    use std::os::unix::ffi::OsStrExt;
    let os_str = std::ffi::OsStr::from_bytes(bytes.as_slice());
    std::borrow::Cow::Borrowed(std::path::Path::new(os_str))
}

fn to_roc_write_error(err: std::io::Error) -> glue_manual::WriteErr {
    println!("{}", err);
    match err.kind() {
        ErrorKind::NotFound => glue_manual::WriteErr::NotFound(),
        ErrorKind::AlreadyExists => glue_manual::WriteErr::AlreadyExists(),
        ErrorKind::Interrupted => glue_manual::WriteErr::Interrupted(),
        ErrorKind::OutOfMemory => glue_manual::WriteErr::OutOfMemory(),
        ErrorKind::PermissionDenied => glue_manual::WriteErr::PermissionDenied(),
        ErrorKind::TimedOut => glue_manual::WriteErr::TimedOut(),
        // TODO investigate support the following IO errors may need to update API
        ErrorKind::WriteZero => glue_manual::WriteErr::WriteZero(),
        // TODO investigate support the following IO errors
        // std::io::ErrorKind::FileTooLarge <- unstable language feature
        // std::io::ErrorKind::ExecutableFileBusy <- unstable language feature
        // std::io::ErrorKind::FilesystemQuotaExceeded <- unstable language feature
        // std::io::ErrorKind::InvalidFilename <- unstable language feature
        // std::io::ErrorKind::ResourceBusy <- unstable language feature
        // std::io::ErrorKind::ReadOnlyFilesystem <- unstable language feature
        // std::io::ErrorKind::TooManyLinks <- unstable language feature
        // std::io::ErrorKind::StaleNetworkFileHandle <- unstable language feature
        // std::io::ErrorKind::StorageFull <- unstable language feature
        _ => glue_manual::WriteErr::Unsupported(),
    }
}

#[roc_fn(name = "fileDelete")]
fn file_delete(roc_path: &RocList<u8>) -> roc_std::RocResult<(), glue_manual::ReadErr> {
    match std::fs::remove_file(path_from_roc_path(roc_path)) {
        Ok(()) => RocResult::ok(()),
        Err(err) => RocResult::err(to_roc_read_error(err)),
    }
}

#[roc_fn(name = "fileReadBytes")]
fn file_read_bytes(
    roc_path: &RocList<u8>,
) -> roc_std::RocResult<RocList<u8>, glue_manual::ReadErr> {
    let mut bytes = Vec::new();

    match std::fs::File::open(path_from_roc_path(roc_path)) {
        Ok(mut file) => match file.read_to_end(&mut bytes) {
            Ok(_bytes_read) => RocResult::ok(RocList::from(bytes.as_slice())),
            Err(err) => RocResult::err(to_roc_read_error(err)),
        },
        Err(err) => RocResult::err(to_roc_read_error(err)),
    }
}

fn to_roc_read_error(err: std::io::Error) -> glue_manual::ReadErr {
    match err.kind() {
        ErrorKind::Interrupted => glue_manual::ReadErr::Interrupted(),
        ErrorKind::NotFound => glue_manual::ReadErr::NotFound(),
        ErrorKind::OutOfMemory => glue_manual::ReadErr::OutOfMemory(),
        ErrorKind::PermissionDenied => glue_manual::ReadErr::PermissionDenied(),
        ErrorKind::TimedOut => glue_manual::ReadErr::TimedOut(),
        // TODO investigate support the following IO errors may need to update API
        // std::io::ErrorKind:: => glue_manual::ReadErr::TooManyHardlinks,
        // std::io::ErrorKind:: => glue_manual::ReadErr::TooManySymlinks,
        // std::io::ErrorKind:: => glue_manual::ReadErr::Unrecognized,
        // std::io::ErrorKind::StaleNetworkFileHandle <- unstable language feature
        // std::io::ErrorKind::InvalidFilename <- unstable language feature
        _ => glue_manual::ReadErr::Unsupported(),
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct RocFunction_86 {
    closure_data: Vec<u8>,
}

impl RocFunction_86 {
    pub fn force_thunk(mut self) -> String {
        extern "C" {
            fn roc__mainForHost_0_caller(
                arg0: *const (),
                closure_data: *mut u8,
                output: *mut RocStr,
            );
        }

        let mut output = core::mem::MaybeUninit::uninit();
        output.write(RocStr::default());

        unsafe {
            roc__mainForHost_0_caller(&(), self.closure_data.as_mut_ptr(), output.as_mut_ptr());

            output.assume_init().to_string()
        }
    }
}

pub fn mainForHost(request: Vec<u8>) -> RocFunction_86 {
    extern "C" {
        fn roc__mainForHost_1_exposed_generic(
            _: *mut u8,
            _: &mut core::mem::ManuallyDrop<RocList<u8>>,
        );
        fn roc__mainForHost_1_exposed_size() -> i64;
    }

    unsafe {
        let capacity = roc__mainForHost_1_exposed_size() as usize;

        let mut ret = RocFunction_86 {
            closure_data: Vec::with_capacity(capacity),
        };
        ret.closure_data.resize(capacity, 0);

        let bytes = RocList::from(request.as_slice());

        roc__mainForHost_1_exposed_generic(
            ret.closure_data.as_mut_ptr(),
            &mut core::mem::ManuallyDrop::new(bytes),
        );

        ret
    }
}
