#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut, unused_imports)]
#![feature(extern_types)]
use ::rust::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn putchar_unlocked(__c: libc::c_int) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn rpl_mbrtowc(
        pwc: *mut wchar_t,
        s: *const libc::c_char,
        n: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
    fn free(_: *mut libc::c_void);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn mempcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
    fn __ctype_get_mb_cur_max() -> size_t;
    fn quotearg_style(s: quoting_style, arg: *const libc::c_char) -> *mut libc::c_char;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    static mut Version: *const libc::c_char;
    fn strtoimax(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
    ) -> intmax_t;
    fn strtoumax(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
    ) -> uintmax_t;
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn gettext(__msgid: *const libc::c_char) -> *mut libc::c_char;
    fn textdomain(__domainname: *const libc::c_char) -> *mut libc::c_char;
    fn bindtextdomain(
        __domainname: *const libc::c_char,
        __dirname: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn close_stdout();
    fn version_etc(
        stream: *mut FILE,
        command_name: *const libc::c_char,
        package: *const libc::c_char,
        version: *const libc::c_char,
        _: ...
    );
    fn proper_name_lite(
        name_ascii: *const libc::c_char,
        name_utf8: *const libc::c_char,
    ) -> *const libc::c_char;
    static mut program_name: *const libc::c_char;
    fn set_program_name(argv0: *const libc::c_char);
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn cl_strtold(_: *const libc::c_char, _: *mut *mut libc::c_char) -> f128::f128;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn print_unicode_char(
        stream: *mut FILE,
        code: libc::c_uint,
        exit_on_error: libc::c_int,
    );
    fn xprintf(format: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __intmax_t = libc::c_long;
pub type __uintmax_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mbstate_t {
    pub __count: libc::c_int,
    pub __value: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __wch: libc::c_uint,
    pub __wchb: [libc::c_char; 4],
}
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
pub type ssize_t = __ssize_t;
pub type wchar_t = libc::c_uint;
pub type mbstate_t = __mbstate_t;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct infomap {
    pub program: *const libc::c_char,
    pub node: *const libc::c_char,
}
pub type quoting_style = libc::c_uint;
pub const custom_quoting_style: quoting_style = 10;
pub const clocale_quoting_style: quoting_style = 9;
pub const locale_quoting_style: quoting_style = 8;
pub const escape_quoting_style: quoting_style = 7;
pub const c_maybe_quoting_style: quoting_style = 6;
pub const c_quoting_style: quoting_style = 5;
pub const shell_escape_always_quoting_style: quoting_style = 4;
pub const shell_escape_quoting_style: quoting_style = 3;
pub const shell_always_quoting_style: quoting_style = 2;
pub const shell_quoting_style: quoting_style = 1;
pub const literal_quoting_style: quoting_style = 0;
#[inline]
unsafe extern "C" fn mbszero(mut ps: *mut mbstate_t) {
    memset(
        ps as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<mbstate_t>() as libc::c_ulong,
    );
}
#[inline]
unsafe extern "C" fn emit_ancillary_info(mut program: *const libc::c_char) {
    let infomap_0: [infomap; 7] = [
        {
            let mut init = infomap {
                program: b"[\0" as *const u8 as *const libc::c_char,
                node: b"test invocation\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = infomap {
                program: b"coreutils\0" as *const u8 as *const libc::c_char,
                node: b"Multi-call invocation\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = infomap {
                program: b"sha224sum\0" as *const u8 as *const libc::c_char,
                node: b"sha2 utilities\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = infomap {
                program: b"sha256sum\0" as *const u8 as *const libc::c_char,
                node: b"sha2 utilities\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = infomap {
                program: b"sha384sum\0" as *const u8 as *const libc::c_char,
                node: b"sha2 utilities\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = infomap {
                program: b"sha512sum\0" as *const u8 as *const libc::c_char,
                node: b"sha2 utilities\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = infomap {
                program: 0 as *const libc::c_char,
                node: 0 as *const libc::c_char,
            };
            init
        },
    ];
    let mut node: *const libc::c_char = program;
    let mut map_prog: *const infomap = infomap_0.as_ptr();
    while !((*map_prog).program).is_null()
        && !(strcmp(program, (*map_prog).program) == 0 as libc::c_int)
    {
        map_prog = map_prog.offset(1);
        map_prog;
    }
    if !((*map_prog).node).is_null() {
        node = (*map_prog).node;
    }
    printf(
        gettext(b"\n%s online help: <%s>\n\0" as *const u8 as *const libc::c_char),
        b"GNU coreutils\0" as *const u8 as *const libc::c_char,
        b"https://www.gnu.org/software/coreutils/\0" as *const u8 as *const libc::c_char,
    );
    let mut lc_messages: *const libc::c_char = setlocale(
        5 as libc::c_int,
        0 as *const libc::c_char,
    );
    if !lc_messages.is_null()
        && strncmp(
            lc_messages,
            b"en_\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) != 0
    {
        fputs_unlocked(
            gettext(
                b"Report any translation bugs to <https://translationproject.org/team/>\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
    }
    let mut url_program: *const libc::c_char = if strcmp(
        program,
        b"[\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        b"test\0" as *const u8 as *const libc::c_char
    } else {
        program
    };
    printf(
        gettext(b"Full documentation <%s%s>\n\0" as *const u8 as *const libc::c_char),
        b"https://www.gnu.org/software/coreutils/\0" as *const u8 as *const libc::c_char,
        url_program,
    );
    printf(
        gettext(
            b"or available locally via: info '(coreutils) %s%s'\n\0" as *const u8
                as *const libc::c_char,
        ),
        node,
        if node == program {
            b" invocation\0" as *const u8 as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
    );
}
#[inline]
unsafe extern "C" fn to_uchar(mut ch: libc::c_char) -> libc::c_uchar {
    return ch as libc::c_uchar;
}
#[inline]
unsafe extern "C" fn c_isxdigit(mut c: libc::c_int) -> bool {
    match c {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 97 | 98 | 99 | 100 | 101 | 102
        | 65 | 66 | 67 | 68 | 69 | 70 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
static mut exit_status: libc::c_int = 0;
static mut posixly_correct: bool = false;
static mut cfcc_msg: *const libc::c_char = b"warning: %s: character(s) following character constant have been ignored\0"
    as *const u8 as *const libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn usage(mut status: libc::c_int) {
    if status != 0 as libc::c_int {
        fprintf(
            stderr,
            gettext(
                b"Try '%s --help' for more information.\n\0" as *const u8
                    as *const libc::c_char,
            ),
            program_name,
        );
    } else {
        printf(
            gettext(
                b"Usage: %s FORMAT [ARGUMENT]...\n  or:  %s OPTION\n\0" as *const u8
                    as *const libc::c_char,
            ),
            program_name,
            program_name,
        );
        fputs_unlocked(
            gettext(
                b"Print ARGUMENT(s) according to FORMAT, or execute according to OPTION:\n\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"      --help        display this help and exit\n\0" as *const u8
                    as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"      --version     output version information and exit\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"\nFORMAT controls the output as in C printf.  Interpreted sequences are:\n\n  \\\"      double quote\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"  \\\\      backslash\n  \\a      alert (BEL)\n  \\b      backspace\n  \\c      produce no further output\n  \\e      escape\n  \\f      form feed\n  \\n      new line\n  \\r      carriage return\n  \\t      horizontal tab\n  \\v      vertical tab\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"  \\NNN    byte with octal value NNN (1 to 3 digits)\n  \\xHH    byte with hexadecimal value HH (1 to 2 digits)\n  \\uHHHH  Unicode (ISO/IEC 10646) character with hex value HHHH (4 digits)\n  \\UHHHHHHHH  Unicode character with hex value HHHHHHHH (8 digits)\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"  %%      a single %\n  %b      ARGUMENT as a string with '\\' escapes interpreted,\n          except that octal escapes are of the form \\0 or \\0NNN\n  %q      ARGUMENT is printed in a format that can be reused as shell input,\n          escaping non-printable characters with the proposed POSIX $'' syntax.\n\nand all C format specifications ending with one of diouxXfeEgGcs, with\nARGUMENTs converted to proper type first.  Variable widths are handled.\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        printf(
            gettext(
                b"\nYour shell may have its own version of %s, which usually supersedes\nthe version described here.  Please refer to your shell's documentation\nfor details about the options it supports.\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            b"printf\0" as *const u8 as *const libc::c_char,
        );
        emit_ancillary_info(b"printf\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
}
unsafe extern "C" fn verify_numeric(
    mut s: *const libc::c_char,
    mut end: *const libc::c_char,
) {
    if *__errno_location() != 0 {
        if 0 != 0 {
            error(
                0 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                quote(s),
            );
            if 0 as libc::c_int != 0 as libc::c_int {
                unreachable!();
            } else {};
        } else {
            ({
                let __errstatus: libc::c_int = 0 as libc::c_int;
                error(
                    __errstatus,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quote(s),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
            ({
                let __errstatus: libc::c_int = 0 as libc::c_int;
                error(
                    __errstatus,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quote(s),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
        exit_status = 1 as libc::c_int;
    } else if *end != 0 {
        if s == end {
            if 0 != 0 {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"%s: expected a numeric value\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quote(s),
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
                            b"%s: expected a numeric value\0" as *const u8
                                as *const libc::c_char,
                        ),
                        quote(s),
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
                            b"%s: expected a numeric value\0" as *const u8
                                as *const libc::c_char,
                        ),
                        quote(s),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        } else {
            if 0 != 0 {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"%s: value not completely converted\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quote(s),
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
                            b"%s: value not completely converted\0" as *const u8
                                as *const libc::c_char,
                        ),
                        quote(s),
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
                            b"%s: value not completely converted\0" as *const u8
                                as *const libc::c_char,
                        ),
                        quote(s),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
        exit_status = 1 as libc::c_int;
    }
}
unsafe extern "C" fn vstrtoimax(mut s: *const libc::c_char) -> intmax_t {
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut val: intmax_t = 0;
    if (*s as libc::c_int == '"' as i32 || *s as libc::c_int == '\'' as i32)
        && *s.offset(1 as libc::c_int as isize) as libc::c_int != 0
    {
        s = s.offset(1);
        let mut ch: libc::c_uchar = *s as libc::c_uchar;
        val = ch as intmax_t;
        if __ctype_get_mb_cur_max() > 1 as libc::c_int as libc::c_ulong
            && *s.offset(1 as libc::c_int as isize) as libc::c_int != 0
        {
            let mut mbstate: mbstate_t = mbstate_t {
                __count: 0,
                __value: C2RustUnnamed { __wch: 0 },
            };
            mbszero(&mut mbstate);
            let mut wc: wchar_t = 0;
            let mut slen: size_t = strlen(s);
            let mut bytes: ssize_t = 0;
            bytes = rpl_mbrtowc(&mut wc, s, slen, &mut mbstate) as ssize_t;
            if (0 as libc::c_int as libc::c_long) < bytes {
                val = wc as intmax_t;
                s = s.offset((bytes - 1 as libc::c_int as libc::c_long) as isize);
            }
        }
        s = s.offset(1);
        if *s as libc::c_int != 0 as libc::c_int && !posixly_correct {
            if 0 != 0 {
                error(0 as libc::c_int, 0 as libc::c_int, gettext(cfcc_msg), s);
                if 0 as libc::c_int != 0 as libc::c_int {
                    unreachable!();
                } else {};
            } else {
                ({
                    let __errstatus: libc::c_int = 0 as libc::c_int;
                    error(__errstatus, 0 as libc::c_int, gettext(cfcc_msg), s);
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
                ({
                    let __errstatus: libc::c_int = 0 as libc::c_int;
                    error(__errstatus, 0 as libc::c_int, gettext(cfcc_msg), s);
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
    } else {
        *__errno_location() = 0 as libc::c_int;
        val = strtoimax(s, &mut end, 0 as libc::c_int);
        verify_numeric(s, end);
    }
    return val;
}
unsafe extern "C" fn vstrtoumax(mut s: *const libc::c_char) -> uintmax_t {
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut val: uintmax_t = 0;
    if (*s as libc::c_int == '"' as i32 || *s as libc::c_int == '\'' as i32)
        && *s.offset(1 as libc::c_int as isize) as libc::c_int != 0
    {
        s = s.offset(1);
        let mut ch: libc::c_uchar = *s as libc::c_uchar;
        val = ch as uintmax_t;
        if __ctype_get_mb_cur_max() > 1 as libc::c_int as libc::c_ulong
            && *s.offset(1 as libc::c_int as isize) as libc::c_int != 0
        {
            let mut mbstate: mbstate_t = mbstate_t {
                __count: 0,
                __value: C2RustUnnamed { __wch: 0 },
            };
            mbszero(&mut mbstate);
            let mut wc: wchar_t = 0;
            let mut slen: size_t = strlen(s);
            let mut bytes: ssize_t = 0;
            bytes = rpl_mbrtowc(&mut wc, s, slen, &mut mbstate) as ssize_t;
            if (0 as libc::c_int as libc::c_long) < bytes {
                val = wc as uintmax_t;
                s = s.offset((bytes - 1 as libc::c_int as libc::c_long) as isize);
            }
        }
        s = s.offset(1);
        if *s as libc::c_int != 0 as libc::c_int && !posixly_correct {
            if 0 != 0 {
                error(0 as libc::c_int, 0 as libc::c_int, gettext(cfcc_msg), s);
                if 0 as libc::c_int != 0 as libc::c_int {
                    unreachable!();
                } else {};
            } else {
                ({
                    let __errstatus: libc::c_int = 0 as libc::c_int;
                    error(__errstatus, 0 as libc::c_int, gettext(cfcc_msg), s);
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
                ({
                    let __errstatus: libc::c_int = 0 as libc::c_int;
                    error(__errstatus, 0 as libc::c_int, gettext(cfcc_msg), s);
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
    } else {
        *__errno_location() = 0 as libc::c_int;
        val = strtoumax(s, &mut end, 0 as libc::c_int);
        verify_numeric(s, end);
    }
    return val;
}
unsafe extern "C" fn vstrtold(mut s: *const libc::c_char) -> f128::f128 {
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut val: f128::f128 = f128::f128::ZERO;
    if (*s as libc::c_int == '"' as i32 || *s as libc::c_int == '\'' as i32)
        && *s.offset(1 as libc::c_int as isize) as libc::c_int != 0
    {
        s = s.offset(1);
        let mut ch: libc::c_uchar = *s as libc::c_uchar;
        val = f128::f128::new(ch);
        if __ctype_get_mb_cur_max() > 1 as libc::c_int as libc::c_ulong
            && *s.offset(1 as libc::c_int as isize) as libc::c_int != 0
        {
            let mut mbstate: mbstate_t = mbstate_t {
                __count: 0,
                __value: C2RustUnnamed { __wch: 0 },
            };
            mbszero(&mut mbstate);
            let mut wc: wchar_t = 0;
            let mut slen: size_t = strlen(s);
            let mut bytes: ssize_t = 0;
            bytes = rpl_mbrtowc(&mut wc, s, slen, &mut mbstate) as ssize_t;
            if (0 as libc::c_int as libc::c_long) < bytes {
                val = f128::f128::new(wc);
                s = s.offset((bytes - 1 as libc::c_int as libc::c_long) as isize);
            }
        }
        s = s.offset(1);
        if *s as libc::c_int != 0 as libc::c_int && !posixly_correct {
            if 0 != 0 {
                error(0 as libc::c_int, 0 as libc::c_int, gettext(cfcc_msg), s);
                if 0 as libc::c_int != 0 as libc::c_int {
                    unreachable!();
                } else {};
            } else {
                ({
                    let __errstatus: libc::c_int = 0 as libc::c_int;
                    error(__errstatus, 0 as libc::c_int, gettext(cfcc_msg), s);
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
                ({
                    let __errstatus: libc::c_int = 0 as libc::c_int;
                    error(__errstatus, 0 as libc::c_int, gettext(cfcc_msg), s);
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
    } else {
        *__errno_location() = 0 as libc::c_int;
        val = cl_strtold(s, &mut end);
        verify_numeric(s, end);
    }
    return val;
}
unsafe extern "C" fn print_esc_char(mut c: libc::c_char) {
    match c as libc::c_int {
        97 => {
            putchar_unlocked('\u{7}' as i32);
        }
        98 => {
            putchar_unlocked('\u{8}' as i32);
        }
        99 => {
            exit(0 as libc::c_int);
        }
        101 => {
            putchar_unlocked('\u{1b}' as i32);
        }
        102 => {
            putchar_unlocked('\u{c}' as i32);
        }
        110 => {
            putchar_unlocked('\n' as i32);
        }
        114 => {
            putchar_unlocked('\r' as i32);
        }
        116 => {
            putchar_unlocked('\t' as i32);
        }
        118 => {
            putchar_unlocked('\u{b}' as i32);
        }
        _ => {
            putchar_unlocked(c as libc::c_int);
        }
    };
}
unsafe extern "C" fn print_esc(
    mut escstart: *const libc::c_char,
    mut octal_0: bool,
) -> libc::c_int {
    let mut p: *const libc::c_char = escstart.offset(1 as libc::c_int as isize);
    let mut esc_value: libc::c_int = 0 as libc::c_int;
    let mut esc_length: libc::c_int = 0;
    if *p as libc::c_int == 'x' as i32 {
        esc_length = 0 as libc::c_int;
        p = p.offset(1);
        p;
        while esc_length < 2 as libc::c_int
            && c_isxdigit(to_uchar(*p) as libc::c_int) as libc::c_int != 0
        {
            esc_value = esc_value * 16 as libc::c_int
                + (if 'a' as i32 <= *p as libc::c_int && *p as libc::c_int <= 'f' as i32
                {
                    *p as libc::c_int - 'a' as i32 + 10 as libc::c_int
                } else {
                    (if 'A' as i32 <= *p as libc::c_int
                        && *p as libc::c_int <= 'F' as i32
                    {
                        *p as libc::c_int - 'A' as i32 + 10 as libc::c_int
                    } else {
                        *p as libc::c_int - '0' as i32
                    })
                });
            esc_length += 1;
            esc_length;
            p = p.offset(1);
            p;
        }
        if esc_length == 0 as libc::c_int {
            if 0 != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"missing hexadecimal number in escape\0" as *const u8
                            as *const libc::c_char,
                    ),
                );
                if 1 as libc::c_int != 0 as libc::c_int {
                    unreachable!();
                } else {};
            } else {
                ({
                    let __errstatus: libc::c_int = 1 as libc::c_int;
                    error(
                        __errstatus,
                        0 as libc::c_int,
                        gettext(
                            b"missing hexadecimal number in escape\0" as *const u8
                                as *const libc::c_char,
                        ),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
                ({
                    let __errstatus: libc::c_int = 1 as libc::c_int;
                    error(
                        __errstatus,
                        0 as libc::c_int,
                        gettext(
                            b"missing hexadecimal number in escape\0" as *const u8
                                as *const libc::c_char,
                        ),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
        putchar_unlocked(esc_value);
    } else if '0' as i32 <= *p as libc::c_int && *p as libc::c_int <= '7' as i32 {
        esc_length = 0 as libc::c_int;
        p = p
            .offset(
                (octal_0 as libc::c_int != 0 && *p as libc::c_int == '0' as i32)
                    as libc::c_int as isize,
            );
        while esc_length < 3 as libc::c_int
            && ('0' as i32 <= *p as libc::c_int && *p as libc::c_int <= '7' as i32)
        {
            esc_value = esc_value * 8 as libc::c_int + (*p as libc::c_int - '0' as i32);
            esc_length += 1;
            esc_length;
            p = p.offset(1);
            p;
        }
        putchar_unlocked(esc_value);
    } else if *p as libc::c_int != 0
        && !(strchr(
            b"\"\\abcefnrtv\0" as *const u8 as *const libc::c_char,
            *p as libc::c_int,
        ))
            .is_null()
    {
        let fresh0 = p;
        p = p.offset(1);
        print_esc_char(*fresh0);
    } else if *p as libc::c_int == 'u' as i32 || *p as libc::c_int == 'U' as i32 {
        let mut esc_char: libc::c_char = *p;
        let mut uni_value: libc::c_uint = 0;
        uni_value = 0 as libc::c_int as libc::c_uint;
        esc_length = (if esc_char as libc::c_int == 'u' as i32 {
            4 as libc::c_int
        } else {
            8 as libc::c_int
        });
        p = p.offset(1);
        p;
        while esc_length > 0 as libc::c_int {
            if !c_isxdigit(to_uchar(*p) as libc::c_int) {
                if 0 != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        gettext(
                            b"missing hexadecimal number in escape\0" as *const u8
                                as *const libc::c_char,
                        ),
                    );
                    if 1 as libc::c_int != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                } else {
                    ({
                        let __errstatus: libc::c_int = 1 as libc::c_int;
                        error(
                            __errstatus,
                            0 as libc::c_int,
                            gettext(
                                b"missing hexadecimal number in escape\0" as *const u8
                                    as *const libc::c_char,
                            ),
                        );
                        if __errstatus != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                        
                    });
                    ({
                        let __errstatus: libc::c_int = 1 as libc::c_int;
                        error(
                            __errstatus,
                            0 as libc::c_int,
                            gettext(
                                b"missing hexadecimal number in escape\0" as *const u8
                                    as *const libc::c_char,
                            ),
                        );
                        if __errstatus != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                        
                    });
                };
            }
            uni_value = uni_value
                .wrapping_mul(16 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    (if 'a' as i32 <= *p as libc::c_int
                        && *p as libc::c_int <= 'f' as i32
                    {
                        *p as libc::c_int - 'a' as i32 + 10 as libc::c_int
                    } else {
                        (if 'A' as i32 <= *p as libc::c_int
                            && *p as libc::c_int <= 'F' as i32
                        {
                            *p as libc::c_int - 'A' as i32 + 10 as libc::c_int
                        } else {
                            *p as libc::c_int - '0' as i32
                        })
                    }) as libc::c_uint,
                );
            esc_length -= 1;
            esc_length;
            p = p.offset(1);
            p;
        }
        if uni_value >= 0xd800 as libc::c_int as libc::c_uint
            && uni_value <= 0xdfff as libc::c_int as libc::c_uint
        {
            if 0 != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"invalid universal character name \\%c%0*x\0" as *const u8
                            as *const libc::c_char,
                    ),
                    esc_char as libc::c_int,
                    (if esc_char as libc::c_int == 'u' as i32 {
                        4 as libc::c_int
                    } else {
                        8 as libc::c_int
                    }),
                    uni_value,
                );
                if 1 as libc::c_int != 0 as libc::c_int {
                    unreachable!();
                } else {};
            } else {
                ({
                    let __errstatus: libc::c_int = 1 as libc::c_int;
                    error(
                        __errstatus,
                        0 as libc::c_int,
                        gettext(
                            b"invalid universal character name \\%c%0*x\0" as *const u8
                                as *const libc::c_char,
                        ),
                        esc_char as libc::c_int,
                        (if esc_char as libc::c_int == 'u' as i32 {
                            4 as libc::c_int
                        } else {
                            8 as libc::c_int
                        }),
                        uni_value,
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
                ({
                    let __errstatus: libc::c_int = 1 as libc::c_int;
                    error(
                        __errstatus,
                        0 as libc::c_int,
                        gettext(
                            b"invalid universal character name \\%c%0*x\0" as *const u8
                                as *const libc::c_char,
                        ),
                        esc_char as libc::c_int,
                        (if esc_char as libc::c_int == 'u' as i32 {
                            4 as libc::c_int
                        } else {
                            8 as libc::c_int
                        }),
                        uni_value,
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
        print_unicode_char(stdout, uni_value, 0 as libc::c_int);
    } else {
        putchar_unlocked('\\' as i32);
        if *p != 0 {
            putchar_unlocked(*p as libc::c_int);
            p = p.offset(1);
            p;
        }
    }
    return (p.offset_from(escstart) as libc::c_long - 1 as libc::c_int as libc::c_long)
        as libc::c_int;
}
unsafe extern "C" fn print_esc_string(mut str: *const libc::c_char) {
    while *str != 0 {
        if *str as libc::c_int == '\\' as i32 {
            str = str.offset(print_esc(str, 1 as libc::c_int != 0) as isize);
        } else {
            putchar_unlocked(*str as libc::c_int);
        }
        str = str.offset(1);
        str;
    }
}
unsafe extern "C" fn print_direc(
    mut start: *const libc::c_char,
    mut length: size_t,
    mut conversion: libc::c_char,
    mut have_field_width: bool,
    mut field_width: libc::c_int,
    mut have_precision: bool,
    mut precision: libc::c_int,
    mut argument: *const libc::c_char,
) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut length_modifier: *const libc::c_char = 0 as *const libc::c_char;
    let mut length_modifier_len: size_t = 0;
    match conversion as libc::c_int {
        100 | 105 | 111 | 117 | 120 | 88 => {
            length_modifier = b"j\0" as *const u8 as *const libc::c_char;
            length_modifier_len = 1 as libc::c_int as size_t;
        }
        97 | 101 | 102 | 103 | 65 | 69 | 70 | 71 => {
            length_modifier = b"L\0" as *const u8 as *const libc::c_char;
            length_modifier_len = 1 as libc::c_int as size_t;
        }
        _ => {
            length_modifier = start;
            length_modifier_len = 0 as libc::c_int as size_t;
        }
    }
    p = xmalloc(
        length
            .wrapping_add(length_modifier_len)
            .wrapping_add(2 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    q = mempcpy(p as *mut libc::c_void, start as *const libc::c_void, length)
        as *mut libc::c_char;
    q = mempcpy(
        q as *mut libc::c_void,
        length_modifier as *const libc::c_void,
        length_modifier_len,
    ) as *mut libc::c_char;
    let fresh1 = q;
    q = q.offset(1);
    *fresh1 = conversion;
    *q = '\0' as i32 as libc::c_char;
    match conversion as libc::c_int {
        100 | 105 => {
            let mut arg: intmax_t = vstrtoimax(argument);
            if !have_field_width {
                if !have_precision {
                    xprintf(p, arg);
                } else {
                    xprintf(p, precision, arg);
                }
            } else if !have_precision {
                xprintf(p, field_width, arg);
            } else {
                xprintf(p, field_width, precision, arg);
            }
        }
        111 | 117 | 120 | 88 => {
            let mut arg_0: uintmax_t = vstrtoumax(argument);
            if !have_field_width {
                if !have_precision {
                    xprintf(p, arg_0);
                } else {
                    xprintf(p, precision, arg_0);
                }
            } else if !have_precision {
                xprintf(p, field_width, arg_0);
            } else {
                xprintf(p, field_width, precision, arg_0);
            }
        }
        97 | 65 | 101 | 69 | 102 | 70 | 103 | 71 => {
            let mut arg_1: f128::f128 = vstrtold(argument);
            if !have_field_width {
                if !have_precision {
                    xprintf(p, arg_1);
                } else {
                    xprintf(p, precision, arg_1);
                }
            } else if !have_precision {
                xprintf(p, field_width, arg_1);
            } else {
                xprintf(p, field_width, precision, arg_1);
            }
        }
        99 => {
            if !have_field_width {
                xprintf(p, *argument as libc::c_int);
            } else {
                xprintf(p, field_width, *argument as libc::c_int);
            }
        }
        115 => {
            if !have_field_width {
                if !have_precision {
                    xprintf(p, argument);
                } else {
                    xprintf(p, precision, argument);
                }
            } else if !have_precision {
                xprintf(p, field_width, argument);
            } else {
                xprintf(p, field_width, precision, argument);
            }
        }
        _ => {}
    }
    free(p as *mut libc::c_void);
}
unsafe extern "C" fn print_formatted(
    mut format: *const libc::c_char,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut save_argc: libc::c_int = argc;
    let mut f: *const libc::c_char = 0 as *const libc::c_char;
    let mut direc_start: *const libc::c_char = 0 as *const libc::c_char;
    let mut direc_length: size_t = 0;
    let mut have_field_width: bool = false;
    let mut field_width: libc::c_int = 0 as libc::c_int;
    let mut have_precision: bool = false;
    let mut precision: libc::c_int = 0 as libc::c_int;
    let mut ok: [libc::c_char; 256] = [0; 256];
    f = format;
    while *f != 0 {
        match *f as libc::c_int {
            37 => {
                let fresh2 = f;
                f = f.offset(1);
                direc_start = fresh2;
                direc_length = 1 as libc::c_int as size_t;
                have_precision = 0 as libc::c_int != 0;
                have_field_width = have_precision;
                if *f as libc::c_int == '%' as i32 {
                    putchar_unlocked('%' as i32);
                } else if *f as libc::c_int == 'b' as i32 {
                    if argc > 0 as libc::c_int {
                        print_esc_string(*argv);
                        argv = argv.offset(1);
                        argv;
                        argc -= 1;
                        argc;
                    }
                } else if *f as libc::c_int == 'q' as i32 {
                    if argc > 0 as libc::c_int {
                        fputs_unlocked(
                            quotearg_style(shell_escape_quoting_style, *argv),
                            stdout,
                        );
                        argv = argv.offset(1);
                        argv;
                        argc -= 1;
                        argc;
                    }
                } else {
                    memset(
                        ok.as_mut_ptr() as *mut libc::c_void,
                        0 as libc::c_int,
                        ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                    );
                    ok['X' as i32 as usize] = 1 as libc::c_int as libc::c_char;
                    ok['x' as i32 as usize] = ok['X' as i32 as usize];
                    ok['u' as i32 as usize] = ok['x' as i32 as usize];
                    ok['s' as i32 as usize] = ok['u' as i32 as usize];
                    ok['o' as i32 as usize] = ok['s' as i32 as usize];
                    ok['i' as i32 as usize] = ok['o' as i32 as usize];
                    ok['G' as i32 as usize] = ok['i' as i32 as usize];
                    ok['g' as i32 as usize] = ok['G' as i32 as usize];
                    ok['F' as i32 as usize] = ok['g' as i32 as usize];
                    ok['f' as i32 as usize] = ok['F' as i32 as usize];
                    ok['E' as i32 as usize] = ok['f' as i32 as usize];
                    ok['e' as i32 as usize] = ok['E' as i32 as usize];
                    ok['d' as i32 as usize] = ok['e' as i32 as usize];
                    ok['c' as i32 as usize] = ok['d' as i32 as usize];
                    ok['A' as i32 as usize] = ok['c' as i32 as usize];
                    ok['a' as i32 as usize] = ok['A' as i32 as usize];
                    loop {
                        match *f as libc::c_int {
                            73 | 39 => {
                                ok['X' as i32 as usize] = 0 as libc::c_int as libc::c_char;
                                ok['x' as i32 as usize] = ok['X' as i32 as usize];
                                ok['s' as i32 as usize] = ok['x' as i32 as usize];
                                ok['o' as i32 as usize] = ok['s' as i32 as usize];
                                ok['E' as i32 as usize] = ok['o' as i32 as usize];
                                ok['e' as i32 as usize] = ok['E' as i32 as usize];
                                ok['c' as i32 as usize] = ok['e' as i32 as usize];
                                ok['A' as i32 as usize] = ok['c' as i32 as usize];
                                ok['a' as i32 as usize] = ok['A' as i32 as usize];
                            }
                            45 | 43 | 32 => {}
                            35 => {
                                ok['u' as i32 as usize] = 0 as libc::c_int as libc::c_char;
                                ok['s' as i32 as usize] = ok['u' as i32 as usize];
                                ok['i' as i32 as usize] = ok['s' as i32 as usize];
                                ok['d' as i32 as usize] = ok['i' as i32 as usize];
                                ok['c' as i32 as usize] = ok['d' as i32 as usize];
                            }
                            48 => {
                                ok['s' as i32 as usize] = 0 as libc::c_int as libc::c_char;
                                ok['c' as i32 as usize] = ok['s' as i32 as usize];
                            }
                            _ => {
                                break;
                            }
                        }
                        f = f.offset(1);
                        f;
                        direc_length = direc_length.wrapping_add(1);
                        direc_length;
                    }
                    if *f as libc::c_int == '*' as i32 {
                        f = f.offset(1);
                        f;
                        direc_length = direc_length.wrapping_add(1);
                        direc_length;
                        if argc > 0 as libc::c_int {
                            let mut width: intmax_t = vstrtoimax(*argv);
                            if (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                                as libc::c_long <= width
                                && width <= 2147483647 as libc::c_int as libc::c_long
                            {
                                field_width = width as libc::c_int;
                            } else {
                                if 0 != 0 {
                                    error(
                                        1 as libc::c_int,
                                        0 as libc::c_int,
                                        gettext(
                                            b"invalid field width: %s\0" as *const u8
                                                as *const libc::c_char,
                                        ),
                                        quote(*argv),
                                    );
                                    if 1 as libc::c_int != 0 as libc::c_int {
                                        unreachable!();
                                    } else {};
                                } else {
                                    ({
                                        let __errstatus: libc::c_int = 1 as libc::c_int;
                                        error(
                                            __errstatus,
                                            0 as libc::c_int,
                                            gettext(
                                                b"invalid field width: %s\0" as *const u8
                                                    as *const libc::c_char,
                                            ),
                                            quote(*argv),
                                        );
                                        if __errstatus != 0 as libc::c_int {
                                            unreachable!();
                                        } else {};
                                        
                                    });
                                    ({
                                        let __errstatus: libc::c_int = 1 as libc::c_int;
                                        error(
                                            __errstatus,
                                            0 as libc::c_int,
                                            gettext(
                                                b"invalid field width: %s\0" as *const u8
                                                    as *const libc::c_char,
                                            ),
                                            quote(*argv),
                                        );
                                        if __errstatus != 0 as libc::c_int {
                                            unreachable!();
                                        } else {};
                                        
                                    });
                                };
                            }
                            argv = argv.offset(1);
                            argv;
                            argc -= 1;
                            argc;
                        } else {
                            field_width = 0 as libc::c_int;
                        }
                        have_field_width = 1 as libc::c_int != 0;
                    } else {
                        while (*f as libc::c_uint)
                            .wrapping_sub('0' as i32 as libc::c_uint)
                            <= 9 as libc::c_int as libc::c_uint
                        {
                            f = f.offset(1);
                            f;
                            direc_length = direc_length.wrapping_add(1);
                            direc_length;
                        }
                    }
                    if *f as libc::c_int == '.' as i32 {
                        f = f.offset(1);
                        f;
                        direc_length = direc_length.wrapping_add(1);
                        direc_length;
                        ok['c' as i32 as usize] = 0 as libc::c_int as libc::c_char;
                        if *f as libc::c_int == '*' as i32 {
                            f = f.offset(1);
                            f;
                            direc_length = direc_length.wrapping_add(1);
                            direc_length;
                            if argc > 0 as libc::c_int {
                                let mut prec: intmax_t = vstrtoimax(*argv);
                                if prec < 0 as libc::c_int as libc::c_long {
                                    precision = -(1 as libc::c_int);
                                } else if (2147483647 as libc::c_int as libc::c_long) < prec
                                {
                                    if 0 != 0 {
                                        error(
                                            1 as libc::c_int,
                                            0 as libc::c_int,
                                            gettext(
                                                b"invalid precision: %s\0" as *const u8
                                                    as *const libc::c_char,
                                            ),
                                            quote(*argv),
                                        );
                                        if 1 as libc::c_int != 0 as libc::c_int {
                                            unreachable!();
                                        } else {};
                                    } else {
                                        ({
                                            let __errstatus: libc::c_int = 1 as libc::c_int;
                                            error(
                                                __errstatus,
                                                0 as libc::c_int,
                                                gettext(
                                                    b"invalid precision: %s\0" as *const u8
                                                        as *const libc::c_char,
                                                ),
                                                quote(*argv),
                                            );
                                            if __errstatus != 0 as libc::c_int {
                                                unreachable!();
                                            } else {};
                                            
                                        });
                                        ({
                                            let __errstatus: libc::c_int = 1 as libc::c_int;
                                            error(
                                                __errstatus,
                                                0 as libc::c_int,
                                                gettext(
                                                    b"invalid precision: %s\0" as *const u8
                                                        as *const libc::c_char,
                                                ),
                                                quote(*argv),
                                            );
                                            if __errstatus != 0 as libc::c_int {
                                                unreachable!();
                                            } else {};
                                            
                                        });
                                    };
                                } else {
                                    precision = prec as libc::c_int;
                                }
                                argv = argv.offset(1);
                                argv;
                                argc -= 1;
                                argc;
                            } else {
                                precision = 0 as libc::c_int;
                            }
                            have_precision = 1 as libc::c_int != 0;
                        } else {
                            while (*f as libc::c_uint)
                                .wrapping_sub('0' as i32 as libc::c_uint)
                                <= 9 as libc::c_int as libc::c_uint
                            {
                                f = f.offset(1);
                                f;
                                direc_length = direc_length.wrapping_add(1);
                                direc_length;
                            }
                        }
                    }
                    while *f as libc::c_int == 'l' as i32
                        || *f as libc::c_int == 'L' as i32
                        || *f as libc::c_int == 'h' as i32
                        || *f as libc::c_int == 'j' as i32
                        || *f as libc::c_int == 't' as i32
                        || *f as libc::c_int == 'z' as i32
                    {
                        f = f.offset(1);
                        f;
                    }
                    let mut conversion: libc::c_uchar = *f as libc::c_uchar;
                    let mut speclen: libc::c_int = (if (f
                        .offset(1 as libc::c_int as isize)
                        .offset_from(direc_start) as libc::c_long)
                        < 2147483647 as libc::c_int as libc::c_long
                    {
                        f.offset(1 as libc::c_int as isize).offset_from(direc_start)
                            as libc::c_long
                    } else {
                        2147483647 as libc::c_int as libc::c_long
                    }) as libc::c_int;
                    if ok[conversion as usize] == 0 {
                        if 0 != 0 {
                            error(
                                1 as libc::c_int,
                                0 as libc::c_int,
                                gettext(
                                    b"%.*s: invalid conversion specification\0" as *const u8
                                        as *const libc::c_char,
                                ),
                                speclen,
                                direc_start,
                            );
                            if 1 as libc::c_int != 0 as libc::c_int {
                                unreachable!();
                            } else {};
                        } else {
                            ({
                                let __errstatus: libc::c_int = 1 as libc::c_int;
                                error(
                                    __errstatus,
                                    0 as libc::c_int,
                                    gettext(
                                        b"%.*s: invalid conversion specification\0" as *const u8
                                            as *const libc::c_char,
                                    ),
                                    speclen,
                                    direc_start,
                                );
                                if __errstatus != 0 as libc::c_int {
                                    unreachable!();
                                } else {};
                                
                            });
                            ({
                                let __errstatus: libc::c_int = 1 as libc::c_int;
                                error(
                                    __errstatus,
                                    0 as libc::c_int,
                                    gettext(
                                        b"%.*s: invalid conversion specification\0" as *const u8
                                            as *const libc::c_char,
                                    ),
                                    speclen,
                                    direc_start,
                                );
                                if __errstatus != 0 as libc::c_int {
                                    unreachable!();
                                } else {};
                                
                            });
                        };
                    }
                    print_direc(
                        direc_start,
                        direc_length,
                        *f,
                        have_field_width,
                        field_width,
                        have_precision,
                        precision,
                        if argc <= 0 as libc::c_int {
                            b"\0" as *const u8 as *const libc::c_char
                        } else {
                            argc -= 1;
                            argc;
                            let fresh3 = argv;
                            argv = argv.offset(1);
                            *fresh3 as *const libc::c_char
                        },
                    );
                }
            }
            92 => {
                f = f.offset(print_esc(f, 0 as libc::c_int != 0) as isize);
            }
            _ => {
                putchar_unlocked(*f as libc::c_int);
            }
        }
        f = f.offset(1);
        f;
    }
    return save_argc - argc;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut format: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut args_used: libc::c_int = 0;
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    exit_status = 0 as libc::c_int;
    posixly_correct = !(getenv(b"POSIXLY_CORRECT\0" as *const u8 as *const libc::c_char))
        .is_null();
    if argc == 2 as libc::c_int {
        if strcmp(
            *argv.offset(1 as libc::c_int as isize),
            b"--help\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            usage(0 as libc::c_int);
        }
        if strcmp(
            *argv.offset(1 as libc::c_int as isize),
            b"--version\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            version_etc(
                stdout,
                b"printf\0" as *const u8 as *const libc::c_char,
                b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                Version,
                proper_name_lite(
                    b"David MacKenzie\0" as *const u8 as *const libc::c_char,
                    b"David MacKenzie\0" as *const u8 as *const libc::c_char,
                ),
                0 as *mut libc::c_void as *mut libc::c_char,
            );
            return 0 as libc::c_int;
        }
    }
    if (1 as libc::c_int) < argc
        && strcmp(
            *argv.offset(1 as libc::c_int as isize),
            b"--\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
    {
        argc -= 1;
        argc;
        argv = argv.offset(1);
        argv;
    }
    if argc <= 1 as libc::c_int {
        if 0 != 0 {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                gettext(b"missing operand\0" as *const u8 as *const libc::c_char),
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
                    gettext(b"missing operand\0" as *const u8 as *const libc::c_char),
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
                    gettext(b"missing operand\0" as *const u8 as *const libc::c_char),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
        usage(1 as libc::c_int);
    }
    format = *argv.offset(1 as libc::c_int as isize);
    argc -= 2 as libc::c_int;
    argv = argv.offset(2 as libc::c_int as isize);
    loop {
        args_used = print_formatted(format, argc, argv);
        argc -= args_used;
        argv = argv.offset(args_used as isize);
        if !(args_used > 0 as libc::c_int && argc > 0 as libc::c_int) {
            break;
        }
    }
    if argc > 0 as libc::c_int {
        if 0 != 0 {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"warning: ignoring excess arguments, starting with %s\0"
                        as *const u8 as *const libc::c_char,
                ),
                quote(*argv.offset(0 as libc::c_int as isize)),
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
                        b"warning: ignoring excess arguments, starting with %s\0"
                            as *const u8 as *const libc::c_char,
                    ),
                    quote(*argv.offset(0 as libc::c_int as isize)),
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
                        b"warning: ignoring excess arguments, starting with %s\0"
                            as *const u8 as *const libc::c_char,
                    ),
                    quote(*argv.offset(0 as libc::c_int as isize)),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
    }
    return exit_status;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
