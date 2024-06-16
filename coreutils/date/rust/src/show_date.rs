use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type tm_zone;
    static mut stdout: *mut FILE;
    fn localtime_rz(
        __tz: timezone_t,
        __timer: *const time_t,
        __result: *mut tm,
    ) -> *mut tm;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn umaxtostr(_: uintmax_t, _: *mut libc::c_char) -> *mut libc::c_char;
    fn imaxtostr(_: intmax_t, _: *mut libc::c_char) -> *mut libc::c_char;
    fn gettext(__msgid: *const libc::c_char) -> *mut libc::c_char;
    fn fprintftime(
        fp: *mut FILE,
        fmt: *const libc::c_char,
        tm: *const tm,
        zone: timezone_t,
        nanoseconds: libc::c_int,
    ) -> size_t;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type __intmax_t = libc::c_long;
pub type __uintmax_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
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
pub type FILE = _IO_FILE;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
pub type timezone_t = *mut tm_zone;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
#[inline]
unsafe extern "C" fn timetostr(
    mut t: time_t,
    mut buf: *mut libc::c_char,
) -> *mut libc::c_char {
    return if !((0 as libc::c_int as time_t) < -(1 as libc::c_int) as time_t) {
        imaxtostr(t, buf)
    } else {
        umaxtostr(t as uintmax_t, buf)
    };
}
#[no_mangle]
pub unsafe extern "C" fn show_date(
    mut format: *const libc::c_char,
    mut when: timespec,
    mut tz: timezone_t,
) -> bool {
    let mut tm: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const libc::c_char,
    };
    if !(localtime_rz(tz, &mut when.tv_sec, &mut tm)).is_null() {
        fprintftime(stdout, format, &mut tm, tz, when.tv_nsec as libc::c_int);
        return 1 as libc::c_int != 0;
    } else {
        let mut buf: [libc::c_char; 21] = [0; 21];
        if 0 != 0 {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"time %s is out of range\0" as *const u8 as *const libc::c_char,
                ),
                quote(timetostr(when.tv_sec, buf.as_mut_ptr())),
            );
            if 0 as libc::c_int != 0 as libc::c_int {
                unreachable!();
            } else {};
        } else {
            ({
                let __errstatus: libc::c_int = 0 as libc::c_int;
                error(
                    __errstatus,
                    0 as libc::c_int,
                    gettext(
                        b"time %s is out of range\0" as *const u8 as *const libc::c_char,
                    ),
                    quote(timetostr(when.tv_sec, buf.as_mut_ptr())),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
            ({
                let __errstatus: libc::c_int = 0 as libc::c_int;
                error(
                    __errstatus,
                    0 as libc::c_int,
                    gettext(
                        b"time %s is out of range\0" as *const u8 as *const libc::c_char,
                    ),
                    quote(timetostr(when.tv_sec, buf.as_mut_ptr())),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
        return 0 as libc::c_int != 0;
    };
}
