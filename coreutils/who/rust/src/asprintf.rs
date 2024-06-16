use ::libc;
extern "C" {
    fn rpl_vasprintf(
        result: *mut *mut libc::c_char,
        format: *const libc::c_char,
        args: ::core::ffi::VaList,
    ) -> libc::c_int;
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
pub type va_list = __builtin_va_list;
#[no_mangle]
pub unsafe extern "C" fn rpl_asprintf(
    mut resultp: *mut *mut libc::c_char,
    mut format: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut args_0: ::core::ffi::VaListImpl;
    let mut result: libc::c_int = 0;
    args_0 = args.clone();
    result = rpl_vasprintf(resultp, format, args_0.as_va_list());
    return result;
}
