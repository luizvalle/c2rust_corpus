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
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
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
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn __errno_location() -> *mut libc::c_int;
    fn strsignal(__sig: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    static mut Version: *const libc::c_char;
    fn strtoimax(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
    ) -> intmax_t;
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
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn sig2str(_: libc::c_int, _: *mut libc::c_char) -> libc::c_int;
    fn operand2sig(operand: *const libc::c_char) -> libc::c_int;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type __intmax_t = libc::c_long;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type intmax_t = __intmax_t;
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
static mut short_options: [libc::c_char; 113] = unsafe {
    *::core::mem::transmute::<
        &[u8; 113],
        &[libc::c_char; 113],
    >(
        b"0::1::2::3::4::5::6::7::8::9::A::B::C::D::E::F::G::H::I::J::K::M::N::O::P::Q::R::S::T::U::V::W::X::Y::Z::Lln:s:t\0",
    )
};
static mut long_options: [option; 6] = [
    {
        let mut init = option {
            name: b"list\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'l' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"signal\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 's' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"table\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 't' as i32,
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
                b"Usage: %s [-s SIGNAL | -SIGNAL] PID...\n  or:  %s -l [SIGNAL]...\n  or:  %s -t [SIGNAL]...\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            program_name,
            program_name,
            program_name,
        );
        fputs_unlocked(
            gettext(
                b"Send signals to processes, or list signals.\n\0" as *const u8
                    as *const libc::c_char,
            ),
            stdout,
        );
        emit_mandatory_arg_note();
        fputs_unlocked(
            gettext(
                b"  -s, --signal=SIGNAL, -SIGNAL\n                   specify the name or number of the signal to be sent\n  -l, --list       list signal names, or convert signal names to/from numbers\n  -t, --table      print a table of signal information\n\0"
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
                b"\nSIGNAL may be a signal name like 'HUP', or a signal number like '1',\nor the exit status of a process terminated by a signal.\nPID is an integer; if negative it identifies a process group.\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        printf(
            gettext(
                b"\nYour shell may have its own version of %s, which usually supersedes\nthe version described here.  Please refer to your shell's documentation\nfor details about the options it supports.\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            b"kill\0" as *const u8 as *const libc::c_char,
        );
        emit_ancillary_info(b"kill\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
}
unsafe extern "C" fn print_table_row(
    mut num_width: libc::c_int,
    mut signum: libc::c_int,
    mut name_width: libc::c_int,
    mut signame: *const libc::c_char,
) {
    let mut description: *const libc::c_char = strsignal(signum);
    printf(
        b"%*d %-*s %s\n\0" as *const u8 as *const libc::c_char,
        num_width,
        signum,
        name_width,
        signame,
        if !description.is_null() {
            description
        } else {
            b"?\0" as *const u8 as *const libc::c_char
        },
    );
}
unsafe extern "C" fn list_signals(
    mut table: bool,
    mut argv: *const *mut libc::c_char,
) -> libc::c_int {
    let mut signum: libc::c_int = 0;
    let mut status: libc::c_int = 0 as libc::c_int;
    let mut signame: [libc::c_char; 19] = [0; 19];
    if table {
        let mut name_width: libc::c_int = 0 as libc::c_int;
        let mut num_width: libc::c_int = 1 as libc::c_int;
        signum = 1 as libc::c_int;
        while signum
            <= (64 as libc::c_int + 1 as libc::c_int - 1 as libc::c_int)
                / 10 as libc::c_int
        {
            num_width += 1;
            num_width;
            signum *= 10 as libc::c_int;
        }
        signum = 1 as libc::c_int;
        while signum <= 64 as libc::c_int + 1 as libc::c_int - 1 as libc::c_int {
            if sig2str(signum, signame.as_mut_ptr()) == 0 as libc::c_int {
                let mut len: idx_t = strlen(signame.as_mut_ptr()) as idx_t;
                if (name_width as libc::c_long) < len {
                    name_width = len as libc::c_int;
                }
            }
            signum += 1;
            signum;
        }
        if !argv.is_null() {
            while !(*argv).is_null() {
                signum = operand2sig(*argv);
                if signum < 0 as libc::c_int {
                    status = 1 as libc::c_int;
                } else {
                    if sig2str(signum, signame.as_mut_ptr()) != 0 as libc::c_int {
                        snprintf(
                            signame.as_mut_ptr(),
                            ::core::mem::size_of::<[libc::c_char; 19]>()
                                as libc::c_ulong,
                            b"SIG%d\0" as *const u8 as *const libc::c_char,
                            signum,
                        );
                    }
                    print_table_row(num_width, signum, name_width, signame.as_mut_ptr());
                }
                argv = argv.offset(1);
                argv;
            }
        } else {
            signum = 1 as libc::c_int;
            while signum <= 64 as libc::c_int + 1 as libc::c_int - 1 as libc::c_int {
                if sig2str(signum, signame.as_mut_ptr()) == 0 as libc::c_int {
                    print_table_row(num_width, signum, name_width, signame.as_mut_ptr());
                }
                signum += 1;
                signum;
            }
        }
    } else if !argv.is_null() {
        while !(*argv).is_null() {
            signum = operand2sig(*argv);
            if signum < 0 as libc::c_int {
                status = 1 as libc::c_int;
            } else if (**argv as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
                <= 9 as libc::c_int as libc::c_uint
            {
                if sig2str(signum, signame.as_mut_ptr()) == 0 as libc::c_int {
                    puts(signame.as_mut_ptr());
                } else {
                    printf(b"%d\n\0" as *const u8 as *const libc::c_char, signum);
                }
            } else {
                printf(b"%d\n\0" as *const u8 as *const libc::c_char, signum);
            }
            argv = argv.offset(1);
            argv;
        }
    } else {
        signum = 1 as libc::c_int;
        while signum <= 64 as libc::c_int + 1 as libc::c_int - 1 as libc::c_int {
            if sig2str(signum, signame.as_mut_ptr()) == 0 as libc::c_int {
                puts(signame.as_mut_ptr());
            }
            signum += 1;
            signum;
        }
    }
    return status;
}
unsafe extern "C" fn send_signals(
    mut signum: libc::c_int,
    mut argv: *const *mut libc::c_char,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let mut arg: *const libc::c_char = *argv;
    loop {
        let mut endp: *mut libc::c_char = 0 as *mut libc::c_char;
        *__errno_location() = 0 as libc::c_int;
        let mut n: intmax_t = strtoimax(arg, &mut endp, 10 as libc::c_int);
        let mut pid: pid_t = 0;
        if *__errno_location() == 34 as libc::c_int
            || {
                let (fresh0, fresh1) = n.overflowing_add((0 as libc::c_int).into());
                *(&mut pid as *mut pid_t) = fresh0 as i32;
                fresh1 as libc::c_int != 0
            } || arg == endp as *const libc::c_char || *endp as libc::c_int != 0
        {
            if 0 != 0 {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"%s: invalid process id\0" as *const u8 as *const libc::c_char,
                    ),
                    quote(arg),
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
                            b"%s: invalid process id\0" as *const u8
                                as *const libc::c_char,
                        ),
                        quote(arg),
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
                            b"%s: invalid process id\0" as *const u8
                                as *const libc::c_char,
                        ),
                        quote(arg),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
            status = 1 as libc::c_int;
        } else if kill(pid, signum) != 0 as libc::c_int {
            if *__errno_location() == 22 as libc::c_int {
                if 0 != 0 {
                    error(
                        0 as libc::c_int,
                        *__errno_location(),
                        b"%d\0" as *const u8 as *const libc::c_char,
                        signum,
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
                            b"%d\0" as *const u8 as *const libc::c_char,
                            signum,
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
                            b"%d\0" as *const u8 as *const libc::c_char,
                            signum,
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
                        *__errno_location(),
                        b"%s\0" as *const u8 as *const libc::c_char,
                        quote(arg),
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
                            quote(arg),
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
                            quote(arg),
                        );
                        if __errstatus != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                        
                    });
                };
            }
            status = 1 as libc::c_int;
        }
        argv = argv.offset(1);
        arg = *argv;
        if arg.is_null() {
            break;
        }
    }
    return status;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut optc: libc::c_int = 0;
    let mut list: bool = 0 as libc::c_int != 0;
    let mut table: bool = 0 as libc::c_int != 0;
    let mut signum: libc::c_int = -(1 as libc::c_int);
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
            short_options.as_ptr(),
            long_options.as_ptr(),
            0 as *mut libc::c_int,
        );
        if !(optc != -(1 as libc::c_int)) {
            break;
        }
        match optc {
            48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                if optind != 2 as libc::c_int {
                    optind -= 1;
                    optind;
                    break;
                } else {
                    current_block = 15203473443888960181;
                }
            }
            65 | 66 | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 77 | 78 | 79 | 80 | 81
            | 82 | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 => {
                current_block = 15203473443888960181;
            }
            110 | 115 => {
                current_block = 1813457589346486104;
            }
            76 | 116 => {
                table = 1 as libc::c_int != 0;
                current_block = 14187826137497519470;
            }
            108 => {
                current_block = 14187826137497519470;
            }
            -2 => {
                usage(0 as libc::c_int);
                continue;
            }
            -3 => {
                version_etc(
                    stdout,
                    b"kill\0" as *const u8 as *const libc::c_char,
                    b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                    Version,
                    proper_name_lite(
                        b"Paul Eggert\0" as *const u8 as *const libc::c_char,
                        b"Paul Eggert\0" as *const u8 as *const libc::c_char,
                    ),
                    0 as *mut libc::c_void as *mut libc::c_char,
                );
                exit(0 as libc::c_int);
            }
            _ => {
                usage(1 as libc::c_int);
                continue;
            }
        }
        match current_block {
            15203473443888960181 => {
                if optarg.is_null() {
                    optarg = (*argv.offset((optind - 1 as libc::c_int) as isize))
                        .offset(
                            strlen(*argv.offset((optind - 1 as libc::c_int) as isize))
                                as isize,
                        );
                }
                if optarg
                    != (*argv.offset((optind - 1 as libc::c_int) as isize))
                        .offset(2 as libc::c_int as isize)
                {
                    if 0 != 0 {
                        error(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            gettext(
                                b"invalid option -- %c\0" as *const u8
                                    as *const libc::c_char,
                            ),
                            optc,
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
                                    b"invalid option -- %c\0" as *const u8
                                        as *const libc::c_char,
                                ),
                                optc,
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
                                    b"invalid option -- %c\0" as *const u8
                                        as *const libc::c_char,
                                ),
                                optc,
                            );
                            if __errstatus != 0 as libc::c_int {
                                unreachable!();
                            } else {};
                            
                        });
                    };
                    usage(1 as libc::c_int);
                }
                optarg = optarg.offset(-1);
                optarg;
            }
            14187826137497519470 => {
                if list {
                    if 0 != 0 {
                        error(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            gettext(
                                b"multiple -l or -t options specified\0" as *const u8
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
                                    b"multiple -l or -t options specified\0" as *const u8
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
                                    b"multiple -l or -t options specified\0" as *const u8
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
                list = 1 as libc::c_int != 0;
                continue;
            }
            _ => {}
        }
        if 0 as libc::c_int <= signum {
            if 0 != 0 {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"%s: multiple signals specified\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quote(optarg),
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
                            b"%s: multiple signals specified\0" as *const u8
                                as *const libc::c_char,
                        ),
                        quote(optarg),
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
                            b"%s: multiple signals specified\0" as *const u8
                                as *const libc::c_char,
                        ),
                        quote(optarg),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
            usage(1 as libc::c_int);
        }
        signum = operand2sig(optarg);
        if signum < 0 as libc::c_int {
            usage(1 as libc::c_int);
        }
    }
    if signum < 0 as libc::c_int {
        signum = 15 as libc::c_int;
    } else if list {
        if 0 != 0 {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"cannot combine signal with -l or -t\0" as *const u8
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
                        b"cannot combine signal with -l or -t\0" as *const u8
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
                        b"cannot combine signal with -l or -t\0" as *const u8
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
    if !list && argc <= optind {
        if 0 != 0 {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                gettext(b"no process ID specified\0" as *const u8 as *const libc::c_char),
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
                        b"no process ID specified\0" as *const u8 as *const libc::c_char,
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
                        b"no process ID specified\0" as *const u8 as *const libc::c_char,
                    ),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
        usage(1 as libc::c_int);
    }
    return if list as libc::c_int != 0 {
        list_signals(
            table,
            if optind < argc {
                argv.offset(optind as isize)
            } else {
                0 as *mut *mut libc::c_char
            },
        )
    } else {
        send_signals(signum, argv.offset(optind as isize))
    };
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
