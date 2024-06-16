#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut, unused_imports)]
#![feature(extern_types)]
use ::rust::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush_unlocked(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn getc_unlocked(__stream: *mut FILE) -> libc::c_int;
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
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn rpl_fclose(stream: *mut FILE) -> libc::c_int;
    fn rpl_fopen(filename: *const libc::c_char, mode: *const libc::c_char) -> *mut FILE;
    fn fpurge(gl_stream: *mut FILE) -> libc::c_int;
    static mut optind: libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn quotearg_n_style_colon(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn set_program_name(argv0: *const libc::c_char);
    static mut program_name: *const libc::c_char;
    fn proper_name_lite(
        name_ascii: *const libc::c_char,
        name_utf8: *const libc::c_char,
    ) -> *const libc::c_char;
    fn version_etc(
        stream: *mut FILE,
        command_name: *const libc::c_char,
        package: *const libc::c_char,
        version: *const libc::c_char,
        _: ...
    );
    fn close_stdout();
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
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn xnmalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn fadvise(fp: *mut FILE, advice: fadvice_t);
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type C2RustUnnamed = libc::c_int;
pub const GETOPT_VERSION_CHAR: C2RustUnnamed = -3;
pub const GETOPT_HELP_CHAR: C2RustUnnamed = -2;
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
unsafe extern "C" fn bad_cast(mut s: *const libc::c_char) -> *mut libc::c_char {
    return s as *mut libc::c_char;
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
unsafe extern "C" fn emit_mandatory_arg_note() {
    fputs_unlocked(
        gettext(
            b"\nMandatory arguments to long options are mandatory for short options too.\n\0"
                as *const u8 as *const libc::c_char,
        ),
        stdout,
    );
}
#[inline]
unsafe extern "C" fn emit_stdin_note() {
    fputs_unlocked(
        gettext(
            b"\nWith no FILE, or when FILE is -, read standard input.\n\0" as *const u8
                as *const libc::c_char,
        ),
        stdout,
    );
}
static mut have_read_stdin: bool = false;
static mut serial_merge: bool = false;
static mut delims: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut delim_end: *const libc::c_char = 0 as *const libc::c_char;
static mut line_delim: libc::c_uchar = '\n' as i32 as libc::c_uchar;
static mut longopts: [option; 6] = [
    {
        let mut init = option {
            name: b"serial\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 's' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"delimiters\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'd' as i32,
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
unsafe extern "C" fn collapse_escapes(mut strptr: *const libc::c_char) -> libc::c_int {
    let mut strout: *mut libc::c_char = xstrdup(strptr);
    let mut backslash_at_end: bool = 0 as libc::c_int != 0;
    delims = strout;
    while *strptr != 0 {
        if *strptr as libc::c_int != '\\' as i32 {
            let fresh0 = strptr;
            strptr = strptr.offset(1);
            let fresh1 = strout;
            strout = strout.offset(1);
            *fresh1 = *fresh0;
        } else {
            strptr = strptr.offset(1);
            match *strptr as libc::c_int {
                48 => {
                    let fresh2 = strout;
                    strout = strout.offset(1);
                    *fresh2 = '\0' as i32 as libc::c_char;
                }
                98 => {
                    let fresh3 = strout;
                    strout = strout.offset(1);
                    *fresh3 = '\u{8}' as i32 as libc::c_char;
                }
                102 => {
                    let fresh4 = strout;
                    strout = strout.offset(1);
                    *fresh4 = '\u{c}' as i32 as libc::c_char;
                }
                110 => {
                    let fresh5 = strout;
                    strout = strout.offset(1);
                    *fresh5 = '\n' as i32 as libc::c_char;
                }
                114 => {
                    let fresh6 = strout;
                    strout = strout.offset(1);
                    *fresh6 = '\r' as i32 as libc::c_char;
                }
                116 => {
                    let fresh7 = strout;
                    strout = strout.offset(1);
                    *fresh7 = '\t' as i32 as libc::c_char;
                }
                118 => {
                    let fresh8 = strout;
                    strout = strout.offset(1);
                    *fresh8 = '\u{b}' as i32 as libc::c_char;
                }
                92 => {
                    let fresh9 = strout;
                    strout = strout.offset(1);
                    *fresh9 = '\\' as i32 as libc::c_char;
                }
                0 => {
                    backslash_at_end = 1 as libc::c_int != 0;
                    break;
                }
                _ => {
                    let fresh10 = strout;
                    strout = strout.offset(1);
                    *fresh10 = *strptr;
                }
            }
            strptr = strptr.offset(1);
            strptr;
        }
    }
    delim_end = strout;
    return if backslash_at_end as libc::c_int != 0 {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn xputchar(mut c: libc::c_char) {
    if putchar_unlocked(c as libc::c_int) < 0 as libc::c_int {
        write_error();
    }
}
unsafe extern "C" fn paste_parallel(
    mut nfiles: size_t,
    mut fnamptr: *mut *mut libc::c_char,
) -> bool {
    let mut ok: bool = 1 as libc::c_int != 0;
    let mut delbuf: *mut libc::c_char = xmalloc(
        nfiles.wrapping_add(2 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    let mut fileptr: *mut *mut FILE = xnmalloc(
        nfiles.wrapping_add(1 as libc::c_int as libc::c_ulong),
        ::core::mem::size_of::<*mut FILE>() as libc::c_ulong,
    ) as *mut *mut FILE;
    let mut files_open: size_t = 0;
    let mut opened_stdin: bool = 0 as libc::c_int != 0;
    files_open = 0 as libc::c_int as size_t;
    while files_open < nfiles {
        if strcmp(
            *fnamptr.offset(files_open as isize),
            b"-\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            have_read_stdin = 1 as libc::c_int != 0;
            let ref mut fresh11 = *fileptr.offset(files_open as isize);
            *fresh11 = stdin;
        } else {
            let ref mut fresh12 = *fileptr.offset(files_open as isize);
            *fresh12 = rpl_fopen(
                *fnamptr.offset(files_open as isize),
                b"r\0" as *const u8 as *const libc::c_char,
            );
            if (*fileptr.offset(files_open as isize)).is_null() {
                if 0 != 0 {
                    error(
                        1 as libc::c_int,
                        *__errno_location(),
                        b"%s\0" as *const u8 as *const libc::c_char,
                        quotearg_n_style_colon(
                            0 as libc::c_int,
                            shell_escape_quoting_style,
                            *fnamptr.offset(files_open as isize),
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
                                *fnamptr.offset(files_open as isize),
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
                                *fnamptr.offset(files_open as isize),
                            ),
                        );
                        if __errstatus != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                        
                    });
                };
            } else if fileno(*fileptr.offset(files_open as isize)) == 0 as libc::c_int {
                opened_stdin = 1 as libc::c_int != 0;
            }
            fadvise(*fileptr.offset(files_open as isize), FADVISE_SEQUENTIAL);
        }
        files_open = files_open.wrapping_add(1);
        files_open;
    }
    if opened_stdin as libc::c_int != 0 && have_read_stdin as libc::c_int != 0 {
        if 0 != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"standard input is closed\0" as *const u8 as *const libc::c_char,
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
                        b"standard input is closed\0" as *const u8 as *const libc::c_char,
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
                        b"standard input is closed\0" as *const u8 as *const libc::c_char,
                    ),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
    }
    while files_open != 0 {
        let mut somedone: bool = 0 as libc::c_int != 0;
        let mut delimptr: *const libc::c_char = delims;
        let mut delims_saved: size_t = 0 as libc::c_int as size_t;
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < nfiles && files_open != 0 {
            let mut chr: libc::c_int = 0;
            let mut err: libc::c_int = 0;
            let mut sometodo: bool = 0 as libc::c_int != 0;
            if !(*fileptr.offset(i as isize)).is_null() {
                chr = getc_unlocked(*fileptr.offset(i as isize));
                err = *__errno_location();
                if chr != -(1 as libc::c_int) && delims_saved != 0 {
                    if fwrite_unlocked(
                        delbuf as *const libc::c_void,
                        1 as libc::c_int as size_t,
                        delims_saved,
                        stdout,
                    ) != delims_saved
                    {
                        write_error();
                    }
                    delims_saved = 0 as libc::c_int as size_t;
                }
                while chr != -(1 as libc::c_int) {
                    sometodo = 1 as libc::c_int != 0;
                    if chr == line_delim as libc::c_int {
                        break;
                    }
                    xputchar(chr as libc::c_char);
                    chr = getc_unlocked(*fileptr.offset(i as isize));
                    err = *__errno_location();
                }
            }
            if !sometodo {
                if !(*fileptr.offset(i as isize)).is_null() {
                    if ferror_unlocked(*fileptr.offset(i as isize)) == 0 {
                        err = 0 as libc::c_int;
                    }
                    if *fileptr.offset(i as isize) == stdin {
                        clearerr_unlocked(*fileptr.offset(i as isize));
                    } else if rpl_fclose(*fileptr.offset(i as isize))
                        == -(1 as libc::c_int) && err == 0
                    {
                        err = *__errno_location();
                    }
                    if err != 0 {
                        if 0 != 0 {
                            error(
                                0 as libc::c_int,
                                err,
                                b"%s\0" as *const u8 as *const libc::c_char,
                                quotearg_n_style_colon(
                                    0 as libc::c_int,
                                    shell_escape_quoting_style,
                                    *fnamptr.offset(i as isize),
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
                                    err,
                                    b"%s\0" as *const u8 as *const libc::c_char,
                                    quotearg_n_style_colon(
                                        0 as libc::c_int,
                                        shell_escape_quoting_style,
                                        *fnamptr.offset(i as isize),
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
                                    err,
                                    b"%s\0" as *const u8 as *const libc::c_char,
                                    quotearg_n_style_colon(
                                        0 as libc::c_int,
                                        shell_escape_quoting_style,
                                        *fnamptr.offset(i as isize),
                                    ),
                                );
                                if __errstatus != 0 as libc::c_int {
                                    unreachable!();
                                } else {};
                                
                            });
                        };
                        ok = 0 as libc::c_int != 0;
                    }
                    let ref mut fresh13 = *fileptr.offset(i as isize);
                    *fresh13 = 0 as *mut FILE;
                    files_open = files_open.wrapping_sub(1);
                    files_open;
                }
                if i.wrapping_add(1 as libc::c_int as libc::c_ulong) == nfiles {
                    if somedone {
                        if delims_saved != 0 {
                            if fwrite_unlocked(
                                delbuf as *const libc::c_void,
                                1 as libc::c_int as size_t,
                                delims_saved,
                                stdout,
                            ) != delims_saved
                            {
                                write_error();
                            }
                            delims_saved = 0 as libc::c_int as size_t;
                        }
                        xputchar(line_delim as libc::c_char);
                    }
                } else {
                    if *delimptr as libc::c_int != '\0' as i32 {
                        let fresh14 = delims_saved;
                        delims_saved = delims_saved.wrapping_add(1);
                        *delbuf.offset(fresh14 as isize) = *delimptr;
                    }
                    delimptr = delimptr.offset(1);
                    if delimptr == delim_end {
                        delimptr = delims;
                    }
                }
            } else {
                somedone = 1 as libc::c_int != 0;
                if i.wrapping_add(1 as libc::c_int as libc::c_ulong) != nfiles {
                    if chr != line_delim as libc::c_int && chr != -(1 as libc::c_int) {
                        xputchar(chr as libc::c_char);
                    }
                    if *delimptr as libc::c_int != '\0' as i32 {
                        xputchar(*delimptr);
                    }
                    delimptr = delimptr.offset(1);
                    if delimptr == delim_end {
                        delimptr = delims;
                    }
                } else {
                    let mut c: libc::c_char = (if chr == -(1 as libc::c_int) {
                        line_delim as libc::c_int
                    } else {
                        chr
                    }) as libc::c_char;
                    xputchar(c);
                }
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    free(fileptr as *mut libc::c_void);
    free(delbuf as *mut libc::c_void);
    return ok;
}
unsafe extern "C" fn paste_serial(
    mut nfiles: size_t,
    mut fnamptr: *mut *mut libc::c_char,
) -> bool {
    let mut ok: bool = 1 as libc::c_int != 0;
    let mut charnew: libc::c_int = 0;
    let mut charold: libc::c_int = 0;
    let mut delimptr: *const libc::c_char = 0 as *const libc::c_char;
    let mut fileptr: *mut FILE = 0 as *mut FILE;
    let mut current_block_35: u64;
    while nfiles != 0 {
        let mut saved_errno: libc::c_int = 0;
        let mut is_stdin: bool = strcmp(
            *fnamptr,
            b"-\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int;
        if is_stdin {
            have_read_stdin = 1 as libc::c_int != 0;
            fileptr = stdin;
            current_block_35 = 11650488183268122163;
        } else {
            fileptr = rpl_fopen(*fnamptr, b"r\0" as *const u8 as *const libc::c_char);
            if fileptr.is_null() {
                if 0 != 0 {
                    error(
                        0 as libc::c_int,
                        *__errno_location(),
                        b"%s\0" as *const u8 as *const libc::c_char,
                        quotearg_n_style_colon(
                            0 as libc::c_int,
                            shell_escape_quoting_style,
                            *fnamptr,
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
                            *__errno_location(),
                            b"%s\0" as *const u8 as *const libc::c_char,
                            quotearg_n_style_colon(
                                0 as libc::c_int,
                                shell_escape_quoting_style,
                                *fnamptr,
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
                            *__errno_location(),
                            b"%s\0" as *const u8 as *const libc::c_char,
                            quotearg_n_style_colon(
                                0 as libc::c_int,
                                shell_escape_quoting_style,
                                *fnamptr,
                            ),
                        );
                        if __errstatus != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                        
                    });
                };
                ok = 0 as libc::c_int != 0;
                current_block_35 = 8258075665625361029;
            } else {
                fadvise(fileptr, FADVISE_SEQUENTIAL);
                current_block_35 = 11650488183268122163;
            }
        }
        match current_block_35 {
            11650488183268122163 => {
                delimptr = delims;
                charold = getc_unlocked(fileptr);
                saved_errno = *__errno_location();
                if charold != -(1 as libc::c_int) {
                    loop {
                        charnew = getc_unlocked(fileptr);
                        if !(charnew != -(1 as libc::c_int)) {
                            break;
                        }
                        if charold == line_delim as libc::c_int {
                            if *delimptr as libc::c_int != '\0' as i32 {
                                xputchar(*delimptr);
                            }
                            delimptr = delimptr.offset(1);
                            if delimptr == delim_end {
                                delimptr = delims;
                            }
                        } else {
                            xputchar(charold as libc::c_char);
                        }
                        charold = charnew;
                    }
                    saved_errno = *__errno_location();
                    xputchar(charold as libc::c_char);
                }
                if charold != line_delim as libc::c_int {
                    xputchar(line_delim as libc::c_char);
                }
                if ferror_unlocked(fileptr) == 0 {
                    saved_errno = 0 as libc::c_int;
                }
                if is_stdin {
                    clearerr_unlocked(fileptr);
                } else if rpl_fclose(fileptr) != 0 as libc::c_int && saved_errno == 0 {
                    saved_errno = *__errno_location();
                }
                if saved_errno != 0 {
                    if 0 != 0 {
                        error(
                            0 as libc::c_int,
                            saved_errno,
                            b"%s\0" as *const u8 as *const libc::c_char,
                            quotearg_n_style_colon(
                                0 as libc::c_int,
                                shell_escape_quoting_style,
                                *fnamptr,
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
                                saved_errno,
                                b"%s\0" as *const u8 as *const libc::c_char,
                                quotearg_n_style_colon(
                                    0 as libc::c_int,
                                    shell_escape_quoting_style,
                                    *fnamptr,
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
                                saved_errno,
                                b"%s\0" as *const u8 as *const libc::c_char,
                                quotearg_n_style_colon(
                                    0 as libc::c_int,
                                    shell_escape_quoting_style,
                                    *fnamptr,
                                ),
                            );
                            if __errstatus != 0 as libc::c_int {
                                unreachable!();
                            } else {};
                            
                        });
                    };
                    ok = 0 as libc::c_int != 0;
                }
            }
            _ => {}
        }
        nfiles = nfiles.wrapping_sub(1);
        nfiles;
        fnamptr = fnamptr.offset(1);
        fnamptr;
    }
    return ok;
}
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
                b"Usage: %s [OPTION]... [FILE]...\n\0" as *const u8
                    as *const libc::c_char,
            ),
            program_name,
        );
        fputs_unlocked(
            gettext(
                b"Write lines consisting of the sequentially corresponding lines from\neach FILE, separated by TABs, to standard output.\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        emit_stdin_note();
        emit_mandatory_arg_note();
        fputs_unlocked(
            gettext(
                b"  -d, --delimiters=LIST   reuse characters from LIST instead of TABs\n  -s, --serial            paste one file at a time instead of in parallel\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"  -z, --zero-terminated    line delimiter is NUL, not newline\n\0"
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
        emit_ancillary_info(b"paste\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut optc: libc::c_int = 0;
    let mut delim_arg: *const libc::c_char = b"\t\0" as *const u8 as *const libc::c_char;
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    have_read_stdin = 0 as libc::c_int != 0;
    serial_merge = 0 as libc::c_int != 0;
    loop {
        optc = getopt_long(
            argc,
            argv,
            b"d:sz\0" as *const u8 as *const libc::c_char,
            longopts.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(optc != -(1 as libc::c_int)) {
            break;
        }
        match optc {
            100 => {
                delim_arg = if *optarg.offset(0 as libc::c_int as isize) as libc::c_int
                    == '\0' as i32
                {
                    b"\\0\0" as *const u8 as *const libc::c_char
                } else {
                    optarg as *const libc::c_char
                };
            }
            115 => {
                serial_merge = 1 as libc::c_int != 0;
            }
            122 => {
                line_delim = '\0' as i32 as libc::c_uchar;
            }
            -2 => {
                usage(0 as libc::c_int);
            }
            -3 => {
                version_etc(
                    stdout,
                    b"paste\0" as *const u8 as *const libc::c_char,
                    b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                    Version,
                    proper_name_lite(
                        b"David M. Ihnat\0" as *const u8 as *const libc::c_char,
                        b"David M. Ihnat\0" as *const u8 as *const libc::c_char,
                    ),
                    proper_name_lite(
                        b"David MacKenzie\0" as *const u8 as *const libc::c_char,
                        b"David MacKenzie\0" as *const u8 as *const libc::c_char,
                    ),
                    0 as *mut libc::c_void as *mut libc::c_char,
                );
                exit(0 as libc::c_int);
            }
            _ => {
                usage(1 as libc::c_int);
            }
        }
    }
    let mut nfiles: libc::c_int = argc - optind;
    if nfiles == 0 as libc::c_int {
        let ref mut fresh15 = *argv.offset(optind as isize);
        *fresh15 = bad_cast(b"-\0" as *const u8 as *const libc::c_char);
        nfiles += 1;
        nfiles;
    }
    if collapse_escapes(delim_arg) != 0 {
        if 0 != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"delimiter list ends with an unescaped backslash: %s\0" as *const u8
                        as *const libc::c_char,
                ),
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    c_maybe_quoting_style,
                    delim_arg,
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
                        b"delimiter list ends with an unescaped backslash: %s\0"
                            as *const u8 as *const libc::c_char,
                    ),
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        c_maybe_quoting_style,
                        delim_arg,
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
                        b"delimiter list ends with an unescaped backslash: %s\0"
                            as *const u8 as *const libc::c_char,
                    ),
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        c_maybe_quoting_style,
                        delim_arg,
                    ),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
    }
    let mut ok: bool = if serial_merge as libc::c_int != 0 {
        Some(
            paste_serial as unsafe extern "C" fn(size_t, *mut *mut libc::c_char) -> bool,
        )
    } else {
        Some(
            paste_parallel
                as unsafe extern "C" fn(size_t, *mut *mut libc::c_char) -> bool,
        )
    }
        .expect(
            "non-null function pointer",
        )(nfiles as size_t, &mut *argv.offset(optind as isize));
    free(delims as *mut libc::c_void);
    if have_read_stdin as libc::c_int != 0 && rpl_fclose(stdin) == -(1 as libc::c_int) {
        if 0 != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                b"-\0" as *const u8 as *const libc::c_char,
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
                    b"-\0" as *const u8 as *const libc::c_char,
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
                    b"-\0" as *const u8 as *const libc::c_char,
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
    }
    return if ok as libc::c_int != 0 { 0 as libc::c_int } else { 1 as libc::c_int };
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
