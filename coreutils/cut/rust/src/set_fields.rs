use ::libc;
extern "C" {
    fn free(_: *mut libc::c_void);
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn gettext(__msgid: *const libc::c_char) -> *mut libc::c_char;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn x2nrealloc(p: *mut libc::c_void, pn: *mut size_t, s: size_t) -> *mut libc::c_void;
    fn ximemdup0(p: *const libc::c_void, s: idx_t) -> *mut libc::c_char;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn usage(status: libc::c_int);
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
}
pub type __uintmax_t = libc::c_ulong;
pub type size_t = libc::c_ulong;
pub type ptrdiff_t = libc::c_long;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type uintmax_t = __uintmax_t;
pub type idx_t = ptrdiff_t;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct field_range_pair {
    pub lo: uintmax_t,
    pub hi: uintmax_t,
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const SETFLD_ERRMSG_USE_POS: C2RustUnnamed_0 = 4;
pub const SETFLD_COMPLEMENT: C2RustUnnamed_0 = 2;
pub const SETFLD_ALLOW_DASH: C2RustUnnamed_0 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub _gl_dummy: libc::c_int,
}
#[inline]
unsafe extern "C" fn to_uchar(mut ch: libc::c_char) -> libc::c_uchar {
    return ch as libc::c_uchar;
}
#[no_mangle]
pub static mut frp: *mut field_range_pair = 0 as *const field_range_pair
    as *mut field_range_pair;
#[no_mangle]
pub static mut n_frp: size_t = 0;
static mut n_frp_allocated: size_t = 0;
unsafe extern "C" fn add_range_pair(mut lo: uintmax_t, mut hi: uintmax_t) {
    if n_frp == n_frp_allocated {
        frp = (if ::core::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong != 0 {
            x2nrealloc(
                frp as *mut libc::c_void,
                &mut n_frp_allocated,
                ::core::mem::size_of::<field_range_pair>() as libc::c_ulong,
            )
        } else {
            x2nrealloc(
                frp as *mut libc::c_void,
                &mut n_frp_allocated,
                ::core::mem::size_of::<field_range_pair>() as libc::c_ulong,
            )
        }) as *mut field_range_pair;
    }
    (*frp.offset(n_frp as isize)).lo = lo;
    (*frp.offset(n_frp as isize)).hi = hi;
    n_frp = n_frp.wrapping_add(1);
    n_frp;
}
unsafe extern "C" fn compare_ranges(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    let mut ap: *const field_range_pair = a as *const field_range_pair;
    let mut bp: *const field_range_pair = b as *const field_range_pair;
    return ((*ap).lo > (*bp).lo) as libc::c_int - ((*ap).lo < (*bp).lo) as libc::c_int;
}
unsafe extern "C" fn complement_rp() {
    let mut c: *mut field_range_pair = frp;
    let mut n: size_t = n_frp;
    frp = 0 as *mut field_range_pair;
    n_frp = 0 as libc::c_int as size_t;
    n_frp_allocated = 0 as libc::c_int as size_t;
    if (*c.offset(0 as libc::c_int as isize)).lo > 1 as libc::c_int as libc::c_ulong {
        add_range_pair(
            1 as libc::c_int as uintmax_t,
            ((*c.offset(0 as libc::c_int as isize)).lo)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    }
    let mut i: size_t = 1 as libc::c_int as size_t;
    while i < n {
        if !(((*c.offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)).hi)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            == (*c.offset(i as isize)).lo)
        {
            add_range_pair(
                ((*c.offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
                    .hi)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
                ((*c.offset(i as isize)).lo)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
        }
        i = i.wrapping_add(1);
        i;
    }
    if (*c.offset(n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)).hi
        < 18446744073709551615 as libc::c_ulong
    {
        add_range_pair(
            ((*c.offset(n.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)).hi)
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
            18446744073709551615 as libc::c_ulong,
        );
    }
    free(c as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn set_fields(
    mut fieldstr: *const libc::c_char,
    mut options: libc::c_uint,
) {
    let mut initial: uintmax_t = 1 as libc::c_int as uintmax_t;
    let mut value: uintmax_t = 0 as libc::c_int as uintmax_t;
    let mut lhs_specified: bool = 0 as libc::c_int != 0;
    let mut rhs_specified: bool = 0 as libc::c_int != 0;
    let mut dash_found: bool = 0 as libc::c_int != 0;
    let mut in_digits: bool = 0 as libc::c_int != 0;
    if options & SETFLD_ALLOW_DASH as libc::c_int as libc::c_uint != 0
        && strcmp(fieldstr, b"-\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
    {
        value = 1 as libc::c_int as uintmax_t;
        lhs_specified = 1 as libc::c_int != 0;
        dash_found = 1 as libc::c_int != 0;
        fieldstr = fieldstr.offset(1);
        fieldstr;
    }
    loop {
        if *fieldstr as libc::c_int == '-' as i32 {
            in_digits = 0 as libc::c_int != 0;
            if dash_found {
                if 0 != 0 {
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        if options
                            & SETFLD_ERRMSG_USE_POS as libc::c_int as libc::c_uint != 0
                        {
                            gettext(
                                b"invalid byte or character range\0" as *const u8
                                    as *const libc::c_char,
                            )
                        } else {
                            gettext(
                                b"invalid field range\0" as *const u8 as *const libc::c_char,
                            )
                        },
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
                            if options
                                & SETFLD_ERRMSG_USE_POS as libc::c_int as libc::c_uint != 0
                            {
                                gettext(
                                    b"invalid byte or character range\0" as *const u8
                                        as *const libc::c_char,
                                )
                            } else {
                                gettext(
                                    b"invalid field range\0" as *const u8 as *const libc::c_char,
                                )
                            },
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
                            if options
                                & SETFLD_ERRMSG_USE_POS as libc::c_int as libc::c_uint != 0
                            {
                                gettext(
                                    b"invalid byte or character range\0" as *const u8
                                        as *const libc::c_char,
                                )
                            } else {
                                gettext(
                                    b"invalid field range\0" as *const u8 as *const libc::c_char,
                                )
                            },
                        );
                        if __errstatus != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                        
                    });
                };
                usage(1 as libc::c_int);
            }
            dash_found = 1 as libc::c_int != 0;
            fieldstr = fieldstr.offset(1);
            fieldstr;
            if lhs_specified as libc::c_int != 0 && value == 0 {
                if 0 != 0 {
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        if options
                            & SETFLD_ERRMSG_USE_POS as libc::c_int as libc::c_uint != 0
                        {
                            gettext(
                                b"byte/character positions are numbered from 1\0"
                                    as *const u8 as *const libc::c_char,
                            )
                        } else {
                            gettext(
                                b"fields are numbered from 1\0" as *const u8
                                    as *const libc::c_char,
                            )
                        },
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
                            if options
                                & SETFLD_ERRMSG_USE_POS as libc::c_int as libc::c_uint != 0
                            {
                                gettext(
                                    b"byte/character positions are numbered from 1\0"
                                        as *const u8 as *const libc::c_char,
                                )
                            } else {
                                gettext(
                                    b"fields are numbered from 1\0" as *const u8
                                        as *const libc::c_char,
                                )
                            },
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
                            if options
                                & SETFLD_ERRMSG_USE_POS as libc::c_int as libc::c_uint != 0
                            {
                                gettext(
                                    b"byte/character positions are numbered from 1\0"
                                        as *const u8 as *const libc::c_char,
                                )
                            } else {
                                gettext(
                                    b"fields are numbered from 1\0" as *const u8
                                        as *const libc::c_char,
                                )
                            },
                        );
                        if __errstatus != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                        
                    });
                };
                usage(1 as libc::c_int);
            }
            initial = if lhs_specified as libc::c_int != 0 {
                value
            } else {
                1 as libc::c_int as libc::c_ulong
            };
            value = 0 as libc::c_int as uintmax_t;
        } else if *fieldstr as libc::c_int == ',' as i32
            || *(*__ctype_b_loc()).offset(to_uchar(*fieldstr) as libc::c_int as isize)
                as libc::c_int & _ISblank as libc::c_int as libc::c_ushort as libc::c_int
                != 0 || *fieldstr as libc::c_int == '\0' as i32
        {
            in_digits = 0 as libc::c_int != 0;
            if dash_found {
                dash_found = 0 as libc::c_int != 0;
                if !lhs_specified && !rhs_specified {
                    if options & SETFLD_ALLOW_DASH as libc::c_int as libc::c_uint != 0 {
                        initial = 1 as libc::c_int as uintmax_t;
                    } else {
                        if 0 != 0 {
                            error(
                                0 as libc::c_int,
                                0 as libc::c_int,
                                gettext(
                                    b"invalid range with no endpoint: -\0" as *const u8
                                        as *const libc::c_char,
                                ),
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
                                        b"invalid range with no endpoint: -\0" as *const u8
                                            as *const libc::c_char,
                                    ),
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
                                        b"invalid range with no endpoint: -\0" as *const u8
                                            as *const libc::c_char,
                                    ),
                                );
                                if __errstatus != 0 as libc::c_int {
                                    unreachable!();
                                } else {};
                                
                            });
                        };
                        usage(1 as libc::c_int);
                    }
                }
                if !rhs_specified {
                    add_range_pair(initial, 18446744073709551615 as libc::c_ulong);
                } else {
                    if value < initial {
                        if 0 != 0 {
                            error(
                                0 as libc::c_int,
                                0 as libc::c_int,
                                gettext(
                                    b"invalid decreasing range\0" as *const u8
                                        as *const libc::c_char,
                                ),
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
                                        b"invalid decreasing range\0" as *const u8
                                            as *const libc::c_char,
                                    ),
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
                                        b"invalid decreasing range\0" as *const u8
                                            as *const libc::c_char,
                                    ),
                                );
                                if __errstatus != 0 as libc::c_int {
                                    unreachable!();
                                } else {};
                                
                            });
                        };
                        usage(1 as libc::c_int);
                    }
                    add_range_pair(initial, value);
                }
                value = 0 as libc::c_int as uintmax_t;
            } else {
                if value == 0 as libc::c_int as libc::c_ulong {
                    if 0 != 0 {
                        error(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            if options
                                & SETFLD_ERRMSG_USE_POS as libc::c_int as libc::c_uint != 0
                            {
                                gettext(
                                    b"byte/character positions are numbered from 1\0"
                                        as *const u8 as *const libc::c_char,
                                )
                            } else {
                                gettext(
                                    b"fields are numbered from 1\0" as *const u8
                                        as *const libc::c_char,
                                )
                            },
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
                                if options
                                    & SETFLD_ERRMSG_USE_POS as libc::c_int as libc::c_uint != 0
                                {
                                    gettext(
                                        b"byte/character positions are numbered from 1\0"
                                            as *const u8 as *const libc::c_char,
                                    )
                                } else {
                                    gettext(
                                        b"fields are numbered from 1\0" as *const u8
                                            as *const libc::c_char,
                                    )
                                },
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
                                if options
                                    & SETFLD_ERRMSG_USE_POS as libc::c_int as libc::c_uint != 0
                                {
                                    gettext(
                                        b"byte/character positions are numbered from 1\0"
                                            as *const u8 as *const libc::c_char,
                                    )
                                } else {
                                    gettext(
                                        b"fields are numbered from 1\0" as *const u8
                                            as *const libc::c_char,
                                    )
                                },
                            );
                            if __errstatus != 0 as libc::c_int {
                                unreachable!();
                            } else {};
                            
                        });
                    };
                    usage(1 as libc::c_int);
                }
                add_range_pair(value, value);
                value = 0 as libc::c_int as uintmax_t;
            }
            if *fieldstr as libc::c_int == '\0' as i32 {
                break;
            }
            fieldstr = fieldstr.offset(1);
            fieldstr;
            lhs_specified = 0 as libc::c_int != 0;
            rhs_specified = 0 as libc::c_int != 0;
        } else if (*fieldstr as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
            <= 9 as libc::c_int as libc::c_uint
        {
            static mut num_start: *const libc::c_char = 0 as *const libc::c_char;
            if !in_digits || num_start.is_null() {
                num_start = fieldstr;
            }
            in_digits = 1 as libc::c_int != 0;
            if dash_found {
                rhs_specified = 1 as libc::c_int != 0;
            } else {
                lhs_specified = 1 as libc::c_int != 0;
            }
            if !(!((if (0 as libc::c_int as uintmax_t) < -(1 as libc::c_int) as uintmax_t
                && (if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    value
                })
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    < 0 as libc::c_int as libc::c_ulong
                && ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int
                } else {
                    10 as libc::c_int
                }) - 1 as libc::c_int) < 0 as libc::c_int
                && (if (10 as libc::c_int) < 0 as libc::c_int {
                    if value < 0 as libc::c_int as libc::c_ulong {
                        if (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                -(1 as libc::c_int) as uintmax_t
                            })
                                .wrapping_add(10 as libc::c_int as libc::c_ulong)
                        })
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            < 0 as libc::c_int as libc::c_ulong
                        {
                            (value
                                < (-(1 as libc::c_int) as uintmax_t)
                                    .wrapping_div(10 as libc::c_int as libc::c_ulong))
                                as libc::c_int
                        } else {
                            ((if (if (if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                10 as libc::c_int
                            }) - 1 as libc::c_int) < 0 as libc::c_int
                            {
                                !(((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    10 as libc::c_int
                                }) + 1 as libc::c_int)
                                    << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    10 as libc::c_int
                                }) + 0 as libc::c_int
                            }) < 0 as libc::c_int
                            {
                                ((10 as libc::c_int)
                                    < -(if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        10 as libc::c_int
                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                    {
                                        ((((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            10 as libc::c_int
                                        }) + 1 as libc::c_int)
                                            << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            10 as libc::c_int
                                        }) - 1 as libc::c_int
                                    })) as libc::c_int
                            } else {
                                ((0 as libc::c_int) < 10 as libc::c_int) as libc::c_int
                            }) != 0
                            {
                                ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    10 as libc::c_int
                                }) as libc::c_ulong)
                                    .wrapping_add(-(1 as libc::c_int) as uintmax_t)
                                    >> (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            } else {
                                (-(1 as libc::c_int) as uintmax_t)
                                    .wrapping_div(-(10 as libc::c_int) as libc::c_ulong)
                            })
                                <= (-(1 as libc::c_int) as libc::c_ulong)
                                    .wrapping_sub(value)) as libc::c_int
                        }
                    } else {
                        if (if (if (if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                10 as libc::c_int
                            }) as libc::c_ulong)
                                .wrapping_add(0 as libc::c_int as uintmax_t)
                        })
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            < 0 as libc::c_int as libc::c_ulong
                        {
                            !((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    10 as libc::c_int
                                }) as libc::c_ulong)
                                    .wrapping_add(0 as libc::c_int as uintmax_t)
                            })
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                << (::core::mem::size_of::<libc::c_ulong>()
                                    as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    10 as libc::c_int
                                }) as libc::c_ulong)
                                    .wrapping_add(0 as libc::c_int as uintmax_t)
                            })
                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                        }) < 0 as libc::c_int as libc::c_ulong
                        {
                            (((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                10 as libc::c_int
                            }) as libc::c_ulong)
                                .wrapping_add(0 as libc::c_int as uintmax_t)
                                < (if (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        10 as libc::c_int
                                    }) as libc::c_ulong)
                                        .wrapping_add(0 as libc::c_int as uintmax_t)
                                })
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    < 0 as libc::c_int as libc::c_ulong
                                {
                                    ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            10 as libc::c_int
                                        }) as libc::c_ulong)
                                            .wrapping_add(0 as libc::c_int as uintmax_t)
                                    })
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        << (::core::mem::size_of::<libc::c_ulong>()
                                            as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            10 as libc::c_int
                                        }) as libc::c_ulong)
                                            .wrapping_add(0 as libc::c_int as uintmax_t)
                                    })
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                })
                                    .wrapping_neg()) as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_ulong)
                                < ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    10 as libc::c_int
                                }) as libc::c_ulong)
                                    .wrapping_add(0 as libc::c_int as uintmax_t)) as libc::c_int
                        }) != 0 && 10 as libc::c_int == -(1 as libc::c_int)
                        {
                            if (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                value
                            })
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                < 0 as libc::c_int as libc::c_ulong
                            {
                                ((0 as libc::c_int as libc::c_ulong)
                                    < value.wrapping_add(0 as libc::c_int as uintmax_t))
                                    as libc::c_int
                            } else {
                                ((0 as libc::c_int as libc::c_ulong) < value
                                    && (-(1 as libc::c_int) as libc::c_ulong)
                                        .wrapping_sub(0 as libc::c_int as uintmax_t)
                                        < value.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                                    as libc::c_int
                            }
                        } else {
                            ((0 as libc::c_int as uintmax_t)
                                .wrapping_div(10 as libc::c_int as libc::c_ulong) < value)
                                as libc::c_int
                        }
                    }
                } else {
                    if 10 as libc::c_int == 0 as libc::c_int {
                        0 as libc::c_int
                    } else {
                        if value < 0 as libc::c_int as libc::c_ulong {
                            if (if (if (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    value
                                })
                                    .wrapping_add(0 as libc::c_int as uintmax_t)
                            })
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                < 0 as libc::c_int as libc::c_ulong
                            {
                                !((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        value
                                    })
                                        .wrapping_add(0 as libc::c_int as uintmax_t)
                                })
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    << (::core::mem::size_of::<libc::c_ulong>()
                                        as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        value
                                    })
                                        .wrapping_add(0 as libc::c_int as uintmax_t)
                                })
                                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                            }) < 0 as libc::c_int as libc::c_ulong
                            {
                                ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_ulong
                                } else {
                                    value
                                })
                                    .wrapping_add(0 as libc::c_int as uintmax_t)
                                    < (if (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            value
                                        })
                                            .wrapping_add(0 as libc::c_int as uintmax_t)
                                    })
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        < 0 as libc::c_int as libc::c_ulong
                                    {
                                        ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                value
                                            })
                                                .wrapping_add(0 as libc::c_int as uintmax_t)
                                        })
                                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            << (::core::mem::size_of::<libc::c_ulong>()
                                                as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_ulong
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                value
                                            })
                                                .wrapping_add(0 as libc::c_int as uintmax_t)
                                        })
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    })
                                        .wrapping_neg()) as libc::c_int
                            } else {
                                ((0 as libc::c_int as libc::c_ulong)
                                    < (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        value
                                    })
                                        .wrapping_add(0 as libc::c_int as uintmax_t)) as libc::c_int
                            }) != 0 && value == -(1 as libc::c_int) as libc::c_ulong
                            {
                                if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    10 as libc::c_int
                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                {
                                    ((0 as libc::c_int as libc::c_ulong)
                                        < (10 as libc::c_int as libc::c_ulong)
                                            .wrapping_add(0 as libc::c_int as uintmax_t)) as libc::c_int
                                } else {
                                    ((-(1 as libc::c_int) as libc::c_ulong)
                                        .wrapping_sub(0 as libc::c_int as uintmax_t)
                                        < (10 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
                                        as libc::c_int
                                }
                            } else {
                                ((0 as libc::c_int as uintmax_t).wrapping_div(value)
                                    < 10 as libc::c_int as libc::c_ulong) as libc::c_int
                            }
                        } else {
                            ((-(1 as libc::c_int) as uintmax_t)
                                .wrapping_div(10 as libc::c_int as libc::c_ulong) < value)
                                as libc::c_int
                        }
                    }
                }) != 0
            {
                let (fresh4, _fresh5) = value.overflowing_mul((10 as libc::c_int).try_into().unwrap());
                *(&mut value as *mut uintmax_t) = fresh4;
                1 as libc::c_int
            } else {
                let (fresh6, fresh7) = value.overflowing_mul((10 as libc::c_int).try_into().unwrap());
                *(&mut value as *mut uintmax_t) = fresh6;
                fresh7 as libc::c_int
            }) != 0)
                && {
                    let (fresh8, fresh9) = value
                        .overflowing_add((*fieldstr as libc::c_int - '0' as i32).try_into().unwrap());
                    *(&mut value as *mut uintmax_t) = fresh8;
                    !fresh9
                }) || value == 18446744073709551615 as libc::c_ulong
            {
                let mut len: size_t = strspn(
                    num_start,
                    b"0123456789\0" as *const u8 as *const libc::c_char,
                );
                let mut bad_num: *mut libc::c_char = ximemdup0(
                    num_start as *const libc::c_void,
                    len as idx_t,
                );
                if 0 != 0 {
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        if options
                            & SETFLD_ERRMSG_USE_POS as libc::c_int as libc::c_uint != 0
                        {
                            gettext(
                                b"byte/character offset %s is too large\0" as *const u8
                                    as *const libc::c_char,
                            )
                        } else {
                            gettext(
                                b"field number %s is too large\0" as *const u8
                                    as *const libc::c_char,
                            )
                        },
                        quote(bad_num),
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
                            if options
                                & SETFLD_ERRMSG_USE_POS as libc::c_int as libc::c_uint != 0
                            {
                                gettext(
                                    b"byte/character offset %s is too large\0" as *const u8
                                        as *const libc::c_char,
                                )
                            } else {
                                gettext(
                                    b"field number %s is too large\0" as *const u8
                                        as *const libc::c_char,
                                )
                            },
                            quote(bad_num),
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
                            if options
                                & SETFLD_ERRMSG_USE_POS as libc::c_int as libc::c_uint != 0
                            {
                                gettext(
                                    b"byte/character offset %s is too large\0" as *const u8
                                        as *const libc::c_char,
                                )
                            } else {
                                gettext(
                                    b"field number %s is too large\0" as *const u8
                                        as *const libc::c_char,
                                )
                            },
                            quote(bad_num),
                        );
                        if __errstatus != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                        
                    });
                };
                free(bad_num as *mut libc::c_void);
                usage(1 as libc::c_int);
            }
            fieldstr = fieldstr.offset(1);
            fieldstr;
        } else {
            if 0 != 0 {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    if options & SETFLD_ERRMSG_USE_POS as libc::c_int as libc::c_uint
                        != 0
                    {
                        gettext(
                            b"invalid byte/character position %s\0" as *const u8
                                as *const libc::c_char,
                        )
                    } else {
                        gettext(
                            b"invalid field value %s\0" as *const u8
                                as *const libc::c_char,
                        )
                    },
                    quote(fieldstr),
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
                        if options
                            & SETFLD_ERRMSG_USE_POS as libc::c_int as libc::c_uint != 0
                        {
                            gettext(
                                b"invalid byte/character position %s\0" as *const u8
                                    as *const libc::c_char,
                            )
                        } else {
                            gettext(
                                b"invalid field value %s\0" as *const u8
                                    as *const libc::c_char,
                            )
                        },
                        quote(fieldstr),
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
                        if options
                            & SETFLD_ERRMSG_USE_POS as libc::c_int as libc::c_uint != 0
                        {
                            gettext(
                                b"invalid byte/character position %s\0" as *const u8
                                    as *const libc::c_char,
                            )
                        } else {
                            gettext(
                                b"invalid field value %s\0" as *const u8
                                    as *const libc::c_char,
                            )
                        },
                        quote(fieldstr),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
            usage(1 as libc::c_int);
        }
    }
    if n_frp == 0 {
        if 0 != 0 {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                if options & SETFLD_ERRMSG_USE_POS as libc::c_int as libc::c_uint != 0 {
                    gettext(
                        b"missing list of byte/character positions\0" as *const u8
                            as *const libc::c_char,
                    )
                } else {
                    gettext(
                        b"missing list of fields\0" as *const u8 as *const libc::c_char,
                    )
                },
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
                    if options & SETFLD_ERRMSG_USE_POS as libc::c_int as libc::c_uint
                        != 0
                    {
                        gettext(
                            b"missing list of byte/character positions\0" as *const u8
                                as *const libc::c_char,
                        )
                    } else {
                        gettext(
                            b"missing list of fields\0" as *const u8
                                as *const libc::c_char,
                        )
                    },
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
                    if options & SETFLD_ERRMSG_USE_POS as libc::c_int as libc::c_uint
                        != 0
                    {
                        gettext(
                            b"missing list of byte/character positions\0" as *const u8
                                as *const libc::c_char,
                        )
                    } else {
                        gettext(
                            b"missing list of fields\0" as *const u8
                                as *const libc::c_char,
                        )
                    },
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
        usage(1 as libc::c_int);
    }
    qsort(
        frp as *mut libc::c_void,
        n_frp,
        ::core::mem::size_of::<field_range_pair>() as libc::c_ulong,
        Some(
            compare_ranges
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < n_frp {
        let mut j: size_t = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
        while j < n_frp {
            if !((*frp.offset(j as isize)).lo <= (*frp.offset(i as isize)).hi) {
                break;
            }
            (*frp.offset(i as isize))
                .hi = if (*frp.offset(j as isize)).hi > (*frp.offset(i as isize)).hi {
                (*frp.offset(j as isize)).hi
            } else {
                (*frp.offset(i as isize)).hi
            };
            memmove(
                frp.offset(j as isize) as *mut libc::c_void,
                frp.offset(j as isize).offset(1 as libc::c_int as isize)
                    as *const libc::c_void,
                n_frp
                    .wrapping_sub(j)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<field_range_pair>() as libc::c_ulong,
                    ),
            );
            n_frp = n_frp.wrapping_sub(1);
            n_frp;
            j = j.wrapping_sub(1);
            j;
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    if options & SETFLD_COMPLEMENT as libc::c_int as libc::c_uint != 0 {
        complement_rp();
    }
    n_frp = n_frp.wrapping_add(1);
    n_frp;
    frp = xrealloc(
        frp as *mut libc::c_void,
        n_frp.wrapping_mul(::core::mem::size_of::<field_range_pair>() as libc::c_ulong),
    ) as *mut field_range_pair;
    let ref mut fresh10 = (*frp
        .offset(n_frp.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
        .hi;
    *fresh10 = 18446744073709551615 as libc::c_ulong;
    (*frp.offset(n_frp.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize))
        .lo = *fresh10;
}
