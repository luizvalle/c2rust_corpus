use ::libc;
extern "C" {
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn mktime_z(__tz: timezone_t, __tm: *mut tm) -> time_t;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn toupper(_: libc::c_int) -> libc::c_int;
    fn tolower(_: libc::c_int) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type time_t = __time_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm_zone {
    pub next: *mut tm_zone,
    pub tz_is_set: libc::c_char,
    pub abbrs: [libc::c_char; 0],
}
pub type timezone_t = *mut tm_zone;
pub type pad_style = libc::c_uint;
pub const NO_PAD: pad_style = 4;
pub const SPACE_PAD: pad_style = 3;
pub const SIGN_PAD: pad_style = 2;
pub const ALWAYS_ZERO_PAD: pad_style = 1;
pub const ZERO_PAD: pad_style = 0;
#[no_mangle]
pub unsafe extern "C" fn nstrftime(
    mut s: *mut libc::c_char,
    mut maxsize: size_t,
    mut format: *const libc::c_char,
    mut tp: *const tm,
    mut tz: timezone_t,
    mut ns: libc::c_int,
) -> size_t {
    let mut tzset_called: bool = 0 as libc::c_int != 0;
    return __strftime_internal(
        s,
        maxsize,
        format,
        tp,
        0 as libc::c_int != 0,
        ZERO_PAD,
        -(1 as libc::c_int),
        &mut tzset_called,
        tz,
        ns,
    );
}
unsafe extern "C" fn __strftime_internal(
    mut s: *mut libc::c_char,
    mut maxsize: size_t,
    mut format: *const libc::c_char,
    mut tp: *const tm,
    mut upcase: bool,
    mut yr_spec: pad_style,
    mut width: libc::c_int,
    mut tzset_called: *mut bool,
    mut tz: timezone_t,
    mut ns: libc::c_int,
) -> size_t {
    let mut current_block: u64;
    let mut saved_errno: libc::c_int = *__errno_location();
    let mut hour12: libc::c_int = (*tp).tm_hour;
    let mut zone: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut p: *mut libc::c_char = s;
    let mut f: *const libc::c_char = 0 as *const libc::c_char;
    zone = 0 as *const libc::c_char;
    zone = (*tp).tm_zone;
    if zone.is_null() {
        zone = b"\0" as *const u8 as *const libc::c_char;
    }
    if hour12 > 12 as libc::c_int {
        hour12 -= 12 as libc::c_int;
    } else if hour12 == 0 as libc::c_int {
        hour12 = 12 as libc::c_int;
    }
    f = format;
    while *f as libc::c_int != '\0' as i32 {
        let mut pad: pad_style = ZERO_PAD;
        let mut modifier: libc::c_int = 0;
        let mut digits: libc::c_int = 0 as libc::c_int;
        let mut number_value: libc::c_int = 0;
        let mut u_number_value: libc::c_uint = 0;
        let mut negative_number: bool = false;
        let mut always_output_a_sign: bool = false;
        let mut tz_colon_mask: libc::c_int = 0;
        let mut subfmt: *const libc::c_char = 0 as *const libc::c_char;
        let mut bufp: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut buf: [libc::c_char; 23] = [0; 23];
        let mut to_lowcase: bool = 0 as libc::c_int != 0;
        let mut to_uppcase: bool = upcase;
        let mut colons: size_t = 0;
        let mut change_case: bool = 0 as libc::c_int != 0;
        let mut format_char: libc::c_int = 0;
        let mut subwidth: libc::c_int = 0;
        if *f as libc::c_int != '%' as i32 {
            let mut _n: size_t = 1 as libc::c_int as size_t;
            let mut _w: size_t = (if pad as libc::c_uint
                == NO_PAD as libc::c_int as libc::c_uint || width < 0 as libc::c_int
            {
                0 as libc::c_int
            } else {
                width
            }) as size_t;
            let mut _incr: size_t = if _n < _w { _w } else { _n };
            if _incr >= maxsize.wrapping_sub(i) {
                *__errno_location() = 34 as libc::c_int;
                return 0 as libc::c_int as size_t;
            }
            if !p.is_null() {
                if _n < _w {
                    let mut _delta: size_t = _w.wrapping_sub(_n);
                    if pad as libc::c_uint
                        == ALWAYS_ZERO_PAD as libc::c_int as libc::c_uint
                        || pad as libc::c_uint == SIGN_PAD as libc::c_int as libc::c_uint
                    {
                        memset(p as *mut libc::c_void, '0' as i32, _delta);
                        p = p.offset(_delta as isize);
                    } else {
                        memset(p as *mut libc::c_void, ' ' as i32, _delta);
                        p = p.offset(_delta as isize);
                    }
                }
                *p = *f;
                p = p.offset(_n as isize);
            }
            i = (i as libc::c_ulong).wrapping_add(_incr) as size_t as size_t;
        } else {
            let mut percent: *const libc::c_char = f;
            loop {
                f = f.offset(1);
                match *f as libc::c_int {
                    95 => {
                        pad = SPACE_PAD;
                    }
                    45 => {
                        pad = NO_PAD;
                    }
                    43 => {
                        pad = SIGN_PAD;
                    }
                    48 => {
                        pad = ALWAYS_ZERO_PAD;
                    }
                    94 => {
                        to_uppcase = 1 as libc::c_int != 0;
                    }
                    35 => {
                        change_case = 1 as libc::c_int != 0;
                    }
                    _ => {
                        break;
                    }
                }
            }
            if (*f as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
                <= 9 as libc::c_int as libc::c_uint
            {
                width = 0 as libc::c_int;
                loop {
                    if (if (0 as libc::c_int) < -(1 as libc::c_int)
                        && ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            width
                        }) - 1 as libc::c_int) < 0 as libc::c_int
                        && ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            10 as libc::c_int
                        }) - 1 as libc::c_int) < 0 as libc::c_int
                        && (if (10 as libc::c_int) < 0 as libc::c_int {
                            if width < 0 as libc::c_int {
                                if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        -(1 as libc::c_int)
                                    }) + 10 as libc::c_int
                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                {
                                    (width < -(1 as libc::c_int) / 10 as libc::c_int)
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
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            10 as libc::c_int
                                        }) + -(1 as libc::c_int)
                                            >> (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                    } else {
                                        -(1 as libc::c_int) / -(10 as libc::c_int)
                                    }) <= -(1 as libc::c_int) - width) as libc::c_int
                                }
                            } else {
                                if (if (if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        10 as libc::c_int
                                    }) + 0 as libc::c_int
                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                {
                                    !(((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            10 as libc::c_int
                                        }) + 0 as libc::c_int
                                    }) + 1 as libc::c_int)
                                        << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            10 as libc::c_int
                                        }) + 0 as libc::c_int
                                    }) + 0 as libc::c_int
                                }) < 0 as libc::c_int
                                {
                                    (((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        10 as libc::c_int
                                    }) + 0 as libc::c_int)
                                        < -(if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                10 as libc::c_int
                                            }) + 0 as libc::c_int
                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                        {
                                            ((((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    10 as libc::c_int
                                                }) + 0 as libc::c_int
                                            }) + 1 as libc::c_int)
                                                << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    10 as libc::c_int
                                                }) + 0 as libc::c_int
                                            }) - 1 as libc::c_int
                                        })) as libc::c_int
                                } else {
                                    ((0 as libc::c_int)
                                        < (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            10 as libc::c_int
                                        }) + 0 as libc::c_int) as libc::c_int
                                }) != 0 && 10 as libc::c_int == -(1 as libc::c_int)
                                {
                                    if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        width
                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                    {
                                        ((0 as libc::c_int) < width + 0 as libc::c_int)
                                            as libc::c_int
                                    } else {
                                        ((0 as libc::c_int) < width
                                            && (-(1 as libc::c_int) - 0 as libc::c_int)
                                                < width - 1 as libc::c_int) as libc::c_int
                                    }
                                } else {
                                    ((0 as libc::c_int / 10 as libc::c_int) < width)
                                        as libc::c_int
                                }
                            }
                        } else {
                            if 10 as libc::c_int == 0 as libc::c_int {
                                0 as libc::c_int
                            } else {
                                if width < 0 as libc::c_int {
                                    if (if (if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            width
                                        }) + 0 as libc::c_int
                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                    {
                                        !(((((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                width
                                            }) + 0 as libc::c_int
                                        }) + 1 as libc::c_int)
                                            << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                width
                                            }) + 0 as libc::c_int
                                        }) + 0 as libc::c_int
                                    }) < 0 as libc::c_int
                                    {
                                        (((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            width
                                        }) + 0 as libc::c_int)
                                            < -(if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    width
                                                }) + 0 as libc::c_int
                                            }) - 1 as libc::c_int) < 0 as libc::c_int
                                            {
                                                ((((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        width
                                                    }) + 0 as libc::c_int
                                                }) + 1 as libc::c_int)
                                                    << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        width
                                                    }) + 0 as libc::c_int
                                                }) - 1 as libc::c_int
                                            })) as libc::c_int
                                    } else {
                                        ((0 as libc::c_int)
                                            < (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                width
                                            }) + 0 as libc::c_int) as libc::c_int
                                    }) != 0 && width == -(1 as libc::c_int)
                                    {
                                        if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            10 as libc::c_int
                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                        {
                                            ((0 as libc::c_int) < 10 as libc::c_int + 0 as libc::c_int)
                                                as libc::c_int
                                        } else {
                                            ((-(1 as libc::c_int) - 0 as libc::c_int)
                                                < 10 as libc::c_int - 1 as libc::c_int) as libc::c_int
                                        }
                                    } else {
                                        (0 as libc::c_int / width < 10 as libc::c_int)
                                            as libc::c_int
                                    }
                                } else {
                                    ((-(1 as libc::c_int) / 10 as libc::c_int) < width)
                                        as libc::c_int
                                }
                            }
                        }) != 0
                    {
                        let (fresh4, fresh5) = width.overflowing_mul(10 as libc::c_int);
                        *(&mut width as *mut libc::c_int) = fresh4;
                        1 as libc::c_int
                    } else {
                        let (fresh6, fresh7) = width.overflowing_mul(10 as libc::c_int);
                        *(&mut width as *mut libc::c_int) = fresh6;
                        fresh7 as libc::c_int
                    }) != 0
                        || {
                            let (fresh8, fresh9) = width
                                .overflowing_add(*f as libc::c_int - '0' as i32);
                            *(&mut width as *mut libc::c_int) = fresh8;
                            fresh9 as libc::c_int != 0
                        }
                    {
                        width = 2147483647 as libc::c_int;
                    }
                    f = f.offset(1);
                    f;
                    if !((*f as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
                        <= 9 as libc::c_int as libc::c_uint)
                    {
                        break;
                    }
                }
            }
            match *f as libc::c_int {
                69 | 79 => {
                    let fresh10 = f;
                    f = f.offset(1);
                    modifier = *fresh10 as libc::c_int;
                }
                _ => {
                    modifier = 0 as libc::c_int;
                }
            }
            format_char = *f as libc::c_int;
            match format_char {
                37 => {
                    if f.offset(-(1 as libc::c_int as isize)) != percent {
                        current_block = 15371827921599879935;
                    } else {
                        let mut _n_0: size_t = 1 as libc::c_int as size_t;
                        let mut _w_0: size_t = (if pad as libc::c_uint
                            == NO_PAD as libc::c_int as libc::c_uint
                            || width < 0 as libc::c_int
                        {
                            0 as libc::c_int
                        } else {
                            width
                        }) as size_t;
                        let mut _incr_0: size_t = if _n_0 < _w_0 { _w_0 } else { _n_0 };
                        if _incr_0 >= maxsize.wrapping_sub(i) {
                            *__errno_location() = 34 as libc::c_int;
                            return 0 as libc::c_int as size_t;
                        }
                        if !p.is_null() {
                            if _n_0 < _w_0 {
                                let mut _delta_0: size_t = _w_0.wrapping_sub(_n_0);
                                if pad as libc::c_uint
                                    == ALWAYS_ZERO_PAD as libc::c_int as libc::c_uint
                                    || pad as libc::c_uint
                                        == SIGN_PAD as libc::c_int as libc::c_uint
                                {
                                    memset(p as *mut libc::c_void, '0' as i32, _delta_0);
                                    p = p.offset(_delta_0 as isize);
                                } else {
                                    memset(p as *mut libc::c_void, ' ' as i32, _delta_0);
                                    p = p.offset(_delta_0 as isize);
                                }
                            }
                            *p = *f;
                            p = p.offset(_n_0 as isize);
                        }
                        i = (i as libc::c_ulong).wrapping_add(_incr_0) as size_t
                            as size_t;
                        current_block = 3276175668257526147;
                    }
                }
                97 => {
                    if modifier != 0 as libc::c_int {
                        current_block = 8263860476293076988;
                    } else {
                        if change_case {
                            to_uppcase = 1 as libc::c_int != 0;
                            to_lowcase = 0 as libc::c_int != 0;
                        }
                        current_block = 11183847479862768785;
                    }
                }
                65 => {
                    if modifier != 0 as libc::c_int {
                        current_block = 8263860476293076988;
                    } else {
                        if change_case {
                            to_uppcase = 1 as libc::c_int != 0;
                            to_lowcase = 0 as libc::c_int != 0;
                        }
                        current_block = 11183847479862768785;
                    }
                }
                98 | 104 => {
                    if change_case {
                        to_uppcase = 1 as libc::c_int != 0;
                        to_lowcase = 0 as libc::c_int != 0;
                    }
                    if modifier == 'E' as i32 {
                        current_block = 8263860476293076988;
                    } else {
                        current_block = 11183847479862768785;
                    }
                }
                66 => {
                    if modifier == 'E' as i32 {
                        current_block = 8263860476293076988;
                    } else {
                        if change_case {
                            to_uppcase = 1 as libc::c_int != 0;
                            to_lowcase = 0 as libc::c_int != 0;
                        }
                        current_block = 11183847479862768785;
                    }
                }
                99 => {
                    if modifier == 'O' as i32 {
                        current_block = 8263860476293076988;
                    } else {
                        current_block = 11183847479862768785;
                    }
                }
                67 => {
                    if modifier == 'E' as i32 {
                        current_block = 11183847479862768785;
                    } else {
                        let mut negative_year: bool = (*tp).tm_year
                            < -(1900 as libc::c_int);
                        let mut zero_thru_1899: bool = !negative_year as libc::c_int
                            & ((*tp).tm_year < 0 as libc::c_int) as libc::c_int != 0;
                        let mut century: libc::c_int = ((*tp).tm_year
                            - 99 as libc::c_int * zero_thru_1899 as libc::c_int)
                            / 100 as libc::c_int
                            + 1900 as libc::c_int / 100 as libc::c_int;
                        digits = 2 as libc::c_int;
                        negative_number = negative_year;
                        u_number_value = century as libc::c_uint;
                        current_block = 15989891087299139605;
                    }
                }
                120 => {
                    if modifier == 'O' as i32 {
                        current_block = 8263860476293076988;
                    } else {
                        current_block = 11183847479862768785;
                    }
                }
                68 => {
                    if modifier != 0 as libc::c_int {
                        current_block = 8263860476293076988;
                    } else {
                        subfmt = b"%m/%d/%y\0" as *const u8 as *const libc::c_char;
                        current_block = 15577625217767911782;
                    }
                }
                100 => {
                    if modifier == 'E' as i32 {
                        current_block = 8263860476293076988;
                    } else {
                        digits = 2 as libc::c_int;
                        number_value = (*tp).tm_mday;
                        current_block = 2989734949248052559;
                    }
                }
                101 => {
                    if modifier == 'E' as i32 {
                        current_block = 8263860476293076988;
                    } else {
                        digits = 2 as libc::c_int;
                        number_value = (*tp).tm_mday;
                        current_block = 7615250673885775401;
                    }
                }
                70 => {
                    if modifier != 0 as libc::c_int {
                        current_block = 8263860476293076988;
                    } else {
                        if pad as libc::c_uint == ZERO_PAD as libc::c_int as libc::c_uint
                            && width < 0 as libc::c_int
                        {
                            pad = SIGN_PAD;
                            subwidth = 4 as libc::c_int;
                        } else {
                            subwidth = width - 6 as libc::c_int;
                            if subwidth < 0 as libc::c_int {
                                subwidth = 0 as libc::c_int;
                            }
                        }
                        subfmt = b"%Y-%m-%d\0" as *const u8 as *const libc::c_char;
                        current_block = 6559734051898665361;
                    }
                }
                72 => {
                    if modifier == 'E' as i32 {
                        current_block = 8263860476293076988;
                    } else {
                        digits = 2 as libc::c_int;
                        number_value = (*tp).tm_hour;
                        current_block = 2989734949248052559;
                    }
                }
                73 => {
                    if modifier == 'E' as i32 {
                        current_block = 8263860476293076988;
                    } else {
                        digits = 2 as libc::c_int;
                        number_value = hour12;
                        current_block = 2989734949248052559;
                    }
                }
                107 => {
                    if modifier == 'E' as i32 {
                        current_block = 8263860476293076988;
                    } else {
                        digits = 2 as libc::c_int;
                        number_value = (*tp).tm_hour;
                        current_block = 7615250673885775401;
                    }
                }
                108 => {
                    if modifier == 'E' as i32 {
                        current_block = 8263860476293076988;
                    } else {
                        digits = 2 as libc::c_int;
                        number_value = hour12;
                        current_block = 7615250673885775401;
                    }
                }
                106 => {
                    if modifier == 'E' as i32 {
                        current_block = 8263860476293076988;
                    } else {
                        digits = 3 as libc::c_int;
                        negative_number = (*tp).tm_yday < -(1 as libc::c_int);
                        u_number_value = ((*tp).tm_yday as libc::c_uint)
                            .wrapping_add(1 as libc::c_uint);
                        current_block = 11258798127552131746;
                    }
                }
                77 => {
                    if modifier == 'E' as i32 {
                        current_block = 8263860476293076988;
                    } else {
                        digits = 2 as libc::c_int;
                        number_value = (*tp).tm_min;
                        current_block = 2989734949248052559;
                    }
                }
                109 => {
                    if modifier == 'E' as i32 {
                        current_block = 8263860476293076988;
                    } else {
                        digits = 2 as libc::c_int;
                        negative_number = (*tp).tm_mon < -(1 as libc::c_int);
                        u_number_value = ((*tp).tm_mon as libc::c_uint)
                            .wrapping_add(1 as libc::c_uint);
                        current_block = 11258798127552131746;
                    }
                }
                78 => {
                    if modifier == 'E' as i32 {
                        current_block = 8263860476293076988;
                    } else {
                        let mut n: libc::c_int = ns;
                        let mut ns_digits: libc::c_int = 9 as libc::c_int;
                        if width <= 0 as libc::c_int {
                            width = ns_digits;
                        }
                        let mut ndigs: libc::c_int = ns_digits;
                        while width < ndigs
                            || (1 as libc::c_int) < ndigs
                                && n % 10 as libc::c_int == 0 as libc::c_int
                        {
                            ndigs -= 1;
                            ndigs;
                            n /= 10 as libc::c_int;
                        }
                        let mut j: libc::c_int = ndigs;
                        while (0 as libc::c_int) < j {
                            buf[(j - 1 as libc::c_int)
                                as usize] = (n % 10 as libc::c_int + '0' as i32)
                                as libc::c_char;
                            n /= 10 as libc::c_int;
                            j -= 1;
                            j;
                        }
                        if pad as libc::c_uint == ZERO_PAD as libc::c_int as libc::c_uint
                        {
                            pad = ALWAYS_ZERO_PAD;
                        }
                        let mut _n_5: size_t = ndigs as size_t;
                        let mut _w_5: size_t = (if pad as libc::c_uint
                            == NO_PAD as libc::c_int as libc::c_uint
                            || (0 as libc::c_int) < 0 as libc::c_int
                        {
                            0 as libc::c_int
                        } else {
                            0 as libc::c_int
                        }) as size_t;
                        let mut _incr_5: size_t = if _n_5 < _w_5 { _w_5 } else { _n_5 };
                        if _incr_5 >= maxsize.wrapping_sub(i) {
                            *__errno_location() = 34 as libc::c_int;
                            return 0 as libc::c_int as size_t;
                        }
                        if !p.is_null() {
                            if _n_5 < _w_5 {
                                let mut _delta_5: size_t = _w_5.wrapping_sub(_n_5);
                                if pad as libc::c_uint
                                    == ALWAYS_ZERO_PAD as libc::c_int as libc::c_uint
                                    || pad as libc::c_uint
                                        == SIGN_PAD as libc::c_int as libc::c_uint
                                {
                                    memset(p as *mut libc::c_void, '0' as i32, _delta_5);
                                    p = p.offset(_delta_5 as isize);
                                } else {
                                    memset(p as *mut libc::c_void, ' ' as i32, _delta_5);
                                    p = p.offset(_delta_5 as isize);
                                }
                            }
                            if to_lowcase {
                                memcpy_lowcase(p, buf.as_mut_ptr(), _n_5);
                            } else if to_uppcase {
                                memcpy_uppcase(p, buf.as_mut_ptr(), _n_5);
                            } else {
                                memcpy(
                                    p as *mut libc::c_void,
                                    buf.as_mut_ptr() as *const libc::c_void,
                                    _n_5,
                                );
                            }
                            p = p.offset(_n_5 as isize);
                        }
                        i = (i as libc::c_ulong).wrapping_add(_incr_5) as size_t
                            as size_t;
                        let mut _n_6: size_t = 0 as libc::c_int as size_t;
                        let mut _w_6: size_t = (if pad as libc::c_uint
                            == NO_PAD as libc::c_int as libc::c_uint
                            || width - ndigs < 0 as libc::c_int
                        {
                            0 as libc::c_int
                        } else {
                            width - ndigs
                        }) as size_t;
                        let mut _incr_6: size_t = if _n_6 < _w_6 { _w_6 } else { _n_6 };
                        if _incr_6 >= maxsize.wrapping_sub(i) {
                            *__errno_location() = 34 as libc::c_int;
                            return 0 as libc::c_int as size_t;
                        }
                        if !p.is_null() {
                            if _n_6 < _w_6 {
                                let mut _delta_6: size_t = _w_6.wrapping_sub(_n_6);
                                if pad as libc::c_uint
                                    == ALWAYS_ZERO_PAD as libc::c_int as libc::c_uint
                                    || pad as libc::c_uint
                                        == SIGN_PAD as libc::c_int as libc::c_uint
                                {
                                    memset(p as *mut libc::c_void, '0' as i32, _delta_6);
                                    p = p.offset(_delta_6 as isize);
                                } else {
                                    memset(p as *mut libc::c_void, ' ' as i32, _delta_6);
                                    p = p.offset(_delta_6 as isize);
                                }
                            }
                            p = p.offset(_n_6 as isize);
                        }
                        i = (i as libc::c_ulong).wrapping_add(_incr_6) as size_t
                            as size_t;
                        current_block = 3276175668257526147;
                    }
                }
                110 => {
                    let mut _n_7: size_t = 1 as libc::c_int as size_t;
                    let mut _w_7: size_t = (if pad as libc::c_uint
                        == NO_PAD as libc::c_int as libc::c_uint
                        || width < 0 as libc::c_int
                    {
                        0 as libc::c_int
                    } else {
                        width
                    }) as size_t;
                    let mut _incr_7: size_t = if _n_7 < _w_7 { _w_7 } else { _n_7 };
                    if _incr_7 >= maxsize.wrapping_sub(i) {
                        *__errno_location() = 34 as libc::c_int;
                        return 0 as libc::c_int as size_t;
                    }
                    if !p.is_null() {
                        if _n_7 < _w_7 {
                            let mut _delta_7: size_t = _w_7.wrapping_sub(_n_7);
                            if pad as libc::c_uint
                                == ALWAYS_ZERO_PAD as libc::c_int as libc::c_uint
                                || pad as libc::c_uint
                                    == SIGN_PAD as libc::c_int as libc::c_uint
                            {
                                memset(p as *mut libc::c_void, '0' as i32, _delta_7);
                                p = p.offset(_delta_7 as isize);
                            } else {
                                memset(p as *mut libc::c_void, ' ' as i32, _delta_7);
                                p = p.offset(_delta_7 as isize);
                            }
                        }
                        *p = '\n' as i32 as libc::c_char;
                        p = p.offset(_n_7 as isize);
                    }
                    i = (i as libc::c_ulong).wrapping_add(_incr_7) as size_t as size_t;
                    current_block = 3276175668257526147;
                }
                80 => {
                    to_lowcase = 1 as libc::c_int != 0;
                    format_char = 'p' as i32;
                    current_block = 11808183273756294017;
                }
                112 => {
                    current_block = 11808183273756294017;
                }
                113 => {
                    digits = 1 as libc::c_int;
                    negative_number = 0 as libc::c_int != 0;
                    u_number_value = (((*tp).tm_mon * 11 as libc::c_int
                        >> 5 as libc::c_int) + 1 as libc::c_int) as libc::c_uint;
                    current_block = 11258798127552131746;
                }
                82 => {
                    subfmt = b"%H:%M\0" as *const u8 as *const libc::c_char;
                    current_block = 15577625217767911782;
                }
                114 => {
                    current_block = 11183847479862768785;
                }
                83 => {
                    if modifier == 'E' as i32 {
                        current_block = 8263860476293076988;
                    } else {
                        digits = 2 as libc::c_int;
                        number_value = (*tp).tm_sec;
                        current_block = 2989734949248052559;
                    }
                }
                115 => {
                    let mut ltm: tm = tm {
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
                    let mut t: time_t = 0;
                    ltm = *tp;
                    ltm.tm_yday = -(1 as libc::c_int);
                    t = mktime_z(tz, &mut ltm);
                    if ltm.tm_yday < 0 as libc::c_int {
                        *__errno_location() = 75 as libc::c_int;
                        return 0 as libc::c_int as size_t;
                    }
                    bufp = buf
                        .as_mut_ptr()
                        .offset(
                            (::core::mem::size_of::<[libc::c_char; 23]>()
                                as libc::c_ulong)
                                .wrapping_div(
                                    ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                ) as isize,
                        );
                    negative_number = t < 0 as libc::c_int as libc::c_long;
                    loop {
                        let mut d: libc::c_int = (t % 10 as libc::c_int as libc::c_long)
                            as libc::c_int;
                        t /= 10 as libc::c_int as libc::c_long;
                        bufp = bufp.offset(-1);
                        *bufp = ((if negative_number as libc::c_int != 0 {
                            -d
                        } else {
                            d
                        }) + '0' as i32) as libc::c_char;
                        if !(t != 0 as libc::c_int as libc::c_long) {
                            break;
                        }
                    }
                    digits = 1 as libc::c_int;
                    always_output_a_sign = 0 as libc::c_int != 0;
                    current_block = 3210443151502082791;
                }
                88 => {
                    if modifier == 'O' as i32 {
                        current_block = 8263860476293076988;
                    } else {
                        current_block = 11183847479862768785;
                    }
                }
                84 => {
                    subfmt = b"%H:%M:%S\0" as *const u8 as *const libc::c_char;
                    current_block = 15577625217767911782;
                }
                116 => {
                    let mut _n_8: size_t = 1 as libc::c_int as size_t;
                    let mut _w_8: size_t = (if pad as libc::c_uint
                        == NO_PAD as libc::c_int as libc::c_uint
                        || width < 0 as libc::c_int
                    {
                        0 as libc::c_int
                    } else {
                        width
                    }) as size_t;
                    let mut _incr_8: size_t = if _n_8 < _w_8 { _w_8 } else { _n_8 };
                    if _incr_8 >= maxsize.wrapping_sub(i) {
                        *__errno_location() = 34 as libc::c_int;
                        return 0 as libc::c_int as size_t;
                    }
                    if !p.is_null() {
                        if _n_8 < _w_8 {
                            let mut _delta_8: size_t = _w_8.wrapping_sub(_n_8);
                            if pad as libc::c_uint
                                == ALWAYS_ZERO_PAD as libc::c_int as libc::c_uint
                                || pad as libc::c_uint
                                    == SIGN_PAD as libc::c_int as libc::c_uint
                            {
                                memset(p as *mut libc::c_void, '0' as i32, _delta_8);
                                p = p.offset(_delta_8 as isize);
                            } else {
                                memset(p as *mut libc::c_void, ' ' as i32, _delta_8);
                                p = p.offset(_delta_8 as isize);
                            }
                        }
                        *p = '\t' as i32 as libc::c_char;
                        p = p.offset(_n_8 as isize);
                    }
                    i = (i as libc::c_ulong).wrapping_add(_incr_8) as size_t as size_t;
                    current_block = 3276175668257526147;
                }
                117 => {
                    digits = 1 as libc::c_int;
                    number_value = ((*tp).tm_wday - 1 as libc::c_int + 7 as libc::c_int)
                        % 7 as libc::c_int + 1 as libc::c_int;
                    current_block = 2989734949248052559;
                }
                85 => {
                    if modifier == 'E' as i32 {
                        current_block = 8263860476293076988;
                    } else {
                        digits = 2 as libc::c_int;
                        number_value = ((*tp).tm_yday - (*tp).tm_wday + 7 as libc::c_int)
                            / 7 as libc::c_int;
                        current_block = 2989734949248052559;
                    }
                }
                86 | 103 | 71 => {
                    if modifier == 'E' as i32 {
                        current_block = 8263860476293076988;
                    } else {
                        let mut year: libc::c_int = (*tp).tm_year
                            + (if (*tp).tm_year < 0 as libc::c_int {
                                1900 as libc::c_int % 400 as libc::c_int
                            } else {
                                1900 as libc::c_int % 400 as libc::c_int
                                    - 400 as libc::c_int
                            });
                        let mut year_adjust: libc::c_int = 0 as libc::c_int;
                        let mut days: libc::c_int = iso_week_days(
                            (*tp).tm_yday,
                            (*tp).tm_wday,
                        );
                        if days < 0 as libc::c_int {
                            year_adjust = -(1 as libc::c_int);
                            days = iso_week_days(
                                (*tp).tm_yday
                                    + (365 as libc::c_int
                                        + ((year - 1 as libc::c_int) % 4 as libc::c_int
                                            == 0 as libc::c_int
                                            && ((year - 1 as libc::c_int) % 100 as libc::c_int
                                                != 0 as libc::c_int
                                                || (year - 1 as libc::c_int) % 400 as libc::c_int
                                                    == 0 as libc::c_int)) as libc::c_int),
                                (*tp).tm_wday,
                            );
                        } else {
                            let mut d_0: libc::c_int = iso_week_days(
                                (*tp).tm_yday
                                    - (365 as libc::c_int
                                        + (year % 4 as libc::c_int == 0 as libc::c_int
                                            && (year % 100 as libc::c_int != 0 as libc::c_int
                                                || year % 400 as libc::c_int == 0 as libc::c_int))
                                            as libc::c_int),
                                (*tp).tm_wday,
                            );
                            if 0 as libc::c_int <= d_0 {
                                year_adjust = 1 as libc::c_int;
                                days = d_0;
                            }
                        }
                        match *f as libc::c_int {
                            103 => {
                                let mut yy: libc::c_int = ((*tp).tm_year
                                    % 100 as libc::c_int + year_adjust) % 100 as libc::c_int;
                                digits = 2 as libc::c_int;
                                negative_number = 0 as libc::c_int != 0;
                                u_number_value = (if 0 as libc::c_int <= yy {
                                    yy
                                } else if (*tp).tm_year
                                    < -(1900 as libc::c_int) - year_adjust
                                {
                                    -yy
                                } else {
                                    yy + 100 as libc::c_int
                                }) as libc::c_uint;
                                current_block = 15989891087299139605;
                            }
                            71 => {
                                digits = 4 as libc::c_int;
                                negative_number = (*tp).tm_year
                                    < -(1900 as libc::c_int) - year_adjust;
                                u_number_value = ((*tp).tm_year as libc::c_uint)
                                    .wrapping_add(1900 as libc::c_int as libc::c_uint)
                                    .wrapping_add(year_adjust as libc::c_uint);
                                current_block = 15989891087299139605;
                            }
                            _ => {
                                digits = 2 as libc::c_int;
                                number_value = days / 7 as libc::c_int + 1 as libc::c_int;
                                current_block = 2989734949248052559;
                            }
                        }
                    }
                }
                87 => {
                    if modifier == 'E' as i32 {
                        current_block = 8263860476293076988;
                    } else {
                        digits = 2 as libc::c_int;
                        number_value = ((*tp).tm_yday
                            - ((*tp).tm_wday - 1 as libc::c_int + 7 as libc::c_int)
                                % 7 as libc::c_int + 7 as libc::c_int) / 7 as libc::c_int;
                        current_block = 2989734949248052559;
                    }
                }
                119 => {
                    if modifier == 'E' as i32 {
                        current_block = 8263860476293076988;
                    } else {
                        digits = 1 as libc::c_int;
                        number_value = (*tp).tm_wday;
                        current_block = 2989734949248052559;
                    }
                }
                89 => {
                    if modifier == 'E' as i32 {
                        current_block = 11183847479862768785;
                    } else if modifier == 'O' as i32 {
                        current_block = 8263860476293076988;
                    } else {
                        digits = 4 as libc::c_int;
                        negative_number = (*tp).tm_year < -(1900 as libc::c_int);
                        u_number_value = ((*tp).tm_year as libc::c_uint)
                            .wrapping_add(1900 as libc::c_int as libc::c_uint);
                        current_block = 15989891087299139605;
                    }
                }
                121 => {
                    if modifier == 'E' as i32 {
                        current_block = 11183847479862768785;
                    } else {
                        let mut yy_0: libc::c_int = (*tp).tm_year % 100 as libc::c_int;
                        if yy_0 < 0 as libc::c_int {
                            yy_0 = if (*tp).tm_year < -(1900 as libc::c_int) {
                                -yy_0
                            } else {
                                yy_0 + 100 as libc::c_int
                            };
                        }
                        digits = 2 as libc::c_int;
                        negative_number = 0 as libc::c_int != 0;
                        u_number_value = yy_0 as libc::c_uint;
                        current_block = 15989891087299139605;
                    }
                }
                90 => {
                    if change_case {
                        to_uppcase = 0 as libc::c_int != 0;
                        to_lowcase = 1 as libc::c_int != 0;
                    }
                    let mut _n_9: size_t = strlen(zone);
                    let mut _w_9: size_t = (if pad as libc::c_uint
                        == NO_PAD as libc::c_int as libc::c_uint
                        || width < 0 as libc::c_int
                    {
                        0 as libc::c_int
                    } else {
                        width
                    }) as size_t;
                    let mut _incr_9: size_t = if _n_9 < _w_9 { _w_9 } else { _n_9 };
                    if _incr_9 >= maxsize.wrapping_sub(i) {
                        *__errno_location() = 34 as libc::c_int;
                        return 0 as libc::c_int as size_t;
                    }
                    if !p.is_null() {
                        if _n_9 < _w_9 {
                            let mut _delta_9: size_t = _w_9.wrapping_sub(_n_9);
                            if pad as libc::c_uint
                                == ALWAYS_ZERO_PAD as libc::c_int as libc::c_uint
                                || pad as libc::c_uint
                                    == SIGN_PAD as libc::c_int as libc::c_uint
                            {
                                memset(p as *mut libc::c_void, '0' as i32, _delta_9);
                                p = p.offset(_delta_9 as isize);
                            } else {
                                memset(p as *mut libc::c_void, ' ' as i32, _delta_9);
                                p = p.offset(_delta_9 as isize);
                            }
                        }
                        if to_lowcase {
                            memcpy_lowcase(p, zone, _n_9);
                        } else if to_uppcase {
                            memcpy_uppcase(p, zone, _n_9);
                        } else {
                            memcpy(
                                p as *mut libc::c_void,
                                zone as *const libc::c_void,
                                _n_9,
                            );
                        }
                        p = p.offset(_n_9 as isize);
                    }
                    i = (i as libc::c_ulong).wrapping_add(_incr_9) as size_t as size_t;
                    current_block = 3276175668257526147;
                }
                58 => {
                    colons = 1 as libc::c_int as size_t;
                    while *f.offset(colons as isize) as libc::c_int == ':' as i32 {
                        colons = colons.wrapping_add(1);
                        colons;
                    }
                    if *f.offset(colons as isize) as libc::c_int != 'z' as i32 {
                        current_block = 8263860476293076988;
                    } else {
                        f = f.offset(colons as isize);
                        current_block = 1479974985720799577;
                    }
                }
                122 => {
                    colons = 0 as libc::c_int as size_t;
                    current_block = 1479974985720799577;
                }
                0 => {
                    current_block = 15371827921599879935;
                }
                _ => {
                    current_block = 8263860476293076988;
                }
            }
            match current_block {
                3276175668257526147 => {}
                _ => {
                    match current_block {
                        15989891087299139605 => {
                            if pad as libc::c_uint
                                == ZERO_PAD as libc::c_int as libc::c_uint
                            {
                                pad = yr_spec;
                            }
                            always_output_a_sign = pad as libc::c_uint
                                == SIGN_PAD as libc::c_int as libc::c_uint
                                && (((if digits == 2 as libc::c_int {
                                    99 as libc::c_int
                                } else {
                                    9999 as libc::c_int
                                }) as libc::c_uint) < u_number_value || digits < width);
                            current_block = 694183399175083089;
                        }
                        1479974985720799577 => {
                            if (*tp).tm_isdst < 0 as libc::c_int {
                                current_block = 3276175668257526147;
                            } else {
                                let mut diff: libc::c_int = 0;
                                let mut hour_diff: libc::c_int = 0;
                                let mut min_diff: libc::c_int = 0;
                                let mut sec_diff: libc::c_int = 0;
                                diff = (*tp).tm_gmtoff as libc::c_int;
                                negative_number = diff < 0 as libc::c_int
                                    || diff == 0 as libc::c_int
                                        && *zone as libc::c_int == '-' as i32;
                                hour_diff = diff / 60 as libc::c_int / 60 as libc::c_int;
                                min_diff = diff / 60 as libc::c_int % 60 as libc::c_int;
                                sec_diff = diff % 60 as libc::c_int;
                                match colons {
                                    0 => {
                                        current_block = 6225579869370739012;
                                        match current_block {
                                            6225579869370739012 => {
                                                digits = 5 as libc::c_int;
                                                tz_colon_mask = 0 as libc::c_int;
                                                u_number_value = (hour_diff * 100 as libc::c_int + min_diff)
                                                    as libc::c_uint;
                                                current_block = 5865189281628542250;
                                            }
                                            7866309814085982790 => {
                                                if sec_diff != 0 as libc::c_int {
                                                    current_block = 7054538615515007887;
                                                } else if min_diff != 0 as libc::c_int {
                                                    current_block = 10471999855724930313;
                                                } else {
                                                    digits = 3 as libc::c_int;
                                                    tz_colon_mask = 0 as libc::c_int;
                                                    u_number_value = hour_diff as libc::c_uint;
                                                    current_block = 5865189281628542250;
                                                }
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            10471999855724930313 => {
                                                digits = 6 as libc::c_int;
                                                tz_colon_mask = 0o4 as libc::c_int;
                                                u_number_value = (hour_diff * 100 as libc::c_int + min_diff)
                                                    as libc::c_uint;
                                            }
                                            7054538615515007887 => {
                                                digits = 9 as libc::c_int;
                                                tz_colon_mask = 0o24 as libc::c_int;
                                                u_number_value = (hour_diff * 10000 as libc::c_int
                                                    + min_diff * 100 as libc::c_int + sec_diff) as libc::c_uint;
                                            }
                                            _ => {}
                                        }
                                        always_output_a_sign = 1 as libc::c_int != 0;
                                        current_block = 14355787296497129114;
                                    }
                                    1 => {
                                        current_block = 10471999855724930313;
                                        match current_block {
                                            6225579869370739012 => {
                                                digits = 5 as libc::c_int;
                                                tz_colon_mask = 0 as libc::c_int;
                                                u_number_value = (hour_diff * 100 as libc::c_int + min_diff)
                                                    as libc::c_uint;
                                                current_block = 5865189281628542250;
                                            }
                                            7866309814085982790 => {
                                                if sec_diff != 0 as libc::c_int {
                                                    current_block = 7054538615515007887;
                                                } else if min_diff != 0 as libc::c_int {
                                                    current_block = 10471999855724930313;
                                                } else {
                                                    digits = 3 as libc::c_int;
                                                    tz_colon_mask = 0 as libc::c_int;
                                                    u_number_value = hour_diff as libc::c_uint;
                                                    current_block = 5865189281628542250;
                                                }
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            10471999855724930313 => {
                                                digits = 6 as libc::c_int;
                                                tz_colon_mask = 0o4 as libc::c_int;
                                                u_number_value = (hour_diff * 100 as libc::c_int + min_diff)
                                                    as libc::c_uint;
                                            }
                                            7054538615515007887 => {
                                                digits = 9 as libc::c_int;
                                                tz_colon_mask = 0o24 as libc::c_int;
                                                u_number_value = (hour_diff * 10000 as libc::c_int
                                                    + min_diff * 100 as libc::c_int + sec_diff) as libc::c_uint;
                                            }
                                            _ => {}
                                        }
                                        always_output_a_sign = 1 as libc::c_int != 0;
                                        current_block = 14355787296497129114;
                                    }
                                    2 => {
                                        current_block = 7054538615515007887;
                                        match current_block {
                                            6225579869370739012 => {
                                                digits = 5 as libc::c_int;
                                                tz_colon_mask = 0 as libc::c_int;
                                                u_number_value = (hour_diff * 100 as libc::c_int + min_diff)
                                                    as libc::c_uint;
                                                current_block = 5865189281628542250;
                                            }
                                            7866309814085982790 => {
                                                if sec_diff != 0 as libc::c_int {
                                                    current_block = 7054538615515007887;
                                                } else if min_diff != 0 as libc::c_int {
                                                    current_block = 10471999855724930313;
                                                } else {
                                                    digits = 3 as libc::c_int;
                                                    tz_colon_mask = 0 as libc::c_int;
                                                    u_number_value = hour_diff as libc::c_uint;
                                                    current_block = 5865189281628542250;
                                                }
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            10471999855724930313 => {
                                                digits = 6 as libc::c_int;
                                                tz_colon_mask = 0o4 as libc::c_int;
                                                u_number_value = (hour_diff * 100 as libc::c_int + min_diff)
                                                    as libc::c_uint;
                                            }
                                            7054538615515007887 => {
                                                digits = 9 as libc::c_int;
                                                tz_colon_mask = 0o24 as libc::c_int;
                                                u_number_value = (hour_diff * 10000 as libc::c_int
                                                    + min_diff * 100 as libc::c_int + sec_diff) as libc::c_uint;
                                            }
                                            _ => {}
                                        }
                                        always_output_a_sign = 1 as libc::c_int != 0;
                                        current_block = 14355787296497129114;
                                    }
                                    3 => {
                                        current_block = 7866309814085982790;
                                        match current_block {
                                            6225579869370739012 => {
                                                digits = 5 as libc::c_int;
                                                tz_colon_mask = 0 as libc::c_int;
                                                u_number_value = (hour_diff * 100 as libc::c_int + min_diff)
                                                    as libc::c_uint;
                                                current_block = 5865189281628542250;
                                            }
                                            7866309814085982790 => {
                                                if sec_diff != 0 as libc::c_int {
                                                    current_block = 7054538615515007887;
                                                } else if min_diff != 0 as libc::c_int {
                                                    current_block = 10471999855724930313;
                                                } else {
                                                    digits = 3 as libc::c_int;
                                                    tz_colon_mask = 0 as libc::c_int;
                                                    u_number_value = hour_diff as libc::c_uint;
                                                    current_block = 5865189281628542250;
                                                }
                                            }
                                            _ => {}
                                        }
                                        match current_block {
                                            10471999855724930313 => {
                                                digits = 6 as libc::c_int;
                                                tz_colon_mask = 0o4 as libc::c_int;
                                                u_number_value = (hour_diff * 100 as libc::c_int + min_diff)
                                                    as libc::c_uint;
                                            }
                                            7054538615515007887 => {
                                                digits = 9 as libc::c_int;
                                                tz_colon_mask = 0o24 as libc::c_int;
                                                u_number_value = (hour_diff * 10000 as libc::c_int
                                                    + min_diff * 100 as libc::c_int + sec_diff) as libc::c_uint;
                                            }
                                            _ => {}
                                        }
                                        always_output_a_sign = 1 as libc::c_int != 0;
                                        current_block = 14355787296497129114;
                                    }
                                    _ => {
                                        current_block = 8263860476293076988;
                                    }
                                }
                            }
                        }
                        11808183273756294017 => {
                            if change_case {
                                to_uppcase = 0 as libc::c_int != 0;
                                to_lowcase = 1 as libc::c_int != 0;
                            }
                            current_block = 11183847479862768785;
                        }
                        15371827921599879935 => {
                            f = f.offset(-1);
                            f;
                            current_block = 8263860476293076988;
                        }
                        7615250673885775401 => {
                            if pad as libc::c_uint
                                == ZERO_PAD as libc::c_int as libc::c_uint
                            {
                                pad = SPACE_PAD;
                            }
                            current_block = 2989734949248052559;
                        }
                        15577625217767911782 => {
                            subwidth = -(1 as libc::c_int);
                            current_block = 6559734051898665361;
                        }
                        _ => {}
                    }
                    match current_block {
                        3276175668257526147 => {}
                        _ => {
                            match current_block {
                                2989734949248052559 => {
                                    negative_number = number_value < 0 as libc::c_int;
                                    u_number_value = number_value as libc::c_uint;
                                    current_block = 11258798127552131746;
                                }
                                8263860476293076988 => {
                                    let mut _n_10: size_t = (f.offset_from(percent)
                                        as libc::c_long + 1 as libc::c_int as libc::c_long)
                                        as size_t;
                                    let mut _w_10: size_t = (if pad as libc::c_uint
                                        == NO_PAD as libc::c_int as libc::c_uint
                                        || width < 0 as libc::c_int
                                    {
                                        0 as libc::c_int
                                    } else {
                                        width
                                    }) as size_t;
                                    let mut _incr_10: size_t = if _n_10 < _w_10 {
                                        _w_10
                                    } else {
                                        _n_10
                                    };
                                    if _incr_10 >= maxsize.wrapping_sub(i) {
                                        *__errno_location() = 34 as libc::c_int;
                                        return 0 as libc::c_int as size_t;
                                    }
                                    if !p.is_null() {
                                        if _n_10 < _w_10 {
                                            let mut _delta_10: size_t = _w_10.wrapping_sub(_n_10);
                                            if pad as libc::c_uint
                                                == ALWAYS_ZERO_PAD as libc::c_int as libc::c_uint
                                                || pad as libc::c_uint
                                                    == SIGN_PAD as libc::c_int as libc::c_uint
                                            {
                                                memset(p as *mut libc::c_void, '0' as i32, _delta_10);
                                                p = p.offset(_delta_10 as isize);
                                            } else {
                                                memset(p as *mut libc::c_void, ' ' as i32, _delta_10);
                                                p = p.offset(_delta_10 as isize);
                                            }
                                        }
                                        if to_lowcase {
                                            memcpy_lowcase(p, percent, _n_10);
                                        } else if to_uppcase {
                                            memcpy_uppcase(p, percent, _n_10);
                                        } else {
                                            memcpy(
                                                p as *mut libc::c_void,
                                                percent as *const libc::c_void,
                                                _n_10,
                                            );
                                        }
                                        p = p.offset(_n_10 as isize);
                                    }
                                    i = (i as libc::c_ulong).wrapping_add(_incr_10) as size_t
                                        as size_t;
                                    current_block = 3276175668257526147;
                                }
                                6559734051898665361 => {
                                    let mut len: size_t = __strftime_internal(
                                        0 as *mut libc::c_char,
                                        -(1 as libc::c_int) as size_t,
                                        subfmt,
                                        tp,
                                        to_uppcase,
                                        pad,
                                        subwidth,
                                        tzset_called,
                                        tz,
                                        ns,
                                    );
                                    let mut _n_1: size_t = len;
                                    let mut _w_1: size_t = (if pad as libc::c_uint
                                        == NO_PAD as libc::c_int as libc::c_uint
                                        || width < 0 as libc::c_int
                                    {
                                        0 as libc::c_int
                                    } else {
                                        width
                                    }) as size_t;
                                    let mut _incr_1: size_t = if _n_1 < _w_1 {
                                        _w_1
                                    } else {
                                        _n_1
                                    };
                                    if _incr_1 >= maxsize.wrapping_sub(i) {
                                        *__errno_location() = 34 as libc::c_int;
                                        return 0 as libc::c_int as size_t;
                                    }
                                    if !p.is_null() {
                                        if _n_1 < _w_1 {
                                            let mut _delta_1: size_t = _w_1.wrapping_sub(_n_1);
                                            if pad as libc::c_uint
                                                == ALWAYS_ZERO_PAD as libc::c_int as libc::c_uint
                                                || pad as libc::c_uint
                                                    == SIGN_PAD as libc::c_int as libc::c_uint
                                            {
                                                memset(p as *mut libc::c_void, '0' as i32, _delta_1);
                                                p = p.offset(_delta_1 as isize);
                                            } else {
                                                memset(p as *mut libc::c_void, ' ' as i32, _delta_1);
                                                p = p.offset(_delta_1 as isize);
                                            }
                                        }
                                        __strftime_internal(
                                            p,
                                            maxsize.wrapping_sub(i),
                                            subfmt,
                                            tp,
                                            to_uppcase,
                                            pad,
                                            subwidth,
                                            tzset_called,
                                            tz,
                                            ns,
                                        );
                                        p = p.offset(_n_1 as isize);
                                    }
                                    i = (i as libc::c_ulong).wrapping_add(_incr_1) as size_t
                                        as size_t;
                                    current_block = 3276175668257526147;
                                }
                                _ => {}
                            }
                            match current_block {
                                3276175668257526147 => {}
                                _ => {
                                    match current_block {
                                        11258798127552131746 => {
                                            always_output_a_sign = 0 as libc::c_int != 0;
                                            current_block = 694183399175083089;
                                        }
                                        _ => {}
                                    }
                                    match current_block {
                                        694183399175083089 => {
                                            tz_colon_mask = 0 as libc::c_int;
                                            current_block = 14355787296497129114;
                                        }
                                        _ => {}
                                    }
                                    match current_block {
                                        14355787296497129114 => {
                                            if modifier == 'O' as i32 && !negative_number {
                                                current_block = 11183847479862768785;
                                            } else {
                                                bufp = buf
                                                    .as_mut_ptr()
                                                    .offset(
                                                        (::core::mem::size_of::<[libc::c_char; 23]>()
                                                            as libc::c_ulong)
                                                            .wrapping_div(
                                                                ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                                            ) as isize,
                                                    );
                                                if negative_number {
                                                    u_number_value = u_number_value.wrapping_neg();
                                                }
                                                loop {
                                                    if tz_colon_mask & 1 as libc::c_int != 0 {
                                                        bufp = bufp.offset(-1);
                                                        *bufp = ':' as i32 as libc::c_char;
                                                    }
                                                    tz_colon_mask >>= 1 as libc::c_int;
                                                    bufp = bufp.offset(-1);
                                                    *bufp = u_number_value
                                                        .wrapping_rem(10 as libc::c_int as libc::c_uint)
                                                        .wrapping_add('0' as i32 as libc::c_uint) as libc::c_char;
                                                    u_number_value = u_number_value
                                                        .wrapping_div(10 as libc::c_int as libc::c_uint);
                                                    if !(u_number_value != 0 as libc::c_int as libc::c_uint
                                                        || tz_colon_mask != 0 as libc::c_int)
                                                    {
                                                        break;
                                                    }
                                                }
                                                current_block = 3210443151502082791;
                                            }
                                        }
                                        _ => {}
                                    }
                                    match current_block {
                                        11183847479862768785 => {
                                            let mut ufmt: [libc::c_char; 5] = [0; 5];
                                            let mut u: *mut libc::c_char = ufmt.as_mut_ptr();
                                            let mut ubuf: [libc::c_char; 1024] = [0; 1024];
                                            let mut len_0: size_t = 0;
                                            let fresh11 = u;
                                            u = u.offset(1);
                                            *fresh11 = ' ' as i32 as libc::c_char;
                                            let fresh12 = u;
                                            u = u.offset(1);
                                            *fresh12 = '%' as i32 as libc::c_char;
                                            if modifier != 0 as libc::c_int {
                                                let fresh13 = u;
                                                u = u.offset(1);
                                                *fresh13 = modifier as libc::c_char;
                                            }
                                            let fresh14 = u;
                                            u = u.offset(1);
                                            *fresh14 = format_char as libc::c_char;
                                            *u = '\0' as i32 as libc::c_char;
                                            len_0 = strftime(
                                                ubuf.as_mut_ptr(),
                                                ::core::mem::size_of::<[libc::c_char; 1024]>()
                                                    as libc::c_ulong,
                                                ufmt.as_mut_ptr(),
                                                tp,
                                            );
                                            if len_0 != 0 as libc::c_int as libc::c_ulong {
                                                let mut _n_2: size_t = len_0
                                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                                                let mut _w_2: size_t = (if pad as libc::c_uint
                                                    == NO_PAD as libc::c_int as libc::c_uint
                                                    || width < 0 as libc::c_int
                                                {
                                                    0 as libc::c_int
                                                } else {
                                                    width
                                                }) as size_t;
                                                let mut _incr_2: size_t = if _n_2 < _w_2 {
                                                    _w_2
                                                } else {
                                                    _n_2
                                                };
                                                if _incr_2 >= maxsize.wrapping_sub(i) {
                                                    *__errno_location() = 34 as libc::c_int;
                                                    return 0 as libc::c_int as size_t;
                                                }
                                                if !p.is_null() {
                                                    if _n_2 < _w_2 {
                                                        let mut _delta_2: size_t = _w_2.wrapping_sub(_n_2);
                                                        if pad as libc::c_uint
                                                            == ALWAYS_ZERO_PAD as libc::c_int as libc::c_uint
                                                            || pad as libc::c_uint
                                                                == SIGN_PAD as libc::c_int as libc::c_uint
                                                        {
                                                            memset(p as *mut libc::c_void, '0' as i32, _delta_2);
                                                            p = p.offset(_delta_2 as isize);
                                                        } else {
                                                            memset(p as *mut libc::c_void, ' ' as i32, _delta_2);
                                                            p = p.offset(_delta_2 as isize);
                                                        }
                                                    }
                                                    if to_lowcase {
                                                        memcpy_lowcase(
                                                            p,
                                                            ubuf.as_mut_ptr().offset(1 as libc::c_int as isize),
                                                            _n_2,
                                                        );
                                                    } else if to_uppcase {
                                                        memcpy_uppcase(
                                                            p,
                                                            ubuf.as_mut_ptr().offset(1 as libc::c_int as isize),
                                                            _n_2,
                                                        );
                                                    } else {
                                                        memcpy(
                                                            p as *mut libc::c_void,
                                                            ubuf.as_mut_ptr().offset(1 as libc::c_int as isize)
                                                                as *const libc::c_void,
                                                            _n_2,
                                                        );
                                                    }
                                                    p = p.offset(_n_2 as isize);
                                                }
                                                i = (i as libc::c_ulong).wrapping_add(_incr_2) as size_t
                                                    as size_t;
                                            }
                                        }
                                        _ => {
                                            if pad as libc::c_uint
                                                == ZERO_PAD as libc::c_int as libc::c_uint
                                            {
                                                pad = ALWAYS_ZERO_PAD;
                                            }
                                            if width < 0 as libc::c_int {
                                                width = digits;
                                            }
                                            let mut sign_char: libc::c_char = (if negative_number
                                                as libc::c_int != 0
                                            {
                                                '-' as i32
                                            } else if always_output_a_sign as libc::c_int != 0 {
                                                '+' as i32
                                            } else {
                                                0 as libc::c_int
                                            }) as libc::c_char;
                                            let mut numlen: libc::c_int = buf
                                                .as_mut_ptr()
                                                .offset(
                                                    (::core::mem::size_of::<[libc::c_char; 23]>()
                                                        as libc::c_ulong)
                                                        .wrapping_div(
                                                            ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                                                        ) as isize,
                                                )
                                                .offset_from(bufp) as libc::c_long as libc::c_int;
                                            let mut shortage: libc::c_int = width
                                                - (sign_char != 0) as libc::c_int - numlen;
                                            let mut padding: libc::c_int = if pad as libc::c_uint
                                                == NO_PAD as libc::c_int as libc::c_uint
                                                || shortage <= 0 as libc::c_int
                                            {
                                                0 as libc::c_int
                                            } else {
                                                shortage
                                            };
                                            if sign_char != 0 {
                                                if pad as libc::c_uint
                                                    == SPACE_PAD as libc::c_int as libc::c_uint
                                                {
                                                    if !p.is_null() {
                                                        memset(
                                                            p as *mut libc::c_void,
                                                            ' ' as i32,
                                                            padding as libc::c_ulong,
                                                        );
                                                        p = p.offset(padding as isize);
                                                    }
                                                    i = (i as libc::c_ulong)
                                                        .wrapping_add(padding as libc::c_ulong) as size_t as size_t;
                                                    width -= padding;
                                                }
                                                let mut _n_3: size_t = 1 as libc::c_int as size_t;
                                                let mut _w_3: size_t = (if pad as libc::c_uint
                                                    == NO_PAD as libc::c_int as libc::c_uint
                                                    || (0 as libc::c_int) < 0 as libc::c_int
                                                {
                                                    0 as libc::c_int
                                                } else {
                                                    0 as libc::c_int
                                                }) as size_t;
                                                let mut _incr_3: size_t = if _n_3 < _w_3 {
                                                    _w_3
                                                } else {
                                                    _n_3
                                                };
                                                if _incr_3 >= maxsize.wrapping_sub(i) {
                                                    *__errno_location() = 34 as libc::c_int;
                                                    return 0 as libc::c_int as size_t;
                                                }
                                                if !p.is_null() {
                                                    if _n_3 < _w_3 {
                                                        let mut _delta_3: size_t = _w_3.wrapping_sub(_n_3);
                                                        if pad as libc::c_uint
                                                            == ALWAYS_ZERO_PAD as libc::c_int as libc::c_uint
                                                            || pad as libc::c_uint
                                                                == SIGN_PAD as libc::c_int as libc::c_uint
                                                        {
                                                            memset(p as *mut libc::c_void, '0' as i32, _delta_3);
                                                            p = p.offset(_delta_3 as isize);
                                                        } else {
                                                            memset(p as *mut libc::c_void, ' ' as i32, _delta_3);
                                                            p = p.offset(_delta_3 as isize);
                                                        }
                                                    }
                                                    *p = sign_char;
                                                    p = p.offset(_n_3 as isize);
                                                }
                                                i = (i as libc::c_ulong).wrapping_add(_incr_3) as size_t
                                                    as size_t;
                                                width -= 1;
                                                width;
                                            }
                                            let mut _n_4: size_t = numlen as size_t;
                                            let mut _w_4: size_t = (if pad as libc::c_uint
                                                == NO_PAD as libc::c_int as libc::c_uint
                                                || width < 0 as libc::c_int
                                            {
                                                0 as libc::c_int
                                            } else {
                                                width
                                            }) as size_t;
                                            let mut _incr_4: size_t = if _n_4 < _w_4 {
                                                _w_4
                                            } else {
                                                _n_4
                                            };
                                            if _incr_4 >= maxsize.wrapping_sub(i) {
                                                *__errno_location() = 34 as libc::c_int;
                                                return 0 as libc::c_int as size_t;
                                            }
                                            if !p.is_null() {
                                                if _n_4 < _w_4 {
                                                    let mut _delta_4: size_t = _w_4.wrapping_sub(_n_4);
                                                    if pad as libc::c_uint
                                                        == ALWAYS_ZERO_PAD as libc::c_int as libc::c_uint
                                                        || pad as libc::c_uint
                                                            == SIGN_PAD as libc::c_int as libc::c_uint
                                                    {
                                                        memset(p as *mut libc::c_void, '0' as i32, _delta_4);
                                                        p = p.offset(_delta_4 as isize);
                                                    } else {
                                                        memset(p as *mut libc::c_void, ' ' as i32, _delta_4);
                                                        p = p.offset(_delta_4 as isize);
                                                    }
                                                }
                                                if to_lowcase {
                                                    memcpy_lowcase(p, bufp, _n_4);
                                                } else if to_uppcase {
                                                    memcpy_uppcase(p, bufp, _n_4);
                                                } else {
                                                    memcpy(
                                                        p as *mut libc::c_void,
                                                        bufp as *const libc::c_void,
                                                        _n_4,
                                                    );
                                                }
                                                p = p.offset(_n_4 as isize);
                                            }
                                            i = (i as libc::c_ulong).wrapping_add(_incr_4) as size_t
                                                as size_t;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        width = -(1 as libc::c_int);
        f = f.offset(1);
        f;
    }
    if !p.is_null() && maxsize != 0 as libc::c_int as libc::c_ulong {
        *p = '\0' as i32 as libc::c_char;
    }
    *__errno_location() = saved_errno;
    return i;
}
unsafe extern "C" fn memcpy_uppcase(
    mut dest: *mut libc::c_char,
    mut src: *const libc::c_char,
    mut len: size_t,
) -> *mut libc::c_char {
    loop {
        let fresh15 = len;
        len = len.wrapping_sub(1);
        if !(fresh15 > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
        *dest
            .offset(
                len as isize,
            ) = toupper(*src.offset(len as isize) as libc::c_uchar as libc::c_int)
            as libc::c_char;
    }
    return dest;
}
unsafe extern "C" fn memcpy_lowcase(
    mut dest: *mut libc::c_char,
    mut src: *const libc::c_char,
    mut len: size_t,
) -> *mut libc::c_char {
    loop {
        let fresh16 = len;
        len = len.wrapping_sub(1);
        if !(fresh16 > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
        *dest
            .offset(
                len as isize,
            ) = tolower(*src.offset(len as isize) as libc::c_uchar as libc::c_int)
            as libc::c_char;
    }
    return dest;
}
#[inline]
unsafe extern "C" fn iso_week_days(
    mut yday: libc::c_int,
    mut wday: libc::c_int,
) -> libc::c_int {
    let mut big_enough_multiple_of_7: libc::c_int = (--(366 as libc::c_int)
        / 7 as libc::c_int + 2 as libc::c_int) * 7 as libc::c_int;
    return yday
        - (yday - wday + 4 as libc::c_int + big_enough_multiple_of_7) % 7 as libc::c_int
        + 4 as libc::c_int - 1 as libc::c_int;
}
