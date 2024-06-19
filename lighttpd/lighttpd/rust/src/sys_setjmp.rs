use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn __sigsetjmp(__env: *mut __jmp_buf_tag, __savemask: libc::c_int) -> libc::c_int;
    fn siglongjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
    fn ck_bt_abort(
        filename: *const libc::c_char,
        line: libc::c_uint,
        msg: *const libc::c_char,
    ) -> !;
}
pub type __off64_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
pub type off_t = __off64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type __jmp_buf = [libc::c_ulonglong; 22];
pub type sigjmp_buf = [__jmp_buf_tag; 1];
pub type sig_atomic_t = __sig_atomic_t;
static mut sys_setjmp_sigbus_jmp_valid: sig_atomic_t = 0;
static mut sys_setjmp_sigbus_jmp_buf: sigjmp_buf = [__jmp_buf_tag {
    __jmpbuf: [0; 22],
    __mask_was_saved: 0,
    __saved_mask: __sigset_t { __val: [0; 16] },
}; 1];
#[no_mangle]
pub unsafe extern "C" fn sys_setjmp_sigbus(mut sig: libc::c_int) {
    if sys_setjmp_sigbus_jmp_valid != 0 {
        siglongjmp(sys_setjmp_sigbus_jmp_buf.as_mut_ptr(), 1 as libc::c_int);
    }
    ck_bt_abort(
        b"sys-setjmp.c\0" as *const u8 as *const libc::c_char,
        39 as libc::c_int as libc::c_uint,
        b"SIGBUS\0" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn sys_setjmp_eval3(
    mut cb: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_void, off_t) -> off_t,
    >,
    mut dst: *mut libc::c_void,
    mut src: *const libc::c_void,
    mut len: off_t,
) -> off_t {
    let mut rv: off_t = 0;
    ::core::ptr::write_volatile(
        &mut sys_setjmp_sigbus_jmp_valid as *mut sig_atomic_t,
        (__sigsetjmp(sys_setjmp_sigbus_jmp_buf.as_mut_ptr(), 0 as libc::c_int) == 0)
            as libc::c_int,
    );
    if ::core::ptr::read_volatile::<
        sig_atomic_t,
    >(&sys_setjmp_sigbus_jmp_valid as *const sig_atomic_t) != 0
    {
        rv = cb.expect("non-null function pointer")(dst, src, len);
    } else {
        *__errno_location() = 14 as libc::c_int;
        return -(1 as libc::c_int) as off_t;
    }
    ::core::ptr::write_volatile(
        &mut sys_setjmp_sigbus_jmp_valid as *mut sig_atomic_t,
        0 as libc::c_int,
    );
    return rv;
}
