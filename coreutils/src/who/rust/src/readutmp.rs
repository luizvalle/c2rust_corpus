use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
    fn timespec_get(__ts: *mut timespec, __base: libc::c_int) -> libc::c_int;
    fn setutxent();
    fn endutxent();
    fn getutxent() -> *mut utmpx;
    fn utmpxname(__file: *const libc::c_char) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn fread_unlocked(
        __ptr: *mut libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn rpl_fclose(stream: *mut FILE) -> libc::c_int;
    fn rpl_fopen(filename: *const libc::c_char, mode: *const libc::c_char) -> *mut FILE;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strnlen(__string: *const libc::c_char, __maxlen: size_t) -> size_t;
    fn sysinfo(__info: *mut sysinfo) -> libc::c_int;
    fn xpalloc(
        pa: *mut libc::c_void,
        pn: *mut idx_t,
        n_incr_min: idx_t,
        n_max: ptrdiff_t,
        s: idx_t,
    ) -> *mut libc::c_void;
    fn ximemdup0(p: *const libc::c_void, s: idx_t) -> *mut libc::c_char;
}
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __blksize_t = libc::c_int;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type intptr_t = libc::c_long;
pub type idx_t = ptrdiff_t;
pub type pid_t = __pid_t;
pub type clockid_t = __clockid_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __exit_status {
    pub e_termination: libc::c_short,
    pub e_exit: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utmpx {
    pub ut_type: libc::c_short,
    pub ut_pid: __pid_t,
    pub ut_line: [libc::c_char; 32],
    pub ut_id: [libc::c_char; 4],
    pub ut_user: [libc::c_char; 32],
    pub ut_host: [libc::c_char; 256],
    pub ut_exit: __exit_status,
    pub ut_session: libc::c_long,
    pub ut_tv: timeval,
    pub ut_addr_v6: [__int32_t; 4],
    pub __glibc_reserved: [libc::c_char; 20],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gl_utmp {
    pub ut_user: *mut libc::c_char,
    pub ut_id: *mut libc::c_char,
    pub ut_line: *mut libc::c_char,
    pub ut_host: *mut libc::c_char,
    pub ut_ts: timespec,
    pub ut_pid: pid_t,
    pub ut_session: pid_t,
    pub ut_type: libc::c_short,
    pub ut_exit: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub e_termination: libc::c_int,
    pub e_exit: libc::c_int,
}
pub type STRUCT_UTMP = gl_utmp;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const READ_UTMP_NO_BOOT_TIME: C2RustUnnamed_0 = 8;
pub const READ_UTMP_BOOT_TIME: C2RustUnnamed_0 = 4;
pub const READ_UTMP_USER_PROCESS: C2RustUnnamed_0 = 2;
pub const READ_UTMP_CHECK_PIDS: C2RustUnnamed_0 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utmp_alloc {
    pub utmp: *mut gl_utmp,
    pub filled: idx_t,
    pub string_bytes: idx_t,
    pub alloc_bytes: idx_t,
}
pub type __kernel_long_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sysinfo {
    pub uptime: __kernel_long_t,
    pub loads: [__kernel_ulong_t; 3],
    pub totalram: __kernel_ulong_t,
    pub freeram: __kernel_ulong_t,
    pub sharedram: __kernel_ulong_t,
    pub bufferram: __kernel_ulong_t,
    pub totalswap: __kernel_ulong_t,
    pub freeswap: __kernel_ulong_t,
    pub procs: __u16,
    pub pad: __u16,
    pub totalhigh: __kernel_ulong_t,
    pub freehigh: __kernel_ulong_t,
    pub mem_unit: __u32,
    pub _f: [libc::c_char; 0],
}
pub type __u32 = libc::c_uint;
pub type __kernel_ulong_t = libc::c_ulong;
pub type __u16 = libc::c_ushort;
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_mode: __mode_t,
    pub st_nlink: __nlink_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub st_rdev: __dev_t,
    pub __pad1: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub __pad2: libc::c_int,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [libc::c_int; 2],
}
#[inline]
unsafe extern "C" fn get_stat_mtime(mut st: *const stat) -> timespec {
    return (*st).st_mtim;
}
unsafe extern "C" fn get_linux_uptime(mut p_uptime: *mut timespec) -> libc::c_int {
    if clock_gettime(7 as libc::c_int, p_uptime) >= 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    let mut fp: *mut FILE = rpl_fopen(
        b"/proc/uptime\0" as *const u8 as *const libc::c_char,
        b"re\0" as *const u8 as *const libc::c_char,
    );
    if !fp.is_null() {
        let mut buf: [libc::c_char; 33] = [0; 33];
        let mut n: size_t = fread_unlocked(
            buf.as_mut_ptr() as *mut libc::c_void,
            1 as libc::c_int as size_t,
            (::core::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            fp,
        );
        rpl_fclose(fp);
        if n > 0 as libc::c_int as libc::c_ulong {
            buf[n as usize] = '\0' as i32 as libc::c_char;
            let mut s: time_t = 0 as libc::c_int as time_t;
            let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
            p = buf.as_mut_ptr();
            while '0' as i32 <= *p as libc::c_int && *p as libc::c_int <= '9' as i32 {
                s = 10 as libc::c_int as libc::c_long * s
                    + (*p as libc::c_int - '0' as i32) as libc::c_long;
                p = p.offset(1);
                p;
            }
            if buf.as_mut_ptr() < p {
                let mut ns: libc::c_long = 0 as libc::c_int as libc::c_long;
                let fresh0 = p;
                p = p.offset(1);
                if *fresh0 as libc::c_int == '.' as i32 {
                    let mut i: libc::c_int = 0 as libc::c_int;
                    while i < 9 as libc::c_int {
                        ns = 10 as libc::c_int as libc::c_long * ns
                            + (if '0' as i32 <= *p as libc::c_int
                                && *p as libc::c_int <= '9' as i32
                            {
                                let fresh1 = p;
                                p = p.offset(1);
                                *fresh1 as libc::c_int - '0' as i32
                            } else {
                                0 as libc::c_int
                            }) as libc::c_long;
                        i += 1;
                        i;
                    }
                }
                (*p_uptime).tv_sec = s;
                (*p_uptime).tv_nsec = ns;
                return 0 as libc::c_int;
            }
        }
    }
    let mut info: sysinfo = sysinfo {
        uptime: 0,
        loads: [0; 3],
        totalram: 0,
        freeram: 0,
        sharedram: 0,
        bufferram: 0,
        totalswap: 0,
        freeswap: 0,
        procs: 0,
        pad: 0,
        totalhigh: 0,
        freehigh: 0,
        mem_unit: 0,
        _f: [0; 0],
    };
    if sysinfo(&mut info) >= 0 as libc::c_int {
        (*p_uptime).tv_sec = info.uptime;
        (*p_uptime).tv_nsec = 0 as libc::c_int as __syscall_slong_t;
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn get_linux_boot_time_fallback(
    mut p_boot_time: *mut timespec,
) -> libc::c_int {
    let boot_touched_files: [*const libc::c_char; 4] = [
        b"/var/lib/systemd/random-seed\0" as *const u8 as *const libc::c_char,
        b"/var/lib/urandom/random-seed\0" as *const u8 as *const libc::c_char,
        b"/var/lib/random-seed\0" as *const u8 as *const libc::c_char,
        b"/var/run/utmp\0" as *const u8 as *const libc::c_char,
    ];
    let mut i: idx_t = 0 as libc::c_int as idx_t;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[*const libc::c_char; 4]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
    {
        let mut filename: *const libc::c_char = boot_touched_files[i as usize];
        let mut statbuf: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_mode: 0,
            st_nlink: 0,
            st_uid: 0,
            st_gid: 0,
            st_rdev: 0,
            __pad1: 0,
            st_size: 0,
            st_blksize: 0,
            __pad2: 0,
            st_blocks: 0,
            st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
            __glibc_reserved: [0; 2],
        };
        if stat(filename, &mut statbuf) >= 0 as libc::c_int {
            *p_boot_time = get_stat_mtime(&mut statbuf);
            return 0 as libc::c_int;
        }
        i += 1;
        i;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn get_linux_boot_time_final_fallback(
    mut p_boot_time: *mut timespec,
) -> libc::c_int {
    let mut uptime: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    if get_linux_uptime(&mut uptime) >= 0 as libc::c_int {
        let mut result: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
        if timespec_get(&mut result, 1 as libc::c_int) == 0 {
            return -(1 as libc::c_int);
        }
        if result.tv_nsec < uptime.tv_nsec {
            result.tv_nsec += 1000000000 as libc::c_int as libc::c_long;
            result.tv_sec -= 1 as libc::c_int as libc::c_long;
        }
        result.tv_sec -= uptime.tv_sec;
        result.tv_nsec -= uptime.tv_nsec;
        *p_boot_time = result;
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn extract_trimmed_name(
    mut ut: *const STRUCT_UTMP,
) -> *mut libc::c_char {
    let mut name: *const libc::c_char = (*ut).ut_user;
    let mut len: idx_t = strlen(name) as idx_t;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    p = name.offset(len as isize);
    while name < p
        && *p.offset(-(1 as libc::c_int) as isize) as libc::c_int == ' ' as i32
    {
        p = p.offset(-1);
        p;
    }
    return ximemdup0(name as *const libc::c_void, p.offset_from(name) as libc::c_long);
}
unsafe extern "C" fn desirable_utmp_entry(
    mut ut: *const STRUCT_UTMP,
    mut options: libc::c_int,
) -> bool {
    let mut boot_time: bool = (*ut).ut_type as libc::c_int == 2 as libc::c_int;
    if options & READ_UTMP_BOOT_TIME as libc::c_int != 0 && !boot_time {
        return 0 as libc::c_int != 0;
    }
    if options & READ_UTMP_NO_BOOT_TIME as libc::c_int != 0
        && boot_time as libc::c_int != 0
    {
        return 0 as libc::c_int != 0;
    }
    let mut user_proc: bool = *((*ut).ut_user).offset(0 as libc::c_int as isize)
        as libc::c_int != 0 && (*ut).ut_type as libc::c_int == 7 as libc::c_int;
    if options & READ_UTMP_USER_PROCESS as libc::c_int != 0 && !user_proc {
        return 0 as libc::c_int != 0;
    }
    if options & READ_UTMP_CHECK_PIDS as libc::c_int != 0
        && user_proc as libc::c_int != 0 && (0 as libc::c_int) < (*ut).ut_pid
        && (kill((*ut).ut_pid, 0 as libc::c_int) < 0 as libc::c_int
            && *__errno_location() == 3 as libc::c_int)
    {
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn add_utmp(
    mut a: utmp_alloc,
    mut options: libc::c_int,
    mut user: *const libc::c_char,
    mut user_len: idx_t,
    mut id: *const libc::c_char,
    mut id_len: idx_t,
    mut line: *const libc::c_char,
    mut line_len: idx_t,
    mut host: *const libc::c_char,
    mut host_len: idx_t,
    mut pid: pid_t,
    mut type_0: libc::c_short,
    mut ts: timespec,
    mut session: libc::c_long,
    mut termination: libc::c_int,
    mut exit: libc::c_int,
) -> utmp_alloc {
    let mut entry_bytes: libc::c_int = ::core::mem::size_of::<gl_utmp>() as libc::c_ulong
        as libc::c_int;
    let mut avail: idx_t = a.alloc_bytes
        - (entry_bytes as libc::c_long * a.filled + a.string_bytes);
    let mut needed_string_bytes: idx_t = user_len + 1 as libc::c_int as libc::c_long
        + (id_len + 1 as libc::c_int as libc::c_long)
        + (line_len + 1 as libc::c_int as libc::c_long)
        + (host_len + 1 as libc::c_int as libc::c_long);
    let mut needed: idx_t = entry_bytes as libc::c_long + needed_string_bytes;
    if avail < needed {
        let mut old_string_offset: idx_t = a.alloc_bytes - a.string_bytes;
        let mut new: *mut libc::c_void = xpalloc(
            a.utmp as *mut libc::c_void,
            &mut a.alloc_bytes,
            needed - avail,
            -(1 as libc::c_int) as ptrdiff_t,
            1 as libc::c_int as idx_t,
        );
        let mut new_string_offset: idx_t = a.alloc_bytes - a.string_bytes;
        a.utmp = new as *mut gl_utmp;
        let mut q: *mut libc::c_char = new as *mut libc::c_char;
        memmove(
            q.offset(new_string_offset as isize) as *mut libc::c_void,
            q.offset(old_string_offset as isize) as *const libc::c_void,
            a.string_bytes as libc::c_ulong,
        );
    }
    let mut ut: *mut gl_utmp = &mut *(a.utmp).offset(a.filled as isize) as *mut gl_utmp;
    let mut stringlim: *mut libc::c_char = (a.utmp as *mut libc::c_char)
        .offset(a.alloc_bytes as isize);
    let mut p: *mut libc::c_char = stringlim.offset(-(a.string_bytes as isize));
    p = p.offset(-1);
    *p = '\0' as i32 as libc::c_char;
    p = memcpy(
        p.offset(-(user_len as isize)) as *mut libc::c_void,
        user as *const libc::c_void,
        user_len as libc::c_ulong,
    ) as *mut libc::c_char;
    (*ut).ut_user = p;
    p = p.offset(-1);
    *p = '\0' as i32 as libc::c_char;
    p = memcpy(
        p.offset(-(id_len as isize)) as *mut libc::c_void,
        id as *const libc::c_void,
        id_len as libc::c_ulong,
    ) as *mut libc::c_char;
    (*ut).ut_id = p;
    p = p.offset(-1);
    *p = '\0' as i32 as libc::c_char;
    p = memcpy(
        p.offset(-(line_len as isize)) as *mut libc::c_void,
        line as *const libc::c_void,
        line_len as libc::c_ulong,
    ) as *mut libc::c_char;
    (*ut).ut_line = p;
    p = p.offset(-1);
    *p = '\0' as i32 as libc::c_char;
    (*ut)
        .ut_host = memcpy(
        p.offset(-(host_len as isize)) as *mut libc::c_void,
        host as *const libc::c_void,
        host_len as libc::c_ulong,
    ) as *mut libc::c_char;
    (*ut).ut_ts = ts;
    (*ut).ut_pid = pid;
    (*ut).ut_session = session as pid_t;
    (*ut).ut_type = type_0;
    (*ut).ut_exit.e_termination = termination;
    (*ut).ut_exit.e_exit = exit;
    if desirable_utmp_entry(ut, options) {
        (*ut)
            .ut_user = ((*ut).ut_user).offset_from(stringlim) as libc::c_long
            as *mut libc::c_char;
        (*ut)
            .ut_id = ((*ut).ut_id).offset_from(stringlim) as libc::c_long
            as *mut libc::c_char;
        (*ut)
            .ut_line = ((*ut).ut_line).offset_from(stringlim) as libc::c_long
            as *mut libc::c_char;
        (*ut)
            .ut_host = ((*ut).ut_host).offset_from(stringlim) as libc::c_long
            as *mut libc::c_char;
        a.filled += 1;
        a.filled;
        a.string_bytes += needed_string_bytes;
    }
    return a;
}
unsafe extern "C" fn finish_utmp(mut a: utmp_alloc) -> utmp_alloc {
    let mut stringlim: *mut libc::c_char = (a.utmp as *mut libc::c_char)
        .offset(a.alloc_bytes as isize);
    let mut i: idx_t = 0 as libc::c_int as idx_t;
    while i < a.filled {
        let ref mut fresh2 = (*(a.utmp).offset(i as isize)).ut_user;
        *fresh2 = stringlim
            .offset((*(a.utmp).offset(i as isize)).ut_user as intptr_t as isize);
        let ref mut fresh3 = (*(a.utmp).offset(i as isize)).ut_id;
        *fresh3 = stringlim
            .offset((*(a.utmp).offset(i as isize)).ut_id as intptr_t as isize);
        let ref mut fresh4 = (*(a.utmp).offset(i as isize)).ut_line;
        *fresh4 = stringlim
            .offset((*(a.utmp).offset(i as isize)).ut_line as intptr_t as isize);
        let ref mut fresh5 = (*(a.utmp).offset(i as isize)).ut_host;
        *fresh5 = stringlim
            .offset((*(a.utmp).offset(i as isize)).ut_host as intptr_t as isize);
        i += 1;
        i;
    }
    return a;
}
unsafe extern "C" fn have_boot_time(mut a: utmp_alloc) -> bool {
    let mut i: idx_t = 0 as libc::c_int as idx_t;
    while i < a.filled {
        let mut ut: *mut gl_utmp = &mut *(a.utmp).offset(i as isize) as *mut gl_utmp;
        if (*ut).ut_type as libc::c_int == 2 as libc::c_int {
            return 1 as libc::c_int != 0;
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn read_utmp_from_file(
    mut file: *const libc::c_char,
    mut n_entries: *mut idx_t,
    mut utmp_buf: *mut *mut STRUCT_UTMP,
    mut options: libc::c_int,
) -> libc::c_int {
    if options & READ_UTMP_BOOT_TIME as libc::c_int != 0 as libc::c_int
        && options
            & (READ_UTMP_USER_PROCESS as libc::c_int
                | READ_UTMP_NO_BOOT_TIME as libc::c_int) != 0 as libc::c_int
    {
        *n_entries = 0 as libc::c_int as idx_t;
        *utmp_buf = 0 as *mut STRUCT_UTMP;
        return 0 as libc::c_int;
    }
    let mut a: utmp_alloc = {
        let mut init = utmp_alloc {
            utmp: 0 as *mut gl_utmp,
            filled: 0,
            string_bytes: 0,
            alloc_bytes: 0,
        };
        init
    };
    utmpxname(file as *mut libc::c_char);
    setutxent();
    let mut file_is_utmp: bool = strcmp(
        file,
        b"/var/run/utmp\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int;
    let mut runlevel_ts: timespec = {
        let mut init = timespec {
            tv_sec: 0 as libc::c_int as __time_t,
            tv_nsec: 0,
        };
        init
    };
    let mut entry: *const libc::c_void = 0 as *const libc::c_void;
    loop {
        entry = getutxent() as *const libc::c_void;
        if entry.is_null() {
            break;
        }
        let mut ut: *const utmpx = entry as *const utmpx;
        let mut ts: timespec = {
            let mut init = timespec {
                tv_sec: (*ut).ut_tv.tv_sec,
                tv_nsec: (*ut).ut_tv.tv_usec * 1000 as libc::c_int as libc::c_long,
            };
            init
        };
        a = add_utmp(
            a,
            options,
            ((*ut).ut_user).as_ptr(),
            strnlen(
                ((*ut).ut_user).as_ptr(),
                ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            ) as idx_t,
            ((*ut).ut_id).as_ptr(),
            strnlen(
                ((*ut).ut_id).as_ptr(),
                ::core::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong,
            ) as idx_t,
            ((*ut).ut_line).as_ptr(),
            strnlen(
                ((*ut).ut_line).as_ptr(),
                ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            ) as idx_t,
            ((*ut).ut_host).as_ptr(),
            strnlen(
                ((*ut).ut_host).as_ptr(),
                ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
            ) as idx_t,
            (*ut).ut_pid,
            (*ut).ut_type,
            ts,
            (*ut).ut_session,
            (*ut).ut_exit.e_termination as libc::c_int,
            (*ut).ut_exit.e_exit as libc::c_int,
        );
        if file_is_utmp as libc::c_int != 0
            && memcmp(
                ((*ut).ut_user).as_ptr() as *const libc::c_void,
                b"runlevel\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                (strlen(b"runlevel\0" as *const u8 as *const libc::c_char))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) == 0 as libc::c_int
            && memcmp(
                ((*ut).ut_line).as_ptr() as *const libc::c_void,
                b"~\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                (strlen(b"~\0" as *const u8 as *const libc::c_char))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) == 0 as libc::c_int
        {
            runlevel_ts = ts;
        }
    }
    endutxent();
    if options
        & (READ_UTMP_USER_PROCESS as libc::c_int | READ_UTMP_NO_BOOT_TIME as libc::c_int)
        == 0 as libc::c_int && file_is_utmp as libc::c_int != 0
    {
        let mut i: idx_t = 0 as libc::c_int as idx_t;
        while i < a.filled {
            let mut ut_0: *mut gl_utmp = &mut *(a.utmp).offset(i as isize)
                as *mut gl_utmp;
            if (*ut_0).ut_type as libc::c_int == 2 as libc::c_int {
                if (*ut_0).ut_ts.tv_sec <= 60 as libc::c_int as libc::c_long
                    && runlevel_ts.tv_sec != 0 as libc::c_int as libc::c_long
                {
                    (*ut_0).ut_ts = runlevel_ts;
                }
                break;
            } else {
                i += 1;
                i;
            }
        }
        if !have_boot_time(a) {
            let mut boot_time: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
            if get_linux_boot_time_fallback(&mut boot_time) >= 0 as libc::c_int {
                a = add_utmp(
                    a,
                    options,
                    b"reboot\0" as *const u8 as *const libc::c_char,
                    strlen(b"reboot\0" as *const u8 as *const libc::c_char) as idx_t,
                    b"\0" as *const u8 as *const libc::c_char,
                    0 as libc::c_int as idx_t,
                    b"~\0" as *const u8 as *const libc::c_char,
                    strlen(b"~\0" as *const u8 as *const libc::c_char) as idx_t,
                    b"\0" as *const u8 as *const libc::c_char,
                    0 as libc::c_int as idx_t,
                    0 as libc::c_int,
                    2 as libc::c_int as libc::c_short,
                    boot_time,
                    0 as libc::c_int as libc::c_long,
                    0 as libc::c_int,
                    0 as libc::c_int,
                );
            }
        }
    }
    if options
        & (READ_UTMP_USER_PROCESS as libc::c_int | READ_UTMP_NO_BOOT_TIME as libc::c_int)
        == 0 as libc::c_int
        && strcmp(file, b"/var/run/utmp\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int && !have_boot_time(a)
    {
        let mut boot_time_0: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
        if get_linux_boot_time_final_fallback(&mut boot_time_0) >= 0 as libc::c_int {
            a = add_utmp(
                a,
                options,
                b"reboot\0" as *const u8 as *const libc::c_char,
                strlen(b"reboot\0" as *const u8 as *const libc::c_char) as idx_t,
                b"\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int as idx_t,
                b"~\0" as *const u8 as *const libc::c_char,
                strlen(b"~\0" as *const u8 as *const libc::c_char) as idx_t,
                b"\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int as idx_t,
                0 as libc::c_int,
                2 as libc::c_int as libc::c_short,
                boot_time_0,
                0 as libc::c_int as libc::c_long,
                0 as libc::c_int,
                0 as libc::c_int,
            );
        }
    }
    a = finish_utmp(a);
    *n_entries = a.filled;
    *utmp_buf = a.utmp;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn read_utmp(
    mut file: *const libc::c_char,
    mut n_entries: *mut idx_t,
    mut utmp_buf: *mut *mut STRUCT_UTMP,
    mut options: libc::c_int,
) -> libc::c_int {
    return read_utmp_from_file(file, n_entries, utmp_buf, options);
}
