use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn free(_: *mut libc::c_void);
    fn vasnprintf(
        resultbuf: *mut libc::c_char,
        lengthp: *mut size_t,
        format: *const libc::c_char,
        args: ::core::ffi::VaList,
    ) -> *mut libc::c_char;
}
pub type __builtin_va_list = __va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list {
    pub __stack: *mut libc::c_void,
    pub __gr_top: *mut libc::c_void,
    pub __vr_top: *mut libc::c_void,
    pub __gr_offs: libc::c_int,
    pub __vr_offs: libc::c_int,
}
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
#[no_mangle]
pub unsafe extern "C" fn rpl_vasprintf(
    mut resultp: *mut *mut libc::c_char,
    mut format: *const libc::c_char,
    mut args: ::core::ffi::VaList,
) -> libc::c_int {
    let mut length: size_t = 0;
    let mut result: *mut libc::c_char = vasnprintf(
        0 as *mut libc::c_char,
        &mut length,
        format,
        args.as_va_list(),
    );
    if result.is_null() {
        return -(1 as libc::c_int);
    }
    if length > 2147483647 as libc::c_int as libc::c_ulong {
        free(result as *mut libc::c_void);
        *__errno_location() = 75 as libc::c_int;
        return -(1 as libc::c_int);
    }
    *resultp = result;
    return length as libc::c_int;
}
