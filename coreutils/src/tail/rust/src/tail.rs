#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut, unused_imports)]
#![feature(extern_types, label_break_value)]
use ::rust::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type hash_table;
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
    fn fwrite_unlocked(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn clearerr_unlocked(__stream: *mut FILE);
    fn fpurge(gl_stream: *mut FILE) -> libc::c_int;
    fn fflush_unlocked(__stream: *mut FILE) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn raise(__sig: libc::c_int) -> libc::c_int;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn getpagesize() -> libc::c_int;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn lstat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn rawmemchr(__s: *const libc::c_void, __c: libc::c_int) -> *mut libc::c_void;
    fn memrchr(
        __s: *const libc::c_void,
        __c: libc::c_int,
        __n: size_t,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    static mut Version: *const libc::c_char;
    fn rpl_fcntl(fd: libc::c_int, action: libc::c_int, _: ...) -> libc::c_int;
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
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn xpalloc(
        pa: *mut libc::c_void,
        pn: *mut idx_t,
        n_incr_min: idx_t,
        n_max: ptrdiff_t,
        s: idx_t,
    ) -> *mut libc::c_void;
    fn last_component(filename: *const libc::c_char) -> *mut libc::c_char;
    fn dir_len(file: *const libc::c_char) -> size_t;
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
    fn offtostr(_: off_t, _: *mut libc::c_char) -> *mut libc::c_char;
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
    fn xnmalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    static mut argmatch_die: argmatch_exit_fn;
    fn __xargmatch_internal(
        context: *const libc::c_char,
        arg: *const libc::c_char,
        arglist: *const *const libc::c_char,
        vallist: *const libc::c_void,
        valsize: size_t,
        exit_fn: argmatch_exit_fn,
        allow_abbreviation: bool,
    ) -> ptrdiff_t;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn cl_strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn open_safer(_: *const libc::c_char, _: libc::c_int, _: ...) -> libc::c_int;
    fn iopoll(fdin: libc::c_int, fdout: libc::c_int, block: bool) -> libc::c_int;
    fn isapipe(fd: libc::c_int) -> libc::c_int;
    fn posix2_version() -> libc::c_int;
    fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> size_t;
    fn xdectoumax(
        n_str: *const libc::c_char,
        min: uintmax_t,
        max: uintmax_t,
        suffixes: *const libc::c_char,
        err: *const libc::c_char,
        err_exit: libc::c_int,
    ) -> uintmax_t;
    fn xnanosleep(_: libc::c_double) -> libc::c_int;
    fn xstrtoumax(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
        _: *mut uintmax_t,
        _: *const libc::c_char,
    ) -> strtol_error;
    fn xstrtod(
        str: *const libc::c_char,
        ptr: *mut *const libc::c_char,
        result: *mut libc::c_double,
        convert: Option::<
            unsafe extern "C" fn(
                *const libc::c_char,
                *mut *mut libc::c_char,
            ) -> libc::c_double,
        >,
    ) -> bool;
    fn hash_get_n_entries(table: *const Hash_table) -> size_t;
    fn hash_lookup(
        table: *const Hash_table,
        entry: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn hash_free(table: *mut Hash_table);
    fn hash_initialize(
        candidate: size_t,
        tuning: *const Hash_tuning,
        hasher: Hash_hasher,
        comparator: Hash_comparator,
        data_freer: Hash_data_freer,
    ) -> *mut Hash_table;
    fn hash_insert(
        table: *mut Hash_table,
        entry: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn hash_remove(
        table: *mut Hash_table,
        entry: *const libc::c_void,
    ) -> *mut libc::c_void;
    fn poll(__fds: *mut pollfd, __nfds: nfds_t, __timeout: libc::c_int) -> libc::c_int;
    fn inotify_init() -> libc::c_int;
    fn inotify_add_watch(
        __fd: libc::c_int,
        __name: *const libc::c_char,
        __mask: uint32_t,
    ) -> libc::c_int;
    fn inotify_rm_watch(__fd: libc::c_int, __wd: libc::c_int) -> libc::c_int;
    fn fstatfs(__fildes: libc::c_int, __buf: *mut statfs) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __fsid_t {
    pub __val: [libc::c_int; 2],
}
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_int;
pub type __blkcnt_t = libc::c_long;
pub type __fsblkcnt_t = libc::c_ulong;
pub type __fsfilcnt_t = libc::c_ulong;
pub type __fsword_t = libc::c_long;
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
pub type off_t = __off_t;
pub type ptrdiff_t = libc::c_long;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type mode_t = __mode_t;
pub type pid_t = __pid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type blksize_t = __blksize_t;
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
pub type uint32_t = __uint32_t;
pub type uintmax_t = __uintmax_t;
pub type idx_t = ptrdiff_t;
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
pub type strtol_error = libc::c_uint;
pub const LONGINT_INVALID: strtol_error = 4;
pub const LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW: strtol_error = 3;
pub const LONGINT_INVALID_SUFFIX_CHAR: strtol_error = 2;
pub const LONGINT_OVERFLOW: strtol_error = 1;
pub const LONGINT_OK: strtol_error = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_tuning {
    pub shrink_threshold: libc::c_float,
    pub shrink_factor: libc::c_float,
    pub growth_threshold: libc::c_float,
    pub growth_factor: libc::c_float,
    pub is_n_buckets: bool,
}
pub type Hash_tuning = hash_tuning;
pub type Hash_table = hash_table;
pub type Hash_hasher = Option::<
    unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
>;
pub type Hash_comparator = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> bool,
>;
pub type Hash_data_freer = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type nfds_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct inotify_event {
    pub wd: libc::c_int,
    pub mask: uint32_t,
    pub cookie: uint32_t,
    pub len: uint32_t,
    pub name: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct statfs {
    pub f_type: __fsword_t,
    pub f_bsize: __fsword_t,
    pub f_blocks: __fsblkcnt_t,
    pub f_bfree: __fsblkcnt_t,
    pub f_bavail: __fsblkcnt_t,
    pub f_files: __fsfilcnt_t,
    pub f_ffree: __fsfilcnt_t,
    pub f_fsid: __fsid_t,
    pub f_namelen: __fsword_t,
    pub f_frsize: __fsword_t,
    pub f_flags: __fsword_t,
    pub f_spare: [__fsword_t; 4],
}
pub type Follow_mode = libc::c_uint;
pub const Follow_descriptor: Follow_mode = 2;
pub const Follow_name: Follow_mode = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct File_spec {
    pub name: *mut libc::c_char,
    pub size: off_t,
    pub mtime: timespec,
    pub dev: dev_t,
    pub ino: ino_t,
    pub mode: mode_t,
    pub ignore: bool,
    pub remote: bool,
    pub tailable: bool,
    pub fd: libc::c_int,
    pub errnum: libc::c_int,
    pub blocking: libc::c_int,
    pub wd: libc::c_int,
    pub parent_wd: libc::c_int,
    pub basename_start: size_t,
    pub n_unchanged_stats: uintmax_t,
}
pub type header_mode = libc::c_uint;
pub const never: header_mode = 2;
pub const always: header_mode = 1;
pub const multiple_files: header_mode = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const DISABLE_INOTIFY_OPTION: C2RustUnnamed_0 = 261;
pub const LONG_FOLLOW_OPTION: C2RustUnnamed_0 = 260;
pub const PRESUME_INPUT_PIPE_OPTION: C2RustUnnamed_0 = 259;
pub const PID_OPTION: C2RustUnnamed_0 = 258;
pub const MAX_UNCHANGED_STATS_OPTION: C2RustUnnamed_0 = 257;
pub const RETRY_OPTION: C2RustUnnamed_0 = 256;
pub type LBUFFER = linebuffer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linebuffer {
    pub buffer: [libc::c_char; 8192],
    pub nbytes: size_t,
    pub nlines: size_t,
    pub next: *mut linebuffer,
}
pub type CBUFFER = charbuffer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct charbuffer {
    pub buffer: [libc::c_char; 8192],
    pub nbytes: size_t,
    pub next: *mut charbuffer,
}
#[inline]
unsafe extern "C" fn timespec_cmp(mut a: timespec, mut b: timespec) -> libc::c_int {
    return 2 as libc::c_int
        * ((a.tv_sec > b.tv_sec) as libc::c_int - (a.tv_sec < b.tv_sec) as libc::c_int)
        + ((a.tv_nsec > b.tv_nsec) as libc::c_int
            - (a.tv_nsec < b.tv_nsec) as libc::c_int);
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
unsafe extern "C" fn usable_st_size(mut sb: *const stat) -> bool {
    return (*sb).st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint
        || (*sb).st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o120000 as libc::c_int as libc::c_uint
        || ((*sb).st_mode).wrapping_sub((*sb).st_mode) != 0 || 0 as libc::c_int != 0;
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
unsafe extern "C" fn get_stat_mtime(mut st: *const stat) -> timespec {
    return (*st).st_mtim;
}
#[inline]
unsafe extern "C" fn xset_binary_mode_error() {}
#[inline]
unsafe extern "C" fn xset_binary_mode(mut fd: libc::c_int, mut mode: libc::c_int) {
    if set_binary_mode(fd, mode) < 0 as libc::c_int {
        xset_binary_mode_error();
    }
}
#[inline]
unsafe extern "C" fn __gl_setmode(
    mut _fd: libc::c_int,
    mut _mode: libc::c_int,
) -> libc::c_int {
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn set_binary_mode(
    mut fd: libc::c_int,
    mut mode: libc::c_int,
) -> libc::c_int {
    return __gl_setmode(fd, mode);
}
#[inline]
unsafe extern "C" fn is_local_fs_type(mut magic: libc::c_ulong) -> libc::c_int {
    match magic {
        1513908720 => return 1 as libc::c_int,
        1633904243 => return 0 as libc::c_int,
        44533 => return 1 as libc::c_int,
        44543 => return 1 as libc::c_int,
        1397113167 => return 0 as libc::c_int,
        151263540 => return 1 as libc::c_int,
        1635083891 => return 0 as libc::c_int,
        391 => return 1 as libc::c_int,
        325456742 => return 1 as libc::c_int,
        3393526350 => return 1 as libc::c_int,
        1111905073 => return 1 as libc::c_int,
        1650746742 => return 1 as libc::c_int,
        464386766 => return 1 as libc::c_int,
        1819242352 => return 1 as libc::c_int,
        3405662737 => return 1 as libc::c_int,
        1112100429 => return 1 as libc::c_int,
        2435016766 => return 1 as libc::c_int,
        1936880249 => return 1 as libc::c_int,
        12805120 => return 0 as libc::c_int,
        2613483 => return 1 as libc::c_int,
        1667723888 => return 1 as libc::c_int,
        4283649346 => return 0 as libc::c_int,
        1937076805 => return 0 as libc::c_int,
        19920823 => return 1 as libc::c_int,
        1650812272 => return 1 as libc::c_int,
        684539205 => return 1 as libc::c_int,
        1161678120 => return 1 as libc::c_int,
        1684300152 => return 1 as libc::c_int,
        1684170528 => return 1 as libc::c_int,
        4979 => return 1 as libc::c_int,
        1162691661 => return 1 as libc::c_int,
        7377 => return 1 as libc::c_int,
        1145913666 => return 1 as libc::c_int,
        61791 => return 1 as libc::c_int,
        3730735588 => return 1 as libc::c_int,
        4278867 => return 1 as libc::c_int,
        3774210530 => return 1 as libc::c_int,
        538032816 => return 1 as libc::c_int,
        1163413075 => return 1 as libc::c_int,
        24053 => return 1 as libc::c_int,
        4989 => return 1 as libc::c_int,
        61267 => return 1 as libc::c_int,
        61265 => return 1 as libc::c_int,
        4076150800 => return 1 as libc::c_int,
        16390 => return 1 as libc::c_int,
        428016422 => return 0 as libc::c_int,
        1702057286 => return 0 as libc::c_int,
        1702057283 => return 0 as libc::c_int,
        195894762 => return 1 as libc::c_int,
        18225520 => return 0 as libc::c_int,
        1196443219 => return 0 as libc::c_int,
        16964 => return 1 as libc::c_int,
        18475 => return 1 as libc::c_int,
        18520 => return 1 as libc::c_int,
        12648430 => return 1 as libc::c_int,
        4187351113 => return 1 as libc::c_int,
        2508478710 => return 1 as libc::c_int,
        288389204 => return 1 as libc::c_int,
        19993000 => return 0 as libc::c_int,
        732765674 => return 1 as libc::c_int,
        38496 => return 1 as libc::c_int,
        16388 => return 1 as libc::c_int,
        16384 => return 1 as libc::c_int,
        1984 => return 1 as libc::c_int,
        29366 => return 1 as libc::c_int,
        827541066 => return 1 as libc::c_int,
        1799439955 => return 0 as libc::c_int,
        3380511080 => return 1 as libc::c_int,
        198183888 => return 0 as libc::c_int,
        1397109069 => return 1 as libc::c_int,
        4991 => return 1 as libc::c_int,
        5007 => return 1 as libc::c_int,
        9320 => return 1 as libc::c_int,
        9336 => return 1 as libc::c_int,
        19802 => return 1 as libc::c_int,
        427819522 => return 1 as libc::c_int,
        19780 => return 1 as libc::c_int,
        22092 => return 0 as libc::c_int,
        26985 => return 0 as libc::c_int,
        1852207972 => return 0 as libc::c_int,
        13364 => return 1 as libc::c_int,
        1853056627 => return 1 as libc::c_int,
        1397118030 => return 1 as libc::c_int,
        40865 => return 1 as libc::c_int,
        1952539503 => return 0 as libc::c_int,
        2035054128 => return 0 as libc::c_int,
        2866260714 => return 0 as libc::c_int,
        1346978886 => return 1 as libc::c_int,
        1346981957 => return 0 as libc::c_int,
        3344373136 => return 1 as libc::c_int,
        2088527475 => return 0 as libc::c_int,
        40864 => return 1 as libc::c_int,
        1634035564 => return 1 as libc::c_int,
        47 => return 1 as libc::c_int,
        1746473250 => return 1 as libc::c_int,
        2240043254 => return 1 as libc::c_int,
        124082209 => return 1 as libc::c_int,
        1382369651 => return 1 as libc::c_int,
        29301 => return 1 as libc::c_int,
        1733912937 => return 1 as libc::c_int,
        1573531125 => return 1 as libc::c_int,
        1397048141 => return 1 as libc::c_int,
        1935894131 => return 1 as libc::c_int,
        4185718668 => return 1 as libc::c_int,
        1128357203 => return 1 as libc::c_int,
        20859 => return 0 as libc::c_int,
        4266872130 => return 0 as libc::c_int,
        3203391149 => return 0 as libc::c_int,
        1397703499 => return 1 as libc::c_int,
        1936814952 => return 1 as libc::c_int,
        1650812274 => return 1 as libc::c_int,
        19920822 => return 1 as libc::c_int,
        19920821 => return 1 as libc::c_int,
        16914836 => return 1 as libc::c_int,
        1953653091 => return 1 as libc::c_int,
        604313861 => return 1 as libc::c_int,
        352400198 => return 1 as libc::c_int,
        72020 => return 1 as libc::c_int,
        1410924800 => return 1 as libc::c_int,
        40866 => return 1 as libc::c_int,
        16914839 => return 1 as libc::c_int,
        2020557398 => return 0 as libc::c_int,
        3133910204 => return 0 as libc::c_int,
        2768370933 => return 0 as libc::c_int,
        1448756819 => return 1 as libc::c_int,
        1397114950 => return 1 as libc::c_int,
        2881100148 => return 1 as libc::c_int,
        19920820 => return 1 as libc::c_int,
        1481003842 => return 1 as libc::c_int,
        19911021 => return 1 as libc::c_int,
        51 => return 1 as libc::c_int,
        801189825 => return 1 as libc::c_int,
        1515144787 => return 1 as libc::c_int,
        1479104553 => return 1 as libc::c_int,
        _ => return -(1 as libc::c_int),
    };
}
static mut follow_mode_string: [*const libc::c_char; 3] = [
    b"descriptor\0" as *const u8 as *const libc::c_char,
    b"name\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut follow_mode_map: [Follow_mode; 2] = [Follow_descriptor, Follow_name];
static mut reopen_inaccessible_files: bool = false;
static mut count_lines: bool = false;
static mut follow_mode: Follow_mode = Follow_descriptor;
static mut forever: bool = false;
static mut monitor_output: bool = false;
static mut from_start: bool = false;
static mut print_headers: bool = false;
static mut line_end: libc::c_char = 0;
static mut max_n_unchanged_stats_between_opens: uintmax_t = 5 as libc::c_int
    as uintmax_t;
static mut nbpids: libc::c_int = 0 as libc::c_int;
static mut pids: *mut pid_t = 0 as *const pid_t as *mut pid_t;
static mut pids_alloc: idx_t = 0;
static mut page_size: idx_t = 0;
static mut have_read_stdin: bool = false;
static mut presume_input_pipe: bool = false;
static mut disable_inotify: bool = false;
static mut long_options: [option; 16] = [
    {
        let mut init = option {
            name: b"bytes\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'c' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"follow\0" as *const u8 as *const libc::c_char,
            has_arg: 2 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: LONG_FOLLOW_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"lines\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'n' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"max-unchanged-stats\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: MAX_UNCHANGED_STATS_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"-disable-inotify\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: DISABLE_INOTIFY_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"pid\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: PID_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"-presume-input-pipe\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: PRESUME_INPUT_PIPE_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"quiet\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'q' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"retry\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: RETRY_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"silent\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'q' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"sleep-interval\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 's' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"verbose\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'v' as i32,
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
        printf(
            gettext(
                b"Print the last %d lines of each FILE to standard output.\nWith more than one FILE, precede each with a header giving the file name.\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            10 as libc::c_int,
        );
        emit_stdin_note();
        emit_mandatory_arg_note();
        fputs_unlocked(
            gettext(
                b"  -c, --bytes=[+]NUM       output the last NUM bytes; or use -c +NUM to\n                             output starting with byte NUM of each file\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"  -f, --follow[={name|descriptor}]\n                           output appended data as the file grows;\n                             an absent option argument means 'descriptor'\n  -F                       same as --follow=name --retry\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        printf(
            gettext(
                b"  -n, --lines=[+]NUM       output the last NUM lines, instead of the last %d;\n                             or use -n +NUM to skip NUM-1 lines at the start\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            10 as libc::c_int,
        );
        printf(
            gettext(
                b"      --max-unchanged-stats=N\n                           with --follow=name, reopen a FILE which has not\n                             changed size after N (default %d) iterations\n                             to see if it has been unlinked or renamed\n                             (this is the usual case of rotated log files);\n                             with inotify, this option is rarely useful\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            5 as libc::c_int,
        );
        fputs_unlocked(
            gettext(
                b"      --pid=PID            with -f, terminate after process ID, PID dies;\n                             can be repeated to watch multiple processes\n  -q, --quiet, --silent    never output headers giving file names\n      --retry              keep trying to open a file if it is inaccessible\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"  -s, --sleep-interval=N   with -f, sleep for approximately N seconds\n                             (default 1.0) between iterations;\n                             with inotify and --pid=P, check process P at\n                             least once every N seconds\n  -v, --verbose            always output headers giving file names\n\0"
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
        fputs_unlocked(
            gettext(
                b"\nNUM may have a multiplier suffix:\nb 512, kB 1000, K 1024, MB 1000*1000, M 1024*1024,\nGB 1000*1000*1000, G 1024*1024*1024, and so on for T, P, E, Z, Y, R, Q.\nBinary prefixes can be used, too: KiB=K, MiB=M, and so on.\n\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"With --follow (-f), tail defaults to following the file descriptor, which\nmeans that even if a tail'ed file is renamed, tail will continue to track\nits end.  This default behavior is not desirable when you really want to\ntrack the actual name of the file, not the file descriptor (e.g., log\nrotation).  Use --follow=name in that case.  That causes tail to track the\nnamed file in a way that accommodates renaming, removal and creation.\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        emit_ancillary_info(b"tail\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
}
unsafe extern "C" fn die_pipe() {
    raise(13 as libc::c_int);
    exit(1 as libc::c_int);
}
unsafe extern "C" fn check_output_alive() {
    if !monitor_output {
        return;
    }
    if iopoll(-(1 as libc::c_int), 1 as libc::c_int, 0 as libc::c_int != 0)
        == -(2 as libc::c_int)
    {
        die_pipe();
    }
}
unsafe extern "C" fn valid_file_spec(mut f: *const File_spec) -> bool {
    return ((*f).fd == -(1 as libc::c_int)) as libc::c_int
        ^ ((*f).errnum == 0 as libc::c_int) as libc::c_int != 0;
}
unsafe extern "C" fn pretty_name(mut f: *const File_spec) -> *const libc::c_char {
    return if strcmp((*f).name, b"-\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        gettext(b"standard input\0" as *const u8 as *const libc::c_char)
    } else {
        (*f).name
    };
}
unsafe extern "C" fn record_open_fd(
    mut f: *mut File_spec,
    mut fd: libc::c_int,
    mut size: off_t,
    mut st: *const stat,
    mut blocking: libc::c_int,
) {
    (*f).fd = fd;
    (*f).size = size;
    (*f).mtime = get_stat_mtime(st);
    (*f).dev = (*st).st_dev;
    (*f).ino = (*st).st_ino;
    (*f).mode = (*st).st_mode;
    (*f).blocking = blocking;
    (*f).n_unchanged_stats = 0 as libc::c_int as uintmax_t;
    (*f).ignore = 0 as libc::c_int != 0;
}
unsafe extern "C" fn close_fd(mut fd: libc::c_int, mut filename: *const libc::c_char) {
    if fd != -(1 as libc::c_int) && fd != 0 as libc::c_int && close(fd) != 0 {
        if 0 != 0 {
            error(
                0 as libc::c_int,
                *__errno_location(),
                gettext(b"closing %s (fd=%d)\0" as *const u8 as *const libc::c_char),
                quotearg_style(shell_escape_always_quoting_style, filename),
                fd,
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
                    gettext(b"closing %s (fd=%d)\0" as *const u8 as *const libc::c_char),
                    quotearg_style(shell_escape_always_quoting_style, filename),
                    fd,
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
                    gettext(b"closing %s (fd=%d)\0" as *const u8 as *const libc::c_char),
                    quotearg_style(shell_escape_always_quoting_style, filename),
                    fd,
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
    }
}
unsafe extern "C" fn write_header(mut pretty_filename: *const libc::c_char) {
    static mut first_file: bool = 1 as libc::c_int != 0;
    printf(
        b"%s==> %s <==\n\0" as *const u8 as *const libc::c_char,
        if first_file as libc::c_int != 0 {
            b"\0" as *const u8 as *const libc::c_char
        } else {
            b"\n\0" as *const u8 as *const libc::c_char
        },
        pretty_filename,
    );
    first_file = 0 as libc::c_int != 0;
}
unsafe extern "C" fn xwrite_stdout(
    mut buffer: *const libc::c_char,
    mut n_bytes: size_t,
) {
    if n_bytes > 0 as libc::c_int as libc::c_ulong
        && fwrite_unlocked(
            buffer as *const libc::c_void,
            1 as libc::c_int as size_t,
            n_bytes,
            stdout,
        ) < n_bytes
    {
        clearerr_unlocked(stdout);
        if 0 != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                gettext(b"error writing %s\0" as *const u8 as *const libc::c_char),
                quotearg_style(
                    shell_escape_always_quoting_style,
                    b"standard output\0" as *const u8 as *const libc::c_char,
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
                    gettext(b"error writing %s\0" as *const u8 as *const libc::c_char),
                    quotearg_style(
                        shell_escape_always_quoting_style,
                        b"standard output\0" as *const u8 as *const libc::c_char,
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
                    gettext(b"error writing %s\0" as *const u8 as *const libc::c_char),
                    quotearg_style(
                        shell_escape_always_quoting_style,
                        b"standard output\0" as *const u8 as *const libc::c_char,
                    ),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
    }
}
unsafe extern "C" fn dump_remainder(
    mut want_header: bool,
    mut pretty_filename: *const libc::c_char,
    mut fd: libc::c_int,
    mut n_bytes: uintmax_t,
) -> uintmax_t {
    let mut n_written: uintmax_t = 0;
    let mut n_remaining: uintmax_t = n_bytes;
    n_written = 0 as libc::c_int as uintmax_t;
    loop {
        let mut buffer: [libc::c_char; 8192] = [0; 8192];
        let mut n: size_t = if n_remaining < 8192 as libc::c_int as libc::c_ulong {
            n_remaining
        } else {
            8192 as libc::c_int as libc::c_ulong
        };
        let mut bytes_read: size_t = safe_read(
            fd,
            buffer.as_mut_ptr() as *mut libc::c_void,
            n,
        );
        if bytes_read == -(1 as libc::c_int) as size_t {
            if *__errno_location() != 11 as libc::c_int {
                if 0 != 0 {
                    error(
                        1 as libc::c_int,
                        *__errno_location(),
                        gettext(
                            b"error reading %s\0" as *const u8 as *const libc::c_char,
                        ),
                        quotearg_style(
                            shell_escape_always_quoting_style,
                            pretty_filename,
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
                                b"error reading %s\0" as *const u8 as *const libc::c_char,
                            ),
                            quotearg_style(
                                shell_escape_always_quoting_style,
                                pretty_filename,
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
                                b"error reading %s\0" as *const u8 as *const libc::c_char,
                            ),
                            quotearg_style(
                                shell_escape_always_quoting_style,
                                pretty_filename,
                            ),
                        );
                        if __errstatus != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                        
                    });
                };
            }
            break;
        } else {
            if bytes_read == 0 as libc::c_int as libc::c_ulong {
                break;
            }
            if want_header {
                write_header(pretty_filename);
                want_header = 0 as libc::c_int != 0;
            }
            xwrite_stdout(buffer.as_mut_ptr(), bytes_read);
            n_written = (n_written as libc::c_ulong).wrapping_add(bytes_read)
                as uintmax_t as uintmax_t;
            if !(n_bytes != 18446744073709551615 as libc::c_ulong) {
                continue;
            }
            n_remaining = (n_remaining as libc::c_ulong).wrapping_sub(bytes_read)
                as uintmax_t as uintmax_t;
            if n_remaining == 0 as libc::c_int as libc::c_ulong
                || n_bytes
                    == (18446744073709551615 as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            {
                break;
            }
        }
    }
    return n_written;
}
unsafe extern "C" fn xlseek(
    mut fd: libc::c_int,
    mut offset: off_t,
    mut whence: libc::c_int,
    mut filename: *const libc::c_char,
) -> off_t {
    let mut new_offset: off_t = lseek(fd, offset, whence);
    let mut buf: [libc::c_char; 21] = [0; 21];
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    if 0 as libc::c_int as libc::c_long <= new_offset {
        return new_offset;
    }
    s = offtostr(offset, buf.as_mut_ptr());
    match whence {
        0 => {
            if 0 != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    gettext(
                        b"%s: cannot seek to offset %s\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        filename,
                    ),
                    s,
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
                            b"%s: cannot seek to offset %s\0" as *const u8
                                as *const libc::c_char,
                        ),
                        quotearg_n_style_colon(
                            0 as libc::c_int,
                            shell_escape_quoting_style,
                            filename,
                        ),
                        s,
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
                            b"%s: cannot seek to offset %s\0" as *const u8
                                as *const libc::c_char,
                        ),
                        quotearg_n_style_colon(
                            0 as libc::c_int,
                            shell_escape_quoting_style,
                            filename,
                        ),
                        s,
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
        1 => {
            if 0 != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    gettext(
                        b"%s: cannot seek to relative offset %s\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        filename,
                    ),
                    s,
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
                            b"%s: cannot seek to relative offset %s\0" as *const u8
                                as *const libc::c_char,
                        ),
                        quotearg_n_style_colon(
                            0 as libc::c_int,
                            shell_escape_quoting_style,
                            filename,
                        ),
                        s,
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
                            b"%s: cannot seek to relative offset %s\0" as *const u8
                                as *const libc::c_char,
                        ),
                        quotearg_n_style_colon(
                            0 as libc::c_int,
                            shell_escape_quoting_style,
                            filename,
                        ),
                        s,
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
        2 => {
            if 0 != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    gettext(
                        b"%s: cannot seek to end-relative offset %s\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        filename,
                    ),
                    s,
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
                            b"%s: cannot seek to end-relative offset %s\0" as *const u8
                                as *const libc::c_char,
                        ),
                        quotearg_n_style_colon(
                            0 as libc::c_int,
                            shell_escape_quoting_style,
                            filename,
                        ),
                        s,
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
                            b"%s: cannot seek to end-relative offset %s\0" as *const u8
                                as *const libc::c_char,
                        ),
                        quotearg_n_style_colon(
                            0 as libc::c_int,
                            shell_escape_quoting_style,
                            filename,
                        ),
                        s,
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
        _ => {
            unreachable!();
        }
    }
    panic!("Reached end of non-void function without returning");
}
unsafe extern "C" fn file_lines(
    mut pretty_filename: *const libc::c_char,
    mut fd: libc::c_int,
    mut sb: *const stat,
    mut n_lines: uintmax_t,
    mut start_pos: off_t,
    mut end_pos: off_t,
    mut read_pos: *mut uintmax_t,
) -> bool {
    let mut buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bytes_read: size_t = 0;
    let mut bufsize: blksize_t = 8192 as libc::c_int;
    let mut pos: off_t = end_pos;
    let mut ok: bool = 1 as libc::c_int != 0;
    if n_lines == 0 as libc::c_int as libc::c_ulong {
        return 1 as libc::c_int != 0;
    }
    if (*sb).st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"((((sb->st_mode)) & 0170000) == (0100000))\0" as *const u8
                as *const libc::c_char,
            b"tail.c\0" as *const u8 as *const libc::c_char,
            543 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 95],
                &[libc::c_char; 95],
            >(
                b"_Bool file_lines(const char *, int, const struct stat *, uintmax_t, off_t, off_t, uintmax_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_10359: {
        if (*sb).st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o100000 as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"((((sb->st_mode)) & 0170000) == (0100000))\0" as *const u8
                    as *const libc::c_char,
                b"tail.c\0" as *const u8 as *const libc::c_char,
                543 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 95],
                    &[libc::c_char; 95],
                >(
                    b"_Bool file_lines(const char *, int, const struct stat *, uintmax_t, off_t, off_t, uintmax_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if (*sb).st_size % page_size == 0 as libc::c_int as libc::c_long {
        bufsize = (if 8192 as libc::c_int as libc::c_long > page_size {
            8192 as libc::c_int as libc::c_long
        } else {
            page_size
        }) as blksize_t;
    }
    buffer = xmalloc(bufsize as size_t) as *mut libc::c_char;
    bytes_read = ((pos - start_pos) % bufsize as libc::c_long) as size_t;
    if bytes_read == 0 as libc::c_int as libc::c_ulong {
        bytes_read = bufsize as size_t;
    }
    pos = (pos as libc::c_ulong).wrapping_sub(bytes_read) as off_t as off_t;
    xlseek(fd, pos, 0 as libc::c_int, pretty_filename);
    bytes_read = safe_read(fd, buffer as *mut libc::c_void, bytes_read);
    if bytes_read == -(1 as libc::c_int) as size_t {
        if 0 != 0 {
            error(
                0 as libc::c_int,
                *__errno_location(),
                gettext(b"error reading %s\0" as *const u8 as *const libc::c_char),
                quotearg_style(shell_escape_always_quoting_style, pretty_filename),
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
                    gettext(b"error reading %s\0" as *const u8 as *const libc::c_char),
                    quotearg_style(shell_escape_always_quoting_style, pretty_filename),
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
                    gettext(b"error reading %s\0" as *const u8 as *const libc::c_char),
                    quotearg_style(shell_escape_always_quoting_style, pretty_filename),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
        ok = 0 as libc::c_int != 0;
    } else {
        *read_pos = (pos as libc::c_ulong).wrapping_add(bytes_read);
        if bytes_read != 0
            && *buffer
                .offset(
                    bytes_read.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                ) as libc::c_int != line_end as libc::c_int
        {
            n_lines = n_lines.wrapping_sub(1);
            n_lines;
        }
        's_79: loop {
            let mut n: size_t = bytes_read;
            while n != 0 {
                let mut nl: *const libc::c_char = 0 as *const libc::c_char;
                nl = memrchr(buffer as *const libc::c_void, line_end as libc::c_int, n)
                    as *const libc::c_char;
                if nl.is_null() {
                    break;
                }
                n = nl.offset_from(buffer) as libc::c_long as size_t;
                let fresh0 = n_lines;
                n_lines = n_lines.wrapping_sub(1);
                if !(fresh0 == 0 as libc::c_int as libc::c_ulong) {
                    continue;
                }
                xwrite_stdout(
                    nl.offset(1 as libc::c_int as isize),
                    bytes_read
                        .wrapping_sub(n.wrapping_add(1 as libc::c_int as libc::c_ulong)),
                );
                *read_pos = (*read_pos as libc::c_ulong)
                    .wrapping_add(
                        dump_remainder(
                            0 as libc::c_int != 0,
                            pretty_filename,
                            fd,
                            (end_pos as libc::c_ulong)
                                .wrapping_sub(
                                    (pos as libc::c_ulong).wrapping_add(bytes_read),
                                ),
                        ),
                    ) as uintmax_t as uintmax_t;
                break 's_79;
            }
            if pos == start_pos {
                xlseek(fd, start_pos, 0 as libc::c_int, pretty_filename);
                *read_pos = (start_pos as libc::c_ulong)
                    .wrapping_add(
                        dump_remainder(
                            0 as libc::c_int != 0,
                            pretty_filename,
                            fd,
                            end_pos as uintmax_t,
                        ),
                    );
                break;
            } else {
                pos -= bufsize as libc::c_long;
                xlseek(fd, pos, 0 as libc::c_int, pretty_filename);
                bytes_read = safe_read(
                    fd,
                    buffer as *mut libc::c_void,
                    bufsize as size_t,
                );
                if bytes_read == -(1 as libc::c_int) as size_t {
                    if 0 != 0 {
                        error(
                            0 as libc::c_int,
                            *__errno_location(),
                            gettext(
                                b"error reading %s\0" as *const u8 as *const libc::c_char,
                            ),
                            quotearg_style(
                                shell_escape_always_quoting_style,
                                pretty_filename,
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
                                gettext(
                                    b"error reading %s\0" as *const u8 as *const libc::c_char,
                                ),
                                quotearg_style(
                                    shell_escape_always_quoting_style,
                                    pretty_filename,
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
                                gettext(
                                    b"error reading %s\0" as *const u8 as *const libc::c_char,
                                ),
                                quotearg_style(
                                    shell_escape_always_quoting_style,
                                    pretty_filename,
                                ),
                            );
                            if __errstatus != 0 as libc::c_int {
                                unreachable!();
                            } else {};
                            
                        });
                    };
                    ok = 0 as libc::c_int != 0;
                    break;
                } else {
                    *read_pos = (pos as libc::c_ulong).wrapping_add(bytes_read);
                    if !(bytes_read > 0 as libc::c_int as libc::c_ulong) {
                        break;
                    }
                }
            }
        }
    }
    free(buffer as *mut libc::c_void);
    return ok;
}
unsafe extern "C" fn pipe_lines(
    mut pretty_filename: *const libc::c_char,
    mut fd: libc::c_int,
    mut n_lines: uintmax_t,
    mut read_pos: *mut uintmax_t,
) -> bool {
    let mut first: *mut LBUFFER = 0 as *mut LBUFFER;
    let mut last: *mut LBUFFER = 0 as *mut LBUFFER;
    let mut tmp: *mut LBUFFER = 0 as *mut LBUFFER;
    let mut total_lines: size_t = 0 as libc::c_int as size_t;
    let mut ok: bool = 1 as libc::c_int != 0;
    let mut n_read: size_t = 0;
    last = xmalloc(::core::mem::size_of::<LBUFFER>() as libc::c_ulong) as *mut LBUFFER;
    first = last;
    (*first).nlines = 0 as libc::c_int as size_t;
    (*first).nbytes = (*first).nlines;
    (*first).next = 0 as *mut linebuffer;
    tmp = xmalloc(::core::mem::size_of::<LBUFFER>() as libc::c_ulong) as *mut LBUFFER;
    loop {
        n_read = safe_read(
            fd,
            ((*tmp).buffer).as_mut_ptr() as *mut libc::c_void,
            8192 as libc::c_int as size_t,
        );
        if n_read == 0 as libc::c_int as libc::c_ulong
            || n_read == -(1 as libc::c_int) as size_t
        {
            break;
        }
        (*tmp).nbytes = n_read;
        *read_pos = (*read_pos as libc::c_ulong).wrapping_add(n_read) as uintmax_t
            as uintmax_t;
        (*tmp).nlines = 0 as libc::c_int as size_t;
        (*tmp).next = 0 as *mut linebuffer;
        let mut buffer_end: *const libc::c_char = ((*tmp).buffer)
            .as_mut_ptr()
            .offset(n_read as isize);
        let mut p: *const libc::c_char = ((*tmp).buffer).as_mut_ptr();
        loop {
            p = memchr(
                p as *const libc::c_void,
                line_end as libc::c_int,
                buffer_end.offset_from(p) as libc::c_long as libc::c_ulong,
            ) as *const libc::c_char;
            if p.is_null() {
                break;
            }
            p = p.offset(1);
            p;
            (*tmp).nlines = ((*tmp).nlines).wrapping_add(1);
            (*tmp).nlines;
        }
        total_lines = (total_lines as libc::c_ulong).wrapping_add((*tmp).nlines)
            as size_t as size_t;
        if ((*tmp).nbytes).wrapping_add((*last).nbytes)
            < 8192 as libc::c_int as libc::c_ulong
        {
            memcpy(
                &mut *((*last).buffer).as_mut_ptr().offset((*last).nbytes as isize)
                    as *mut libc::c_char as *mut libc::c_void,
                ((*tmp).buffer).as_mut_ptr() as *const libc::c_void,
                (*tmp).nbytes,
            );
            (*last)
                .nbytes = ((*last).nbytes as libc::c_ulong).wrapping_add((*tmp).nbytes)
                as size_t as size_t;
            (*last)
                .nlines = ((*last).nlines as libc::c_ulong).wrapping_add((*tmp).nlines)
                as size_t as size_t;
        } else {
            (*last).next = tmp;
            last = (*last).next;
            if total_lines.wrapping_sub((*first).nlines) > n_lines {
                tmp = first;
                total_lines = (total_lines as libc::c_ulong)
                    .wrapping_sub((*first).nlines) as size_t as size_t;
                first = (*first).next;
            } else {
                tmp = xmalloc(::core::mem::size_of::<LBUFFER>() as libc::c_ulong)
                    as *mut LBUFFER;
            }
        }
    }
    free(tmp as *mut libc::c_void);
    if n_read == -(1 as libc::c_int) as size_t {
        if 0 != 0 {
            error(
                0 as libc::c_int,
                *__errno_location(),
                gettext(b"error reading %s\0" as *const u8 as *const libc::c_char),
                quotearg_style(shell_escape_always_quoting_style, pretty_filename),
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
                    gettext(b"error reading %s\0" as *const u8 as *const libc::c_char),
                    quotearg_style(shell_escape_always_quoting_style, pretty_filename),
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
                    gettext(b"error reading %s\0" as *const u8 as *const libc::c_char),
                    quotearg_style(shell_escape_always_quoting_style, pretty_filename),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
        ok = 0 as libc::c_int != 0;
    } else if !((*last).nbytes == 0 as libc::c_int as libc::c_ulong) {
        if !(n_lines == 0 as libc::c_int as libc::c_ulong) {
            if (*last)
                .buffer[((*last).nbytes).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                as usize] as libc::c_int != line_end as libc::c_int
            {
                (*last).nlines = ((*last).nlines).wrapping_add(1);
                (*last).nlines;
                total_lines = total_lines.wrapping_add(1);
                total_lines;
            }
            tmp = first;
            while total_lines.wrapping_sub((*tmp).nlines) > n_lines {
                total_lines = (total_lines as libc::c_ulong).wrapping_sub((*tmp).nlines)
                    as size_t as size_t;
                tmp = (*tmp).next;
            }
            let mut beg: *const libc::c_char = ((*tmp).buffer).as_mut_ptr();
            let mut buffer_end_0: *const libc::c_char = ((*tmp).buffer)
                .as_mut_ptr()
                .offset((*tmp).nbytes as isize);
            if total_lines > n_lines {
                let mut j: size_t = 0;
                j = total_lines.wrapping_sub(n_lines);
                while j != 0 {
                    beg = rawmemchr(beg as *const libc::c_void, line_end as libc::c_int)
                        as *const libc::c_char;
                    beg = beg.offset(1);
                    beg;
                    j = j.wrapping_sub(1);
                    j;
                }
            }
            xwrite_stdout(beg, buffer_end_0.offset_from(beg) as libc::c_long as size_t);
            tmp = (*tmp).next;
            while !tmp.is_null() {
                xwrite_stdout(((*tmp).buffer).as_mut_ptr(), (*tmp).nbytes);
                tmp = (*tmp).next;
            }
        }
    }
    while !first.is_null() {
        tmp = (*first).next;
        free(first as *mut libc::c_void);
        first = tmp;
    }
    return ok;
}
unsafe extern "C" fn pipe_bytes(
    mut pretty_filename: *const libc::c_char,
    mut fd: libc::c_int,
    mut n_bytes: uintmax_t,
    mut read_pos: *mut uintmax_t,
) -> bool {
    let mut first: *mut CBUFFER = 0 as *mut CBUFFER;
    let mut last: *mut CBUFFER = 0 as *mut CBUFFER;
    let mut tmp: *mut CBUFFER = 0 as *mut CBUFFER;
    let mut i: size_t = 0;
    let mut total_bytes: size_t = 0 as libc::c_int as size_t;
    let mut ok: bool = 1 as libc::c_int != 0;
    let mut n_read: size_t = 0;
    last = xmalloc(::core::mem::size_of::<CBUFFER>() as libc::c_ulong) as *mut CBUFFER;
    first = last;
    (*first).nbytes = 0 as libc::c_int as size_t;
    (*first).next = 0 as *mut charbuffer;
    tmp = xmalloc(::core::mem::size_of::<CBUFFER>() as libc::c_ulong) as *mut CBUFFER;
    loop {
        n_read = safe_read(
            fd,
            ((*tmp).buffer).as_mut_ptr() as *mut libc::c_void,
            8192 as libc::c_int as size_t,
        );
        if n_read == 0 as libc::c_int as libc::c_ulong
            || n_read == -(1 as libc::c_int) as size_t
        {
            break;
        }
        *read_pos = (*read_pos as libc::c_ulong).wrapping_add(n_read) as uintmax_t
            as uintmax_t;
        (*tmp).nbytes = n_read;
        (*tmp).next = 0 as *mut charbuffer;
        total_bytes = (total_bytes as libc::c_ulong).wrapping_add((*tmp).nbytes)
            as size_t as size_t;
        if ((*tmp).nbytes).wrapping_add((*last).nbytes)
            < 8192 as libc::c_int as libc::c_ulong
        {
            memcpy(
                &mut *((*last).buffer).as_mut_ptr().offset((*last).nbytes as isize)
                    as *mut libc::c_char as *mut libc::c_void,
                ((*tmp).buffer).as_mut_ptr() as *const libc::c_void,
                (*tmp).nbytes,
            );
            (*last)
                .nbytes = ((*last).nbytes as libc::c_ulong).wrapping_add((*tmp).nbytes)
                as size_t as size_t;
        } else {
            (*last).next = tmp;
            last = (*last).next;
            if total_bytes.wrapping_sub((*first).nbytes) > n_bytes {
                tmp = first;
                total_bytes = (total_bytes as libc::c_ulong)
                    .wrapping_sub((*first).nbytes) as size_t as size_t;
                first = (*first).next;
            } else {
                tmp = xmalloc(::core::mem::size_of::<CBUFFER>() as libc::c_ulong)
                    as *mut CBUFFER;
            }
        }
    }
    free(tmp as *mut libc::c_void);
    if n_read == -(1 as libc::c_int) as size_t {
        if 0 != 0 {
            error(
                0 as libc::c_int,
                *__errno_location(),
                gettext(b"error reading %s\0" as *const u8 as *const libc::c_char),
                quotearg_style(shell_escape_always_quoting_style, pretty_filename),
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
                    gettext(b"error reading %s\0" as *const u8 as *const libc::c_char),
                    quotearg_style(shell_escape_always_quoting_style, pretty_filename),
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
                    gettext(b"error reading %s\0" as *const u8 as *const libc::c_char),
                    quotearg_style(shell_escape_always_quoting_style, pretty_filename),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
        ok = 0 as libc::c_int != 0;
    } else {
        tmp = first;
        while total_bytes.wrapping_sub((*tmp).nbytes) > n_bytes {
            total_bytes = (total_bytes as libc::c_ulong).wrapping_sub((*tmp).nbytes)
                as size_t as size_t;
            tmp = (*tmp).next;
        }
        if total_bytes > n_bytes {
            i = total_bytes.wrapping_sub(n_bytes);
        } else {
            i = 0 as libc::c_int as size_t;
        }
        xwrite_stdout(
            &mut *((*tmp).buffer).as_mut_ptr().offset(i as isize),
            ((*tmp).nbytes).wrapping_sub(i),
        );
        tmp = (*tmp).next;
        while !tmp.is_null() {
            xwrite_stdout(((*tmp).buffer).as_mut_ptr(), (*tmp).nbytes);
            tmp = (*tmp).next;
        }
    }
    while !first.is_null() {
        tmp = (*first).next;
        free(first as *mut libc::c_void);
        first = tmp;
    }
    return ok;
}
unsafe extern "C" fn start_bytes(
    mut pretty_filename: *const libc::c_char,
    mut fd: libc::c_int,
    mut n_bytes: uintmax_t,
    mut read_pos: *mut uintmax_t,
) -> libc::c_int {
    let mut buffer: [libc::c_char; 8192] = [0; 8192];
    while (0 as libc::c_int as libc::c_ulong) < n_bytes {
        let mut bytes_read: size_t = safe_read(
            fd,
            buffer.as_mut_ptr() as *mut libc::c_void,
            8192 as libc::c_int as size_t,
        );
        if bytes_read == 0 as libc::c_int as libc::c_ulong {
            return -(1 as libc::c_int);
        }
        if bytes_read == -(1 as libc::c_int) as size_t {
            if 0 != 0 {
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    gettext(b"error reading %s\0" as *const u8 as *const libc::c_char),
                    quotearg_style(shell_escape_always_quoting_style, pretty_filename),
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
                            b"error reading %s\0" as *const u8 as *const libc::c_char,
                        ),
                        quotearg_style(
                            shell_escape_always_quoting_style,
                            pretty_filename,
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
                        gettext(
                            b"error reading %s\0" as *const u8 as *const libc::c_char,
                        ),
                        quotearg_style(
                            shell_escape_always_quoting_style,
                            pretty_filename,
                        ),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
            return 1 as libc::c_int;
        }
        *read_pos = (*read_pos as libc::c_ulong).wrapping_add(bytes_read) as uintmax_t
            as uintmax_t;
        if bytes_read <= n_bytes {
            n_bytes = (n_bytes as libc::c_ulong).wrapping_sub(bytes_read) as uintmax_t
                as uintmax_t;
        } else {
            let mut n_remaining: size_t = bytes_read.wrapping_sub(n_bytes);
            xwrite_stdout(
                &mut *buffer.as_mut_ptr().offset(n_bytes as isize),
                n_remaining,
            );
            break;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn start_lines(
    mut pretty_filename: *const libc::c_char,
    mut fd: libc::c_int,
    mut n_lines: uintmax_t,
    mut read_pos: *mut uintmax_t,
) -> libc::c_int {
    if n_lines == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    loop {
        let mut buffer: [libc::c_char; 8192] = [0; 8192];
        let mut bytes_read: size_t = safe_read(
            fd,
            buffer.as_mut_ptr() as *mut libc::c_void,
            8192 as libc::c_int as size_t,
        );
        if bytes_read == 0 as libc::c_int as libc::c_ulong {
            return -(1 as libc::c_int);
        }
        if bytes_read == -(1 as libc::c_int) as size_t {
            if 0 != 0 {
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    gettext(b"error reading %s\0" as *const u8 as *const libc::c_char),
                    quotearg_style(shell_escape_always_quoting_style, pretty_filename),
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
                            b"error reading %s\0" as *const u8 as *const libc::c_char,
                        ),
                        quotearg_style(
                            shell_escape_always_quoting_style,
                            pretty_filename,
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
                        gettext(
                            b"error reading %s\0" as *const u8 as *const libc::c_char,
                        ),
                        quotearg_style(
                            shell_escape_always_quoting_style,
                            pretty_filename,
                        ),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
            return 1 as libc::c_int;
        }
        let mut buffer_end: *mut libc::c_char = buffer
            .as_mut_ptr()
            .offset(bytes_read as isize);
        *read_pos = (*read_pos as libc::c_ulong).wrapping_add(bytes_read) as uintmax_t
            as uintmax_t;
        let mut p: *mut libc::c_char = buffer.as_mut_ptr();
        loop {
            p = memchr(
                p as *const libc::c_void,
                line_end as libc::c_int,
                buffer_end.offset_from(p) as libc::c_long as libc::c_ulong,
            ) as *mut libc::c_char;
            if p.is_null() {
                break;
            }
            p = p.offset(1);
            p;
            n_lines = n_lines.wrapping_sub(1);
            if n_lines == 0 as libc::c_int as libc::c_ulong {
                if p < buffer_end {
                    xwrite_stdout(
                        p,
                        buffer_end.offset_from(p) as libc::c_long as size_t,
                    );
                }
                return 0 as libc::c_int;
            }
        }
    };
}
unsafe extern "C" fn fremote(
    mut fd: libc::c_int,
    mut name: *const libc::c_char,
) -> bool {
    let mut remote: bool = 1 as libc::c_int != 0;
    let mut buf: statfs = statfs {
        f_type: 0,
        f_bsize: 0,
        f_blocks: 0,
        f_bfree: 0,
        f_bavail: 0,
        f_files: 0,
        f_ffree: 0,
        f_fsid: __fsid_t { __val: [0; 2] },
        f_namelen: 0,
        f_frsize: 0,
        f_flags: 0,
        f_spare: [0; 4],
    };
    let mut err: libc::c_int = fstatfs(fd, &mut buf);
    if err != 0 as libc::c_int {
        if *__errno_location() != 38 as libc::c_int {
            if 0 != 0 {
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    gettext(
                        b"cannot determine location of %s. reverting to polling\0"
                            as *const u8 as *const libc::c_char,
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
                        *__errno_location(),
                        gettext(
                            b"cannot determine location of %s. reverting to polling\0"
                                as *const u8 as *const libc::c_char,
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
                        *__errno_location(),
                        gettext(
                            b"cannot determine location of %s. reverting to polling\0"
                                as *const u8 as *const libc::c_char,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, name),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
    } else {
        remote = is_local_fs_type(buf.f_type as libc::c_ulong) <= 0 as libc::c_int;
    }
    return remote;
}
unsafe extern "C" fn recheck(mut f: *mut File_spec, mut blocking: bool) {
    let mut new_stats: stat = stat {
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
    let mut ok: bool = 1 as libc::c_int != 0;
    let mut is_stdin: bool = strcmp(
        (*f).name,
        b"-\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int;
    let mut was_tailable: bool = (*f).tailable;
    let mut prev_errnum: libc::c_int = (*f).errnum;
    let mut new_file: bool = false;
    let mut fd: libc::c_int = if is_stdin as libc::c_int != 0 {
        0 as libc::c_int
    } else {
        open_safer(
            (*f).name,
            0 as libc::c_int
                | (if blocking as libc::c_int != 0 {
                    0 as libc::c_int
                } else {
                    0o4000 as libc::c_int
                }),
        )
    };
    if valid_file_spec(f) {} else {
        __assert_fail(
            b"valid_file_spec (f)\0" as *const u8 as *const libc::c_char,
            b"tail.c\0" as *const u8 as *const libc::c_char,
            987 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 40],
                &[libc::c_char; 40],
            >(b"void recheck(struct File_spec *, _Bool)\0"))
                .as_ptr(),
        );
    }
    'c_13353: {
        if valid_file_spec(f) {} else {
            __assert_fail(
                b"valid_file_spec (f)\0" as *const u8 as *const libc::c_char,
                b"tail.c\0" as *const u8 as *const libc::c_char,
                987 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 40],
                    &[libc::c_char; 40],
                >(b"void recheck(struct File_spec *, _Bool)\0"))
                    .as_ptr(),
            );
        }
    };
    (*f)
        .tailable = !(reopen_inaccessible_files as libc::c_int != 0
        && fd == -(1 as libc::c_int));
    if !disable_inotify && lstat((*f).name, &mut new_stats) == 0
        && new_stats.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o120000 as libc::c_int as libc::c_uint
    {
        ok = 0 as libc::c_int != 0;
        (*f).errnum = -(1 as libc::c_int);
        (*f).ignore = 1 as libc::c_int != 0;
        if 0 != 0 {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"%s has been replaced with an untailable symbolic link\0"
                        as *const u8 as *const libc::c_char,
                ),
                quotearg_style(shell_escape_always_quoting_style, pretty_name(f)),
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
                        b"%s has been replaced with an untailable symbolic link\0"
                            as *const u8 as *const libc::c_char,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, pretty_name(f)),
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
                        b"%s has been replaced with an untailable symbolic link\0"
                            as *const u8 as *const libc::c_char,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, pretty_name(f)),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
    } else if fd == -(1 as libc::c_int) || fstat(fd, &mut new_stats) < 0 as libc::c_int {
        ok = 0 as libc::c_int != 0;
        (*f).errnum = *__errno_location();
        if !(*f).tailable {
            if was_tailable {
                if 0 != 0 {
                    error(
                        0 as libc::c_int,
                        (*f).errnum,
                        gettext(
                            b"%s has become inaccessible\0" as *const u8
                                as *const libc::c_char,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, pretty_name(f)),
                    );
                    if 0 as libc::c_int != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                } else {
                    ({
                        let __errstatus: libc::c_int = 0 as libc::c_int;
                        error(
                            __errstatus,
                            (*f).errnum,
                            gettext(
                                b"%s has become inaccessible\0" as *const u8
                                    as *const libc::c_char,
                            ),
                            quotearg_style(
                                shell_escape_always_quoting_style,
                                pretty_name(f),
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
                            (*f).errnum,
                            gettext(
                                b"%s has become inaccessible\0" as *const u8
                                    as *const libc::c_char,
                            ),
                            quotearg_style(
                                shell_escape_always_quoting_style,
                                pretty_name(f),
                            ),
                        );
                        if __errstatus != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                        
                    });
                };
            }
        } else if prev_errnum != *__errno_location() {
            if 0 != 0 {
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    b"%s\0" as *const u8 as *const libc::c_char,
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        pretty_name(f),
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
                            pretty_name(f),
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
                            pretty_name(f),
                        ),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
    } else if !(new_stats.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint
        || new_stats.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o10000 as libc::c_int as libc::c_uint
        || new_stats.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o140000 as libc::c_int as libc::c_uint
        || new_stats.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o20000 as libc::c_int as libc::c_uint)
    {
        ok = 0 as libc::c_int != 0;
        (*f).errnum = -(1 as libc::c_int);
        (*f).tailable = 0 as libc::c_int != 0;
        (*f)
            .ignore = !(reopen_inaccessible_files as libc::c_int != 0
            && follow_mode as libc::c_uint
                == Follow_name as libc::c_int as libc::c_uint);
        if was_tailable as libc::c_int != 0 || prev_errnum != (*f).errnum {
            if 0 != 0 {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"%s has been replaced with an untailable file%s\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, pretty_name(f)),
                    if (*f).ignore as libc::c_int != 0 {
                        gettext(
                            b"; giving up on this name\0" as *const u8
                                as *const libc::c_char,
                        ) as *const libc::c_char
                    } else {
                        b"\0" as *const u8 as *const libc::c_char
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
                        gettext(
                            b"%s has been replaced with an untailable file%s\0"
                                as *const u8 as *const libc::c_char,
                        ),
                        quotearg_style(
                            shell_escape_always_quoting_style,
                            pretty_name(f),
                        ),
                        if (*f).ignore as libc::c_int != 0 {
                            gettext(
                                b"; giving up on this name\0" as *const u8
                                    as *const libc::c_char,
                            ) as *const libc::c_char
                        } else {
                            b"\0" as *const u8 as *const libc::c_char
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
                        gettext(
                            b"%s has been replaced with an untailable file%s\0"
                                as *const u8 as *const libc::c_char,
                        ),
                        quotearg_style(
                            shell_escape_always_quoting_style,
                            pretty_name(f),
                        ),
                        if (*f).ignore as libc::c_int != 0 {
                            gettext(
                                b"; giving up on this name\0" as *const u8
                                    as *const libc::c_char,
                            ) as *const libc::c_char
                        } else {
                            b"\0" as *const u8 as *const libc::c_char
                        },
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
    } else {
        (*f).remote = fremote(fd, pretty_name(f));
        if (*f).remote as libc::c_int != 0 && !disable_inotify {
            ok = 0 as libc::c_int != 0;
            (*f).errnum = -(1 as libc::c_int);
            if 0 != 0 {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"%s has been replaced with an untailable remote file\0"
                            as *const u8 as *const libc::c_char,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, pretty_name(f)),
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
                            b"%s has been replaced with an untailable remote file\0"
                                as *const u8 as *const libc::c_char,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, pretty_name(f)),
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
                            b"%s has been replaced with an untailable remote file\0"
                                as *const u8 as *const libc::c_char,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, pretty_name(f)),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
            (*f).ignore = 1 as libc::c_int != 0;
            (*f).remote = 1 as libc::c_int != 0;
        } else {
            (*f).errnum = 0 as libc::c_int;
        }
    }
    new_file = 0 as libc::c_int != 0;
    if !ok {
        close_fd(fd, pretty_name(f));
        close_fd((*f).fd, pretty_name(f));
        (*f).fd = -(1 as libc::c_int);
    } else if prev_errnum != 0 && prev_errnum != 2 as libc::c_int {
        new_file = 1 as libc::c_int != 0;
        if (*f).fd == -(1 as libc::c_int) {} else {
            __assert_fail(
                b"f->fd == -1\0" as *const u8 as *const libc::c_char,
                b"tail.c\0" as *const u8 as *const libc::c_char,
                1064 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 40],
                    &[libc::c_char; 40],
                >(b"void recheck(struct File_spec *, _Bool)\0"))
                    .as_ptr(),
            );
        }
        'c_12481: {
            if (*f).fd == -(1 as libc::c_int) {} else {
                __assert_fail(
                    b"f->fd == -1\0" as *const u8 as *const libc::c_char,
                    b"tail.c\0" as *const u8 as *const libc::c_char,
                    1064 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 40],
                        &[libc::c_char; 40],
                    >(b"void recheck(struct File_spec *, _Bool)\0"))
                        .as_ptr(),
                );
            }
        };
        if 0 != 0 {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"%s has become accessible\0" as *const u8 as *const libc::c_char,
                ),
                quotearg_style(shell_escape_always_quoting_style, pretty_name(f)),
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
                        b"%s has become accessible\0" as *const u8 as *const libc::c_char,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, pretty_name(f)),
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
                        b"%s has become accessible\0" as *const u8 as *const libc::c_char,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, pretty_name(f)),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
    } else if (*f).fd == -(1 as libc::c_int) {
        new_file = 1 as libc::c_int != 0;
        if 0 != 0 {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"%s has appeared;  following new file\0" as *const u8
                        as *const libc::c_char,
                ),
                quotearg_style(shell_escape_always_quoting_style, pretty_name(f)),
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
                        b"%s has appeared;  following new file\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, pretty_name(f)),
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
                        b"%s has appeared;  following new file\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, pretty_name(f)),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
    } else if (*f).ino != new_stats.st_ino || (*f).dev != new_stats.st_dev {
        new_file = 1 as libc::c_int != 0;
        if 0 != 0 {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"%s has been replaced;  following new file\0" as *const u8
                        as *const libc::c_char,
                ),
                quotearg_style(shell_escape_always_quoting_style, pretty_name(f)),
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
                        b"%s has been replaced;  following new file\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, pretty_name(f)),
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
                        b"%s has been replaced;  following new file\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, pretty_name(f)),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
        close_fd((*f).fd, pretty_name(f));
    } else {
        close_fd(fd, pretty_name(f));
    }
    if new_file {
        record_open_fd(
            f,
            fd,
            0 as libc::c_int as off_t,
            &mut new_stats,
            if is_stdin as libc::c_int != 0 {
                -(1 as libc::c_int)
            } else {
                blocking as libc::c_int
            },
        );
        if new_stats.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o100000 as libc::c_int as libc::c_uint
        {
            xlseek(fd, 0 as libc::c_int as off_t, 0 as libc::c_int, pretty_name(f));
        }
    }
}
unsafe extern "C" fn any_live_files(
    mut f: *const File_spec,
    mut n_files: size_t,
) -> bool {
    if reopen_inaccessible_files as libc::c_int != 0
        && follow_mode as libc::c_uint == Follow_name as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int != 0;
    }
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < n_files {
        if 0 as libc::c_int <= (*f.offset(i as isize)).fd {
            return 1 as libc::c_int != 0
        } else if !(*f.offset(i as isize)).ignore
            && reopen_inaccessible_files as libc::c_int != 0
        {
            return 1 as libc::c_int != 0
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn writers_are_dead() -> bool {
    if nbpids == 0 {
        return 0 as libc::c_int != 0;
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < nbpids {
        if kill(*pids.offset(i as isize), 0 as libc::c_int) == 0 as libc::c_int
            || *__errno_location() == 1 as libc::c_int
        {
            return 0 as libc::c_int != 0;
        }
        i += 1;
        i;
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn tail_forever(
    mut f: *mut File_spec,
    mut n_files: size_t,
    mut sleep_interval: libc::c_double,
) {
    let mut blocking: bool = nbpids == 0
        && follow_mode as libc::c_uint
            == Follow_descriptor as libc::c_int as libc::c_uint
        && n_files == 1 as libc::c_int as libc::c_ulong
        && (*f.offset(0 as libc::c_int as isize)).fd != -(1 as libc::c_int)
        && !((*f.offset(0 as libc::c_int as isize)).mode
            & 0o170000 as libc::c_int as libc::c_uint
            == 0o100000 as libc::c_int as libc::c_uint);
    let mut last: size_t = 0;
    let mut writers_dead: bool = 0 as libc::c_int != 0;
    last = n_files.wrapping_sub(1 as libc::c_int as libc::c_ulong);
    loop {
        let mut i: size_t = 0;
        let mut any_input: bool = 0 as libc::c_int != 0;
        let mut current_block_47: u64;
        i = 0 as libc::c_int as size_t;
        while i < n_files {
            let mut fd: libc::c_int = 0;
            let mut name: *const libc::c_char = 0 as *const libc::c_char;
            let mut mode: mode_t = 0;
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
            let mut bytes_read: uintmax_t = 0;
            if !(*f.offset(i as isize)).ignore {
                if (*f.offset(i as isize)).fd < 0 as libc::c_int {
                    recheck(&mut *f.offset(i as isize), blocking);
                } else {
                    fd = (*f.offset(i as isize)).fd;
                    name = pretty_name(&mut *f.offset(i as isize));
                    mode = (*f.offset(i as isize)).mode;
                    if (*f.offset(i as isize)).blocking != blocking as libc::c_int {
                        let mut old_flags: libc::c_int = rpl_fcntl(fd, 3 as libc::c_int);
                        let mut new_flags: libc::c_int = old_flags
                            | (if blocking as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                0o4000 as libc::c_int
                            });
                        if old_flags < 0 as libc::c_int
                            || new_flags != old_flags
                                && rpl_fcntl(fd, 4 as libc::c_int, new_flags)
                                    == -(1 as libc::c_int)
                        {
                            if !((*f.offset(i as isize)).mode
                                & 0o170000 as libc::c_int as libc::c_uint
                                == 0o100000 as libc::c_int as libc::c_uint
                                && *__errno_location() == 1 as libc::c_int)
                            {
                                if 0 != 0 {
                                    error(
                                        1 as libc::c_int,
                                        *__errno_location(),
                                        gettext(
                                            b"%s: cannot change nonblocking mode\0" as *const u8
                                                as *const libc::c_char,
                                        ),
                                        quotearg_n_style_colon(
                                            0 as libc::c_int,
                                            shell_escape_quoting_style,
                                            name,
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
                                                b"%s: cannot change nonblocking mode\0" as *const u8
                                                    as *const libc::c_char,
                                            ),
                                            quotearg_n_style_colon(
                                                0 as libc::c_int,
                                                shell_escape_quoting_style,
                                                name,
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
                                                b"%s: cannot change nonblocking mode\0" as *const u8
                                                    as *const libc::c_char,
                                            ),
                                            quotearg_n_style_colon(
                                                0 as libc::c_int,
                                                shell_escape_quoting_style,
                                                name,
                                            ),
                                        );
                                        if __errstatus != 0 as libc::c_int {
                                            unreachable!();
                                        } else {};
                                        
                                    });
                                };
                            }
                        } else {
                            (*f.offset(i as isize)).blocking = blocking as libc::c_int;
                        }
                    }
                    let mut read_unchanged: bool = 0 as libc::c_int != 0;
                    if (*f.offset(i as isize)).blocking == 0 {
                        if fstat(fd, &mut stats) != 0 as libc::c_int {
                            (*f.offset(i as isize)).fd = -(1 as libc::c_int);
                            (*f.offset(i as isize)).errnum = *__errno_location();
                            if 0 != 0 {
                                error(
                                    0 as libc::c_int,
                                    *__errno_location(),
                                    b"%s\0" as *const u8 as *const libc::c_char,
                                    quotearg_n_style_colon(
                                        0 as libc::c_int,
                                        shell_escape_quoting_style,
                                        name,
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
                                            name,
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
                                            name,
                                        ),
                                    );
                                    if __errstatus != 0 as libc::c_int {
                                        unreachable!();
                                    } else {};
                                    
                                });
                            };
                            close(fd);
                            current_block_47 = 17778012151635330486;
                        } else {
                            if (*f.offset(i as isize)).mode == stats.st_mode
                                && (!(stats.st_mode
                                    & 0o170000 as libc::c_int as libc::c_uint
                                    == 0o100000 as libc::c_int as libc::c_uint)
                                    || (*f.offset(i as isize)).size == stats.st_size)
                                && timespec_cmp(
                                    (*f.offset(i as isize)).mtime,
                                    get_stat_mtime(&mut stats),
                                ) == 0 as libc::c_int
                            {
                                let ref mut fresh1 = (*f.offset(i as isize))
                                    .n_unchanged_stats;
                                let fresh2 = *fresh1;
                                *fresh1 = (*fresh1).wrapping_add(1);
                                if max_n_unchanged_stats_between_opens <= fresh2
                                    && follow_mode as libc::c_uint
                                        == Follow_name as libc::c_int as libc::c_uint
                                {
                                    recheck(
                                        &mut *f.offset(i as isize),
                                        (*f.offset(i as isize)).blocking != 0,
                                    );
                                    (*f.offset(i as isize))
                                        .n_unchanged_stats = 0 as libc::c_int as uintmax_t;
                                }
                                if fd != (*f.offset(i as isize)).fd
                                    || stats.st_mode & 0o170000 as libc::c_int as libc::c_uint
                                        == 0o100000 as libc::c_int as libc::c_uint
                                    || (1 as libc::c_int as libc::c_ulong) < n_files
                                {
                                    current_block_47 = 17778012151635330486;
                                } else {
                                    read_unchanged = 1 as libc::c_int != 0;
                                    current_block_47 = 8693738493027456495;
                                }
                            } else {
                                current_block_47 = 8693738493027456495;
                            }
                            match current_block_47 {
                                17778012151635330486 => {}
                                _ => {
                                    if fd == (*f.offset(i as isize)).fd {} else {
                                        __assert_fail(
                                            b"fd == f[i].fd\0" as *const u8 as *const libc::c_char,
                                            b"tail.c\0" as *const u8 as *const libc::c_char,
                                            1258 as libc::c_int as libc::c_uint,
                                            (*::core::mem::transmute::<
                                                &[u8; 54],
                                                &[libc::c_char; 54],
                                            >(
                                                b"void tail_forever(struct File_spec *, size_t, double)\0",
                                            ))
                                                .as_ptr(),
                                        );
                                    }
                                    'c_14118: {
                                        if fd == (*f.offset(i as isize)).fd {} else {
                                            __assert_fail(
                                                b"fd == f[i].fd\0" as *const u8 as *const libc::c_char,
                                                b"tail.c\0" as *const u8 as *const libc::c_char,
                                                1258 as libc::c_int as libc::c_uint,
                                                (*::core::mem::transmute::<
                                                    &[u8; 54],
                                                    &[libc::c_char; 54],
                                                >(
                                                    b"void tail_forever(struct File_spec *, size_t, double)\0",
                                                ))
                                                    .as_ptr(),
                                            );
                                        }
                                    };
                                    (*f.offset(i as isize)).mtime = get_stat_mtime(&mut stats);
                                    (*f.offset(i as isize)).mode = stats.st_mode;
                                    if !read_unchanged {
                                        (*f.offset(i as isize))
                                            .n_unchanged_stats = 0 as libc::c_int as uintmax_t;
                                    }
                                    if mode & 0o170000 as libc::c_int as libc::c_uint
                                        == 0o100000 as libc::c_int as libc::c_uint
                                        && stats.st_size < (*f.offset(i as isize)).size
                                    {
                                        if 0 != 0 {
                                            error(
                                                0 as libc::c_int,
                                                0 as libc::c_int,
                                                gettext(
                                                    b"%s: file truncated\0" as *const u8 as *const libc::c_char,
                                                ),
                                                quotearg_n_style_colon(
                                                    0 as libc::c_int,
                                                    shell_escape_quoting_style,
                                                    name,
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
                                                        b"%s: file truncated\0" as *const u8 as *const libc::c_char,
                                                    ),
                                                    quotearg_n_style_colon(
                                                        0 as libc::c_int,
                                                        shell_escape_quoting_style,
                                                        name,
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
                                                        b"%s: file truncated\0" as *const u8 as *const libc::c_char,
                                                    ),
                                                    quotearg_n_style_colon(
                                                        0 as libc::c_int,
                                                        shell_escape_quoting_style,
                                                        name,
                                                    ),
                                                );
                                                if __errstatus != 0 as libc::c_int {
                                                    unreachable!();
                                                } else {};
                                                
                                            });
                                        };
                                        xlseek(
                                            fd,
                                            0 as libc::c_int as off_t,
                                            0 as libc::c_int,
                                            name,
                                        );
                                        (*f.offset(i as isize)).size = 0 as libc::c_int as off_t;
                                    }
                                    if i != last {
                                        if print_headers {
                                            write_header(name);
                                        }
                                        last = i;
                                    }
                                    current_block_47 = 3222590281903869779;
                                }
                            }
                        }
                    } else {
                        current_block_47 = 3222590281903869779;
                    }
                    match current_block_47 {
                        17778012151635330486 => {}
                        _ => {
                            let mut bytes_to_read: uintmax_t = 0;
                            if (*f.offset(i as isize)).blocking != 0 {
                                bytes_to_read = (18446744073709551615 as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                            } else if mode & 0o170000 as libc::c_int as libc::c_uint
                                == 0o100000 as libc::c_int as libc::c_uint
                                && (*f.offset(i as isize)).remote as libc::c_int != 0
                            {
                                bytes_to_read = (stats.st_size
                                    - (*f.offset(i as isize)).size) as uintmax_t;
                            } else {
                                bytes_to_read = 18446744073709551615 as libc::c_ulong;
                            }
                            bytes_read = dump_remainder(
                                0 as libc::c_int != 0,
                                name,
                                fd,
                                bytes_to_read,
                            );
                            if read_unchanged as libc::c_int != 0 && bytes_read != 0 {
                                (*f.offset(i as isize))
                                    .n_unchanged_stats = 0 as libc::c_int as uintmax_t;
                            }
                            any_input = (any_input as libc::c_int
                                | (bytes_read != 0 as libc::c_int as libc::c_ulong)
                                    as libc::c_int) != 0;
                            let ref mut fresh3 = (*f.offset(i as isize)).size;
                            *fresh3 = (*fresh3 as libc::c_ulong).wrapping_add(bytes_read)
                                as off_t as off_t;
                        }
                    }
                }
            }
            i = i.wrapping_add(1);
            i;
        }
        if !any_live_files(f, n_files) {
            if 0 != 0 {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    gettext(b"no files remaining\0" as *const u8 as *const libc::c_char),
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
                            b"no files remaining\0" as *const u8 as *const libc::c_char,
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
                            b"no files remaining\0" as *const u8 as *const libc::c_char,
                        ),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
            break;
        } else {
            if (!any_input || blocking as libc::c_int != 0)
                && fflush_unlocked(stdout) != 0 as libc::c_int
            {
                write_error();
            }
            check_output_alive();
            if any_input {
                continue;
            }
            if writers_dead {
                break;
            }
            writers_dead = writers_are_dead();
            if !writers_dead && xnanosleep(sleep_interval) != 0 {
                if 0 != 0 {
                    error(
                        1 as libc::c_int,
                        *__errno_location(),
                        gettext(
                            b"cannot read realtime clock\0" as *const u8
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
                                b"cannot read realtime clock\0" as *const u8
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
                                b"cannot read realtime clock\0" as *const u8
                                    as *const libc::c_char,
                            ),
                        );
                        if __errstatus != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                        
                    });
                };
            }
        }
    };
}
unsafe extern "C" fn any_remote_file(
    mut f: *const File_spec,
    mut n_files: size_t,
) -> bool {
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < n_files {
        if 0 as libc::c_int <= (*f.offset(i as isize)).fd
            && (*f.offset(i as isize)).remote as libc::c_int != 0
        {
            return 1 as libc::c_int != 0;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn any_non_remote_file(
    mut f: *const File_spec,
    mut n_files: size_t,
) -> bool {
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < n_files {
        if 0 as libc::c_int <= (*f.offset(i as isize)).fd
            && !(*f.offset(i as isize)).remote
        {
            return 1 as libc::c_int != 0;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn any_symlinks(mut f: *const File_spec, mut n_files: size_t) -> bool {
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
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < n_files {
        if lstat((*f.offset(i as isize)).name, &mut st) == 0 as libc::c_int
            && st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o120000 as libc::c_int as libc::c_uint
        {
            return 1 as libc::c_int != 0;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn any_non_regular_fifo(
    mut f: *const File_spec,
    mut n_files: size_t,
) -> bool {
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < n_files {
        if 0 as libc::c_int <= (*f.offset(i as isize)).fd
            && !((*f.offset(i as isize)).mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o100000 as libc::c_int as libc::c_uint)
            && !((*f.offset(i as isize)).mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o10000 as libc::c_int as libc::c_uint)
        {
            return 1 as libc::c_int != 0;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn tailable_stdin(
    mut f: *const File_spec,
    mut n_files: size_t,
) -> bool {
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < n_files {
        if !(*f.offset(i as isize)).ignore
            && strcmp(
                (*f.offset(i as isize)).name,
                b"-\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            return 1 as libc::c_int != 0;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn wd_hasher(
    mut entry: *const libc::c_void,
    mut tabsize: size_t,
) -> size_t {
    let mut spec: *const File_spec = entry as *const File_spec;
    return ((*spec).wd as libc::c_ulong).wrapping_rem(tabsize);
}
unsafe extern "C" fn wd_comparator(
    mut e1: *const libc::c_void,
    mut e2: *const libc::c_void,
) -> bool {
    let mut spec1: *const File_spec = e1 as *const File_spec;
    let mut spec2: *const File_spec = e2 as *const File_spec;
    return (*spec1).wd == (*spec2).wd;
}
unsafe extern "C" fn check_fspec(
    mut fspec: *mut File_spec,
    mut prev_fspec: *mut *mut File_spec,
) {
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
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    if (*fspec).fd == -(1 as libc::c_int) {
        return;
    }
    name = pretty_name(fspec);
    if fstat((*fspec).fd, &mut stats) != 0 as libc::c_int {
        (*fspec).errnum = *__errno_location();
        close_fd((*fspec).fd, name);
        (*fspec).fd = -(1 as libc::c_int);
        return;
    }
    if (*fspec).mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint && stats.st_size < (*fspec).size
    {
        if 0 != 0 {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                gettext(b"%s: file truncated\0" as *const u8 as *const libc::c_char),
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    name,
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
                    gettext(b"%s: file truncated\0" as *const u8 as *const libc::c_char),
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        name,
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
                    gettext(b"%s: file truncated\0" as *const u8 as *const libc::c_char),
                    quotearg_n_style_colon(
                        0 as libc::c_int,
                        shell_escape_quoting_style,
                        name,
                    ),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
        xlseek((*fspec).fd, 0 as libc::c_int as off_t, 0 as libc::c_int, name);
        (*fspec).size = 0 as libc::c_int as off_t;
    } else if (*fspec).mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint && stats.st_size == (*fspec).size
        && timespec_cmp((*fspec).mtime, get_stat_mtime(&mut stats)) == 0 as libc::c_int
    {
        return
    }
    let mut want_header: bool = print_headers as libc::c_int != 0
        && fspec != *prev_fspec;
    let mut bytes_read: uintmax_t = dump_remainder(
        want_header,
        name,
        (*fspec).fd,
        18446744073709551615 as libc::c_ulong,
    );
    (*fspec)
        .size = ((*fspec).size as libc::c_ulong).wrapping_add(bytes_read) as off_t
        as off_t;
    if bytes_read != 0 {
        *prev_fspec = fspec;
        if fflush_unlocked(stdout) != 0 as libc::c_int {
            write_error();
        }
    }
}
unsafe extern "C" fn tail_forever_inotify(
    mut wd: libc::c_int,
    mut f: *mut File_spec,
    mut n_files: size_t,
    mut sleep_interval: libc::c_double,
    mut wd_to_namep: *mut *mut Hash_table,
) {
    let mut max_realloc: libc::c_uint = 3 as libc::c_int as libc::c_uint;
    let mut wd_to_name: *mut Hash_table = 0 as *mut Hash_table;
    let mut found_watchable_file: bool = 0 as libc::c_int != 0;
    let mut tailed_but_unwatchable: bool = 0 as libc::c_int != 0;
    let mut found_unwatchable_dir: bool = 0 as libc::c_int != 0;
    let mut no_inotify_resources: bool = 0 as libc::c_int != 0;
    let mut writers_dead: bool = 0 as libc::c_int != 0;
    let mut prev_fspec: *mut File_spec = 0 as *mut File_spec;
    let mut evlen: size_t = 0 as libc::c_int as size_t;
    let mut evbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut evbuf_off: size_t = 0 as libc::c_int as size_t;
    let mut len: size_t = 0 as libc::c_int as size_t;
    wd_to_name = hash_initialize(
        n_files,
        0 as *const Hash_tuning,
        Some(wd_hasher as unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t),
        Some(
            wd_comparator
                as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> bool,
        ),
        None,
    );
    if wd_to_name.is_null() {
        xalloc_die();
    }
    *wd_to_namep = wd_to_name;
    let mut inotify_wd_mask: uint32_t = 0x2 as libc::c_int as uint32_t;
    if follow_mode as libc::c_uint == Follow_name as libc::c_int as libc::c_uint {
        inotify_wd_mask
            |= (0x4 as libc::c_int | 0x400 as libc::c_int | 0x800 as libc::c_int)
                as libc::c_uint;
    }
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < n_files {
        if !(*f.offset(i as isize)).ignore {
            let mut fnlen: size_t = strlen((*f.offset(i as isize)).name);
            if evlen < fnlen {
                evlen = fnlen;
            }
            (*f.offset(i as isize)).wd = -(1 as libc::c_int);
            if follow_mode as libc::c_uint == Follow_name as libc::c_int as libc::c_uint
            {
                let mut dirlen: size_t = dir_len((*f.offset(i as isize)).name);
                let mut prev: libc::c_char = *((*f.offset(i as isize)).name)
                    .offset(dirlen as isize);
                (*f.offset(i as isize))
                    .basename_start = (last_component((*f.offset(i as isize)).name))
                    .offset_from((*f.offset(i as isize)).name) as libc::c_long as size_t;
                *((*f.offset(i as isize)).name)
                    .offset(dirlen as isize) = '\0' as i32 as libc::c_char;
                (*f.offset(i as isize))
                    .parent_wd = inotify_add_watch(
                    wd,
                    if dirlen != 0 {
                        (*f.offset(i as isize)).name as *const libc::c_char
                    } else {
                        b".\0" as *const u8 as *const libc::c_char
                    },
                    (0x100 as libc::c_int | 0x200 as libc::c_int | 0x80 as libc::c_int
                        | 0x4 as libc::c_int | 0x400 as libc::c_int) as uint32_t,
                );
                *((*f.offset(i as isize)).name).offset(dirlen as isize) = prev;
                if (*f.offset(i as isize)).parent_wd < 0 as libc::c_int {
                    if *__errno_location() != 28 as libc::c_int {
                        if 0 != 0 {
                            error(
                                0 as libc::c_int,
                                *__errno_location(),
                                gettext(
                                    b"cannot watch parent directory of %s\0" as *const u8
                                        as *const libc::c_char,
                                ),
                                quotearg_style(
                                    shell_escape_always_quoting_style,
                                    (*f.offset(i as isize)).name,
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
                                    gettext(
                                        b"cannot watch parent directory of %s\0" as *const u8
                                            as *const libc::c_char,
                                    ),
                                    quotearg_style(
                                        shell_escape_always_quoting_style,
                                        (*f.offset(i as isize)).name,
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
                                    gettext(
                                        b"cannot watch parent directory of %s\0" as *const u8
                                            as *const libc::c_char,
                                    ),
                                    quotearg_style(
                                        shell_escape_always_quoting_style,
                                        (*f.offset(i as isize)).name,
                                    ),
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
                                    b"inotify resources exhausted\0" as *const u8
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
                                        b"inotify resources exhausted\0" as *const u8
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
                                        b"inotify resources exhausted\0" as *const u8
                                            as *const libc::c_char,
                                    ),
                                );
                                if __errstatus != 0 as libc::c_int {
                                    unreachable!();
                                } else {};
                                
                            });
                        };
                    }
                    found_unwatchable_dir = 1 as libc::c_int != 0;
                    break;
                }
            }
            (*f.offset(i as isize))
                .wd = inotify_add_watch(
                wd,
                (*f.offset(i as isize)).name,
                inotify_wd_mask,
            );
            if (*f.offset(i as isize)).wd < 0 as libc::c_int {
                if (*f.offset(i as isize)).fd != -(1 as libc::c_int) {
                    tailed_but_unwatchable = 1 as libc::c_int != 0;
                }
                if *__errno_location() == 28 as libc::c_int
                    || *__errno_location() == 12 as libc::c_int
                {
                    no_inotify_resources = 1 as libc::c_int != 0;
                    if 0 != 0 {
                        error(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            gettext(
                                b"inotify resources exhausted\0" as *const u8
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
                                    b"inotify resources exhausted\0" as *const u8
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
                                    b"inotify resources exhausted\0" as *const u8
                                        as *const libc::c_char,
                                ),
                            );
                            if __errstatus != 0 as libc::c_int {
                                unreachable!();
                            } else {};
                            
                        });
                    };
                    break;
                } else if *__errno_location() != (*f.offset(i as isize)).errnum {
                    if 0 != 0 {
                        error(
                            0 as libc::c_int,
                            *__errno_location(),
                            gettext(
                                b"cannot watch %s\0" as *const u8 as *const libc::c_char,
                            ),
                            quotearg_style(
                                shell_escape_always_quoting_style,
                                (*f.offset(i as isize)).name,
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
                                gettext(
                                    b"cannot watch %s\0" as *const u8 as *const libc::c_char,
                                ),
                                quotearg_style(
                                    shell_escape_always_quoting_style,
                                    (*f.offset(i as isize)).name,
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
                                gettext(
                                    b"cannot watch %s\0" as *const u8 as *const libc::c_char,
                                ),
                                quotearg_style(
                                    shell_escape_always_quoting_style,
                                    (*f.offset(i as isize)).name,
                                ),
                            );
                            if __errstatus != 0 as libc::c_int {
                                unreachable!();
                            } else {};
                            
                        });
                    };
                }
            } else {
                if (hash_insert(
                    wd_to_name,
                    &mut *f.offset(i as isize) as *mut File_spec as *const libc::c_void,
                ))
                    .is_null()
                {
                    xalloc_die();
                }
                found_watchable_file = 1 as libc::c_int != 0;
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    if no_inotify_resources as libc::c_int != 0
        || found_unwatchable_dir as libc::c_int != 0
        || follow_mode as libc::c_uint
            == Follow_descriptor as libc::c_int as libc::c_uint
            && tailed_but_unwatchable as libc::c_int != 0
    {
        return;
    }
    if follow_mode as libc::c_uint == Follow_descriptor as libc::c_int as libc::c_uint
        && !found_watchable_file
    {
        exit(1 as libc::c_int);
    }
    prev_fspec = &mut *f
        .offset(n_files.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
        as *mut File_spec;
    i = 0 as libc::c_int as size_t;
    while i < n_files {
        if !(*f.offset(i as isize)).ignore {
            if follow_mode as libc::c_uint == Follow_name as libc::c_int as libc::c_uint
            {
                recheck(&mut *f.offset(i as isize), 0 as libc::c_int != 0);
            } else if (*f.offset(i as isize)).fd != -(1 as libc::c_int) {
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
                if stat((*f.offset(i as isize)).name, &mut stats) == 0 as libc::c_int
                    && ((*f.offset(i as isize)).dev != stats.st_dev
                        || (*f.offset(i as isize)).ino != stats.st_ino)
                {
                    if 0 != 0 {
                        error(
                            0 as libc::c_int,
                            *__errno_location(),
                            gettext(
                                b"%s was replaced\0" as *const u8 as *const libc::c_char,
                            ),
                            quotearg_style(
                                shell_escape_always_quoting_style,
                                pretty_name(&mut *f.offset(i as isize)),
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
                                gettext(
                                    b"%s was replaced\0" as *const u8 as *const libc::c_char,
                                ),
                                quotearg_style(
                                    shell_escape_always_quoting_style,
                                    pretty_name(&mut *f.offset(i as isize)),
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
                                gettext(
                                    b"%s was replaced\0" as *const u8 as *const libc::c_char,
                                ),
                                quotearg_style(
                                    shell_escape_always_quoting_style,
                                    pretty_name(&mut *f.offset(i as isize)),
                                ),
                            );
                            if __errstatus != 0 as libc::c_int {
                                unreachable!();
                            } else {};
                            
                        });
                    };
                    return;
                }
            }
            check_fspec(&mut *f.offset(i as isize), &mut prev_fspec);
        }
        i = i.wrapping_add(1);
        i;
    }
    evlen = (evlen as libc::c_ulong)
        .wrapping_add(
            (::core::mem::size_of::<inotify_event>() as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as size_t as size_t;
    evbuf = xmalloc(evlen) as *mut libc::c_char;
    loop {
        let mut fspec: *mut File_spec = 0 as *mut File_spec;
        let mut ev: *mut inotify_event = 0 as *mut inotify_event;
        let mut void_ev: *mut libc::c_void = 0 as *mut libc::c_void;
        if follow_mode as libc::c_uint == Follow_name as libc::c_int as libc::c_uint
            && !reopen_inaccessible_files
            && hash_get_n_entries(wd_to_name) == 0 as libc::c_int as libc::c_ulong
        {
            if 0 != 0 {
                error(
                    1 as libc::c_int,
                    0 as libc::c_int,
                    gettext(b"no files remaining\0" as *const u8 as *const libc::c_char),
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
                            b"no files remaining\0" as *const u8 as *const libc::c_char,
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
                            b"no files remaining\0" as *const u8 as *const libc::c_char,
                        ),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
        if len <= evbuf_off {
            let mut file_change: libc::c_int = 0;
            let mut pfd: [pollfd; 2] = [pollfd {
                fd: 0,
                events: 0,
                revents: 0,
            }; 2];
            loop {
                let mut delay: libc::c_int = -(1 as libc::c_int);
                if nbpids != 0 {
                    if writers_dead {
                        exit(0 as libc::c_int);
                    }
                    writers_dead = writers_are_dead();
                    if writers_dead as libc::c_int != 0
                        || sleep_interval <= 0 as libc::c_int as libc::c_double
                    {
                        delay = 0 as libc::c_int;
                    } else if sleep_interval
                        < (2147483647 as libc::c_int / 1000 as libc::c_int
                            - 1 as libc::c_int) as libc::c_double
                    {
                        let mut ddelay: libc::c_double = sleep_interval
                            * 1000 as libc::c_int as libc::c_double;
                        delay = ddelay as libc::c_int;
                        delay += ((delay as libc::c_double) < ddelay) as libc::c_int;
                    }
                }
                pfd[0 as libc::c_int as usize].fd = wd;
                pfd[0 as libc::c_int as usize]
                    .events = 0x1 as libc::c_int as libc::c_short;
                pfd[1 as libc::c_int as usize].fd = 1 as libc::c_int;
                pfd[1 as libc::c_int as usize]
                    .revents = 0 as libc::c_int as libc::c_short;
                pfd[1 as libc::c_int as usize]
                    .events = pfd[1 as libc::c_int as usize].revents;
                file_change = poll(
                    pfd.as_mut_ptr(),
                    (monitor_output as libc::c_int + 1 as libc::c_int) as nfds_t,
                    delay,
                );
                if !(file_change == 0 as libc::c_int) {
                    break;
                }
            }
            if file_change < 0 as libc::c_int {
                if 0 != 0 {
                    error(
                        1 as libc::c_int,
                        *__errno_location(),
                        gettext(
                            b"error waiting for inotify and output events\0" as *const u8
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
                                b"error waiting for inotify and output events\0"
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
                            *__errno_location(),
                            gettext(
                                b"error waiting for inotify and output events\0"
                                    as *const u8 as *const libc::c_char,
                            ),
                        );
                        if __errstatus != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                        
                    });
                };
            }
            if pfd[1 as libc::c_int as usize].revents != 0 {
                die_pipe();
            }
            len = safe_read(wd, evbuf as *mut libc::c_void, evlen);
            evbuf_off = 0 as libc::c_int as size_t;
            if (len == 0 as libc::c_int as libc::c_ulong
                || len == -(1 as libc::c_int) as size_t
                    && *__errno_location() == 22 as libc::c_int)
                && {
                    let fresh4 = max_realloc;
                    max_realloc = max_realloc.wrapping_sub(1);
                    fresh4 != 0
                }
            {
                len = 0 as libc::c_int as size_t;
                evlen = (evlen as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
                evbuf = xrealloc(evbuf as *mut libc::c_void, evlen) as *mut libc::c_char;
                continue;
            } else if len == 0 as libc::c_int as libc::c_ulong
                || len == -(1 as libc::c_int) as size_t
            {
                if 0 != 0 {
                    error(
                        1 as libc::c_int,
                        *__errno_location(),
                        gettext(
                            b"error reading inotify event\0" as *const u8
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
                                b"error reading inotify event\0" as *const u8
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
                                b"error reading inotify event\0" as *const u8
                                    as *const libc::c_char,
                            ),
                        );
                        if __errstatus != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                        
                    });
                };
            }
        }
        void_ev = evbuf.offset(evbuf_off as isize) as *mut libc::c_void;
        ev = void_ev as *mut inotify_event;
        evbuf_off = (evbuf_off as libc::c_ulong)
            .wrapping_add(
                (::core::mem::size_of::<inotify_event>() as libc::c_ulong)
                    .wrapping_add((*ev).len as libc::c_ulong),
            ) as size_t as size_t;
        if (*ev).mask & 0x400 as libc::c_int as libc::c_uint != 0 && (*ev).len == 0 {
            i = 0 as libc::c_int as size_t;
            while i < n_files {
                if (*ev).wd == (*f.offset(i as isize)).parent_wd {
                    if 0 != 0 {
                        error(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            gettext(
                                b"directory containing watched file was removed\0"
                                    as *const u8 as *const libc::c_char,
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
                                    b"directory containing watched file was removed\0"
                                        as *const u8 as *const libc::c_char,
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
                                    b"directory containing watched file was removed\0"
                                        as *const u8 as *const libc::c_char,
                                ),
                            );
                            if __errstatus != 0 as libc::c_int {
                                unreachable!();
                            } else {};
                            
                        });
                    };
                    return;
                }
                i = i.wrapping_add(1);
                i;
            }
        }
        if (*ev).len != 0 {
            let mut j: size_t = 0;
            j = 0 as libc::c_int as size_t;
            while j < n_files {
                if (*f.offset(j as isize)).parent_wd == (*ev).wd
                    && strcmp(
                        ((*ev).name).as_mut_ptr(),
                        ((*f.offset(j as isize)).name)
                            .offset((*f.offset(j as isize)).basename_start as isize),
                    ) == 0 as libc::c_int
                {
                    break;
                }
                j = j.wrapping_add(1);
                j;
            }
            if j == n_files {
                continue;
            }
            fspec = &mut *f.offset(j as isize) as *mut File_spec;
            let mut new_wd: libc::c_int = -(1 as libc::c_int);
            let mut deleting: bool = (*ev).mask & 0x200 as libc::c_int as libc::c_uint
                != 0;
            if !deleting {
                new_wd = inotify_add_watch(
                    wd,
                    (*f.offset(j as isize)).name,
                    inotify_wd_mask,
                );
            }
            if !deleting && new_wd < 0 as libc::c_int {
                if *__errno_location() == 28 as libc::c_int
                    || *__errno_location() == 12 as libc::c_int
                {
                    if 0 != 0 {
                        error(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            gettext(
                                b"inotify resources exhausted\0" as *const u8
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
                                    b"inotify resources exhausted\0" as *const u8
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
                                    b"inotify resources exhausted\0" as *const u8
                                        as *const libc::c_char,
                                ),
                            );
                            if __errstatus != 0 as libc::c_int {
                                unreachable!();
                            } else {};
                            
                        });
                    };
                    return;
                } else {
                    if 0 != 0 {
                        error(
                            0 as libc::c_int,
                            *__errno_location(),
                            gettext(
                                b"cannot watch %s\0" as *const u8 as *const libc::c_char,
                            ),
                            quotearg_style(
                                shell_escape_always_quoting_style,
                                (*f.offset(j as isize)).name,
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
                                gettext(
                                    b"cannot watch %s\0" as *const u8 as *const libc::c_char,
                                ),
                                quotearg_style(
                                    shell_escape_always_quoting_style,
                                    (*f.offset(j as isize)).name,
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
                                gettext(
                                    b"cannot watch %s\0" as *const u8 as *const libc::c_char,
                                ),
                                quotearg_style(
                                    shell_escape_always_quoting_style,
                                    (*f.offset(j as isize)).name,
                                ),
                            );
                            if __errstatus != 0 as libc::c_int {
                                unreachable!();
                            } else {};
                            
                        });
                    };
                }
            }
            let mut new_watch: bool = false;
            new_watch = !deleting
                && ((*fspec).wd < 0 as libc::c_int || new_wd != (*fspec).wd);
            if new_watch {
                if 0 as libc::c_int <= (*fspec).wd {
                    inotify_rm_watch(wd, (*fspec).wd);
                    hash_remove(wd_to_name, fspec as *const libc::c_void);
                }
                (*fspec).wd = new_wd;
                if new_wd == -(1 as libc::c_int) {
                    continue;
                }
                let mut prev_0: *mut File_spec = hash_remove(
                    wd_to_name,
                    fspec as *const libc::c_void,
                ) as *mut File_spec;
                if !prev_0.is_null() && prev_0 != fspec {
                    if follow_mode as libc::c_uint
                        == Follow_name as libc::c_int as libc::c_uint
                    {
                        recheck(prev_0, 0 as libc::c_int != 0);
                    }
                    (*prev_0).wd = -(1 as libc::c_int);
                    close_fd((*prev_0).fd, pretty_name(prev_0));
                }
                if (hash_insert(wd_to_name, fspec as *const libc::c_void)).is_null() {
                    xalloc_die();
                }
            }
            if follow_mode as libc::c_uint == Follow_name as libc::c_int as libc::c_uint
            {
                recheck(fspec, 0 as libc::c_int != 0);
            }
        } else {
            let mut key: File_spec = File_spec {
                name: 0 as *mut libc::c_char,
                size: 0,
                mtime: timespec { tv_sec: 0, tv_nsec: 0 },
                dev: 0,
                ino: 0,
                mode: 0,
                ignore: false,
                remote: false,
                tailable: false,
                fd: 0,
                errnum: 0,
                blocking: 0,
                wd: 0,
                parent_wd: 0,
                basename_start: 0,
                n_unchanged_stats: 0,
            };
            key.wd = (*ev).wd;
            fspec = hash_lookup(
                wd_to_name,
                &mut key as *mut File_spec as *const libc::c_void,
            ) as *mut File_spec;
        }
        if fspec.is_null() {
            continue;
        }
        if (*ev).mask
            & (0x4 as libc::c_int | 0x200 as libc::c_int | 0x400 as libc::c_int
                | 0x800 as libc::c_int) as libc::c_uint != 0
        {
            if (*ev).mask & 0x400 as libc::c_int as libc::c_uint != 0 {
                inotify_rm_watch(wd, (*fspec).wd);
                hash_remove(wd_to_name, fspec as *const libc::c_void);
            }
            recheck(fspec, 0 as libc::c_int != 0);
        } else {
            check_fspec(fspec, &mut prev_fspec);
        }
    };
}
unsafe extern "C" fn tail_bytes(
    mut pretty_filename: *const libc::c_char,
    mut fd: libc::c_int,
    mut n_bytes: uintmax_t,
    mut read_pos: *mut uintmax_t,
) -> bool {
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
    if fstat(fd, &mut stats) != 0 {
        if 0 != 0 {
            error(
                0 as libc::c_int,
                *__errno_location(),
                gettext(b"cannot fstat %s\0" as *const u8 as *const libc::c_char),
                quotearg_style(shell_escape_always_quoting_style, pretty_filename),
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
                    gettext(b"cannot fstat %s\0" as *const u8 as *const libc::c_char),
                    quotearg_style(shell_escape_always_quoting_style, pretty_filename),
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
                    gettext(b"cannot fstat %s\0" as *const u8 as *const libc::c_char),
                    quotearg_style(shell_escape_always_quoting_style, pretty_filename),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
        return 0 as libc::c_int != 0;
    }
    if from_start {
        if !presume_input_pipe
            && n_bytes
                <= (if (0 as libc::c_int as off_t) < -(1 as libc::c_int) as off_t {
                    -(1 as libc::c_int) as off_t
                } else {
                    (((1 as libc::c_int as off_t)
                        << (::core::mem::size_of::<off_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int as libc::c_long)
                        * 2 as libc::c_int as libc::c_long
                        + 1 as libc::c_int as libc::c_long
                }) as libc::c_ulong
            && (stats.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o100000 as libc::c_int as libc::c_uint
                && xlseek(fd, n_bytes as off_t, 1 as libc::c_int, pretty_filename)
                    >= 0 as libc::c_int as libc::c_long
                || lseek(fd, n_bytes as __off_t, 1 as libc::c_int)
                    != -(1 as libc::c_int) as libc::c_long)
        {
            *read_pos = (*read_pos as libc::c_ulong).wrapping_add(n_bytes) as uintmax_t
                as uintmax_t;
        } else {
            let mut t: libc::c_int = start_bytes(pretty_filename, fd, n_bytes, read_pos);
            if t != 0 {
                return t < 0 as libc::c_int;
            }
        }
        n_bytes = 18446744073709551615 as libc::c_ulong;
    } else {
        let mut end_pos: off_t = -(1 as libc::c_int) as off_t;
        let mut current_pos: off_t = -(1 as libc::c_int) as off_t;
        let mut copy_from_current_pos: bool = 0 as libc::c_int != 0;
        if !presume_input_pipe
            && n_bytes
                <= (if (0 as libc::c_int as off_t) < -(1 as libc::c_int) as off_t {
                    -(1 as libc::c_int) as off_t
                } else {
                    (((1 as libc::c_int as off_t)
                        << (::core::mem::size_of::<off_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int as libc::c_long)
                        * 2 as libc::c_int as libc::c_long
                        + 1 as libc::c_int as libc::c_long
                }) as libc::c_ulong
        {
            if usable_st_size(&mut stats) {
                end_pos = stats.st_size;
                let mut smallish_size: off_t = (if (0 as libc::c_int) < stats.st_blksize
                    && stats.st_blksize as libc::c_ulong
                        <= (-(1 as libc::c_int) as size_t)
                            .wrapping_div(8 as libc::c_int as libc::c_ulong)
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                {
                    stats.st_blksize
                } else {
                    512 as libc::c_int
                }) as off_t;
                copy_from_current_pos = smallish_size < end_pos;
            } else {
                current_pos = lseek(
                    fd,
                    n_bytes.wrapping_neg() as __off_t,
                    2 as libc::c_int,
                );
                copy_from_current_pos = current_pos
                    != -(1 as libc::c_int) as libc::c_long;
                if copy_from_current_pos {
                    end_pos = (current_pos as libc::c_ulong).wrapping_add(n_bytes)
                        as off_t;
                }
            }
        }
        if !copy_from_current_pos {
            return pipe_bytes(pretty_filename, fd, n_bytes, read_pos);
        }
        if current_pos == -(1 as libc::c_int) as libc::c_long {
            current_pos = xlseek(
                fd,
                0 as libc::c_int as off_t,
                1 as libc::c_int,
                pretty_filename,
            );
        }
        if current_pos < end_pos {
            let mut bytes_remaining: off_t = end_pos - current_pos;
            if n_bytes < bytes_remaining as libc::c_ulong {
                current_pos = (end_pos as libc::c_ulong).wrapping_sub(n_bytes) as off_t;
                xlseek(fd, current_pos, 0 as libc::c_int, pretty_filename);
            }
        }
        *read_pos = current_pos as uintmax_t;
    }
    *read_pos = (*read_pos as libc::c_ulong)
        .wrapping_add(
            dump_remainder(0 as libc::c_int != 0, pretty_filename, fd, n_bytes),
        ) as uintmax_t as uintmax_t;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn tail_lines(
    mut pretty_filename: *const libc::c_char,
    mut fd: libc::c_int,
    mut n_lines: uintmax_t,
    mut read_pos: *mut uintmax_t,
) -> bool {
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
    if fstat(fd, &mut stats) != 0 {
        if 0 != 0 {
            error(
                0 as libc::c_int,
                *__errno_location(),
                gettext(b"cannot fstat %s\0" as *const u8 as *const libc::c_char),
                quotearg_style(shell_escape_always_quoting_style, pretty_filename),
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
                    gettext(b"cannot fstat %s\0" as *const u8 as *const libc::c_char),
                    quotearg_style(shell_escape_always_quoting_style, pretty_filename),
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
                    gettext(b"cannot fstat %s\0" as *const u8 as *const libc::c_char),
                    quotearg_style(shell_escape_always_quoting_style, pretty_filename),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
        return 0 as libc::c_int != 0;
    }
    if from_start {
        let mut t: libc::c_int = start_lines(pretty_filename, fd, n_lines, read_pos);
        if t != 0 {
            return t < 0 as libc::c_int;
        }
        *read_pos = (*read_pos as libc::c_ulong)
            .wrapping_add(
                dump_remainder(
                    0 as libc::c_int != 0,
                    pretty_filename,
                    fd,
                    18446744073709551615 as libc::c_ulong,
                ),
            ) as uintmax_t as uintmax_t;
    } else {
        let mut start_pos: off_t = -(1 as libc::c_int) as off_t;
        let mut end_pos: off_t = 0;
        if !presume_input_pipe
            && stats.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o100000 as libc::c_int as libc::c_uint
            && {
                start_pos = lseek(fd, 0 as libc::c_int as __off_t, 1 as libc::c_int);
                start_pos != -(1 as libc::c_int) as libc::c_long
            }
            && {
                end_pos = lseek(fd, 0 as libc::c_int as __off_t, 2 as libc::c_int);
                start_pos < end_pos
            }
        {
            *read_pos = end_pos as uintmax_t;
            if end_pos != 0 as libc::c_int as libc::c_long
                && !file_lines(
                    pretty_filename,
                    fd,
                    &mut stats,
                    n_lines,
                    start_pos,
                    end_pos,
                    read_pos,
                )
            {
                return 0 as libc::c_int != 0;
            }
        } else {
            if start_pos != -(1 as libc::c_int) as libc::c_long {
                xlseek(fd, start_pos, 0 as libc::c_int, pretty_filename);
            }
            return pipe_lines(pretty_filename, fd, n_lines, read_pos);
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn tail(
    mut filename: *const libc::c_char,
    mut fd: libc::c_int,
    mut n_units: uintmax_t,
    mut read_pos: *mut uintmax_t,
) -> bool {
    *read_pos = 0 as libc::c_int as uintmax_t;
    if count_lines {
        return tail_lines(filename, fd, n_units, read_pos)
    } else {
        return tail_bytes(filename, fd, n_units, read_pos)
    };
}
unsafe extern "C" fn tail_file(mut f: *mut File_spec, mut n_units: uintmax_t) -> bool {
    let mut fd: libc::c_int = 0;
    let mut ok: bool = false;
    let mut is_stdin: bool = strcmp(
        (*f).name,
        b"-\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int;
    if is_stdin {
        have_read_stdin = 1 as libc::c_int != 0;
        fd = 0 as libc::c_int;
        xset_binary_mode(0 as libc::c_int, 0 as libc::c_int);
    } else {
        fd = open_safer((*f).name, 0 as libc::c_int | 0 as libc::c_int);
    }
    (*f)
        .tailable = !(reopen_inaccessible_files as libc::c_int != 0
        && fd == -(1 as libc::c_int));
    if fd == -(1 as libc::c_int) {
        if forever {
            (*f).fd = -(1 as libc::c_int);
            (*f).errnum = *__errno_location();
            (*f).ignore = !reopen_inaccessible_files;
            (*f).ino = 0 as libc::c_int as ino_t;
            (*f).dev = 0 as libc::c_int as dev_t;
        }
        if 0 != 0 {
            error(
                0 as libc::c_int,
                *__errno_location(),
                gettext(
                    b"cannot open %s for reading\0" as *const u8 as *const libc::c_char,
                ),
                quotearg_style(shell_escape_always_quoting_style, pretty_name(f)),
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
                        b"cannot open %s for reading\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, pretty_name(f)),
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
                        b"cannot open %s for reading\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, pretty_name(f)),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
        ok = 0 as libc::c_int != 0;
    } else {
        let mut read_pos: uintmax_t = 0;
        if print_headers {
            write_header(pretty_name(f));
        }
        ok = tail(pretty_name(f), fd, n_units, &mut read_pos);
        if forever {
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
            (*f).errnum = ok as libc::c_int - 1 as libc::c_int;
            if fstat(fd, &mut stats) < 0 as libc::c_int {
                ok = 0 as libc::c_int != 0;
                (*f).errnum = *__errno_location();
                if 0 != 0 {
                    error(
                        0 as libc::c_int,
                        *__errno_location(),
                        gettext(
                            b"error reading %s\0" as *const u8 as *const libc::c_char,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, pretty_name(f)),
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
                                b"error reading %s\0" as *const u8 as *const libc::c_char,
                            ),
                            quotearg_style(
                                shell_escape_always_quoting_style,
                                pretty_name(f),
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
                            gettext(
                                b"error reading %s\0" as *const u8 as *const libc::c_char,
                            ),
                            quotearg_style(
                                shell_escape_always_quoting_style,
                                pretty_name(f),
                            ),
                        );
                        if __errstatus != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                        
                    });
                };
            } else if !(stats.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o100000 as libc::c_int as libc::c_uint
                || stats.st_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o10000 as libc::c_int as libc::c_uint
                || stats.st_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o140000 as libc::c_int as libc::c_uint
                || stats.st_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o20000 as libc::c_int as libc::c_uint)
            {
                ok = 0 as libc::c_int != 0;
                (*f).errnum = -(1 as libc::c_int);
                (*f).tailable = 0 as libc::c_int != 0;
                (*f).ignore = !reopen_inaccessible_files;
                if 0 != 0 {
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        gettext(
                            b"%s: cannot follow end of this type of file%s\0"
                                as *const u8 as *const libc::c_char,
                        ),
                        quotearg_n_style_colon(
                            0 as libc::c_int,
                            shell_escape_quoting_style,
                            pretty_name(f),
                        ),
                        if (*f).ignore as libc::c_int != 0 {
                            gettext(
                                b"; giving up on this name\0" as *const u8
                                    as *const libc::c_char,
                            ) as *const libc::c_char
                        } else {
                            b"\0" as *const u8 as *const libc::c_char
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
                            gettext(
                                b"%s: cannot follow end of this type of file%s\0"
                                    as *const u8 as *const libc::c_char,
                            ),
                            quotearg_n_style_colon(
                                0 as libc::c_int,
                                shell_escape_quoting_style,
                                pretty_name(f),
                            ),
                            if (*f).ignore as libc::c_int != 0 {
                                gettext(
                                    b"; giving up on this name\0" as *const u8
                                        as *const libc::c_char,
                                ) as *const libc::c_char
                            } else {
                                b"\0" as *const u8 as *const libc::c_char
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
                            gettext(
                                b"%s: cannot follow end of this type of file%s\0"
                                    as *const u8 as *const libc::c_char,
                            ),
                            quotearg_n_style_colon(
                                0 as libc::c_int,
                                shell_escape_quoting_style,
                                pretty_name(f),
                            ),
                            if (*f).ignore as libc::c_int != 0 {
                                gettext(
                                    b"; giving up on this name\0" as *const u8
                                        as *const libc::c_char,
                                ) as *const libc::c_char
                            } else {
                                b"\0" as *const u8 as *const libc::c_char
                            },
                        );
                        if __errstatus != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                        
                    });
                };
            }
            if !ok {
                (*f).ignore = !reopen_inaccessible_files;
                close_fd(fd, pretty_name(f));
                (*f).fd = -(1 as libc::c_int);
            } else {
                record_open_fd(
                    f,
                    fd,
                    read_pos as off_t,
                    &mut stats,
                    if is_stdin as libc::c_int != 0 {
                        -(1 as libc::c_int)
                    } else {
                        1 as libc::c_int
                    },
                );
                (*f).remote = fremote(fd, pretty_name(f));
            }
        } else if !is_stdin && close(fd) != 0 {
            if 0 != 0 {
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    gettext(b"error reading %s\0" as *const u8 as *const libc::c_char),
                    quotearg_style(shell_escape_always_quoting_style, pretty_name(f)),
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
                            b"error reading %s\0" as *const u8 as *const libc::c_char,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, pretty_name(f)),
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
                            b"error reading %s\0" as *const u8 as *const libc::c_char,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, pretty_name(f)),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
            ok = 0 as libc::c_int != 0;
        }
    }
    return ok;
}
unsafe extern "C" fn parse_obsolete_option(
    mut argc: libc::c_int,
    mut argv: *const *mut libc::c_char,
    mut n_units: *mut uintmax_t,
) -> bool {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut n_string: *const libc::c_char = 0 as *const libc::c_char;
    let mut n_string_end: *const libc::c_char = 0 as *const libc::c_char;
    let mut default_count: libc::c_int = 10 as libc::c_int;
    let mut t_from_start: bool = false;
    let mut t_count_lines: bool = 1 as libc::c_int != 0;
    let mut t_forever: bool = 0 as libc::c_int != 0;
    if !(argc == 2 as libc::c_int
        || argc == 3 as libc::c_int
            && !(*(*argv.offset(2 as libc::c_int as isize))
                .offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
                && *(*argv.offset(2 as libc::c_int as isize))
                    .offset(1 as libc::c_int as isize) as libc::c_int != 0)
        || 3 as libc::c_int <= argc && argc <= 4 as libc::c_int
            && strcmp(
                *argv.offset(2 as libc::c_int as isize),
                b"--\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int)
    {
        return 0 as libc::c_int != 0;
    }
    let mut posix_ver: libc::c_int = posix2_version();
    let mut obsolete_usage: bool = posix_ver < 200112 as libc::c_int;
    let mut traditional_usage: bool = obsolete_usage as libc::c_int != 0
        || 200809 as libc::c_int <= posix_ver;
    p = *argv.offset(1 as libc::c_int as isize);
    let fresh5 = p;
    p = p.offset(1);
    match *fresh5 as libc::c_int {
        43 => {
            if !traditional_usage {
                return 0 as libc::c_int != 0;
            }
            t_from_start = 1 as libc::c_int != 0;
        }
        45 => {
            if !obsolete_usage
                && *p
                    .offset(
                        (*p.offset(0 as libc::c_int as isize) as libc::c_int
                            == 'c' as i32) as libc::c_int as isize,
                    ) == 0
            {
                return 0 as libc::c_int != 0;
            }
            t_from_start = 0 as libc::c_int != 0;
        }
        _ => return 0 as libc::c_int != 0,
    }
    n_string = p;
    while (*p as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
        <= 9 as libc::c_int as libc::c_uint
    {
        p = p.offset(1);
        p;
    }
    n_string_end = p;
    let mut current_block_19: u64;
    match *p as libc::c_int {
        98 => {
            default_count *= 512 as libc::c_int;
            current_block_19 = 12856559154846489347;
        }
        99 => {
            current_block_19 = 12856559154846489347;
        }
        108 => {
            current_block_19 = 7044594549367080378;
        }
        _ => {
            current_block_19 = 5783071609795492627;
        }
    }
    match current_block_19 {
        12856559154846489347 => {
            t_count_lines = 0 as libc::c_int != 0;
            current_block_19 = 7044594549367080378;
        }
        _ => {}
    }
    match current_block_19 {
        7044594549367080378 => {
            p = p.offset(1);
            p;
        }
        _ => {}
    }
    if *p as libc::c_int == 'f' as i32 {
        t_forever = 1 as libc::c_int != 0;
        p = p.offset(1);
        p;
    }
    if *p != 0 {
        return 0 as libc::c_int != 0;
    }
    if n_string == n_string_end {
        *n_units = default_count as uintmax_t;
    } else if xstrtoumax(
        n_string,
        0 as *mut *mut libc::c_char,
        10 as libc::c_int,
        n_units,
        b"b\0" as *const u8 as *const libc::c_char,
    ) as libc::c_uint & !(LONGINT_INVALID_SUFFIX_CHAR as libc::c_int) as libc::c_uint
        != LONGINT_OK as libc::c_int as libc::c_uint
    {
        if 0 != 0 {
            error(
                1 as libc::c_int,
                *__errno_location(),
                b"%s: %s\0" as *const u8 as *const libc::c_char,
                gettext(b"invalid number\0" as *const u8 as *const libc::c_char),
                quote(*argv.offset(1 as libc::c_int as isize)),
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
                    b"%s: %s\0" as *const u8 as *const libc::c_char,
                    gettext(b"invalid number\0" as *const u8 as *const libc::c_char),
                    quote(*argv.offset(1 as libc::c_int as isize)),
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
                    b"%s: %s\0" as *const u8 as *const libc::c_char,
                    gettext(b"invalid number\0" as *const u8 as *const libc::c_char),
                    quote(*argv.offset(1 as libc::c_int as isize)),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
    }
    from_start = t_from_start;
    count_lines = t_count_lines;
    forever = t_forever;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn parse_options(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut n_units: *mut uintmax_t,
    mut header_mode: *mut header_mode,
    mut sleep_interval: *mut libc::c_double,
) {
    let mut c: libc::c_int = 0;
    loop {
        c = getopt_long(
            argc,
            argv,
            b"c:n:fFqs:vz0123456789\0" as *const u8 as *const libc::c_char,
            long_options.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        let mut current_block_33: u64;
        match c {
            70 => {
                forever = 1 as libc::c_int != 0;
                follow_mode = Follow_name;
                reopen_inaccessible_files = 1 as libc::c_int != 0;
                current_block_33 = 4567019141635105728;
            }
            99 | 110 => {
                count_lines = c == 'n' as i32;
                if *optarg as libc::c_int == '+' as i32 {
                    from_start = 1 as libc::c_int != 0;
                } else if *optarg as libc::c_int == '-' as i32 {
                    optarg = optarg.offset(1);
                    optarg;
                }
                *n_units = xdectoumax(
                    optarg,
                    0 as libc::c_int as uintmax_t,
                    18446744073709551615 as libc::c_ulong,
                    b"bkKmMGTPEZYRQ0\0" as *const u8 as *const libc::c_char,
                    if count_lines as libc::c_int != 0 {
                        gettext(
                            b"invalid number of lines\0" as *const u8
                                as *const libc::c_char,
                        )
                    } else {
                        gettext(
                            b"invalid number of bytes\0" as *const u8
                                as *const libc::c_char,
                        )
                    },
                    0 as libc::c_int,
                );
                current_block_33 = 4567019141635105728;
            }
            102 | 260 => {
                forever = 1 as libc::c_int != 0;
                if optarg.is_null() {
                    follow_mode = Follow_descriptor;
                } else {
                    follow_mode = follow_mode_map[__xargmatch_internal(
                        b"--follow\0" as *const u8 as *const libc::c_char,
                        optarg,
                        follow_mode_string.as_ptr(),
                        follow_mode_map.as_ptr() as *const libc::c_void,
                        ::core::mem::size_of::<Follow_mode>() as libc::c_ulong,
                        argmatch_die,
                        1 as libc::c_int != 0,
                    ) as usize];
                }
                current_block_33 = 4567019141635105728;
            }
            256 => {
                reopen_inaccessible_files = 1 as libc::c_int != 0;
                current_block_33 = 4567019141635105728;
            }
            257 => {
                max_n_unchanged_stats_between_opens = xdectoumax(
                    optarg,
                    0 as libc::c_int as uintmax_t,
                    18446744073709551615 as libc::c_ulong,
                    b"\0" as *const u8 as *const libc::c_char,
                    gettext(
                        b"invalid maximum number of unchanged stats between opens\0"
                            as *const u8 as *const libc::c_char,
                    ),
                    0 as libc::c_int,
                );
                current_block_33 = 4567019141635105728;
            }
            261 => {
                disable_inotify = 1 as libc::c_int != 0;
                current_block_33 = 4567019141635105728;
            }
            258 => {
                if nbpids as libc::c_long == pids_alloc {
                    pids = xpalloc(
                        pids as *mut libc::c_void,
                        &mut pids_alloc,
                        1 as libc::c_int as idx_t,
                        if (2147483647 as libc::c_int as libc::c_long)
                            < 9223372036854775807 as libc::c_long
                        {
                            2147483647 as libc::c_int as libc::c_long
                        } else {
                            9223372036854775807 as libc::c_long
                        },
                        ::core::mem::size_of::<pid_t>() as libc::c_ulong as idx_t,
                    ) as *mut pid_t;
                }
                let fresh6 = nbpids;
                nbpids = nbpids + 1;
                *pids
                    .offset(
                        fresh6 as isize,
                    ) = xdectoumax(
                    optarg,
                    0 as libc::c_int as uintmax_t,
                    (if (0 as libc::c_int) < -(1 as libc::c_int) {
                        -(1 as libc::c_int)
                    } else {
                        (((1 as libc::c_int)
                            << (::core::mem::size_of::<pid_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                    }) as uintmax_t,
                    b"\0" as *const u8 as *const libc::c_char,
                    gettext(b"invalid PID\0" as *const u8 as *const libc::c_char),
                    0 as libc::c_int,
                ) as pid_t;
                current_block_33 = 4567019141635105728;
            }
            259 => {
                presume_input_pipe = 1 as libc::c_int != 0;
                current_block_33 = 4567019141635105728;
            }
            113 => {
                *header_mode = never;
                current_block_33 = 4567019141635105728;
            }
            115 => {
                let mut s: libc::c_double = 0.;
                if !(xstrtod(
                    optarg,
                    0 as *mut *const libc::c_char,
                    &mut s,
                    Some(
                        cl_strtod
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                *mut *mut libc::c_char,
                            ) -> libc::c_double,
                    ),
                ) as libc::c_int != 0 && 0 as libc::c_int as libc::c_double <= s)
                {
                    if 0 != 0 {
                        error(
                            1 as libc::c_int,
                            0 as libc::c_int,
                            gettext(
                                b"invalid number of seconds: %s\0" as *const u8
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
                                    b"invalid number of seconds: %s\0" as *const u8
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
                                    b"invalid number of seconds: %s\0" as *const u8
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
                *sleep_interval = s;
                current_block_33 = 4567019141635105728;
            }
            118 => {
                *header_mode = always;
                current_block_33 = 4567019141635105728;
            }
            122 => {
                line_end = '\0' as i32 as libc::c_char;
                current_block_33 = 4567019141635105728;
            }
            -2 => {
                usage(0 as libc::c_int);
                current_block_33 = 4567019141635105728;
            }
            -3 => {
                version_etc(
                    stdout,
                    b"tail\0" as *const u8 as *const libc::c_char,
                    b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                    Version,
                    proper_name_lite(
                        b"Paul Rubin\0" as *const u8 as *const libc::c_char,
                        b"Paul Rubin\0" as *const u8 as *const libc::c_char,
                    ),
                    proper_name_lite(
                        b"David MacKenzie\0" as *const u8 as *const libc::c_char,
                        b"David MacKenzie\0" as *const u8 as *const libc::c_char,
                    ),
                    proper_name_lite(
                        b"Ian Lance Taylor\0" as *const u8 as *const libc::c_char,
                        b"Ian Lance Taylor\0" as *const u8 as *const libc::c_char,
                    ),
                    proper_name_lite(
                        b"Jim Meyering\0" as *const u8 as *const libc::c_char,
                        b"Jim Meyering\0" as *const u8 as *const libc::c_char,
                    ),
                    0 as *mut libc::c_void as *mut libc::c_char,
                );
                exit(0 as libc::c_int);
            }
            48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                if 0 != 0 {
                    error(
                        1 as libc::c_int,
                        0 as libc::c_int,
                        gettext(
                            b"option used in invalid context -- %c\0" as *const u8
                                as *const libc::c_char,
                        ),
                        c,
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
                                b"option used in invalid context -- %c\0" as *const u8
                                    as *const libc::c_char,
                            ),
                            c,
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
                                b"option used in invalid context -- %c\0" as *const u8
                                    as *const libc::c_char,
                            ),
                            c,
                        );
                        if __errstatus != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                        
                    });
                };
                current_block_33 = 17156019370052222107;
            }
            _ => {
                current_block_33 = 17156019370052222107;
            }
        }
        match current_block_33 {
            17156019370052222107 => {
                usage(1 as libc::c_int);
            }
            _ => {}
        }
    }
    if reopen_inaccessible_files {
        if !forever {
            reopen_inaccessible_files = 0 as libc::c_int != 0;
            if 0 != 0 {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"warning: --retry ignored; --retry is useful only when following\0"
                            as *const u8 as *const libc::c_char,
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
                            b"warning: --retry ignored; --retry is useful only when following\0"
                                as *const u8 as *const libc::c_char,
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
                            b"warning: --retry ignored; --retry is useful only when following\0"
                                as *const u8 as *const libc::c_char,
                        ),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        } else if follow_mode as libc::c_uint
            == Follow_descriptor as libc::c_int as libc::c_uint
        {
            if 0 != 0 {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"warning: --retry only effective for the initial open\0"
                            as *const u8 as *const libc::c_char,
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
                            b"warning: --retry only effective for the initial open\0"
                                as *const u8 as *const libc::c_char,
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
                            b"warning: --retry only effective for the initial open\0"
                                as *const u8 as *const libc::c_char,
                        ),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
    }
    if nbpids != 0 && !forever {
        if 0 != 0 {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"warning: PID ignored; --pid=PID is useful only when following\0"
                        as *const u8 as *const libc::c_char,
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
                        b"warning: PID ignored; --pid=PID is useful only when following\0"
                            as *const u8 as *const libc::c_char,
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
                        b"warning: PID ignored; --pid=PID is useful only when following\0"
                            as *const u8 as *const libc::c_char,
                    ),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
    } else if nbpids != 0
        && kill(*pids.offset(0 as libc::c_int as isize), 0 as libc::c_int)
            != 0 as libc::c_int && *__errno_location() == 38 as libc::c_int
    {
        if 0 != 0 {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"warning: --pid=PID is not supported on this system\0" as *const u8
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
                        b"warning: --pid=PID is not supported on this system\0"
                            as *const u8 as *const libc::c_char,
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
                        b"warning: --pid=PID is not supported on this system\0"
                            as *const u8 as *const libc::c_char,
                    ),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
        nbpids = 0 as libc::c_int;
        free(pids as *mut libc::c_void);
    }
}
unsafe extern "C" fn ignore_fifo_and_pipe(
    mut f: *mut File_spec,
    mut n_files: size_t,
) -> size_t {
    let mut n_viable: size_t = 0 as libc::c_int as size_t;
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < n_files {
        let mut is_a_fifo_or_pipe: bool = strcmp(
            (*f.offset(i as isize)).name,
            b"-\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int && !(*f.offset(i as isize)).ignore
            && 0 as libc::c_int <= (*f.offset(i as isize)).fd
            && ((*f.offset(i as isize)).mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o10000 as libc::c_int as libc::c_uint
                || 1 as libc::c_int != 1 as libc::c_int
                    && isapipe((*f.offset(i as isize)).fd) != 0);
        if is_a_fifo_or_pipe {
            (*f.offset(i as isize)).fd = -(1 as libc::c_int);
            (*f.offset(i as isize)).ignore = 1 as libc::c_int != 0;
        } else {
            n_viable = n_viable.wrapping_add(1);
            n_viable;
        }
        i = i.wrapping_add(1);
        i;
    }
    return n_viable;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut header_mode: header_mode = multiple_files;
    let mut ok: bool = 1 as libc::c_int != 0;
    let mut n_units: uintmax_t = 10 as libc::c_int as uintmax_t;
    let mut n_files: size_t = 0;
    let mut file: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut F: *mut File_spec = 0 as *mut File_spec;
    let mut i: size_t = 0;
    let mut obsolete_option: bool = false;
    let mut sleep_interval: libc::c_double = 1.0f64;
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    page_size = getpagesize() as idx_t;
    have_read_stdin = 0 as libc::c_int != 0;
    count_lines = 1 as libc::c_int != 0;
    print_headers = 0 as libc::c_int != 0;
    from_start = print_headers;
    forever = from_start;
    line_end = '\n' as i32 as libc::c_char;
    obsolete_option = parse_obsolete_option(argc, argv, &mut n_units);
    argc -= obsolete_option as libc::c_int;
    argv = argv.offset(obsolete_option as libc::c_int as isize);
    parse_options(argc, argv, &mut n_units, &mut header_mode, &mut sleep_interval);
    if from_start {
        if n_units != 0 {
            n_units = n_units.wrapping_sub(1);
            n_units;
        }
    }
    if optind < argc {
        n_files = (argc - optind) as size_t;
        file = argv.offset(optind as isize);
    } else {
        static mut dummy_stdin: *mut libc::c_char = b"-\0" as *const u8
            as *const libc::c_char as *mut libc::c_char;
        n_files = 1 as libc::c_int as size_t;
        file = &mut dummy_stdin;
    }
    let mut found_hyphen: bool = 0 as libc::c_int != 0;
    i = 0 as libc::c_int as size_t;
    while i < n_files {
        if strcmp(*file.offset(i as isize), b"-\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            found_hyphen = 1 as libc::c_int != 0;
        }
        i = i.wrapping_add(1);
        i;
    }
    if found_hyphen as libc::c_int != 0
        && follow_mode as libc::c_uint == Follow_name as libc::c_int as libc::c_uint
    {
        if 0 != 0 {
            error(
                1 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"cannot follow %s by name\0" as *const u8 as *const libc::c_char,
                ),
                quotearg_style(
                    shell_escape_always_quoting_style,
                    b"-\0" as *const u8 as *const libc::c_char,
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
                        b"cannot follow %s by name\0" as *const u8 as *const libc::c_char,
                    ),
                    quotearg_style(
                        shell_escape_always_quoting_style,
                        b"-\0" as *const u8 as *const libc::c_char,
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
                        b"cannot follow %s by name\0" as *const u8 as *const libc::c_char,
                    ),
                    quotearg_style(
                        shell_escape_always_quoting_style,
                        b"-\0" as *const u8 as *const libc::c_char,
                    ),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
    }
    if forever as libc::c_int != 0 && found_hyphen as libc::c_int != 0 {
        let mut in_stat: stat = stat {
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
        let mut blocking_stdin: bool = false;
        blocking_stdin = nbpids == 0
            && follow_mode as libc::c_uint
                == Follow_descriptor as libc::c_int as libc::c_uint
            && n_files == 1 as libc::c_int as libc::c_ulong
            && fstat(0 as libc::c_int, &mut in_stat) == 0
            && !(in_stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o100000 as libc::c_int as libc::c_uint);
        if !blocking_stdin && isatty(0 as libc::c_int) != 0 {
            if 0 != 0 {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"warning: following standard input indefinitely is ineffective\0"
                            as *const u8 as *const libc::c_char,
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
                            b"warning: following standard input indefinitely is ineffective\0"
                                as *const u8 as *const libc::c_char,
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
                            b"warning: following standard input indefinitely is ineffective\0"
                                as *const u8 as *const libc::c_char,
                        ),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
    }
    if n_units == 0 && !forever && !from_start {
        return 0 as libc::c_int;
    }
    F = xnmalloc(n_files, ::core::mem::size_of::<File_spec>() as libc::c_ulong)
        as *mut File_spec;
    i = 0 as libc::c_int as size_t;
    while i < n_files {
        let ref mut fresh7 = (*F.offset(i as isize)).name;
        *fresh7 = *file.offset(i as isize);
        i = i.wrapping_add(1);
        i;
    }
    if header_mode as libc::c_uint == always as libc::c_int as libc::c_uint
        || header_mode as libc::c_uint == multiple_files as libc::c_int as libc::c_uint
            && n_files > 1 as libc::c_int as libc::c_ulong
    {
        print_headers = 1 as libc::c_int != 0;
    }
    xset_binary_mode(1 as libc::c_int, 0 as libc::c_int);
    i = 0 as libc::c_int as size_t;
    while i < n_files {
        ok = (ok as libc::c_int
            & tail_file(&mut *F.offset(i as isize), n_units) as libc::c_int) != 0;
        i = i.wrapping_add(1);
        i;
    }
    if forever as libc::c_int != 0 && ignore_fifo_and_pipe(F, n_files) != 0 {
        let mut out_stat: stat = stat {
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
        if fstat(1 as libc::c_int, &mut out_stat) < 0 as libc::c_int {
            if 0 != 0 {
                error(
                    1 as libc::c_int,
                    *__errno_location(),
                    gettext(b"standard output\0" as *const u8 as *const libc::c_char),
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
                        gettext(b"standard output\0" as *const u8 as *const libc::c_char),
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
                        gettext(b"standard output\0" as *const u8 as *const libc::c_char),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
        monitor_output = out_stat.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o10000 as libc::c_int as libc::c_uint
            || 1 as libc::c_int != 1 as libc::c_int && isapipe(1 as libc::c_int) != 0;
        if !disable_inotify
            && (tailable_stdin(F, n_files) as libc::c_int != 0
                || any_remote_file(F, n_files) as libc::c_int != 0
                || !any_non_remote_file(F, n_files)
                || any_symlinks(F, n_files) as libc::c_int != 0
                || any_non_regular_fifo(F, n_files) as libc::c_int != 0
                || !ok
                    && follow_mode as libc::c_uint
                        == Follow_descriptor as libc::c_int as libc::c_uint)
        {
            disable_inotify = 1 as libc::c_int != 0;
        }
        if !disable_inotify {
            let mut wd: libc::c_int = inotify_init();
            if 0 as libc::c_int <= wd {
                if fflush_unlocked(stdout) != 0 as libc::c_int {
                    write_error();
                }
                let mut ht: *mut Hash_table = 0 as *mut Hash_table;
                tail_forever_inotify(wd, F, n_files, sleep_interval, &mut ht);
                hash_free(ht);
                close(wd);
                *__errno_location() = 0 as libc::c_int;
            }
            if 0 != 0 {
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    gettext(
                        b"inotify cannot be used, reverting to polling\0" as *const u8
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
                        *__errno_location(),
                        gettext(
                            b"inotify cannot be used, reverting to polling\0"
                                as *const u8 as *const libc::c_char,
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
                        gettext(
                            b"inotify cannot be used, reverting to polling\0"
                                as *const u8 as *const libc::c_char,
                        ),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
        disable_inotify = 1 as libc::c_int != 0;
        tail_forever(F, n_files, sleep_interval);
    }
    if have_read_stdin as libc::c_int != 0 && close(0 as libc::c_int) < 0 as libc::c_int
    {
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
    exit(if ok as libc::c_int != 0 { 0 as libc::c_int } else { 1 as libc::c_int });
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
