use ::libc;
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type idx_t = ptrdiff_t;
#[inline]
unsafe extern "C" fn c_isalnum(mut c: libc::c_int) -> bool {
    match c {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 97 | 98 | 99 | 100 | 101 | 102
        | 103 | 104 | 105 | 106 | 107 | 108 | 109 | 110 | 111 | 112 | 113 | 114 | 115
        | 116 | 117 | 118 | 119 | 120 | 121 | 122 | 65 | 66 | 67 | 68 | 69 | 70 | 71 | 72
        | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88
        | 89 | 90 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
#[inline]
unsafe extern "C" fn c_isalpha(mut c: libc::c_int) -> bool {
    match c {
        97 | 98 | 99 | 100 | 101 | 102 | 103 | 104 | 105 | 106 | 107 | 108 | 109 | 110
        | 111 | 112 | 113 | 114 | 115 | 116 | 117 | 118 | 119 | 120 | 121 | 122 | 65 | 66
        | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82
        | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
#[inline]
unsafe extern "C" fn c_isdigit(mut c: libc::c_int) -> bool {
    match c {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
unsafe extern "C" fn file_prefixlen(
    mut s: *const libc::c_char,
    mut len: *mut ptrdiff_t,
) -> idx_t {
    let mut n: size_t = *len as size_t;
    let mut prefixlen: idx_t = 0 as libc::c_int as idx_t;
    let mut i: idx_t = 0 as libc::c_int as idx_t;
    loop {
        if if *len < 0 as libc::c_int as libc::c_long {
            (*s.offset(i as isize) == 0) as libc::c_int
        } else {
            (i as libc::c_ulong == n) as libc::c_int
        } != 0
        {
            *len = i;
            return prefixlen;
        }
        i += 1;
        i;
        prefixlen = i;
        while ((i + 1 as libc::c_int as libc::c_long) as libc::c_ulong) < n
            && *s.offset(i as isize) as libc::c_int == '.' as i32
            && (c_isalpha(
                *s.offset((i + 1 as libc::c_int as libc::c_long) as isize) as libc::c_int,
            ) as libc::c_int != 0
                || *s.offset((i + 1 as libc::c_int as libc::c_long) as isize)
                    as libc::c_int == '~' as i32)
        {
            i += 2 as libc::c_int as libc::c_long;
            while (i as libc::c_ulong) < n
                && (c_isalnum(*s.offset(i as isize) as libc::c_int) as libc::c_int != 0
                    || *s.offset(i as isize) as libc::c_int == '~' as i32)
            {
                i += 1;
                i;
            }
        }
    };
}
unsafe extern "C" fn order(
    mut s: *const libc::c_char,
    mut pos: idx_t,
    mut len: idx_t,
) -> libc::c_int {
    if pos == len {
        return -(1 as libc::c_int);
    }
    let mut c: libc::c_uchar = *s.offset(pos as isize) as libc::c_uchar;
    if c_isdigit(c as libc::c_int) {
        return 0 as libc::c_int
    } else if c_isalpha(c as libc::c_int) {
        return c as libc::c_int
    } else if c as libc::c_int == '~' as i32 {
        return -(2 as libc::c_int)
    } else {
        return c as libc::c_int
            + (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
            + 1 as libc::c_int
    };
}
unsafe extern "C" fn verrevcmp(
    mut s1: *const libc::c_char,
    mut s1_len: idx_t,
    mut s2: *const libc::c_char,
    mut s2_len: idx_t,
) -> libc::c_int {
    let mut s1_pos: idx_t = 0 as libc::c_int as idx_t;
    let mut s2_pos: idx_t = 0 as libc::c_int as idx_t;
    while s1_pos < s1_len || s2_pos < s2_len {
        let mut first_diff: libc::c_int = 0 as libc::c_int;
        while s1_pos < s1_len && !c_isdigit(*s1.offset(s1_pos as isize) as libc::c_int)
            || s2_pos < s2_len && !c_isdigit(*s2.offset(s2_pos as isize) as libc::c_int)
        {
            let mut s1_c: libc::c_int = order(s1, s1_pos, s1_len);
            let mut s2_c: libc::c_int = order(s2, s2_pos, s2_len);
            if s1_c != s2_c {
                return s1_c - s2_c;
            }
            s1_pos += 1;
            s1_pos;
            s2_pos += 1;
            s2_pos;
        }
        while s1_pos < s1_len && *s1.offset(s1_pos as isize) as libc::c_int == '0' as i32
        {
            s1_pos += 1;
            s1_pos;
        }
        while s2_pos < s2_len && *s2.offset(s2_pos as isize) as libc::c_int == '0' as i32
        {
            s2_pos += 1;
            s2_pos;
        }
        while s1_pos < s1_len && s2_pos < s2_len
            && c_isdigit(*s1.offset(s1_pos as isize) as libc::c_int) as libc::c_int != 0
            && c_isdigit(*s2.offset(s2_pos as isize) as libc::c_int) as libc::c_int != 0
        {
            if first_diff == 0 {
                first_diff = *s1.offset(s1_pos as isize) as libc::c_int
                    - *s2.offset(s2_pos as isize) as libc::c_int;
            }
            s1_pos += 1;
            s1_pos;
            s2_pos += 1;
            s2_pos;
        }
        if s1_pos < s1_len
            && c_isdigit(*s1.offset(s1_pos as isize) as libc::c_int) as libc::c_int != 0
        {
            return 1 as libc::c_int;
        }
        if s2_pos < s2_len
            && c_isdigit(*s2.offset(s2_pos as isize) as libc::c_int) as libc::c_int != 0
        {
            return -(1 as libc::c_int);
        }
        if first_diff != 0 {
            return first_diff;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn filevercmp(
    mut s1: *const libc::c_char,
    mut s2: *const libc::c_char,
) -> libc::c_int {
    return filenvercmp(
        s1,
        -(1 as libc::c_int) as ptrdiff_t,
        s2,
        -(1 as libc::c_int) as ptrdiff_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn filenvercmp(
    mut a: *const libc::c_char,
    mut alen: ptrdiff_t,
    mut b: *const libc::c_char,
    mut blen: ptrdiff_t,
) -> libc::c_int {
    let mut aempty: bool = if alen < 0 as libc::c_int as libc::c_long {
        (*a.offset(0 as libc::c_int as isize) == 0) as libc::c_int
    } else {
        (alen == 0) as libc::c_int
    } != 0;
    let mut bempty: bool = if blen < 0 as libc::c_int as libc::c_long {
        (*b.offset(0 as libc::c_int as isize) == 0) as libc::c_int
    } else {
        (blen == 0) as libc::c_int
    } != 0;
    if aempty {
        return -(!bempty as libc::c_int);
    }
    if bempty {
        return 1 as libc::c_int;
    }
    if *a.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32 {
        if *b.offset(0 as libc::c_int as isize) as libc::c_int != '.' as i32 {
            return -(1 as libc::c_int);
        }
        let mut adot: bool = if alen < 0 as libc::c_int as libc::c_long {
            (*a.offset(1 as libc::c_int as isize) == 0) as libc::c_int
        } else {
            (alen == 1 as libc::c_int as libc::c_long) as libc::c_int
        } != 0;
        let mut bdot: bool = if blen < 0 as libc::c_int as libc::c_long {
            (*b.offset(1 as libc::c_int as isize) == 0) as libc::c_int
        } else {
            (blen == 1 as libc::c_int as libc::c_long) as libc::c_int
        } != 0;
        if adot {
            return -(!bdot as libc::c_int);
        }
        if bdot {
            return 1 as libc::c_int;
        }
        let mut adotdot: bool = *a.offset(1 as libc::c_int as isize) as libc::c_int
            == '.' as i32
            && (if alen < 0 as libc::c_int as libc::c_long {
                (*a.offset(2 as libc::c_int as isize) == 0) as libc::c_int
            } else {
                (alen == 2 as libc::c_int as libc::c_long) as libc::c_int
            }) != 0;
        let mut bdotdot: bool = *b.offset(1 as libc::c_int as isize) as libc::c_int
            == '.' as i32
            && (if blen < 0 as libc::c_int as libc::c_long {
                (*b.offset(2 as libc::c_int as isize) == 0) as libc::c_int
            } else {
                (blen == 2 as libc::c_int as libc::c_long) as libc::c_int
            }) != 0;
        if adotdot {
            return -(!bdotdot as libc::c_int);
        }
        if bdotdot {
            return 1 as libc::c_int;
        }
    } else if *b.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32 {
        return 1 as libc::c_int
    }
    let mut aprefixlen: idx_t = file_prefixlen(a, &mut alen);
    let mut bprefixlen: idx_t = file_prefixlen(b, &mut blen);
    let mut one_pass_only: bool = aprefixlen == alen && bprefixlen == blen;
    let mut result: libc::c_int = verrevcmp(a, aprefixlen, b, bprefixlen);
    return if result != 0 || one_pass_only as libc::c_int != 0 {
        result
    } else {
        verrevcmp(a, alen, b, blen)
    };
}
