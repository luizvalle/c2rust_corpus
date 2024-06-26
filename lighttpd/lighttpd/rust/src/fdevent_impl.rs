use ::libc;
extern "C" {
    pub type fdlog_st;
    fn fdevent_socket_nb_cloexec_init();
    fn ck_calloc(nmemb: size_t, elt_sz: size_t) -> *mut libc::c_void;
    fn ck_realloc_u32(
        list: *mut *mut libc::c_void,
        n: size_t,
        x: size_t,
        elt_sz: size_t,
    ) -> *mut libc::c_void;
    fn log_error(
        errh: *mut log_error_st,
        filename: *const libc::c_char,
        line: libc::c_uint,
        fmt: *const libc::c_char,
        _: ...
    );
    fn log_perror(
        errh: *mut log_error_st,
        filename: *const libc::c_char,
        line: libc::c_uint,
        fmt: *const libc::c_char,
        _: ...
    );
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn free(_: *mut libc::c_void);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn epoll_create1(__flags: libc::c_int) -> libc::c_int;
    fn epoll_ctl(
        __epfd: libc::c_int,
        __op: libc::c_int,
        __fd: libc::c_int,
        __event: *mut epoll_event,
    ) -> libc::c_int;
    fn epoll_wait(
        __epfd: libc::c_int,
        __events: *mut epoll_event,
        __maxevents: libc::c_int,
        __timeout: libc::c_int,
    ) -> libc::c_int;
    fn poll(__fds: *mut pollfd, __nfds: nfds_t, __timeout: libc::c_int) -> libc::c_int;
}
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type size_t = libc::c_ulong;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type uintptr_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct epoll_event {
    pub events: uint32_t,
    pub data: epoll_data_t,
}
pub type epoll_data_t = epoll_data;
#[derive(Copy, Clone)]
#[repr(C)]
pub union epoll_data {
    pub ptr: *mut libc::c_void,
    pub fd: libc::c_int,
    pub u32_0: uint32_t,
    pub u64_0: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ev_map {
    pub et: fdevent_handler_t,
    pub name: *const libc::c_char,
}
pub const EPOLL_CLOEXEC: C2RustUnnamed = 524288;
pub const EPOLLHUP: EPOLL_EVENTS = 16;
pub const EPOLLERR: EPOLL_EVENTS = 8;
pub type nfds_t = libc::c_ulong;
pub type C2RustUnnamed = libc::c_uint;
pub type EPOLL_EVENTS = libc::c_uint;
pub const EPOLLET: EPOLL_EVENTS = 2147483648;
pub const EPOLLONESHOT: EPOLL_EVENTS = 1073741824;
pub const EPOLLWAKEUP: EPOLL_EVENTS = 536870912;
pub const EPOLLEXCLUSIVE: EPOLL_EVENTS = 268435456;
pub const EPOLLRDHUP: EPOLL_EVENTS = 8192;
pub const EPOLLMSG: EPOLL_EVENTS = 1024;
pub const EPOLLWRBAND: EPOLL_EVENTS = 512;
pub const EPOLLWRNORM: EPOLL_EVENTS = 256;
pub const EPOLLRDBAND: EPOLL_EVENTS = 128;
pub const EPOLLRDNORM: EPOLL_EVENTS = 64;
pub const EPOLLOUT: EPOLL_EVENTS = 4;
pub const EPOLLPRI: EPOLL_EVENTS = 2;
pub const EPOLLIN: EPOLL_EVENTS = 1;
#[no_mangle]
#[cold]
pub unsafe extern "C" fn fdevent_config(
    mut event_handler_name: *mut *const libc::c_char,
    mut errh: *mut log_error_st,
) -> libc::c_int {
    fdevent_socket_nb_cloexec_init();
    static mut event_handlers: [ev_map; 4] = [
        {
            let mut init = ev_map {
                et: FDEVENT_HANDLER_LINUX_SYSEPOLL,
                name: b"linux-sysepoll\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ev_map {
                et: FDEVENT_HANDLER_LINUX_SYSEPOLL,
                name: b"epoll\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ev_map {
                et: FDEVENT_HANDLER_POLL,
                name: b"poll\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = ev_map {
                et: FDEVENT_HANDLER_UNSET,
                name: 0 as *const libc::c_char,
            };
            init
        },
    ];
    let mut event_handler: *const libc::c_char = *event_handler_name;
    let mut et: fdevent_handler_t = FDEVENT_HANDLER_UNSET;
    if !event_handler.is_null()
        && 0 as libc::c_int
            == strcmp(event_handler, b"libev\0" as *const u8 as *const libc::c_char)
    {
        event_handler = 0 as *const libc::c_char;
    }
    if !event_handler.is_null()
        && 0 as libc::c_int
            == strcmp(event_handler, b"select\0" as *const u8 as *const libc::c_char)
    {
        event_handler = b"poll\0" as *const u8 as *const libc::c_char;
    }
    if event_handler.is_null() {
        et = event_handlers[0 as libc::c_int as usize].et;
        *event_handler_name = event_handlers[0 as libc::c_int as usize].name;
        if FDEVENT_HANDLER_UNSET as libc::c_int as libc::c_uint == et as libc::c_uint {
            log_error(
                errh,
                b"fdevent_impl.c\0" as *const u8 as *const libc::c_char,
                103 as libc::c_int as libc::c_uint,
                b"sorry, there is no event handler for this system\0" as *const u8
                    as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    } else {
        let mut i: uint32_t = 0 as libc::c_int as uint32_t;
        while !(event_handlers[i as usize].name).is_null() {
            if 0 as libc::c_int == strcmp(event_handlers[i as usize].name, event_handler)
            {
                et = event_handlers[i as usize].et;
                break;
            } else {
                i = i.wrapping_add(1);
                i;
            }
        }
        if FDEVENT_HANDLER_UNSET as libc::c_int as libc::c_uint == et as libc::c_uint {
            log_error(
                errh,
                b"fdevent_impl.c\0" as *const u8 as *const libc::c_char,
                122 as libc::c_int as libc::c_uint,
                b"the selected event-handler in unknown or not supported: %s\0"
                    as *const u8 as *const libc::c_char,
                event_handler,
            );
            return -(1 as libc::c_int);
        }
    }
    return et as libc::c_int;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn fdevent_show_event_handlers() -> *const libc::c_char {
    return b"\nEvent Handlers:\n\n\t- select (generic)\n\t+ poll (Unix)\n\t+ epoll (Linux)\n\t- /dev/poll (Solaris)\n\t- eventports (Solaris)\n\t- kqueue (FreeBSD)\n\0"
        as *const u8 as *const libc::c_char;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn fdevent_init(
    mut event_handler: *const libc::c_char,
    mut max_fds: *mut libc::c_int,
    mut cur_fds: *mut libc::c_int,
    mut errh: *mut log_error_st,
) -> *mut fdevents {
    let mut ev: *mut fdevents = 0 as *mut fdevents;
    let mut maxfds: uint32_t = if 0 as libc::c_int != *max_fds {
        *max_fds as uint32_t
    } else {
        4096 as libc::c_int as libc::c_uint
    };
    let mut type_0: libc::c_int = fdevent_config(&mut event_handler, errh);
    if type_0 <= 0 as libc::c_int {
        return 0 as *mut fdevents;
    }
    *max_fds = maxfds as libc::c_int;
    maxfds = maxfds.wrapping_add(1);
    maxfds;
    ev = ck_calloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<fdevents>() as libc::c_ulong,
    ) as *mut fdevents;
    (*ev).errh = errh;
    (*ev).cur_fds = cur_fds;
    (*ev).event_handler = event_handler;
    (*ev)
        .fdarray = ck_calloc(
        maxfds as size_t,
        ::core::mem::size_of::<*mut fdnode>() as libc::c_ulong,
    ) as *mut *mut fdnode;
    (*ev).maxfds = maxfds;
    match type_0 {
        2 => {
            if 0 as libc::c_int == fdevent_poll_init(ev) {
                return ev;
            }
        }
        3 => {
            if 0 as libc::c_int == fdevent_linux_sysepoll_init(ev) {
                return ev;
            }
        }
        _ => {}
    }
    free((*ev).fdarray as *mut libc::c_void);
    free(ev as *mut libc::c_void);
    log_error(
        errh,
        b"fdevent_impl.c\0" as *const u8 as *const libc::c_char,
        240 as libc::c_int as libc::c_uint,
        b"event-handler failed: %s; try to set server.event-handler = \"poll\" or \"select\"\0"
            as *const u8 as *const libc::c_char,
        event_handler,
    );
    return 0 as *mut fdevents;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn fdevent_free(mut ev: *mut fdevents) {
    if ev.is_null() {
        return;
    }
    if ((*ev).free).is_some() {
        ((*ev).free).expect("non-null function pointer")(ev);
    }
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*ev).maxfds {
        if !(*((*ev).fdarray).offset(i as isize)).is_null() {
            free(
                (*((*ev).fdarray).offset(i as isize) as uintptr_t
                    & !(0x3 as libc::c_int) as libc::c_ulong) as *mut fdnode
                    as *mut libc::c_void,
            );
        }
        i = i.wrapping_add(1);
        i;
    }
    free((*ev).fdarray as *mut libc::c_void);
    free(ev as *mut libc::c_void);
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn fdevent_reset(mut ev: *mut fdevents) -> libc::c_int {
    let mut rc: libc::c_int = if ((*ev).reset).is_some() {
        ((*ev).reset).expect("non-null function pointer")(ev)
    } else {
        0 as libc::c_int
    };
    if -(1 as libc::c_int) == rc {
        log_error(
            (*ev).errh,
            b"fdevent_impl.c\0" as *const u8 as *const libc::c_char,
            271 as libc::c_int as libc::c_uint,
            b"event-handler failed: %s; try to set server.event-handler = \"poll\" or \"select\"\0"
                as *const u8 as *const libc::c_char,
            if !((*ev).event_handler).is_null() {
                (*ev).event_handler
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        );
    }
    return rc;
}
unsafe extern "C" fn fdevent_sched_run(ev: *mut fdevents) {
    let mut fdn: *mut fdnode = (*ev).pendclose;
    while !fdn.is_null() {
        let mut fd: libc::c_int = (*fdn).fd;
        if 0 as libc::c_int != close(fd) {
            log_perror(
                (*ev).errh,
                b"fdevent_impl.c\0" as *const u8 as *const libc::c_char,
                291 as libc::c_int as libc::c_uint,
                b"close() %d\0" as *const u8 as *const libc::c_char,
                fd,
            );
        }
        *(*ev).cur_fds -= 1;
        *(*ev).cur_fds;
        let fdn_tmp: *mut fdnode = fdn;
        fdn = (*fdn).ctx as *mut fdnode;
        let ref mut fresh0 = *((*ev).fdarray).offset(fd as isize);
        *fresh0 = 0 as *mut fdnode;
        free(fdn_tmp as *mut libc::c_void);
    }
    (*ev).pendclose = 0 as *mut fdnode;
}
#[no_mangle]
pub unsafe extern "C" fn fdevent_poll(
    ev: *mut fdevents,
    timeout_ms: libc::c_int,
) -> libc::c_int {
    let n: libc::c_int = ((*ev).poll)
        .expect(
            "non-null function pointer",
        )(ev, if !((*ev).pendclose).is_null() { 0 as libc::c_int } else { timeout_ms });
    if n >= 0 as libc::c_int {
        fdevent_sched_run(ev);
    } else if *__errno_location() != 4 as libc::c_int {
        log_perror(
            (*ev).errh,
            b"fdevent_impl.c\0" as *const u8 as *const libc::c_char,
            318 as libc::c_int as libc::c_uint,
            b"fdevent_poll()\0" as *const u8 as *const libc::c_char,
        );
    }
    return n;
}
unsafe extern "C" fn fdevent_linux_sysepoll_event_del(
    mut ev: *mut fdevents,
    mut fdn: *mut fdnode,
) -> libc::c_int {
    return epoll_ctl((*ev).epoll_fd, 2 as libc::c_int, (*fdn).fd, 0 as *mut epoll_event);
}
unsafe extern "C" fn fdevent_linux_sysepoll_event_set(
    mut ev: *mut fdevents,
    mut fdn: *mut fdnode,
    mut events: libc::c_int,
) -> libc::c_int {
    let mut op: libc::c_int = if -(1 as libc::c_int) == (*fdn).fde_ndx {
        1 as libc::c_int
    } else {
        3 as libc::c_int
    };
    (*fdn).fde_ndx = (*fdn).fd;
    let mut fd: libc::c_int = (*fdn).fde_ndx;
    let mut ep: epoll_event = epoll_event {
        events: 0,
        data: epoll_data {
            ptr: 0 as *mut libc::c_void,
        },
    };
    ep.events = (events | EPOLLERR as libc::c_int | EPOLLHUP as libc::c_int) as uint32_t;
    ep.data.ptr = fdn as *mut libc::c_void;
    return epoll_ctl((*ev).epoll_fd, op, fd, &mut ep);
}
unsafe extern "C" fn fdevent_linux_sysepoll_poll(
    ev: *mut fdevents,
    mut timeout_ms: libc::c_int,
) -> libc::c_int {
    let epoll_events: *mut epoll_event = (*ev).epoll_events;
    let mut n: libc::c_int = epoll_wait(
        (*ev).epoll_fd,
        epoll_events,
        (*ev).maxfds as libc::c_int,
        timeout_ms,
    );
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        let fdn: *mut fdnode = (*epoll_events.offset(i as isize)).data.ptr
            as *mut fdnode;
        let mut revents: libc::c_int = (*epoll_events.offset(i as isize)).events
            as libc::c_int;
        if ((*fdn).handler).is_some() {
            (Some(((*fdn).handler).expect("non-null function pointer")))
                .expect("non-null function pointer")((*fdn).ctx, revents);
        }
        i += 1;
        i;
    }
    return n;
}
#[cold]
unsafe extern "C" fn fdevent_linux_sysepoll_free(mut ev: *mut fdevents) {
    close((*ev).epoll_fd);
    free((*ev).epoll_events as *mut libc::c_void);
}
#[cold]
unsafe extern "C" fn fdevent_linux_sysepoll_init(mut ev: *mut fdevents) -> libc::c_int {
    (*ev).type_0 = FDEVENT_HANDLER_LINUX_SYSEPOLL;
    (*ev)
        .event_set = Some(
        fdevent_linux_sysepoll_event_set
            as unsafe extern "C" fn(
                *mut fdevents,
                *mut fdnode,
                libc::c_int,
            ) -> libc::c_int,
    );
    (*ev)
        .event_del = Some(
        fdevent_linux_sysepoll_event_del
            as unsafe extern "C" fn(*mut fdevents, *mut fdnode) -> libc::c_int,
    );
    (*ev)
        .poll = Some(
        fdevent_linux_sysepoll_poll
            as unsafe extern "C" fn(*mut fdevents, libc::c_int) -> libc::c_int,
    );
    (*ev)
        .free = Some(
        fdevent_linux_sysepoll_free as unsafe extern "C" fn(*mut fdevents) -> (),
    );
    (*ev).epoll_fd = epoll_create1(EPOLL_CLOEXEC as libc::c_int);
    if -(1 as libc::c_int) == (*ev).epoll_fd {
        return -(1 as libc::c_int);
    }
    (*ev)
        .epoll_events = ck_calloc(
        (*ev).maxfds as size_t,
        ::core::mem::size_of::<epoll_event>() as libc::c_ulong,
    ) as *mut epoll_event;
    return 0 as libc::c_int;
}
unsafe extern "C" fn fdevent_poll_event_del(
    mut ev: *mut fdevents,
    mut fdn: *mut fdnode,
) -> libc::c_int {
    let mut fd: libc::c_int = (*fdn).fd;
    let mut k: libc::c_int = (*fdn).fde_ndx;
    if k as uint32_t >= (*ev).used || (*((*ev).pollfds).offset(k as isize)).fd != fd {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    (*((*ev).pollfds).offset(k as isize)).fd = -(1 as libc::c_int);
    if (*ev).unused.size == (*ev).unused.used {
        ck_realloc_u32(
            &mut (*ev).unused.ptr as *mut *mut libc::c_int as *mut *mut libc::c_void,
            (*ev).unused.size as size_t,
            16 as libc::c_int as size_t,
            ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        );
        (*ev)
            .unused
            .size = ((*ev).unused.size as libc::c_uint)
            .wrapping_add(16 as libc::c_int as libc::c_uint) as uint32_t as uint32_t;
    }
    let fresh1 = (*ev).unused.used;
    (*ev).unused.used = ((*ev).unused.used).wrapping_add(1);
    *((*ev).unused.ptr).offset(fresh1 as isize) = k;
    return 0 as libc::c_int;
}
unsafe extern "C" fn fdevent_poll_event_set(
    mut ev: *mut fdevents,
    mut fdn: *mut fdnode,
    mut events: libc::c_int,
) -> libc::c_int {
    let mut fd: libc::c_int = (*fdn).fd;
    let mut k: libc::c_int = (*fdn).fde_ndx;
    if k >= 0 as libc::c_int {
        if k as uint32_t >= (*ev).used || (*((*ev).pollfds).offset(k as isize)).fd != fd
        {
            *__errno_location() = 22 as libc::c_int;
            return -(1 as libc::c_int);
        }
        (*((*ev).pollfds).offset(k as isize)).events = events as libc::c_short;
        return 0 as libc::c_int;
    }
    if (*ev).unused.used > 0 as libc::c_int as libc::c_uint {
        (*ev).unused.used = ((*ev).unused.used).wrapping_sub(1);
        k = *((*ev).unused.ptr).offset((*ev).unused.used as isize);
    } else {
        if (*ev).size == (*ev).used {
            ck_realloc_u32(
                &mut (*ev).pollfds as *mut *mut pollfd as *mut *mut libc::c_void,
                (*ev).size as size_t,
                16 as libc::c_int as size_t,
                ::core::mem::size_of::<pollfd>() as libc::c_ulong,
            );
            (*ev)
                .size = ((*ev).size as libc::c_uint)
                .wrapping_add(16 as libc::c_int as libc::c_uint) as uint32_t as uint32_t;
        }
        let fresh2 = (*ev).used;
        (*ev).used = ((*ev).used).wrapping_add(1);
        k = fresh2 as libc::c_int;
    }
    (*fdn).fde_ndx = k;
    (*((*ev).pollfds).offset(k as isize)).fd = fd;
    (*((*ev).pollfds).offset(k as isize)).events = events as libc::c_short;
    return 0 as libc::c_int;
}
unsafe extern "C" fn fdevent_poll_poll(
    mut ev: *mut fdevents,
    mut timeout_ms: libc::c_int,
) -> libc::c_int {
    let n: libc::c_int = poll((*ev).pollfds, (*ev).used as nfds_t, timeout_ms);
    let fdarray: *mut *mut fdnode = (*ev).fdarray;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut m: libc::c_int = 0 as libc::c_int;
    while m < n {
        let pfds: *mut pollfd = (*ev).pollfds;
        while 0 as libc::c_int == (*pfds.offset(i as isize)).revents as libc::c_int {
            i += 1;
            i;
        }
        let mut fdn: *mut fdnode = *fdarray
            .offset((*pfds.offset(i as isize)).fd as isize);
        if 0 as libc::c_int as libc::c_ulong
            == fdn as uintptr_t & 0x3 as libc::c_int as libc::c_ulong
        {
            (Some(((*fdn).handler).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )((*fdn).ctx, (*pfds.offset(i as isize)).revents as libc::c_int);
        }
        i += 1;
        i;
        m += 1;
        m;
    }
    return n;
}
#[cold]
unsafe extern "C" fn fdevent_poll_free(mut ev: *mut fdevents) {
    free((*ev).pollfds as *mut libc::c_void);
    if !((*ev).unused.ptr).is_null() {
        free((*ev).unused.ptr as *mut libc::c_void);
    }
}
#[cold]
unsafe extern "C" fn fdevent_poll_init(mut ev: *mut fdevents) -> libc::c_int {
    (*ev).type_0 = FDEVENT_HANDLER_POLL;
    (*ev)
        .event_set = Some(
        fdevent_poll_event_set
            as unsafe extern "C" fn(
                *mut fdevents,
                *mut fdnode,
                libc::c_int,
            ) -> libc::c_int,
    );
    (*ev)
        .event_del = Some(
        fdevent_poll_event_del
            as unsafe extern "C" fn(*mut fdevents, *mut fdnode) -> libc::c_int,
    );
    (*ev)
        .poll = Some(
        fdevent_poll_poll
            as unsafe extern "C" fn(*mut fdevents, libc::c_int) -> libc::c_int,
    );
    (*ev).free = Some(fdevent_poll_free as unsafe extern "C" fn(*mut fdevents) -> ());
    return 0 as libc::c_int;
}
