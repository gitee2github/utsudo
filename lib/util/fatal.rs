/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */
 #![allow(
    non_camel_case_types,
    unused_variables,
    unused_assignments,
    unused_mut,
    unused_unsafe,
    non_upper_case_globals,
    dead_code
)]
use crate::__LC_MESSAGES;
use libc::FILE;

#[macro_export]
macro_rules! __LC_MESSAGES {
    () => {
        5
    };
}


extern "C" {
    fn free(__ptr: *mut libc::c_void);
    fn exit(__status: libc::c_int);
    fn strerror(__errnum: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn gai_strerror(__ecode: libc::c_int) -> *const libc::c_char;
    fn malloc(__size: libc::c_ulong) -> *mut libc::c_void;
    fn sudo_getprogname() -> *const libc::c_char;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
}

pub type ssize_t = libc::c_long;
pub type sudo_fatal_callback_t = Option<unsafe extern "C" fn()>;
static mut sudo_warn_conversation: sudo_conv_t = None;
static mut sudo_warn_setlocale: Option<unsafe extern "C" fn(bool, *mut libc::c_int) -> bool> = None;
static mut sudo_warn_setlocale_prev: Option<unsafe extern "C" fn(bool, *mut libc::c_int) -> bool> =
    None;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_fatal_callback {
    pub entries: STRUCT_unnamed,
    pub func: Option<unsafe extern "C" fn() -> ()>,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct STRUCT_unnamed {
    pub sle_next: *mut sudo_fatal_callback,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sudo_fatal_callback_list {
    pub slh_first: *mut sudo_fatal_callback,
}
