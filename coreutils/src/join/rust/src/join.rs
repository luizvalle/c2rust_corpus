#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut, unused_imports)]
#![feature(extern_types, label_break_value)]
use ::rust::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    static mut Version: *const libc::c_char;
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
    fn xzalloc(s: size_t) -> *mut libc::c_void;
    fn xpalloc(
        pa: *mut libc::c_void,
        pn: *mut idx_t,
        n_incr_min: idx_t,
        n_max: ptrdiff_t,
        s: idx_t,
    ) -> *mut libc::c_void;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush_unlocked(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn putchar_unlocked(__c: libc::c_int) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fwrite_unlocked(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn clearerr_unlocked(__stream: *mut FILE);
    fn ferror_unlocked(__stream: *mut FILE) -> libc::c_int;
    fn rpl_fclose(stream: *mut FILE) -> libc::c_int;
    fn fpurge(gl_stream: *mut FILE) -> libc::c_int;
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
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn quotearg_style(s: quoting_style, arg: *const libc::c_char) -> *mut libc::c_char;
    fn quotearg_n_style_colon(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn fadvise(fp: *mut FILE, advice: fadvice_t);
    fn hard_locale(category: libc::c_int) -> bool;
    fn readlinebuffer_delim(
        linebuffer: *mut linebuffer,
        stream: *mut FILE,
        delimiter: libc::c_char,
    ) -> *mut linebuffer;
    fn iswblank(__wc: wint_t) -> libc::c_int;
    fn mbrtoc32(
        __pc32: *mut char32_t,
        __s: *const libc::c_char,
        __n: size_t,
        __p: *mut mbstate_t,
    ) -> size_t;
    fn memcasecmp(
        vs1: *const libc::c_void,
        vs2: *const libc::c_void,
        n: size_t,
    ) -> libc::c_int;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn fopen_safer(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn xmemcoll(
        _: *mut libc::c_char,
        _: size_t,
        _: *mut libc::c_char,
        _: size_t,
    ) -> libc::c_int;
    fn xstrtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
        _: *mut libc::c_long,
        _: *const libc::c_char,
    ) -> strtol_error;
    fn xstrtoimax(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
        _: *mut intmax_t,
        _: *const libc::c_char,
    ) -> strtol_error;
}
pub type __uint32_t = libc::c_uint;
pub type __uint_least32_t = __uint32_t;
pub type __intmax_t = libc::c_long;
pub type __uintmax_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type ptrdiff_t = libc::c_long;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
pub type idx_t = ptrdiff_t;
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
pub type C2RustUnnamed_0 = libc::c_int;
pub const GETOPT_VERSION_CHAR: C2RustUnnamed_0 = -3;
pub const GETOPT_HELP_CHAR: C2RustUnnamed_0 = -2;
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
pub type fadvice_t = libc::c_uint;
pub const FADVISE_RANDOM: fadvice_t = 1;
pub const FADVISE_WILLNEED: fadvice_t = 3;
pub const FADVISE_DONTNEED: fadvice_t = 4;
pub const FADVISE_NOREUSE: fadvice_t = 5;
pub const FADVISE_SEQUENTIAL: fadvice_t = 2;
pub const FADVISE_NORMAL: fadvice_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linebuffer {
    pub size: idx_t,
    pub length: idx_t,
    pub buffer: *mut libc::c_char,
}
pub type mbstate_t = __mbstate_t;
pub type char32_t = __uint_least32_t;
pub type wint_t = libc::c_uint;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const MCEL_LEN_MAX: C2RustUnnamed_1 = 4;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const MCEL_CHAR_MAX: C2RustUnnamed_2 = 1114111;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const MCEL_ERR_MIN: C2RustUnnamed_3 = 128;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mcel_t {
    pub ch: char32_t,
    pub err: libc::c_uchar,
    pub len: libc::c_uchar,
}
pub type C2RustUnnamed_4 = libc::c_uint;
pub const MCEL_ERR_SHIFT: C2RustUnnamed_4 = 14;
pub type strtol_error = libc::c_uint;
pub const LONGINT_INVALID: strtol_error = 4;
pub const LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW: strtol_error = 3;
pub const LONGINT_INVALID_SUFFIX_CHAR: strtol_error = 2;
pub const LONGINT_OVERFLOW: strtol_error = 1;
pub const LONGINT_OK: strtol_error = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct outlist {
    pub file: libc::c_int,
    pub field: idx_t,
    pub next: *mut outlist,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct field {
    pub beg: *mut libc::c_char,
    pub len: idx_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct line {
    pub buf: linebuffer,
    pub nfields: idx_t,
    pub nfields_allocated: idx_t,
    pub fields: *mut field,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct seq {
    pub count: idx_t,
    pub alloc: idx_t,
    pub lines: *mut *mut line,
}
pub type C2RustUnnamed_5 = libc::c_uint;
pub const CHECK_ORDER_DISABLED: C2RustUnnamed_5 = 2;
pub const CHECK_ORDER_ENABLED: C2RustUnnamed_5 = 1;
pub const CHECK_ORDER_DEFAULT: C2RustUnnamed_5 = 0;
pub type C2RustUnnamed_6 = libc::c_uint;
pub const HEADER_LINE_OPTION: C2RustUnnamed_6 = 258;
pub const NOCHECK_ORDER_OPTION: C2RustUnnamed_6 = 257;
pub const CHECK_ORDER_OPTION: C2RustUnnamed_6 = 256;
pub type operand_status = libc::c_uint;
pub const MIGHT_BE_O_ARG: operand_status = 3;
pub const MIGHT_BE_J2_ARG: operand_status = 2;
pub const MIGHT_BE_J1_ARG: operand_status = 1;
pub const MUST_BE_OPERAND: operand_status = 0;
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
unsafe extern "C" fn write_error() {
    let mut saved_errno: libc::c_int = *__errno_location();
    fflush_unlocked(stdout);
    fpurge(stdout);
    clearerr_unlocked(stdout);
    if 0 != 0 {
        error(
            1 as libc::c_int,
            saved_errno,
            gettext(b"write error\0" as *const u8 as *const libc::c_char),
        );
        if 1 as libc::c_int != 0 as libc::c_int {
            unreachable!();
        } else {};
    } else {
        ({
            let __errstatus: libc::c_int = 1 as libc::c_int;
            error(
                __errstatus,
                saved_errno,
                gettext(b"write error\0" as *const u8 as *const libc::c_char),
            );
            if __errstatus != 0 as libc::c_int {
                unreachable!();
            } else {};
            
        });
        ({
            let __errstatus: libc::c_int = 1 as libc::c_int;
            error(
                __errstatus,
                saved_errno,
                gettext(b"write error\0" as *const u8 as *const libc::c_char),
            );
            if __errstatus != 0 as libc::c_int {
                unreachable!();
            } else {};
            
        });
    };
}
#[inline]
unsafe extern "C" fn c32isblank(mut wc: wint_t) -> libc::c_int {
    return iswblank(wc);
}
#[inline]
unsafe extern "C" fn mcel_ch(mut ch: char32_t, mut len: size_t) -> mcel_t {
    if (0 as libc::c_int as libc::c_ulong) < len {} else {
        unreachable!();
    };
    if len <= MCEL_LEN_MAX as libc::c_int as libc::c_ulong {} else {
        unreachable!();
    };
    if ch <= MCEL_CHAR_MAX as libc::c_int as libc::c_uint {} else {
        unreachable!();
    };
    return {
        let mut init = mcel_t {
            ch: ch,
            err: 0,
            len: len as libc::c_uchar,
        };
        init
    };
}
#[inline]
unsafe extern "C" fn mcel_err(mut err: libc::c_uchar) -> mcel_t {
    if MCEL_ERR_MIN as libc::c_int <= err as libc::c_int {} else {
        unreachable!();
    };
    return {
        let mut init = mcel_t {
            ch: 0,
            err: err,
            len: 1 as libc::c_int as libc::c_uchar,
        };
        init
    };
}
#[inline]
unsafe extern "C" fn mcel_cmp(mut c1: mcel_t, mut c2: mcel_t) -> libc::c_int {
    let mut ch1: libc::c_int = c1.ch as libc::c_int;
    let mut ch2: libc::c_int = c2.ch as libc::c_int;
    return (c1.err as libc::c_int - c2.err as libc::c_int)
        * ((1 as libc::c_int) << MCEL_ERR_SHIFT as libc::c_int) + (ch1 - ch2);
}
#[inline]
unsafe extern "C" fn mcel_isbasic(mut c: libc::c_char) -> bool {
    return (0 as libc::c_int <= c as libc::c_int
        && (c as libc::c_int) < MCEL_ERR_MIN as libc::c_int) as libc::c_int
        as libc::c_long != 0;
}
#[inline]
unsafe extern "C" fn mcel_scan(
    mut p: *const libc::c_char,
    mut lim: *const libc::c_char,
) -> mcel_t {
    let mut c: libc::c_char = *p;
    if mcel_isbasic(c) {
        return mcel_ch(c as char32_t, 1 as libc::c_int as size_t);
    }
    let mut mbs: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2RustUnnamed { __wch: 0 },
    };
    mbs.__count = 0 as libc::c_int;
    let mut ch: char32_t = 0;
    let mut len: size_t = mbrtoc32(
        &mut ch,
        p,
        lim.offset_from(p) as libc::c_long as size_t,
        &mut mbs,
    );
    if ((-(1 as libc::c_int) as size_t).wrapping_div(2 as libc::c_int as libc::c_ulong)
        < len) as libc::c_int as libc::c_long != 0
    {
        return mcel_err(c as libc::c_uchar);
    }
    return mcel_ch(ch, len);
}
#[inline]
unsafe extern "C" fn mcel_scant(
    mut p: *const libc::c_char,
    mut terminator: libc::c_char,
) -> mcel_t {
    if mcel_isbasic(*p) {
        return mcel_ch(*p as char32_t, 1 as libc::c_int as size_t);
    }
    let mut lim: *const libc::c_char = p.offset(1 as libc::c_int as isize);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < MCEL_LEN_MAX as libc::c_int - 1 as libc::c_int {
        lim = lim
            .offset(
                (*lim as libc::c_int != terminator as libc::c_int) as libc::c_int
                    as isize,
            );
        i += 1;
        i;
    }
    return mcel_scan(p, lim);
}
#[inline]
unsafe extern "C" fn mcel_scanz(mut p: *const libc::c_char) -> mcel_t {
    return mcel_scant(p, '\0' as i32 as libc::c_char);
}
#[inline]
unsafe extern "C" fn skip_str_matching(
    mut str: *const libc::c_char,
    mut predicate: Option::<unsafe extern "C" fn(mcel_t) -> bool>,
    mut ok: bool,
) -> *mut libc::c_char {
    let mut s: *const libc::c_char = str;
    let mut g: mcel_t = mcel_t { ch: 0, err: 0, len: 0 };
    while *s as libc::c_int != 0
        && {
            g = mcel_scanz(s);
            predicate.expect("non-null function pointer")(g) as libc::c_int
                == ok as libc::c_int
        }
    {
        s = s.offset(g.len as libc::c_int as isize);
    }
    return s as *mut libc::c_char;
}
#[inline]
unsafe extern "C" fn skip_buf_matching(
    mut buf: *const libc::c_char,
    mut lim: *const libc::c_char,
    mut predicate: Option::<unsafe extern "C" fn(mcel_t) -> bool>,
    mut ok: bool,
) -> *mut libc::c_char {
    let mut s: *const libc::c_char = buf;
    let mut g: mcel_t = mcel_t { ch: 0, err: 0, len: 0 };
    while s < lim
        && {
            g = mcel_scan(s, lim);
            predicate.expect("non-null function pointer")(g) as libc::c_int
                == ok as libc::c_int
        }
    {
        s = s.offset(g.len as libc::c_int as isize);
    }
    return s as *mut libc::c_char;
}
static mut prevline: [*mut line; 2] = [
    0 as *const line as *mut line,
    0 as *const line as *mut line,
];
static mut line_no: [uintmax_t; 2] = [
    0 as libc::c_int as uintmax_t,
    0 as libc::c_int as uintmax_t,
];
static mut g_names: [*mut libc::c_char; 2] = [0 as *const libc::c_char
    as *mut libc::c_char; 2];
static mut spareline: [*mut line; 2] = [
    0 as *const line as *mut line,
    0 as *const line as *mut line,
];
static mut hard_LC_COLLATE: bool = false;
static mut print_unpairables_1: bool = false;
static mut print_unpairables_2: bool = false;
static mut print_pairables: bool = false;
static mut seen_unpairable: bool = false;
static mut issued_disorder_warning: [bool; 2] = [false; 2];
static mut empty_filler: *const libc::c_char = 0 as *const libc::c_char;
static mut autoformat: bool = false;
static mut autocount_1: idx_t = 0;
static mut autocount_2: idx_t = 0;
static mut join_field_1: ptrdiff_t = -(1 as libc::c_int) as ptrdiff_t;
static mut join_field_2: ptrdiff_t = -(1 as libc::c_int) as ptrdiff_t;
static mut outlist_head: outlist = outlist {
    file: 0,
    field: 0,
    next: 0 as *const outlist as *mut outlist,
};
static mut outlist_end: *mut outlist = unsafe {
    &outlist_head as *const outlist as *mut outlist
};
static mut tab: mcel_t = mcel_t { ch: 0, err: 0, len: 0 };
static mut output_separator: *const libc::c_char = b" \0" as *const u8
    as *const libc::c_char;
static mut output_seplen: idx_t = 1 as libc::c_int as idx_t;
static mut check_input_order: C2RustUnnamed_5 = CHECK_ORDER_DEFAULT;
static mut longopts: [option; 8] = [
    {
        let mut init = option {
            name: b"ignore-case\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'i' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"check-order\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: CHECK_ORDER_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"nocheck-order\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: NOCHECK_ORDER_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"zero-terminated\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'z' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"header\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: HEADER_LINE_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"help\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: GETOPT_HELP_CHAR as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"version\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: GETOPT_VERSION_CHAR as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: 0 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
];
static mut uni_blank: line = line {
    buf: linebuffer {
        size: 0,
        length: 0,
        buffer: 0 as *const libc::c_char as *mut libc::c_char,
    },
    nfields: 0,
    nfields_allocated: 0,
    fields: 0 as *const field as *mut field,
};
static mut ignore_case: bool = false;
static mut join_header_lines: bool = false;
static mut eolchar: libc::c_char = '\n' as i32 as libc::c_char;
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
                b"Usage: %s [OPTION]... FILE1 FILE2\n\0" as *const u8
                    as *const libc::c_char,
            ),
            program_name,
        );
        fputs_unlocked(
            gettext(
                b"For each pair of input lines with identical join fields, write a line to\nstandard output.  The default join field is the first, delimited by blanks.\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"\nWhen FILE1 or FILE2 (not both) is -, read standard input.\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"\n  -a FILENUM             also print unpairable lines from file FILENUM, where\n                           FILENUM is 1 or 2, corresponding to FILE1 or FILE2\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"  -e STRING              replace missing (empty) input fields with STRING;\n                           I.e., missing fields specified with '-12jo' options\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"  -i, --ignore-case      ignore differences in case when comparing fields\n  -j FIELD               equivalent to '-1 FIELD -2 FIELD'\n  -o FORMAT              obey FORMAT while constructing output line\n  -t CHAR                use CHAR as input and output field separator\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"  -v FILENUM             like -a FILENUM, but suppress joined output lines\n  -1 FIELD               join on this FIELD of file 1\n  -2 FIELD               join on this FIELD of file 2\n      --check-order      check that the input is correctly sorted, even\n                           if all input lines are pairable\n      --nocheck-order    do not check that the input is correctly sorted\n      --header           treat the first line in each file as field headers,\n                           print them without trying to pair them\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"  -z, --zero-terminated  line delimiter is NUL, not newline\n\0"
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
                b"\nUnless -t CHAR is given, leading blanks separate fields and are ignored,\nelse fields are separated by CHAR.  Any FIELD is a field number counted\nfrom 1.  FORMAT is one or more comma or blank separated specifications,\neach being 'FILENUM.FIELD' or '0'.  Default FORMAT outputs the join field,\nthe remaining fields from FILE1, the remaining fields from FILE2, all\nseparated by CHAR.  If FORMAT is the keyword 'auto', then the first\nline of each file determines the number of fields output for each line.\n\nImportant: FILE1 and FILE2 must be sorted on the join fields.\nE.g., use \"sort -k 1b,1\" if 'join' has no options,\nor use \"join -t ''\" if 'sort' has no options.\nComparisons honor the rules specified by 'LC_COLLATE'.\nIf the input is not sorted and some lines cannot be joined, a\nwarning message will be given.\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        emit_ancillary_info(b"join\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
}
unsafe extern "C" fn extract_field(
    mut line: *mut line,
    mut field: *mut libc::c_char,
    mut len: idx_t,
) {
    if (*line).nfields >= (*line).nfields_allocated {
        (*line)
            .fields = xpalloc(
            (*line).fields as *mut libc::c_void,
            &mut (*line).nfields_allocated,
            1 as libc::c_int as idx_t,
            -(1 as libc::c_int) as ptrdiff_t,
            ::core::mem::size_of::<field>() as libc::c_ulong as idx_t,
        ) as *mut field;
    }
    let ref mut fresh0 = (*((*line).fields).offset((*line).nfields as isize)).beg;
    *fresh0 = field;
    (*((*line).fields).offset((*line).nfields as isize)).len = len;
    (*line).nfields += 1;
    (*line).nfields;
}
unsafe extern "C" fn eq_tab(mut g: mcel_t) -> bool {
    return mcel_cmp(g, tab) == 0 as libc::c_int;
}
unsafe extern "C" fn newline_or_blank(mut g: mcel_t) -> bool {
    return g.ch == '\n' as i32 as libc::c_uint || c32isblank(g.ch) != 0;
}
unsafe extern "C" fn xfields(mut line: *mut line) {
    let mut ptr: *mut libc::c_char = (*line).buf.buffer;
    let mut lim: *const libc::c_char = ptr
        .offset((*line).buf.length as isize)
        .offset(-(1 as libc::c_int as isize));
    if ptr == lim as *mut libc::c_char {
        return;
    }
    if tab.len == 0 {
        loop {
            ptr = skip_buf_matching(
                ptr,
                lim,
                Some(newline_or_blank as unsafe extern "C" fn(mcel_t) -> bool),
                1 as libc::c_int != 0,
            );
            if !(ptr < lim as *mut libc::c_char) {
                break;
            }
            let mut sep: *mut libc::c_char = skip_buf_matching(
                ptr,
                lim,
                Some(newline_or_blank as unsafe extern "C" fn(mcel_t) -> bool),
                0 as libc::c_int != 0,
            );
            extract_field(line, ptr, sep.offset_from(ptr) as libc::c_long);
            ptr = sep;
        }
    } else {
        if tab.ch != '\n' as i32 as libc::c_uint {
            let mut sep_0: *mut libc::c_char = 0 as *mut libc::c_char;
            loop {
                sep_0 = skip_buf_matching(
                    ptr,
                    lim,
                    Some(eq_tab as unsafe extern "C" fn(mcel_t) -> bool),
                    0 as libc::c_int != 0,
                );
                if !(sep_0 < lim as *mut libc::c_char) {
                    break;
                }
                extract_field(line, ptr, sep_0.offset_from(ptr) as libc::c_long);
                ptr = sep_0.offset((mcel_scan(sep_0, lim)).len as libc::c_int as isize);
            }
        }
        extract_field(line, ptr, lim.offset_from(ptr) as libc::c_long);
    };
}
unsafe extern "C" fn freeline(mut line: *mut line) {
    if line.is_null() {
        return;
    }
    free((*line).fields as *mut libc::c_void);
    (*line).fields = 0 as *mut field;
    free((*line).buf.buffer as *mut libc::c_void);
    (*line).buf.buffer = 0 as *mut libc::c_char;
}
unsafe extern "C" fn keycmp(
    mut line1: *const line,
    mut line2: *const line,
    mut jf_1: idx_t,
    mut jf_2: idx_t,
) -> libc::c_int {
    let mut beg1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut beg2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len1: idx_t = 0;
    let mut len2: idx_t = 0;
    let mut diff: libc::c_int = 0;
    if jf_1 < (*line1).nfields {
        beg1 = (*((*line1).fields).offset(jf_1 as isize)).beg;
        len1 = (*((*line1).fields).offset(jf_1 as isize)).len;
    } else {
        beg1 = 0 as *mut libc::c_char;
        len1 = 0 as libc::c_int as idx_t;
    }
    if jf_2 < (*line2).nfields {
        beg2 = (*((*line2).fields).offset(jf_2 as isize)).beg;
        len2 = (*((*line2).fields).offset(jf_2 as isize)).len;
    } else {
        beg2 = 0 as *mut libc::c_char;
        len2 = 0 as libc::c_int as idx_t;
    }
    if len1 == 0 as libc::c_int as libc::c_long {
        return if len2 == 0 as libc::c_int as libc::c_long {
            0 as libc::c_int
        } else {
            -(1 as libc::c_int)
        };
    }
    if len2 == 0 as libc::c_int as libc::c_long {
        return 1 as libc::c_int;
    }
    if ignore_case {
        diff = memcasecmp(
            beg1 as *const libc::c_void,
            beg2 as *const libc::c_void,
            (if len1 < len2 { len1 } else { len2 }) as size_t,
        );
    } else {
        if hard_LC_COLLATE {
            return xmemcoll(beg1, len1 as size_t, beg2, len2 as size_t);
        }
        diff = memcmp(
            beg1 as *const libc::c_void,
            beg2 as *const libc::c_void,
            (if len1 < len2 { len1 } else { len2 }) as libc::c_ulong,
        );
    }
    if diff != 0 {
        return diff;
    }
    return (len1 > len2) as libc::c_int - (len1 < len2) as libc::c_int;
}
unsafe extern "C" fn check_order(
    mut prev: *const line,
    mut current: *const line,
    mut whatfile: libc::c_int,
) {
    if check_input_order as libc::c_uint
        != CHECK_ORDER_DISABLED as libc::c_int as libc::c_uint
        && (check_input_order as libc::c_uint
            == CHECK_ORDER_ENABLED as libc::c_int as libc::c_uint
            || seen_unpairable as libc::c_int != 0)
    {
        if !issued_disorder_warning[(whatfile - 1 as libc::c_int) as usize] {
            let mut join_field: idx_t = if whatfile == 1 as libc::c_int {
                join_field_1
            } else {
                join_field_2
            };
            if keycmp(prev, current, join_field, join_field) > 0 as libc::c_int {
                let mut len: idx_t = (*current).buf.length;
                if (0 as libc::c_int as libc::c_long) < len
                    && *((*current).buf.buffer)
                        .offset((len - 1 as libc::c_int as libc::c_long) as isize)
                        as libc::c_int == '\n' as i32
                {
                    len -= 1;
                    len;
                }
                len = if (2147483647 as libc::c_int as libc::c_long) < len {
                    2147483647 as libc::c_int as libc::c_long
                } else {
                    len
                };
                if 0 != 0 {
                    error(
                        if check_input_order as libc::c_uint
                            == CHECK_ORDER_ENABLED as libc::c_int as libc::c_uint
                        {
                            1 as libc::c_int
                        } else {
                            0 as libc::c_int
                        },
                        0 as libc::c_int,
                        gettext(
                            b"%s:%ju: is not sorted: %.*s\0" as *const u8
                                as *const libc::c_char,
                        ),
                        g_names[(whatfile - 1 as libc::c_int) as usize],
                        line_no[(whatfile - 1 as libc::c_int) as usize],
                        len as libc::c_int,
                        (*current).buf.buffer,
                    );
                    if (if check_input_order as libc::c_uint
                        == CHECK_ORDER_ENABLED as libc::c_int as libc::c_uint
                    {
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) != 0 as libc::c_int
                    {
                        unreachable!();
                    } else {};
                } else {
                    ({
                        let __errstatus: libc::c_int = if check_input_order
                            as libc::c_uint
                            == CHECK_ORDER_ENABLED as libc::c_int as libc::c_uint
                        {
                            1 as libc::c_int
                        } else {
                            0 as libc::c_int
                        };
                        error(
                            __errstatus,
                            0 as libc::c_int,
                            gettext(
                                b"%s:%ju: is not sorted: %.*s\0" as *const u8
                                    as *const libc::c_char,
                            ),
                            g_names[(whatfile - 1 as libc::c_int) as usize],
                            line_no[(whatfile - 1 as libc::c_int) as usize],
                            len as libc::c_int,
                            (*current).buf.buffer,
                        );
                        if __errstatus != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                        
                    });
                    ({
                        let __errstatus: libc::c_int = if check_input_order
                            as libc::c_uint
                            == CHECK_ORDER_ENABLED as libc::c_int as libc::c_uint
                        {
                            1 as libc::c_int
                        } else {
                            0 as libc::c_int
                        };
                        error(
                            __errstatus,
                            0 as libc::c_int,
                            gettext(
                                b"%s:%ju: is not sorted: %.*s\0" as *const u8
                                    as *const libc::c_char,
                            ),
                            g_names[(whatfile - 1 as libc::c_int) as usize],
                            line_no[(whatfile - 1 as libc::c_int) as usize],
                            len as libc::c_int,
                            (*current).buf.buffer,
                        );
                        if __errstatus != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                        
                    });
                };
                issued_disorder_warning[(whatfile - 1 as libc::c_int)
                    as usize] = 1 as libc::c_int != 0;
            }
        }
    }
}
#[inline]
unsafe extern "C" fn reset_line(mut line: *mut line) {
    (*line).nfields = 0 as libc::c_int as idx_t;
}
unsafe extern "C" fn init_linep(mut linep: *mut *mut line) -> *mut line {
    let mut line: *mut line = xzalloc(::core::mem::size_of::<line>() as libc::c_ulong)
        as *mut line;
    *linep = line;
    return line;
}
unsafe extern "C" fn get_line(
    mut fp: *mut FILE,
    mut linep: *mut *mut line,
    mut which: libc::c_int,
) -> bool {
    let mut line: *mut line = *linep;
    if line == prevline[(which - 1 as libc::c_int) as usize] {
        let mut tmp: *mut line = line;
        line = spareline[(which - 1 as libc::c_int) as usize];
        spareline[(which - 1 as libc::c_int) as usize] = tmp;
        *linep = line;
    }
    if !line.is_null() {
        reset_line(line);
    } else {
        line = init_linep(linep);
    }
    if (readlinebuffer_delim(&mut (*line).buf, fp, eolchar)).is_null() {
        if ferror_unlocked(fp) != 0 {
            if 0 != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    gettext(b"read error\0" as *const u8 as *const libc::c_char),
                );
                if 1 as libc::c_int != 0 as libc::c_int {
                    unreachable!();
                } else {};
            } else {
                ({
                    let __errstatus: libc::c_int = 1 as libc::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        gettext(b"read error\0" as *const u8 as *const libc::c_char),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
                ({
                    let __errstatus: libc::c_int = 1 as libc::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        gettext(b"read error\0" as *const u8 as *const libc::c_char),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
        freeline(line);
        return 0 as libc::c_int != 0;
    }
    line_no[(which - 1 as libc::c_int)
        as usize] = (line_no[(which - 1 as libc::c_int) as usize]).wrapping_add(1);
    line_no[(which - 1 as libc::c_int) as usize];
    xfields(line);
    if !(prevline[(which - 1 as libc::c_int) as usize]).is_null() {
        check_order(prevline[(which - 1 as libc::c_int) as usize], line, which);
    }
    prevline[(which - 1 as libc::c_int) as usize] = line;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn free_spareline() {
    let mut i: idx_t = 0 as libc::c_int as idx_t;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[*mut line; 2]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut line>() as libc::c_ulong)
    {
        if !(spareline[i as usize]).is_null() {
            freeline(spareline[i as usize]);
            free(spareline[i as usize] as *mut libc::c_void);
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn initseq(mut seq: *mut seq) {
    (*seq).count = 0 as libc::c_int as idx_t;
    (*seq).alloc = 0 as libc::c_int as idx_t;
    (*seq).lines = 0 as *mut *mut line;
}
unsafe extern "C" fn getseq(
    mut fp: *mut FILE,
    mut seq: *mut seq,
    mut whichfile: libc::c_int,
) -> bool {
    if (*seq).count == (*seq).alloc {
        (*seq)
            .lines = xpalloc(
            (*seq).lines as *mut libc::c_void,
            &mut (*seq).alloc,
            1 as libc::c_int as idx_t,
            -(1 as libc::c_int) as ptrdiff_t,
            ::core::mem::size_of::<*mut line>() as libc::c_ulong as idx_t,
        ) as *mut *mut line;
        let mut i: idx_t = (*seq).count;
        while i < (*seq).alloc {
            let ref mut fresh1 = *((*seq).lines).offset(i as isize);
            *fresh1 = 0 as *mut line;
            i += 1;
            i;
        }
    }
    if get_line(fp, &mut *((*seq).lines).offset((*seq).count as isize), whichfile) {
        (*seq).count += 1;
        (*seq).count;
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn advance_seq(
    mut fp: *mut FILE,
    mut seq: *mut seq,
    mut first: bool,
    mut whichfile: libc::c_int,
) -> bool {
    if first {
        (*seq).count = 0 as libc::c_int as idx_t;
    }
    return getseq(fp, seq, whichfile);
}
unsafe extern "C" fn delseq(mut seq: *mut seq) {
    let mut i: idx_t = 0 as libc::c_int as idx_t;
    while i < (*seq).alloc {
        freeline(*((*seq).lines).offset(i as isize));
        free(*((*seq).lines).offset(i as isize) as *mut libc::c_void);
        i += 1;
        i;
    }
    free((*seq).lines as *mut libc::c_void);
}
unsafe extern "C" fn prfield(mut n: idx_t, mut line: *const line) {
    if n < (*line).nfields {
        let mut len: idx_t = (*((*line).fields).offset(n as isize)).len;
        if len != 0 {
            fwrite_unlocked(
                (*((*line).fields).offset(n as isize)).beg as *const libc::c_void,
                1 as libc::c_int as size_t,
                len as size_t,
                stdout,
            );
        } else if !empty_filler.is_null() {
            fputs_unlocked(empty_filler, stdout);
        }
    } else if !empty_filler.is_null() {
        fputs_unlocked(empty_filler, stdout);
    }
}
unsafe extern "C" fn prfields(
    mut line: *const line,
    mut join_field: idx_t,
    mut autocount: idx_t,
) {
    let mut i: idx_t = 0;
    let mut nfields: idx_t = if autoformat as libc::c_int != 0 {
        autocount
    } else {
        (*line).nfields
    };
    i = 0 as libc::c_int as idx_t;
    while i < join_field && i < nfields {
        fwrite_unlocked(
            output_separator as *const libc::c_void,
            1 as libc::c_int as size_t,
            output_seplen as size_t,
            stdout,
        );
        prfield(i, line);
        i += 1;
        i;
    }
    i = join_field + 1 as libc::c_int as libc::c_long;
    while i < nfields {
        fwrite_unlocked(
            output_separator as *const libc::c_void,
            1 as libc::c_int as size_t,
            output_seplen as size_t,
            stdout,
        );
        prfield(i, line);
        i += 1;
        i;
    }
}
unsafe extern "C" fn prjoin(mut line1: *const line, mut line2: *const line) {
    let mut outlist: *const outlist = 0 as *const outlist;
    let mut field: idx_t = 0;
    let mut line: *const line = 0 as *const line;
    outlist = outlist_head.next;
    if !outlist.is_null() {
        let mut o: *const outlist = 0 as *const outlist;
        o = outlist;
        loop {
            if (*o).file == 0 as libc::c_int {
                if line1 == &mut uni_blank as *mut line as *const line {
                    line = line2;
                    field = join_field_2;
                } else {
                    line = line1;
                    field = join_field_1;
                }
            } else {
                line = if (*o).file == 1 as libc::c_int { line1 } else { line2 };
                field = (*o).field;
            }
            prfield(field, line);
            o = (*o).next;
            if o.is_null() {
                break;
            }
            fwrite_unlocked(
                output_separator as *const libc::c_void,
                1 as libc::c_int as size_t,
                output_seplen as size_t,
                stdout,
            );
        }
        putchar_unlocked(eolchar as libc::c_int);
    } else {
        if line1 == &mut uni_blank as *mut line as *const line {
            line = line2;
            field = join_field_2;
        } else {
            line = line1;
            field = join_field_1;
        }
        prfield(field, line);
        prfields(line1, join_field_1, autocount_1);
        prfields(line2, join_field_2, autocount_2);
        putchar_unlocked(eolchar as libc::c_int);
    }
    if ferror_unlocked(stdout) != 0 {
        write_error();
    }
}
unsafe extern "C" fn system_join(mut fp1: *mut FILE, mut fp2: *mut FILE) {
    let mut seq1: seq = seq {
        count: 0,
        alloc: 0,
        lines: 0 as *mut *mut line,
    };
    let mut seq2: seq = seq {
        count: 0,
        alloc: 0,
        lines: 0 as *mut *mut line,
    };
    let mut diff: libc::c_int = 0;
    let mut eof1: bool = false;
    let mut eof2: bool = false;
    fadvise(fp1, FADVISE_SEQUENTIAL);
    fadvise(fp2, FADVISE_SEQUENTIAL);
    initseq(&mut seq1);
    getseq(fp1, &mut seq1, 1 as libc::c_int);
    initseq(&mut seq2);
    getseq(fp2, &mut seq2, 2 as libc::c_int);
    if autoformat {
        autocount_1 = if seq1.count != 0 {
            (**(seq1.lines).offset(0 as libc::c_int as isize)).nfields
        } else {
            0 as libc::c_int as libc::c_long
        };
        autocount_2 = if seq2.count != 0 {
            (**(seq2.lines).offset(0 as libc::c_int as isize)).nfields
        } else {
            0 as libc::c_int as libc::c_long
        };
    }
    if join_header_lines as libc::c_int != 0 && (seq1.count != 0 || seq2.count != 0) {
        let mut hline1: *const line = if seq1.count != 0 {
            *(seq1.lines).offset(0 as libc::c_int as isize)
        } else {
            &mut uni_blank
        };
        let mut hline2: *const line = if seq2.count != 0 {
            *(seq2.lines).offset(0 as libc::c_int as isize)
        } else {
            &mut uni_blank
        };
        prjoin(hline1, hline2);
        prevline[0 as libc::c_int as usize] = 0 as *mut line;
        prevline[1 as libc::c_int as usize] = 0 as *mut line;
        if seq1.count != 0 {
            advance_seq(fp1, &mut seq1, 1 as libc::c_int != 0, 1 as libc::c_int);
        }
        if seq2.count != 0 {
            advance_seq(fp2, &mut seq2, 1 as libc::c_int != 0, 2 as libc::c_int);
        }
    }
    while seq1.count != 0 && seq2.count != 0 {
        diff = keycmp(
            *(seq1.lines).offset(0 as libc::c_int as isize),
            *(seq2.lines).offset(0 as libc::c_int as isize),
            join_field_1,
            join_field_2,
        );
        if diff < 0 as libc::c_int {
            if print_unpairables_1 {
                prjoin(*(seq1.lines).offset(0 as libc::c_int as isize), &mut uni_blank);
            }
            advance_seq(fp1, &mut seq1, 1 as libc::c_int != 0, 1 as libc::c_int);
            seen_unpairable = 1 as libc::c_int != 0;
        } else if diff > 0 as libc::c_int {
            if print_unpairables_2 {
                prjoin(&mut uni_blank, *(seq2.lines).offset(0 as libc::c_int as isize));
            }
            advance_seq(fp2, &mut seq2, 1 as libc::c_int != 0, 2 as libc::c_int);
            seen_unpairable = 1 as libc::c_int != 0;
        } else {
            eof1 = 0 as libc::c_int != 0;
            loop {
                if !advance_seq(
                    fp1,
                    &mut seq1,
                    0 as libc::c_int != 0,
                    1 as libc::c_int,
                ) {
                    eof1 = 1 as libc::c_int != 0;
                    seq1.count += 1;
                    seq1.count;
                    break;
                } else if !(keycmp(
                    *(seq1.lines)
                        .offset(
                            (seq1.count - 1 as libc::c_int as libc::c_long) as isize,
                        ),
                    *(seq2.lines).offset(0 as libc::c_int as isize),
                    join_field_1,
                    join_field_2,
                ) == 0)
                {
                    break;
                }
            }
            eof2 = 0 as libc::c_int != 0;
            loop {
                if !advance_seq(
                    fp2,
                    &mut seq2,
                    0 as libc::c_int != 0,
                    2 as libc::c_int,
                ) {
                    eof2 = 1 as libc::c_int != 0;
                    seq2.count += 1;
                    seq2.count;
                    break;
                } else if !(keycmp(
                    *(seq1.lines).offset(0 as libc::c_int as isize),
                    *(seq2.lines)
                        .offset(
                            (seq2.count - 1 as libc::c_int as libc::c_long) as isize,
                        ),
                    join_field_1,
                    join_field_2,
                ) == 0)
                {
                    break;
                }
            }
            if print_pairables {
                let mut i: idx_t = 0 as libc::c_int as idx_t;
                while i < seq1.count - 1 as libc::c_int as libc::c_long {
                    let mut j: idx_t = 0;
                    j = 0 as libc::c_int as idx_t;
                    while j < seq2.count - 1 as libc::c_int as libc::c_long {
                        prjoin(
                            *(seq1.lines).offset(i as isize),
                            *(seq2.lines).offset(j as isize),
                        );
                        j += 1;
                        j;
                    }
                    i += 1;
                    i;
                }
            }
            if !eof1 {
                let mut tmp: *mut line = *(seq1.lines).offset(0 as libc::c_int as isize);
                let ref mut fresh2 = *(seq1.lines).offset(0 as libc::c_int as isize);
                *fresh2 = *(seq1.lines)
                    .offset((seq1.count - 1 as libc::c_int as libc::c_long) as isize);
                let ref mut fresh3 = *(seq1.lines)
                    .offset((seq1.count - 1 as libc::c_int as libc::c_long) as isize);
                *fresh3 = tmp;
                seq1.count = 1 as libc::c_int as idx_t;
            } else {
                seq1.count = 0 as libc::c_int as idx_t;
            }
            if !eof2 {
                let mut tmp_0: *mut line = *(seq2.lines)
                    .offset(0 as libc::c_int as isize);
                let ref mut fresh4 = *(seq2.lines).offset(0 as libc::c_int as isize);
                *fresh4 = *(seq2.lines)
                    .offset((seq2.count - 1 as libc::c_int as libc::c_long) as isize);
                let ref mut fresh5 = *(seq2.lines)
                    .offset((seq2.count - 1 as libc::c_int as libc::c_long) as isize);
                *fresh5 = tmp_0;
                seq2.count = 1 as libc::c_int as idx_t;
            } else {
                seq2.count = 0 as libc::c_int as idx_t;
            }
        }
    }
    let mut line: *mut line = 0 as *mut line;
    let mut checktail: bool = 0 as libc::c_int != 0;
    if check_input_order as libc::c_uint
        != CHECK_ORDER_DISABLED as libc::c_int as libc::c_uint
        && !(issued_disorder_warning[0 as libc::c_int as usize] as libc::c_int != 0
            && issued_disorder_warning[1 as libc::c_int as usize] as libc::c_int != 0)
    {
        checktail = 1 as libc::c_int != 0;
    }
    if (print_unpairables_1 as libc::c_int != 0 || checktail as libc::c_int != 0)
        && seq1.count != 0
    {
        if print_unpairables_1 {
            prjoin(*(seq1.lines).offset(0 as libc::c_int as isize), &mut uni_blank);
        }
        if seq2.count != 0 {
            seen_unpairable = 1 as libc::c_int != 0;
        }
        while get_line(fp1, &mut line, 1 as libc::c_int) {
            if print_unpairables_1 {
                prjoin(line, &mut uni_blank);
            }
            if issued_disorder_warning[0 as libc::c_int as usize] as libc::c_int != 0
                && !print_unpairables_1
            {
                break;
            }
        }
    }
    if (print_unpairables_2 as libc::c_int != 0 || checktail as libc::c_int != 0)
        && seq2.count != 0
    {
        if print_unpairables_2 {
            prjoin(&mut uni_blank, *(seq2.lines).offset(0 as libc::c_int as isize));
        }
        if seq1.count != 0 {
            seen_unpairable = 1 as libc::c_int != 0;
        }
        while get_line(fp2, &mut line, 2 as libc::c_int) {
            if print_unpairables_2 {
                prjoin(&mut uni_blank, line);
            }
            if issued_disorder_warning[1 as libc::c_int as usize] as libc::c_int != 0
                && !print_unpairables_2
            {
                break;
            }
        }
    }
    freeline(line);
    free(line as *mut libc::c_void);
    delseq(&mut seq1);
    delseq(&mut seq2);
}
unsafe extern "C" fn add_field(mut file: libc::c_int, mut field: idx_t) {
    let mut o: *mut outlist = 0 as *mut outlist;
    if file == 0 as libc::c_int || file == 1 as libc::c_int || file == 2 as libc::c_int
    {} else {
        __assert_fail(
            b"file == 0 || file == 1 || file == 2\0" as *const u8 as *const libc::c_char,
            b"join.c\0" as *const u8 as *const libc::c_char,
            826 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"void add_field(int, idx_t)\0"))
                .as_ptr(),
        );
    }
    'c_11407: {
        if file == 0 as libc::c_int || file == 1 as libc::c_int
            || file == 2 as libc::c_int
        {} else {
            __assert_fail(
                b"file == 0 || file == 1 || file == 2\0" as *const u8
                    as *const libc::c_char,
                b"join.c\0" as *const u8 as *const libc::c_char,
                826 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 27],
                    &[libc::c_char; 27],
                >(b"void add_field(int, idx_t)\0"))
                    .as_ptr(),
            );
        }
    };
    if file != 0 as libc::c_int || field == 0 as libc::c_int as libc::c_long {} else {
        __assert_fail(
            b"file != 0 || field == 0\0" as *const u8 as *const libc::c_char,
            b"join.c\0" as *const u8 as *const libc::c_char,
            827 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"void add_field(int, idx_t)\0"))
                .as_ptr(),
        );
    }
    'c_11356: {
        if file != 0 as libc::c_int || field == 0 as libc::c_int as libc::c_long
        {} else {
            __assert_fail(
                b"file != 0 || field == 0\0" as *const u8 as *const libc::c_char,
                b"join.c\0" as *const u8 as *const libc::c_char,
                827 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 27],
                    &[libc::c_char; 27],
                >(b"void add_field(int, idx_t)\0"))
                    .as_ptr(),
            );
        }
    };
    o = xmalloc(::core::mem::size_of::<outlist>() as libc::c_ulong) as *mut outlist;
    (*o).file = file;
    (*o).field = field;
    (*o).next = 0 as *mut outlist;
    (*outlist_end).next = o;
    outlist_end = o;
}
unsafe extern "C" fn string_to_join_field(mut str: *const libc::c_char) -> idx_t {
    let mut val: intmax_t = 0;
    let mut s_err: strtol_error = xstrtoimax(
        str,
        0 as *mut *mut libc::c_char,
        10 as libc::c_int,
        &mut val,
        b"\0" as *const u8 as *const libc::c_char,
    );
    if s_err as libc::c_uint == LONGINT_OVERFLOW as libc::c_int as libc::c_uint
        || s_err as libc::c_uint == LONGINT_OK as libc::c_int as libc::c_uint
            && (9223372036854775807 as libc::c_long) < val
    {
        val = 9223372036854775807 as libc::c_long;
    } else if s_err as libc::c_uint != LONGINT_OK as libc::c_int as libc::c_uint
        || val <= 0 as libc::c_int as libc::c_long
    {
        if 0 != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"invalid field number: %s\0" as *const u8 as *const libc::c_char,
                ),
                quote(str),
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
                        b"invalid field number: %s\0" as *const u8 as *const libc::c_char,
                    ),
                    quote(str),
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
                        b"invalid field number: %s\0" as *const u8 as *const libc::c_char,
                    ),
                    quote(str),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
    }
    return val - 1 as libc::c_int as libc::c_long;
}
unsafe extern "C" fn decode_field_spec(
    mut s: *const libc::c_char,
    mut file_index: *mut libc::c_int,
    mut field_index: *mut idx_t,
) {
    match *s.offset(0 as libc::c_int as isize) as libc::c_int {
        48 => {
            if *s.offset(1 as libc::c_int as isize) != 0 {
                if 0 != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        gettext(
                            b"invalid field specifier: %s\0" as *const u8
                                as *const libc::c_char,
                        ),
                        quote(s),
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
                                b"invalid field specifier: %s\0" as *const u8
                                    as *const libc::c_char,
                            ),
                            quote(s),
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
                                b"invalid field specifier: %s\0" as *const u8
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
            *file_index = 0 as libc::c_int;
            *field_index = 0 as libc::c_int as idx_t;
        }
        49 | 50 => {
            if *s.offset(1 as libc::c_int as isize) as libc::c_int != '.' as i32 {
                if 0 != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        gettext(
                            b"invalid field specifier: %s\0" as *const u8
                                as *const libc::c_char,
                        ),
                        quote(s),
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
                                b"invalid field specifier: %s\0" as *const u8
                                    as *const libc::c_char,
                            ),
                            quote(s),
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
                                b"invalid field specifier: %s\0" as *const u8
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
            *file_index = *s.offset(0 as libc::c_int as isize) as libc::c_int
                - '0' as i32;
            *field_index = string_to_join_field(s.offset(2 as libc::c_int as isize));
        }
        _ => {
            if 0 != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"invalid file number in field spec: %s\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quote(s),
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
                            b"invalid file number in field spec: %s\0" as *const u8
                                as *const libc::c_char,
                        ),
                        quote(s),
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
                            b"invalid file number in field spec: %s\0" as *const u8
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
    };
}
unsafe extern "C" fn comma_or_blank(mut g: mcel_t) -> bool {
    return g.ch == ',' as i32 as libc::c_uint || c32isblank(g.ch) != 0;
}
unsafe extern "C" fn add_field_list(mut str: *mut libc::c_char) {
    let mut p: *mut libc::c_char = str;
    loop {
        let mut file_index: libc::c_int = 0;
        let mut field_index: idx_t = 0;
        let mut spec_item: *const libc::c_char = p;
        p = skip_str_matching(
            spec_item,
            Some(comma_or_blank as unsafe extern "C" fn(mcel_t) -> bool),
            0 as libc::c_int != 0,
        );
        if *p != 0 {
            let mut g: mcel_t = mcel_scanz(p);
            *p = '\0' as i32 as libc::c_char;
            p = p.offset(g.len as libc::c_int as isize);
        }
        decode_field_spec(spec_item, &mut file_index, &mut field_index);
        add_field(file_index, field_index);
        if !(*p != 0) {
            break;
        }
    };
}
unsafe extern "C" fn set_join_field(mut var: *mut ptrdiff_t, mut val: idx_t) {
    if 0 as libc::c_int as libc::c_long <= *var && *var != val {
        if 0 != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"incompatible join fields %td, %td\0" as *const u8
                        as *const libc::c_char,
                ),
                *var,
                val,
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
                        b"incompatible join fields %td, %td\0" as *const u8
                            as *const libc::c_char,
                    ),
                    *var,
                    val,
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
                        b"incompatible join fields %td, %td\0" as *const u8
                            as *const libc::c_char,
                    ),
                    *var,
                    val,
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
    }
    *var = val;
}
unsafe extern "C" fn add_file_name(
    mut name: *mut libc::c_char,
    mut names: *mut *mut libc::c_char,
    mut operand_status: *mut libc::c_int,
    mut joption_count: *mut libc::c_int,
    mut nfiles: *mut libc::c_int,
    mut prev_optc_status: *mut libc::c_int,
    mut optc_status: *mut libc::c_int,
) {
    let mut n: libc::c_int = *nfiles;
    if n == 2 as libc::c_int {
        let mut op0: bool = *operand_status.offset(0 as libc::c_int as isize)
            == MUST_BE_OPERAND as libc::c_int;
        let mut arg: *mut libc::c_char = *names.offset(op0 as isize);
        let mut current_block_6: u64;
        match *operand_status.offset(op0 as isize) {
            0 => {
                if 0 != 0 {
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        gettext(
                            b"extra operand %s\0" as *const u8 as *const libc::c_char,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, name),
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
                                b"extra operand %s\0" as *const u8 as *const libc::c_char,
                            ),
                            quotearg_style(shell_escape_always_quoting_style, name),
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
                                b"extra operand %s\0" as *const u8 as *const libc::c_char,
                            ),
                            quotearg_style(shell_escape_always_quoting_style, name),
                        );
                        if __errstatus != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                        
                    });
                };
                usage(1 as libc::c_int);
                current_block_6 = 12272387030238183927;
            }
            1 => {
                current_block_6 = 12272387030238183927;
            }
            2 => {
                let ref mut fresh7 = *joption_count.offset(1 as libc::c_int as isize);
                *fresh7 -= 1;
                let _ = *fresh7;
                set_join_field(&mut join_field_2, string_to_join_field(arg));
                current_block_6 = 13183875560443969876;
            }
            3 => {
                add_field_list(arg);
                current_block_6 = 13183875560443969876;
            }
            _ => {
                current_block_6 = 13183875560443969876;
            }
        }
        match current_block_6 {
            12272387030238183927 => {
                let ref mut fresh6 = *joption_count.offset(0 as libc::c_int as isize);
                *fresh6 -= 1;
                let _ = *fresh6;
                set_join_field(&mut join_field_1, string_to_join_field(arg));
            }
            _ => {}
        }
        if !op0 {
            *operand_status
                .offset(
                    0 as libc::c_int as isize,
                ) = *operand_status.offset(1 as libc::c_int as isize);
            let ref mut fresh8 = *names.offset(0 as libc::c_int as isize);
            *fresh8 = *names.offset(1 as libc::c_int as isize);
        }
        n = 1 as libc::c_int;
    }
    *operand_status.offset(n as isize) = *prev_optc_status;
    let ref mut fresh9 = *names.offset(n as isize);
    *fresh9 = name;
    *nfiles = n + 1 as libc::c_int;
    if *prev_optc_status == MIGHT_BE_O_ARG as libc::c_int {
        *optc_status = MIGHT_BE_O_ARG as libc::c_int;
    }
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut optc_status: libc::c_int = 0;
    let mut prev_optc_status: libc::c_int = MUST_BE_OPERAND as libc::c_int;
    let mut operand_status: [libc::c_int; 2] = [0; 2];
    let mut joption_count: [libc::c_int; 2] = [0 as libc::c_int, 0 as libc::c_int];
    let mut fp1: *mut FILE = 0 as *mut FILE;
    let mut fp2: *mut FILE = 0 as *mut FILE;
    let mut optc: libc::c_int = 0;
    let mut nfiles: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    hard_LC_COLLATE = hard_locale(3 as libc::c_int);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    atexit(Some(free_spareline as unsafe extern "C" fn() -> ()));
    print_pairables = 1 as libc::c_int != 0;
    seen_unpairable = 0 as libc::c_int != 0;
    issued_disorder_warning[1 as libc::c_int as usize] = 0 as libc::c_int != 0;
    issued_disorder_warning[0 as libc::c_int
        as usize] = issued_disorder_warning[1 as libc::c_int as usize];
    check_input_order = CHECK_ORDER_DEFAULT;
    loop {
        optc = getopt_long(
            argc,
            argv,
            b"-a:e:i1:2:j:o:t:v:z\0" as *const u8 as *const libc::c_char,
            longopts.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(optc != -(1 as libc::c_int)) {
            break;
        }
        optc_status = MUST_BE_OPERAND as libc::c_int;
        let mut current_block_66: u64;
        match optc {
            118 => {
                print_pairables = 0 as libc::c_int != 0;
                current_block_66 = 16768231286522592859;
            }
            97 => {
                current_block_66 = 16768231286522592859;
            }
            101 => {
                if !empty_filler.is_null()
                    && !(strcmp(empty_filler, optarg) == 0 as libc::c_int)
                {
                    if 0 != 0 {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            gettext(
                                b"conflicting empty-field replacement strings\0"
                                    as *const u8 as *const libc::c_char,
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
                                    b"conflicting empty-field replacement strings\0"
                                        as *const u8 as *const libc::c_char,
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
                                    b"conflicting empty-field replacement strings\0"
                                        as *const u8 as *const libc::c_char,
                                ),
                            );
                            if __errstatus != 0 as libc::c_int {
                                unreachable!();
                            } else {};
                            
                        });
                    };
                }
                empty_filler = optarg;
                current_block_66 = 317151059986244064;
            }
            105 => {
                ignore_case = 1 as libc::c_int != 0;
                current_block_66 = 317151059986244064;
            }
            49 => {
                set_join_field(&mut join_field_1, string_to_join_field(optarg));
                current_block_66 = 317151059986244064;
            }
            50 => {
                set_join_field(&mut join_field_2, string_to_join_field(optarg));
                current_block_66 = 317151059986244064;
            }
            106 => {
                if (*optarg.offset(0 as libc::c_int as isize) as libc::c_int
                    == '1' as i32
                    || *optarg.offset(0 as libc::c_int as isize) as libc::c_int
                        == '2' as i32) && *optarg.offset(1 as libc::c_int as isize) == 0
                    && optarg
                        == (*argv.offset((optind - 1 as libc::c_int) as isize))
                            .offset(2 as libc::c_int as isize)
                {
                    let mut is_j2: bool = *optarg.offset(0 as libc::c_int as isize)
                        as libc::c_int == '2' as i32;
                    joption_count[is_j2 as usize] += 1;
                    joption_count[is_j2 as usize];
                    optc_status = MIGHT_BE_J1_ARG as libc::c_int + is_j2 as libc::c_int;
                } else {
                    set_join_field(&mut join_field_1, string_to_join_field(optarg));
                    set_join_field(&mut join_field_2, join_field_1);
                }
                current_block_66 = 317151059986244064;
            }
            111 => {
                if strcmp(optarg, b"auto\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    autoformat = 1 as libc::c_int != 0;
                } else {
                    add_field_list(optarg);
                    optc_status = MIGHT_BE_O_ARG as libc::c_int;
                }
                current_block_66 = 317151059986244064;
            }
            116 => {
                let mut newtab: mcel_t = mcel_t { ch: 0, err: 0, len: 0 };
                if *optarg == 0 {
                    newtab = mcel_ch(
                        '\n' as i32 as char32_t,
                        1 as libc::c_int as size_t,
                    );
                } else if strcmp(optarg, b"\\0\0" as *const u8 as *const libc::c_char)
                    == 0 as libc::c_int
                {
                    newtab = mcel_ch(
                        '\0' as i32 as char32_t,
                        1 as libc::c_int as size_t,
                    );
                    output_separator = b"\0" as *const u8 as *const libc::c_char;
                } else {
                    newtab = mcel_scanz(optarg);
                    if *optarg.offset(newtab.len as isize) != 0 {
                        if 0 != 0 {
                            error(
                                1 as libc::c_int,
                                0 as libc::c_int,
                                gettext(
                                    b"multi-character tab %s\0" as *const u8
                                        as *const libc::c_char,
                                ),
                                quote(optarg),
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
                                        b"multi-character tab %s\0" as *const u8
                                            as *const libc::c_char,
                                    ),
                                    quote(optarg),
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
                                        b"multi-character tab %s\0" as *const u8
                                            as *const libc::c_char,
                                    ),
                                    quote(optarg),
                                );
                                if __errstatus != 0 as libc::c_int {
                                    unreachable!();
                                } else {};
                                
                            });
                        };
                    }
                    output_separator = optarg;
                }
                if tab.len as libc::c_int != 0
                    && mcel_cmp(tab, newtab) != 0 as libc::c_int
                {
                    if 0 != 0 {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            gettext(
                                b"incompatible tabs\0" as *const u8 as *const libc::c_char,
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
                                    b"incompatible tabs\0" as *const u8 as *const libc::c_char,
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
                                    b"incompatible tabs\0" as *const u8 as *const libc::c_char,
                                ),
                            );
                            if __errstatus != 0 as libc::c_int {
                                unreachable!();
                            } else {};
                            
                        });
                    };
                }
                tab = newtab;
                output_seplen = newtab.len as idx_t;
                current_block_66 = 317151059986244064;
            }
            122 => {
                eolchar = 0 as libc::c_int as libc::c_char;
                current_block_66 = 317151059986244064;
            }
            257 => {
                check_input_order = CHECK_ORDER_DISABLED;
                current_block_66 = 317151059986244064;
            }
            256 => {
                check_input_order = CHECK_ORDER_ENABLED;
                current_block_66 = 317151059986244064;
            }
            1 => {
                add_file_name(
                    optarg,
                    g_names.as_mut_ptr(),
                    operand_status.as_mut_ptr(),
                    joption_count.as_mut_ptr(),
                    &mut nfiles,
                    &mut prev_optc_status,
                    &mut optc_status,
                );
                current_block_66 = 317151059986244064;
            }
            258 => {
                join_header_lines = 1 as libc::c_int != 0;
                current_block_66 = 317151059986244064;
            }
            -2 => {
                usage(0 as libc::c_int);
                current_block_66 = 317151059986244064;
            }
            -3 => {
                version_etc(
                    stdout,
                    b"join\0" as *const u8 as *const libc::c_char,
                    b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                    Version,
                    proper_name_lite(
                        b"Mike Haertel\0" as *const u8 as *const libc::c_char,
                        b"Mike Haertel\0" as *const u8 as *const libc::c_char,
                    ),
                    0 as *mut libc::c_void as *mut libc::c_char,
                );
                exit(0 as libc::c_int);
            }
            _ => {
                usage(1 as libc::c_int);
                current_block_66 = 317151059986244064;
            }
        }
        match current_block_66 {
            16768231286522592859 => {
                let mut val: libc::c_long = 0;
                if xstrtol(
                    optarg,
                    0 as *mut *mut libc::c_char,
                    10 as libc::c_int,
                    &mut val,
                    b"\0" as *const u8 as *const libc::c_char,
                ) as libc::c_uint != LONGINT_OK as libc::c_int as libc::c_uint
                    || val != 1 as libc::c_int as libc::c_long
                        && val != 2 as libc::c_int as libc::c_long
                {
                    if 0 != 0 {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            gettext(
                                b"invalid file number: %s\0" as *const u8
                                    as *const libc::c_char,
                            ),
                            quote(optarg),
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
                                    b"invalid file number: %s\0" as *const u8
                                        as *const libc::c_char,
                                ),
                                quote(optarg),
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
                                    b"invalid file number: %s\0" as *const u8
                                        as *const libc::c_char,
                                ),
                                quote(optarg),
                            );
                            if __errstatus != 0 as libc::c_int {
                                unreachable!();
                            } else {};
                            
                        });
                    };
                }
                if val == 1 as libc::c_int as libc::c_long {
                    print_unpairables_1 = 1 as libc::c_int != 0;
                } else {
                    print_unpairables_2 = 1 as libc::c_int != 0;
                }
            }
            _ => {}
        }
        prev_optc_status = optc_status;
    }
    prev_optc_status = MUST_BE_OPERAND as libc::c_int;
    while optind < argc {
        let fresh10 = optind;
        optind = optind + 1;
        add_file_name(
            *argv.offset(fresh10 as isize),
            g_names.as_mut_ptr(),
            operand_status.as_mut_ptr(),
            joption_count.as_mut_ptr(),
            &mut nfiles,
            &mut prev_optc_status,
            &mut optc_status,
        );
    }
    if nfiles != 2 as libc::c_int {
        if nfiles == 0 as libc::c_int {
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
        } else {
            if 0 != 0 {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"missing operand after %s\0" as *const u8 as *const libc::c_char,
                    ),
                    quote(*argv.offset((argc - 1 as libc::c_int) as isize)),
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
                            b"missing operand after %s\0" as *const u8
                                as *const libc::c_char,
                        ),
                        quote(*argv.offset((argc - 1 as libc::c_int) as isize)),
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
                            b"missing operand after %s\0" as *const u8
                                as *const libc::c_char,
                        ),
                        quote(*argv.offset((argc - 1 as libc::c_int) as isize)),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
        usage(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        if joption_count[i as usize] != 0 as libc::c_int {
            set_join_field(&mut join_field_1, i as idx_t);
            set_join_field(&mut join_field_2, i as idx_t);
        }
        i += 1;
        i;
    }
    if join_field_1 < 0 as libc::c_int as libc::c_long {
        join_field_1 = 0 as libc::c_int as ptrdiff_t;
    }
    if join_field_2 < 0 as libc::c_int as libc::c_long {
        join_field_2 = 0 as libc::c_int as ptrdiff_t;
    }
    fp1 = if strcmp(
        g_names[0 as libc::c_int as usize],
        b"-\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        stdin
    } else {
        fopen_safer(
            g_names[0 as libc::c_int as usize],
            b"r\0" as *const u8 as *const libc::c_char,
        )
    };
    if fp1.is_null() {
        if 0 != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    g_names[0 as libc::c_int as usize],
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
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        g_names[0 as libc::c_int as usize],
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
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        g_names[0 as libc::c_int as usize],
                    ),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
    }
    fp2 = if strcmp(
        g_names[1 as libc::c_int as usize],
        b"-\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        stdin
    } else {
        fopen_safer(
            g_names[1 as libc::c_int as usize],
            b"r\0" as *const u8 as *const libc::c_char,
        )
    };
    if fp2.is_null() {
        if 0 != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    g_names[1 as libc::c_int as usize],
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
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        g_names[1 as libc::c_int as usize],
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
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        g_names[1 as libc::c_int as usize],
                    ),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
    }
    if fp1 == fp2 {
        if 0 != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                gettext(
                    b"both files cannot be standard input\0" as *const u8
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
                    *__errno_location(),
                    gettext(
                        b"both files cannot be standard input\0" as *const u8
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
                    *__errno_location(),
                    gettext(
                        b"both files cannot be standard input\0" as *const u8
                            as *const libc::c_char,
                    ),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
    }
    system_join(fp1, fp2);
    if rpl_fclose(fp1) != 0 as libc::c_int {
        if 0 != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    g_names[0 as libc::c_int as usize],
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
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        g_names[0 as libc::c_int as usize],
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
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        g_names[0 as libc::c_int as usize],
                    ),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
    }
    if rpl_fclose(fp2) != 0 as libc::c_int {
        if 0 != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    g_names[1 as libc::c_int as usize],
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
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        g_names[1 as libc::c_int as usize],
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
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        g_names[1 as libc::c_int as usize],
                    ),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
    }
    if issued_disorder_warning[0 as libc::c_int as usize] as libc::c_int != 0
        || issued_disorder_warning[1 as libc::c_int as usize] as libc::c_int != 0
    {
        if 0 != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"input is not in sorted order\0" as *const u8 as *const libc::c_char,
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
                        b"input is not in sorted order\0" as *const u8
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
                        b"input is not in sorted order\0" as *const u8
                            as *const libc::c_char,
                    ),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
    } else {
        return 0 as libc::c_int
    }
    return 0;
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
