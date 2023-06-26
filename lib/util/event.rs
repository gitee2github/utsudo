#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

pub type __uint32_t = libc::c_uint;
pub type __uid_t = libc::c_uint;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}

static mut default_base: *mut sudo_event_base = 0 as *const sudo_event_base as *mut sudo_event_base;
static mut signal_base: *mut sudo_event_base = 0 as *const sudo_event_base as *mut sudo_event_base;

#[no_mangle]
pub unsafe extern "C" fn sudo_ev_activate(mut base: *mut sudo_event_base, mut ev: *mut sudo_event) {
    (*ev).active_entries.tqe_next = 0 as *mut sudo_event;
    (*ev).active_entries.tqe_prev = (*base).active.tqh_last;
    *(*base).active.tqh_last = ev;
    (*base).active.tqh_last = &mut (*ev).active_entries.tqe_next;
    (*ev).flags = ((*ev).flags as libc::c_int | 0x2 as libc::c_int) as libc::c_short;
}

#[inline]
unsafe extern "C" fn sudo_ev_deactivate(mut base: *mut sudo_event_base, mut ev: *mut sudo_event) {
    (*ev).flags = ((*ev).flags as libc::c_int & !(0x2 as libc::c_int)) as libc::c_short;
    if !((*ev).active_entries.tqe_next).is_null() {
        (*(*ev).active_entries.tqe_next).active_entries.tqe_prev = (*ev).active_entries.tqe_prev;
    } else {
        (*base).active.tqh_last = (*ev).active_entries.tqe_prev;
    }
    *(*ev).active_entries.tqe_prev = (*ev).active_entries.tqe_next;
}

unsafe extern "C" fn sudo_ev_deactivate_all(mut base: *mut sudo_event_base) {
    let mut ev: *mut sudo_event = 0 as *mut sudo_event;
    let sudo_debug_subsys: libc::c_int = (4 as libc::c_int) << 6 as libc::c_int;
    sudo_debug_enter_v1(
        (*::core::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(b"sudo_ev_deactivate_all\0"))
            .as_ptr(),
        b"event.c\0" as *const u8 as *const libc::c_char,
        89 as libc::c_int,
        sudo_debug_subsys,
    );

        loop {
        ev = (*base).active.tqh_first;
        if ev.is_null() {
            break;
        }
        sudo_ev_deactivate(base, ev);
    }
    sudo_debug_exit_v1(
        (*::core::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(b"sudo_ev_deactivate_all\0"))
            .as_ptr(),
        b"event.c\0" as *const u8 as *const libc::c_char,
        94 as libc::c_int,
        sudo_debug_subsys,
    );
}

unsafe extern "C" fn sudo_ev_activate_sigevents(mut base: *mut sudo_event_base) {
    let mut ev: *mut sudo_event = 0 as *mut sudo_event;
    let mut set: sigset_t = sigset_t { __val: [0; 16] };
    let mut oset: sigset_t = sigset_t { __val: [0; 16] };
    let mut i: libc::c_int = 0;
    let sudo_debug_subsys: libc::c_int = (4 as libc::c_int) << 6 as libc::c_int;
    sudo_debug_enter_v1(
        (*::core::mem::transmute::<&[u8; 27], &[libc::c_char; 27]>(
            b"sudo_ev_activate_sigevents\0",
        ))
        .as_ptr(),
        b"event.c\0" as *const u8 as *const libc::c_char,
        107 as libc::c_int,
        sudo_debug_subsys,
    );

        sigfillset(&mut set);
    sigprocmask(0 as libc::c_int, &mut set, &mut oset);
    (*base).signal_caught = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int + 1 as libc::c_int {
        if !((*base).signal_pending[i as usize] == 0) {
            (*base).signal_pending[i as usize] = 0 as libc::c_int;
            ev = (*base).signals[i as usize].tqh_first;
            while !ev.is_null() {
                if (*ev).events as libc::c_int & 0x20 as libc::c_int != 0 {
                    let mut sc: *mut sudo_ev_siginfo_container =
                        (*ev).closure as *mut sudo_ev_siginfo_container;
                    if (*(*base).siginfo[i as usize]).si_signo == 0 as libc::c_int {
                        (*sc).siginfo = 0 as *mut siginfo_t;
                    } else {
                        (*sc).siginfo = ((*sc).si_buf).as_mut_ptr() as *mut siginfo_t;
                        memcpy(
                            (*sc).siginfo as *mut libc::c_void,
                            (*base).siginfo[i as usize] as *const libc::c_void,
                            ::core::mem::size_of::<siginfo_t>() as libc::c_ulong,
                        );
                    }
                }
                (*ev).revents = ((*ev).events as libc::c_int
                    & (0x10 as libc::c_int | 0x20 as libc::c_int))
                    as libc::c_short;
                (*ev).active_entries.tqe_next = 0 as *mut sudo_event;
                (*ev).active_entries.tqe_prev = (*base).active.tqh_last;
                *(*base).active.tqh_last = ev;
                (*base).active.tqh_last = &mut (*ev).active_entries.tqe_next;
                (*ev).flags = ((*ev).flags as libc::c_int | 0x2 as libc::c_int) as libc::c_short;
                ev = (*ev).entries.tqe_next;
            }
        }
        i += 1;
    }
    sigprocmask(2 as libc::c_int, &mut oset, 0 as *mut sigset_t);
    sudo_debug_exit_v1(
        (*::core::mem::transmute::<&[u8; 27], &[libc::c_char; 27]>(
            b"sudo_ev_activate_sigevents\0",
        ))
        .as_ptr(),
        b"event.c\0" as *const u8 as *const libc::c_char,
        139 as libc::c_int,
        sudo_debug_subsys,
    );
}

unsafe extern "C" fn signal_pipe_cb(
    mut fd: libc::c_int,
    mut _what: libc::c_int,
    mut v: *mut libc::c_void,
){
    let mut base: *mut sudo_event_base = v as *mut sudo_event_base;
    let mut ch: libc::c_uchar = 0;
    let mut nread: ssize_t = 0;
    let sudo_debug_subsys: libc::c_int = (4 as libc::c_int) << 6 as libc::c_int;
    sudo_debug_enter_v1(
        (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"signal_pipe_cb\0")).as_ptr(),
        b"event.c\0" as *const u8 as *const libc::c_char,
        151 as libc::c_int,
        sudo_debug_subsys,
    );

        loop {
        nread = read(
            fd,
            &mut ch as *mut libc::c_uchar as *mut libc::c_void,
            1 as libc::c_int as size_t,
        );
        if !(nread > 0 as libc::c_int as libc::c_long) {
            break;
        }
        sudo_debug_printf2_v1(
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"signal_pipe_cb\0"))
                .as_ptr(),
            b"event.c\0" as *const u8 as *const libc::c_char,
            160 as libc::c_int,
            6 as libc::c_int | sudo_debug_subsys,
            b"%s: received signal %d\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"signal_pipe_cb\0"))
                .as_ptr(),
            ch as libc::c_int,
        );
    }

        if nread == -(1 as libc::c_int) as libc::c_long && *__errno_location() != 11 as libc::c_int {
        sudo_debug_printf2_v1(
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"signal_pipe_cb\0"))
                .as_ptr(),
            b"event.c\0" as *const u8 as *const libc::c_char,
            164 as libc::c_int,
            2 as libc::c_int
                | (1 as libc::c_int) << 5 as libc::c_int
                | (1 as libc::c_int) << 4 as libc::c_int
                | sudo_debug_subsys,
            b"%s: error reading from signal pipe fd %d\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"signal_pipe_cb\0"))
                .as_ptr(),
            fd,
        );
    }
    sudo_ev_activate_sigevents(base);
    sudo_debug_exit_v1(
        (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"signal_pipe_cb\0")).as_ptr(),
        b"event.c\0" as *const u8 as *const libc::c_char,
        170 as libc::c_int,
        sudo_debug_subsys,
    );

}

unsafe extern "C" fn sudo_ev_base_init(mut base: *mut sudo_event_base) -> libc::c_int{
    let mut i: libc::c_int = 0;
    let sudo_debug_subsys: libc::c_int = (4 as libc::c_int) << 6 as libc::c_int;
    sudo_debug_enter_v1(
        (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"sudo_ev_base_init\0"))
            .as_ptr(),
        b"event.c\0" as *const u8 as *const libc::c_char,
        177 as libc::c_int,
        sudo_debug_subsys,
    );
    (*base).events.tqh_first = 0 as *mut sudo_event;
    (*base).events.tqh_last = &mut (*base).events.tqh_first;
    (*base).timeouts.tqh_first = 0 as *mut sudo_event;
    (*base).timeouts.tqh_last = &mut (*base).timeouts.tqh_first;
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int + 1 as libc::c_int {
        (*base).signals[i as usize].tqh_first = 0 as *mut sudo_event;
        (*base).signals[i as usize].tqh_last =
            &mut (*((*base).signals).as_mut_ptr().offset(i as isize)).tqh_first;
        i += 1;
    }
        if sudo_ev_base_alloc_impl(base) != 0 as libc::c_int {
        sudo_debug_printf2_v1(
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"sudo_ev_base_init\0"))
                .as_ptr(),
            b"event.c\0" as *const u8 as *const libc::c_char,
            185 as libc::c_int,
            2 as libc::c_int | sudo_debug_subsys,
            b"%s: unable to allocate impl base\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"sudo_ev_base_init\0"))
                .as_ptr(),
        );
    } else if pipe2(
        ((*base).signal_pipe).as_mut_ptr(),
        0o4000 as libc::c_int | 0o2000000 as libc::c_int,
    ) != 0 as libc::c_int
    {
        sudo_debug_printf2_v1(
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"sudo_ev_base_init\0"))
                .as_ptr(),
            b"event.c\0" as *const u8 as *const libc::c_char,
            190 as libc::c_int,
            2 as libc::c_int | sudo_debug_subsys,
            b"%s: unable to create signal pipe\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"sudo_ev_base_init\0"))
                .as_ptr(),
        );
    } else {
        sudo_ev_init(
            &mut (*base).signal_event,
            (*base).signal_pipe[0 as libc::c_int as usize],
            (0x2 as libc::c_int | 0x8 as libc::c_int) as libc::c_short,
            Some(
                signal_pipe_cb
                    as unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_void) -> (),
            ),
            base as *mut libc::c_void,
        );
        let mut sudo_debug_ret: libc::c_int = 0 as libc::c_int;
        sudo_debug_exit_int_v1(
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"sudo_ev_base_init\0"))
                .as_ptr(),
            b"event.c\0" as *const u8 as *const libc::c_char,
            196 as libc::c_int,
            sudo_debug_subsys,
            sudo_debug_ret,
        );
        return sudo_debug_ret;
    }
        sudo_ev_base_free_impl(base);
    let mut sudo_debug_ret_0: libc::c_int = -(1 as libc::c_int);
    sudo_debug_exit_int_v1(
        (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"sudo_ev_base_init\0"))
            .as_ptr(),
        b"event.c\0" as *const u8 as *const libc::c_char,
        200 as libc::c_int,
        sudo_debug_subsys,
        sudo_debug_ret_0,
    );
    return sudo_debug_ret_0;
}

#[no_mangle]
pub unsafe extern "C" fn sudo_ev_base_alloc_v1() -> *mut sudo_event_base {
    let mut base: *mut sudo_event_base = 0 as *mut sudo_event_base;
    let sudo_debug_subsys: libc::c_int = (4 as libc::c_int) << 6 as libc::c_int;
    sudo_debug_enter_v1(
        (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"sudo_ev_base_alloc_v1\0"))
            .as_ptr(),
        b"event.c\0" as *const u8 as *const libc::c_char,
        207 as libc::c_int,
        sudo_debug_subsys,
    );
        base = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<sudo_event_base>() as libc::c_ulong,
    ) as *mut sudo_event_base;
    if base.is_null() {
        sudo_debug_printf2_v1(
            (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"sudo_ev_base_alloc_v1\0"))
                .as_ptr(),
            b"event.c\0" as *const u8 as *const libc::c_char,
            212 as libc::c_int,
            2 as libc::c_int | (1 as libc::c_int) << 5 as libc::c_int | sudo_debug_subsys,
            b"%s: unable to allocate base\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"sudo_ev_base_alloc_v1\0"))
                .as_ptr(),
        );
        let mut sudo_debug_ret: *mut libc::c_void = 0 as *mut libc::c_void;
        sudo_debug_exit_ptr_v1(
            (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"sudo_ev_base_alloc_v1\0"))
                .as_ptr(),
            b"event.c\0" as *const u8 as *const libc::c_char,
            213 as libc::c_int,
            sudo_debug_subsys,
            sudo_debug_ret,
        );
        return sudo_debug_ret as *mut sudo_event_base;
    }

    if sudo_ev_base_init(base) != 0 as libc::c_int {
        free(base as *mut libc::c_void);
        let mut sudo_debug_ret_0: *mut libc::c_void = 0 as *mut libc::c_void;
        sudo_debug_exit_ptr_v1(
            (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"sudo_ev_base_alloc_v1\0"))
                .as_ptr(),
            b"event.c\0" as *const u8 as *const libc::c_char,
            217 as libc::c_int,
            sudo_debug_subsys,
            sudo_debug_ret_0,
        );
        return sudo_debug_ret_0 as *mut sudo_event_base;
    }
        let mut sudo_debug_ret_1: *mut libc::c_void = base as *mut libc::c_void;
    sudo_debug_exit_ptr_v1(
        (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"sudo_ev_base_alloc_v1\0"))
            .as_ptr(),
        b"event.c\0" as *const u8 as *const libc::c_char,
        219 as libc::c_int,
        sudo_debug_subsys,
        sudo_debug_ret_1,
    );
    return sudo_debug_ret_1 as *mut sudo_event_base;
}

#[no_mangle]
pub unsafe extern "C" fn sudo_ev_base_free_v1(mut base: *mut sudo_event_base) {
    let mut ev: *mut sudo_event = 0 as *mut sudo_event;
    let mut next: *mut sudo_event = 0 as *mut sudo_event;
    let mut i: libc::c_int = 0;
    let sudo_debug_subsys: libc::c_int = (4 as libc::c_int) << 6 as libc::c_int;
    sudo_debug_enter_v1(
        (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"sudo_ev_base_free_v1\0"))
            .as_ptr(),
        b"event.c\0" as *const u8 as *const libc::c_char,
        227 as libc::c_int,
        sudo_debug_subsys,
    );
    if base.is_null() {
        sudo_debug_exit_v1(
            (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"sudo_ev_base_free_v1\0"))
                .as_ptr(),
            b"event.c\0" as *const u8 as *const libc::c_char,
            230 as libc::c_int,
            sudo_debug_subsys,
        );
        return;
    }
    if default_base == base {
        default_base = 0 as *mut sudo_event_base;
    }
    ev = (*base).events.tqh_first;
    while !ev.is_null() && {
        next = (*ev).entries.tqe_next;
        1 as libc::c_int != 0
    } {
        sudo_ev_del_v1(base, ev);
        ev = next;
    }
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int + 1 as libc::c_int {
        ev = (*base).signals[i as usize].tqh_first;
        while !ev.is_null() && {
            next = (*ev).entries.tqe_next;
            1 as libc::c_int != 0
        } {
            sudo_ev_del_v1(base, ev);
            ev = next;
        }
        free((*base).siginfo[i as usize] as *mut libc::c_void);
        free((*base).orig_handlers[i as usize] as *mut libc::c_void);
        i += 1;
    }
    sudo_ev_base_free_impl(base);
    close((*base).signal_pipe[0 as libc::c_int as usize]);
    close((*base).signal_pipe[1 as libc::c_int as usize]);
    free(base as *mut libc::c_void);
    sudo_debug_exit_v1(
        (*::core::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"sudo_ev_base_free_v1\0"))
            .as_ptr(),
        b"event.c\0" as *const u8 as *const libc::c_char,
        252 as libc::c_int,
        sudo_debug_subsys,
    );
}