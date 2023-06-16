#![allow(
    mutable_transmutes,
    non_camel_case_types,
    unused_assignments,
    unused_mut
)]

use crate::sudo_debug::*;
use crate::sudo_debug_macro::*;

extern "C" {
    fn sudo_debug_printf2_v1(
        func: *const libc::c_char,
        file: *const libc::c_char,
        lineno: libc::c_int,
        level: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
}