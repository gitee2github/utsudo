/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut,
)]

use crate::sudo_debug::sudo_debug_enter_v1;
use crate::sudo_debug::sudo_debug_exit_id_t_v1;

    use crate::sudo_debug_macro::SUDO_DEBUG_UTIL;

use crate::INT_MAX;

// #define INT_MIN (-__INT_MAX__ - 1)
#[macro_export]
macro_rules! INT_MIN {
    () => {
        (-(INT_MAX!()) - 1)
    };
    }


// #define UINT_MAX (__INT_MAX__ * 2U + 1U)
#[macro_export]
macro_rules! UINT_MAX {
    () => {
        ((INT_MAX!()) * (2 as libc::c_uint) + 1 as libc::c_uint)
    };
}
// #define	EINVAL		22	/* Invalid argument */
#[macro_export]
macro_rules! EINVAL {
    () => {
        22
    };
}
type id_t = u32;

extern "C" {
    fn sudo_strtonumx(
        str: *const libc::c_char,
        minval: libc::c_longlong,
        maxval: libc::c_longlong,
        endp: *mut *mut libc::c_char,
        errstrp: *mut *const libc::c_char,
    ) -> libc::c_longlong;
    fn __errno_location() -> *mut libc::c_int;
}

/*
 * Make sure that the ID ends with a valid separator char.
 */
unsafe extern "C" fn valid_separator(
    mut p: *const libc::c_char,
    mut ep: *const libc::c_char,
    mut sep: *const libc::c_char,
) -> bool {
    let mut valid: bool = false;
    if ep != p {
        /* check for valid separator (including '\0') */
        if sep.is_null() {
            sep = b"\0" as *const u8 as *const libc::c_char;
        }
        loop {
            if *ep == *sep {
                valid = true;
            }
            if !(*sep as libc::c_int != '\u{0}' as i32) {
                break;
            }
            sep = sep.offset(1);
        }
    } // !eq != p
    return valid;
}













