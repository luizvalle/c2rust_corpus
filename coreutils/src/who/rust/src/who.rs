#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut, unused_imports)]
#![feature(extern_types)]
#![feature(f16_and_f128)]
use ::rust::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut optind: libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn rpl_asprintf(
        result: *mut *mut libc::c_char,
        format: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn ttyname(__fd: libc::c_int) -> *mut libc::c_char;
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn fstatat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __buf: *mut stat,
        __flag: libc::c_int,
    ) -> libc::c_int;
    fn rpl_time(__tp: *mut time_t) -> time_t;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
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
    fn stpcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn quotearg_n_style_colon(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
    static mut Version: *const libc::c_char;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
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
    fn xalloc_die();
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xpalloc(
        pa: *mut libc::c_void,
        pn: *mut idx_t,
        n_incr_min: idx_t,
        n_max: ptrdiff_t,
        s: idx_t,
    ) -> *mut libc::c_void;
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
    fn imaxtostr(_: intmax_t, _: *mut libc::c_char) -> *mut libc::c_char;
    fn umaxtostr(_: uintmax_t, _: *mut libc::c_char) -> *mut libc::c_char;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn canon_host(host: *const libc::c_char) -> *mut libc::c_char;
    fn extract_trimmed_name(ut: *const STRUCT_UTMP) -> *mut libc::c_char;
    fn read_utmp(
        file: *const libc::c_char,
        n_entries: *mut idx_t,
        utmp_buf: *mut *mut STRUCT_UTMP,
        options: libc::c_int,
    ) -> libc::c_int;
    fn hard_locale(category: libc::c_int) -> bool;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type size_t = libc::c_ulong;
pub type __intmax_t = libc::c_long;
pub type __uintmax_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
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
pub type pid_t = __pid_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
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
pub type C2RustUnnamed = libc::c_uint;
pub const O_PATHSEARCH: C2RustUnnamed = 2097152;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
pub type idx_t = ptrdiff_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gl_utmp {
    pub ut_user: *mut libc::c_char,
    pub ut_id: *mut libc::c_char,
    pub ut_line: *mut libc::c_char,
    pub ut_host: *mut libc::c_char,
    pub ut_ts: timespec,
    pub ut_pid: pid_t,
    pub ut_session: pid_t,
    pub ut_type: libc::c_short,
    pub ut_exit: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub e_termination: libc::c_int,
    pub e_exit: libc::c_int,
}
pub type STRUCT_UTMP = gl_utmp;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const READ_UTMP_NO_BOOT_TIME: C2RustUnnamed_2 = 8;
pub const READ_UTMP_BOOT_TIME: C2RustUnnamed_2 = 4;
pub const READ_UTMP_USER_PROCESS: C2RustUnnamed_2 = 2;
pub const READ_UTMP_CHECK_PIDS: C2RustUnnamed_2 = 1;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const LOOKUP_OPTION: C2RustUnnamed_3 = 256;
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
unsafe extern "C" fn timetostr(
    mut t: time_t,
    mut buf: *mut libc::c_char,
) -> *mut libc::c_char {
    return if !((0 as libc::c_int as time_t) < -(1 as libc::c_int) as time_t) {
        imaxtostr(t, buf)
    } else {
        umaxtostr(t as uintmax_t, buf)
    };
}
#[inline]
unsafe extern "C" fn c_isprint(mut c: libc::c_int) -> bool {
    match c {
        32 | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 97 | 98 | 99 | 100 | 101
        | 102 | 103 | 104 | 105 | 106 | 107 | 108 | 109 | 110 | 111 | 112 | 113 | 114
        | 115 | 116 | 117 | 118 | 119 | 120 | 121 | 122 | 33 | 34 | 35 | 36 | 37 | 38
        | 39 | 40 | 41 | 42 | 43 | 44 | 45 | 46 | 47 | 58 | 59 | 60 | 61 | 62 | 63 | 64
        | 91 | 92 | 93 | 94 | 95 | 96 | 123 | 124 | 125 | 126 | 65 | 66 | 67 | 68 | 69
        | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82 | 83 | 84 | 85
        | 86 | 87 | 88 | 89 | 90 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
static mut do_lookup: bool = false;
static mut short_list: bool = false;
static mut short_output: bool = false;
static mut include_idle: bool = false;
static mut include_heading: bool = false;
static mut include_mesg: bool = false;
static mut include_exit: bool = false;
static mut need_boottime: bool = false;
static mut need_deadprocs: bool = false;
static mut need_login: bool = false;
static mut need_initspawn: bool = false;
static mut need_clockchange: bool = false;
static mut need_runlevel: bool = false;
static mut need_users: bool = false;
static mut my_line_only: bool = false;
static mut time_format: *const libc::c_char = 0 as *const libc::c_char;
static mut time_format_width: libc::c_int = 0;
static mut longopts: [option; 18] = [
    {
        let mut init = option {
            name: b"all\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'a' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"boot\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'b' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"count\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'q' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"dead\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'd' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"heading\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'H' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"login\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'l' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"lookup\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: LOOKUP_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"message\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'T' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"mesg\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'T' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"process\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'p' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"runlevel\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'r' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"short\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 's' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"time\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 't' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"users\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'u' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"writable\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'T' as i32,
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
static mut now: time_t = 0;
unsafe extern "C" fn idle_string(
    mut when: time_t,
    mut boottime: time_t,
) -> *const libc::c_char {
    if now
        == !(if (0 as libc::c_int as time_t) < -(1 as libc::c_int) as time_t {
            -(1 as libc::c_int) as time_t
        } else {
            (((1 as libc::c_int as time_t)
                << (::core::mem::size_of::<time_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                - 1 as libc::c_int as libc::c_long) * 2 as libc::c_int as libc::c_long
                + 1 as libc::c_int as libc::c_long
        })
    {
        rpl_time(&mut now);
    }
    let mut seconds_idle: libc::c_int = 0;
    if boottime < when && when <= now
        && {
            let (fresh0, fresh1) = now.overflowing_sub(when);
            *(&mut seconds_idle as *mut libc::c_int) = fresh0;
            !fresh1
        } && seconds_idle < 24 as libc::c_int * 60 as libc::c_int * 60 as libc::c_int
    {
        if seconds_idle < 60 as libc::c_int {
            return b"  .  \0" as *const u8 as *const libc::c_char
        } else {
            static mut idle_hhmm: [libc::c_char; 6] = [0; 6];
            sprintf(
                idle_hhmm.as_mut_ptr(),
                b"%02d:%02d\0" as *const u8 as *const libc::c_char,
                seconds_idle / (60 as libc::c_int * 60 as libc::c_int),
                seconds_idle % (60 as libc::c_int * 60 as libc::c_int)
                    / 60 as libc::c_int,
            );
            return idle_hhmm.as_mut_ptr();
        }
    }
    return gettext(b" old \0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn time_string(mut utmp_ent: *const gl_utmp) -> *const libc::c_char {
    static mut buf: [libc::c_char; 33] = [0; 33];
    let mut tmp: *mut tm = localtime(&(*utmp_ent).ut_ts.tv_sec);
    if !tmp.is_null() {
        strftime(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong,
            time_format,
            tmp,
        );
        return buf.as_mut_ptr();
    } else {
        return timetostr((*utmp_ent).ut_ts.tv_sec, buf.as_mut_ptr())
    };
}
unsafe extern "C" fn print_line(
    mut user: *const libc::c_char,
    state: libc::c_char,
    mut line: *const libc::c_char,
    mut time_str: *const libc::c_char,
    mut idle: *const libc::c_char,
    mut pid: *const libc::c_char,
    mut comment: *const libc::c_char,
    mut exitstr: *const libc::c_char,
) {
    static mut mesg: [libc::c_char; 3] = [
        ' ' as i32 as libc::c_char,
        'x' as i32 as libc::c_char,
        '\0' as i32 as libc::c_char,
    ];
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut x_idle: [libc::c_char; 8] = [0; 8];
    let mut x_pid: [libc::c_char; 13] = [0; 13];
    let mut x_exitstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut err: libc::c_int = 0;
    mesg[1 as libc::c_int as usize] = state;
    if include_idle as libc::c_int != 0 && !short_output
        && strlen(idle)
            < (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        sprintf(
            x_idle.as_mut_ptr(),
            b" %-6s\0" as *const u8 as *const libc::c_char,
            idle,
        );
    } else {
        *x_idle.as_mut_ptr() = '\0' as i32 as libc::c_char;
    }
    if !short_output
        && strlen(pid)
            < (::core::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        sprintf(x_pid.as_mut_ptr(), b" %10s\0" as *const u8 as *const libc::c_char, pid);
    } else {
        *x_pid.as_mut_ptr() = '\0' as i32 as libc::c_char;
    }
    x_exitstr = xmalloc(
        if include_exit as libc::c_int != 0 {
            (1 as libc::c_int as libc::c_ulong)
                .wrapping_add(
                    (if 12 as libc::c_int as libc::c_ulong > strlen(exitstr) {
                        12 as libc::c_int as libc::c_ulong
                    } else {
                        strlen(exitstr)
                    }),
                )
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
        } else {
            1 as libc::c_int as libc::c_ulong
        },
    ) as *mut libc::c_char;
    if include_exit {
        sprintf(x_exitstr, b" %-12s\0" as *const u8 as *const libc::c_char, exitstr);
    } else {
        *x_exitstr = '\0' as i32 as libc::c_char;
    }
    err = rpl_asprintf(
        &mut buf as *mut *mut libc::c_char,
        b"%-8s%s %-12s %-*s%s%s %-8s%s\0" as *const u8 as *const libc::c_char,
        if !user.is_null() {
            user
        } else {
            b"   .\0" as *const u8 as *const libc::c_char
        },
        if include_mesg as libc::c_int != 0 {
            mesg.as_mut_ptr() as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        line,
        time_format_width,
        time_str,
        x_idle.as_mut_ptr(),
        x_pid.as_mut_ptr(),
        comment,
        x_exitstr,
    );
    if err == -(1 as libc::c_int) {
        xalloc_die();
    }
    let mut p: *mut libc::c_char = buf.offset(strlen(buf) as isize);
    loop {
        p = p.offset(-1);
        if !(*p as libc::c_int == ' ' as i32) {
            break;
        }
    }
    *p.offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    puts(buf);
    free(buf as *mut libc::c_void);
    free(x_exitstr as *mut libc::c_void);
}
unsafe extern "C" fn is_tty_writable(mut pstat: *const stat) -> bool {
    return (*pstat).st_mode & (0o200 as libc::c_int >> 3 as libc::c_int) as libc::c_uint
        != 0;
}
unsafe extern "C" fn print_user(mut utmp_ent: *const gl_utmp, mut boottime: time_t) {
    let mut stats: stat = stat {
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
    let mut last_change: time_t = 0;
    let mut mesg: libc::c_char = 0;
    let mut idlestr: [libc::c_char; 7] = [0; 7];
    let mut pidstr: [libc::c_char; 12] = [0; 12];
    sprintf(
        pidstr.as_mut_ptr(),
        b"%ld\0" as *const u8 as *const libc::c_char,
        (*utmp_ent).ut_pid as libc::c_long,
    );
    static mut hoststr: *mut libc::c_char = 0 as *const libc::c_char
        as *mut libc::c_char;
    static mut hostlen: idx_t = 0;
    let mut line: *mut libc::c_char = (*utmp_ent).ut_line;
    let mut space: *mut libc::c_char = strchr(line, ' ' as i32);
    line = if !space.is_null() { space.offset(1 as libc::c_int as isize) } else { line };
    let mut dirfd: libc::c_int = 0;
    if *line.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32 {
        dirfd = -(100 as libc::c_int);
    } else {
        static mut dev_dirfd: libc::c_int = 0;
        if dev_dirfd == 0 {
            dev_dirfd = open(
                b"/dev\0" as *const u8 as *const libc::c_char,
                O_PATHSEARCH as libc::c_int | 0o40000 as libc::c_int,
            );
            if dev_dirfd < 0 as libc::c_int {
                dev_dirfd = -(100 as libc::c_int) - 1 as libc::c_int;
            }
        }
        dirfd = dev_dirfd;
    }
    if -(100 as libc::c_int) <= dirfd
        && fstatat(dirfd, line, &mut stats, 0 as libc::c_int) == 0 as libc::c_int
    {
        mesg = (if is_tty_writable(&mut stats) as libc::c_int != 0 {
            '+' as i32
        } else {
            '-' as i32
        }) as libc::c_char;
        last_change = stats.st_atim.tv_sec;
    } else {
        mesg = '?' as i32 as libc::c_char;
        last_change = 0 as libc::c_int as time_t;
    }
    if last_change != 0 {
        sprintf(
            idlestr.as_mut_ptr(),
            b"%.*s\0" as *const u8 as *const libc::c_char,
            6 as libc::c_int,
            idle_string(last_change, boottime),
        );
    } else {
        sprintf(idlestr.as_mut_ptr(), b"  ?\0" as *const u8 as *const libc::c_char);
    }
    if *((*utmp_ent).ut_host).offset(0 as libc::c_int as isize) != 0 {
        let mut host: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut display: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut ut_host: *mut libc::c_char = (*utmp_ent).ut_host;
        display = strchr(ut_host, ':' as i32);
        if !display.is_null() {
            let fresh2 = display;
            display = display.offset(1);
            *fresh2 = '\0' as i32 as libc::c_char;
        }
        if *ut_host as libc::c_int != 0 && do_lookup as libc::c_int != 0 {
            host = canon_host(ut_host);
        }
        if host.is_null() {
            host = ut_host;
        }
        if !display.is_null() {
            let mut needed: idx_t = (strlen(host))
                .wrapping_add(strlen(display))
                .wrapping_add(4 as libc::c_int as libc::c_ulong) as idx_t;
            if hostlen < needed {
                free(hoststr as *mut libc::c_void);
                hoststr = xpalloc(
                    0 as *mut libc::c_void,
                    &mut hostlen,
                    needed - hostlen,
                    -(1 as libc::c_int) as ptrdiff_t,
                    1 as libc::c_int as idx_t,
                ) as *mut libc::c_char;
            }
            let mut p: *mut libc::c_char = hoststr;
            let fresh3 = p;
            p = p.offset(1);
            *fresh3 = '(' as i32 as libc::c_char;
            p = stpcpy(p, host);
            let fresh4 = p;
            p = p.offset(1);
            *fresh4 = ':' as i32 as libc::c_char;
            strcpy(stpcpy(p, display), b")\0" as *const u8 as *const libc::c_char);
        } else {
            let mut needed_0: idx_t = (strlen(host))
                .wrapping_add(3 as libc::c_int as libc::c_ulong) as idx_t;
            if hostlen < needed_0 {
                free(hoststr as *mut libc::c_void);
                hoststr = xpalloc(
                    0 as *mut libc::c_void,
                    &mut hostlen,
                    needed_0 - hostlen,
                    -(1 as libc::c_int) as ptrdiff_t,
                    1 as libc::c_int as idx_t,
                ) as *mut libc::c_char;
            }
            let mut p_0: *mut libc::c_char = hoststr;
            let fresh5 = p_0;
            p_0 = p_0.offset(1);
            *fresh5 = '(' as i32 as libc::c_char;
            strcpy(stpcpy(p_0, host), b")\0" as *const u8 as *const libc::c_char);
        }
        if host != ut_host {
            free(host as *mut libc::c_void);
        }
    } else {
        if hostlen < 1 as libc::c_int as libc::c_long {
            hoststr = xpalloc(
                hoststr as *mut libc::c_void,
                &mut hostlen,
                1 as libc::c_int as idx_t,
                -(1 as libc::c_int) as ptrdiff_t,
                1 as libc::c_int as idx_t,
            ) as *mut libc::c_char;
        }
        *hoststr = '\0' as i32 as libc::c_char;
    }
    print_line(
        (*utmp_ent).ut_user,
        mesg,
        (*utmp_ent).ut_line,
        time_string(utmp_ent),
        idlestr.as_mut_ptr(),
        pidstr.as_mut_ptr(),
        if !hoststr.is_null() {
            hoststr as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        b"\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn print_boottime(mut utmp_ent: *const gl_utmp) {
    print_line(
        b"\0" as *const u8 as *const libc::c_char,
        ' ' as i32 as libc::c_char,
        gettext(b"system boot\0" as *const u8 as *const libc::c_char),
        time_string(utmp_ent),
        b"\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn make_id_equals_comment(
    mut utmp_ent: *const gl_utmp,
) -> *mut libc::c_char {
    let mut id: *const libc::c_char = (*utmp_ent).ut_id;
    let mut idlen: idx_t = strlen(id) as idx_t;
    let mut prefix: *const libc::c_char = gettext(
        b"id=\0" as *const u8 as *const libc::c_char,
    );
    let mut prefixlen: idx_t = strlen(prefix) as idx_t;
    let mut comment: *mut libc::c_char = xmalloc(
        (prefixlen + idlen + 1 as libc::c_int as libc::c_long) as size_t,
    ) as *mut libc::c_char;
    let mut p: *mut libc::c_char = mempcpy(
        comment as *mut libc::c_void,
        prefix as *const libc::c_void,
        prefixlen as libc::c_ulong,
    ) as *mut libc::c_char;
    p = mempcpy(
        p as *mut libc::c_void,
        id as *const libc::c_void,
        idlen as libc::c_ulong,
    ) as *mut libc::c_char;
    *p = '\0' as i32 as libc::c_char;
    return comment;
}
unsafe extern "C" fn print_deadprocs(mut utmp_ent: *const gl_utmp) {
    static mut exitstr: *mut libc::c_char = 0 as *const libc::c_char
        as *mut libc::c_char;
    let mut comment: *mut libc::c_char = make_id_equals_comment(utmp_ent);
    let mut pidstr: [libc::c_char; 12] = [0; 12];
    sprintf(
        pidstr.as_mut_ptr(),
        b"%ld\0" as *const u8 as *const libc::c_char,
        (*utmp_ent).ut_pid as libc::c_long,
    );
    if exitstr.is_null() {
        exitstr = xmalloc(
            (strlen(gettext(b"term=\0" as *const u8 as *const libc::c_char)))
                .wrapping_add(
                    (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(
                            !((0 as libc::c_int) < -(1 as libc::c_int)) as libc::c_int
                                as libc::c_ulong,
                        )
                        .wrapping_mul(146 as libc::c_int as libc::c_ulong)
                        .wrapping_add(484 as libc::c_int as libc::c_ulong)
                        .wrapping_div(485 as libc::c_int as libc::c_ulong)
                        .wrapping_add(
                            !((0 as libc::c_int) < -(1 as libc::c_int)) as libc::c_int
                                as libc::c_ulong,
                        ),
                )
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(
                    strlen(gettext(b"exit=\0" as *const u8 as *const libc::c_char)),
                )
                .wrapping_add(
                    (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(
                            !((0 as libc::c_int) < -(1 as libc::c_int)) as libc::c_int
                                as libc::c_ulong,
                        )
                        .wrapping_mul(146 as libc::c_int as libc::c_ulong)
                        .wrapping_add(484 as libc::c_int as libc::c_ulong)
                        .wrapping_div(485 as libc::c_int as libc::c_ulong)
                        .wrapping_add(
                            !((0 as libc::c_int) < -(1 as libc::c_int)) as libc::c_int
                                as libc::c_ulong,
                        ),
                )
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
    }
    sprintf(
        exitstr,
        b"%s%d %s%d\0" as *const u8 as *const libc::c_char,
        gettext(b"term=\0" as *const u8 as *const libc::c_char),
        (*utmp_ent).ut_exit.e_termination,
        gettext(b"exit=\0" as *const u8 as *const libc::c_char),
        (*utmp_ent).ut_exit.e_exit,
    );
    print_line(
        b"\0" as *const u8 as *const libc::c_char,
        ' ' as i32 as libc::c_char,
        (*utmp_ent).ut_line,
        time_string(utmp_ent),
        b"\0" as *const u8 as *const libc::c_char,
        pidstr.as_mut_ptr(),
        comment,
        exitstr,
    );
    free(comment as *mut libc::c_void);
}
unsafe extern "C" fn print_login(mut utmp_ent: *const gl_utmp) {
    let mut comment: *mut libc::c_char = make_id_equals_comment(utmp_ent);
    let mut pidstr: [libc::c_char; 12] = [0; 12];
    sprintf(
        pidstr.as_mut_ptr(),
        b"%ld\0" as *const u8 as *const libc::c_char,
        (*utmp_ent).ut_pid as libc::c_long,
    );
    print_line(
        gettext(b"LOGIN\0" as *const u8 as *const libc::c_char),
        ' ' as i32 as libc::c_char,
        (*utmp_ent).ut_line,
        time_string(utmp_ent),
        b"\0" as *const u8 as *const libc::c_char,
        pidstr.as_mut_ptr(),
        comment,
        b"\0" as *const u8 as *const libc::c_char,
    );
    free(comment as *mut libc::c_void);
}
unsafe extern "C" fn print_initspawn(mut utmp_ent: *const gl_utmp) {
    let mut comment: *mut libc::c_char = make_id_equals_comment(utmp_ent);
    let mut pidstr: [libc::c_char; 12] = [0; 12];
    sprintf(
        pidstr.as_mut_ptr(),
        b"%ld\0" as *const u8 as *const libc::c_char,
        (*utmp_ent).ut_pid as libc::c_long,
    );
    print_line(
        b"\0" as *const u8 as *const libc::c_char,
        ' ' as i32 as libc::c_char,
        (*utmp_ent).ut_line,
        time_string(utmp_ent),
        b"\0" as *const u8 as *const libc::c_char,
        pidstr.as_mut_ptr(),
        comment,
        b"\0" as *const u8 as *const libc::c_char,
    );
    free(comment as *mut libc::c_void);
}
unsafe extern "C" fn print_clockchange(mut utmp_ent: *const gl_utmp) {
    print_line(
        b"\0" as *const u8 as *const libc::c_char,
        ' ' as i32 as libc::c_char,
        gettext(b"clock change\0" as *const u8 as *const libc::c_char),
        time_string(utmp_ent),
        b"\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn print_runlevel(mut utmp_ent: *const gl_utmp) {
    static mut runlevline: *mut libc::c_char = 0 as *const libc::c_char
        as *mut libc::c_char;
    static mut comment: *mut libc::c_char = 0 as *const libc::c_char
        as *mut libc::c_char;
    let mut last: libc::c_uchar = ((*utmp_ent).ut_pid / 256 as libc::c_int)
        as libc::c_uchar;
    let mut curr: libc::c_uchar = ((*utmp_ent).ut_pid % 256 as libc::c_int)
        as libc::c_uchar;
    if runlevline.is_null() {
        runlevline = xmalloc(
            (strlen(gettext(b"run-level\0" as *const u8 as *const libc::c_char)))
                .wrapping_add(3 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
    }
    sprintf(
        runlevline,
        b"%s %c\0" as *const u8 as *const libc::c_char,
        gettext(b"run-level\0" as *const u8 as *const libc::c_char),
        curr as libc::c_int,
    );
    if comment.is_null() {
        comment = xmalloc(
            (strlen(gettext(b"last=\0" as *const u8 as *const libc::c_char)))
                .wrapping_add(2 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
    }
    sprintf(
        comment,
        b"%s%c\0" as *const u8 as *const libc::c_char,
        gettext(b"last=\0" as *const u8 as *const libc::c_char),
        if last as libc::c_int == 'N' as i32 { 'S' as i32 } else { last as libc::c_int },
    );
    print_line(
        b"\0" as *const u8 as *const libc::c_char,
        ' ' as i32 as libc::c_char,
        runlevline,
        time_string(utmp_ent),
        b"\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
        if c_isprint(last as libc::c_int) as libc::c_int != 0 {
            comment as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        b"\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn list_entries_who(mut n: idx_t, mut utmp_buf: *const gl_utmp) {
    let mut entries: idx_t = 0 as libc::c_int as idx_t;
    let mut separator: *const libc::c_char = b"\0" as *const u8 as *const libc::c_char;
    loop {
        let fresh6 = n;
        n = n - 1;
        if !(fresh6 != 0) {
            break;
        }
        if *((*utmp_buf).ut_user).offset(0 as libc::c_int as isize) as libc::c_int != 0
            && (*utmp_buf).ut_type as libc::c_int == 7 as libc::c_int
        {
            let mut trimmed_name: *mut libc::c_char = 0 as *mut libc::c_char;
            trimmed_name = extract_trimmed_name(utmp_buf);
            printf(
                b"%s%s\0" as *const u8 as *const libc::c_char,
                separator,
                trimmed_name,
            );
            free(trimmed_name as *mut libc::c_void);
            separator = b" \0" as *const u8 as *const libc::c_char;
            entries += 1;
            entries;
        }
        utmp_buf = utmp_buf.offset(1);
        utmp_buf;
    }
    printf(gettext(b"\n# users=%td\n\0" as *const u8 as *const libc::c_char), entries);
}
unsafe extern "C" fn print_heading() {
    print_line(
        gettext(b"NAME\0" as *const u8 as *const libc::c_char),
        ' ' as i32 as libc::c_char,
        gettext(b"LINE\0" as *const u8 as *const libc::c_char),
        gettext(b"TIME\0" as *const u8 as *const libc::c_char),
        gettext(b"IDLE\0" as *const u8 as *const libc::c_char),
        gettext(b"PID\0" as *const u8 as *const libc::c_char),
        gettext(b"COMMENT\0" as *const u8 as *const libc::c_char),
        gettext(b"EXIT\0" as *const u8 as *const libc::c_char),
    );
}
unsafe extern "C" fn scan_entries(mut n: idx_t, mut utmp_buf: *const gl_utmp) {
    let mut ttyname_b: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut boottime: time_t = !if (0 as libc::c_int as time_t)
        < -(1 as libc::c_int) as time_t
    {
        -(1 as libc::c_int) as time_t
    } else {
        (((1 as libc::c_int as time_t)
            << (::core::mem::size_of::<time_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
            - 1 as libc::c_int as libc::c_long) * 2 as libc::c_int as libc::c_long
            + 1 as libc::c_int as libc::c_long
    };
    if include_heading {
        print_heading();
    }
    if my_line_only {
        ttyname_b = ttyname(0 as libc::c_int);
        if ttyname_b.is_null() {
            return;
        }
        if strncmp(
            ttyname_b,
            b"/dev/\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) == 0 as libc::c_int
        {
            ttyname_b = ttyname_b
                .offset(
                    (::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                );
        }
    }
    loop {
        let fresh7 = n;
        n = n - 1;
        if !(fresh7 != 0) {
            break;
        }
        if !my_line_only || strcmp(ttyname_b, (*utmp_buf).ut_line) == 0 as libc::c_int {
            if need_users as libc::c_int != 0
                && (*((*utmp_buf).ut_user).offset(0 as libc::c_int as isize)
                    as libc::c_int != 0
                    && (*utmp_buf).ut_type as libc::c_int == 7 as libc::c_int)
            {
                print_user(utmp_buf, boottime);
            } else if need_runlevel as libc::c_int != 0
                && (*utmp_buf).ut_type as libc::c_int == 1 as libc::c_int
            {
                print_runlevel(utmp_buf);
            } else if need_boottime as libc::c_int != 0
                && (*utmp_buf).ut_type as libc::c_int == 2 as libc::c_int
            {
                print_boottime(utmp_buf);
            } else if need_clockchange as libc::c_int != 0
                && (*utmp_buf).ut_type as libc::c_int == 3 as libc::c_int
            {
                print_clockchange(utmp_buf);
            } else if need_initspawn as libc::c_int != 0
                && (*utmp_buf).ut_type as libc::c_int == 5 as libc::c_int
            {
                print_initspawn(utmp_buf);
            } else if need_login as libc::c_int != 0
                && (*utmp_buf).ut_type as libc::c_int == 6 as libc::c_int
            {
                print_login(utmp_buf);
            } else if need_deadprocs as libc::c_int != 0
                && (*utmp_buf).ut_type as libc::c_int == 8 as libc::c_int
            {
                print_deadprocs(utmp_buf);
            }
        }
        if (*utmp_buf).ut_type as libc::c_int == 2 as libc::c_int {
            boottime = (*utmp_buf).ut_ts.tv_sec;
        }
        utmp_buf = utmp_buf.offset(1);
        utmp_buf;
    };
}
unsafe extern "C" fn who(mut filename: *const libc::c_char, mut options: libc::c_int) {
    let mut n_users: idx_t = 0;
    let mut utmp_buf: *mut gl_utmp = 0 as *mut gl_utmp;
    if short_list {
        options |= READ_UTMP_USER_PROCESS as libc::c_int;
    }
    if read_utmp(filename, &mut n_users, &mut utmp_buf, options) != 0 as libc::c_int {
        if 0 != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                b"%s\0" as *const u8 as *const libc::c_char,
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    filename,
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
                        filename,
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
                        filename,
                    ),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
    }
    if short_list {
        list_entries_who(n_users, utmp_buf);
    } else {
        scan_entries(n_users, utmp_buf);
    }
    free(utmp_buf as *mut libc::c_void);
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
                b"Usage: %s [OPTION]... [ FILE | ARG1 ARG2 ]\n\0" as *const u8
                    as *const libc::c_char,
            ),
            program_name,
        );
        fputs_unlocked(
            gettext(
                b"Print information about users who are currently logged in.\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"\n  -a, --all         same as -b -d --login -p -r -t -T -u\n  -b, --boot        time of last system boot\n  -d, --dead        print dead processes\n  -H, --heading     print line of column headings\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"  -l, --login       print system login processes\n\0" as *const u8
                    as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"      --lookup      attempt to canonicalize hostnames via DNS\n  -m                only hostname and user associated with stdin\n  -p, --process     print active processes spawned by init\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"  -q, --count       all login names and number of users logged on\n  -r, --runlevel    print current runlevel\n  -s, --short       print only name, line, and time (default)\n  -t, --time        print last system clock change\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"  -T, -w, --mesg    add user's message status as +, - or ?\n  -u, --users       list users logged in\n      --message     same as -T\n      --writable    same as -T\n\0"
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
        printf(
            gettext(
                b"\nIf FILE is not specified, use %s.  %s as FILE is common.\nIf ARG1 ARG2 given, -m presumed: 'am i' or 'mom likes' are usual.\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            b"/var/run/utmp\0" as *const u8 as *const libc::c_char,
            b"/var/log/wtmp\0" as *const u8 as *const libc::c_char,
        );
        emit_ancillary_info(b"who\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut optc: libc::c_int = 0;
    let mut assumptions: bool = 1 as libc::c_int != 0;
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    loop {
        optc = getopt_long(
            argc,
            argv,
            b"abdlmpqrstuwHT\0" as *const u8 as *const libc::c_char,
            longopts.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(optc != -(1 as libc::c_int)) {
            break;
        }
        match optc {
            97 => {
                need_boottime = 1 as libc::c_int != 0;
                need_deadprocs = 1 as libc::c_int != 0;
                need_login = 1 as libc::c_int != 0;
                need_initspawn = 1 as libc::c_int != 0;
                need_runlevel = 1 as libc::c_int != 0;
                need_clockchange = 1 as libc::c_int != 0;
                need_users = 1 as libc::c_int != 0;
                include_mesg = 1 as libc::c_int != 0;
                include_idle = 1 as libc::c_int != 0;
                include_exit = 1 as libc::c_int != 0;
                assumptions = 0 as libc::c_int != 0;
            }
            98 => {
                need_boottime = 1 as libc::c_int != 0;
                assumptions = 0 as libc::c_int != 0;
            }
            100 => {
                need_deadprocs = 1 as libc::c_int != 0;
                include_idle = 1 as libc::c_int != 0;
                include_exit = 1 as libc::c_int != 0;
                assumptions = 0 as libc::c_int != 0;
            }
            72 => {
                include_heading = 1 as libc::c_int != 0;
            }
            108 => {
                need_login = 1 as libc::c_int != 0;
                include_idle = 1 as libc::c_int != 0;
                assumptions = 0 as libc::c_int != 0;
            }
            109 => {
                my_line_only = 1 as libc::c_int != 0;
            }
            112 => {
                need_initspawn = 1 as libc::c_int != 0;
                assumptions = 0 as libc::c_int != 0;
            }
            113 => {
                short_list = 1 as libc::c_int != 0;
            }
            114 => {
                need_runlevel = 1 as libc::c_int != 0;
                include_idle = 1 as libc::c_int != 0;
                assumptions = 0 as libc::c_int != 0;
            }
            115 => {
                short_output = 1 as libc::c_int != 0;
            }
            116 => {
                need_clockchange = 1 as libc::c_int != 0;
                assumptions = 0 as libc::c_int != 0;
            }
            84 | 119 => {
                include_mesg = 1 as libc::c_int != 0;
            }
            117 => {
                need_users = 1 as libc::c_int != 0;
                include_idle = 1 as libc::c_int != 0;
                assumptions = 0 as libc::c_int != 0;
            }
            256 => {
                do_lookup = 1 as libc::c_int != 0;
            }
            -2 => {
                usage(0 as libc::c_int);
            }
            -3 => {
                version_etc(
                    stdout,
                    b"who\0" as *const u8 as *const libc::c_char,
                    b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                    Version,
                    proper_name_lite(
                        b"Joseph Arceneaux\0" as *const u8 as *const libc::c_char,
                        b"Joseph Arceneaux\0" as *const u8 as *const libc::c_char,
                    ),
                    proper_name_lite(
                        b"David MacKenzie\0" as *const u8 as *const libc::c_char,
                        b"David MacKenzie\0" as *const u8 as *const libc::c_char,
                    ),
                    proper_name_lite(
                        b"Michael Stone\0" as *const u8 as *const libc::c_char,
                        b"Michael Stone\0" as *const u8 as *const libc::c_char,
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
    if assumptions {
        need_users = 1 as libc::c_int != 0;
        short_output = 1 as libc::c_int != 0;
    }
    if include_exit {
        short_output = 0 as libc::c_int != 0;
    }
    if hard_locale(2 as libc::c_int) {
        time_format = b"%Y-%m-%d %H:%M\0" as *const u8 as *const libc::c_char;
        time_format_width = 4 as libc::c_int + 1 as libc::c_int + 2 as libc::c_int
            + 1 as libc::c_int + 2 as libc::c_int + 1 as libc::c_int + 2 as libc::c_int
            + 1 as libc::c_int + 2 as libc::c_int;
    } else {
        time_format = b"%b %e %H:%M\0" as *const u8 as *const libc::c_char;
        time_format_width = 3 as libc::c_int + 1 as libc::c_int + 2 as libc::c_int
            + 1 as libc::c_int + 2 as libc::c_int + 1 as libc::c_int + 2 as libc::c_int;
    }
    let mut current_block_70: u64;
    match argc - optind {
        2 => {
            my_line_only = 1 as libc::c_int != 0;
            current_block_70 = 1248533399827161865;
        }
        -1 | 0 => {
            current_block_70 = 1248533399827161865;
        }
        1 => {
            who(*argv.offset(optind as isize), 0 as libc::c_int);
            current_block_70 = 1134115459065347084;
        }
        _ => {
            if 0 != 0 {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    gettext(b"extra operand %s\0" as *const u8 as *const libc::c_char),
                    quote(*argv.offset((optind + 2 as libc::c_int) as isize)),
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
                        quote(*argv.offset((optind + 2 as libc::c_int) as isize)),
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
                        quote(*argv.offset((optind + 2 as libc::c_int) as isize)),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
            usage(1 as libc::c_int);
            current_block_70 = 1134115459065347084;
        }
    }
    match current_block_70 {
        1248533399827161865 => {
            who(
                b"/var/run/utmp\0" as *const u8 as *const libc::c_char,
                READ_UTMP_CHECK_PIDS as libc::c_int,
            );
        }
        _ => {}
    }
    return 0 as libc::c_int;
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
unsafe extern "C" fn run_static_initializers() {
    now = !if (0 as libc::c_int as time_t) < -(1 as libc::c_int) as time_t {
        -(1 as libc::c_int) as time_t
    } else {
        (((1 as libc::c_int as time_t)
            << (::core::mem::size_of::<time_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
            - 1 as libc::c_int as libc::c_long) * 2 as libc::c_int as libc::c_long
            + 1 as libc::c_int as libc::c_long
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
