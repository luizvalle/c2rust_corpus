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
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn lstat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn quotearg_style(s: quoting_style, arg: *const libc::c_char) -> *mut libc::c_char;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    static mut Version: *const libc::c_char;
    fn current_timespec() -> timespec;
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
    static mut argmatch_die: argmatch_exit_fn;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn __xargmatch_internal(
        context: *const libc::c_char,
        arg: *const libc::c_char,
        arglist: *const *const libc::c_char,
        vallist: *const libc::c_void,
        valsize: size_t,
        exit_fn: argmatch_exit_fn,
        allow_abbreviation: bool,
    ) -> ptrdiff_t;
    fn fd_reopen(
        _: libc::c_int,
        _: *const libc::c_char,
        _: libc::c_int,
        _: mode_t,
    ) -> libc::c_int;
    fn parse_datetime(
        _: *mut timespec,
        _: *const libc::c_char,
        _: *const timespec,
    ) -> bool;
    fn posixtime(
        p: *mut time_t,
        s: *const libc::c_char,
        syntax_bits: libc::c_uint,
    ) -> bool;
    fn posix2_version() -> libc::c_int;
    fn fdutimensat(
        fd: libc::c_int,
        dir: libc::c_int,
        name: *const libc::c_char,
        _: *const timespec,
        atflag: libc::c_int,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_int;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
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
pub type ptrdiff_t = libc::c_long;
pub type mode_t = __mode_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
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
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_mode: __mode_t,
    pub st_nlink: __nlink_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub st_rdev: __dev_t,
    pub __pad1: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub __pad2: libc::c_int,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [libc::c_int; 2],
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
pub type argmatch_exit_fn = Option::<unsafe extern "C" fn() -> ()>;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const TIME_OPTION: C2RustUnnamed_0 = 256;
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
unsafe extern "C" fn get_stat_atime(mut st: *const stat) -> timespec {
    return (*st).st_atim;
}
#[inline]
unsafe extern "C" fn get_stat_mtime(mut st: *const stat) -> timespec {
    return (*st).st_mtim;
}
static mut change_times: libc::c_int = 0;
static mut no_create: bool = false;
static mut use_ref: bool = false;
static mut no_dereference: bool = false;
static mut amtime_now: bool = false;
static mut newtime: [timespec; 2] = [timespec { tv_sec: 0, tv_nsec: 0 }; 2];
static mut ref_file: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut longopts: [option; 8] = [
    {
        let mut init = option {
            name: b"time\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: TIME_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-create\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'c' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"date\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'd' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"reference\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'r' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"no-dereference\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'h' as i32,
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
static mut time_args: [*const libc::c_char; 6] = [
    b"atime\0" as *const u8 as *const libc::c_char,
    b"access\0" as *const u8 as *const libc::c_char,
    b"use\0" as *const u8 as *const libc::c_char,
    b"mtime\0" as *const u8 as *const libc::c_char,
    b"modify\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut time_masks: [libc::c_int; 5] = [
    1 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    2 as libc::c_int,
    2 as libc::c_int,
];
unsafe extern "C" fn date_relative(
    mut flex_date: *const libc::c_char,
    mut now: timespec,
) -> timespec {
    let mut result: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    if !parse_datetime(&mut result, flex_date, &mut now) {
        if 0 != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                gettext(b"invalid date format %s\0" as *const u8 as *const libc::c_char),
                quote(flex_date),
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
                        b"invalid date format %s\0" as *const u8 as *const libc::c_char,
                    ),
                    quote(flex_date),
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
                        b"invalid date format %s\0" as *const u8 as *const libc::c_char,
                    ),
                    quote(flex_date),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
    }
    return result;
}
unsafe extern "C" fn touch(mut file: *const libc::c_char) -> bool {
    let mut fd: libc::c_int = -(1 as libc::c_int);
    let mut open_errno: libc::c_int = 0 as libc::c_int;
    let mut t: *const timespec = newtime.as_mut_ptr();
    if strcmp(file, b"-\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        fd = 1 as libc::c_int;
    } else if !(no_create as libc::c_int != 0 || no_dereference as libc::c_int != 0) {
        fd = fd_reopen(
            0 as libc::c_int,
            file,
            0o1 as libc::c_int | 0o100 as libc::c_int | 0o4000 as libc::c_int
                | 0o400 as libc::c_int,
            (0o400 as libc::c_int | 0o200 as libc::c_int
                | 0o400 as libc::c_int >> 3 as libc::c_int
                | 0o200 as libc::c_int >> 3 as libc::c_int
                | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int) as mode_t,
        );
        if fd < 0 as libc::c_int {
            open_errno = *__errno_location();
        }
    }
    if change_times != 1 as libc::c_int | 2 as libc::c_int {
        if change_times == 2 as libc::c_int {
            newtime[0 as libc::c_int as usize]
                .tv_nsec = ((1 as libc::c_long) << 30 as libc::c_int)
                - 2 as libc::c_long;
        } else {
            if change_times == 1 as libc::c_int {} else {
                __assert_fail(
                    b"change_times == 1\0" as *const u8 as *const libc::c_char,
                    b"touch.c\0" as *const u8 as *const libc::c_char,
                    145 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 26],
                        &[libc::c_char; 26],
                    >(b"_Bool touch(const char *)\0"))
                        .as_ptr(),
                );
            }
            'c_7826: {
                if change_times == 1 as libc::c_int {} else {
                    __assert_fail(
                        b"change_times == 1\0" as *const u8 as *const libc::c_char,
                        b"touch.c\0" as *const u8 as *const libc::c_char,
                        145 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 26],
                            &[libc::c_char; 26],
                        >(b"_Bool touch(const char *)\0"))
                            .as_ptr(),
                    );
                }
            };
            newtime[1 as libc::c_int as usize]
                .tv_nsec = ((1 as libc::c_long) << 30 as libc::c_int)
                - 2 as libc::c_long;
        }
    }
    if amtime_now {
        t = 0 as *const timespec;
    }
    let mut file_opt: *const libc::c_char = if fd == 1 as libc::c_int {
        0 as *const libc::c_char
    } else {
        file
    };
    let mut atflag: libc::c_int = if no_dereference as libc::c_int != 0 {
        0x100 as libc::c_int
    } else {
        0 as libc::c_int
    };
    let mut utime_errno: libc::c_int = if fdutimensat(
        fd,
        -(100 as libc::c_int),
        file_opt,
        t,
        atflag,
    ) == 0 as libc::c_int
    {
        0 as libc::c_int
    } else {
        *__errno_location()
    };
    if fd == 0 as libc::c_int {
        if close(0 as libc::c_int) != 0 as libc::c_int {
            if 0 != 0 {
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    gettext(b"failed to close %s\0" as *const u8 as *const libc::c_char),
                    quotearg_style(shell_escape_always_quoting_style, file),
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
                        gettext(
                            b"failed to close %s\0" as *const u8 as *const libc::c_char,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, file),
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
                        gettext(
                            b"failed to close %s\0" as *const u8 as *const libc::c_char,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, file),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
            return 0 as libc::c_int != 0;
        }
    } else if fd == 1 as libc::c_int {
        if utime_errno == 9 as libc::c_int && no_create as libc::c_int != 0 {
            return 1 as libc::c_int != 0;
        }
    }
    if utime_errno != 0 as libc::c_int {
        let mut st: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_mode: 0,
            st_nlink: 0,
            st_uid: 0,
            st_gid: 0,
            st_rdev: 0,
            __pad1: 0,
            st_size: 0,
            st_blksize: 0,
            __pad2: 0,
            st_blocks: 0,
            st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
            __glibc_reserved: [0; 2],
        };
        if open_errno != 0
            && !(open_errno == 21 as libc::c_int
                || open_errno == 22 as libc::c_int
                    && stat(file, &mut st) == 0 as libc::c_int
                    && st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                        == 0o40000 as libc::c_int as libc::c_uint)
        {
            if 0 != 0 {
                error(
                    0 as libc::c_int,
                    open_errno,
                    gettext(b"cannot touch %s\0" as *const u8 as *const libc::c_char),
                    quotearg_style(shell_escape_always_quoting_style, file),
                );
                if 0 as libc::c_int != 0 as libc::c_int {
                    unreachable!();
                } else {};
            } else {
                ({
                    let __errstatus: libc::c_int = 0 as libc::c_int;
                    error(
                        __errstatus,
                        open_errno,
                        gettext(
                            b"cannot touch %s\0" as *const u8 as *const libc::c_char,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, file),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
                ({
                    let __errstatus: libc::c_int = 0 as libc::c_int;
                    error(
                        __errstatus,
                        open_errno,
                        gettext(
                            b"cannot touch %s\0" as *const u8 as *const libc::c_char,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, file),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        } else {
            if no_create as libc::c_int != 0 && utime_errno == 2 as libc::c_int {
                return 1 as libc::c_int != 0;
            }
            if 0 != 0 {
                error(
                    0 as libc::c_int,
                    utime_errno,
                    gettext(
                        b"setting times of %s\0" as *const u8 as *const libc::c_char,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, file),
                );
                if 0 as libc::c_int != 0 as libc::c_int {
                    unreachable!();
                } else {};
            } else {
                ({
                    let __errstatus: libc::c_int = 0 as libc::c_int;
                    error(
                        __errstatus,
                        utime_errno,
                        gettext(
                            b"setting times of %s\0" as *const u8 as *const libc::c_char,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, file),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
                ({
                    let __errstatus: libc::c_int = 0 as libc::c_int;
                    error(
                        __errstatus,
                        utime_errno,
                        gettext(
                            b"setting times of %s\0" as *const u8 as *const libc::c_char,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, file),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
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
                b"Usage: %s [OPTION]... FILE...\n\0" as *const u8 as *const libc::c_char,
            ),
            program_name,
        );
        fputs_unlocked(
            gettext(
                b"Update the access and modification times of each FILE to the current time.\n\nA FILE argument that does not exist is created empty, unless -c or -h\nis supplied.\n\nA FILE argument string of - is handled specially and causes touch to\nchange the times of the file associated with standard output.\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        emit_mandatory_arg_note();
        fputs_unlocked(
            gettext(
                b"  -a                     change only the access time\n  -c, --no-create        do not create any files\n  -d, --date=STRING      parse STRING and use it instead of current time\n  -f                     (ignored)\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"  -h, --no-dereference   affect each symbolic link instead of any referenced\n                         file (useful only on systems that can change the\n                         timestamps of a symlink)\n  -m                     change only the modification time\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"  -r, --reference=FILE   use this file's times instead of current time\n  -t [[CC]YY]MMDDhhmm[.ss]  use specified time instead of current time,\n                         with a date-time format that differs from -d's\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"      --time=WORD        specify which time to change:\n                           access time (-a): 'access', 'atime', 'use';\n                           modification time (-m): 'modify', 'mtime'\n\0"
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
        emit_ancillary_info(b"touch\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut date_set: bool = 0 as libc::c_int != 0;
    let mut ok: bool = 1 as libc::c_int != 0;
    let mut flex_date: *const libc::c_char = 0 as *const libc::c_char;
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    change_times = 0 as libc::c_int;
    use_ref = 0 as libc::c_int != 0;
    no_create = use_ref;
    loop {
        c = getopt_long(
            argc,
            argv,
            b"acd:fhmr:t:\0" as *const u8 as *const libc::c_char,
            longopts.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            97 => {
                change_times |= 1 as libc::c_int;
            }
            99 => {
                no_create = 1 as libc::c_int != 0;
            }
            100 => {
                flex_date = optarg;
            }
            102 => {}
            104 => {
                no_dereference = 1 as libc::c_int != 0;
            }
            109 => {
                change_times |= 2 as libc::c_int;
            }
            114 => {
                use_ref = 1 as libc::c_int != 0;
                ref_file = optarg;
            }
            116 => {
                if !posixtime(
                    &mut (*newtime.as_mut_ptr().offset(0 as libc::c_int as isize))
                        .tv_sec,
                    optarg,
                    (0 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int)
                        as libc::c_uint,
                ) {
                    if 0 != 0 {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            gettext(
                                b"invalid date format %s\0" as *const u8
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
                                    b"invalid date format %s\0" as *const u8
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
                                    b"invalid date format %s\0" as *const u8
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
                newtime[0 as libc::c_int as usize]
                    .tv_nsec = 0 as libc::c_int as __syscall_slong_t;
                newtime[1 as libc::c_int as usize] = newtime[0 as libc::c_int as usize];
                date_set = 1 as libc::c_int != 0;
            }
            256 => {
                change_times
                    |= time_masks[__xargmatch_internal(
                        b"--time\0" as *const u8 as *const libc::c_char,
                        optarg,
                        time_args.as_ptr(),
                        time_masks.as_ptr() as *const libc::c_void,
                        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        argmatch_die,
                        1 as libc::c_int != 0,
                    ) as usize];
            }
            -2 => {
                usage(0 as libc::c_int);
            }
            -3 => {
                version_etc(
                    stdout,
                    b"touch\0" as *const u8 as *const libc::c_char,
                    b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                    Version,
                    proper_name_lite(
                        b"Paul Rubin\0" as *const u8 as *const libc::c_char,
                        b"Paul Rubin\0" as *const u8 as *const libc::c_char,
                    ),
                    proper_name_lite(
                        b"Arnold Robbins\0" as *const u8 as *const libc::c_char,
                        b"Arnold Robbins\0" as *const u8 as *const libc::c_char,
                    ),
                    proper_name_lite(
                        b"Jim Kingdon\0" as *const u8 as *const libc::c_char,
                        b"Jim Kingdon\0" as *const u8 as *const libc::c_char,
                    ),
                    proper_name_lite(
                        b"David MacKenzie\0" as *const u8 as *const libc::c_char,
                        b"David MacKenzie\0" as *const u8 as *const libc::c_char,
                    ),
                    proper_name_lite(
                        b"Randy Smith\0" as *const u8 as *const libc::c_char,
                        b"Randy Smith\0" as *const u8 as *const libc::c_char,
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
    if change_times == 0 as libc::c_int {
        change_times = 1 as libc::c_int | 2 as libc::c_int;
    }
    if date_set as libc::c_int != 0
        && (use_ref as libc::c_int != 0 || !flex_date.is_null())
    {
        if 0 != 0 {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"cannot specify times from more than one source\0" as *const u8
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
                        b"cannot specify times from more than one source\0" as *const u8
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
                        b"cannot specify times from more than one source\0" as *const u8
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
    if use_ref {
        let mut ref_stats: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_mode: 0,
            st_nlink: 0,
            st_uid: 0,
            st_gid: 0,
            st_rdev: 0,
            __pad1: 0,
            st_size: 0,
            st_blksize: 0,
            __pad2: 0,
            st_blocks: 0,
            st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
            __glibc_reserved: [0; 2],
        };
        if if no_dereference as libc::c_int != 0 {
            lstat(ref_file, &mut ref_stats)
        } else {
            stat(ref_file, &mut ref_stats)
        } != 0
        {
            if 0 != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    gettext(
                        b"failed to get attributes of %s\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, ref_file),
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
                            b"failed to get attributes of %s\0" as *const u8
                                as *const libc::c_char,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, ref_file),
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
                            b"failed to get attributes of %s\0" as *const u8
                                as *const libc::c_char,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, ref_file),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
        newtime[0 as libc::c_int as usize] = get_stat_atime(&mut ref_stats);
        newtime[1 as libc::c_int as usize] = get_stat_mtime(&mut ref_stats);
        date_set = 1 as libc::c_int != 0;
        if !flex_date.is_null() {
            if change_times & 1 as libc::c_int != 0 {
                newtime[0 as libc::c_int
                    as usize] = date_relative(
                    flex_date,
                    newtime[0 as libc::c_int as usize],
                );
            }
            if change_times & 2 as libc::c_int != 0 {
                newtime[1 as libc::c_int
                    as usize] = date_relative(
                    flex_date,
                    newtime[1 as libc::c_int as usize],
                );
            }
        }
    } else if !flex_date.is_null() {
        let mut now: timespec = current_timespec();
        newtime[0 as libc::c_int as usize] = date_relative(flex_date, now);
        newtime[1 as libc::c_int as usize] = newtime[0 as libc::c_int as usize];
        date_set = 1 as libc::c_int != 0;
        if change_times == 1 as libc::c_int | 2 as libc::c_int
            && newtime[0 as libc::c_int as usize].tv_sec == now.tv_sec
            && newtime[0 as libc::c_int as usize].tv_nsec == now.tv_nsec
        {
            let mut notnow: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
            let mut notnow1: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
            notnow.tv_sec = now.tv_sec ^ 1 as libc::c_int as libc::c_long;
            notnow.tv_nsec = now.tv_nsec;
            notnow1 = date_relative(flex_date, notnow);
            if notnow1.tv_sec == notnow.tv_sec && notnow1.tv_nsec == notnow.tv_nsec {
                date_set = 0 as libc::c_int != 0;
            }
        }
    }
    if !date_set && 2 as libc::c_int <= argc - optind
        && posix2_version() < 200112 as libc::c_int
        && posixtime(
            &mut (*newtime.as_mut_ptr().offset(0 as libc::c_int as isize)).tv_sec,
            *argv.offset(optind as isize),
            (1 as libc::c_int | 8 as libc::c_int) as libc::c_uint,
        ) as libc::c_int != 0
    {
        newtime[0 as libc::c_int as usize]
            .tv_nsec = 0 as libc::c_int as __syscall_slong_t;
        newtime[1 as libc::c_int as usize] = newtime[0 as libc::c_int as usize];
        date_set = 1 as libc::c_int != 0;
        if (getenv(b"POSIXLY_CORRECT\0" as *const u8 as *const libc::c_char)).is_null() {
            let mut tm: *const tm = localtime(
                &mut (*newtime.as_mut_ptr().offset(0 as libc::c_int as isize)).tv_sec,
            );
            if !tm.is_null() {
                if 0 != 0 {
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        gettext(
                            b"warning: 'touch %s' is obsolete; use 'touch -t %04ld%02d%02d%02d%02d.%02d'\0"
                                as *const u8 as *const libc::c_char,
                        ),
                        *argv.offset(optind as isize),
                        (*tm).tm_year as libc::c_long + 1900 as libc::c_long,
                        (*tm).tm_mon + 1 as libc::c_int,
                        (*tm).tm_mday,
                        (*tm).tm_hour,
                        (*tm).tm_min,
                        (*tm).tm_sec,
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
                                b"warning: 'touch %s' is obsolete; use 'touch -t %04ld%02d%02d%02d%02d.%02d'\0"
                                    as *const u8 as *const libc::c_char,
                            ),
                            *argv.offset(optind as isize),
                            (*tm).tm_year as libc::c_long + 1900 as libc::c_long,
                            (*tm).tm_mon + 1 as libc::c_int,
                            (*tm).tm_mday,
                            (*tm).tm_hour,
                            (*tm).tm_min,
                            (*tm).tm_sec,
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
                                b"warning: 'touch %s' is obsolete; use 'touch -t %04ld%02d%02d%02d%02d.%02d'\0"
                                    as *const u8 as *const libc::c_char,
                            ),
                            *argv.offset(optind as isize),
                            (*tm).tm_year as libc::c_long + 1900 as libc::c_long,
                            (*tm).tm_mon + 1 as libc::c_int,
                            (*tm).tm_mday,
                            (*tm).tm_hour,
                            (*tm).tm_min,
                            (*tm).tm_sec,
                        );
                        if __errstatus != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                        
                    });
                };
            }
        }
        optind += 1;
        optind;
    }
    if !date_set {
        if change_times == 1 as libc::c_int | 2 as libc::c_int {
            amtime_now = 1 as libc::c_int != 0;
        } else {
            newtime[0 as libc::c_int as usize]
                .tv_nsec = ((1 as libc::c_long) << 30 as libc::c_int)
                - 1 as libc::c_long;
            newtime[1 as libc::c_int as usize]
                .tv_nsec = newtime[0 as libc::c_int as usize].tv_nsec;
        }
    }
    if optind == argc {
        if 0 != 0 {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                gettext(b"missing file operand\0" as *const u8 as *const libc::c_char),
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
                        b"missing file operand\0" as *const u8 as *const libc::c_char,
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
                        b"missing file operand\0" as *const u8 as *const libc::c_char,
                    ),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
        usage(1 as libc::c_int);
    }
    while optind < argc {
        ok = (ok as libc::c_int & touch(*argv.offset(optind as isize)) as libc::c_int)
            as bool;
        optind += 1;
        optind;
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
