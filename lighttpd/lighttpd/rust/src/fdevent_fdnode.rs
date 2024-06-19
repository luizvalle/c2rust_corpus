use ::libc;
extern "C" {
    pub type epoll_event;
    pub type pollfd;
    pub type fdlog_st;
    fn ck_calloc(nmemb: size_t, elt_sz: size_t) -> *mut libc::c_void;
    fn log_perror(
        errh: *mut log_error_st,
        filename: *const libc::c_char,
        line: libc::c_uint,
        fmt: *const libc::c_char,
        _: ...
    );
    fn free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
}
pub type __uint32_t = libc::c_uint;
pub type size_t = libc::c_ulong;
pub type uint32_t = __uint32_t;
pub type uintptr_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fdnode_st {
    pub handler: fdevent_handler,
    pub ctx: *mut libc::c_void,
    pub fd: libc::c_int,
    pub events: libc::c_int,
    pub fde_ndx: libc::c_int,
}
pub type fdevent_handler = Option::<
    unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> handler_t,
>;
pub type handler_t = libc::c_uint;
pub const HANDLER_ERROR: handler_t = 4;
pub const HANDLER_COMEBACK: handler_t = 3;
pub const HANDLER_WAIT_FOR_EVENT: handler_t = 2;
pub const HANDLER_FINISHED: handler_t = 1;
pub const HANDLER_GO_ON: handler_t = 0;
pub type fdnode = fdnode_st;
pub type log_error_st = fdlog_st;
pub type fdevent_handler_t = libc::c_uint;
pub const FDEVENT_HANDLER_FREEBSD_KQUEUE: fdevent_handler_t = 6;
pub const FDEVENT_HANDLER_SOLARIS_PORT: fdevent_handler_t = 5;
pub const FDEVENT_HANDLER_SOLARIS_DEVPOLL: fdevent_handler_t = 4;
pub const FDEVENT_HANDLER_LINUX_SYSEPOLL: fdevent_handler_t = 3;
pub const FDEVENT_HANDLER_POLL: fdevent_handler_t = 2;
pub const FDEVENT_HANDLER_SELECT: fdevent_handler_t = 1;
pub const FDEVENT_HANDLER_UNSET: fdevent_handler_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buffer_int {
    pub ptr: *mut libc::c_int,
    pub used: uint32_t,
    pub size: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fdevents {
    pub fdarray: *mut *mut fdnode,
    pub pendclose: *mut fdnode,
    pub event_set: Option::<
        unsafe extern "C" fn(*mut fdevents, *mut fdnode, libc::c_int) -> libc::c_int,
    >,
    pub event_del: Option::<
        unsafe extern "C" fn(*mut fdevents, *mut fdnode) -> libc::c_int,
    >,
    pub poll: Option::<unsafe extern "C" fn(*mut fdevents, libc::c_int) -> libc::c_int>,
    pub errh: *mut log_error_st,
    pub cur_fds: *mut libc::c_int,
    pub maxfds: uint32_t,
    pub epoll_fd: libc::c_int,
    pub epoll_events: *mut epoll_event,
    pub pollfds: *mut pollfd,
    pub size: uint32_t,
    pub used: uint32_t,
    pub unused: buffer_int,
    pub reset: Option::<unsafe extern "C" fn(*mut fdevents) -> libc::c_int>,
    pub free: Option::<unsafe extern "C" fn(*mut fdevents) -> ()>,
    pub event_handler: *const libc::c_char,
    pub type_0: fdevent_handler_t,
}
unsafe extern "C" fn fdnode_init() -> *mut fdnode {
    return ck_calloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<fdnode>() as libc::c_ulong,
    ) as *mut fdnode;
}
unsafe extern "C" fn fdnode_free(mut fdn: *mut fdnode) {
    free(fdn as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn fdevent_register(
    mut ev: *mut fdevents,
    mut fd: libc::c_int,
    mut handler: fdevent_handler,
    mut ctx: *mut libc::c_void,
) -> *mut fdnode {
    let ref mut fresh0 = *((*ev).fdarray).offset(fd as isize);
    *fresh0 = fdnode_init();
    let mut fdn: *mut fdnode = *fresh0;
    (*fdn).handler = handler;
    (*fdn).fd = fd;
    (*fdn).ctx = ctx;
    (*fdn).events = 0 as libc::c_int;
    (*fdn).fde_ndx = -(1 as libc::c_int);
    return fdn;
}
#[no_mangle]
pub unsafe extern "C" fn fdevent_unregister(
    mut ev: *mut fdevents,
    mut fdn: *mut fdnode,
) {
    let mut fdn_slot: *mut *mut fdnode = &mut *((*ev).fdarray).offset((*fdn).fd as isize)
        as *mut *mut fdnode;
    if *fdn_slot as uintptr_t & 0x3 as libc::c_int as libc::c_ulong != 0 {
        return;
    }
    *fdn_slot = 0 as *mut fdnode;
    fdnode_free(fdn);
}
#[no_mangle]
pub unsafe extern "C" fn fdevent_sched_close(
    mut ev: *mut fdevents,
    mut fdn: *mut fdnode,
) {
    let mut fdn_slot: *mut *mut fdnode = &mut *((*ev).fdarray).offset((*fdn).fd as isize)
        as *mut *mut fdnode;
    if *fdn_slot as uintptr_t & 0x3 as libc::c_int as libc::c_ulong != 0 {
        return;
    }
    *fdn_slot = (fdn as uintptr_t | 0x3 as libc::c_int as libc::c_ulong) as *mut fdnode;
    (*fdn)
        .handler = ::core::mem::transmute::<
        *mut libc::c_void,
        fdevent_handler,
    >(0 as *mut libc::c_void);
    (*fdn).ctx = (*ev).pendclose as *mut libc::c_void;
    (*ev).pendclose = fdn;
}
#[cold]
#[inline(never)]
unsafe extern "C" fn fdevent_fdnode_event_unsetter_retry(
    mut ev: *mut fdevents,
    mut fdn: *mut fdnode,
) -> libc::c_int {
    loop {
        match *__errno_location() {
            11 | 4 => {}
            _ => {
                log_perror(
                    (*ev).errh,
                    b"fdevent_fdnode.c\0" as *const u8 as *const libc::c_char,
                    96 as libc::c_int as libc::c_uint,
                    b"fdevent event_del failed on fd %d\0" as *const u8
                        as *const libc::c_char,
                    (*fdn).fd,
                );
                return 0 as libc::c_int;
            }
        }
        if !(0 as libc::c_int
            != ((*ev).event_del).expect("non-null function pointer")(ev, fdn))
        {
            break;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn fdevent_fdnode_event_unsetter(
    mut ev: *mut fdevents,
    mut fdn: *mut fdnode,
) {
    if -(1 as libc::c_int) == (*fdn).fde_ndx {
        return;
    }
    if 0 as libc::c_int != ((*ev).event_del).expect("non-null function pointer")(ev, fdn)
    {
        fdevent_fdnode_event_unsetter_retry(ev, fdn);
    }
    (*fdn).fde_ndx = -(1 as libc::c_int);
    (*fdn).events = 0 as libc::c_int;
}
#[cold]
#[inline(never)]
unsafe extern "C" fn fdevent_fdnode_event_setter_retry(
    mut ev: *mut fdevents,
    mut fdn: *mut fdnode,
    mut events: libc::c_int,
) -> libc::c_int {
    loop {
        match *__errno_location() {
            11 | 4 => {}
            _ => {
                log_perror(
                    (*ev).errh,
                    b"fdevent_fdnode.c\0" as *const u8 as *const libc::c_char,
                    133 as libc::c_int as libc::c_uint,
                    b"fdevent event_set failed on fd %d\0" as *const u8
                        as *const libc::c_char,
                    (*fdn).fd,
                );
                return 0 as libc::c_int;
            }
        }
        if !(0 as libc::c_int
            != ((*ev).event_set).expect("non-null function pointer")(ev, fdn, events))
        {
            break;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn fdevent_fdnode_event_setter(
    mut ev: *mut fdevents,
    mut fdn: *mut fdnode,
    mut events: libc::c_int,
) {
    if (*fdn).events == events {
        return;
    }
    if 0 as libc::c_int
        == ((*ev).event_set).expect("non-null function pointer")(ev, fdn, events)
        || fdevent_fdnode_event_setter_retry(ev, fdn, events) != 0
    {
        (*fdn).events = events;
    }
}
#[no_mangle]
pub unsafe extern "C" fn fdevent_fdnode_event_del(
    mut ev: *mut fdevents,
    mut fdn: *mut fdnode,
) {
    if !fdn.is_null() {
        fdevent_fdnode_event_unsetter(ev, fdn);
    }
}
#[no_mangle]
pub unsafe extern "C" fn fdevent_fdnode_event_set(
    mut ev: *mut fdevents,
    mut fdn: *mut fdnode,
    mut events: libc::c_int,
) {
    if !fdn.is_null() {
        fdevent_fdnode_event_setter(ev, fdn, events);
    }
}
#[no_mangle]
pub unsafe extern "C" fn fdevent_fdnode_event_add(
    mut ev: *mut fdevents,
    mut fdn: *mut fdnode,
    mut event: libc::c_int,
) {
    if !fdn.is_null() {
        fdevent_fdnode_event_setter(ev, fdn, (*fdn).events | event);
    }
}
#[no_mangle]
pub unsafe extern "C" fn fdevent_fdnode_event_clr(
    mut ev: *mut fdevents,
    mut fdn: *mut fdnode,
    mut event: libc::c_int,
) {
    if !fdn.is_null() {
        fdevent_fdnode_event_setter(ev, fdn, (*fdn).events & !event);
    }
}
