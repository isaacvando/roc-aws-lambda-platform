#![allow(non_snake_case)]

use core::ffi::c_void;
use roc_std::RocList;
use roc_std::RocStr;
mod http_client;
mod roc_app;
use roc_fn::roc_fn;

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
