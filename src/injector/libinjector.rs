#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::os::raw::c_int;

pub const INJERR_SUCCESS: c_int = 0;
pub const INJERR_OTHER: c_int = -1;
pub const INJERR_NO_MEMORY: c_int = -2;
pub const INJERR_NO_PROCESS: c_int = -3;
pub const INJERR_NO_LIBRARY: c_int = -4;
pub const INJERR_NO_FUNCTION: c_int = -4;
pub const INJERR_ERROR_IN_TARGET: c_int = -5;
pub const INJERR_FILE_NOT_FOUND: c_int = -6;
pub const INJERR_INVALID_MEMORY_AREA: c_int = -7;
pub const INJERR_PERMISSION: c_int = -8;
pub const INJERR_UNSUPPORTED_TARGET: c_int = -9;
pub const INJERR_INVALID_ELF_FORMAT: c_int = -10;
pub const INJERR_WAIT_TRACEE: c_int = -11;

pub type __pid_t = c_int;
pub type pid_t = __pid_t;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct injector {
    _unused: [u8; 0],
}
pub type injector_t = injector;

#[link(name = "injector")]
extern "C" {
    pub fn injector_attach(injector: *mut *mut injector_t, pid: pid_t) -> c_int;
	
    pub fn injector_inject(
        injector: *mut injector_t,
        path: *const ::std::os::raw::c_char,
        handle: *mut *mut ::std::os::raw::c_void,
    ) -> c_int;
	
    pub fn injector_uninject(
        injector: *mut injector_t,
        handle: *mut ::std::os::raw::c_void,
    ) -> c_int;
	
    pub fn injector_detach(injector: *mut injector_t) -> c_int;
	
    pub fn injector_error() -> *const ::std::os::raw::c_char;
}
