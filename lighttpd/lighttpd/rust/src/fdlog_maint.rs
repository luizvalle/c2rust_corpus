use ::libc;
extern "C" {
    fn buffer_free_ptr(b: *mut buffer);
    fn fdlog_init(
        fn_0: *const libc::c_char,
        fd: libc::c_int,
        mode: libc::c_int,
    ) -> *mut fdlog_st;
    fn fdlog_free(fdlog: *mut fdlog_st);
    fn ck_realloc_u32(
        list: *mut *mut libc::c_void,
        n: size_t,
        x: size_t,
        elt_sz: size_t,
    ) -> *mut libc::c_void;
    fn __errno_location() -> *mut libc::c_int;
    fn free(_: *mut libc::c_void);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    fn fdevent_fcntl_set_nb(fd: libc::c_int) -> libc::c_int;
    fn fdevent_open_cloexec(
        pathname: *const libc::c_char,
        symlinks: libc::c_int,
        flags: libc::c_int,
        mode: mode_t,
    ) -> libc::c_int;
    fn fdevent_pipe_cloexec(
        fds: *mut libc::c_int,
        bufsz_hint: libc::c_uint,
    ) -> libc::c_int;
    fn fdevent_open_devnull() -> libc::c_int;
    fn fdevent_sh_exec(
        cmdstr: *const libc::c_char,
        envp: *mut *mut libc::c_char,
        fdin: libc::c_int,
        fdout: libc::c_int,
        fderr: libc::c_int,
    ) -> pid_t;
    static mut log_monotonic_secs: unix_time64_t;
    fn write_all(fd: libc::c_int, buf: *const libc::c_void, count: size_t) -> ssize_t;
    fn log_pri(
        errh: *mut log_error_st,
        filename: *const libc::c_char,
        line: libc::c_uint,
        pri: libc::c_int,
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
    fn closelog();
    fn openlog(
        __ident: *const libc::c_char,
        __option: libc::c_int,
        __facility: libc::c_int,
    );
}
pub type __uint32_t = libc::c_uint;
pub type __mode_t = libc::c_uint;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type mode_t = __mode_t;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
pub type uint32_t = __uint32_t;
pub type unix_time64_t = time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fdlog_st {
    pub mode: C2RustUnnamed,
    pub fd: libc::c_int,
    pub b: buffer,
    pub fn_0: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buffer {
    pub ptr: *mut libc::c_char,
    pub used: uint32_t,
    pub size: uint32_t,
}
pub type C2RustUnnamed = libc::c_uint;
pub const FDLOG_PIPE: C2RustUnnamed = 3;
pub const FDLOG_SYSLOG: C2RustUnnamed = 2;
pub const FDLOG_FD: C2RustUnnamed = 1;
pub const FDLOG_FILE: C2RustUnnamed = 0;
pub type log_error_st = fdlog_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fdlog_pipe {
    pub fdlog: *mut fdlog_st,
    pub pid: pid_t,
    pub fd: libc::c_int,
    pub start: unix_time64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fdlog_pipes_t {
    pub ptr: *mut fdlog_pipe,
    pub used: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fdlog_files_t {
    pub ptr: *mut *mut fdlog_st,
    pub used: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct facility_name_st {
    pub name: *const libc::c_char,
    pub val: libc::c_int,
}
#[inline]
unsafe extern "C" fn buffer_is_blank(mut b: *const buffer) -> libc::c_int {
    return ((*b).used < 2 as libc::c_int as libc::c_uint) as libc::c_int;
}
#[inline]
unsafe extern "C" fn buffer_clen(mut b: *const buffer) -> uint32_t {
    return ((*b).used)
        .wrapping_sub(
            (0 as libc::c_int as libc::c_uint != (*b).used) as libc::c_int
                as libc::c_uint,
        );
}
#[inline]
unsafe extern "C" fn buffer_clear(mut b: *mut buffer) {
    (*b).used = 0 as libc::c_int as uint32_t;
}
static mut fdlog_files: fdlog_files_t = fdlog_files_t {
    ptr: 0 as *const *mut fdlog_st as *mut *mut fdlog_st,
    used: 0,
};
static mut fdlog_pipes: fdlog_pipes_t = fdlog_pipes_t {
    ptr: 0 as *const fdlog_pipe as *mut fdlog_pipe,
    used: 0,
};
unsafe extern "C" fn fdlog_pipe_spawn(
    fn_0: *const libc::c_char,
    rfd: libc::c_int,
) -> pid_t {
    let mut devnull: libc::c_int = fdevent_open_devnull();
    if -(1 as libc::c_int) == devnull {
        return -(1 as libc::c_int);
    }
    let pid: pid_t = fdevent_sh_exec(
        fn_0,
        0 as *mut *mut libc::c_char,
        rfd,
        devnull,
        devnull,
    );
    if pid > 0 as libc::c_int {
        close(devnull);
    } else {
        let mut errnum: libc::c_int = *__errno_location();
        close(devnull);
        *__errno_location() = errnum;
    }
    return pid;
}
#[inline(never)]
unsafe extern "C" fn fdlog_pipe_restart(
    fdp: *mut fdlog_pipe,
    ts: unix_time64_t,
) -> libc::c_int {
    if ((*fdp).start + 5 as libc::c_int as libc::c_long) < ts {
        (*fdp).start = ts;
        (*fdp).pid = fdlog_pipe_spawn((*(*fdp).fdlog).fn_0, (*fdp).fd);
    }
    return if (*fdp).pid > 0 as libc::c_int {
        1 as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn fdlog_pipes_restart(ts: unix_time64_t) {
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < fdlog_pipes.used {
        let fdp: *mut fdlog_pipe = (fdlog_pipes.ptr).offset(i as isize);
        if !((*fdp).pid > 0 as libc::c_int) {
            fdlog_pipe_restart(fdp, ts);
        }
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn fdlog_pipes_waitpid_cb(pid: pid_t) -> libc::c_int {
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < fdlog_pipes.used {
        let fdp: *mut fdlog_pipe = (fdlog_pipes.ptr).offset(i as isize);
        if (*fdp).pid != pid {
            i = i.wrapping_add(1);
            i;
        } else {
            (*fdp).pid = -(1 as libc::c_int);
            return fdlog_pipe_restart(fdp, log_monotonic_secs);
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn fdlog_pipes_close(retain: *mut fdlog_st) {
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < fdlog_pipes.used {
        let fdp: *mut fdlog_pipe = (fdlog_pipes.ptr).offset(i as isize);
        let fdlog: *mut fdlog_st = (*fdp).fdlog;
        close((*fdp).fd);
        (*fdp).fd = -(1 as libc::c_int);
        if !(fdlog == retain) {
            fdlog_free(fdlog);
        }
        i = i.wrapping_add(1);
        i;
    }
    free(fdlog_pipes.ptr as *mut libc::c_void);
    fdlog_pipes.ptr = 0 as *mut fdlog_pipe;
    fdlog_pipes.used = 0 as libc::c_int as uint32_t;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn fdlog_pipes_abandon_pids() {
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < fdlog_pipes.used {
        let fdp: *mut fdlog_pipe = (fdlog_pipes.ptr).offset(i as isize);
        (*fdp).pid = -(1 as libc::c_int);
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn fdlog_pipe_serrh(fd: libc::c_int) {
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < fdlog_pipes.used {
        let fdlog: *mut fdlog_st = (*(fdlog_pipes.ptr).offset(i as isize)).fdlog;
        if (*fdlog).fd != fd {
            i = i.wrapping_add(1);
            i;
        } else {
            (*fdlog).fd = 2 as libc::c_int;
            break;
        }
    }
}
unsafe extern "C" fn fdlog_pipe_init(
    fn_0: *const libc::c_char,
    mut fds: *const libc::c_int,
    pid: pid_t,
) -> *mut fdlog_st {
    if fdlog_pipes.used & (4 as libc::c_int - 1 as libc::c_int) as libc::c_uint == 0 {
        ck_realloc_u32(
            &mut fdlog_pipes.ptr as *mut *mut fdlog_pipe as *mut *mut libc::c_void,
            fdlog_pipes.used as size_t,
            4 as libc::c_int as size_t,
            ::core::mem::size_of::<fdlog_pipe>() as libc::c_ulong,
        );
    }
    let fresh0 = fdlog_pipes.used;
    fdlog_pipes.used = (fdlog_pipes.used).wrapping_add(1);
    let fdp: *mut fdlog_pipe = (fdlog_pipes.ptr).offset(fresh0 as isize);
    (*fdp).fd = *fds.offset(0 as libc::c_int as isize);
    (*fdp).pid = pid;
    (*fdp).start = log_monotonic_secs;
    (*fdp)
        .fdlog = fdlog_init(
        fn_0,
        *fds.offset(1 as libc::c_int as isize),
        FDLOG_PIPE as libc::c_int,
    );
    return (*fdp).fdlog;
}
unsafe extern "C" fn fdlog_pipe_open(fn_0: *const libc::c_char) -> *mut fdlog_st {
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < fdlog_pipes.used {
        let fdlog: *mut fdlog_st = (*(fdlog_pipes.ptr).offset(i as isize)).fdlog;
        if 0 as libc::c_int != strcmp((*fdlog).fn_0, fn_0) {
            i = i.wrapping_add(1);
            i;
        } else {
            return fdlog
        }
    }
    let mut fds: [libc::c_int; 2] = [0; 2];
    if fdevent_pipe_cloexec(fds.as_mut_ptr(), 65536 as libc::c_int as libc::c_uint) != 0
    {
        return 0 as *mut fdlog_st;
    }
    let mut pid: pid_t = fdlog_pipe_spawn(fn_0, fds[0 as libc::c_int as usize]);
    if pid > 0 as libc::c_int {
        0 as libc::c_int != fdevent_fcntl_set_nb(fds[1 as libc::c_int as usize]);
        return fdlog_pipe_init(fn_0, fds.as_mut_ptr() as *const libc::c_int, pid);
    } else {
        let mut errnum: libc::c_int = *__errno_location();
        close(fds[0 as libc::c_int as usize]);
        close(fds[1 as libc::c_int as usize]);
        *__errno_location() = errnum;
        return 0 as *mut fdlog_st;
    };
}
unsafe extern "C" fn fdlog_file_init(
    fn_0: *const libc::c_char,
    fd: libc::c_int,
) -> *mut fdlog_st {
    if fdlog_files.used & (4 as libc::c_int - 1 as libc::c_int) as libc::c_uint == 0 {
        ck_realloc_u32(
            &mut fdlog_files.ptr as *mut *mut *mut fdlog_st as *mut *mut libc::c_void,
            fdlog_files.used as size_t,
            4 as libc::c_int as size_t,
            ::core::mem::size_of::<*mut fdlog_st>() as libc::c_ulong,
        );
    }
    let fresh1 = fdlog_files.used;
    fdlog_files.used = (fdlog_files.used).wrapping_add(1);
    let ref mut fresh2 = *(fdlog_files.ptr).offset(fresh1 as isize);
    *fresh2 = fdlog_init(fn_0, fd, FDLOG_FILE as libc::c_int);
    return *fresh2;
}
unsafe extern "C" fn fdlog_file_open_fd(fn_0: *const libc::c_char) -> libc::c_int {
    let mut flags: libc::c_int = 0o2000 as libc::c_int | 0o1 as libc::c_int
        | 0o100 as libc::c_int;
    return fdevent_open_cloexec(
        fn_0,
        1 as libc::c_int,
        flags,
        0o644 as libc::c_int as mode_t,
    );
}
unsafe extern "C" fn fdlog_file_open(fn_0: *const libc::c_char) -> *mut fdlog_st {
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < fdlog_files.used {
        let fdlog: *mut fdlog_st = *(fdlog_files.ptr).offset(i as isize);
        if 0 as libc::c_int != strcmp((*fdlog).fn_0, fn_0) {
            i = i.wrapping_add(1);
            i;
        } else {
            return fdlog
        }
    }
    let mut fd: libc::c_int = fdlog_file_open_fd(fn_0);
    return if -(1 as libc::c_int) != fd {
        fdlog_file_init(fn_0, fd)
    } else {
        0 as *mut fdlog_st
    };
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn fdlog_open(fn_0: *const libc::c_char) -> *mut fdlog_st {
    return if *fn_0.offset(0 as libc::c_int as isize) as libc::c_int != '|' as i32 {
        fdlog_file_open(fn_0)
    } else {
        fdlog_pipe_open(fn_0.offset(1 as libc::c_int as isize))
    };
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn fdlog_files_flush(errh: *mut fdlog_st, memrel: libc::c_int) {
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < fdlog_files.used {
        let fdlog: *mut fdlog_st = *(fdlog_files.ptr).offset(i as isize);
        let b: *mut buffer = &mut (*fdlog).b;
        if buffer_is_blank(b) == 0 {
            let wr: ssize_t = write_all(
                (*fdlog).fd,
                (*b).ptr as *const libc::c_void,
                buffer_clen(b) as size_t,
            );
            buffer_clear(b);
            if -(1 as libc::c_int) as libc::c_long == wr {
                log_perror(
                    errh,
                    b"fdlog_maint.c\0" as *const u8 as *const libc::c_char,
                    255 as libc::c_int as libc::c_uint,
                    b"error flushing log %s\0" as *const u8 as *const libc::c_char,
                    (*fdlog).fn_0,
                );
            }
        }
        if memrel != 0 && !((*b).ptr).is_null() {
            buffer_free_ptr(b);
        }
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn fdlog_files_cycle(errh: *mut fdlog_st) {
    fdlog_files_flush(errh, 0 as libc::c_int);
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < fdlog_files.used {
        let fdlog: *mut fdlog_st = *(fdlog_files.ptr).offset(i as isize);
        let mut fd: libc::c_int = fdlog_file_open_fd((*fdlog).fn_0);
        if -(1 as libc::c_int) != fd {
            if (*fdlog).fd > 2 as libc::c_int {
                close((*fdlog).fd);
                (*fdlog).fd = fd;
            } else {
                if (*fdlog).fd != dup2(fd, (*fdlog).fd) {
                    log_perror(
                        errh,
                        b"fdlog_maint.c\0" as *const u8 as *const libc::c_char,
                        277 as libc::c_int as libc::c_uint,
                        b"dup2() %s to %d\0" as *const u8 as *const libc::c_char,
                        (*fdlog).fn_0,
                        (*fdlog).fd,
                    );
                }
                close(fd);
            }
        } else {
            log_perror(
                errh,
                b"fdlog_maint.c\0" as *const u8 as *const libc::c_char,
                287 as libc::c_int as libc::c_uint,
                b"error cycling log %s\0" as *const u8 as *const libc::c_char,
                (*fdlog).fn_0,
            );
        }
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn fdlog_files_close(retain: *mut fdlog_st) {
    fdlog_files_flush(retain, 0 as libc::c_int);
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < fdlog_files.used {
        let fdlog: *mut fdlog_st = *(fdlog_files.ptr).offset(i as isize);
        if !(fdlog == retain) {
            fdlog_free(fdlog);
        }
        i = i.wrapping_add(1);
        i;
    }
    free(fdlog_files.ptr as *mut libc::c_void);
    fdlog_files.ptr = 0 as *mut *mut fdlog_st;
    fdlog_files.used = 0 as libc::c_int as uint32_t;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn fdlog_closeall(errh: *mut fdlog_st) {
    fdlog_files_close(errh);
    fdlog_pipes_close(errh);
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn fdlog_flushall(errh: *mut fdlog_st) {
    fdlog_files_flush(errh, 1 as libc::c_int);
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < fdlog_pipes.used {
        let b: *mut buffer = &mut (*(*(fdlog_pipes.ptr).offset(i as isize)).fdlog).b;
        if !((*b).ptr).is_null() {
            buffer_free_ptr(b);
        }
        i = i.wrapping_add(1);
        i;
    }
    if !((*errh).b.ptr).is_null() {
        buffer_free_ptr(&mut (*errh).b);
    }
}
static mut fdlog_syslogging: libc::c_int = 0;
#[no_mangle]
#[cold]
pub unsafe extern "C" fn fdlog_closelog() {
    if fdlog_syslogging != 0 {
        fdlog_syslogging = 0 as libc::c_int;
        closelog();
    }
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn fdlog_openlog(
    errh: *mut fdlog_st,
    syslog_facility: *const buffer,
) {
    if fdlog_syslogging != 0 {
        return;
    }
    fdlog_syslogging = 1 as libc::c_int;
    let mut facility: libc::c_int = -(1 as libc::c_int);
    if !syslog_facility.is_null() {
        static mut facility_names: [facility_name_st; 21] = [
            {
                let mut init = facility_name_st {
                    name: b"auth\0" as *const u8 as *const libc::c_char,
                    val: (4 as libc::c_int) << 3 as libc::c_int,
                };
                init
            },
            {
                let mut init = facility_name_st {
                    name: b"authpriv\0" as *const u8 as *const libc::c_char,
                    val: (10 as libc::c_int) << 3 as libc::c_int,
                };
                init
            },
            {
                let mut init = facility_name_st {
                    name: b"cron\0" as *const u8 as *const libc::c_char,
                    val: (9 as libc::c_int) << 3 as libc::c_int,
                };
                init
            },
            {
                let mut init = facility_name_st {
                    name: b"daemon\0" as *const u8 as *const libc::c_char,
                    val: (3 as libc::c_int) << 3 as libc::c_int,
                };
                init
            },
            {
                let mut init = facility_name_st {
                    name: b"ftp\0" as *const u8 as *const libc::c_char,
                    val: (11 as libc::c_int) << 3 as libc::c_int,
                };
                init
            },
            {
                let mut init = facility_name_st {
                    name: b"kern\0" as *const u8 as *const libc::c_char,
                    val: (0 as libc::c_int) << 3 as libc::c_int,
                };
                init
            },
            {
                let mut init = facility_name_st {
                    name: b"lpr\0" as *const u8 as *const libc::c_char,
                    val: (6 as libc::c_int) << 3 as libc::c_int,
                };
                init
            },
            {
                let mut init = facility_name_st {
                    name: b"mail\0" as *const u8 as *const libc::c_char,
                    val: (2 as libc::c_int) << 3 as libc::c_int,
                };
                init
            },
            {
                let mut init = facility_name_st {
                    name: b"news\0" as *const u8 as *const libc::c_char,
                    val: (7 as libc::c_int) << 3 as libc::c_int,
                };
                init
            },
            {
                let mut init = facility_name_st {
                    name: b"security\0" as *const u8 as *const libc::c_char,
                    val: (4 as libc::c_int) << 3 as libc::c_int,
                };
                init
            },
            {
                let mut init = facility_name_st {
                    name: b"syslog\0" as *const u8 as *const libc::c_char,
                    val: (5 as libc::c_int) << 3 as libc::c_int,
                };
                init
            },
            {
                let mut init = facility_name_st {
                    name: b"user\0" as *const u8 as *const libc::c_char,
                    val: (1 as libc::c_int) << 3 as libc::c_int,
                };
                init
            },
            {
                let mut init = facility_name_st {
                    name: b"uucp\0" as *const u8 as *const libc::c_char,
                    val: (8 as libc::c_int) << 3 as libc::c_int,
                };
                init
            },
            {
                let mut init = facility_name_st {
                    name: b"local0\0" as *const u8 as *const libc::c_char,
                    val: (16 as libc::c_int) << 3 as libc::c_int,
                };
                init
            },
            {
                let mut init = facility_name_st {
                    name: b"local1\0" as *const u8 as *const libc::c_char,
                    val: (17 as libc::c_int) << 3 as libc::c_int,
                };
                init
            },
            {
                let mut init = facility_name_st {
                    name: b"local2\0" as *const u8 as *const libc::c_char,
                    val: (18 as libc::c_int) << 3 as libc::c_int,
                };
                init
            },
            {
                let mut init = facility_name_st {
                    name: b"local3\0" as *const u8 as *const libc::c_char,
                    val: (19 as libc::c_int) << 3 as libc::c_int,
                };
                init
            },
            {
                let mut init = facility_name_st {
                    name: b"local4\0" as *const u8 as *const libc::c_char,
                    val: (20 as libc::c_int) << 3 as libc::c_int,
                };
                init
            },
            {
                let mut init = facility_name_st {
                    name: b"local5\0" as *const u8 as *const libc::c_char,
                    val: (21 as libc::c_int) << 3 as libc::c_int,
                };
                init
            },
            {
                let mut init = facility_name_st {
                    name: b"local6\0" as *const u8 as *const libc::c_char,
                    val: (22 as libc::c_int) << 3 as libc::c_int,
                };
                init
            },
            {
                let mut init = facility_name_st {
                    name: b"local7\0" as *const u8 as *const libc::c_char,
                    val: (23 as libc::c_int) << 3 as libc::c_int,
                };
                init
            },
        ];
        let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        while (i as libc::c_ulong)
            < (::core::mem::size_of::<[facility_name_st; 21]>() as libc::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<facility_name_st>() as libc::c_ulong,
                )
        {
            let mut f: *const facility_name_st = facility_names
                .as_ptr()
                .offset(i as isize);
            if 0 as libc::c_int == strcmp((*syslog_facility).ptr, (*f).name) {
                facility = (*f).val;
                break;
            } else {
                i = i.wrapping_add(1);
                i;
            }
        }
        if -(1 as libc::c_int) == facility {
            log_pri(
                errh,
                b"fdlog_maint.c\0" as *const u8 as *const libc::c_char,
                410 as libc::c_int as libc::c_uint,
                4 as libc::c_int,
                b"unrecognized syslog facility: \"%s\"; defaulting to \"daemon\" facility\0"
                    as *const u8 as *const libc::c_char,
                (*syslog_facility).ptr,
            );
        }
    }
    openlog(
        b"lighttpd\0" as *const u8 as *const libc::c_char,
        0x2 as libc::c_int | 0x1 as libc::c_int,
        if -(1 as libc::c_int) == facility {
            (3 as libc::c_int) << 3 as libc::c_int
        } else {
            facility
        },
    );
}
