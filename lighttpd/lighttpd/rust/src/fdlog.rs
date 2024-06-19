use ::libc;
extern "C" {
    fn ck_calloc(nmemb: size_t, elt_sz: size_t) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn close(__fd: libc::c_int) -> libc::c_int;
}
pub type __uint32_t = libc::c_uint;
pub type size_t = libc::c_ulong;
pub type uint32_t = __uint32_t;
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
#[no_mangle]
#[cold]
pub unsafe extern "C" fn fdlog_init(
    fn_0: *const libc::c_char,
    fd: libc::c_int,
    mode: libc::c_int,
) -> *mut fdlog_st {
    let fdlog: *mut fdlog_st = ck_calloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<fdlog_st>() as libc::c_ulong,
    ) as *mut fdlog_st;
    (*fdlog).fn_0 = fn_0;
    (*fdlog).fd = if fd >= 0 as libc::c_int { fd } else { 2 as libc::c_int };
    (*fdlog).mode = mode as C2RustUnnamed;
    return fdlog;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn fdlog_free(fdlog: *mut fdlog_st) {
    if (*fdlog).fd > 2 as libc::c_int {
        close((*fdlog).fd);
    }
    free((*fdlog).b.ptr as *mut libc::c_void);
    free(fdlog as *mut libc::c_void);
}
