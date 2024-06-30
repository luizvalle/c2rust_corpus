#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut, unused_imports)]
#![feature(extern_types)]
use ::rust::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __dirstream;
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
    static mut optind: libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn lstat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn rmdir(__path: *const libc::c_char) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn quotearg_style(s: quoting_style, arg: *const libc::c_char) -> *mut libc::c_char;
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
    fn openat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __oflag: libc::c_int,
        _: ...
    ) -> libc::c_int;
    fn fdopendir(__fd: libc::c_int) -> *mut DIR;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
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
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn strip_trailing_slashes(file: *mut libc::c_char) -> bool;
    fn exit(_: libc::c_int) -> !;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn prog_fprintf(fp: *mut FILE, fmt: *const libc::c_char, _: ...);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
pub type DIR = __dirstream;
pub type C2RustUnnamed = libc::c_int;
pub const DS_NONEMPTY: C2RustUnnamed = 0;
pub const DS_EMPTY: C2RustUnnamed = -1;
pub const DS_UNKNOWN: C2RustUnnamed = -2;
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
pub type C2RustUnnamed_1 = libc::c_uint;
pub const IGNORE_FAIL_ON_NON_EMPTY_OPTION: C2RustUnnamed_1 = 256;
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
unsafe extern "C" fn directory_status(
    mut fd_cwd: libc::c_int,
    mut dir: *const libc::c_char,
) -> libc::c_int {
    let mut dirp: *mut DIR = 0 as *mut DIR;
    let mut no_direntries: bool = false;
    let mut saved_errno: libc::c_int = 0;
    let mut fd: libc::c_int = openat(
        fd_cwd,
        dir,
        0 as libc::c_int | 0o40000 as libc::c_int | 0o400 as libc::c_int
            | 0o100000 as libc::c_int | 0o4000 as libc::c_int,
    );
    if fd < 0 as libc::c_int {
        return *__errno_location();
    }
    dirp = fdopendir(fd);
    if dirp.is_null() {
        saved_errno = *__errno_location();
        close(fd);
        return saved_errno;
    }
    *__errno_location() = 0 as libc::c_int;
    no_direntries = (readdir_ignoring_dot_and_dotdot(dirp)).is_null();
    saved_errno = *__errno_location();
    closedir(dirp);
    return if no_direntries as libc::c_int != 0 && saved_errno == 0 as libc::c_int {
        DS_EMPTY as libc::c_int
    } else {
        saved_errno
    };
}
#[inline]
unsafe extern "C" fn readdir_ignoring_dot_and_dotdot(
    mut dirp: *mut DIR,
) -> *const dirent {
    loop {
        let mut dp: *const dirent = readdir(dirp);
        if dp.is_null() || !dot_or_dotdot(((*dp).d_name).as_ptr()) {
            return dp;
        }
    };
}
#[inline]
unsafe extern "C" fn dot_or_dotdot(mut file_name: *const libc::c_char) -> bool {
    if *file_name.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32 {
        let mut sep: libc::c_char = *file_name
            .offset(
                ((*file_name.offset(1 as libc::c_int as isize) as libc::c_int
                    == '.' as i32) as libc::c_int + 1 as libc::c_int) as isize,
            );
        return sep == 0 || sep as libc::c_int == '/' as i32;
    } else {
        return 0 as libc::c_int != 0
    };
}
static mut remove_empty_parents: bool = false;
static mut ignore_fail_on_non_empty: bool = false;
static mut verbose: bool = false;
static mut longopts: [option; 7] = [
    {
        let mut init = option {
            name: b"ignore-fail-on-non-empty\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: IGNORE_FAIL_ON_NON_EMPTY_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"path\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'p' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"parents\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'p' as i32,
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
unsafe extern "C" fn errno_rmdir_non_empty(mut error_number: libc::c_int) -> bool {
    return error_number == 39 as libc::c_int || error_number == 17 as libc::c_int;
}
unsafe extern "C" fn errno_may_be_non_empty(mut error_number: libc::c_int) -> bool {
    match error_number {
        13 | 1 | 30 | 16 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
unsafe extern "C" fn ignorable_failure(
    mut error_number: libc::c_int,
    mut dir: *const libc::c_char,
) -> bool {
    return ignore_fail_on_non_empty as libc::c_int != 0
        && (errno_rmdir_non_empty(error_number) as libc::c_int != 0
            || errno_may_be_non_empty(error_number) as libc::c_int != 0
                && directory_status(-(100 as libc::c_int), dir)
                    == DS_NONEMPTY as libc::c_int);
}
unsafe extern "C" fn remove_parents(mut dir: *mut libc::c_char) -> bool {
    let mut slash: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ok: bool = 1 as libc::c_int != 0;
    strip_trailing_slashes(dir);
    loop {
        slash = strrchr(dir, '/' as i32);
        if slash.is_null() {
            break;
        }
        while slash > dir && *slash as libc::c_int == '/' as i32 {
            slash = slash.offset(-1);
            slash;
        }
        *slash.offset(1 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
        if verbose {
            prog_fprintf(
                stdout,
                gettext(b"removing directory, %s\0" as *const u8 as *const libc::c_char),
                quotearg_style(shell_escape_always_quoting_style, dir),
            );
        }
        ok = rmdir(dir) == 0 as libc::c_int;
        let mut rmdir_errno: libc::c_int = *__errno_location();
        if ok {
            continue;
        }
        if ignorable_failure(rmdir_errno, dir) {
            ok = 1 as libc::c_int != 0;
        } else {
            let mut error_msg: *const libc::c_char = 0 as *const libc::c_char;
            if rmdir_errno != 20 as libc::c_int {
                error_msg = b"failed to remove directory %s\0" as *const u8
                    as *const libc::c_char;
            } else {
                error_msg = b"failed to remove %s\0" as *const u8 as *const libc::c_char;
            }
            if 0 != 0 {
                error(
                    0 as libc::c_int,
                    rmdir_errno,
                    gettext(error_msg),
                    quotearg_style(shell_escape_always_quoting_style, dir),
                );
                if 0 as libc::c_int != 0 as libc::c_int {
                    unreachable!();
                } else {};
            } else {
                ({
                    let __errstatus: libc::c_int = 0 as libc::c_int;
                    error(
                        __errstatus,
                        rmdir_errno,
                        gettext(error_msg),
                        quotearg_style(shell_escape_always_quoting_style, dir),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
                ({
                    let __errstatus: libc::c_int = 0 as libc::c_int;
                    error(
                        __errstatus,
                        rmdir_errno,
                        gettext(error_msg),
                        quotearg_style(shell_escape_always_quoting_style, dir),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
        break;
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
                b"Usage: %s [OPTION]... DIRECTORY...\n\0" as *const u8
                    as *const libc::c_char,
            ),
            program_name,
        );
        fputs_unlocked(
            gettext(
                b"Remove the DIRECTORY(ies), if they are empty.\n\n\0" as *const u8
                    as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"      --ignore-fail-on-non-empty\n                    ignore each failure to remove a non-empty directory\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"  -p, --parents     remove DIRECTORY and its ancestors;\n                    e.g., 'rmdir -p a/b' is similar to 'rmdir a/b a'\n\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"  -v, --verbose     output a diagnostic for every directory processed\n\0"
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
        emit_ancillary_info(b"rmdir\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut ok: bool = 1 as libc::c_int != 0;
    let mut optc: libc::c_int = 0;
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    remove_empty_parents = 0 as libc::c_int != 0;
    loop {
        optc = getopt_long(
            argc,
            argv,
            b"pv\0" as *const u8 as *const libc::c_char,
            longopts.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(optc != -(1 as libc::c_int)) {
            break;
        }
        match optc {
            112 => {
                remove_empty_parents = 1 as libc::c_int != 0;
            }
            256 => {
                ignore_fail_on_non_empty = 1 as libc::c_int != 0;
            }
            118 => {
                verbose = 1 as libc::c_int != 0;
            }
            -2 => {
                usage(0 as libc::c_int);
            }
            -3 => {
                version_etc(
                    stdout,
                    b"rmdir\0" as *const u8 as *const libc::c_char,
                    b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                    Version,
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
    if optind == argc {
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
    while optind < argc {
        let mut dir: *mut libc::c_char = *argv.offset(optind as isize);
        if verbose {
            prog_fprintf(
                stdout,
                gettext(b"removing directory, %s\0" as *const u8 as *const libc::c_char),
                quotearg_style(shell_escape_always_quoting_style, dir),
            );
        }
        if rmdir(dir) != 0 as libc::c_int {
            let mut rmdir_errno: libc::c_int = *__errno_location();
            if !ignorable_failure(rmdir_errno, dir) {
                let mut custom_error: bool = 0 as libc::c_int != 0;
                if rmdir_errno == 20 as libc::c_int {
                    let mut last_unix_slash: *const libc::c_char = strrchr(
                        dir,
                        '/' as i32,
                    );
                    if !last_unix_slash.is_null()
                        && *last_unix_slash.offset(1 as libc::c_int as isize)
                            as libc::c_int == '\0' as i32
                    {
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
                        let mut ret: libc::c_int = stat(dir, &mut st);
                        if ret != 0 as libc::c_int
                            && *__errno_location() != 20 as libc::c_int
                            || ret == 0 as libc::c_int
                                && st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                                    == 0o40000 as libc::c_int as libc::c_uint
                        {
                            let mut dir_arg: *mut libc::c_char = xstrdup(dir);
                            strip_trailing_slashes(dir);
                            ret = lstat(dir, &mut st);
                            if ret == 0 as libc::c_int
                                && st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                                    == 0o120000 as libc::c_int as libc::c_uint
                            {
                                if 0 != 0 {
                                    error(
                                        0 as libc::c_int,
                                        0 as libc::c_int,
                                        gettext(
                                            b"failed to remove %s: Symbolic link not followed\0"
                                                as *const u8 as *const libc::c_char,
                                        ),
                                        quotearg_style(shell_escape_always_quoting_style, dir_arg),
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
                                                b"failed to remove %s: Symbolic link not followed\0"
                                                    as *const u8 as *const libc::c_char,
                                            ),
                                            quotearg_style(shell_escape_always_quoting_style, dir_arg),
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
                                                b"failed to remove %s: Symbolic link not followed\0"
                                                    as *const u8 as *const libc::c_char,
                                            ),
                                            quotearg_style(shell_escape_always_quoting_style, dir_arg),
                                        );
                                        if __errstatus != 0 as libc::c_int {
                                            unreachable!();
                                        } else {};
                                        
                                    });
                                };
                                custom_error = 1 as libc::c_int != 0;
                            }
                            free(dir_arg as *mut libc::c_void);
                        }
                    }
                }
                if !custom_error {
                    if 0 != 0 {
                        error(
                            0 as libc::c_int,
                            rmdir_errno,
                            gettext(
                                b"failed to remove %s\0" as *const u8 as *const libc::c_char,
                            ),
                            quotearg_style(shell_escape_always_quoting_style, dir),
                        );
                        if 0 as libc::c_int != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                    } else {
                        ({
                            let __errstatus: libc::c_int = 0 as libc::c_int;
                            error(
                                __errstatus,
                                rmdir_errno,
                                gettext(
                                    b"failed to remove %s\0" as *const u8 as *const libc::c_char,
                                ),
                                quotearg_style(shell_escape_always_quoting_style, dir),
                            );
                            if __errstatus != 0 as libc::c_int {
                                unreachable!();
                            } else {};
                            
                        });
                        ({
                            let __errstatus: libc::c_int = 0 as libc::c_int;
                            error(
                                __errstatus,
                                rmdir_errno,
                                gettext(
                                    b"failed to remove %s\0" as *const u8 as *const libc::c_char,
                                ),
                                quotearg_style(shell_escape_always_quoting_style, dir),
                            );
                            if __errstatus != 0 as libc::c_int {
                                unreachable!();
                            } else {};
                            
                        });
                    };
                }
                ok = 0 as libc::c_int != 0;
            }
        } else if remove_empty_parents {
            ok = (ok as libc::c_int & remove_parents(dir) as libc::c_int) as bool;
        }
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
