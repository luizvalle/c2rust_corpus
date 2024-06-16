#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut, unused_imports)]
#![feature(extern_types, label_break_value)]
#[macro_use]
extern crate c2rust_bitfields;
use ::rust::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type hash_table;
    pub type heap;
    pub type randread_source;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn toupper(_: libc::c_int) -> libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> libc::c_int;
    fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn strtold(_: *const libc::c_char, _: *mut *mut libc::c_char) -> f128::f128;
    fn getrlimit(__resource: __rlimit_resource_t, __rlimits: *mut rlimit) -> libc::c_int;
    fn sigismember(__set: *const sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    fn euidaccess(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    fn execlp(
        __file: *const libc::c_char,
        __arg: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn _exit(_: libc::c_int) -> !;
    fn fork() -> __pid_t;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn raise(__sig: libc::c_int) -> libc::c_int;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn rpl_pipe2(fd: *mut libc::c_int, flags: libc::c_int) -> libc::c_int;
    fn pthread_sigmask(
        __how: libc::c_int,
        __newmask: *const __sigset_t,
        __oldmask: *mut __sigset_t,
    ) -> libc::c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
    ) -> __pid_t;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn ftruncate(__fd: libc::c_int, __length: __off_t) -> libc::c_int;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush_unlocked(__stream: *mut FILE) -> libc::c_int;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn stpcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    static mut Version: *const libc::c_char;
    static mut exit_failure: libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn localeconv() -> *mut lconv;
    fn gettext(__msgid: *const libc::c_char) -> *mut libc::c_char;
    fn ngettext(
        __msgid1: *const libc::c_char,
        __msgid2: *const libc::c_char,
        __n: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn textdomain(__domainname: *const libc::c_char) -> *mut libc::c_char;
    fn bindtextdomain(
        __domainname: *const libc::c_char,
        __dirname: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn strxfrm(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_ulong;
    fn xalloc_die();
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xcalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn x2nrealloc(p: *mut libc::c_void, pn: *mut size_t, s: size_t) -> *mut libc::c_void;
    fn xmemdup(p: *const libc::c_void, s: size_t) -> *mut libc::c_void;
    fn xnmalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn fputc_unlocked(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn putchar_unlocked(__c: libc::c_int) -> libc::c_int;
    fn fputs_unlocked(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fread_unlocked(
        __ptr: *mut libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn fwrite_unlocked(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn clearerr_unlocked(__stream: *mut FILE);
    fn feof_unlocked(__stream: *mut FILE) -> libc::c_int;
    fn ferror_unlocked(__stream: *mut FILE) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn rpl_fclose(stream: *mut FILE) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn inttostr(_: libc::c_int, _: *mut libc::c_char) -> *mut libc::c_char;
    fn uinttostr(_: libc::c_uint, _: *mut libc::c_char) -> *mut libc::c_char;
    fn umaxtostr(_: uintmax_t, _: *mut libc::c_char) -> *mut libc::c_char;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn quotearg_n_style(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn quotearg_style(s: quoting_style, arg: *const libc::c_char) -> *mut libc::c_char;
    fn quotearg_n_style_colon(
        n: libc::c_int,
        s: quoting_style,
        arg: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
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
    fn quote_n(n: libc::c_int, arg: *const libc::c_char) -> *const libc::c_char;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    fn fadvise(fp: *mut FILE, advice: fadvice_t);
    fn filenvercmp(
        a: *const libc::c_char,
        alen: ptrdiff_t,
        b: *const libc::c_char,
        blen: ptrdiff_t,
    ) -> libc::c_int;
    fn hard_locale(category: libc::c_int) -> bool;
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
    fn heap_free(_: *mut heap);
    fn heap_alloc(
        _: Option::<
            unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
        >,
        _: size_t,
    ) -> *mut heap;
    fn heap_insert(heap: *mut heap, item: *mut libc::c_void) -> libc::c_int;
    fn heap_remove_top(heap: *mut heap) -> *mut libc::c_void;
    fn mbsnwidth(
        buf: *const libc::c_char,
        nbytes: size_t,
        flags: libc::c_int,
    ) -> libc::c_int;
    fn num_processors(query: nproc_query) -> libc::c_ulong;
    fn physmem_total() -> libc::c_double;
    fn physmem_available() -> libc::c_double;
    fn posix2_version() -> libc::c_int;
    fn randread_free(_: *mut randread_source) -> libc::c_int;
    fn randread_new(_: *const libc::c_char, _: size_t) -> *mut randread_source;
    fn randread(_: *mut randread_source, _: *mut libc::c_void, _: size_t);
    fn readtokens0_init(t: *mut Tokens);
    fn readtokens0(in_0: *mut FILE, t: *mut Tokens) -> bool;
    fn mkostemp_safer(_: *mut libc::c_char, _: libc::c_int) -> libc::c_int;
    fn strnumcmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn xmemcoll0(
        _: *const libc::c_char,
        _: size_t,
        _: *const libc::c_char,
        _: size_t,
    ) -> libc::c_int;
    fn xnanosleep(_: libc::c_double) -> libc::c_int;
    fn xstrtoumax(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
        _: *mut uintmax_t,
        _: *const libc::c_char,
    ) -> strtol_error;
    fn xstrtol_fatal(
        _: strtol_error,
        _: libc::c_int,
        _: libc::c_char,
        _: *const option,
        _: *const libc::c_char,
    );
    fn nl_langinfo(__item: nl_item) -> *mut libc::c_char;
    fn dlerror() -> *mut libc::c_char;
    fn dlsym(
        __handle: *mut libc::c_void,
        __name: *const libc::c_char,
    ) -> *mut libc::c_void;
    fn dlopen(__file: *const libc::c_char, __mode: libc::c_int) -> *mut libc::c_void;
}
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
pub type __clock_t = libc::c_long;
pub type __rlim_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_int;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type pid_t = __pid_t;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __atomic_wide_counter {
    pub __value64: libc::c_ulonglong,
    pub __value32: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_int,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub __wseq: __atomic_wide_counter,
    pub __g1_start: __atomic_wide_counter,
    pub __g_refs: [libc::c_uint; 2],
    pub __g_size: [libc::c_uint; 2],
    pub __g1_orig_size: libc::c_uint,
    pub __wrefs: libc::c_uint,
    pub __g_signals: [libc::c_uint; 2],
}
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [libc::c_char; 8],
    pub __align: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_condattr_t {
    pub __size: [libc::c_char; 8],
    pub __align: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 64],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 48],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48],
    pub __align: libc::c_longlong,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
}
pub type ptrdiff_t = libc::c_long;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type __rlimit_resource = libc::c_uint;
pub const __RLIM_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_RTTIME: __rlimit_resource = 15;
pub const __RLIMIT_RTPRIO: __rlimit_resource = 14;
pub const __RLIMIT_NICE: __rlimit_resource = 13;
pub const __RLIMIT_MSGQUEUE: __rlimit_resource = 12;
pub const __RLIMIT_SIGPENDING: __rlimit_resource = 11;
pub const __RLIMIT_LOCKS: __rlimit_resource = 10;
pub const __RLIMIT_MEMLOCK: __rlimit_resource = 8;
pub const __RLIMIT_NPROC: __rlimit_resource = 6;
pub const RLIMIT_AS: __rlimit_resource = 9;
pub const __RLIMIT_OFILE: __rlimit_resource = 7;
pub const RLIMIT_NOFILE: __rlimit_resource = 7;
pub const __RLIMIT_RSS: __rlimit_resource = 5;
pub const RLIMIT_CORE: __rlimit_resource = 4;
pub const RLIMIT_STACK: __rlimit_resource = 3;
pub const RLIMIT_DATA: __rlimit_resource = 2;
pub const RLIMIT_FSIZE: __rlimit_resource = 1;
pub const RLIMIT_CPU: __rlimit_resource = 0;
pub type rlim_t = __rlim_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rlimit {
    pub rlim_cur: rlim_t,
    pub rlim_max: rlim_t,
}
pub type __rlimit_resource_t = __rlimit_resource;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub _pad: [libc::c_int; 28],
    pub _kill: C2RustUnnamed_10,
    pub _timer: C2RustUnnamed_9,
    pub _rt: C2RustUnnamed_8,
    pub _sigchld: C2RustUnnamed_7,
    pub _sigfault: C2RustUnnamed_4,
    pub _sigpoll: C2RustUnnamed_3,
    pub _sigsys: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub _addr_bnd: C2RustUnnamed_6,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_11,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option::<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option::<
        unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
    >,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lconv {
    pub decimal_point: *mut libc::c_char,
    pub thousands_sep: *mut libc::c_char,
    pub grouping: *mut libc::c_char,
    pub int_curr_symbol: *mut libc::c_char,
    pub currency_symbol: *mut libc::c_char,
    pub mon_decimal_point: *mut libc::c_char,
    pub mon_thousands_sep: *mut libc::c_char,
    pub mon_grouping: *mut libc::c_char,
    pub positive_sign: *mut libc::c_char,
    pub negative_sign: *mut libc::c_char,
    pub int_frac_digits: libc::c_char,
    pub frac_digits: libc::c_char,
    pub p_cs_precedes: libc::c_char,
    pub p_sep_by_space: libc::c_char,
    pub n_cs_precedes: libc::c_char,
    pub n_sep_by_space: libc::c_char,
    pub p_sign_posn: libc::c_char,
    pub n_sign_posn: libc::c_char,
    pub int_p_cs_precedes: libc::c_char,
    pub int_p_sep_by_space: libc::c_char,
    pub int_n_cs_precedes: libc::c_char,
    pub int_n_sep_by_space: libc::c_char,
    pub int_p_sign_posn: libc::c_char,
    pub int_n_sign_posn: libc::c_char,
}
pub const PLURAL_REDUCER: C2RustUnnamed_12 = 1000000;
pub type C2RustUnnamed_12 = libc::c_uint;
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct obstack {
    pub chunk_size: size_t,
    pub chunk: *mut _obstack_chunk,
    pub object_base: *mut libc::c_char,
    pub next_free: *mut libc::c_char,
    pub chunk_limit: *mut libc::c_char,
    pub temp: C2RustUnnamed_15,
    pub alignment_mask: size_t,
    pub chunkfun: C2RustUnnamed_14,
    pub freefun: C2RustUnnamed_13,
    pub extra_arg: *mut libc::c_void,
    #[bitfield(name = "use_extra_arg", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "maybe_empty_object", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "alloc_failed", ty = "libc::c_uint", bits = "2..=2")]
    pub use_extra_arg_maybe_empty_object_alloc_failed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_13 {
    pub plain: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub extra: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_14 {
    pub plain: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub extra: Option::<
        unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_15 {
    pub i: size_t,
    pub p: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _obstack_chunk {
    pub limit: *mut libc::c_char,
    pub prev: *mut _obstack_chunk,
    pub contents: [libc::c_char; 0],
}
pub type C2RustUnnamed_16 = libc::c_int;
pub const GETOPT_VERSION_CHAR: C2RustUnnamed_16 = -3;
pub const GETOPT_HELP_CHAR: C2RustUnnamed_16 = -2;
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
pub type fadvice_t = libc::c_uint;
pub const FADVISE_RANDOM: fadvice_t = 1;
pub const FADVISE_WILLNEED: fadvice_t = 3;
pub const FADVISE_DONTNEED: fadvice_t = 4;
pub const FADVISE_NOREUSE: fadvice_t = 5;
pub const FADVISE_SEQUENTIAL: fadvice_t = 2;
pub const FADVISE_NORMAL: fadvice_t = 0;
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
pub type nproc_query = libc::c_uint;
pub const NPROC_CURRENT_OVERRIDABLE: nproc_query = 2;
pub const NPROC_CURRENT: nproc_query = 1;
pub const NPROC_ALL: nproc_query = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tokens {
    pub n_tok: size_t,
    pub tok: *mut *mut libc::c_char,
    pub tok_len: *mut size_t,
    pub o_data: obstack,
    pub o_tok: obstack,
    pub o_tok_len: obstack,
}
pub type strtol_error = libc::c_uint;
pub const LONGINT_INVALID: strtol_error = 4;
pub const LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW: strtol_error = 3;
pub const LONGINT_INVALID_SUFFIX_CHAR: strtol_error = 2;
pub const LONGINT_OVERFLOW: strtol_error = 1;
pub const LONGINT_OK: strtol_error = 0;
pub type nl_item = libc::c_int;
pub type C2RustUnnamed_17 = libc::c_uint;
pub const _NL_NUM: C2RustUnnamed_17 = 786449;
pub const _NL_NUM_LC_IDENTIFICATION: C2RustUnnamed_17 = 786448;
pub const _NL_IDENTIFICATION_CODESET: C2RustUnnamed_17 = 786447;
pub const _NL_IDENTIFICATION_CATEGORY: C2RustUnnamed_17 = 786446;
pub const _NL_IDENTIFICATION_DATE: C2RustUnnamed_17 = 786445;
pub const _NL_IDENTIFICATION_REVISION: C2RustUnnamed_17 = 786444;
pub const _NL_IDENTIFICATION_ABBREVIATION: C2RustUnnamed_17 = 786443;
pub const _NL_IDENTIFICATION_APPLICATION: C2RustUnnamed_17 = 786442;
pub const _NL_IDENTIFICATION_AUDIENCE: C2RustUnnamed_17 = 786441;
pub const _NL_IDENTIFICATION_TERRITORY: C2RustUnnamed_17 = 786440;
pub const _NL_IDENTIFICATION_LANGUAGE: C2RustUnnamed_17 = 786439;
pub const _NL_IDENTIFICATION_FAX: C2RustUnnamed_17 = 786438;
pub const _NL_IDENTIFICATION_TEL: C2RustUnnamed_17 = 786437;
pub const _NL_IDENTIFICATION_EMAIL: C2RustUnnamed_17 = 786436;
pub const _NL_IDENTIFICATION_CONTACT: C2RustUnnamed_17 = 786435;
pub const _NL_IDENTIFICATION_ADDRESS: C2RustUnnamed_17 = 786434;
pub const _NL_IDENTIFICATION_SOURCE: C2RustUnnamed_17 = 786433;
pub const _NL_IDENTIFICATION_TITLE: C2RustUnnamed_17 = 786432;
pub const _NL_NUM_LC_MEASUREMENT: C2RustUnnamed_17 = 720898;
pub const _NL_MEASUREMENT_CODESET: C2RustUnnamed_17 = 720897;
pub const _NL_MEASUREMENT_MEASUREMENT: C2RustUnnamed_17 = 720896;
pub const _NL_NUM_LC_TELEPHONE: C2RustUnnamed_17 = 655365;
pub const _NL_TELEPHONE_CODESET: C2RustUnnamed_17 = 655364;
pub const _NL_TELEPHONE_INT_PREFIX: C2RustUnnamed_17 = 655363;
pub const _NL_TELEPHONE_INT_SELECT: C2RustUnnamed_17 = 655362;
pub const _NL_TELEPHONE_TEL_DOM_FMT: C2RustUnnamed_17 = 655361;
pub const _NL_TELEPHONE_TEL_INT_FMT: C2RustUnnamed_17 = 655360;
pub const _NL_NUM_LC_ADDRESS: C2RustUnnamed_17 = 589837;
pub const _NL_ADDRESS_CODESET: C2RustUnnamed_17 = 589836;
pub const _NL_ADDRESS_LANG_LIB: C2RustUnnamed_17 = 589835;
pub const _NL_ADDRESS_LANG_TERM: C2RustUnnamed_17 = 589834;
pub const _NL_ADDRESS_LANG_AB: C2RustUnnamed_17 = 589833;
pub const _NL_ADDRESS_LANG_NAME: C2RustUnnamed_17 = 589832;
pub const _NL_ADDRESS_COUNTRY_ISBN: C2RustUnnamed_17 = 589831;
pub const _NL_ADDRESS_COUNTRY_NUM: C2RustUnnamed_17 = 589830;
pub const _NL_ADDRESS_COUNTRY_CAR: C2RustUnnamed_17 = 589829;
pub const _NL_ADDRESS_COUNTRY_AB3: C2RustUnnamed_17 = 589828;
pub const _NL_ADDRESS_COUNTRY_AB2: C2RustUnnamed_17 = 589827;
pub const _NL_ADDRESS_COUNTRY_POST: C2RustUnnamed_17 = 589826;
pub const _NL_ADDRESS_COUNTRY_NAME: C2RustUnnamed_17 = 589825;
pub const _NL_ADDRESS_POSTAL_FMT: C2RustUnnamed_17 = 589824;
pub const _NL_NUM_LC_NAME: C2RustUnnamed_17 = 524295;
pub const _NL_NAME_CODESET: C2RustUnnamed_17 = 524294;
pub const _NL_NAME_NAME_MS: C2RustUnnamed_17 = 524293;
pub const _NL_NAME_NAME_MISS: C2RustUnnamed_17 = 524292;
pub const _NL_NAME_NAME_MRS: C2RustUnnamed_17 = 524291;
pub const _NL_NAME_NAME_MR: C2RustUnnamed_17 = 524290;
pub const _NL_NAME_NAME_GEN: C2RustUnnamed_17 = 524289;
pub const _NL_NAME_NAME_FMT: C2RustUnnamed_17 = 524288;
pub const _NL_NUM_LC_PAPER: C2RustUnnamed_17 = 458755;
pub const _NL_PAPER_CODESET: C2RustUnnamed_17 = 458754;
pub const _NL_PAPER_WIDTH: C2RustUnnamed_17 = 458753;
pub const _NL_PAPER_HEIGHT: C2RustUnnamed_17 = 458752;
pub const _NL_NUM_LC_MESSAGES: C2RustUnnamed_17 = 327685;
pub const _NL_MESSAGES_CODESET: C2RustUnnamed_17 = 327684;
pub const __NOSTR: C2RustUnnamed_17 = 327683;
pub const __YESSTR: C2RustUnnamed_17 = 327682;
pub const __NOEXPR: C2RustUnnamed_17 = 327681;
pub const __YESEXPR: C2RustUnnamed_17 = 327680;
pub const _NL_NUM_LC_NUMERIC: C2RustUnnamed_17 = 65542;
pub const _NL_NUMERIC_CODESET: C2RustUnnamed_17 = 65541;
pub const _NL_NUMERIC_THOUSANDS_SEP_WC: C2RustUnnamed_17 = 65540;
pub const _NL_NUMERIC_DECIMAL_POINT_WC: C2RustUnnamed_17 = 65539;
pub const __GROUPING: C2RustUnnamed_17 = 65538;
pub const THOUSEP: C2RustUnnamed_17 = 65537;
pub const __THOUSANDS_SEP: C2RustUnnamed_17 = 65537;
pub const RADIXCHAR: C2RustUnnamed_17 = 65536;
pub const __DECIMAL_POINT: C2RustUnnamed_17 = 65536;
pub const _NL_NUM_LC_MONETARY: C2RustUnnamed_17 = 262190;
pub const _NL_MONETARY_CODESET: C2RustUnnamed_17 = 262189;
pub const _NL_MONETARY_THOUSANDS_SEP_WC: C2RustUnnamed_17 = 262188;
pub const _NL_MONETARY_DECIMAL_POINT_WC: C2RustUnnamed_17 = 262187;
pub const _NL_MONETARY_CONVERSION_RATE: C2RustUnnamed_17 = 262186;
pub const _NL_MONETARY_DUO_VALID_TO: C2RustUnnamed_17 = 262185;
pub const _NL_MONETARY_DUO_VALID_FROM: C2RustUnnamed_17 = 262184;
pub const _NL_MONETARY_UNO_VALID_TO: C2RustUnnamed_17 = 262183;
pub const _NL_MONETARY_UNO_VALID_FROM: C2RustUnnamed_17 = 262182;
pub const _NL_MONETARY_DUO_INT_N_SIGN_POSN: C2RustUnnamed_17 = 262181;
pub const _NL_MONETARY_DUO_INT_P_SIGN_POSN: C2RustUnnamed_17 = 262180;
pub const _NL_MONETARY_DUO_N_SIGN_POSN: C2RustUnnamed_17 = 262179;
pub const _NL_MONETARY_DUO_P_SIGN_POSN: C2RustUnnamed_17 = 262178;
pub const _NL_MONETARY_DUO_INT_N_SEP_BY_SPACE: C2RustUnnamed_17 = 262177;
pub const _NL_MONETARY_DUO_INT_N_CS_PRECEDES: C2RustUnnamed_17 = 262176;
pub const _NL_MONETARY_DUO_INT_P_SEP_BY_SPACE: C2RustUnnamed_17 = 262175;
pub const _NL_MONETARY_DUO_INT_P_CS_PRECEDES: C2RustUnnamed_17 = 262174;
pub const _NL_MONETARY_DUO_N_SEP_BY_SPACE: C2RustUnnamed_17 = 262173;
pub const _NL_MONETARY_DUO_N_CS_PRECEDES: C2RustUnnamed_17 = 262172;
pub const _NL_MONETARY_DUO_P_SEP_BY_SPACE: C2RustUnnamed_17 = 262171;
pub const _NL_MONETARY_DUO_P_CS_PRECEDES: C2RustUnnamed_17 = 262170;
pub const _NL_MONETARY_DUO_FRAC_DIGITS: C2RustUnnamed_17 = 262169;
pub const _NL_MONETARY_DUO_INT_FRAC_DIGITS: C2RustUnnamed_17 = 262168;
pub const _NL_MONETARY_DUO_CURRENCY_SYMBOL: C2RustUnnamed_17 = 262167;
pub const _NL_MONETARY_DUO_INT_CURR_SYMBOL: C2RustUnnamed_17 = 262166;
pub const __INT_N_SIGN_POSN: C2RustUnnamed_17 = 262165;
pub const __INT_P_SIGN_POSN: C2RustUnnamed_17 = 262164;
pub const __INT_N_SEP_BY_SPACE: C2RustUnnamed_17 = 262163;
pub const __INT_N_CS_PRECEDES: C2RustUnnamed_17 = 262162;
pub const __INT_P_SEP_BY_SPACE: C2RustUnnamed_17 = 262161;
pub const __INT_P_CS_PRECEDES: C2RustUnnamed_17 = 262160;
pub const _NL_MONETARY_CRNCYSTR: C2RustUnnamed_17 = 262159;
pub const __N_SIGN_POSN: C2RustUnnamed_17 = 262158;
pub const __P_SIGN_POSN: C2RustUnnamed_17 = 262157;
pub const __N_SEP_BY_SPACE: C2RustUnnamed_17 = 262156;
pub const __N_CS_PRECEDES: C2RustUnnamed_17 = 262155;
pub const __P_SEP_BY_SPACE: C2RustUnnamed_17 = 262154;
pub const __P_CS_PRECEDES: C2RustUnnamed_17 = 262153;
pub const __FRAC_DIGITS: C2RustUnnamed_17 = 262152;
pub const __INT_FRAC_DIGITS: C2RustUnnamed_17 = 262151;
pub const __NEGATIVE_SIGN: C2RustUnnamed_17 = 262150;
pub const __POSITIVE_SIGN: C2RustUnnamed_17 = 262149;
pub const __MON_GROUPING: C2RustUnnamed_17 = 262148;
pub const __MON_THOUSANDS_SEP: C2RustUnnamed_17 = 262147;
pub const __MON_DECIMAL_POINT: C2RustUnnamed_17 = 262146;
pub const __CURRENCY_SYMBOL: C2RustUnnamed_17 = 262145;
pub const __INT_CURR_SYMBOL: C2RustUnnamed_17 = 262144;
pub const _NL_NUM_LC_CTYPE: C2RustUnnamed_17 = 86;
pub const _NL_CTYPE_EXTRA_MAP_14: C2RustUnnamed_17 = 85;
pub const _NL_CTYPE_EXTRA_MAP_13: C2RustUnnamed_17 = 84;
pub const _NL_CTYPE_EXTRA_MAP_12: C2RustUnnamed_17 = 83;
pub const _NL_CTYPE_EXTRA_MAP_11: C2RustUnnamed_17 = 82;
pub const _NL_CTYPE_EXTRA_MAP_10: C2RustUnnamed_17 = 81;
pub const _NL_CTYPE_EXTRA_MAP_9: C2RustUnnamed_17 = 80;
pub const _NL_CTYPE_EXTRA_MAP_8: C2RustUnnamed_17 = 79;
pub const _NL_CTYPE_EXTRA_MAP_7: C2RustUnnamed_17 = 78;
pub const _NL_CTYPE_EXTRA_MAP_6: C2RustUnnamed_17 = 77;
pub const _NL_CTYPE_EXTRA_MAP_5: C2RustUnnamed_17 = 76;
pub const _NL_CTYPE_EXTRA_MAP_4: C2RustUnnamed_17 = 75;
pub const _NL_CTYPE_EXTRA_MAP_3: C2RustUnnamed_17 = 74;
pub const _NL_CTYPE_EXTRA_MAP_2: C2RustUnnamed_17 = 73;
pub const _NL_CTYPE_EXTRA_MAP_1: C2RustUnnamed_17 = 72;
pub const _NL_CTYPE_NONASCII_CASE: C2RustUnnamed_17 = 71;
pub const _NL_CTYPE_MAP_TO_NONASCII: C2RustUnnamed_17 = 70;
pub const _NL_CTYPE_TRANSLIT_IGNORE: C2RustUnnamed_17 = 69;
pub const _NL_CTYPE_TRANSLIT_IGNORE_LEN: C2RustUnnamed_17 = 68;
pub const _NL_CTYPE_TRANSLIT_DEFAULT_MISSING: C2RustUnnamed_17 = 67;
pub const _NL_CTYPE_TRANSLIT_DEFAULT_MISSING_LEN: C2RustUnnamed_17 = 66;
pub const _NL_CTYPE_TRANSLIT_TO_TBL: C2RustUnnamed_17 = 65;
pub const _NL_CTYPE_TRANSLIT_TO_IDX: C2RustUnnamed_17 = 64;
pub const _NL_CTYPE_TRANSLIT_FROM_TBL: C2RustUnnamed_17 = 63;
pub const _NL_CTYPE_TRANSLIT_FROM_IDX: C2RustUnnamed_17 = 62;
pub const _NL_CTYPE_TRANSLIT_TAB_SIZE: C2RustUnnamed_17 = 61;
pub const _NL_CTYPE_OUTDIGIT9_WC: C2RustUnnamed_17 = 60;
pub const _NL_CTYPE_OUTDIGIT8_WC: C2RustUnnamed_17 = 59;
pub const _NL_CTYPE_OUTDIGIT7_WC: C2RustUnnamed_17 = 58;
pub const _NL_CTYPE_OUTDIGIT6_WC: C2RustUnnamed_17 = 57;
pub const _NL_CTYPE_OUTDIGIT5_WC: C2RustUnnamed_17 = 56;
pub const _NL_CTYPE_OUTDIGIT4_WC: C2RustUnnamed_17 = 55;
pub const _NL_CTYPE_OUTDIGIT3_WC: C2RustUnnamed_17 = 54;
pub const _NL_CTYPE_OUTDIGIT2_WC: C2RustUnnamed_17 = 53;
pub const _NL_CTYPE_OUTDIGIT1_WC: C2RustUnnamed_17 = 52;
pub const _NL_CTYPE_OUTDIGIT0_WC: C2RustUnnamed_17 = 51;
pub const _NL_CTYPE_OUTDIGIT9_MB: C2RustUnnamed_17 = 50;
pub const _NL_CTYPE_OUTDIGIT8_MB: C2RustUnnamed_17 = 49;
pub const _NL_CTYPE_OUTDIGIT7_MB: C2RustUnnamed_17 = 48;
pub const _NL_CTYPE_OUTDIGIT6_MB: C2RustUnnamed_17 = 47;
pub const _NL_CTYPE_OUTDIGIT5_MB: C2RustUnnamed_17 = 46;
pub const _NL_CTYPE_OUTDIGIT4_MB: C2RustUnnamed_17 = 45;
pub const _NL_CTYPE_OUTDIGIT3_MB: C2RustUnnamed_17 = 44;
pub const _NL_CTYPE_OUTDIGIT2_MB: C2RustUnnamed_17 = 43;
pub const _NL_CTYPE_OUTDIGIT1_MB: C2RustUnnamed_17 = 42;
pub const _NL_CTYPE_OUTDIGIT0_MB: C2RustUnnamed_17 = 41;
pub const _NL_CTYPE_INDIGITS9_WC: C2RustUnnamed_17 = 40;
pub const _NL_CTYPE_INDIGITS8_WC: C2RustUnnamed_17 = 39;
pub const _NL_CTYPE_INDIGITS7_WC: C2RustUnnamed_17 = 38;
pub const _NL_CTYPE_INDIGITS6_WC: C2RustUnnamed_17 = 37;
pub const _NL_CTYPE_INDIGITS5_WC: C2RustUnnamed_17 = 36;
pub const _NL_CTYPE_INDIGITS4_WC: C2RustUnnamed_17 = 35;
pub const _NL_CTYPE_INDIGITS3_WC: C2RustUnnamed_17 = 34;
pub const _NL_CTYPE_INDIGITS2_WC: C2RustUnnamed_17 = 33;
pub const _NL_CTYPE_INDIGITS1_WC: C2RustUnnamed_17 = 32;
pub const _NL_CTYPE_INDIGITS0_WC: C2RustUnnamed_17 = 31;
pub const _NL_CTYPE_INDIGITS_WC_LEN: C2RustUnnamed_17 = 30;
pub const _NL_CTYPE_INDIGITS9_MB: C2RustUnnamed_17 = 29;
pub const _NL_CTYPE_INDIGITS8_MB: C2RustUnnamed_17 = 28;
pub const _NL_CTYPE_INDIGITS7_MB: C2RustUnnamed_17 = 27;
pub const _NL_CTYPE_INDIGITS6_MB: C2RustUnnamed_17 = 26;
pub const _NL_CTYPE_INDIGITS5_MB: C2RustUnnamed_17 = 25;
pub const _NL_CTYPE_INDIGITS4_MB: C2RustUnnamed_17 = 24;
pub const _NL_CTYPE_INDIGITS3_MB: C2RustUnnamed_17 = 23;
pub const _NL_CTYPE_INDIGITS2_MB: C2RustUnnamed_17 = 22;
pub const _NL_CTYPE_INDIGITS1_MB: C2RustUnnamed_17 = 21;
pub const _NL_CTYPE_INDIGITS0_MB: C2RustUnnamed_17 = 20;
pub const _NL_CTYPE_INDIGITS_MB_LEN: C2RustUnnamed_17 = 19;
pub const _NL_CTYPE_MAP_OFFSET: C2RustUnnamed_17 = 18;
pub const _NL_CTYPE_CLASS_OFFSET: C2RustUnnamed_17 = 17;
pub const _NL_CTYPE_TOLOWER32: C2RustUnnamed_17 = 16;
pub const _NL_CTYPE_TOUPPER32: C2RustUnnamed_17 = 15;
pub const CODESET: C2RustUnnamed_17 = 14;
pub const _NL_CTYPE_CODESET_NAME: C2RustUnnamed_17 = 14;
pub const _NL_CTYPE_MB_CUR_MAX: C2RustUnnamed_17 = 13;
pub const _NL_CTYPE_WIDTH: C2RustUnnamed_17 = 12;
pub const _NL_CTYPE_MAP_NAMES: C2RustUnnamed_17 = 11;
pub const _NL_CTYPE_CLASS_NAMES: C2RustUnnamed_17 = 10;
pub const _NL_CTYPE_GAP6: C2RustUnnamed_17 = 9;
pub const _NL_CTYPE_GAP5: C2RustUnnamed_17 = 8;
pub const _NL_CTYPE_GAP4: C2RustUnnamed_17 = 7;
pub const _NL_CTYPE_GAP3: C2RustUnnamed_17 = 6;
pub const _NL_CTYPE_CLASS32: C2RustUnnamed_17 = 5;
pub const _NL_CTYPE_GAP2: C2RustUnnamed_17 = 4;
pub const _NL_CTYPE_TOLOWER: C2RustUnnamed_17 = 3;
pub const _NL_CTYPE_GAP1: C2RustUnnamed_17 = 2;
pub const _NL_CTYPE_TOUPPER: C2RustUnnamed_17 = 1;
pub const _NL_CTYPE_CLASS: C2RustUnnamed_17 = 0;
pub const _NL_NUM_LC_COLLATE: C2RustUnnamed_17 = 196627;
pub const _NL_COLLATE_CODESET: C2RustUnnamed_17 = 196626;
pub const _NL_COLLATE_COLLSEQWC: C2RustUnnamed_17 = 196625;
pub const _NL_COLLATE_COLLSEQMB: C2RustUnnamed_17 = 196624;
pub const _NL_COLLATE_SYMB_EXTRAMB: C2RustUnnamed_17 = 196623;
pub const _NL_COLLATE_SYMB_TABLEMB: C2RustUnnamed_17 = 196622;
pub const _NL_COLLATE_SYMB_HASH_SIZEMB: C2RustUnnamed_17 = 196621;
pub const _NL_COLLATE_INDIRECTWC: C2RustUnnamed_17 = 196620;
pub const _NL_COLLATE_EXTRAWC: C2RustUnnamed_17 = 196619;
pub const _NL_COLLATE_WEIGHTWC: C2RustUnnamed_17 = 196618;
pub const _NL_COLLATE_TABLEWC: C2RustUnnamed_17 = 196617;
pub const _NL_COLLATE_GAP3: C2RustUnnamed_17 = 196616;
pub const _NL_COLLATE_GAP2: C2RustUnnamed_17 = 196615;
pub const _NL_COLLATE_GAP1: C2RustUnnamed_17 = 196614;
pub const _NL_COLLATE_INDIRECTMB: C2RustUnnamed_17 = 196613;
pub const _NL_COLLATE_EXTRAMB: C2RustUnnamed_17 = 196612;
pub const _NL_COLLATE_WEIGHTMB: C2RustUnnamed_17 = 196611;
pub const _NL_COLLATE_TABLEMB: C2RustUnnamed_17 = 196610;
pub const _NL_COLLATE_RULESETS: C2RustUnnamed_17 = 196609;
pub const _NL_COLLATE_NRULES: C2RustUnnamed_17 = 196608;
pub const _NL_NUM_LC_TIME: C2RustUnnamed_17 = 131231;
pub const _NL_WABALTMON_12: C2RustUnnamed_17 = 131230;
pub const _NL_WABALTMON_11: C2RustUnnamed_17 = 131229;
pub const _NL_WABALTMON_10: C2RustUnnamed_17 = 131228;
pub const _NL_WABALTMON_9: C2RustUnnamed_17 = 131227;
pub const _NL_WABALTMON_8: C2RustUnnamed_17 = 131226;
pub const _NL_WABALTMON_7: C2RustUnnamed_17 = 131225;
pub const _NL_WABALTMON_6: C2RustUnnamed_17 = 131224;
pub const _NL_WABALTMON_5: C2RustUnnamed_17 = 131223;
pub const _NL_WABALTMON_4: C2RustUnnamed_17 = 131222;
pub const _NL_WABALTMON_3: C2RustUnnamed_17 = 131221;
pub const _NL_WABALTMON_2: C2RustUnnamed_17 = 131220;
pub const _NL_WABALTMON_1: C2RustUnnamed_17 = 131219;
pub const _NL_ABALTMON_12: C2RustUnnamed_17 = 131218;
pub const _NL_ABALTMON_11: C2RustUnnamed_17 = 131217;
pub const _NL_ABALTMON_10: C2RustUnnamed_17 = 131216;
pub const _NL_ABALTMON_9: C2RustUnnamed_17 = 131215;
pub const _NL_ABALTMON_8: C2RustUnnamed_17 = 131214;
pub const _NL_ABALTMON_7: C2RustUnnamed_17 = 131213;
pub const _NL_ABALTMON_6: C2RustUnnamed_17 = 131212;
pub const _NL_ABALTMON_5: C2RustUnnamed_17 = 131211;
pub const _NL_ABALTMON_4: C2RustUnnamed_17 = 131210;
pub const _NL_ABALTMON_3: C2RustUnnamed_17 = 131209;
pub const _NL_ABALTMON_2: C2RustUnnamed_17 = 131208;
pub const _NL_ABALTMON_1: C2RustUnnamed_17 = 131207;
pub const _NL_WALTMON_12: C2RustUnnamed_17 = 131206;
pub const _NL_WALTMON_11: C2RustUnnamed_17 = 131205;
pub const _NL_WALTMON_10: C2RustUnnamed_17 = 131204;
pub const _NL_WALTMON_9: C2RustUnnamed_17 = 131203;
pub const _NL_WALTMON_8: C2RustUnnamed_17 = 131202;
pub const _NL_WALTMON_7: C2RustUnnamed_17 = 131201;
pub const _NL_WALTMON_6: C2RustUnnamed_17 = 131200;
pub const _NL_WALTMON_5: C2RustUnnamed_17 = 131199;
pub const _NL_WALTMON_4: C2RustUnnamed_17 = 131198;
pub const _NL_WALTMON_3: C2RustUnnamed_17 = 131197;
pub const _NL_WALTMON_2: C2RustUnnamed_17 = 131196;
pub const _NL_WALTMON_1: C2RustUnnamed_17 = 131195;
pub const __ALTMON_12: C2RustUnnamed_17 = 131194;
pub const __ALTMON_11: C2RustUnnamed_17 = 131193;
pub const __ALTMON_10: C2RustUnnamed_17 = 131192;
pub const __ALTMON_9: C2RustUnnamed_17 = 131191;
pub const __ALTMON_8: C2RustUnnamed_17 = 131190;
pub const __ALTMON_7: C2RustUnnamed_17 = 131189;
pub const __ALTMON_6: C2RustUnnamed_17 = 131188;
pub const __ALTMON_5: C2RustUnnamed_17 = 131187;
pub const __ALTMON_4: C2RustUnnamed_17 = 131186;
pub const __ALTMON_3: C2RustUnnamed_17 = 131185;
pub const __ALTMON_2: C2RustUnnamed_17 = 131184;
pub const __ALTMON_1: C2RustUnnamed_17 = 131183;
pub const _NL_TIME_CODESET: C2RustUnnamed_17 = 131182;
pub const _NL_W_DATE_FMT: C2RustUnnamed_17 = 131181;
pub const _DATE_FMT: C2RustUnnamed_17 = 131180;
pub const _NL_TIME_TIMEZONE: C2RustUnnamed_17 = 131179;
pub const _NL_TIME_CAL_DIRECTION: C2RustUnnamed_17 = 131178;
pub const _NL_TIME_FIRST_WORKDAY: C2RustUnnamed_17 = 131177;
pub const _NL_TIME_FIRST_WEEKDAY: C2RustUnnamed_17 = 131176;
pub const _NL_TIME_WEEK_1STWEEK: C2RustUnnamed_17 = 131175;
pub const _NL_TIME_WEEK_1STDAY: C2RustUnnamed_17 = 131174;
pub const _NL_TIME_WEEK_NDAYS: C2RustUnnamed_17 = 131173;
pub const _NL_WERA_T_FMT: C2RustUnnamed_17 = 131172;
pub const _NL_WERA_D_T_FMT: C2RustUnnamed_17 = 131171;
pub const _NL_WALT_DIGITS: C2RustUnnamed_17 = 131170;
pub const _NL_WERA_D_FMT: C2RustUnnamed_17 = 131169;
pub const _NL_WERA_YEAR: C2RustUnnamed_17 = 131168;
pub const _NL_WT_FMT_AMPM: C2RustUnnamed_17 = 131167;
pub const _NL_WT_FMT: C2RustUnnamed_17 = 131166;
pub const _NL_WD_FMT: C2RustUnnamed_17 = 131165;
pub const _NL_WD_T_FMT: C2RustUnnamed_17 = 131164;
pub const _NL_WPM_STR: C2RustUnnamed_17 = 131163;
pub const _NL_WAM_STR: C2RustUnnamed_17 = 131162;
pub const _NL_WMON_12: C2RustUnnamed_17 = 131161;
pub const _NL_WMON_11: C2RustUnnamed_17 = 131160;
pub const _NL_WMON_10: C2RustUnnamed_17 = 131159;
pub const _NL_WMON_9: C2RustUnnamed_17 = 131158;
pub const _NL_WMON_8: C2RustUnnamed_17 = 131157;
pub const _NL_WMON_7: C2RustUnnamed_17 = 131156;
pub const _NL_WMON_6: C2RustUnnamed_17 = 131155;
pub const _NL_WMON_5: C2RustUnnamed_17 = 131154;
pub const _NL_WMON_4: C2RustUnnamed_17 = 131153;
pub const _NL_WMON_3: C2RustUnnamed_17 = 131152;
pub const _NL_WMON_2: C2RustUnnamed_17 = 131151;
pub const _NL_WMON_1: C2RustUnnamed_17 = 131150;
pub const _NL_WABMON_12: C2RustUnnamed_17 = 131149;
pub const _NL_WABMON_11: C2RustUnnamed_17 = 131148;
pub const _NL_WABMON_10: C2RustUnnamed_17 = 131147;
pub const _NL_WABMON_9: C2RustUnnamed_17 = 131146;
pub const _NL_WABMON_8: C2RustUnnamed_17 = 131145;
pub const _NL_WABMON_7: C2RustUnnamed_17 = 131144;
pub const _NL_WABMON_6: C2RustUnnamed_17 = 131143;
pub const _NL_WABMON_5: C2RustUnnamed_17 = 131142;
pub const _NL_WABMON_4: C2RustUnnamed_17 = 131141;
pub const _NL_WABMON_3: C2RustUnnamed_17 = 131140;
pub const _NL_WABMON_2: C2RustUnnamed_17 = 131139;
pub const _NL_WABMON_1: C2RustUnnamed_17 = 131138;
pub const _NL_WDAY_7: C2RustUnnamed_17 = 131137;
pub const _NL_WDAY_6: C2RustUnnamed_17 = 131136;
pub const _NL_WDAY_5: C2RustUnnamed_17 = 131135;
pub const _NL_WDAY_4: C2RustUnnamed_17 = 131134;
pub const _NL_WDAY_3: C2RustUnnamed_17 = 131133;
pub const _NL_WDAY_2: C2RustUnnamed_17 = 131132;
pub const _NL_WDAY_1: C2RustUnnamed_17 = 131131;
pub const _NL_WABDAY_7: C2RustUnnamed_17 = 131130;
pub const _NL_WABDAY_6: C2RustUnnamed_17 = 131129;
pub const _NL_WABDAY_5: C2RustUnnamed_17 = 131128;
pub const _NL_WABDAY_4: C2RustUnnamed_17 = 131127;
pub const _NL_WABDAY_3: C2RustUnnamed_17 = 131126;
pub const _NL_WABDAY_2: C2RustUnnamed_17 = 131125;
pub const _NL_WABDAY_1: C2RustUnnamed_17 = 131124;
pub const _NL_TIME_ERA_ENTRIES: C2RustUnnamed_17 = 131123;
pub const _NL_TIME_ERA_NUM_ENTRIES: C2RustUnnamed_17 = 131122;
pub const ERA_T_FMT: C2RustUnnamed_17 = 131121;
pub const ERA_D_T_FMT: C2RustUnnamed_17 = 131120;
pub const ALT_DIGITS: C2RustUnnamed_17 = 131119;
pub const ERA_D_FMT: C2RustUnnamed_17 = 131118;
pub const __ERA_YEAR: C2RustUnnamed_17 = 131117;
pub const ERA: C2RustUnnamed_17 = 131116;
pub const T_FMT_AMPM: C2RustUnnamed_17 = 131115;
pub const T_FMT: C2RustUnnamed_17 = 131114;
pub const D_FMT: C2RustUnnamed_17 = 131113;
pub const D_T_FMT: C2RustUnnamed_17 = 131112;
pub const PM_STR: C2RustUnnamed_17 = 131111;
pub const AM_STR: C2RustUnnamed_17 = 131110;
pub const MON_12: C2RustUnnamed_17 = 131109;
pub const MON_11: C2RustUnnamed_17 = 131108;
pub const MON_10: C2RustUnnamed_17 = 131107;
pub const MON_9: C2RustUnnamed_17 = 131106;
pub const MON_8: C2RustUnnamed_17 = 131105;
pub const MON_7: C2RustUnnamed_17 = 131104;
pub const MON_6: C2RustUnnamed_17 = 131103;
pub const MON_5: C2RustUnnamed_17 = 131102;
pub const MON_4: C2RustUnnamed_17 = 131101;
pub const MON_3: C2RustUnnamed_17 = 131100;
pub const MON_2: C2RustUnnamed_17 = 131099;
pub const MON_1: C2RustUnnamed_17 = 131098;
pub const ABMON_12: C2RustUnnamed_17 = 131097;
pub const ABMON_11: C2RustUnnamed_17 = 131096;
pub const ABMON_10: C2RustUnnamed_17 = 131095;
pub const ABMON_9: C2RustUnnamed_17 = 131094;
pub const ABMON_8: C2RustUnnamed_17 = 131093;
pub const ABMON_7: C2RustUnnamed_17 = 131092;
pub const ABMON_6: C2RustUnnamed_17 = 131091;
pub const ABMON_5: C2RustUnnamed_17 = 131090;
pub const ABMON_4: C2RustUnnamed_17 = 131089;
pub const ABMON_3: C2RustUnnamed_17 = 131088;
pub const ABMON_2: C2RustUnnamed_17 = 131087;
pub const ABMON_1: C2RustUnnamed_17 = 131086;
pub const DAY_7: C2RustUnnamed_17 = 131085;
pub const DAY_6: C2RustUnnamed_17 = 131084;
pub const DAY_5: C2RustUnnamed_17 = 131083;
pub const DAY_4: C2RustUnnamed_17 = 131082;
pub const DAY_3: C2RustUnnamed_17 = 131081;
pub const DAY_2: C2RustUnnamed_17 = 131080;
pub const DAY_1: C2RustUnnamed_17 = 131079;
pub const ABDAY_7: C2RustUnnamed_17 = 131078;
pub const ABDAY_6: C2RustUnnamed_17 = 131077;
pub const ABDAY_5: C2RustUnnamed_17 = 131076;
pub const ABDAY_4: C2RustUnnamed_17 = 131075;
pub const ABDAY_3: C2RustUnnamed_17 = 131074;
pub const ABDAY_2: C2RustUnnamed_17 = 131073;
pub const ABDAY_1: C2RustUnnamed_17 = 131072;
pub type C2RustUnnamed_18 = libc::c_uint;
pub const SUBTHREAD_LINES_HEURISTIC: C2RustUnnamed_18 = 131072;
pub type C2RustUnnamed_19 = libc::c_uint;
pub const DEFAULT_MAX_THREADS: C2RustUnnamed_19 = 8;
pub type C2RustUnnamed_20 = libc::c_uint;
pub const SORT_FAILURE: C2RustUnnamed_20 = 2;
pub const SORT_OUT_OF_ORDER: C2RustUnnamed_20 = 1;
pub type C2RustUnnamed_21 = libc::c_uint;
pub const MAX_FORK_TRIES_DECOMPRESS: C2RustUnnamed_21 = 9;
pub const MAX_FORK_TRIES_COMPRESS: C2RustUnnamed_21 = 4;
pub type C2RustUnnamed_22 = libc::c_uint;
pub const MERGE_ROOT: C2RustUnnamed_22 = 1;
pub const MERGE_END: C2RustUnnamed_22 = 0;
pub type blanktype = libc::c_uint;
pub const bl_both: blanktype = 2;
pub const bl_end: blanktype = 1;
pub const bl_start: blanktype = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct line {
    pub text: *mut libc::c_char,
    pub length: size_t,
    pub keybeg: *mut libc::c_char,
    pub keylim: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buffer {
    pub buf: *mut libc::c_char,
    pub used: size_t,
    pub nlines: size_t,
    pub alloc: size_t,
    pub left: size_t,
    pub line_bytes: size_t,
    pub eof: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct keyfield {
    pub sword: size_t,
    pub schar: size_t,
    pub eword: size_t,
    pub echar: size_t,
    pub ignore: *const bool,
    pub translate: *const libc::c_char,
    pub skipsblanks: bool,
    pub skipeblanks: bool,
    pub numeric: bool,
    pub random: bool,
    pub general_numeric: bool,
    pub human_numeric: bool,
    pub month: bool,
    pub reverse: bool,
    pub version: bool,
    pub traditional_used: bool,
    pub next: *mut keyfield,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct month {
    pub name: *const libc::c_char,
    pub val: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct merge_node {
    pub lo: *mut line,
    pub hi: *mut line,
    pub end_lo: *mut line,
    pub end_hi: *mut line,
    pub dest: *mut *mut line,
    pub nlo: size_t,
    pub nhi: size_t,
    pub parent: *mut merge_node,
    pub lo_child: *mut merge_node,
    pub hi_child: *mut merge_node,
    pub level: libc::c_uint,
    pub queued: bool,
    pub lock: pthread_mutex_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct merge_node_queue {
    pub priority_queue: *mut heap,
    pub mutex: pthread_mutex_t,
    pub cond: pthread_cond_t,
}
pub type C2RustUnnamed_23 = libc::c_uint;
pub const NON_CHAR: C2RustUnnamed_23 = 256;
pub type C2RustUnnamed_24 = libc::c_uint;
pub const TAB_DEFAULT: C2RustUnnamed_24 = 256;
pub type C2RustUnnamed_25 = libc::c_uint;
pub const PARALLEL_OPTION: C2RustUnnamed_25 = 263;
pub const SORT_OPTION: C2RustUnnamed_25 = 262;
pub const RANDOM_SOURCE_OPTION: C2RustUnnamed_25 = 261;
pub const NMERGE_OPTION: C2RustUnnamed_25 = 260;
pub const FILES0_FROM_OPTION: C2RustUnnamed_25 = 259;
pub const DEBUG_PROGRAM_OPTION: C2RustUnnamed_25 = 258;
pub const COMPRESS_PROGRAM_OPTION: C2RustUnnamed_25 = 257;
pub const CHECK_OPTION: C2RustUnnamed_25 = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cs_status {
    pub valid: bool,
    pub sigs: sigset_t,
}
pub type C2RustUnnamed_26 = libc::c_uint;
pub const REAPED: C2RustUnnamed_26 = 2;
pub const UNREAPED: C2RustUnnamed_26 = 1;
pub const UNCOMPRESSED: C2RustUnnamed_26 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tempnode {
    pub next: *mut tempnode,
    pub pid: pid_t,
    pub state: libc::c_char,
    pub name: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sortfile {
    pub name: *const libc::c_char,
    pub temp: *mut tempnode,
}
pub type C2RustUnnamed_27 = libc::c_uint;
pub const INIT_PROCTAB_SIZE: C2RustUnnamed_27 = 47;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_28 {
    pub _gl_dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MD5state_st {
    pub A: libc::c_uint,
    pub B: libc::c_uint,
    pub C: libc::c_uint,
    pub D: libc::c_uint,
    pub Nl: libc::c_uint,
    pub Nh: libc::c_uint,
    pub data: [libc::c_uint; 16],
    pub num: libc::c_uint,
}
pub type MD5_CTX = MD5state_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct md5_ctx {
    pub CTX: MD5_CTX,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct thread_args {
    pub lines: *mut line,
    pub nthreads: size_t,
    pub total_lines: size_t,
    pub node: *mut merge_node,
    pub queue: *mut merge_node_queue,
    pub tfp: *mut FILE,
    pub output_temp: *const libc::c_char,
}
pub const nsigs: C2RustUnnamed_29 = 11;
pub type C2RustUnnamed_29 = libc::c_uint;
#[inline]
unsafe extern "C" fn initialize_exit_failure(mut status: libc::c_int) {
    if status != 1 as libc::c_int {
        ::core::ptr::write_volatile(&mut exit_failure as *mut libc::c_int, status);
    }
}
#[inline]
unsafe extern "C" fn to_uchar(mut ch: libc::c_char) -> libc::c_uchar {
    return ch as libc::c_uchar;
}
#[inline]
unsafe extern "C" fn select_plural(mut n: uintmax_t) -> libc::c_ulong {
    return if n
        <= (9223372036854775807 as libc::c_long as libc::c_ulong)
            .wrapping_mul(2 as libc::c_ulong)
            .wrapping_add(1 as libc::c_ulong)
    {
        n
    } else {
        n.wrapping_rem(PLURAL_REDUCER as libc::c_int as libc::c_ulong)
            .wrapping_add(PLURAL_REDUCER as libc::c_int as libc::c_ulong)
    };
}
#[inline]
unsafe extern "C" fn psame_inode(mut a: *const stat, mut b: *const stat) -> bool {
    return (*a).st_dev ^ (*b).st_dev | (*a).st_ino ^ (*b).st_ino == 0;
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
static mut decimal_point: libc::c_char = 0;
static mut thousands_sep: libc::c_int = 0;
static mut thousands_sep_ignored: bool = false;
static mut hard_LC_COLLATE: bool = false;
static mut hard_LC_TIME: bool = false;
static mut eolchar: libc::c_char = '\n' as i32 as libc::c_char;
static mut saved_line: line = line {
    text: 0 as *const libc::c_char as *mut libc::c_char,
    length: 0,
    keybeg: 0 as *const libc::c_char as *mut libc::c_char,
    keylim: 0 as *const libc::c_char as *mut libc::c_char,
};
static mut blanks: [bool; 256] = [false; 256];
static mut nonprinting: [bool; 256] = [false; 256];
static mut nondictionary: [bool; 256] = [false; 256];
static mut fold_toupper: [libc::c_char; 256] = [0; 256];
static mut monthtab: [month; 12] = [
    {
        let mut init = month {
            name: b"APR\0" as *const u8 as *const libc::c_char,
            val: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = month {
            name: b"AUG\0" as *const u8 as *const libc::c_char,
            val: 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = month {
            name: b"DEC\0" as *const u8 as *const libc::c_char,
            val: 12 as libc::c_int,
        };
        init
    },
    {
        let mut init = month {
            name: b"FEB\0" as *const u8 as *const libc::c_char,
            val: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = month {
            name: b"JAN\0" as *const u8 as *const libc::c_char,
            val: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = month {
            name: b"JUL\0" as *const u8 as *const libc::c_char,
            val: 7 as libc::c_int,
        };
        init
    },
    {
        let mut init = month {
            name: b"JUN\0" as *const u8 as *const libc::c_char,
            val: 6 as libc::c_int,
        };
        init
    },
    {
        let mut init = month {
            name: b"MAR\0" as *const u8 as *const libc::c_char,
            val: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = month {
            name: b"MAY\0" as *const u8 as *const libc::c_char,
            val: 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = month {
            name: b"NOV\0" as *const u8 as *const libc::c_char,
            val: 11 as libc::c_int,
        };
        init
    },
    {
        let mut init = month {
            name: b"OCT\0" as *const u8 as *const libc::c_char,
            val: 10 as libc::c_int,
        };
        init
    },
    {
        let mut init = month {
            name: b"SEP\0" as *const u8 as *const libc::c_char,
            val: 9 as libc::c_int,
        };
        init
    },
];
static mut merge_buffer_size: size_t = 0;
static mut sort_size: size_t = 0;
static mut temp_dirs: *mut *const libc::c_char = 0 as *const *const libc::c_char
    as *mut *const libc::c_char;
static mut temp_dir_count: size_t = 0;
static mut temp_dir_alloc: size_t = 0;
static mut reverse: bool = false;
static mut stable: bool = false;
static mut tab: libc::c_int = TAB_DEFAULT as libc::c_int;
static mut unique: bool = false;
static mut have_read_stdin: bool = false;
static mut keylist: *mut keyfield = 0 as *const keyfield as *mut keyfield;
static mut compress_program: *const libc::c_char = 0 as *const libc::c_char;
static mut debug: bool = false;
static mut nmerge: libc::c_uint = 16 as libc::c_int as libc::c_uint;
unsafe extern "C" fn async_safe_die(
    mut errnum: libc::c_int,
    mut errstr: *const libc::c_char,
) {
    write(2 as libc::c_int, errstr as *const libc::c_void, strlen(errstr));
    if errnum != 0 {
        let mut errbuf: [libc::c_char; 12] = [0; 12];
        let mut p: *mut libc::c_char = inttostr(errnum, errbuf.as_mut_ptr());
        write(
            2 as libc::c_int,
            b": errno \0" as *const u8 as *const libc::c_char as *const libc::c_void,
            8 as libc::c_int as size_t,
        );
        write(2 as libc::c_int, p as *const libc::c_void, strlen(p));
    }
    write(
        2 as libc::c_int,
        b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        1 as libc::c_int as size_t,
    );
    _exit(SORT_FAILURE as libc::c_int);
}
unsafe extern "C" fn sort_die(
    mut message: *const libc::c_char,
    mut file: *const libc::c_char,
) {
    if 0 != 0 {
        error(
            SORT_FAILURE as libc::c_int,
            *__errno_location(),
            b"%s: %s\0" as *const u8 as *const libc::c_char,
            message,
            quotearg_n_style_colon(
                0 as libc::c_int,
                shell_escape_quoting_style,
                (if !file.is_null() {
                    file
                } else {
                    gettext(b"standard output\0" as *const u8 as *const libc::c_char)
                        as *const libc::c_char
                }),
            ),
        );
        if SORT_FAILURE as libc::c_int != 0 as libc::c_int {
            unreachable!();
        } else {};
    } else {
        ({
            let __errstatus: libc::c_int = SORT_FAILURE as libc::c_int;
            error(
                __errstatus,
                *__errno_location(),
                b"%s: %s\0" as *const u8 as *const libc::c_char,
                message,
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    (if !file.is_null() {
                        file
                    } else {
                        gettext(b"standard output\0" as *const u8 as *const libc::c_char)
                            as *const libc::c_char
                    }),
                ),
            );
            if __errstatus != 0 as libc::c_int {
                unreachable!();
            } else {};
            
        });
        ({
            let __errstatus: libc::c_int = SORT_FAILURE as libc::c_int;
            error(
                __errstatus,
                *__errno_location(),
                b"%s: %s\0" as *const u8 as *const libc::c_char,
                message,
                quotearg_n_style_colon(
                    0 as libc::c_int,
                    shell_escape_quoting_style,
                    (if !file.is_null() {
                        file
                    } else {
                        gettext(b"standard output\0" as *const u8 as *const libc::c_char)
                            as *const libc::c_char
                    }),
                ),
            );
            if __errstatus != 0 as libc::c_int {
                unreachable!();
            } else {};
            
        });
    };
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
                b"Usage: %s [OPTION]... [FILE]...\n  or:  %s [OPTION]... --files0-from=F\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            program_name,
            program_name,
        );
        fputs_unlocked(
            gettext(
                b"Write sorted concatenation of all FILE(s) to standard output.\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        emit_stdin_note();
        emit_mandatory_arg_note();
        fputs_unlocked(
            gettext(b"Ordering options:\n\n\0" as *const u8 as *const libc::c_char),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"  -b, --ignore-leading-blanks  ignore leading blanks\n  -d, --dictionary-order      consider only blanks and alphanumeric characters\n  -f, --ignore-case           fold lower case to upper case characters\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"  -g, --general-numeric-sort  compare according to general numerical value\n  -i, --ignore-nonprinting    consider only printable characters\n  -M, --month-sort            compare (unknown) < 'JAN' < ... < 'DEC'\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"  -h, --human-numeric-sort    compare human readable numbers (e.g., 2K 1G)\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"  -n, --numeric-sort          compare according to string numerical value;\n                                see manual for which strings are supported\n  -R, --random-sort           shuffle, but group identical keys.  See shuf(1)\n      --random-source=FILE    get random bytes from FILE\n  -r, --reverse               reverse the result of comparisons\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"      --sort=WORD             sort according to WORD:\n                                general-numeric -g, human-numeric -h, month -M,\n                                numeric -n, random -R, version -V\n  -V, --version-sort          natural sort of (version) numbers within text\n\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(b"Other options:\n\n\0" as *const u8 as *const libc::c_char),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"      --batch-size=NMERGE   merge at most NMERGE inputs at once;\n                            for more use temp files\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"  -c, --check, --check=diagnose-first  check for sorted input; do not sort\n  -C, --check=quiet, --check=silent  like -c, but do not report first bad line\n      --compress-program=PROG  compress temporaries with PROG;\n                              decompress them with PROG -d\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"      --debug               annotate the part of the line used to sort,\n                              and warn about questionable usage to stderr\n      --files0-from=F       read input from the files specified by\n                            NUL-terminated names in file F;\n                            If F is - then read names from standard input\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"  -k, --key=KEYDEF          sort via a key; KEYDEF gives location and type\n  -m, --merge               merge already sorted files; do not sort\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"  -o, --output=FILE         write result to FILE instead of standard output\n  -s, --stable              stabilize sort by disabling last-resort comparison\n  -S, --buffer-size=SIZE    use SIZE for main memory buffer\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        printf(
            gettext(
                b"  -t, --field-separator=SEP  use SEP instead of non-blank to blank transition\n  -T, --temporary-directory=DIR  use DIR for temporaries, not $TMPDIR or %s;\n                              multiple options specify multiple directories\n      --parallel=N          change the number of sorts run concurrently to N\n  -u, --unique              with -c, check for strict ordering;\n                              without -c, output only the first of an equal run\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            b"/tmp\0" as *const u8 as *const libc::c_char,
        );
        fputs_unlocked(
            gettext(
                b"  -z, --zero-terminated     line delimiter is NUL, not newline\n\0"
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
                b"\nKEYDEF is F[.C][OPTS][,F[.C][OPTS]] for start and stop position, where F is a\nfield number and C a character position in the field; both are origin 1, and\nthe stop position defaults to the line's end.  If neither -t nor -b is in\neffect, characters in a field are counted from the beginning of the preceding\nwhitespace.  OPTS is one or more single-letter ordering options [bdfgiMhnRrV],\nwhich override global ordering options for that key.  If no key is given, use\nthe entire line as the key.  Use --debug to diagnose incorrect key usage.\n\nSIZE may be followed by the following multiplicative suffixes:\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        fputs_unlocked(
            gettext(
                b"% 1% of memory, b 1, K 1024 (default), and so on for M, G, T, P, E, Z, Y, R, Q.\n\n*** WARNING ***\nThe locale specified by the environment affects sort order.\nSet LC_ALL=C to get the traditional sort order that uses\nnative byte values.\n\0"
                    as *const u8 as *const libc::c_char,
            ),
            stdout,
        );
        emit_ancillary_info(b"sort\0" as *const u8 as *const libc::c_char);
    }
    exit(status);
}
static mut short_options: [libc::c_char; 31] = unsafe {
    *::core::mem::transmute::<
        &[u8; 31],
        &[libc::c_char; 31],
    >(b"-bcCdfghik:mMno:rRsS:t:T:uVy:z\0")
};
static mut long_options: [option; 31] = [
    {
        let mut init = option {
            name: b"ignore-leading-blanks\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'b' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"check\0" as *const u8 as *const libc::c_char,
            has_arg: 2 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: CHECK_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"compress-program\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: COMPRESS_PROGRAM_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"debug\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: DEBUG_PROGRAM_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"dictionary-order\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'd' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"ignore-case\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'f' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"files0-from\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: FILES0_FROM_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"general-numeric-sort\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'g' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"ignore-nonprinting\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'i' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"key\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'k' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"merge\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'm' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"month-sort\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'M' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"numeric-sort\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'n' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"human-numeric-sort\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'h' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"version-sort\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'V' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"random-sort\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'R' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"random-source\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: RANDOM_SOURCE_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"sort\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: SORT_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"output\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'o' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"reverse\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'r' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"stable\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 's' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"batch-size\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: NMERGE_OPTION as libc::c_int,
        };
        init
    },
    {
        let mut init = option {
            name: b"buffer-size\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'S' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"field-separator\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 't' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"temporary-directory\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'T' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"unique\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: 'u' as i32,
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
            name: b"parallel\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_int as *mut libc::c_int,
            val: PARALLEL_OPTION as libc::c_int,
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
static mut check_args: [*const libc::c_char; 4] = [
    b"quiet\0" as *const u8 as *const libc::c_char,
    b"silent\0" as *const u8 as *const libc::c_char,
    b"diagnose-first\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut check_types: [libc::c_char; 3] = [
    'C' as i32 as libc::c_char,
    'C' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
];
static mut sort_args: [*const libc::c_char; 7] = [
    b"general-numeric\0" as *const u8 as *const libc::c_char,
    b"human-numeric\0" as *const u8 as *const libc::c_char,
    b"month\0" as *const u8 as *const libc::c_char,
    b"numeric\0" as *const u8 as *const libc::c_char,
    b"random\0" as *const u8 as *const libc::c_char,
    b"version\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut sort_types: [libc::c_char; 6] = [
    'g' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'V' as i32 as libc::c_char,
];
static mut caught_signals: sigset_t = sigset_t { __val: [0; 16] };
unsafe extern "C" fn cs_enter(mut status: *mut cs_status) {
    let mut ret: libc::c_int = pthread_sigmask(
        0 as libc::c_int,
        &mut caught_signals,
        &mut (*status).sigs,
    );
    (*status).valid = ret == 0 as libc::c_int;
}
unsafe extern "C" fn cs_leave(mut status: *const cs_status) {
    if (*status).valid {
        pthread_sigmask(2 as libc::c_int, &(*status).sigs, 0 as *mut __sigset_t);
    }
}
static mut temphead: *mut tempnode = 0 as *const tempnode as *mut tempnode;
static mut temptail: *mut *mut tempnode = unsafe {
    &temphead as *const *mut tempnode as *mut *mut tempnode
};
static mut proctab: *mut Hash_table = 0 as *const Hash_table as *mut Hash_table;
unsafe extern "C" fn proctab_hasher(
    mut entry: *const libc::c_void,
    mut tabsize: size_t,
) -> size_t {
    let mut node: *const tempnode = entry as *const tempnode;
    return ((*node).pid as libc::c_ulong).wrapping_rem(tabsize);
}
unsafe extern "C" fn proctab_comparator(
    mut e1: *const libc::c_void,
    mut e2: *const libc::c_void,
) -> bool {
    let mut n1: *const tempnode = e1 as *const tempnode;
    let mut n2: *const tempnode = e2 as *const tempnode;
    return (*n1).pid == (*n2).pid;
}
static mut nprocs: pid_t = 0;
unsafe extern "C" fn reap(mut pid: pid_t) -> pid_t {
    let mut status: libc::c_int = 0;
    let mut cpid: pid_t = waitpid(
        if pid != 0 { pid } else { -(1 as libc::c_int) },
        &mut status,
        if pid != 0 { 0 as libc::c_int } else { 1 as libc::c_int },
    );
    if cpid < 0 as libc::c_int {
        if 0 != 0 {
            error(
                SORT_FAILURE as libc::c_int,
                *__errno_location(),
                gettext(b"waiting for %s [-d]\0" as *const u8 as *const libc::c_char),
                quotearg_style(shell_escape_always_quoting_style, compress_program),
            );
            if SORT_FAILURE as libc::c_int != 0 as libc::c_int {
                unreachable!();
            } else {};
        } else {
            ({
                let __errstatus: libc::c_int = SORT_FAILURE as libc::c_int;
                error(
                    __errstatus,
                    *__errno_location(),
                    gettext(
                        b"waiting for %s [-d]\0" as *const u8 as *const libc::c_char,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, compress_program),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
            ({
                let __errstatus: libc::c_int = SORT_FAILURE as libc::c_int;
                error(
                    __errstatus,
                    *__errno_location(),
                    gettext(
                        b"waiting for %s [-d]\0" as *const u8 as *const libc::c_char,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, compress_program),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
    } else if (0 as libc::c_int) < cpid
        && ((0 as libc::c_int) < pid || delete_proc(cpid) as libc::c_int != 0)
    {
        if !(status & 0x7f as libc::c_int == 0 as libc::c_int)
            || (status & 0xff00 as libc::c_int) >> 8 as libc::c_int != 0
        {
            if 0 != 0 {
                error(
                    SORT_FAILURE as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"%s [-d] terminated abnormally\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, compress_program),
                );
                if SORT_FAILURE as libc::c_int != 0 as libc::c_int {
                    unreachable!();
                } else {};
            } else {
                ({
                    let __errstatus: libc::c_int = SORT_FAILURE as libc::c_int;
                    error(
                        __errstatus,
                        0 as libc::c_int,
                        gettext(
                            b"%s [-d] terminated abnormally\0" as *const u8
                                as *const libc::c_char,
                        ),
                        quotearg_style(
                            shell_escape_always_quoting_style,
                            compress_program,
                        ),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
                ({
                    let __errstatus: libc::c_int = SORT_FAILURE as libc::c_int;
                    error(
                        __errstatus,
                        0 as libc::c_int,
                        gettext(
                            b"%s [-d] terminated abnormally\0" as *const u8
                                as *const libc::c_char,
                        ),
                        quotearg_style(
                            shell_escape_always_quoting_style,
                            compress_program,
                        ),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
        nprocs -= 1;
        nprocs;
    }
    return cpid;
}
unsafe extern "C" fn register_proc(mut temp: *mut tempnode) {
    if proctab.is_null() {
        proctab = hash_initialize(
            INIT_PROCTAB_SIZE as libc::c_int as size_t,
            0 as *const Hash_tuning,
            Some(
                proctab_hasher
                    as unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
            ),
            Some(
                proctab_comparator
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> bool,
            ),
            None,
        );
        if proctab.is_null() {
            xalloc_die();
        }
    }
    (*temp).state = UNREAPED as libc::c_int as libc::c_char;
    if (hash_insert(proctab, temp as *const libc::c_void)).is_null() {
        xalloc_die();
    }
}
unsafe extern "C" fn delete_proc(mut pid: pid_t) -> bool {
    let mut test: tempnode = tempnode {
        next: 0 as *mut tempnode,
        pid: 0,
        state: 0,
        name: [],
    };
    test.pid = pid;
    let mut node: *mut tempnode = hash_remove(
        proctab,
        &mut test as *mut tempnode as *const libc::c_void,
    ) as *mut tempnode;
    if node.is_null() {
        return 0 as libc::c_int != 0;
    }
    (*node).state = REAPED as libc::c_int as libc::c_char;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn wait_proc(mut pid: pid_t) {
    if delete_proc(pid) {
        reap(pid);
    }
}
unsafe extern "C" fn reap_exited() {
    while (0 as libc::c_int) < nprocs && reap(0 as libc::c_int) != 0 {}
}
unsafe extern "C" fn reap_some() {
    reap(-(1 as libc::c_int));
    reap_exited();
}
unsafe extern "C" fn reap_all() {
    while (0 as libc::c_int) < nprocs {
        reap(-(1 as libc::c_int));
    }
}
unsafe extern "C" fn cleanup() {
    let mut node: *const tempnode = 0 as *const tempnode;
    node = temphead;
    while !node.is_null() {
        unlink(((*node).name).as_ptr());
        node = (*node).next;
    }
    ::core::ptr::write_volatile(&mut temphead as *mut *mut tempnode, 0 as *mut tempnode);
}
unsafe extern "C" fn exit_cleanup() {
    if !temphead.is_null() {
        let mut cs: cs_status = cs_status {
            valid: false,
            sigs: sigset_t { __val: [0; 16] },
        };
        cs_enter(&mut cs);
        cleanup();
        cs_leave(&mut cs);
    }
    close_stdout();
}
unsafe extern "C" fn create_temp_file(
    mut pfd: *mut libc::c_int,
    mut survive_fd_exhaustion: bool,
) -> *mut tempnode {
    static mut slashbase: [libc::c_char; 12] = unsafe {
        *::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"/sortXXXXXX\0")
    };
    static mut temp_dir_index: size_t = 0;
    let mut fd: libc::c_int = 0;
    let mut saved_errno: libc::c_int = 0;
    let mut temp_dir: *const libc::c_char = *temp_dirs.offset(temp_dir_index as isize);
    let mut len: size_t = strlen(temp_dir);
    let mut node: *mut tempnode = xmalloc(
        (13 as libc::c_ulong)
            .wrapping_add(::core::mem::align_of::<tempnode>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(
                len
                    .wrapping_add(
                        ::core::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
                    ),
            )
            & !(::core::mem::align_of::<tempnode>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    ) as *mut tempnode;
    let mut file: *mut libc::c_char = ((*node).name).as_mut_ptr();
    let mut cs: cs_status = cs_status {
        valid: false,
        sigs: sigset_t { __val: [0; 16] },
    };
    memcpy(file as *mut libc::c_void, temp_dir as *const libc::c_void, len);
    memcpy(
        file.offset(len as isize) as *mut libc::c_void,
        slashbase.as_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
    );
    ::core::ptr::write_volatile(
        &mut (*node).next as *mut *mut tempnode,
        0 as *mut tempnode,
    );
    temp_dir_index = temp_dir_index.wrapping_add(1);
    if temp_dir_index == temp_dir_count {
        temp_dir_index = 0 as libc::c_int as size_t;
    }
    cs_enter(&mut cs);
    fd = mkostemp_safer(file, 0o2000000 as libc::c_int);
    if 0 as libc::c_int <= fd {
        ::core::ptr::write_volatile(temptail, node);
        temptail = &mut (*node).next;
    }
    saved_errno = *__errno_location();
    cs_leave(&mut cs);
    *__errno_location() = saved_errno;
    if fd < 0 as libc::c_int {
        if !(survive_fd_exhaustion as libc::c_int != 0
            && *__errno_location() == 24 as libc::c_int)
        {
            if 0 != 0 {
                error(
                    SORT_FAILURE as libc::c_int,
                    *__errno_location(),
                    gettext(
                        b"cannot create temporary file in %s\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, temp_dir),
                );
                if SORT_FAILURE as libc::c_int != 0 as libc::c_int {
                    unreachable!();
                } else {};
            } else {
                ({
                    let __errstatus: libc::c_int = SORT_FAILURE as libc::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        gettext(
                            b"cannot create temporary file in %s\0" as *const u8
                                as *const libc::c_char,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, temp_dir),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
                ({
                    let __errstatus: libc::c_int = SORT_FAILURE as libc::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        gettext(
                            b"cannot create temporary file in %s\0" as *const u8
                                as *const libc::c_char,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, temp_dir),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
        free(node as *mut libc::c_void);
        node = 0 as *mut tempnode;
    }
    *pfd = fd;
    return node;
}
unsafe extern "C" fn get_outstatus() -> *mut stat {
    static mut outstat_errno: libc::c_int = 0;
    static mut outstat: stat = stat {
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
    if outstat_errno == 0 as libc::c_int {
        outstat_errno = if fstat(1 as libc::c_int, &mut outstat) == 0 as libc::c_int {
            -(1 as libc::c_int)
        } else {
            *__errno_location()
        };
    }
    return if outstat_errno < 0 as libc::c_int { &mut outstat } else { 0 as *mut stat };
}
unsafe extern "C" fn stream_open(
    mut file: *const libc::c_char,
    mut how: *const libc::c_char,
) -> *mut FILE {
    let mut fp: *mut FILE = 0 as *mut FILE;
    if *how as libc::c_int == 'r' as i32 {
        if strcmp(file, b"-\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
            have_read_stdin = 1 as libc::c_int != 0;
            fp = stdin;
        } else {
            let mut fd: libc::c_int = open(
                file,
                0 as libc::c_int | 0o2000000 as libc::c_int,
            );
            fp = if fd < 0 as libc::c_int { 0 as *mut FILE } else { fdopen(fd, how) };
        }
        fadvise(fp, FADVISE_SEQUENTIAL);
    } else if *how as libc::c_int == 'w' as i32 {
        if !file.is_null()
            && ftruncate(1 as libc::c_int, 0 as libc::c_int as __off_t)
                != 0 as libc::c_int
        {
            let mut ftruncate_errno: libc::c_int = *__errno_location();
            let mut outst: *mut stat = get_outstatus();
            if outst.is_null()
                || (*outst).st_mode & 0o170000 as libc::c_int as libc::c_uint
                    == 0o100000 as libc::c_int as libc::c_uint
                || ((*outst).st_mode).wrapping_sub((*outst).st_mode) != 0
            {
                if 0 != 0 {
                    error(
                        SORT_FAILURE as libc::c_int,
                        ftruncate_errno,
                        gettext(
                            b"%s: error truncating\0" as *const u8 as *const libc::c_char,
                        ),
                        quotearg_n_style_colon(
                            0 as libc::c_int,
                            shell_escape_quoting_style,
                            file,
                        ),
                    );
                    if SORT_FAILURE as libc::c_int != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                } else {
                    ({
                        let __errstatus: libc::c_int = SORT_FAILURE as libc::c_int;
                        error(
                            __errstatus,
                            ftruncate_errno,
                            gettext(
                                b"%s: error truncating\0" as *const u8
                                    as *const libc::c_char,
                            ),
                            quotearg_n_style_colon(
                                0 as libc::c_int,
                                shell_escape_quoting_style,
                                file,
                            ),
                        );
                        if __errstatus != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                        
                    });
                    ({
                        let __errstatus: libc::c_int = SORT_FAILURE as libc::c_int;
                        error(
                            __errstatus,
                            ftruncate_errno,
                            gettext(
                                b"%s: error truncating\0" as *const u8
                                    as *const libc::c_char,
                            ),
                            quotearg_n_style_colon(
                                0 as libc::c_int,
                                shell_escape_quoting_style,
                                file,
                            ),
                        );
                        if __errstatus != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                        
                    });
                };
            }
        }
        fp = stdout;
    } else {
        if (b"unexpected mode passed to stream_open\0" as *const u8
            as *const libc::c_char)
            .is_null()
        {} else {
            __assert_fail(
                b"!\"unexpected mode passed to stream_open\"\0" as *const u8
                    as *const libc::c_char,
                b"sort.c\0" as *const u8 as *const libc::c_char,
                980 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 46],
                    &[libc::c_char; 46],
                >(b"FILE *stream_open(const char *, const char *)\0"))
                    .as_ptr(),
            );
        }
        'c_10711: {
            if (b"unexpected mode passed to stream_open\0" as *const u8
                as *const libc::c_char)
                .is_null()
            {} else {
                __assert_fail(
                    b"!\"unexpected mode passed to stream_open\"\0" as *const u8
                        as *const libc::c_char,
                    b"sort.c\0" as *const u8 as *const libc::c_char,
                    980 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 46],
                        &[libc::c_char; 46],
                    >(b"FILE *stream_open(const char *, const char *)\0"))
                        .as_ptr(),
                );
            }
        };
    }
    return fp;
}
unsafe extern "C" fn xfopen(
    mut file: *const libc::c_char,
    mut how: *const libc::c_char,
) -> *mut FILE {
    let mut fp: *mut FILE = stream_open(file, how);
    if fp.is_null() {
        sort_die(gettext(b"open failed\0" as *const u8 as *const libc::c_char), file);
    }
    return fp;
}
unsafe extern "C" fn xfclose(mut fp: *mut FILE, mut file: *const libc::c_char) {
    match fileno(fp) {
        0 => {
            clearerr_unlocked(fp);
        }
        1 => {
            if fflush_unlocked(fp) != 0 as libc::c_int {
                sort_die(
                    gettext(b"fflush failed\0" as *const u8 as *const libc::c_char),
                    file,
                );
            }
        }
        _ => {
            if rpl_fclose(fp) != 0 as libc::c_int {
                sort_die(
                    gettext(b"close failed\0" as *const u8 as *const libc::c_char),
                    file,
                );
            }
        }
    };
}
unsafe extern "C" fn move_fd(mut oldfd: libc::c_int, mut newfd: libc::c_int) {
    if oldfd != newfd {
        dup2(oldfd, newfd);
        close(oldfd);
    }
}
unsafe extern "C" fn pipe_fork(
    mut pipefds: *mut libc::c_int,
    mut tries: size_t,
) -> pid_t {
    let mut saved_temphead: *mut tempnode = 0 as *mut tempnode;
    let mut saved_errno: libc::c_int = 0;
    let mut wait_retry: libc::c_double = 0.25f64;
    let mut pid: pid_t = 0;
    let mut cs: cs_status = cs_status {
        valid: false,
        sigs: sigset_t { __val: [0; 16] },
    };
    if rpl_pipe2(pipefds, 0o2000000 as libc::c_int) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if nmerge.wrapping_add(1 as libc::c_int as libc::c_uint) < nprocs as libc::c_uint {
        reap_some();
    }
    loop {
        let fresh0 = tries;
        tries = tries.wrapping_sub(1);
        if !(fresh0 != 0) {
            break;
        }
        cs_enter(&mut cs);
        saved_temphead = temphead;
        ::core::ptr::write_volatile(
            &mut temphead as *mut *mut tempnode,
            0 as *mut tempnode,
        );
        pid = fork();
        saved_errno = *__errno_location();
        if pid != 0 {
            ::core::ptr::write_volatile(
                &mut temphead as *mut *mut tempnode,
                saved_temphead,
            );
        }
        cs_leave(&mut cs);
        *__errno_location() = saved_errno;
        if 0 as libc::c_int <= pid || *__errno_location() != 11 as libc::c_int {
            break;
        }
        xnanosleep(wait_retry);
        wait_retry *= 2 as libc::c_int as libc::c_double;
        reap_exited();
    }
    if pid < 0 as libc::c_int {
        saved_errno = *__errno_location();
        close(*pipefds.offset(0 as libc::c_int as isize));
        close(*pipefds.offset(1 as libc::c_int as isize));
        *__errno_location() = saved_errno;
    } else if pid == 0 as libc::c_int {
        close(0 as libc::c_int);
        close(1 as libc::c_int);
    } else {
        nprocs += 1;
        nprocs;
    }
    return pid;
}
unsafe extern "C" fn maybe_create_temp(
    mut pfp: *mut *mut FILE,
    mut survive_fd_exhaustion: bool,
) -> *mut tempnode {
    let mut tempfd: libc::c_int = 0;
    let mut node: *mut tempnode = create_temp_file(&mut tempfd, survive_fd_exhaustion);
    if node.is_null() {
        return 0 as *mut tempnode;
    }
    (*node).state = UNCOMPRESSED as libc::c_int as libc::c_char;
    if !compress_program.is_null() {
        let mut pipefds: [libc::c_int; 2] = [0; 2];
        (*node)
            .pid = pipe_fork(
            pipefds.as_mut_ptr(),
            MAX_FORK_TRIES_COMPRESS as libc::c_int as size_t,
        );
        if (0 as libc::c_int) < (*node).pid {
            close(tempfd);
            close(pipefds[0 as libc::c_int as usize]);
            tempfd = pipefds[1 as libc::c_int as usize];
            register_proc(node);
        } else if (*node).pid == 0 as libc::c_int {
            close(pipefds[1 as libc::c_int as usize]);
            move_fd(tempfd, 1 as libc::c_int);
            move_fd(pipefds[0 as libc::c_int as usize], 0 as libc::c_int);
            execlp(
                compress_program,
                compress_program,
                0 as *mut libc::c_void as *mut libc::c_char,
            );
            async_safe_die(
                *__errno_location(),
                b"couldn't execute compress program\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    *pfp = fdopen(tempfd, b"w\0" as *const u8 as *const libc::c_char);
    if (*pfp).is_null() {
        sort_die(
            gettext(
                b"couldn't create temporary file\0" as *const u8 as *const libc::c_char,
            ),
            ((*node).name).as_mut_ptr(),
        );
    }
    return node;
}
unsafe extern "C" fn create_temp(mut pfp: *mut *mut FILE) -> *mut tempnode {
    return maybe_create_temp(pfp, 0 as libc::c_int != 0);
}
unsafe extern "C" fn open_temp(mut temp: *mut tempnode) -> *mut FILE {
    let mut tempfd: libc::c_int = 0;
    let mut pipefds: [libc::c_int; 2] = [0; 2];
    let mut fp: *mut FILE = 0 as *mut FILE;
    if (*temp).state as libc::c_int == UNREAPED as libc::c_int {
        wait_proc((*temp).pid);
    }
    tempfd = open(((*temp).name).as_mut_ptr(), 0 as libc::c_int);
    if tempfd < 0 as libc::c_int {
        return 0 as *mut FILE;
    }
    let mut child: pid_t = pipe_fork(
        pipefds.as_mut_ptr(),
        MAX_FORK_TRIES_DECOMPRESS as libc::c_int as size_t,
    );
    let mut current_block_22: u64;
    match child {
        -1 => {
            if *__errno_location() != 24 as libc::c_int {
                if 0 != 0 {
                    error(
                        SORT_FAILURE as libc::c_int,
                        *__errno_location(),
                        gettext(
                            b"couldn't create process for %s -d\0" as *const u8
                                as *const libc::c_char,
                        ),
                        quotearg_style(
                            shell_escape_always_quoting_style,
                            compress_program,
                        ),
                    );
                    if SORT_FAILURE as libc::c_int != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                } else {
                    ({
                        let __errstatus: libc::c_int = SORT_FAILURE as libc::c_int;
                        error(
                            __errstatus,
                            *__errno_location(),
                            gettext(
                                b"couldn't create process for %s -d\0" as *const u8
                                    as *const libc::c_char,
                            ),
                            quotearg_style(
                                shell_escape_always_quoting_style,
                                compress_program,
                            ),
                        );
                        if __errstatus != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                        
                    });
                    ({
                        let __errstatus: libc::c_int = SORT_FAILURE as libc::c_int;
                        error(
                            __errstatus,
                            *__errno_location(),
                            gettext(
                                b"couldn't create process for %s -d\0" as *const u8
                                    as *const libc::c_char,
                            ),
                            quotearg_style(
                                shell_escape_always_quoting_style,
                                compress_program,
                            ),
                        );
                        if __errstatus != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                        
                    });
                };
            }
            close(tempfd);
            *__errno_location() = 24 as libc::c_int;
            current_block_22 = 12124785117276362961;
        }
        0 => {
            close(pipefds[0 as libc::c_int as usize]);
            move_fd(tempfd, 0 as libc::c_int);
            move_fd(pipefds[1 as libc::c_int as usize], 1 as libc::c_int);
            execlp(
                compress_program,
                compress_program,
                b"-d\0" as *const u8 as *const libc::c_char,
                0 as *mut libc::c_void as *mut libc::c_char,
            );
            async_safe_die(
                *__errno_location(),
                b"couldn't execute compress program (with -d)\0" as *const u8
                    as *const libc::c_char,
            );
            current_block_22 = 13858992353700240241;
        }
        _ => {
            current_block_22 = 13858992353700240241;
        }
    }
    match current_block_22 {
        13858992353700240241 => {
            (*temp).pid = child;
            register_proc(temp);
            close(tempfd);
            close(pipefds[1 as libc::c_int as usize]);
            fp = fdopen(
                pipefds[0 as libc::c_int as usize],
                b"r\0" as *const u8 as *const libc::c_char,
            );
            if fp.is_null() {
                let mut saved_errno: libc::c_int = *__errno_location();
                close(pipefds[0 as libc::c_int as usize]);
                *__errno_location() = saved_errno;
            }
        }
        _ => {}
    }
    return fp;
}
unsafe extern "C" fn add_temp_dir(mut dir: *const libc::c_char) {
    if temp_dir_count == temp_dir_alloc {
        temp_dirs = (if ::core::mem::size_of::<C2RustUnnamed_28>() as libc::c_ulong != 0
        {
            x2nrealloc(
                temp_dirs as *mut libc::c_void,
                &mut temp_dir_alloc,
                ::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
            )
        } else {
            x2nrealloc(
                temp_dirs as *mut libc::c_void,
                &mut temp_dir_alloc,
                ::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
            )
        }) as *mut *const libc::c_char;
    }
    let fresh1 = temp_dir_count;
    temp_dir_count = temp_dir_count.wrapping_add(1);
    let ref mut fresh2 = *temp_dirs.offset(fresh1 as isize);
    *fresh2 = dir;
}
unsafe extern "C" fn zaptemp(mut name: *const libc::c_char) {
    let mut pnode: *mut *mut tempnode = 0 as *mut *mut tempnode;
    let mut node: *mut tempnode = 0 as *mut tempnode;
    let mut next: *mut tempnode = 0 as *mut tempnode;
    let mut unlink_status: libc::c_int = 0;
    let mut unlink_errno: libc::c_int = 0 as libc::c_int;
    let mut cs: cs_status = cs_status {
        valid: false,
        sigs: sigset_t { __val: [0; 16] },
    };
    pnode = &mut temphead;
    loop {
        node = *pnode;
        if !(((*node).name).as_mut_ptr() != name as *mut libc::c_char) {
            break;
        }
        pnode = &mut (*node).next;
    }
    if (*node).state as libc::c_int == UNREAPED as libc::c_int {
        wait_proc((*node).pid);
    }
    next = (*node).next;
    cs_enter(&mut cs);
    unlink_status = unlink(name);
    unlink_errno = *__errno_location();
    ::core::ptr::write_volatile(pnode, next);
    cs_leave(&mut cs);
    if unlink_status != 0 as libc::c_int {
        if 0 != 0 {
            error(
                0 as libc::c_int,
                unlink_errno,
                gettext(
                    b"warning: cannot remove: %s\0" as *const u8 as *const libc::c_char,
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
                    unlink_errno,
                    gettext(
                        b"warning: cannot remove: %s\0" as *const u8
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
                let __errstatus: libc::c_int = 0 as libc::c_int;
                error(
                    __errstatus,
                    unlink_errno,
                    gettext(
                        b"warning: cannot remove: %s\0" as *const u8
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
    if next.is_null() {
        temptail = pnode;
    }
    free(node as *mut libc::c_void);
}
unsafe extern "C" fn struct_month_cmp(
    mut m1: *const libc::c_void,
    mut m2: *const libc::c_void,
) -> libc::c_int {
    let mut month1: *const month = m1 as *const month;
    let mut month2: *const month = m2 as *const month;
    return strcmp((*month1).name, (*month2).name);
}
unsafe extern "C" fn inittables() {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i
        < (127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int)
            as libc::c_ulong
    {
        blanks[i
            as usize] = i == '\n' as i32 as libc::c_ulong
            || *(*__ctype_b_loc()).offset(i as libc::c_int as isize) as libc::c_int
                & _ISblank as libc::c_int as libc::c_ushort as libc::c_int != 0;
        nondictionary[i
            as usize] = !blanks[i as usize]
            && *(*__ctype_b_loc()).offset(i as libc::c_int as isize) as libc::c_int
                & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int == 0;
        nonprinting[i
            as usize] = *(*__ctype_b_loc()).offset(i as libc::c_int as isize)
            as libc::c_int & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
            == 0;
        fold_toupper[i as usize] = toupper(i as libc::c_int) as libc::c_char;
        i = i.wrapping_add(1);
        i;
    }
    if hard_LC_TIME {
        i = 0 as libc::c_int as size_t;
        while i < 12 as libc::c_int as libc::c_ulong {
            let mut s: *const libc::c_char = 0 as *const libc::c_char;
            let mut s_len: size_t = 0;
            let mut j: size_t = 0;
            let mut k: size_t = 0;
            let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
            s = nl_langinfo(
                (ABMON_1 as libc::c_int as libc::c_ulong).wrapping_add(i) as nl_item,
            );
            s_len = strlen(s);
            name = xmalloc(s_len.wrapping_add(1 as libc::c_int as libc::c_ulong))
                as *mut libc::c_char;
            monthtab[i as usize].name = name;
            monthtab[i as usize]
                .val = i.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
            k = 0 as libc::c_int as size_t;
            j = k;
            while j < s_len {
                if *(*__ctype_b_loc())
                    .offset(to_uchar(*s.offset(j as isize)) as libc::c_int as isize)
                    as libc::c_int
                    & _ISblank as libc::c_int as libc::c_ushort as libc::c_int == 0
                {
                    let fresh3 = k;
                    k = k.wrapping_add(1);
                    *name
                        .offset(
                            fresh3 as isize,
                        ) = fold_toupper[to_uchar(*s.offset(j as isize)) as usize];
                }
                j = j.wrapping_add(1);
                j;
            }
            *name.offset(k as isize) = '\0' as i32 as libc::c_char;
            i = i.wrapping_add(1);
            i;
        }
        qsort(
            monthtab.as_mut_ptr() as *mut libc::c_void,
            12 as libc::c_int as size_t,
            ::core::mem::size_of::<month>() as libc::c_ulong,
            Some(
                struct_month_cmp
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
        );
    }
}
unsafe extern "C" fn specify_nmerge(
    mut oi: libc::c_int,
    mut c: libc::c_char,
    mut s: *const libc::c_char,
) {
    let mut n: uintmax_t = 0;
    let mut rlimit: rlimit = rlimit { rlim_cur: 0, rlim_max: 0 };
    let mut e: strtol_error = xstrtoumax(
        s,
        0 as *mut *mut libc::c_char,
        10 as libc::c_int,
        &mut n,
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut max_nmerge: libc::c_uint = (if getrlimit(RLIMIT_NOFILE, &mut rlimit)
        == 0 as libc::c_int
    {
        rlimit.rlim_cur
    } else {
        20 as libc::c_int as libc::c_ulong
    })
        .wrapping_sub(3 as libc::c_int as libc::c_ulong) as libc::c_uint;
    if e as libc::c_uint == LONGINT_OK as libc::c_int as libc::c_uint {
        nmerge = n as libc::c_uint;
        if nmerge as libc::c_ulong != n {
            e = LONGINT_OVERFLOW;
        } else if nmerge < 2 as libc::c_int as libc::c_uint {
            if 0 != 0 {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"invalid --%s argument %s\0" as *const u8 as *const libc::c_char,
                    ),
                    long_options[oi as usize].name,
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
                            b"invalid --%s argument %s\0" as *const u8
                                as *const libc::c_char,
                        ),
                        long_options[oi as usize].name,
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
                            b"invalid --%s argument %s\0" as *const u8
                                as *const libc::c_char,
                        ),
                        long_options[oi as usize].name,
                        quote(s),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
            if 0 != 0 {
                error(
                    SORT_FAILURE as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"minimum --%s argument is %s\0" as *const u8
                            as *const libc::c_char,
                    ),
                    long_options[oi as usize].name,
                    quote(b"2\0" as *const u8 as *const libc::c_char),
                );
                if SORT_FAILURE as libc::c_int != 0 as libc::c_int {
                    unreachable!();
                } else {};
            } else {
                ({
                    let __errstatus: libc::c_int = SORT_FAILURE as libc::c_int;
                    error(
                        __errstatus,
                        0 as libc::c_int,
                        gettext(
                            b"minimum --%s argument is %s\0" as *const u8
                                as *const libc::c_char,
                        ),
                        long_options[oi as usize].name,
                        quote(b"2\0" as *const u8 as *const libc::c_char),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
                ({
                    let __errstatus: libc::c_int = SORT_FAILURE as libc::c_int;
                    error(
                        __errstatus,
                        0 as libc::c_int,
                        gettext(
                            b"minimum --%s argument is %s\0" as *const u8
                                as *const libc::c_char,
                        ),
                        long_options[oi as usize].name,
                        quote(b"2\0" as *const u8 as *const libc::c_char),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        } else if max_nmerge < nmerge {
            e = LONGINT_OVERFLOW;
        } else {
            return
        }
    }
    if e as libc::c_uint == LONGINT_OVERFLOW as libc::c_int as libc::c_uint {
        let mut max_nmerge_buf: [libc::c_char; 11] = [0; 11];
        if 0 != 0 {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"--%s argument %s too large\0" as *const u8 as *const libc::c_char,
                ),
                long_options[oi as usize].name,
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
                        b"--%s argument %s too large\0" as *const u8
                            as *const libc::c_char,
                    ),
                    long_options[oi as usize].name,
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
                        b"--%s argument %s too large\0" as *const u8
                            as *const libc::c_char,
                    ),
                    long_options[oi as usize].name,
                    quote(s),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
        if 0 != 0 {
            error(
                SORT_FAILURE as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"maximum --%s argument with current rlimit is %s\0" as *const u8
                        as *const libc::c_char,
                ),
                long_options[oi as usize].name,
                uinttostr(max_nmerge, max_nmerge_buf.as_mut_ptr()),
            );
            if SORT_FAILURE as libc::c_int != 0 as libc::c_int {
                unreachable!();
            } else {};
        } else {
            ({
                let __errstatus: libc::c_int = SORT_FAILURE as libc::c_int;
                error(
                    __errstatus,
                    0 as libc::c_int,
                    gettext(
                        b"maximum --%s argument with current rlimit is %s\0" as *const u8
                            as *const libc::c_char,
                    ),
                    long_options[oi as usize].name,
                    uinttostr(max_nmerge, max_nmerge_buf.as_mut_ptr()),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
            ({
                let __errstatus: libc::c_int = SORT_FAILURE as libc::c_int;
                error(
                    __errstatus,
                    0 as libc::c_int,
                    gettext(
                        b"maximum --%s argument with current rlimit is %s\0" as *const u8
                            as *const libc::c_char,
                    ),
                    long_options[oi as usize].name,
                    uinttostr(max_nmerge, max_nmerge_buf.as_mut_ptr()),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
    } else {
        xstrtol_fatal(e, oi, c, long_options.as_ptr(), s);
    };
}
unsafe extern "C" fn specify_sort_size(
    mut oi: libc::c_int,
    mut c: libc::c_char,
    mut s: *const libc::c_char,
) {
    let mut n: uintmax_t = 0;
    let mut suffix: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut e: strtol_error = xstrtoumax(
        s,
        &mut suffix,
        10 as libc::c_int,
        &mut n,
        b"EgGkKmMPQRtTYZ\0" as *const u8 as *const libc::c_char,
    );
    if e as libc::c_uint == LONGINT_OK as libc::c_int as libc::c_uint
        && (*suffix.offset(-(1 as libc::c_int) as isize) as libc::c_uint)
            .wrapping_sub('0' as i32 as libc::c_uint) <= 9 as libc::c_int as libc::c_uint
    {
        if n
            <= (18446744073709551615 as libc::c_ulong)
                .wrapping_div(1024 as libc::c_int as libc::c_ulong)
        {
            n = (n as libc::c_ulong).wrapping_mul(1024 as libc::c_int as libc::c_ulong)
                as uintmax_t as uintmax_t;
        } else {
            e = LONGINT_OVERFLOW;
        }
    }
    if e as libc::c_uint == LONGINT_INVALID_SUFFIX_CHAR as libc::c_int as libc::c_uint
        && (*suffix.offset(-(1 as libc::c_int) as isize) as libc::c_uint)
            .wrapping_sub('0' as i32 as libc::c_uint) <= 9 as libc::c_int as libc::c_uint
        && *suffix.offset(1 as libc::c_int as isize) == 0
    {
        match *suffix.offset(0 as libc::c_int as isize) as libc::c_int {
            98 => {
                e = LONGINT_OK;
            }
            37 => {
                let mut mem: libc::c_double = physmem_total() * n as libc::c_double
                    / 100 as libc::c_int as libc::c_double;
                if mem < 18446744073709551615 as libc::c_ulong as libc::c_double {
                    n = mem as uintmax_t;
                    e = LONGINT_OK;
                } else {
                    e = LONGINT_OVERFLOW;
                }
            }
            _ => {}
        }
    }
    if e as libc::c_uint == LONGINT_OK as libc::c_int as libc::c_uint {
        if n < sort_size {
            return;
        }
        sort_size = n;
        if sort_size == n {
            sort_size = if sort_size
                > (nmerge as libc::c_ulong)
                    .wrapping_mul(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_add(
                                ::core::mem::size_of::<line>() as libc::c_ulong,
                            ),
                    )
            {
                sort_size
            } else {
                (nmerge as libc::c_ulong)
                    .wrapping_mul(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_add(
                                ::core::mem::size_of::<line>() as libc::c_ulong,
                            ),
                    )
            };
            return;
        }
        e = LONGINT_OVERFLOW;
    }
    xstrtol_fatal(e, oi, c, long_options.as_ptr(), s);
}
unsafe extern "C" fn specify_nthreads(
    mut oi: libc::c_int,
    mut c: libc::c_char,
    mut s: *const libc::c_char,
) -> size_t {
    let mut nthreads: uintmax_t = 0;
    let mut e: strtol_error = xstrtoumax(
        s,
        0 as *mut *mut libc::c_char,
        10 as libc::c_int,
        &mut nthreads,
        b"\0" as *const u8 as *const libc::c_char,
    );
    if e as libc::c_uint == LONGINT_OVERFLOW as libc::c_int as libc::c_uint {
        return 18446744073709551615 as libc::c_ulong;
    }
    if e as libc::c_uint != LONGINT_OK as libc::c_int as libc::c_uint {
        xstrtol_fatal(e, oi, c, long_options.as_ptr(), s);
    }
    if (18446744073709551615 as libc::c_ulong) < nthreads {
        nthreads = 18446744073709551615 as libc::c_ulong;
    }
    if nthreads == 0 as libc::c_int as libc::c_ulong {
        if 0 != 0 {
            error(
                SORT_FAILURE as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"number in parallel must be nonzero\0" as *const u8
                        as *const libc::c_char,
                ),
            );
            if SORT_FAILURE as libc::c_int != 0 as libc::c_int {
                unreachable!();
            } else {};
        } else {
            ({
                let __errstatus: libc::c_int = SORT_FAILURE as libc::c_int;
                error(
                    __errstatus,
                    0 as libc::c_int,
                    gettext(
                        b"number in parallel must be nonzero\0" as *const u8
                            as *const libc::c_char,
                    ),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
            ({
                let __errstatus: libc::c_int = SORT_FAILURE as libc::c_int;
                error(
                    __errstatus,
                    0 as libc::c_int,
                    gettext(
                        b"number in parallel must be nonzero\0" as *const u8
                            as *const libc::c_char,
                    ),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
    }
    return nthreads;
}
unsafe extern "C" fn default_sort_size() -> size_t {
    let mut size: size_t = 18446744073709551615 as libc::c_ulong;
    let mut rlimit: rlimit = rlimit { rlim_cur: 0, rlim_max: 0 };
    if getrlimit(RLIMIT_DATA, &mut rlimit) == 0 as libc::c_int && rlimit.rlim_cur < size
    {
        size = rlimit.rlim_cur;
    }
    if getrlimit(RLIMIT_AS, &mut rlimit) == 0 as libc::c_int && rlimit.rlim_cur < size {
        size = rlimit.rlim_cur;
    }
    size = (size as libc::c_ulong).wrapping_div(2 as libc::c_int as libc::c_ulong)
        as size_t as size_t;
    if getrlimit(__RLIMIT_RSS, &mut rlimit) == 0 as libc::c_int
        && (rlimit.rlim_cur)
            .wrapping_div(16 as libc::c_int as libc::c_ulong)
            .wrapping_mul(15 as libc::c_int as libc::c_ulong) < size
    {
        size = (rlimit.rlim_cur)
            .wrapping_div(16 as libc::c_int as libc::c_ulong)
            .wrapping_mul(15 as libc::c_int as libc::c_ulong);
    }
    let mut avail: libc::c_double = physmem_available();
    let mut total: libc::c_double = physmem_total();
    let mut mem: libc::c_double = if avail > total / 8 as libc::c_int as libc::c_double {
        avail
    } else {
        total / 8 as libc::c_int as libc::c_double
    };
    if total * 0.75f64 < size as libc::c_double {
        size = (total * 0.75f64) as size_t;
    }
    if mem < size as libc::c_double {
        size = mem as size_t;
    }
    return if size
        > (nmerge as libc::c_ulong)
            .wrapping_mul(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(::core::mem::size_of::<line>() as libc::c_ulong),
            )
    {
        size
    } else {
        (nmerge as libc::c_ulong)
            .wrapping_mul(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(::core::mem::size_of::<line>() as libc::c_ulong),
            )
    };
}
unsafe extern "C" fn sort_buffer_size(
    mut fps: *const *mut FILE,
    mut nfps: size_t,
    mut files: *const *mut libc::c_char,
    mut nfiles: size_t,
    mut line_bytes: size_t,
) -> size_t {
    static mut size_bound: size_t = 0;
    let mut worst_case_per_input_byte: size_t = line_bytes
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut size: size_t = worst_case_per_input_byte
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < nfiles {
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
        let mut file_size: off_t = 0;
        let mut worst_case: size_t = 0;
        if (if i < nfps {
            fstat(fileno(*fps.offset(i as isize)), &mut st)
        } else {
            (if strcmp(
                *files.offset(i as isize),
                b"-\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                fstat(0 as libc::c_int, &mut st)
            } else {
                stat(*files.offset(i as isize), &mut st)
            })
        }) != 0 as libc::c_int
        {
            sort_die(
                gettext(b"stat failed\0" as *const u8 as *const libc::c_char),
                *files.offset(i as isize),
            );
        }
        if usable_st_size(&mut st) as libc::c_int != 0
            && (0 as libc::c_int as libc::c_long) < st.st_size
        {
            file_size = st.st_size;
        } else {
            if sort_size != 0 {
                return sort_size;
            }
            file_size = (128 as libc::c_int * 1024 as libc::c_int) as off_t;
        }
        if size_bound == 0 {
            size_bound = sort_size;
            if size_bound == 0 {
                size_bound = default_sort_size();
            }
        }
        worst_case = (file_size as libc::c_ulong)
            .wrapping_mul(worst_case_per_input_byte)
            .wrapping_add(1 as libc::c_int as libc::c_ulong);
        if file_size as libc::c_ulong
            != worst_case.wrapping_div(worst_case_per_input_byte)
            || size_bound.wrapping_sub(size) <= worst_case
        {
            return size_bound;
        }
        size = (size as libc::c_ulong).wrapping_add(worst_case) as size_t as size_t;
        i = i.wrapping_add(1);
        i;
    }
    return size;
}
unsafe extern "C" fn initbuf(
    mut buf: *mut buffer,
    mut line_bytes: size_t,
    mut alloc: size_t,
) {
    loop {
        alloc = (alloc as libc::c_ulong)
            .wrapping_add(
                (::core::mem::size_of::<line>() as libc::c_ulong)
                    .wrapping_sub(
                        alloc
                            .wrapping_rem(
                                ::core::mem::size_of::<line>() as libc::c_ulong,
                            ),
                    ),
            ) as size_t as size_t;
        (*buf).buf = malloc(alloc) as *mut libc::c_char;
        if !((*buf).buf).is_null() {
            break;
        }
        alloc = (alloc as libc::c_ulong).wrapping_div(2 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
        if alloc <= line_bytes.wrapping_add(1 as libc::c_int as libc::c_ulong) {
            xalloc_die();
        }
    }
    (*buf).line_bytes = line_bytes;
    (*buf).alloc = alloc;
    (*buf).nlines = 0 as libc::c_int as size_t;
    (*buf).left = (*buf).nlines;
    (*buf).used = (*buf).left;
    (*buf).eof = 0 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn buffer_linelim(mut buf: *const buffer) -> *mut line {
    let mut linelim: *mut libc::c_void = ((*buf).buf).offset((*buf).alloc as isize)
        as *mut libc::c_void;
    return linelim as *mut line;
}
unsafe extern "C" fn begfield(
    mut line: *const line,
    mut key: *const keyfield,
) -> *mut libc::c_char {
    let mut ptr: *mut libc::c_char = (*line).text;
    let mut lim: *mut libc::c_char = ptr
        .offset((*line).length as isize)
        .offset(-(1 as libc::c_int as isize));
    let mut sword: size_t = (*key).sword;
    let mut schar: size_t = (*key).schar;
    if tab != TAB_DEFAULT as libc::c_int {
        while ptr < lim
            && {
                let fresh4 = sword;
                sword = sword.wrapping_sub(1);
                fresh4 != 0
            }
        {
            while ptr < lim && *ptr as libc::c_int != tab {
                ptr = ptr.offset(1);
                ptr;
            }
            if ptr < lim {
                ptr = ptr.offset(1);
                ptr;
            }
        }
    } else {
        while ptr < lim
            && {
                let fresh5 = sword;
                sword = sword.wrapping_sub(1);
                fresh5 != 0
            }
        {
            while ptr < lim && blanks[to_uchar(*ptr) as usize] as libc::c_int != 0 {
                ptr = ptr.offset(1);
                ptr;
            }
            while ptr < lim && !blanks[to_uchar(*ptr) as usize] {
                ptr = ptr.offset(1);
                ptr;
            }
        }
    }
    if (*key).skipsblanks {
        while ptr < lim && blanks[to_uchar(*ptr) as usize] as libc::c_int != 0 {
            ptr = ptr.offset(1);
            ptr;
        }
    }
    ptr = if lim < ptr.offset(schar as isize) {
        lim
    } else {
        ptr.offset(schar as isize)
    };
    return ptr;
}
unsafe extern "C" fn limfield(
    mut line: *const line,
    mut key: *const keyfield,
) -> *mut libc::c_char {
    let mut ptr: *mut libc::c_char = (*line).text;
    let mut lim: *mut libc::c_char = ptr
        .offset((*line).length as isize)
        .offset(-(1 as libc::c_int as isize));
    let mut eword: size_t = (*key).eword;
    let mut echar: size_t = (*key).echar;
    if echar == 0 as libc::c_int as libc::c_ulong {
        eword = eword.wrapping_add(1);
        eword;
    }
    if tab != TAB_DEFAULT as libc::c_int {
        while ptr < lim
            && {
                let fresh6 = eword;
                eword = eword.wrapping_sub(1);
                fresh6 != 0
            }
        {
            while ptr < lim && *ptr as libc::c_int != tab {
                ptr = ptr.offset(1);
                ptr;
            }
            if ptr < lim && (eword != 0 || echar != 0) {
                ptr = ptr.offset(1);
                ptr;
            }
        }
    } else {
        while ptr < lim
            && {
                let fresh7 = eword;
                eword = eword.wrapping_sub(1);
                fresh7 != 0
            }
        {
            while ptr < lim && blanks[to_uchar(*ptr) as usize] as libc::c_int != 0 {
                ptr = ptr.offset(1);
                ptr;
            }
            while ptr < lim && !blanks[to_uchar(*ptr) as usize] {
                ptr = ptr.offset(1);
                ptr;
            }
        }
    }
    if echar != 0 as libc::c_int as libc::c_ulong {
        if (*key).skipeblanks {
            while ptr < lim && blanks[to_uchar(*ptr) as usize] as libc::c_int != 0 {
                ptr = ptr.offset(1);
                ptr;
            }
        }
        ptr = if lim < ptr.offset(echar as isize) {
            lim
        } else {
            ptr.offset(echar as isize)
        };
    }
    return ptr;
}
unsafe extern "C" fn fillbuf(
    mut buf: *mut buffer,
    mut fp: *mut FILE,
    mut file: *const libc::c_char,
) -> bool {
    let mut key: *const keyfield = keylist;
    let mut eol: libc::c_char = eolchar;
    let mut line_bytes: size_t = (*buf).line_bytes;
    let mut mergesize: size_t = merge_buffer_size
        .wrapping_sub(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_add(::core::mem::size_of::<line>() as libc::c_ulong),
        );
    if (*buf).eof {
        return 0 as libc::c_int != 0;
    }
    if (*buf).used != (*buf).left {
        memmove(
            (*buf).buf as *mut libc::c_void,
            ((*buf).buf).offset((*buf).used as isize).offset(-((*buf).left as isize))
                as *const libc::c_void,
            (*buf).left,
        );
        (*buf).used = (*buf).left;
        (*buf).nlines = 0 as libc::c_int as size_t;
    }
    loop {
        let mut ptr: *mut libc::c_char = ((*buf).buf).offset((*buf).used as isize);
        let mut linelim: *mut line = buffer_linelim(buf);
        let mut line: *mut line = linelim.offset(-((*buf).nlines as isize));
        let mut avail: size_t = (linelim as *mut libc::c_char)
            .offset(-(((*buf).nlines).wrapping_mul(line_bytes) as isize))
            .offset_from(ptr) as libc::c_long as size_t;
        let mut line_start: *mut libc::c_char = if (*buf).nlines != 0 {
            ((*line).text).offset((*line).length as isize)
        } else {
            (*buf).buf
        };
        while line_bytes.wrapping_add(1 as libc::c_int as libc::c_ulong) < avail {
            let mut readsize: size_t = avail
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_div(
                    line_bytes.wrapping_add(1 as libc::c_int as libc::c_ulong),
                );
            let mut bytes_read: size_t = fread_unlocked(
                ptr as *mut libc::c_void,
                1 as libc::c_int as size_t,
                readsize,
                fp,
            );
            let mut ptrlim: *mut libc::c_char = ptr.offset(bytes_read as isize);
            let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
            avail = (avail as libc::c_ulong).wrapping_sub(bytes_read) as size_t
                as size_t;
            if bytes_read != readsize {
                if ferror_unlocked(fp) != 0 {
                    sort_die(
                        gettext(b"read failed\0" as *const u8 as *const libc::c_char),
                        file,
                    );
                }
                if feof_unlocked(fp) != 0 {
                    (*buf).eof = 1 as libc::c_int != 0;
                    if (*buf).buf == ptrlim {
                        return 0 as libc::c_int != 0;
                    }
                    if line_start != ptrlim
                        && *ptrlim.offset(-(1 as libc::c_int) as isize) as libc::c_int
                            != eol as libc::c_int
                    {
                        let fresh8 = ptrlim;
                        ptrlim = ptrlim.offset(1);
                        *fresh8 = eol;
                    }
                }
            }
            loop {
                p = memchr(
                    ptr as *const libc::c_void,
                    eol as libc::c_int,
                    ptrlim.offset_from(ptr) as libc::c_long as libc::c_ulong,
                ) as *mut libc::c_char;
                if p.is_null() {
                    break;
                }
                *p = '\0' as i32 as libc::c_char;
                ptr = p.offset(1 as libc::c_int as isize);
                line = line.offset(-1);
                line;
                (*line).text = line_start;
                (*line).length = ptr.offset_from(line_start) as libc::c_long as size_t;
                mergesize = if mergesize > (*line).length {
                    mergesize
                } else {
                    (*line).length
                };
                avail = (avail as libc::c_ulong).wrapping_sub(line_bytes) as size_t
                    as size_t;
                if !key.is_null() {
                    (*line)
                        .keylim = if (*key).eword
                        == 18446744073709551615 as libc::c_ulong
                    {
                        p
                    } else {
                        limfield(line, key)
                    };
                    if (*key).sword != 18446744073709551615 as libc::c_ulong {
                        (*line).keybeg = begfield(line, key);
                    } else {
                        if (*key).skipsblanks {
                            while blanks[to_uchar(*line_start) as usize] {
                                line_start = line_start.offset(1);
                                line_start;
                            }
                        }
                        (*line).keybeg = line_start;
                    }
                }
                line_start = ptr;
            }
            ptr = ptrlim;
            if (*buf).eof {
                break;
            }
        }
        (*buf).used = ptr.offset_from((*buf).buf) as libc::c_long as size_t;
        (*buf)
            .nlines = (buffer_linelim(buf)).offset_from(line) as libc::c_long as size_t;
        if (*buf).nlines != 0 as libc::c_int as libc::c_ulong {
            (*buf).left = ptr.offset_from(line_start) as libc::c_long as size_t;
            merge_buffer_size = mergesize
                .wrapping_add(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_add(::core::mem::size_of::<line>() as libc::c_ulong),
                );
            return 1 as libc::c_int != 0;
        }
        let mut line_alloc: size_t = ((*buf).alloc)
            .wrapping_div(::core::mem::size_of::<line>() as libc::c_ulong);
        (*buf)
            .buf = x2nrealloc(
            (*buf).buf as *mut libc::c_void,
            &mut line_alloc,
            ::core::mem::size_of::<line>() as libc::c_ulong,
        ) as *mut libc::c_char;
        (*buf)
            .alloc = line_alloc
            .wrapping_mul(::core::mem::size_of::<line>() as libc::c_ulong);
    };
}
static mut unit_order: [libc::c_char; 256] = [
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    6 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    2 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    5 as libc::c_int as libc::c_char,
    10 as libc::c_int as libc::c_char,
    9 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    4 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    8 as libc::c_int as libc::c_char,
    7 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
];
unsafe extern "C" fn traverse_raw_number(
    mut number: *mut *const libc::c_char,
) -> libc::c_char {
    let mut p: *const libc::c_char = *number;
    let mut ch: libc::c_char = 0;
    let mut max_digit: libc::c_char = '\0' as i32 as libc::c_char;
    let mut ends_with_thousands_sep: bool = 0 as libc::c_int != 0;
    loop {
        let fresh9 = p;
        p = p.offset(1);
        ch = *fresh9;
        if !((ch as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
            <= 9 as libc::c_int as libc::c_uint)
        {
            break;
        }
        if (max_digit as libc::c_int) < ch as libc::c_int {
            max_digit = ch;
        }
        ends_with_thousands_sep = *p as libc::c_int == thousands_sep;
        if ends_with_thousands_sep {
            p = p.offset(1);
            p;
        }
    }
    if ends_with_thousands_sep {
        *number = p.offset(-(2 as libc::c_int as isize));
        return max_digit;
    }
    if ch as libc::c_int == decimal_point as libc::c_int {
        loop {
            let fresh10 = p;
            p = p.offset(1);
            ch = *fresh10;
            if !((ch as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
                <= 9 as libc::c_int as libc::c_uint)
            {
                break;
            }
            if (max_digit as libc::c_int) < ch as libc::c_int {
                max_digit = ch;
            }
        }
    }
    *number = p.offset(-(1 as libc::c_int as isize));
    return max_digit;
}
unsafe extern "C" fn find_unit_order(mut number: *const libc::c_char) -> libc::c_int {
    let mut minus_sign: bool = *number as libc::c_int == '-' as i32;
    let mut p: *const libc::c_char = number.offset(minus_sign as libc::c_int as isize);
    let mut max_digit: libc::c_char = traverse_raw_number(&mut p);
    if ('0' as i32) < max_digit as libc::c_int {
        let mut ch: libc::c_uchar = *p as libc::c_uchar;
        let mut order: libc::c_int = unit_order[ch as usize] as libc::c_int;
        return if minus_sign as libc::c_int != 0 { -order } else { order };
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn human_numcompare(
    mut a: *const libc::c_char,
    mut b: *const libc::c_char,
) -> libc::c_int {
    while blanks[to_uchar(*a) as usize] {
        a = a.offset(1);
        a;
    }
    while blanks[to_uchar(*b) as usize] {
        b = b.offset(1);
        b;
    }
    let mut diff: libc::c_int = find_unit_order(a) - find_unit_order(b);
    return if diff != 0 {
        diff
    } else {
        strnumcmp(a, b, decimal_point as libc::c_int, thousands_sep)
    };
}
unsafe extern "C" fn numcompare(
    mut a: *const libc::c_char,
    mut b: *const libc::c_char,
) -> libc::c_int {
    while blanks[to_uchar(*a) as usize] {
        a = a.offset(1);
        a;
    }
    while blanks[to_uchar(*b) as usize] {
        b = b.offset(1);
        b;
    }
    return strnumcmp(a, b, decimal_point as libc::c_int, thousands_sep);
}
unsafe extern "C" fn nan_compare(mut a: f128::f128, mut b: f128::f128) -> libc::c_int {
    let mut buf: [[libc::c_char; 135]; 2] = [[0; 135]; 2];
    snprintf(
        (buf[0 as libc::c_int as usize]).as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 135]>() as libc::c_ulong,
        b"%Lf\0" as *const u8 as *const libc::c_char,
        a,
    );
    snprintf(
        (buf[1 as libc::c_int as usize]).as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 135]>() as libc::c_ulong,
        b"%Lf\0" as *const u8 as *const libc::c_char,
        b,
    );
    return strcmp(
        (buf[0 as libc::c_int as usize]).as_mut_ptr(),
        (buf[1 as libc::c_int as usize]).as_mut_ptr(),
    );
}
unsafe extern "C" fn general_numcompare(
    mut sa: *const libc::c_char,
    mut sb: *const libc::c_char,
) -> libc::c_int {
    let mut ea: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut eb: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut a: f128::f128 = strtold(sa, &mut ea);
    let mut b: f128::f128 = strtold(sb, &mut eb);
    if sa == ea as *const libc::c_char {
        return if sb == eb as *const libc::c_char {
            0 as libc::c_int
        } else {
            -(1 as libc::c_int)
        };
    }
    if sb == eb as *const libc::c_char {
        return 1 as libc::c_int;
    }
    return if a < b {
        -(1 as libc::c_int)
    } else if a > b {
        1 as libc::c_int
    } else if a == b {
        0 as libc::c_int
    } else if b == b {
        -(1 as libc::c_int)
    } else if a == a {
        1 as libc::c_int
    } else {
        nan_compare(a, b)
    };
}
unsafe extern "C" fn getmonth(
    mut month: *const libc::c_char,
    mut ea: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut lo: size_t = 0 as libc::c_int as size_t;
    let mut hi: size_t = 12 as libc::c_int as size_t;
    while blanks[to_uchar(*month) as usize] {
        month = month.offset(1);
        month;
    }
    loop {
        let mut ix: size_t = lo
            .wrapping_add(hi)
            .wrapping_div(2 as libc::c_int as libc::c_ulong);
        let mut m: *const libc::c_char = month;
        let mut n: *const libc::c_char = monthtab[ix as usize].name;
        loop {
            if *n == 0 {
                if !ea.is_null() {
                    *ea = m as *mut libc::c_char;
                }
                return monthtab[ix as usize].val;
            }
            if (to_uchar(fold_toupper[to_uchar(*m) as usize]) as libc::c_int)
                < to_uchar(*n) as libc::c_int
            {
                hi = ix;
                break;
            } else if to_uchar(fold_toupper[to_uchar(*m) as usize]) as libc::c_int
                > to_uchar(*n) as libc::c_int
            {
                lo = ix.wrapping_add(1 as libc::c_int as libc::c_ulong);
                break;
            } else {
                m = m.offset(1);
                m;
                n = n.offset(1);
                n;
            }
        }
        if !(lo < hi) {
            break;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub static mut ptr_MD5_Init: Option::<
    unsafe extern "C" fn(*mut MD5_CTX) -> libc::c_int,
> = None;
#[no_mangle]
pub static mut ptr_MD5_Update: Option::<
    unsafe extern "C" fn(*mut MD5_CTX, *const libc::c_void, size_t) -> libc::c_int,
> = None;
#[no_mangle]
pub static mut ptr_MD5_Final: Option::<
    unsafe extern "C" fn(*mut libc::c_uchar, *mut MD5_CTX) -> libc::c_int,
> = None;
#[inline]
unsafe extern "C" fn md5_init_ctx(mut ctx: *mut md5_ctx) {
    (Some(ptr_MD5_Init.expect("non-null function pointer")))
        .expect("non-null function pointer")(ctx as *mut MD5_CTX);
}
#[inline]
unsafe extern "C" fn md5_process_bytes(
    mut buf: *const libc::c_void,
    mut len: size_t,
    mut ctx: *mut md5_ctx,
) {
    (Some(ptr_MD5_Update.expect("non-null function pointer")))
        .expect("non-null function pointer")(ctx as *mut MD5_CTX, buf, len);
}
#[inline]
unsafe extern "C" fn md5_finish_ctx(
    mut ctx: *mut md5_ctx,
    mut res: *mut libc::c_void,
) -> *mut libc::c_void {
    (Some(ptr_MD5_Final.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(res as *mut libc::c_uchar, ctx as *mut MD5_CTX);
    return res;
}
unsafe extern "C" fn link_failure() {
    if 0 != 0 {
        error(
            SORT_FAILURE as libc::c_int,
            0 as libc::c_int,
            b"%s\0" as *const u8 as *const libc::c_char,
            dlerror(),
        );
        if SORT_FAILURE as libc::c_int != 0 as libc::c_int {
            unreachable!();
        } else {};
    } else {
        ({
            let __errstatus: libc::c_int = SORT_FAILURE as libc::c_int;
            error(
                __errstatus,
                0 as libc::c_int,
                b"%s\0" as *const u8 as *const libc::c_char,
                dlerror(),
            );
            if __errstatus != 0 as libc::c_int {
                unreachable!();
            } else {};
            
        });
        ({
            let __errstatus: libc::c_int = SORT_FAILURE as libc::c_int;
            error(
                __errstatus,
                0 as libc::c_int,
                b"%s\0" as *const u8 as *const libc::c_char,
                dlerror(),
            );
            if __errstatus != 0 as libc::c_int {
                unreachable!();
            } else {};
            
        });
    };
}
unsafe extern "C" fn symbol_address(
    mut handle: *mut libc::c_void,
    mut symbol: *const libc::c_char,
) -> *mut libc::c_void {
    let mut address: *mut libc::c_void = dlsym(handle, symbol);
    if address.is_null() {
        link_failure();
    }
    return address;
}
unsafe extern "C" fn link_libcrypto() {
    let mut handle: *mut libc::c_void = dlopen(
        b"libcrypto.so.3\0" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int | 0x100 as libc::c_int,
    );
    if handle.is_null() {
        link_failure();
    }
    ptr_MD5_Init = ::core::mem::transmute::<
        *mut libc::c_void,
        Option::<unsafe extern "C" fn(*mut MD5_CTX) -> libc::c_int>,
    >(symbol_address(handle, b"MD5_Init\0" as *const u8 as *const libc::c_char));
    ptr_MD5_Update = ::core::mem::transmute::<
        *mut libc::c_void,
        Option::<
            unsafe extern "C" fn(
                *mut MD5_CTX,
                *const libc::c_void,
                size_t,
            ) -> libc::c_int,
        >,
    >(symbol_address(handle, b"MD5_Update\0" as *const u8 as *const libc::c_char));
    ptr_MD5_Final = ::core::mem::transmute::<
        *mut libc::c_void,
        Option::<unsafe extern "C" fn(*mut libc::c_uchar, *mut MD5_CTX) -> libc::c_int>,
    >(symbol_address(handle, b"MD5_Final\0" as *const u8 as *const libc::c_char));
}
static mut random_md5_state: md5_ctx = md5_ctx {
    CTX: MD5_CTX {
        A: 0,
        B: 0,
        C: 0,
        D: 0,
        Nl: 0,
        Nh: 0,
        data: [0; 16],
        num: 0,
    },
};
unsafe extern "C" fn random_md5_state_init(mut random_source: *const libc::c_char) {
    let mut buf: [libc::c_uchar; 16] = [0; 16];
    let mut r: *mut randread_source = randread_new(
        random_source,
        ::core::mem::size_of::<[libc::c_uchar; 16]>() as libc::c_ulong,
    );
    if r.is_null() {
        sort_die(
            gettext(b"open failed\0" as *const u8 as *const libc::c_char),
            if !random_source.is_null() {
                random_source
            } else {
                b"getrandom\0" as *const u8 as *const libc::c_char
            },
        );
    }
    randread(
        r,
        buf.as_mut_ptr() as *mut libc::c_void,
        ::core::mem::size_of::<[libc::c_uchar; 16]>() as libc::c_ulong,
    );
    if randread_free(r) != 0 as libc::c_int {
        sort_die(
            gettext(b"close failed\0" as *const u8 as *const libc::c_char),
            random_source,
        );
    }
    link_libcrypto();
    md5_init_ctx(&mut random_md5_state);
    md5_process_bytes(
        buf.as_mut_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[libc::c_uchar; 16]>() as libc::c_ulong,
        &mut random_md5_state,
    );
}
unsafe extern "C" fn xstrxfrm(
    mut dest: *mut libc::c_char,
    mut src: *const libc::c_char,
    mut destsize: size_t,
) -> size_t {
    *__errno_location() = 0 as libc::c_int;
    let mut translated_size: size_t = strxfrm(dest, src, destsize);
    if *__errno_location() != 0 {
        if 0 != 0 {
            error(
                0 as libc::c_int,
                *__errno_location(),
                gettext(
                    b"string transformation failed\0" as *const u8 as *const libc::c_char,
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
                        b"string transformation failed\0" as *const u8
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
                    *__errno_location(),
                    gettext(
                        b"string transformation failed\0" as *const u8
                            as *const libc::c_char,
                    ),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
        if 0 != 0 {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"set LC_ALL='C' to work around the problem\0" as *const u8
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
                        b"set LC_ALL='C' to work around the problem\0" as *const u8
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
                        b"set LC_ALL='C' to work around the problem\0" as *const u8
                            as *const libc::c_char,
                    ),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
        if 0 != 0 {
            error(
                SORT_FAILURE as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"the original string was %s\0" as *const u8 as *const libc::c_char,
                ),
                quotearg_n_style(0 as libc::c_int, locale_quoting_style, src),
            );
            if SORT_FAILURE as libc::c_int != 0 as libc::c_int {
                unreachable!();
            } else {};
        } else {
            ({
                let __errstatus: libc::c_int = SORT_FAILURE as libc::c_int;
                error(
                    __errstatus,
                    0 as libc::c_int,
                    gettext(
                        b"the original string was %s\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quotearg_n_style(0 as libc::c_int, locale_quoting_style, src),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
            ({
                let __errstatus: libc::c_int = SORT_FAILURE as libc::c_int;
                error(
                    __errstatus,
                    0 as libc::c_int,
                    gettext(
                        b"the original string was %s\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quotearg_n_style(0 as libc::c_int, locale_quoting_style, src),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
    }
    return translated_size;
}
unsafe extern "C" fn compare_random(
    mut texta: *mut libc::c_char,
    mut lena: size_t,
    mut textb: *mut libc::c_char,
    mut lenb: size_t,
) -> libc::c_int {
    let mut xfrm_diff: libc::c_int = 0 as libc::c_int;
    let mut stackbuf: [libc::c_char; 4000] = [0; 4000];
    let mut buf: *mut libc::c_char = stackbuf.as_mut_ptr();
    let mut bufsize: size_t = ::core::mem::size_of::<[libc::c_char; 4000]>()
        as libc::c_ulong;
    let mut allocated: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut dig: [[uint32_t; 4]; 2] = [[0; 4]; 2];
    let mut s: [md5_ctx; 2] = [md5_ctx {
        CTX: MD5_CTX {
            A: 0,
            B: 0,
            C: 0,
            D: 0,
            Nl: 0,
            Nh: 0,
            data: [0; 16],
            num: 0,
        },
    }; 2];
    s[1 as libc::c_int as usize] = random_md5_state;
    s[0 as libc::c_int as usize] = s[1 as libc::c_int as usize];
    if hard_LC_COLLATE {
        let mut lima: *const libc::c_char = texta.offset(lena as isize);
        let mut limb: *const libc::c_char = textb.offset(lenb as isize);
        loop {
            let mut guess_bufsize: size_t = (3 as libc::c_int as libc::c_ulong)
                .wrapping_mul(lena.wrapping_add(lenb))
                .wrapping_add(2 as libc::c_int as libc::c_ulong);
            if bufsize < guess_bufsize {
                bufsize = if guess_bufsize
                    > bufsize
                        .wrapping_mul(3 as libc::c_int as libc::c_ulong)
                        .wrapping_div(2 as libc::c_int as libc::c_ulong)
                {
                    guess_bufsize
                } else {
                    bufsize
                        .wrapping_mul(3 as libc::c_int as libc::c_ulong)
                        .wrapping_div(2 as libc::c_int as libc::c_ulong)
                };
                free(allocated);
                allocated = malloc(bufsize);
                buf = allocated as *mut libc::c_char;
                if buf.is_null() {
                    buf = stackbuf.as_mut_ptr();
                    bufsize = ::core::mem::size_of::<[libc::c_char; 4000]>()
                        as libc::c_ulong;
                }
            }
            let mut sizea: size_t = if texta < lima as *mut libc::c_char {
                (xstrxfrm(buf, texta, bufsize))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            };
            let mut a_fits: bool = sizea <= bufsize;
            let mut sizeb: size_t = if textb < limb as *mut libc::c_char {
                (xstrxfrm(
                    (if a_fits as libc::c_int != 0 {
                        buf.offset(sizea as isize)
                    } else {
                        0 as *mut libc::c_char
                    }),
                    textb,
                    (if a_fits as libc::c_int != 0 {
                        bufsize.wrapping_sub(sizea)
                    } else {
                        0 as libc::c_int as libc::c_ulong
                    }),
                ))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            };
            if !(a_fits as libc::c_int != 0 && sizea.wrapping_add(sizeb) <= bufsize) {
                bufsize = sizea.wrapping_add(sizeb);
                if bufsize
                    < (18446744073709551615 as libc::c_ulong)
                        .wrapping_div(3 as libc::c_int as libc::c_ulong)
                {
                    bufsize = bufsize
                        .wrapping_mul(3 as libc::c_int as libc::c_ulong)
                        .wrapping_div(2 as libc::c_int as libc::c_ulong);
                }
                free(allocated);
                allocated = xmalloc(bufsize);
                buf = allocated as *mut libc::c_char;
                if texta < lima as *mut libc::c_char {
                    strxfrm(buf, texta, sizea);
                }
                if textb < limb as *mut libc::c_char {
                    strxfrm(buf.offset(sizea as isize), textb, sizeb);
                }
            }
            if texta < lima as *mut libc::c_char {
                texta = texta
                    .offset(
                        (strlen(texta)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                            as isize,
                    );
            }
            if textb < limb as *mut libc::c_char {
                textb = textb
                    .offset(
                        (strlen(textb)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                            as isize,
                    );
            }
            if !(texta < lima as *mut libc::c_char || textb < limb as *mut libc::c_char)
            {
                lena = sizea;
                texta = buf;
                lenb = sizeb;
                textb = buf.offset(sizea as isize);
                break;
            } else {
                md5_process_bytes(
                    buf as *const libc::c_void,
                    sizea,
                    &mut *s.as_mut_ptr().offset(0 as libc::c_int as isize),
                );
                md5_process_bytes(
                    buf.offset(sizea as isize) as *const libc::c_void,
                    sizeb,
                    &mut *s.as_mut_ptr().offset(1 as libc::c_int as isize),
                );
                if xfrm_diff == 0 {
                    xfrm_diff = memcmp(
                        buf as *const libc::c_void,
                        buf.offset(sizea as isize) as *const libc::c_void,
                        if sizea < sizeb { sizea } else { sizeb },
                    );
                    if xfrm_diff == 0 {
                        xfrm_diff = (sizea > sizeb) as libc::c_int
                            - (sizea < sizeb) as libc::c_int;
                    }
                }
            }
        }
    }
    md5_process_bytes(
        texta as *const libc::c_void,
        lena,
        &mut *s.as_mut_ptr().offset(0 as libc::c_int as isize),
    );
    md5_finish_ctx(
        &mut *s.as_mut_ptr().offset(0 as libc::c_int as isize),
        (dig[0 as libc::c_int as usize]).as_mut_ptr() as *mut libc::c_void,
    );
    md5_process_bytes(
        textb as *const libc::c_void,
        lenb,
        &mut *s.as_mut_ptr().offset(1 as libc::c_int as isize),
    );
    md5_finish_ctx(
        &mut *s.as_mut_ptr().offset(1 as libc::c_int as isize),
        (dig[1 as libc::c_int as usize]).as_mut_ptr() as *mut libc::c_void,
    );
    let mut diff: libc::c_int = memcmp(
        (dig[0 as libc::c_int as usize]).as_mut_ptr() as *const libc::c_void,
        (dig[1 as libc::c_int as usize]).as_mut_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[uint32_t; 4]>() as libc::c_ulong,
    );
    if diff == 0 {
        if xfrm_diff == 0 {
            xfrm_diff = memcmp(
                texta as *const libc::c_void,
                textb as *const libc::c_void,
                if lena < lenb { lena } else { lenb },
            );
            if xfrm_diff == 0 {
                xfrm_diff = (lena > lenb) as libc::c_int - (lena < lenb) as libc::c_int;
            }
        }
        diff = xfrm_diff;
    }
    free(allocated);
    return diff;
}
unsafe extern "C" fn debug_width(
    mut text: *const libc::c_char,
    mut lim: *const libc::c_char,
) -> size_t {
    let mut width: size_t = mbsnwidth(
        text,
        lim.offset_from(text) as libc::c_long as size_t,
        0 as libc::c_int,
    ) as size_t;
    while text < lim {
        let fresh11 = text;
        text = text.offset(1);
        width = (width as libc::c_ulong)
            .wrapping_add(
                (*fresh11 as libc::c_int == '\t' as i32) as libc::c_int as libc::c_ulong,
            ) as size_t as size_t;
    }
    return width;
}
unsafe extern "C" fn mark_key(mut offset: size_t, mut width: size_t) {
    loop {
        let fresh12 = offset;
        offset = offset.wrapping_sub(1);
        if !(fresh12 != 0) {
            break;
        }
        putchar_unlocked(' ' as i32);
    }
    if width == 0 {
        printf(gettext(b"^ no match for key\n\0" as *const u8 as *const libc::c_char));
    } else {
        loop {
            putchar_unlocked('_' as i32);
            width = width.wrapping_sub(1);
            if !(width != 0) {
                break;
            }
        }
        putchar_unlocked('\n' as i32);
    };
}
#[inline]
unsafe extern "C" fn key_numeric(mut key: *const keyfield) -> bool {
    return (*key).numeric as libc::c_int != 0
        || (*key).general_numeric as libc::c_int != 0
        || (*key).human_numeric as libc::c_int != 0;
}
unsafe extern "C" fn debug_key(mut line: *const line, mut key: *const keyfield) {
    let mut text: *mut libc::c_char = (*line).text;
    let mut beg: *mut libc::c_char = text;
    let mut lim: *mut libc::c_char = text
        .offset((*line).length as isize)
        .offset(-(1 as libc::c_int as isize));
    if !key.is_null() {
        if (*key).sword != 18446744073709551615 as libc::c_ulong {
            beg = begfield(line, key);
        }
        if (*key).eword != 18446744073709551615 as libc::c_ulong {
            lim = limfield(line, key);
        }
        if (*key).skipsblanks as libc::c_int != 0
            && (*key).sword == 18446744073709551615 as libc::c_ulong
            || (*key).month as libc::c_int != 0 || key_numeric(key) as libc::c_int != 0
        {
            let mut saved: libc::c_char = *lim;
            *lim = '\0' as i32 as libc::c_char;
            while blanks[to_uchar(*beg) as usize] {
                beg = beg.offset(1);
                beg;
            }
            let mut tighter_lim: *mut libc::c_char = beg;
            if lim < beg {
                tighter_lim = lim;
            } else if (*key).month {
                getmonth(beg, &mut tighter_lim);
            } else if (*key).general_numeric {
                strtold(beg, &mut tighter_lim);
            } else if (*key).numeric as libc::c_int != 0
                || (*key).human_numeric as libc::c_int != 0
            {
                let mut p: *const libc::c_char = beg
                    .offset(
                        (beg < lim && *beg as libc::c_int == '-' as i32) as libc::c_int
                            as isize,
                    );
                let mut max_digit: libc::c_char = traverse_raw_number(&mut p);
                if '0' as i32 <= max_digit as libc::c_int {
                    let mut ch: libc::c_uchar = *p as libc::c_uchar;
                    tighter_lim = (p as *mut libc::c_char)
                        .offset(
                            ((*key).human_numeric as libc::c_int != 0
                                && unit_order[ch as usize] as libc::c_int != 0)
                                as libc::c_int as isize,
                        );
                }
            } else {
                tighter_lim = lim;
            }
            *lim = saved;
            lim = tighter_lim;
        }
    }
    let mut offset: size_t = debug_width(text, beg);
    let mut width: size_t = debug_width(beg, lim);
    mark_key(offset, width);
}
unsafe extern "C" fn debug_line(mut line: *const line) {
    let mut key: *const keyfield = keylist;
    loop {
        debug_key(line, key);
        if !(!key.is_null()
            && {
                key = (*key).next;
                !key.is_null()
                    || !(unique as libc::c_int != 0 || stable as libc::c_int != 0)
            })
        {
            break;
        }
    };
}
unsafe extern "C" fn default_key_compare(mut key: *const keyfield) -> bool {
    return !(!((*key).ignore).is_null() || !((*key).translate).is_null()
        || (*key).skipsblanks as libc::c_int != 0
        || (*key).skipeblanks as libc::c_int != 0 || key_numeric(key) as libc::c_int != 0
        || (*key).month as libc::c_int != 0 || (*key).version as libc::c_int != 0
        || (*key).random as libc::c_int != 0);
}
unsafe extern "C" fn key_to_opts(mut key: *const keyfield, mut opts: *mut libc::c_char) {
    if (*key).skipsblanks as libc::c_int != 0 || (*key).skipeblanks as libc::c_int != 0 {
        let fresh13 = opts;
        opts = opts.offset(1);
        *fresh13 = 'b' as i32 as libc::c_char;
    }
    if (*key).ignore == nondictionary.as_mut_ptr() as *const bool {
        let fresh14 = opts;
        opts = opts.offset(1);
        *fresh14 = 'd' as i32 as libc::c_char;
    }
    if !((*key).translate).is_null() {
        let fresh15 = opts;
        opts = opts.offset(1);
        *fresh15 = 'f' as i32 as libc::c_char;
    }
    if (*key).general_numeric {
        let fresh16 = opts;
        opts = opts.offset(1);
        *fresh16 = 'g' as i32 as libc::c_char;
    }
    if (*key).human_numeric {
        let fresh17 = opts;
        opts = opts.offset(1);
        *fresh17 = 'h' as i32 as libc::c_char;
    }
    if (*key).ignore == nonprinting.as_mut_ptr() as *const bool {
        let fresh18 = opts;
        opts = opts.offset(1);
        *fresh18 = 'i' as i32 as libc::c_char;
    }
    if (*key).month {
        let fresh19 = opts;
        opts = opts.offset(1);
        *fresh19 = 'M' as i32 as libc::c_char;
    }
    if (*key).numeric {
        let fresh20 = opts;
        opts = opts.offset(1);
        *fresh20 = 'n' as i32 as libc::c_char;
    }
    if (*key).random {
        let fresh21 = opts;
        opts = opts.offset(1);
        *fresh21 = 'R' as i32 as libc::c_char;
    }
    if (*key).reverse {
        let fresh22 = opts;
        opts = opts.offset(1);
        *fresh22 = 'r' as i32 as libc::c_char;
    }
    if (*key).version {
        let fresh23 = opts;
        opts = opts.offset(1);
        *fresh23 = 'V' as i32 as libc::c_char;
    }
    *opts = '\0' as i32 as libc::c_char;
}
unsafe extern "C" fn key_warnings(mut gkey: *const keyfield, mut gkey_only: bool) {
    let mut key: *const keyfield = 0 as *const keyfield;
    let mut ugkey: keyfield = *gkey;
    let mut keynum: libc::c_ulong = 1 as libc::c_int as libc::c_ulong;
    let mut basic_numeric_field: bool = 0 as libc::c_int != 0;
    let mut general_numeric_field: bool = 0 as libc::c_int != 0;
    let mut basic_numeric_field_span: bool = 0 as libc::c_int != 0;
    let mut general_numeric_field_span: bool = 0 as libc::c_int != 0;
    key = keylist;
    while !key.is_null() {
        if key_numeric(key) {
            if (*key).general_numeric {
                general_numeric_field = 1 as libc::c_int != 0;
            } else {
                basic_numeric_field = 1 as libc::c_int != 0;
            }
        }
        if (*key).traditional_used {
            let mut sword: size_t = (*key).sword;
            let mut eword: size_t = (*key).eword;
            let mut tmp: [libc::c_char; 21] = [0; 21];
            let mut obuf: [libc::c_char; 46] = [0; 46];
            let mut nbuf: [libc::c_char; 47] = [0; 47];
            let mut po: *mut libc::c_char = obuf.as_mut_ptr();
            let mut pn: *mut libc::c_char = nbuf.as_mut_ptr();
            if sword == 18446744073709551615 as libc::c_ulong {
                sword = sword.wrapping_add(1);
                sword;
            }
            po = stpcpy(
                stpcpy(po, b"+\0" as *const u8 as *const libc::c_char),
                umaxtostr(sword, tmp.as_mut_ptr()),
            );
            pn = stpcpy(
                stpcpy(pn, b"-k \0" as *const u8 as *const libc::c_char),
                umaxtostr(
                    sword.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    tmp.as_mut_ptr(),
                ),
            );
            if (*key).eword != 18446744073709551615 as libc::c_ulong {
                stpcpy(
                    stpcpy(po, b" -\0" as *const u8 as *const libc::c_char),
                    umaxtostr(
                        eword.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        tmp.as_mut_ptr(),
                    ),
                );
                stpcpy(
                    stpcpy(pn, b",\0" as *const u8 as *const libc::c_char),
                    umaxtostr(
                        eword
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            .wrapping_add(
                                ((*key).echar == 18446744073709551615 as libc::c_ulong)
                                    as libc::c_int as libc::c_ulong,
                            ),
                        tmp.as_mut_ptr(),
                    ),
                );
            }
            if 0 != 0 {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"obsolescent key %s used; consider %s instead\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quote_n(0 as libc::c_int, obuf.as_mut_ptr()),
                    quote_n(1 as libc::c_int, nbuf.as_mut_ptr()),
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
                            b"obsolescent key %s used; consider %s instead\0"
                                as *const u8 as *const libc::c_char,
                        ),
                        quote_n(0 as libc::c_int, obuf.as_mut_ptr()),
                        quote_n(1 as libc::c_int, nbuf.as_mut_ptr()),
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
                            b"obsolescent key %s used; consider %s instead\0"
                                as *const u8 as *const libc::c_char,
                        ),
                        quote_n(0 as libc::c_int, obuf.as_mut_ptr()),
                        quote_n(1 as libc::c_int, nbuf.as_mut_ptr()),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
        let mut zero_width: bool = (*key).sword != 18446744073709551615 as libc::c_ulong
            && (*key).eword < (*key).sword;
        if zero_width {
            if 0 != 0 {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"key %lu has zero width and will be ignored\0" as *const u8
                            as *const libc::c_char,
                    ),
                    keynum,
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
                            b"key %lu has zero width and will be ignored\0" as *const u8
                                as *const libc::c_char,
                        ),
                        keynum,
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
                            b"key %lu has zero width and will be ignored\0" as *const u8
                                as *const libc::c_char,
                        ),
                        keynum,
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
        let mut implicit_skip: bool = key_numeric(key) as libc::c_int != 0
            || (*key).month as libc::c_int != 0;
        let mut line_offset: bool = (*key).eword == 0 as libc::c_int as libc::c_ulong
            && (*key).echar != 0 as libc::c_int as libc::c_ulong;
        if !zero_width && !gkey_only && tab == TAB_DEFAULT as libc::c_int && !line_offset
            && (!(*key).skipsblanks && !implicit_skip
                || !(*key).skipsblanks && (*key).schar != 0
                || !(*key).skipeblanks && (*key).echar != 0)
        {
            if 0 != 0 {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"leading blanks are significant in key %lu; consider also specifying 'b'\0"
                            as *const u8 as *const libc::c_char,
                    ),
                    keynum,
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
                            b"leading blanks are significant in key %lu; consider also specifying 'b'\0"
                                as *const u8 as *const libc::c_char,
                        ),
                        keynum,
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
                            b"leading blanks are significant in key %lu; consider also specifying 'b'\0"
                                as *const u8 as *const libc::c_char,
                        ),
                        keynum,
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
        if !gkey_only && key_numeric(key) as libc::c_int != 0 {
            let mut sword_0: size_t = ((*key).sword)
                .wrapping_add(1 as libc::c_int as libc::c_ulong);
            let mut eword_0: size_t = ((*key).eword)
                .wrapping_add(1 as libc::c_int as libc::c_ulong);
            if sword_0 == 0 {
                sword_0 = sword_0.wrapping_add(1);
                sword_0;
            }
            if eword_0 == 0 || sword_0 < eword_0 {
                if 0 != 0 {
                    error(
                        0 as libc::c_int,
                        0 as libc::c_int,
                        gettext(
                            b"key %lu is numeric and spans multiple fields\0"
                                as *const u8 as *const libc::c_char,
                        ),
                        keynum,
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
                                b"key %lu is numeric and spans multiple fields\0"
                                    as *const u8 as *const libc::c_char,
                            ),
                            keynum,
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
                                b"key %lu is numeric and spans multiple fields\0"
                                    as *const u8 as *const libc::c_char,
                            ),
                            keynum,
                        );
                        if __errstatus != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                        
                    });
                };
                if (*key).general_numeric {
                    general_numeric_field_span = 1 as libc::c_int != 0;
                } else {
                    basic_numeric_field_span = 1 as libc::c_int != 0;
                }
            }
        }
        if !(ugkey.ignore).is_null() && ugkey.ignore == (*key).ignore {
            ugkey.ignore = 0 as *const bool;
        }
        if !(ugkey.translate).is_null() && ugkey.translate == (*key).translate {
            ugkey.translate = 0 as *const libc::c_char;
        }
        ugkey
            .skipsblanks = (ugkey.skipsblanks as libc::c_int
            & !(*key).skipsblanks as libc::c_int) as bool;
        ugkey
            .skipeblanks = (ugkey.skipeblanks as libc::c_int
            & !(*key).skipeblanks as libc::c_int) as bool;
        ugkey
            .month = (ugkey.month as libc::c_int & !(*key).month as libc::c_int) as bool;
        ugkey
            .numeric = (ugkey.numeric as libc::c_int & !(*key).numeric as libc::c_int)
            as bool;
        ugkey
            .general_numeric = (ugkey.general_numeric as libc::c_int
            & !(*key).general_numeric as libc::c_int) as bool;
        ugkey
            .human_numeric = (ugkey.human_numeric as libc::c_int
            & !(*key).human_numeric as libc::c_int) as bool;
        ugkey
            .random = (ugkey.random as libc::c_int & !(*key).random as libc::c_int)
            as bool;
        ugkey
            .version = (ugkey.version as libc::c_int & !(*key).version as libc::c_int)
            as bool;
        ugkey
            .reverse = (ugkey.reverse as libc::c_int & !(*key).reverse as libc::c_int)
            as bool;
        key = (*key).next;
        keynum = keynum.wrapping_add(1);
        keynum;
    }
    let mut number_locale_warned: bool = 0 as libc::c_int != 0;
    if basic_numeric_field_span {
        if if tab == TAB_DEFAULT as libc::c_int {
            (thousands_sep != NON_CHAR as libc::c_int
                && *(*__ctype_b_loc())
                    .offset(
                        to_uchar(thousands_sep as libc::c_char) as libc::c_int as isize,
                    ) as libc::c_int
                    & _ISblank as libc::c_int as libc::c_ushort as libc::c_int != 0)
                as libc::c_int
        } else {
            (tab == thousands_sep) as libc::c_int
        } != 0
        {
            if 0 != 0 {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"field separator %s is treated as a group separator in numbers\0"
                            as *const u8 as *const libc::c_char,
                    ),
                    quote(
                        [thousands_sep as libc::c_char, 0 as libc::c_int as libc::c_char]
                            .as_mut_ptr(),
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
                            b"field separator %s is treated as a group separator in numbers\0"
                                as *const u8 as *const libc::c_char,
                        ),
                        quote(
                            [
                                thousands_sep as libc::c_char,
                                0 as libc::c_int as libc::c_char,
                            ]
                                .as_mut_ptr(),
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
                            b"field separator %s is treated as a group separator in numbers\0"
                                as *const u8 as *const libc::c_char,
                        ),
                        quote(
                            [
                                thousands_sep as libc::c_char,
                                0 as libc::c_int as libc::c_char,
                            ]
                                .as_mut_ptr(),
                        ),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
            number_locale_warned = 1 as libc::c_int != 0;
        }
    }
    if basic_numeric_field_span as libc::c_int != 0
        || general_numeric_field_span as libc::c_int != 0
    {
        if if tab == TAB_DEFAULT as libc::c_int {
            (thousands_sep != NON_CHAR as libc::c_int
                && *(*__ctype_b_loc())
                    .offset(to_uchar(decimal_point) as libc::c_int as isize)
                    as libc::c_int
                    & _ISblank as libc::c_int as libc::c_ushort as libc::c_int != 0)
                as libc::c_int
        } else {
            (tab == decimal_point as libc::c_int) as libc::c_int
        } != 0
        {
            if 0 != 0 {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"field separator %s is treated as a decimal point in numbers\0"
                            as *const u8 as *const libc::c_char,
                    ),
                    quote([decimal_point, 0 as libc::c_int as libc::c_char].as_mut_ptr()),
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
                            b"field separator %s is treated as a decimal point in numbers\0"
                                as *const u8 as *const libc::c_char,
                        ),
                        quote(
                            [decimal_point, 0 as libc::c_int as libc::c_char]
                                .as_mut_ptr(),
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
                            b"field separator %s is treated as a decimal point in numbers\0"
                                as *const u8 as *const libc::c_char,
                        ),
                        quote(
                            [decimal_point, 0 as libc::c_int as libc::c_char]
                                .as_mut_ptr(),
                        ),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
            number_locale_warned = 1 as libc::c_int != 0;
        } else if tab == '-' as i32 {
            if 0 != 0 {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"field separator %s is treated as a minus sign in numbers\0"
                            as *const u8 as *const libc::c_char,
                    ),
                    quote(
                        [tab as libc::c_char, 0 as libc::c_int as libc::c_char]
                            .as_mut_ptr(),
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
                            b"field separator %s is treated as a minus sign in numbers\0"
                                as *const u8 as *const libc::c_char,
                        ),
                        quote(
                            [tab as libc::c_char, 0 as libc::c_int as libc::c_char]
                                .as_mut_ptr(),
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
                            b"field separator %s is treated as a minus sign in numbers\0"
                                as *const u8 as *const libc::c_char,
                        ),
                        quote(
                            [tab as libc::c_char, 0 as libc::c_int as libc::c_char]
                                .as_mut_ptr(),
                        ),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        } else if general_numeric_field_span as libc::c_int != 0 && tab == '+' as i32 {
            if 0 != 0 {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"field separator %s is treated as a plus sign in numbers\0"
                            as *const u8 as *const libc::c_char,
                    ),
                    quote(
                        [tab as libc::c_char, 0 as libc::c_int as libc::c_char]
                            .as_mut_ptr(),
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
                            b"field separator %s is treated as a plus sign in numbers\0"
                                as *const u8 as *const libc::c_char,
                        ),
                        quote(
                            [tab as libc::c_char, 0 as libc::c_int as libc::c_char]
                                .as_mut_ptr(),
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
                            b"field separator %s is treated as a plus sign in numbers\0"
                                as *const u8 as *const libc::c_char,
                        ),
                        quote(
                            [tab as libc::c_char, 0 as libc::c_int as libc::c_char]
                                .as_mut_ptr(),
                        ),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
    }
    if (basic_numeric_field as libc::c_int != 0
        || general_numeric_field as libc::c_int != 0) && !number_locale_warned
    {
        if 0 != 0 {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"%snumbers use %s as a decimal point in this locale\0" as *const u8
                        as *const libc::c_char,
                ),
                (if tab == decimal_point as libc::c_int {
                    b"\0" as *const u8 as *const libc::c_char
                } else {
                    gettext(b"note \0" as *const u8 as *const libc::c_char)
                        as *const libc::c_char
                }),
                quote([decimal_point, 0 as libc::c_int as libc::c_char].as_mut_ptr()),
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
                        b"%snumbers use %s as a decimal point in this locale\0"
                            as *const u8 as *const libc::c_char,
                    ),
                    (if tab == decimal_point as libc::c_int {
                        b"\0" as *const u8 as *const libc::c_char
                    } else {
                        gettext(b"note \0" as *const u8 as *const libc::c_char)
                            as *const libc::c_char
                    }),
                    quote([decimal_point, 0 as libc::c_int as libc::c_char].as_mut_ptr()),
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
                        b"%snumbers use %s as a decimal point in this locale\0"
                            as *const u8 as *const libc::c_char,
                    ),
                    (if tab == decimal_point as libc::c_int {
                        b"\0" as *const u8 as *const libc::c_char
                    } else {
                        gettext(b"note \0" as *const u8 as *const libc::c_char)
                            as *const libc::c_char
                    }),
                    quote([decimal_point, 0 as libc::c_int as libc::c_char].as_mut_ptr()),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
    }
    if basic_numeric_field as libc::c_int != 0
        && thousands_sep_ignored as libc::c_int != 0
    {
        if 0 != 0 {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"the multi-byte number group separator in this locale is not supported\0"
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
                        b"the multi-byte number group separator in this locale is not supported\0"
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
                        b"the multi-byte number group separator in this locale is not supported\0"
                            as *const u8 as *const libc::c_char,
                    ),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
    }
    if !default_key_compare(&mut ugkey)
        || ugkey.reverse as libc::c_int != 0
            && (stable as libc::c_int != 0 || unique as libc::c_int != 0)
            && !keylist.is_null()
    {
        let mut ugkey_reverse: bool = ugkey.reverse;
        if !(stable as libc::c_int != 0 || unique as libc::c_int != 0) {
            ugkey.reverse = 0 as libc::c_int != 0;
        }
        let mut opts: [libc::c_char; 31] = [0; 31];
        key_to_opts(&mut ugkey, opts.as_mut_ptr());
        if 0 != 0 {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                ngettext(
                    b"option '-%s' is ignored\0" as *const u8 as *const libc::c_char,
                    b"options '-%s' are ignored\0" as *const u8 as *const libc::c_char,
                    select_plural(strlen(opts.as_mut_ptr())),
                ),
                opts.as_mut_ptr(),
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
                    ngettext(
                        b"option '-%s' is ignored\0" as *const u8 as *const libc::c_char,
                        b"options '-%s' are ignored\0" as *const u8
                            as *const libc::c_char,
                        select_plural(strlen(opts.as_mut_ptr())),
                    ),
                    opts.as_mut_ptr(),
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
                    ngettext(
                        b"option '-%s' is ignored\0" as *const u8 as *const libc::c_char,
                        b"options '-%s' are ignored\0" as *const u8
                            as *const libc::c_char,
                        select_plural(strlen(opts.as_mut_ptr())),
                    ),
                    opts.as_mut_ptr(),
                );
                if __errstatus != 0 as libc::c_int {
                    unreachable!();
                } else {};
                
            });
        };
        ugkey.reverse = ugkey_reverse;
    }
    if ugkey.reverse as libc::c_int != 0
        && !(stable as libc::c_int != 0 || unique as libc::c_int != 0)
        && !keylist.is_null()
    {
        if 0 != 0 {
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                gettext(
                    b"option '-r' only applies to last-resort comparison\0" as *const u8
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
                        b"option '-r' only applies to last-resort comparison\0"
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
                        b"option '-r' only applies to last-resort comparison\0"
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
unsafe extern "C" fn diff_reversed(
    mut diff: libc::c_int,
    mut reversed: bool,
) -> libc::c_int {
    return if reversed as libc::c_int != 0 {
        (diff < 0 as libc::c_int) as libc::c_int
            - (diff > 0 as libc::c_int) as libc::c_int
    } else {
        diff
    };
}
unsafe extern "C" fn keycompare(mut a: *const line, mut b: *const line) -> libc::c_int {
    let mut key: *mut keyfield = keylist;
    let mut texta: *mut libc::c_char = (*a).keybeg;
    let mut textb: *mut libc::c_char = (*b).keybeg;
    let mut lima: *mut libc::c_char = (*a).keylim;
    let mut limb: *mut libc::c_char = (*b).keylim;
    let mut diff: libc::c_int = 0;
    loop {
        let mut translate: *const libc::c_char = (*key).translate;
        let mut ignore: *const bool = (*key).ignore;
        lima = if texta > lima { texta } else { lima };
        limb = if textb > limb { textb } else { limb };
        let mut lena: size_t = lima.offset_from(texta) as libc::c_long as size_t;
        let mut lenb: size_t = limb.offset_from(textb) as libc::c_long as size_t;
        if hard_LC_COLLATE as libc::c_int != 0 || key_numeric(key) as libc::c_int != 0
            || (*key).month as libc::c_int != 0 || (*key).random as libc::c_int != 0
            || (*key).version as libc::c_int != 0
        {
            let mut ta: *mut libc::c_char = texta;
            let mut tb: *mut libc::c_char = textb;
            let mut tlena: size_t = lena;
            let mut tlenb: size_t = lenb;
            let mut enda: libc::c_char = *ta.offset(tlena as isize);
            let mut endb: libc::c_char = *tb.offset(tlenb as isize);
            let mut allocated: *mut libc::c_void = 0 as *mut libc::c_void;
            let mut stackbuf: [libc::c_char; 4000] = [0; 4000];
            if !ignore.is_null() || !translate.is_null() {
                let mut i: size_t = 0;
                let mut size: size_t = lena
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(lenb)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong);
                if size
                    <= ::core::mem::size_of::<[libc::c_char; 4000]>() as libc::c_ulong
                {
                    ta = stackbuf.as_mut_ptr();
                } else {
                    allocated = xmalloc(size);
                    ta = allocated as *mut libc::c_char;
                }
                tb = ta.offset(lena as isize).offset(1 as libc::c_int as isize);
                i = 0 as libc::c_int as size_t;
                tlena = i;
                while i < lena {
                    if !(!ignore.is_null()
                        && *ignore.offset(to_uchar(*texta.offset(i as isize)) as isize)
                            as libc::c_int != 0)
                    {
                        let fresh24 = tlena;
                        tlena = tlena.wrapping_add(1);
                        *ta
                            .offset(
                                fresh24 as isize,
                            ) = (if !translate.is_null() {
                            *translate
                                .offset(to_uchar(*texta.offset(i as isize)) as isize)
                                as libc::c_int
                        } else {
                            *texta.offset(i as isize) as libc::c_int
                        }) as libc::c_char;
                    }
                    i = i.wrapping_add(1);
                    i;
                }
                i = 0 as libc::c_int as size_t;
                tlenb = i;
                while i < lenb {
                    if !(!ignore.is_null()
                        && *ignore.offset(to_uchar(*textb.offset(i as isize)) as isize)
                            as libc::c_int != 0)
                    {
                        let fresh25 = tlenb;
                        tlenb = tlenb.wrapping_add(1);
                        *tb
                            .offset(
                                fresh25 as isize,
                            ) = (if !translate.is_null() {
                            *translate
                                .offset(to_uchar(*textb.offset(i as isize)) as isize)
                                as libc::c_int
                        } else {
                            *textb.offset(i as isize) as libc::c_int
                        }) as libc::c_char;
                    }
                    i = i.wrapping_add(1);
                    i;
                }
            }
            *ta.offset(tlena as isize) = '\0' as i32 as libc::c_char;
            *tb.offset(tlenb as isize) = '\0' as i32 as libc::c_char;
            if (*key).numeric {
                diff = numcompare(ta, tb);
            } else if (*key).general_numeric {
                diff = general_numcompare(ta, tb);
            } else if (*key).human_numeric {
                diff = human_numcompare(ta, tb);
            } else if (*key).month {
                diff = getmonth(ta, 0 as *mut *mut libc::c_char)
                    - getmonth(tb, 0 as *mut *mut libc::c_char);
            } else if (*key).random {
                diff = compare_random(ta, tlena, tb, tlenb);
            } else if (*key).version {
                diff = filenvercmp(ta, tlena as ptrdiff_t, tb, tlenb as ptrdiff_t);
            } else if tlena == 0 as libc::c_int as libc::c_ulong {
                diff = -((tlenb != 0 as libc::c_int as libc::c_ulong) as libc::c_int);
            } else if tlenb == 0 as libc::c_int as libc::c_ulong {
                diff = 1 as libc::c_int;
            } else {
                diff = xmemcoll0(
                    ta,
                    tlena.wrapping_add(1 as libc::c_int as libc::c_ulong),
                    tb,
                    tlenb.wrapping_add(1 as libc::c_int as libc::c_ulong),
                );
            }
            *ta.offset(tlena as isize) = enda;
            *tb.offset(tlenb as isize) = endb;
            free(allocated);
        } else if !ignore.is_null() {
            if !translate.is_null() {
                loop {
                    while texta < lima
                        && *ignore.offset(to_uchar(*texta) as isize) as libc::c_int != 0
                    {
                        texta = texta.offset(1);
                        texta;
                    }
                    while textb < limb
                        && *ignore.offset(to_uchar(*textb) as isize) as libc::c_int != 0
                    {
                        textb = textb.offset(1);
                        textb;
                    }
                    if !(texta < lima && textb < limb) {
                        diff = (texta < lima) as libc::c_int
                            - (textb < limb) as libc::c_int;
                        break;
                    } else {
                        diff = to_uchar(*translate.offset(to_uchar(*texta) as isize))
                            as libc::c_int
                            - to_uchar(*translate.offset(to_uchar(*textb) as isize))
                                as libc::c_int;
                        if diff != 0 {
                            break;
                        }
                        texta = texta.offset(1);
                        texta;
                        textb = textb.offset(1);
                        textb;
                    }
                }
            } else {
                loop {
                    while texta < lima
                        && *ignore.offset(to_uchar(*texta) as isize) as libc::c_int != 0
                    {
                        texta = texta.offset(1);
                        texta;
                    }
                    while textb < limb
                        && *ignore.offset(to_uchar(*textb) as isize) as libc::c_int != 0
                    {
                        textb = textb.offset(1);
                        textb;
                    }
                    if !(texta < lima && textb < limb) {
                        diff = (texta < lima) as libc::c_int
                            - (textb < limb) as libc::c_int;
                        break;
                    } else {
                        diff = to_uchar(*texta) as libc::c_int
                            - to_uchar(*textb) as libc::c_int;
                        if diff != 0 {
                            break;
                        }
                        texta = texta.offset(1);
                        texta;
                        textb = textb.offset(1);
                        textb;
                    }
                }
            }
        } else {
            let mut lenmin: size_t = if lena < lenb { lena } else { lenb };
            if lenmin == 0 as libc::c_int as libc::c_ulong {
                diff = 0 as libc::c_int;
            } else if !translate.is_null() {
                let mut i_0: size_t = 0 as libc::c_int as size_t;
                loop {
                    diff = to_uchar(
                        *translate.offset(to_uchar(*texta.offset(i_0 as isize)) as isize),
                    ) as libc::c_int
                        - to_uchar(
                            *translate
                                .offset(to_uchar(*textb.offset(i_0 as isize)) as isize),
                        ) as libc::c_int;
                    if diff != 0 {
                        break;
                    }
                    i_0 = i_0.wrapping_add(1);
                    i_0;
                    if !(i_0 < lenmin) {
                        break;
                    }
                }
            } else {
                diff = memcmp(
                    texta as *const libc::c_void,
                    textb as *const libc::c_void,
                    lenmin,
                );
            }
            if diff == 0 {
                diff = (lena > lenb) as libc::c_int - (lena < lenb) as libc::c_int;
            }
        }
        if diff != 0 {
            break;
        }
        key = (*key).next;
        if key.is_null() {
            return 0 as libc::c_int;
        }
        if (*key).eword != 18446744073709551615 as libc::c_ulong {
            lima = limfield(a, key);
            limb = limfield(b, key);
        } else {
            lima = ((*a).text)
                .offset((*a).length as isize)
                .offset(-(1 as libc::c_int as isize));
            limb = ((*b).text)
                .offset((*b).length as isize)
                .offset(-(1 as libc::c_int as isize));
        }
        if (*key).sword != 18446744073709551615 as libc::c_ulong {
            texta = begfield(a, key);
            textb = begfield(b, key);
        } else {
            texta = (*a).text;
            textb = (*b).text;
            if (*key).skipsblanks {
                while texta < lima
                    && blanks[to_uchar(*texta) as usize] as libc::c_int != 0
                {
                    texta = texta.offset(1);
                    texta;
                }
                while textb < limb
                    && blanks[to_uchar(*textb) as usize] as libc::c_int != 0
                {
                    textb = textb.offset(1);
                    textb;
                }
            }
        }
    }
    return diff_reversed(diff, (*key).reverse);
}
unsafe extern "C" fn compare(mut a: *const line, mut b: *const line) -> libc::c_int {
    let mut diff: libc::c_int = 0;
    let mut alen: size_t = 0;
    let mut blen: size_t = 0;
    if !keylist.is_null() {
        diff = keycompare(a, b);
        if diff != 0 || unique as libc::c_int != 0 || stable as libc::c_int != 0 {
            return diff;
        }
    }
    alen = ((*a).length).wrapping_sub(1 as libc::c_int as libc::c_ulong);
    blen = ((*b).length).wrapping_sub(1 as libc::c_int as libc::c_ulong);
    if alen == 0 as libc::c_int as libc::c_ulong {
        diff = -((blen != 0 as libc::c_int as libc::c_ulong) as libc::c_int);
    } else if blen == 0 as libc::c_int as libc::c_ulong {
        diff = 1 as libc::c_int;
    } else if hard_LC_COLLATE {
        diff = xmemcoll0(
            (*a).text,
            alen.wrapping_add(1 as libc::c_int as libc::c_ulong),
            (*b).text,
            blen.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
    } else {
        diff = memcmp(
            (*a).text as *const libc::c_void,
            (*b).text as *const libc::c_void,
            if alen < blen { alen } else { blen },
        );
        if diff == 0 {
            diff = (alen > blen) as libc::c_int - (alen < blen) as libc::c_int;
        }
    }
    return diff_reversed(diff, reverse);
}
unsafe extern "C" fn write_line(
    mut line: *const line,
    mut fp: *mut FILE,
    mut output_file: *const libc::c_char,
) {
    let mut buf: *mut libc::c_char = (*line).text;
    let mut n_bytes: size_t = (*line).length;
    let mut ebuf: *mut libc::c_char = buf.offset(n_bytes as isize);
    if output_file.is_null() && debug as libc::c_int != 0 {
        let mut c: *const libc::c_char = buf;
        while c < ebuf as *const libc::c_char {
            let fresh26 = c;
            c = c.offset(1);
            let mut wc: libc::c_char = *fresh26;
            if wc as libc::c_int == '\t' as i32 {
                wc = '>' as i32 as libc::c_char;
            } else if c == ebuf as *const libc::c_char {
                wc = '\n' as i32 as libc::c_char;
            }
            if fputc_unlocked(wc as libc::c_int, fp) == -(1 as libc::c_int) {
                sort_die(
                    gettext(b"write failed\0" as *const u8 as *const libc::c_char),
                    output_file,
                );
            }
        }
        debug_line(line);
    } else {
        *ebuf.offset(-(1 as libc::c_int) as isize) = eolchar;
        if fwrite_unlocked(
            buf as *const libc::c_void,
            1 as libc::c_int as size_t,
            n_bytes,
            fp,
        ) != n_bytes
        {
            sort_die(
                gettext(b"write failed\0" as *const u8 as *const libc::c_char),
                output_file,
            );
        }
        *ebuf.offset(-(1 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;
    };
}
unsafe extern "C" fn check(
    mut file_name: *const libc::c_char,
    mut checkonly: libc::c_char,
) -> bool {
    let mut fp: *mut FILE = xfopen(
        file_name,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    let mut buf: buffer = buffer {
        buf: 0 as *mut libc::c_char,
        used: 0,
        nlines: 0,
        alloc: 0,
        left: 0,
        line_bytes: 0,
        eof: false,
    };
    let mut temp: line = line {
        text: 0 as *const libc::c_char as *mut libc::c_char,
        length: 0,
        keybeg: 0 as *const libc::c_char as *mut libc::c_char,
        keylim: 0 as *const libc::c_char as *mut libc::c_char,
    };
    let mut alloc: size_t = 0 as libc::c_int as size_t;
    let mut line_number: uintmax_t = 0 as libc::c_int as uintmax_t;
    let mut key: *const keyfield = keylist;
    let mut nonunique: bool = !unique;
    let mut ordered: bool = 1 as libc::c_int != 0;
    initbuf(
        &mut buf,
        ::core::mem::size_of::<line>() as libc::c_ulong,
        if merge_buffer_size > sort_size { merge_buffer_size } else { sort_size },
    );
    temp.text = 0 as *mut libc::c_char;
    let mut current_block_20: u64;
    's_23: while fillbuf(&mut buf, fp, file_name) {
        let mut line: *const line = buffer_linelim(&mut buf);
        let mut linebase: *const line = line.offset(-(buf.nlines as isize));
        if alloc != 0
            && nonunique as libc::c_int
                <= compare(&mut temp, line.offset(-(1 as libc::c_int as isize)))
        {
            current_block_20 = 2473556513754201174;
        } else {
            current_block_20 = 2979737022853876585;
        }
        loop {
            match current_block_20 {
                2473556513754201174 => {
                    if checkonly as libc::c_int == 'c' as i32 {
                        let mut disorder_line: *const line = line
                            .offset(-(1 as libc::c_int as isize));
                        let mut disorder_line_number: uintmax_t = ((buffer_linelim(
                            &mut buf,
                        ))
                            .offset_from(disorder_line) as libc::c_long as libc::c_ulong)
                            .wrapping_add(line_number);
                        let mut hr_buf: [libc::c_char; 21] = [0; 21];
                        fprintf(
                            stderr,
                            gettext(
                                b"%s: %s:%s: disorder: \0" as *const u8
                                    as *const libc::c_char,
                            ),
                            program_name,
                            file_name,
                            umaxtostr(disorder_line_number, hr_buf.as_mut_ptr()),
                        );
                        write_line(
                            disorder_line,
                            stderr,
                            gettext(
                                b"standard error\0" as *const u8 as *const libc::c_char,
                            ),
                        );
                    }
                    ordered = 0 as libc::c_int != 0;
                    break 's_23;
                }
                _ => {
                    line = line.offset(-1);
                    if linebase < line {
                        if nonunique as libc::c_int
                            <= compare(line, line.offset(-(1 as libc::c_int as isize)))
                        {
                            current_block_20 = 2473556513754201174;
                        } else {
                            current_block_20 = 2979737022853876585;
                        }
                    } else {
                        line_number = (line_number as libc::c_ulong)
                            .wrapping_add(buf.nlines) as uintmax_t as uintmax_t;
                        if alloc < (*line).length {
                            loop {
                                alloc = (alloc as libc::c_ulong)
                                    .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t
                                    as size_t;
                                if alloc == 0 {
                                    alloc = (*line).length;
                                    break;
                                } else if !(alloc < (*line).length) {
                                    break;
                                }
                            }
                            free(temp.text as *mut libc::c_void);
                            temp.text = xmalloc(alloc) as *mut libc::c_char;
                        }
                        memcpy(
                            temp.text as *mut libc::c_void,
                            (*line).text as *const libc::c_void,
                            (*line).length,
                        );
                        temp.length = (*line).length;
                        if !key.is_null() {
                            temp
                                .keybeg = (temp.text)
                                .offset(
                                    ((*line).keybeg).offset_from((*line).text) as libc::c_long
                                        as isize,
                                );
                            temp
                                .keylim = (temp.text)
                                .offset(
                                    ((*line).keylim).offset_from((*line).text) as libc::c_long
                                        as isize,
                                );
                        }
                        break;
                    }
                }
            }
        }
    }
    xfclose(fp, file_name);
    free(buf.buf as *mut libc::c_void);
    free(temp.text as *mut libc::c_void);
    return ordered;
}
unsafe extern "C" fn open_input_files(
    mut files: *mut sortfile,
    mut nfiles: size_t,
    mut pfps: *mut *mut *mut FILE,
) -> size_t {
    *pfps = xnmalloc(nfiles, ::core::mem::size_of::<*mut FILE>() as libc::c_ulong)
        as *mut *mut FILE;
    let mut fps: *mut *mut FILE = *pfps;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong) < nfiles {
        let ref mut fresh27 = *fps.offset(i as isize);
        *fresh27 = if !((*files.offset(i as isize)).temp).is_null()
            && (*(*files.offset(i as isize)).temp).state as libc::c_int
                != UNCOMPRESSED as libc::c_int
        {
            open_temp((*files.offset(i as isize)).temp)
        } else {
            stream_open(
                (*files.offset(i as isize)).name,
                b"r\0" as *const u8 as *const libc::c_char,
            )
        };
        if (*fps.offset(i as isize)).is_null() {
            break;
        }
        i += 1;
        i;
    }
    return i as size_t;
}
unsafe extern "C" fn mergefps(
    mut files: *mut sortfile,
    mut ntemps: size_t,
    mut nfiles: size_t,
    mut ofp: *mut FILE,
    mut output_file: *const libc::c_char,
    mut fps: *mut *mut FILE,
) {
    let mut buffer: *mut buffer = xnmalloc(
        nfiles,
        ::core::mem::size_of::<buffer>() as libc::c_ulong,
    ) as *mut buffer;
    let mut saved: line = line {
        text: 0 as *const libc::c_char as *mut libc::c_char,
        length: 0,
        keybeg: 0 as *const libc::c_char as *mut libc::c_char,
        keylim: 0 as *const libc::c_char as *mut libc::c_char,
    };
    let mut savedline: *const line = 0 as *const line;
    let mut savealloc: size_t = 0 as libc::c_int as size_t;
    let mut cur: *mut *const line = xnmalloc(
        nfiles,
        ::core::mem::size_of::<*const line>() as libc::c_ulong,
    ) as *mut *const line;
    let mut base: *mut *const line = xnmalloc(
        nfiles,
        ::core::mem::size_of::<*const line>() as libc::c_ulong,
    ) as *mut *const line;
    let mut ord: *mut size_t = xnmalloc(
        nfiles,
        ::core::mem::size_of::<size_t>() as libc::c_ulong,
    ) as *mut size_t;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut t: size_t = 0;
    let mut key: *const keyfield = keylist;
    saved.text = 0 as *mut libc::c_char;
    i = 0 as libc::c_int as size_t;
    while i < nfiles {
        initbuf(
            &mut *buffer.offset(i as isize),
            ::core::mem::size_of::<line>() as libc::c_ulong,
            if merge_buffer_size > sort_size.wrapping_div(nfiles) {
                merge_buffer_size
            } else {
                sort_size.wrapping_div(nfiles)
            },
        );
        if fillbuf(
            &mut *buffer.offset(i as isize),
            *fps.offset(i as isize),
            (*files.offset(i as isize)).name,
        ) {
            let mut linelim: *const line = buffer_linelim(
                &mut *buffer.offset(i as isize),
            );
            let ref mut fresh28 = *cur.offset(i as isize);
            *fresh28 = linelim.offset(-(1 as libc::c_int as isize));
            let ref mut fresh29 = *base.offset(i as isize);
            *fresh29 = linelim.offset(-((*buffer.offset(i as isize)).nlines as isize));
            i = i.wrapping_add(1);
            i;
        } else {
            xfclose(*fps.offset(i as isize), (*files.offset(i as isize)).name);
            if i < ntemps {
                ntemps = ntemps.wrapping_sub(1);
                ntemps;
                zaptemp((*files.offset(i as isize)).name);
            }
            free((*buffer.offset(i as isize)).buf as *mut libc::c_void);
            nfiles = nfiles.wrapping_sub(1);
            nfiles;
            j = i;
            while j < nfiles {
                *files
                    .offset(
                        j as isize,
                    ) = *files
                    .offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
                let ref mut fresh30 = *fps.offset(j as isize);
                *fresh30 = *fps
                    .offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
                j = j.wrapping_add(1);
                j;
            }
        }
    }
    i = 0 as libc::c_int as size_t;
    while i < nfiles {
        *ord.offset(i as isize) = i;
        i = i.wrapping_add(1);
        i;
    }
    i = 1 as libc::c_int as size_t;
    while i < nfiles {
        if (0 as libc::c_int)
            < compare(
                *cur
                    .offset(
                        *ord
                            .offset(
                                i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                            ) as isize,
                    ),
                *cur.offset(*ord.offset(i as isize) as isize),
            )
        {
            t = *ord.offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
            *ord
                .offset(
                    i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                ) = *ord.offset(i as isize);
            *ord.offset(i as isize) = t;
            i = 0 as libc::c_int as size_t;
        }
        i = i.wrapping_add(1);
        i;
    }
    while nfiles != 0 {
        let mut smallest: *const line = *cur
            .offset(*ord.offset(0 as libc::c_int as isize) as isize);
        if unique {
            if !savedline.is_null() && compare(savedline, smallest) != 0 {
                savedline = 0 as *const line;
                write_line(&mut saved, ofp, output_file);
            }
            if savedline.is_null() {
                savedline = &mut saved;
                if savealloc < (*smallest).length {
                    loop {
                        if savealloc == 0 {
                            savealloc = (*smallest).length;
                            break;
                        } else {
                            savealloc = (savealloc as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t
                                as size_t;
                            if !(savealloc < (*smallest).length) {
                                break;
                            }
                        }
                    }
                    free(saved.text as *mut libc::c_void);
                    saved.text = xmalloc(savealloc) as *mut libc::c_char;
                }
                saved.length = (*smallest).length;
                memcpy(
                    saved.text as *mut libc::c_void,
                    (*smallest).text as *const libc::c_void,
                    saved.length,
                );
                if !key.is_null() {
                    saved
                        .keybeg = (saved.text)
                        .offset(
                            ((*smallest).keybeg).offset_from((*smallest).text)
                                as libc::c_long as isize,
                        );
                    saved
                        .keylim = (saved.text)
                        .offset(
                            ((*smallest).keylim).offset_from((*smallest).text)
                                as libc::c_long as isize,
                        );
                }
            }
        } else {
            write_line(smallest, ofp, output_file);
        }
        if *base.offset(*ord.offset(0 as libc::c_int as isize) as isize) < smallest {
            let ref mut fresh31 = *cur
                .offset(*ord.offset(0 as libc::c_int as isize) as isize);
            *fresh31 = smallest.offset(-(1 as libc::c_int as isize));
        } else if fillbuf(
            &mut *buffer.offset(*ord.offset(0 as libc::c_int as isize) as isize),
            *fps.offset(*ord.offset(0 as libc::c_int as isize) as isize),
            (*files.offset(*ord.offset(0 as libc::c_int as isize) as isize)).name,
        ) {
            let mut linelim_0: *const line = buffer_linelim(
                &mut *buffer.offset(*ord.offset(0 as libc::c_int as isize) as isize),
            );
            let ref mut fresh32 = *cur
                .offset(*ord.offset(0 as libc::c_int as isize) as isize);
            *fresh32 = linelim_0.offset(-(1 as libc::c_int as isize));
            let ref mut fresh33 = *base
                .offset(*ord.offset(0 as libc::c_int as isize) as isize);
            *fresh33 = linelim_0
                .offset(
                    -((*buffer.offset(*ord.offset(0 as libc::c_int as isize) as isize))
                        .nlines as isize),
                );
        } else {
            i = 1 as libc::c_int as size_t;
            while i < nfiles {
                if *ord.offset(i as isize) > *ord.offset(0 as libc::c_int as isize) {
                    let ref mut fresh34 = *ord.offset(i as isize);
                    *fresh34 = (*fresh34).wrapping_sub(1);
                    *fresh34;
                }
                i = i.wrapping_add(1);
                i;
            }
            nfiles = nfiles.wrapping_sub(1);
            nfiles;
            xfclose(
                *fps.offset(*ord.offset(0 as libc::c_int as isize) as isize),
                (*files.offset(*ord.offset(0 as libc::c_int as isize) as isize)).name,
            );
            if *ord.offset(0 as libc::c_int as isize) < ntemps {
                ntemps = ntemps.wrapping_sub(1);
                ntemps;
                zaptemp(
                    (*files.offset(*ord.offset(0 as libc::c_int as isize) as isize)).name,
                );
            }
            free(
                (*buffer.offset(*ord.offset(0 as libc::c_int as isize) as isize)).buf
                    as *mut libc::c_void,
            );
            i = *ord.offset(0 as libc::c_int as isize);
            while i < nfiles {
                let ref mut fresh35 = *fps.offset(i as isize);
                *fresh35 = *fps
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
                *files
                    .offset(
                        i as isize,
                    ) = *files
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
                *buffer
                    .offset(
                        i as isize,
                    ) = *buffer
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
                let ref mut fresh36 = *cur.offset(i as isize);
                *fresh36 = *cur
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
                let ref mut fresh37 = *base.offset(i as isize);
                *fresh37 = *base
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
                i = i.wrapping_add(1);
                i;
            }
            i = 0 as libc::c_int as size_t;
            while i < nfiles {
                *ord
                    .offset(
                        i as isize,
                    ) = *ord
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
                i = i.wrapping_add(1);
                i;
            }
            continue;
        }
        let mut lo: size_t = 1 as libc::c_int as size_t;
        let mut hi: size_t = nfiles;
        let mut probe: size_t = lo;
        let mut ord0: size_t = *ord.offset(0 as libc::c_int as isize);
        let mut count_of_smaller_lines: size_t = 0;
        while lo < hi {
            let mut cmp: libc::c_int = compare(
                *cur.offset(ord0 as isize),
                *cur.offset(*ord.offset(probe as isize) as isize),
            );
            if cmp < 0 as libc::c_int
                || cmp == 0 as libc::c_int && ord0 < *ord.offset(probe as isize)
            {
                hi = probe;
            } else {
                lo = probe.wrapping_add(1 as libc::c_int as libc::c_ulong);
            }
            probe = lo.wrapping_add(hi).wrapping_div(2 as libc::c_int as libc::c_ulong);
        }
        count_of_smaller_lines = lo.wrapping_sub(1 as libc::c_int as libc::c_ulong);
        j = 0 as libc::c_int as size_t;
        while j < count_of_smaller_lines {
            *ord
                .offset(
                    j as isize,
                ) = *ord
                .offset(j.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize);
            j = j.wrapping_add(1);
            j;
        }
        *ord.offset(count_of_smaller_lines as isize) = ord0;
    }
    if unique as libc::c_int != 0 && !savedline.is_null() {
        write_line(&mut saved, ofp, output_file);
        free(saved.text as *mut libc::c_void);
    }
    xfclose(ofp, output_file);
    free(fps as *mut libc::c_void);
    free(buffer as *mut libc::c_void);
    free(ord as *mut libc::c_void);
    free(base as *mut libc::c_void);
    free(cur as *mut libc::c_void);
}
unsafe extern "C" fn mergefiles(
    mut files: *mut sortfile,
    mut ntemps: size_t,
    mut nfiles: size_t,
    mut ofp: *mut FILE,
    mut output_file: *const libc::c_char,
) -> size_t {
    let mut fps: *mut *mut FILE = 0 as *mut *mut FILE;
    let mut nopened: size_t = open_input_files(files, nfiles, &mut fps);
    if nopened < nfiles && nopened < 2 as libc::c_int as libc::c_ulong {
        sort_die(
            gettext(b"open failed\0" as *const u8 as *const libc::c_char),
            (*files.offset(nopened as isize)).name,
        );
    }
    mergefps(files, ntemps, nopened, ofp, output_file, fps);
    return nopened;
}
unsafe extern "C" fn mergelines(
    mut t: *mut line,
    mut nlines: size_t,
    mut lo: *const line,
) {
    let mut nlo: size_t = nlines.wrapping_div(2 as libc::c_int as libc::c_ulong);
    let mut nhi: size_t = nlines.wrapping_sub(nlo);
    let mut hi: *mut line = t.offset(-(nlo as isize));
    loop {
        if compare(
            lo.offset(-(1 as libc::c_int as isize)),
            hi.offset(-(1 as libc::c_int as isize)),
        ) <= 0 as libc::c_int
        {
            lo = lo.offset(-1);
            t = t.offset(-1);
            *t = *lo;
            nlo = nlo.wrapping_sub(1);
            if nlo == 0 {
                return;
            }
        } else {
            hi = hi.offset(-1);
            t = t.offset(-1);
            *t = *hi;
            nhi = nhi.wrapping_sub(1);
            if nhi == 0 {
                loop {
                    lo = lo.offset(-1);
                    t = t.offset(-1);
                    *t = *lo;
                    nlo = nlo.wrapping_sub(1);
                    if !(nlo != 0) {
                        break;
                    }
                }
                return;
            }
        }
    };
}
unsafe extern "C" fn sequential_sort(
    mut lines: *mut line,
    mut nlines: size_t,
    mut temp: *mut line,
    mut to_temp: bool,
) {
    if nlines == 2 as libc::c_int as libc::c_ulong {
        let mut swap: libc::c_int = ((0 as libc::c_int)
            < compare(
                &mut *lines.offset(-(1 as libc::c_int) as isize),
                &mut *lines.offset(-(2 as libc::c_int) as isize),
            )) as libc::c_int;
        if to_temp {
            *temp
                .offset(
                    -(1 as libc::c_int) as isize,
                ) = *lines.offset((-(1 as libc::c_int) - swap) as isize);
            *temp
                .offset(
                    -(2 as libc::c_int) as isize,
                ) = *lines.offset((-(2 as libc::c_int) + swap) as isize);
        } else if swap != 0 {
            *temp
                .offset(
                    -(1 as libc::c_int) as isize,
                ) = *lines.offset(-(1 as libc::c_int) as isize);
            *lines
                .offset(
                    -(1 as libc::c_int) as isize,
                ) = *lines.offset(-(2 as libc::c_int) as isize);
            *lines
                .offset(
                    -(2 as libc::c_int) as isize,
                ) = *temp.offset(-(1 as libc::c_int) as isize);
        }
    } else {
        let mut nlo: size_t = nlines.wrapping_div(2 as libc::c_int as libc::c_ulong);
        let mut nhi: size_t = nlines.wrapping_sub(nlo);
        let mut lo: *mut line = lines;
        let mut hi: *mut line = lines.offset(-(nlo as isize));
        sequential_sort(
            hi,
            nhi,
            temp
                .offset(
                    -((if to_temp as libc::c_int != 0 {
                        nlo
                    } else {
                        0 as libc::c_int as libc::c_ulong
                    }) as isize),
                ),
            to_temp,
        );
        if (1 as libc::c_int as libc::c_ulong) < nlo {
            sequential_sort(lo, nlo, temp, !to_temp);
        } else if !to_temp {
            *temp
                .offset(
                    -(1 as libc::c_int) as isize,
                ) = *lo.offset(-(1 as libc::c_int) as isize);
        }
        let mut dest: *mut line = 0 as *mut line;
        let mut sorted_lo: *const line = 0 as *const line;
        if to_temp {
            dest = temp;
            sorted_lo = lines;
        } else {
            dest = lines;
            sorted_lo = temp;
        }
        mergelines(dest, nlines, sorted_lo);
    };
}
unsafe extern "C" fn merge_tree_init(
    mut nthreads: size_t,
    mut nlines: size_t,
    mut dest: *mut line,
) -> *mut merge_node {
    let mut merge_tree: *mut merge_node = xmalloc(
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<merge_node>() as libc::c_ulong)
            .wrapping_mul(nthreads),
    ) as *mut merge_node;
    let mut root: *mut merge_node = merge_tree;
    (*root).end_hi = 0 as *mut line;
    (*root).end_lo = (*root).end_hi;
    (*root).hi = (*root).end_lo;
    (*root).lo = (*root).hi;
    (*root).dest = 0 as *mut *mut line;
    (*root).nhi = nlines;
    (*root).nlo = (*root).nhi;
    (*root).parent = 0 as *mut merge_node;
    (*root).level = MERGE_END as libc::c_int as libc::c_uint;
    (*root).queued = 0 as libc::c_int != 0;
    pthread_mutex_init(&mut (*root).lock, 0 as *const pthread_mutexattr_t);
    init_node(
        root,
        root.offset(1 as libc::c_int as isize),
        dest,
        nthreads,
        nlines,
        0 as libc::c_int != 0,
    );
    return merge_tree;
}
unsafe extern "C" fn merge_tree_destroy(
    mut nthreads: size_t,
    mut merge_tree: *mut merge_node,
) {
    let mut n_nodes: size_t = nthreads.wrapping_mul(2 as libc::c_int as libc::c_ulong);
    let mut node: *mut merge_node = merge_tree;
    loop {
        let fresh38 = n_nodes;
        n_nodes = n_nodes.wrapping_sub(1);
        if !(fresh38 != 0) {
            break;
        }
        pthread_mutex_destroy(&mut (*node).lock);
        node = node.offset(1);
        node;
    }
    free(merge_tree as *mut libc::c_void);
}
unsafe extern "C" fn init_node(
    mut parent: *mut merge_node,
    mut node_pool: *mut merge_node,
    mut dest: *mut line,
    mut nthreads: size_t,
    mut total_lines: size_t,
    mut is_lo_child: bool,
) -> *mut merge_node {
    let mut nlines: size_t = if is_lo_child as libc::c_int != 0 {
        (*parent).nlo
    } else {
        (*parent).nhi
    };
    let mut nlo: size_t = nlines.wrapping_div(2 as libc::c_int as libc::c_ulong);
    let mut nhi: size_t = nlines.wrapping_sub(nlo);
    let mut lo: *mut line = dest.offset(-(total_lines as isize));
    let mut hi: *mut line = lo.offset(-(nlo as isize));
    let mut parent_end: *mut *mut line = if is_lo_child as libc::c_int != 0 {
        &mut (*parent).end_lo
    } else {
        &mut (*parent).end_hi
    };
    let fresh39 = node_pool;
    node_pool = node_pool.offset(1);
    let mut node: *mut merge_node = fresh39;
    (*node).end_lo = lo;
    (*node).lo = (*node).end_lo;
    (*node).end_hi = hi;
    (*node).hi = (*node).end_hi;
    (*node).dest = parent_end;
    (*node).nlo = nlo;
    (*node).nhi = nhi;
    (*node).parent = parent;
    (*node).level = ((*parent).level).wrapping_add(1 as libc::c_int as libc::c_uint);
    (*node).queued = 0 as libc::c_int != 0;
    pthread_mutex_init(&mut (*node).lock, 0 as *const pthread_mutexattr_t);
    if nthreads > 1 as libc::c_int as libc::c_ulong {
        let mut lo_threads: size_t = nthreads
            .wrapping_div(2 as libc::c_int as libc::c_ulong);
        let mut hi_threads: size_t = nthreads.wrapping_sub(lo_threads);
        (*node).lo_child = node_pool;
        node_pool = init_node(
            node,
            node_pool,
            lo,
            lo_threads,
            total_lines,
            1 as libc::c_int != 0,
        );
        (*node).hi_child = node_pool;
        node_pool = init_node(
            node,
            node_pool,
            hi,
            hi_threads,
            total_lines,
            0 as libc::c_int != 0,
        );
    } else {
        (*node).lo_child = 0 as *mut merge_node;
        (*node).hi_child = 0 as *mut merge_node;
    }
    return node_pool;
}
unsafe extern "C" fn compare_nodes(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    let mut nodea: *const merge_node = a as *const merge_node;
    let mut nodeb: *const merge_node = b as *const merge_node;
    if (*nodea).level == (*nodeb).level {
        return (((*nodea).nlo).wrapping_add((*nodea).nhi)
            < ((*nodeb).nlo).wrapping_add((*nodeb).nhi)) as libc::c_int;
    }
    return ((*nodea).level < (*nodeb).level) as libc::c_int;
}
#[inline]
unsafe extern "C" fn lock_node(mut node: *mut merge_node) {
    pthread_mutex_lock(&mut (*node).lock);
}
#[inline]
unsafe extern "C" fn unlock_node(mut node: *mut merge_node) {
    pthread_mutex_unlock(&mut (*node).lock);
}
unsafe extern "C" fn queue_destroy(mut queue: *mut merge_node_queue) {
    heap_free((*queue).priority_queue);
    pthread_cond_destroy(&mut (*queue).cond);
    pthread_mutex_destroy(&mut (*queue).mutex);
}
unsafe extern "C" fn queue_init(mut queue: *mut merge_node_queue, mut nthreads: size_t) {
    (*queue)
        .priority_queue = heap_alloc(
        Some(
            compare_nodes
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
        (2 as libc::c_int as libc::c_ulong).wrapping_mul(nthreads),
    );
    pthread_mutex_init(&mut (*queue).mutex, 0 as *const pthread_mutexattr_t);
    pthread_cond_init(&mut (*queue).cond, 0 as *const pthread_condattr_t);
}
unsafe extern "C" fn queue_insert(
    mut queue: *mut merge_node_queue,
    mut node: *mut merge_node,
) {
    pthread_mutex_lock(&mut (*queue).mutex);
    heap_insert((*queue).priority_queue, node as *mut libc::c_void);
    (*node).queued = 1 as libc::c_int != 0;
    pthread_cond_signal(&mut (*queue).cond);
    pthread_mutex_unlock(&mut (*queue).mutex);
}
unsafe extern "C" fn queue_pop(mut queue: *mut merge_node_queue) -> *mut merge_node {
    let mut node: *mut merge_node = 0 as *mut merge_node;
    pthread_mutex_lock(&mut (*queue).mutex);
    loop {
        node = heap_remove_top((*queue).priority_queue) as *mut merge_node;
        if !node.is_null() {
            break;
        }
        pthread_cond_wait(&mut (*queue).cond, &mut (*queue).mutex);
    }
    pthread_mutex_unlock(&mut (*queue).mutex);
    lock_node(node);
    (*node).queued = 0 as libc::c_int != 0;
    return node;
}
unsafe extern "C" fn write_unique(
    mut line: *const line,
    mut tfp: *mut FILE,
    mut temp_output: *const libc::c_char,
) {
    if unique {
        if !(saved_line.text).is_null() && compare(line, &mut saved_line) == 0 {
            return;
        }
        saved_line = *line;
    }
    write_line(line, tfp, temp_output);
}
unsafe extern "C" fn mergelines_node(
    mut node: *mut merge_node,
    mut total_lines: size_t,
    mut tfp: *mut FILE,
    mut temp_output: *const libc::c_char,
) {
    let mut lo_orig: *mut line = (*node).lo;
    let mut hi_orig: *mut line = (*node).hi;
    let mut to_merge: size_t = (total_lines
        >> (2 as libc::c_int as libc::c_uint)
            .wrapping_mul(
                ((*node).level).wrapping_add(1 as libc::c_int as libc::c_uint),
            ))
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut merged_lo: size_t = 0;
    let mut merged_hi: size_t = 0;
    if (*node).level > MERGE_ROOT as libc::c_int as libc::c_uint {
        let mut dest: *mut line = *(*node).dest;
        while (*node).lo != (*node).end_lo && (*node).hi != (*node).end_hi
            && {
                let fresh40 = to_merge;
                to_merge = to_merge.wrapping_sub(1);
                fresh40 != 0
            }
        {
            if compare(
                ((*node).lo).offset(-(1 as libc::c_int as isize)),
                ((*node).hi).offset(-(1 as libc::c_int as isize)),
            ) <= 0 as libc::c_int
            {
                (*node).lo = ((*node).lo).offset(-1);
                dest = dest.offset(-1);
                *dest = *(*node).lo;
            } else {
                (*node).hi = ((*node).hi).offset(-1);
                dest = dest.offset(-1);
                *dest = *(*node).hi;
            }
        }
        merged_lo = lo_orig.offset_from((*node).lo) as libc::c_long as size_t;
        merged_hi = hi_orig.offset_from((*node).hi) as libc::c_long as size_t;
        if (*node).nhi == merged_hi {
            while (*node).lo != (*node).end_lo
                && {
                    let fresh41 = to_merge;
                    to_merge = to_merge.wrapping_sub(1);
                    fresh41 != 0
                }
            {
                (*node).lo = ((*node).lo).offset(-1);
                dest = dest.offset(-1);
                *dest = *(*node).lo;
            }
        } else if (*node).nlo == merged_lo {
            while (*node).hi != (*node).end_hi
                && {
                    let fresh42 = to_merge;
                    to_merge = to_merge.wrapping_sub(1);
                    fresh42 != 0
                }
            {
                (*node).hi = ((*node).hi).offset(-1);
                dest = dest.offset(-1);
                *dest = *(*node).hi;
            }
        }
        *(*node).dest = dest;
    } else {
        while (*node).lo != (*node).end_lo && (*node).hi != (*node).end_hi
            && {
                let fresh43 = to_merge;
                to_merge = to_merge.wrapping_sub(1);
                fresh43 != 0
            }
        {
            if compare(
                ((*node).lo).offset(-(1 as libc::c_int as isize)),
                ((*node).hi).offset(-(1 as libc::c_int as isize)),
            ) <= 0 as libc::c_int
            {
                (*node).lo = ((*node).lo).offset(-1);
                write_unique((*node).lo, tfp, temp_output);
            } else {
                (*node).hi = ((*node).hi).offset(-1);
                write_unique((*node).hi, tfp, temp_output);
            }
        }
        merged_lo = lo_orig.offset_from((*node).lo) as libc::c_long as size_t;
        merged_hi = hi_orig.offset_from((*node).hi) as libc::c_long as size_t;
        if (*node).nhi == merged_hi {
            while (*node).lo != (*node).end_lo
                && {
                    let fresh44 = to_merge;
                    to_merge = to_merge.wrapping_sub(1);
                    fresh44 != 0
                }
            {
                (*node).lo = ((*node).lo).offset(-1);
                write_unique((*node).lo, tfp, temp_output);
            }
        } else if (*node).nlo == merged_lo {
            while (*node).hi != (*node).end_hi
                && {
                    let fresh45 = to_merge;
                    to_merge = to_merge.wrapping_sub(1);
                    fresh45 != 0
                }
            {
                (*node).hi = ((*node).hi).offset(-1);
                write_unique((*node).hi, tfp, temp_output);
            }
        }
    }
    merged_lo = lo_orig.offset_from((*node).lo) as libc::c_long as size_t;
    merged_hi = hi_orig.offset_from((*node).hi) as libc::c_long as size_t;
    (*node)
        .nlo = ((*node).nlo as libc::c_ulong).wrapping_sub(merged_lo) as size_t
        as size_t;
    (*node)
        .nhi = ((*node).nhi as libc::c_ulong).wrapping_sub(merged_hi) as size_t
        as size_t;
}
unsafe extern "C" fn queue_check_insert(
    mut queue: *mut merge_node_queue,
    mut node: *mut merge_node,
) {
    if !(*node).queued {
        let mut lo_avail: bool = ((*node).lo).offset_from((*node).end_lo) as libc::c_long
            != 0 as libc::c_int as libc::c_long;
        let mut hi_avail: bool = ((*node).hi).offset_from((*node).end_hi) as libc::c_long
            != 0 as libc::c_int as libc::c_long;
        if if lo_avail as libc::c_int != 0 {
            (hi_avail as libc::c_int != 0 || (*node).nhi == 0) as libc::c_int
        } else {
            (hi_avail as libc::c_int != 0 && (*node).nlo == 0) as libc::c_int
        } != 0
        {
            queue_insert(queue, node);
        }
    }
}
unsafe extern "C" fn queue_check_insert_parent(
    mut queue: *mut merge_node_queue,
    mut node: *mut merge_node,
) {
    if (*node).level > MERGE_ROOT as libc::c_int as libc::c_uint {
        lock_node((*node).parent);
        queue_check_insert(queue, (*node).parent);
        unlock_node((*node).parent);
    } else if ((*node).nlo).wrapping_add((*node).nhi)
        == 0 as libc::c_int as libc::c_ulong
    {
        queue_insert(queue, (*node).parent);
    }
}
unsafe extern "C" fn merge_loop(
    mut queue: *mut merge_node_queue,
    mut total_lines: size_t,
    mut tfp: *mut FILE,
    mut temp_output: *const libc::c_char,
) {
    loop {
        let mut node: *mut merge_node = queue_pop(queue);
        if (*node).level == MERGE_END as libc::c_int as libc::c_uint {
            unlock_node(node);
            queue_insert(queue, node);
            break;
        } else {
            mergelines_node(node, total_lines, tfp, temp_output);
            queue_check_insert(queue, node);
            queue_check_insert_parent(queue, node);
            unlock_node(node);
        }
    };
}
unsafe extern "C" fn sortlines_thread(mut data: *mut libc::c_void) -> *mut libc::c_void {
    let mut args: *const thread_args = data as *const thread_args;
    sortlines(
        (*args).lines,
        (*args).nthreads,
        (*args).total_lines,
        (*args).node,
        (*args).queue,
        (*args).tfp,
        (*args).output_temp,
    );
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn sortlines(
    mut lines: *mut line,
    mut nthreads: size_t,
    mut total_lines: size_t,
    mut node: *mut merge_node,
    mut queue: *mut merge_node_queue,
    mut tfp: *mut FILE,
    mut temp_output: *const libc::c_char,
) {
    let mut nlines: size_t = ((*node).nlo).wrapping_add((*node).nhi);
    let mut lo_threads: size_t = nthreads
        .wrapping_div(2 as libc::c_int as libc::c_ulong);
    let mut hi_threads: size_t = nthreads.wrapping_sub(lo_threads);
    let mut thread: pthread_t = 0;
    let mut args: thread_args = {
        let mut init = thread_args {
            lines: lines,
            nthreads: lo_threads,
            total_lines: total_lines,
            node: (*node).lo_child,
            queue: queue,
            tfp: tfp,
            output_temp: temp_output,
        };
        init
    };
    if nthreads > 1 as libc::c_int as libc::c_ulong
        && SUBTHREAD_LINES_HEURISTIC as libc::c_int as libc::c_ulong <= nlines
        && pthread_create(
            &mut thread,
            0 as *const pthread_attr_t,
            Some(
                sortlines_thread
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            &mut args as *mut thread_args as *mut libc::c_void,
        ) == 0 as libc::c_int
    {
        sortlines(
            lines.offset(-((*node).nlo as isize)),
            hi_threads,
            total_lines,
            (*node).hi_child,
            queue,
            tfp,
            temp_output,
        );
        pthread_join(thread, 0 as *mut *mut libc::c_void);
    } else {
        let mut nlo: size_t = (*node).nlo;
        let mut nhi: size_t = (*node).nhi;
        let mut temp: *mut line = lines.offset(-(total_lines as isize));
        if (1 as libc::c_int as libc::c_ulong) < nhi {
            sequential_sort(
                lines.offset(-(nlo as isize)),
                nhi,
                temp
                    .offset(
                        -(nlo.wrapping_div(2 as libc::c_int as libc::c_ulong) as isize),
                    ),
                0 as libc::c_int != 0,
            );
        }
        if (1 as libc::c_int as libc::c_ulong) < nlo {
            sequential_sort(lines, nlo, temp, 0 as libc::c_int != 0);
        }
        (*node).lo = lines;
        (*node).hi = lines.offset(-(nlo as isize));
        (*node).end_lo = lines.offset(-(nlo as isize));
        (*node).end_hi = lines.offset(-(nlo as isize)).offset(-(nhi as isize));
        queue_insert(queue, node);
        merge_loop(queue, total_lines, tfp, temp_output);
    };
}
unsafe extern "C" fn avoid_trashing_input(
    mut files: *mut sortfile,
    mut ntemps: size_t,
    mut nfiles: size_t,
    mut outfile: *const libc::c_char,
) {
    let mut tempcopy: *mut tempnode = 0 as *mut tempnode;
    let mut i: size_t = ntemps;
    while i < nfiles {
        let mut is_stdin: bool = strcmp(
            (*files.offset(i as isize)).name,
            b"-\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int;
        let mut same: bool = false;
        let mut instat: stat = stat {
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
        if !outfile.is_null()
            && strcmp(outfile, (*files.offset(i as isize)).name) == 0 as libc::c_int
            && !is_stdin
        {
            same = 1 as libc::c_int != 0;
        } else {
            let mut outst: *mut stat = get_outstatus();
            if outst.is_null() {
                break;
            }
            same = (if is_stdin as libc::c_int != 0 {
                fstat(0 as libc::c_int, &mut instat)
            } else {
                stat((*files.offset(i as isize)).name, &mut instat)
            }) == 0 as libc::c_int
                && psame_inode(&mut instat, outst) as libc::c_int != 0;
        }
        if same {
            if tempcopy.is_null() {
                let mut tftp: *mut FILE = 0 as *mut FILE;
                tempcopy = create_temp(&mut tftp);
                mergefiles(
                    &mut *files.offset(i as isize),
                    0 as libc::c_int as size_t,
                    1 as libc::c_int as size_t,
                    tftp,
                    ((*tempcopy).name).as_mut_ptr(),
                );
            }
            let ref mut fresh46 = (*files.offset(i as isize)).name;
            *fresh46 = ((*tempcopy).name).as_mut_ptr();
            let ref mut fresh47 = (*files.offset(i as isize)).temp;
            *fresh47 = tempcopy;
        }
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn check_inputs(
    mut files: *const *mut libc::c_char,
    mut nfiles: size_t,
) {
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < nfiles {
        if !(strcmp(
            *files.offset(i as isize),
            b"-\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int)
        {
            if euidaccess(*files.offset(i as isize), 4 as libc::c_int)
                != 0 as libc::c_int
            {
                sort_die(
                    gettext(b"cannot read\0" as *const u8 as *const libc::c_char),
                    *files.offset(i as isize),
                );
            }
        }
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn check_output(mut outfile: *const libc::c_char) {
    if !outfile.is_null() {
        let mut oflags: libc::c_int = 0o1 as libc::c_int | 0 as libc::c_int
            | 0o2000000 as libc::c_int | 0o100 as libc::c_int;
        let mut outfd: libc::c_int = open(
            outfile,
            oflags,
            0o400 as libc::c_int | 0o200 as libc::c_int
                | 0o400 as libc::c_int >> 3 as libc::c_int
                | 0o200 as libc::c_int >> 3 as libc::c_int
                | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
                | 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int,
        );
        if outfd < 0 as libc::c_int {
            sort_die(
                gettext(b"open failed\0" as *const u8 as *const libc::c_char),
                outfile,
            );
        }
        move_fd(outfd, 1 as libc::c_int);
    }
}
unsafe extern "C" fn merge(
    mut files: *mut sortfile,
    mut ntemps: size_t,
    mut nfiles: size_t,
    mut output_file: *const libc::c_char,
) {
    while (nmerge as libc::c_ulong) < nfiles {
        let mut in_0: size_t = 0;
        let mut out: size_t = 0;
        let mut remainder: size_t = 0;
        let mut cheap_slots: size_t = 0;
        in_0 = 0 as libc::c_int as size_t;
        out = in_0;
        while nmerge as libc::c_ulong <= nfiles.wrapping_sub(in_0) {
            let mut tfp: *mut FILE = 0 as *mut FILE;
            let mut temp: *mut tempnode = create_temp(&mut tfp);
            let mut num_merged: size_t = mergefiles(
                &mut *files.offset(in_0 as isize),
                if ntemps < nmerge as libc::c_ulong {
                    ntemps
                } else {
                    nmerge as libc::c_ulong
                },
                nmerge as size_t,
                tfp,
                ((*temp).name).as_mut_ptr(),
            );
            ntemps = (ntemps as libc::c_ulong)
                .wrapping_sub(if ntemps < num_merged { ntemps } else { num_merged })
                as size_t as size_t;
            let ref mut fresh48 = (*files.offset(out as isize)).name;
            *fresh48 = ((*temp).name).as_mut_ptr();
            let ref mut fresh49 = (*files.offset(out as isize)).temp;
            *fresh49 = temp;
            in_0 = (in_0 as libc::c_ulong).wrapping_add(num_merged) as size_t as size_t;
            out = out.wrapping_add(1);
            out;
        }
        remainder = nfiles.wrapping_sub(in_0);
        cheap_slots = (nmerge as libc::c_ulong)
            .wrapping_sub(out.wrapping_rem(nmerge as libc::c_ulong));
        if cheap_slots < remainder {
            let mut nshortmerge: size_t = remainder
                .wrapping_sub(cheap_slots)
                .wrapping_add(1 as libc::c_int as libc::c_ulong);
            let mut tfp_0: *mut FILE = 0 as *mut FILE;
            let mut temp_0: *mut tempnode = create_temp(&mut tfp_0);
            let mut num_merged_0: size_t = mergefiles(
                &mut *files.offset(in_0 as isize),
                if ntemps < nshortmerge { ntemps } else { nshortmerge },
                nshortmerge,
                tfp_0,
                ((*temp_0).name).as_mut_ptr(),
            );
            ntemps = (ntemps as libc::c_ulong)
                .wrapping_sub(if ntemps < num_merged_0 { ntemps } else { num_merged_0 })
                as size_t as size_t;
            let ref mut fresh50 = (*files.offset(out as isize)).name;
            *fresh50 = ((*temp_0).name).as_mut_ptr();
            let fresh51 = out;
            out = out.wrapping_add(1);
            let ref mut fresh52 = (*files.offset(fresh51 as isize)).temp;
            *fresh52 = temp_0;
            in_0 = (in_0 as libc::c_ulong).wrapping_add(num_merged_0) as size_t
                as size_t;
        }
        memmove(
            &mut *files.offset(out as isize) as *mut sortfile as *mut libc::c_void,
            &mut *files.offset(in_0 as isize) as *mut sortfile as *const libc::c_void,
            nfiles
                .wrapping_sub(in_0)
                .wrapping_mul(::core::mem::size_of::<sortfile>() as libc::c_ulong),
        );
        ntemps = (ntemps as libc::c_ulong).wrapping_add(out) as size_t as size_t;
        nfiles = (nfiles as libc::c_ulong).wrapping_sub(in_0.wrapping_sub(out)) as size_t
            as size_t;
    }
    avoid_trashing_input(files, ntemps, nfiles, output_file);
    loop {
        let mut fps: *mut *mut FILE = 0 as *mut *mut FILE;
        let mut nopened: size_t = open_input_files(files, nfiles, &mut fps);
        if nopened == nfiles {
            let mut ofp: *mut FILE = stream_open(
                output_file,
                b"w\0" as *const u8 as *const libc::c_char,
            );
            if !ofp.is_null() {
                mergefps(files, ntemps, nfiles, ofp, output_file, fps);
                break;
            } else if *__errno_location() != 24 as libc::c_int
                || nopened <= 2 as libc::c_int as libc::c_ulong
            {
                sort_die(
                    gettext(b"open failed\0" as *const u8 as *const libc::c_char),
                    output_file,
                );
            }
        } else if nopened <= 2 as libc::c_int as libc::c_ulong {
            sort_die(
                gettext(b"open failed\0" as *const u8 as *const libc::c_char),
                (*files.offset(nopened as isize)).name,
            );
        }
        let mut tfp_1: *mut FILE = 0 as *mut FILE;
        let mut temp_1: *mut tempnode = 0 as *mut tempnode;
        loop {
            nopened = nopened.wrapping_sub(1);
            nopened;
            xfclose(
                *fps.offset(nopened as isize),
                (*files.offset(nopened as isize)).name,
            );
            temp_1 = maybe_create_temp(
                &mut tfp_1,
                !(nopened <= 2 as libc::c_int as libc::c_ulong),
            );
            if !temp_1.is_null() {
                break;
            }
        }
        mergefps(
            &mut *files.offset(0 as libc::c_int as isize),
            if ntemps < nopened { ntemps } else { nopened },
            nopened,
            tfp_1,
            ((*temp_1).name).as_mut_ptr(),
            fps,
        );
        ntemps = (ntemps as libc::c_ulong)
            .wrapping_sub(if ntemps < nopened { ntemps } else { nopened }) as size_t
            as size_t;
        let ref mut fresh53 = (*files.offset(0 as libc::c_int as isize)).name;
        *fresh53 = ((*temp_1).name).as_mut_ptr();
        let ref mut fresh54 = (*files.offset(0 as libc::c_int as isize)).temp;
        *fresh54 = temp_1;
        memmove(
            &mut *files.offset(1 as libc::c_int as isize) as *mut sortfile
                as *mut libc::c_void,
            &mut *files.offset(nopened as isize) as *mut sortfile as *const libc::c_void,
            nfiles
                .wrapping_sub(nopened)
                .wrapping_mul(::core::mem::size_of::<sortfile>() as libc::c_ulong),
        );
        ntemps = ntemps.wrapping_add(1);
        ntemps;
        nfiles = (nfiles as libc::c_ulong)
            .wrapping_sub(nopened.wrapping_sub(1 as libc::c_int as libc::c_ulong))
            as size_t as size_t;
    };
}
unsafe extern "C" fn sort(
    mut files: *const *mut libc::c_char,
    mut nfiles: size_t,
    mut output_file: *const libc::c_char,
    mut nthreads: size_t,
) {
    let mut buf: buffer = buffer {
        buf: 0 as *mut libc::c_char,
        used: 0,
        nlines: 0,
        alloc: 0,
        left: 0,
        line_bytes: 0,
        eof: false,
    };
    let mut ntemps: size_t = 0 as libc::c_int as size_t;
    let mut output_file_created: bool = 0 as libc::c_int != 0;
    buf.alloc = 0 as libc::c_int as size_t;
    's_10: while nfiles != 0 {
        let mut temp_output: *const libc::c_char = 0 as *const libc::c_char;
        let mut file: *const libc::c_char = *files;
        let mut fp: *mut FILE = xfopen(file, b"r\0" as *const u8 as *const libc::c_char);
        let mut tfp: *mut FILE = 0 as *mut FILE;
        let mut bytes_per_line: size_t = 0;
        if nthreads > 1 as libc::c_int as libc::c_ulong {
            let mut tmp: size_t = 1 as libc::c_int as size_t;
            let mut mult: size_t = 1 as libc::c_int as size_t;
            while tmp < nthreads {
                tmp = (tmp as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
                mult = mult.wrapping_add(1);
                mult;
            }
            bytes_per_line = mult
                .wrapping_mul(::core::mem::size_of::<line>() as libc::c_ulong);
        } else {
            bytes_per_line = (::core::mem::size_of::<line>() as libc::c_ulong)
                .wrapping_mul(3 as libc::c_int as libc::c_ulong)
                .wrapping_div(2 as libc::c_int as libc::c_ulong);
        }
        if buf.alloc == 0 {
            initbuf(
                &mut buf,
                bytes_per_line,
                sort_buffer_size(
                    &mut fp,
                    1 as libc::c_int as size_t,
                    files,
                    nfiles,
                    bytes_per_line,
                ),
            );
        }
        buf.eof = 0 as libc::c_int != 0;
        files = files.offset(1);
        files;
        nfiles = nfiles.wrapping_sub(1);
        nfiles;
        while fillbuf(&mut buf, fp, file) {
            let mut line: *mut line = 0 as *mut line;
            if buf.eof as libc::c_int != 0 && nfiles != 0
                && bytes_per_line.wrapping_add(1 as libc::c_int as libc::c_ulong)
                    < (buf.alloc)
                        .wrapping_sub(buf.used)
                        .wrapping_sub(bytes_per_line.wrapping_mul(buf.nlines))
            {
                buf.left = buf.used;
                break;
            } else {
                saved_line.text = 0 as *mut libc::c_char;
                line = buffer_linelim(&mut buf);
                if buf.eof as libc::c_int != 0 && nfiles == 0 && ntemps == 0
                    && buf.left == 0
                {
                    xfclose(fp, file);
                    tfp = xfopen(
                        output_file,
                        b"w\0" as *const u8 as *const libc::c_char,
                    );
                    temp_output = output_file;
                    output_file_created = 1 as libc::c_int != 0;
                } else {
                    ntemps = ntemps.wrapping_add(1);
                    ntemps;
                    temp_output = ((*create_temp(&mut tfp)).name).as_mut_ptr();
                }
                if (1 as libc::c_int as libc::c_ulong) < buf.nlines {
                    let mut queue: merge_node_queue = merge_node_queue {
                        priority_queue: 0 as *mut heap,
                        mutex: pthread_mutex_t {
                            __data: __pthread_mutex_s {
                                __lock: 0,
                                __count: 0,
                                __owner: 0,
                                __nusers: 0,
                                __kind: 0,
                                __spins: 0,
                                __list: __pthread_list_t {
                                    __prev: 0 as *mut __pthread_internal_list,
                                    __next: 0 as *mut __pthread_internal_list,
                                },
                            },
                        },
                        cond: pthread_cond_t {
                            __data: __pthread_cond_s {
                                __wseq: __atomic_wide_counter {
                                    __value64: 0,
                                },
                                __g1_start: __atomic_wide_counter {
                                    __value64: 0,
                                },
                                __g_refs: [0; 2],
                                __g_size: [0; 2],
                                __g1_orig_size: 0,
                                __wrefs: 0,
                                __g_signals: [0; 2],
                            },
                        },
                    };
                    queue_init(&mut queue, nthreads);
                    let mut merge_tree: *mut merge_node = merge_tree_init(
                        nthreads,
                        buf.nlines,
                        line,
                    );
                    sortlines(
                        line,
                        nthreads,
                        buf.nlines,
                        merge_tree.offset(1 as libc::c_int as isize),
                        &mut queue,
                        tfp,
                        temp_output,
                    );
                    merge_tree_destroy(nthreads, merge_tree);
                    queue_destroy(&mut queue);
                } else {
                    write_unique(
                        line.offset(-(1 as libc::c_int as isize)),
                        tfp,
                        temp_output,
                    );
                }
                xfclose(tfp, temp_output);
                if output_file_created {
                    break 's_10;
                }
            }
        }
        xfclose(fp, file);
    }
    free(buf.buf as *mut libc::c_void);
    if !output_file_created {
        let mut node: *mut tempnode = temphead;
        let mut tempfiles: *mut sortfile = xnmalloc(
            ntemps,
            ::core::mem::size_of::<sortfile>() as libc::c_ulong,
        ) as *mut sortfile;
        let mut i: size_t = 0 as libc::c_int as size_t;
        while !node.is_null() {
            let ref mut fresh55 = (*tempfiles.offset(i as isize)).name;
            *fresh55 = ((*node).name).as_mut_ptr();
            let ref mut fresh56 = (*tempfiles.offset(i as isize)).temp;
            *fresh56 = node;
            node = (*node).next;
            i = i.wrapping_add(1);
            i;
        }
        merge(tempfiles, ntemps, ntemps, output_file);
        free(tempfiles as *mut libc::c_void);
    }
    reap_all();
}
unsafe extern "C" fn insertkey(mut key_arg: *mut keyfield) {
    let mut p: *mut *mut keyfield = 0 as *mut *mut keyfield;
    let mut key: *mut keyfield = xmemdup(
        key_arg as *const libc::c_void,
        ::core::mem::size_of::<keyfield>() as libc::c_ulong,
    ) as *mut keyfield;
    p = &mut keylist;
    while !(*p).is_null() {
        p = &mut (**p).next;
    }
    *p = key;
    (*key).next = 0 as *mut keyfield;
}
unsafe extern "C" fn badfieldspec(
    mut spec: *const libc::c_char,
    mut msgid: *const libc::c_char,
) {
    if 0 != 0 {
        error(
            SORT_FAILURE as libc::c_int,
            0 as libc::c_int,
            gettext(
                b"%s: invalid field specification %s\0" as *const u8
                    as *const libc::c_char,
            ),
            gettext(msgid),
            quote(spec),
        );
        if SORT_FAILURE as libc::c_int != 0 as libc::c_int {
            unreachable!();
        } else {};
    } else {
        ({
            let __errstatus: libc::c_int = SORT_FAILURE as libc::c_int;
            error(
                __errstatus,
                0 as libc::c_int,
                gettext(
                    b"%s: invalid field specification %s\0" as *const u8
                        as *const libc::c_char,
                ),
                gettext(msgid),
                quote(spec),
            );
            if __errstatus != 0 as libc::c_int {
                unreachable!();
            } else {};
            
        });
        ({
            let __errstatus: libc::c_int = SORT_FAILURE as libc::c_int;
            error(
                __errstatus,
                0 as libc::c_int,
                gettext(
                    b"%s: invalid field specification %s\0" as *const u8
                        as *const libc::c_char,
                ),
                gettext(msgid),
                quote(spec),
            );
            if __errstatus != 0 as libc::c_int {
                unreachable!();
            } else {};
            
        });
    };
}
unsafe extern "C" fn incompatible_options(mut opts: *const libc::c_char) {
    if 0 != 0 {
        error(
            SORT_FAILURE as libc::c_int,
            0 as libc::c_int,
            gettext(
                b"options '-%s' are incompatible\0" as *const u8 as *const libc::c_char,
            ),
            opts,
        );
        if SORT_FAILURE as libc::c_int != 0 as libc::c_int {
            unreachable!();
        } else {};
    } else {
        ({
            let __errstatus: libc::c_int = SORT_FAILURE as libc::c_int;
            error(
                __errstatus,
                0 as libc::c_int,
                gettext(
                    b"options '-%s' are incompatible\0" as *const u8
                        as *const libc::c_char,
                ),
                opts,
            );
            if __errstatus != 0 as libc::c_int {
                unreachable!();
            } else {};
            
        });
        ({
            let __errstatus: libc::c_int = SORT_FAILURE as libc::c_int;
            error(
                __errstatus,
                0 as libc::c_int,
                gettext(
                    b"options '-%s' are incompatible\0" as *const u8
                        as *const libc::c_char,
                ),
                opts,
            );
            if __errstatus != 0 as libc::c_int {
                unreachable!();
            } else {};
            
        });
    };
}
unsafe extern "C" fn check_ordering_compatibility() {
    let mut key: *mut keyfield = 0 as *mut keyfield;
    key = keylist;
    while !key.is_null() {
        if (1 as libc::c_int)
            < (*key).numeric as libc::c_int + (*key).general_numeric as libc::c_int
                + (*key).human_numeric as libc::c_int + (*key).month as libc::c_int
                + ((*key).version as libc::c_int | (*key).random as libc::c_int
                    | !((*key).ignore).is_null() as libc::c_int)
        {
            let mut opts: [libc::c_char; 31] = [0; 31];
            (*key).reverse = 0 as libc::c_int != 0;
            (*key).skipeblanks = (*key).reverse;
            (*key).skipsblanks = (*key).skipeblanks;
            key_to_opts(key, opts.as_mut_ptr());
            incompatible_options(opts.as_mut_ptr());
        }
        key = (*key).next;
    }
}
unsafe extern "C" fn parse_field_count(
    mut string: *const libc::c_char,
    mut val: *mut size_t,
    mut msgid: *const libc::c_char,
) -> *const libc::c_char {
    let mut suffix: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: uintmax_t = 0;
    let mut current_block_5: u64;
    match xstrtoumax(
        string,
        &mut suffix,
        10 as libc::c_int,
        &mut n,
        b"\0" as *const u8 as *const libc::c_char,
    ) as libc::c_uint
    {
        0 | 2 => {
            *val = n;
            if *val == n {
                current_block_5 = 13513818773234778473;
            } else {
                current_block_5 = 6918851035123944647;
            }
        }
        1 | 3 => {
            current_block_5 = 6918851035123944647;
        }
        4 => {
            if !msgid.is_null() {
                if 0 != 0 {
                    error(
                        SORT_FAILURE as libc::c_int,
                        0 as libc::c_int,
                        gettext(
                            b"%s: invalid count at start of %s\0" as *const u8
                                as *const libc::c_char,
                        ),
                        gettext(msgid),
                        quote(string),
                    );
                    if SORT_FAILURE as libc::c_int != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                } else {
                    ({
                        let __errstatus: libc::c_int = SORT_FAILURE as libc::c_int;
                        error(
                            __errstatus,
                            0 as libc::c_int,
                            gettext(
                                b"%s: invalid count at start of %s\0" as *const u8
                                    as *const libc::c_char,
                            ),
                            gettext(msgid),
                            quote(string),
                        );
                        if __errstatus != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                        
                    });
                    ({
                        let __errstatus: libc::c_int = SORT_FAILURE as libc::c_int;
                        error(
                            __errstatus,
                            0 as libc::c_int,
                            gettext(
                                b"%s: invalid count at start of %s\0" as *const u8
                                    as *const libc::c_char,
                            ),
                            gettext(msgid),
                            quote(string),
                        );
                        if __errstatus != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                        
                    });
                };
            }
            return 0 as *const libc::c_char;
        }
        _ => {
            current_block_5 = 13513818773234778473;
        }
    }
    match current_block_5 {
        6918851035123944647 => {
            *val = 18446744073709551615 as libc::c_ulong;
        }
        _ => {}
    }
    return suffix;
}
unsafe extern "C" fn sighandler(mut sig: libc::c_int) {
    if 1 as libc::c_int == 0 {
        signal(
            sig,
            ::core::mem::transmute::<
                libc::intptr_t,
                __sighandler_t,
            >(1 as libc::c_int as libc::intptr_t),
        );
    }
    cleanup();
    signal(sig, None);
    raise(sig);
}
unsafe extern "C" fn set_ordering(
    mut s: *const libc::c_char,
    mut key: *mut keyfield,
    mut blanktype: blanktype,
) -> *mut libc::c_char {
    while *s != 0 {
        match *s as libc::c_int {
            98 => {
                if blanktype as libc::c_uint == bl_start as libc::c_int as libc::c_uint
                    || blanktype as libc::c_uint
                        == bl_both as libc::c_int as libc::c_uint
                {
                    (*key).skipsblanks = 1 as libc::c_int != 0;
                }
                if blanktype as libc::c_uint == bl_end as libc::c_int as libc::c_uint
                    || blanktype as libc::c_uint
                        == bl_both as libc::c_int as libc::c_uint
                {
                    (*key).skipeblanks = 1 as libc::c_int != 0;
                }
            }
            100 => {
                (*key).ignore = nondictionary.as_mut_ptr();
            }
            102 => {
                (*key).translate = fold_toupper.as_mut_ptr();
            }
            103 => {
                (*key).general_numeric = 1 as libc::c_int != 0;
            }
            104 => {
                (*key).human_numeric = 1 as libc::c_int != 0;
            }
            105 => {
                if ((*key).ignore).is_null() {
                    (*key).ignore = nonprinting.as_mut_ptr();
                }
            }
            77 => {
                (*key).month = 1 as libc::c_int != 0;
            }
            110 => {
                (*key).numeric = 1 as libc::c_int != 0;
            }
            82 => {
                (*key).random = 1 as libc::c_int != 0;
            }
            114 => {
                (*key).reverse = 1 as libc::c_int != 0;
            }
            86 => {
                (*key).version = 1 as libc::c_int != 0;
            }
            _ => return s as *mut libc::c_char,
        }
        s = s.offset(1);
        s;
    }
    return s as *mut libc::c_char;
}
unsafe extern "C" fn key_init(mut key: *mut keyfield) -> *mut keyfield {
    memset(
        key as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<keyfield>() as libc::c_ulong,
    );
    (*key).eword = 18446744073709551615 as libc::c_ulong;
    return key;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut key: *mut keyfield = 0 as *mut keyfield;
    let mut key_buf: keyfield = keyfield {
        sword: 0,
        schar: 0,
        eword: 0,
        echar: 0,
        ignore: 0 as *const bool,
        translate: 0 as *const libc::c_char,
        skipsblanks: false,
        skipeblanks: false,
        numeric: false,
        random: false,
        general_numeric: false,
        human_numeric: false,
        month: false,
        reverse: false,
        version: false,
        traditional_used: false,
        next: 0 as *mut keyfield,
    };
    let mut gkey: keyfield = keyfield {
        sword: 0,
        schar: 0,
        eword: 0,
        echar: 0,
        ignore: 0 as *const bool,
        translate: 0 as *const libc::c_char,
        skipsblanks: false,
        skipeblanks: false,
        numeric: false,
        random: false,
        general_numeric: false,
        human_numeric: false,
        month: false,
        reverse: false,
        version: false,
        traditional_used: false,
        next: 0 as *mut keyfield,
    };
    let mut gkey_only: bool = 0 as libc::c_int != 0;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut c: libc::c_int = 0 as libc::c_int;
    let mut checkonly: libc::c_char = 0 as libc::c_int as libc::c_char;
    let mut mergeonly: bool = 0 as libc::c_int != 0;
    let mut random_source: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut need_random: bool = 0 as libc::c_int != 0;
    let mut nthreads: size_t = 0 as libc::c_int as size_t;
    let mut nfiles: size_t = 0 as libc::c_int as size_t;
    let mut posixly_correct: bool = !(getenv(
        b"POSIXLY_CORRECT\0" as *const u8 as *const libc::c_char,
    ))
        .is_null();
    let mut posix_ver: libc::c_int = posix2_version();
    let mut traditional_usage: bool = !(200112 as libc::c_int <= posix_ver
        && posix_ver < 200809 as libc::c_int);
    let mut files: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut files_from: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tok: Tokens = Tokens {
        n_tok: 0,
        tok: 0 as *mut *mut libc::c_char,
        tok_len: 0 as *mut size_t,
        o_data: obstack {
            chunk_size: 0,
            chunk: 0 as *mut _obstack_chunk,
            object_base: 0 as *mut libc::c_char,
            next_free: 0 as *mut libc::c_char,
            chunk_limit: 0 as *mut libc::c_char,
            temp: C2RustUnnamed_15 { i: 0 },
            alignment_mask: 0,
            chunkfun: C2RustUnnamed_14 { plain: None },
            freefun: C2RustUnnamed_13 { plain: None },
            extra_arg: 0 as *mut libc::c_void,
            use_extra_arg_maybe_empty_object_alloc_failed: [0; 1],
            c2rust_padding: [0; 7],
        },
        o_tok: obstack {
            chunk_size: 0,
            chunk: 0 as *mut _obstack_chunk,
            object_base: 0 as *mut libc::c_char,
            next_free: 0 as *mut libc::c_char,
            chunk_limit: 0 as *mut libc::c_char,
            temp: C2RustUnnamed_15 { i: 0 },
            alignment_mask: 0,
            chunkfun: C2RustUnnamed_14 { plain: None },
            freefun: C2RustUnnamed_13 { plain: None },
            extra_arg: 0 as *mut libc::c_void,
            use_extra_arg_maybe_empty_object_alloc_failed: [0; 1],
            c2rust_padding: [0; 7],
        },
        o_tok_len: obstack {
            chunk_size: 0,
            chunk: 0 as *mut _obstack_chunk,
            object_base: 0 as *mut libc::c_char,
            next_free: 0 as *mut libc::c_char,
            chunk_limit: 0 as *mut libc::c_char,
            temp: C2RustUnnamed_15 { i: 0 },
            alignment_mask: 0,
            chunkfun: C2RustUnnamed_14 { plain: None },
            freefun: C2RustUnnamed_13 { plain: None },
            extra_arg: 0 as *mut libc::c_void,
            use_extra_arg_maybe_empty_object_alloc_failed: [0; 1],
            c2rust_padding: [0; 7],
        },
    };
    let mut outfile: *const libc::c_char = 0 as *const libc::c_char;
    let mut locale_ok: bool = false;
    set_program_name(*argv.offset(0 as libc::c_int as isize));
    locale_ok = !(setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char))
        .is_null();
    bindtextdomain(
        b"coreutils\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"coreutils\0" as *const u8 as *const libc::c_char);
    initialize_exit_failure(SORT_FAILURE as libc::c_int);
    hard_LC_COLLATE = hard_locale(3 as libc::c_int);
    hard_LC_TIME = hard_locale(2 as libc::c_int);
    let mut locale: *const lconv = localeconv();
    decimal_point = *((*locale).decimal_point).offset(0 as libc::c_int as isize);
    if decimal_point == 0
        || *((*locale).decimal_point).offset(1 as libc::c_int as isize) as libc::c_int
            != 0
    {
        decimal_point = '.' as i32 as libc::c_char;
    }
    thousands_sep = *((*locale).thousands_sep).offset(0 as libc::c_int as isize)
        as libc::c_int;
    if thousands_sep != 0
        && *((*locale).thousands_sep).offset(1 as libc::c_int as isize) as libc::c_int
            != 0
    {
        thousands_sep_ignored = 1 as libc::c_int != 0;
    }
    if thousands_sep == 0
        || *((*locale).thousands_sep).offset(1 as libc::c_int as isize) as libc::c_int
            != 0
    {
        thousands_sep = NON_CHAR as libc::c_int;
    }
    have_read_stdin = 0 as libc::c_int != 0;
    inittables();
    let mut i: size_t = 0;
    static mut sig: [libc::c_int; 11] = [
        14 as libc::c_int,
        1 as libc::c_int,
        2 as libc::c_int,
        13 as libc::c_int,
        3 as libc::c_int,
        15 as libc::c_int,
        29 as libc::c_int,
        27 as libc::c_int,
        26 as libc::c_int,
        24 as libc::c_int,
        25 as libc::c_int,
    ];
    let mut act: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_11 {
            sa_handler: None,
        },
        sa_mask: sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    sigemptyset(&mut caught_signals);
    i = 0 as libc::c_int as size_t;
    while i < nsigs as libc::c_int as libc::c_ulong {
        sigaction(sig[i as usize], 0 as *const sigaction, &mut act);
        if act.__sigaction_handler.sa_handler
            != ::core::mem::transmute::<
                libc::intptr_t,
                __sighandler_t,
            >(1 as libc::c_int as libc::intptr_t)
        {
            sigaddset(&mut caught_signals, sig[i as usize]);
        }
        i = i.wrapping_add(1);
        i;
    }
    act
        .__sigaction_handler
        .sa_handler = Some(sighandler as unsafe extern "C" fn(libc::c_int) -> ());
    act.sa_mask = caught_signals;
    act.sa_flags = 0 as libc::c_int;
    i = 0 as libc::c_int as size_t;
    while i < nsigs as libc::c_int as libc::c_ulong {
        if sigismember(&mut caught_signals, sig[i as usize]) != 0 {
            sigaction(sig[i as usize], &mut act, 0 as *mut sigaction);
        }
        i = i.wrapping_add(1);
        i;
    }
    signal(17 as libc::c_int, None);
    atexit(Some(exit_cleanup as unsafe extern "C" fn() -> ()));
    key_init(&mut gkey);
    gkey.sword = 18446744073709551615 as libc::c_ulong;
    files = xnmalloc(
        argc as size_t,
        ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
    ) as *mut *mut libc::c_char;
    loop {
        let mut oi: libc::c_int = -(1 as libc::c_int);
        if c == -(1 as libc::c_int)
            || posixly_correct as libc::c_int != 0
                && nfiles != 0 as libc::c_int as libc::c_ulong
                && !(traditional_usage as libc::c_int != 0 && checkonly == 0
                    && optind != argc
                    && *(*argv.offset(optind as isize)).offset(0 as libc::c_int as isize)
                        as libc::c_int == '-' as i32
                    && *(*argv.offset(optind as isize)).offset(1 as libc::c_int as isize)
                        as libc::c_int == 'o' as i32
                    && (*(*argv.offset(optind as isize))
                        .offset(2 as libc::c_int as isize) as libc::c_int != 0
                        || optind + 1 as libc::c_int != argc))
            || {
                c = getopt_long(
                    argc,
                    argv,
                    short_options.as_ptr(),
                    long_options.as_ptr(),
                    &mut oi,
                );
                c == -(1 as libc::c_int)
            }
        {
            if argc <= optind {
                break;
            }
            let fresh57 = optind;
            optind = optind + 1;
            let fresh58 = nfiles;
            nfiles = nfiles.wrapping_add(1);
            let ref mut fresh59 = *files.offset(fresh58 as isize);
            *fresh59 = *argv.offset(fresh57 as isize);
        } else {
            let mut current_block_151: u64;
            match c {
                1 => {
                    key = 0 as *mut keyfield;
                    if *optarg.offset(0 as libc::c_int as isize) as libc::c_int
                        == '+' as i32
                    {
                        let mut minus_pos_usage: bool = optind != argc
                            && *(*argv.offset(optind as isize))
                                .offset(0 as libc::c_int as isize) as libc::c_int
                                == '-' as i32
                            && (*(*argv.offset(optind as isize))
                                .offset(1 as libc::c_int as isize) as libc::c_uint)
                                .wrapping_sub('0' as i32 as libc::c_uint)
                                <= 9 as libc::c_int as libc::c_uint;
                        traditional_usage = (traditional_usage as libc::c_int
                            | (minus_pos_usage as libc::c_int != 0 && !posixly_correct)
                                as libc::c_int) as bool;
                        if traditional_usage {
                            key = key_init(&mut key_buf);
                            s = parse_field_count(
                                optarg.offset(1 as libc::c_int as isize),
                                &mut (*key).sword,
                                0 as *const libc::c_char,
                            );
                            if !s.is_null() && *s as libc::c_int == '.' as i32 {
                                s = parse_field_count(
                                    s.offset(1 as libc::c_int as isize),
                                    &mut (*key).schar,
                                    0 as *const libc::c_char,
                                );
                            }
                            if !((*key).sword != 0 || (*key).schar != 0) {
                                (*key).sword = 18446744073709551615 as libc::c_ulong;
                            }
                            if s.is_null()
                                || *set_ordering(s, key, bl_start) as libc::c_int != 0
                            {
                                key = 0 as *mut keyfield;
                            } else {
                                if minus_pos_usage {
                                    let fresh60 = optind;
                                    optind = optind + 1;
                                    let mut optarg1: *const libc::c_char = *argv
                                        .offset(fresh60 as isize);
                                    s = parse_field_count(
                                        optarg1.offset(1 as libc::c_int as isize),
                                        &mut (*key).eword,
                                        b"invalid number after '-'\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    if *s as libc::c_int == '.' as i32 {
                                        s = parse_field_count(
                                            s.offset(1 as libc::c_int as isize),
                                            &mut (*key).echar,
                                            b"invalid number after '.'\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                    }
                                    if (*key).echar == 0 && (*key).eword != 0 {
                                        (*key).eword = ((*key).eword).wrapping_sub(1);
                                        (*key).eword;
                                    }
                                    if *set_ordering(s, key, bl_end) != 0 {
                                        badfieldspec(
                                            optarg1,
                                            b"stray character in field spec\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                    }
                                }
                                (*key).traditional_used = 1 as libc::c_int != 0;
                                insertkey(key);
                            }
                        }
                    }
                    if key.is_null() {
                        let fresh61 = nfiles;
                        nfiles = nfiles.wrapping_add(1);
                        let ref mut fresh62 = *files.offset(fresh61 as isize);
                        *fresh62 = optarg;
                    }
                    current_block_151 = 5832582820025303349;
                }
                262 => {
                    c = sort_types[__xargmatch_internal(
                        b"--sort\0" as *const u8 as *const libc::c_char,
                        optarg,
                        sort_args.as_ptr(),
                        sort_types.as_ptr() as *const libc::c_void,
                        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        argmatch_die,
                        1 as libc::c_int != 0,
                    ) as usize] as libc::c_int;
                    current_block_151 = 4415811450919398055;
                }
                98 | 100 | 102 | 103 | 104 | 105 | 77 | 110 | 114 | 82 | 86 => {
                    current_block_151 = 4415811450919398055;
                }
                256 => {
                    c = if !optarg.is_null() {
                        check_types[__xargmatch_internal(
                            b"--check\0" as *const u8 as *const libc::c_char,
                            optarg,
                            check_args.as_ptr(),
                            check_types.as_ptr() as *const libc::c_void,
                            ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                            argmatch_die,
                            1 as libc::c_int != 0,
                        ) as usize] as libc::c_int
                    } else {
                        'c' as i32
                    };
                    current_block_151 = 17089386665052453066;
                }
                99 | 67 => {
                    current_block_151 = 17089386665052453066;
                }
                257 => {
                    if !compress_program.is_null()
                        && !(strcmp(compress_program, optarg) == 0 as libc::c_int)
                    {
                        if 0 != 0 {
                            error(
                                SORT_FAILURE as libc::c_int,
                                0 as libc::c_int,
                                gettext(
                                    b"multiple compress programs specified\0" as *const u8
                                        as *const libc::c_char,
                                ),
                            );
                            if SORT_FAILURE as libc::c_int != 0 as libc::c_int {
                                unreachable!();
                            } else {};
                        } else {
                            ({
                                let __errstatus: libc::c_int = SORT_FAILURE as libc::c_int;
                                error(
                                    __errstatus,
                                    0 as libc::c_int,
                                    gettext(
                                        b"multiple compress programs specified\0" as *const u8
                                            as *const libc::c_char,
                                    ),
                                );
                                if __errstatus != 0 as libc::c_int {
                                    unreachable!();
                                } else {};
                                
                            });
                            ({
                                let __errstatus: libc::c_int = SORT_FAILURE as libc::c_int;
                                error(
                                    __errstatus,
                                    0 as libc::c_int,
                                    gettext(
                                        b"multiple compress programs specified\0" as *const u8
                                            as *const libc::c_char,
                                    ),
                                );
                                if __errstatus != 0 as libc::c_int {
                                    unreachable!();
                                } else {};
                                
                            });
                        };
                    }
                    compress_program = optarg;
                    current_block_151 = 5832582820025303349;
                }
                258 => {
                    debug = 1 as libc::c_int != 0;
                    current_block_151 = 5832582820025303349;
                }
                259 => {
                    files_from = optarg;
                    current_block_151 = 5832582820025303349;
                }
                107 => {
                    key = key_init(&mut key_buf);
                    s = parse_field_count(
                        optarg,
                        &mut (*key).sword,
                        b"invalid number at field start\0" as *const u8
                            as *const libc::c_char,
                    );
                    let fresh63 = (*key).sword;
                    (*key).sword = ((*key).sword).wrapping_sub(1);
                    if fresh63 == 0 {
                        badfieldspec(
                            optarg,
                            b"field number is zero\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    if *s as libc::c_int == '.' as i32 {
                        s = parse_field_count(
                            s.offset(1 as libc::c_int as isize),
                            &mut (*key).schar,
                            b"invalid number after '.'\0" as *const u8
                                as *const libc::c_char,
                        );
                        let fresh64 = (*key).schar;
                        (*key).schar = ((*key).schar).wrapping_sub(1);
                        if fresh64 == 0 {
                            badfieldspec(
                                optarg,
                                b"character offset is zero\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                    }
                    if !((*key).sword != 0 || (*key).schar != 0) {
                        (*key).sword = 18446744073709551615 as libc::c_ulong;
                    }
                    s = set_ordering(s, key, bl_start);
                    if *s as libc::c_int != ',' as i32 {
                        (*key).eword = 18446744073709551615 as libc::c_ulong;
                        (*key).echar = 0 as libc::c_int as size_t;
                    } else {
                        s = parse_field_count(
                            s.offset(1 as libc::c_int as isize),
                            &mut (*key).eword,
                            b"invalid number after ','\0" as *const u8
                                as *const libc::c_char,
                        );
                        let fresh65 = (*key).eword;
                        (*key).eword = ((*key).eword).wrapping_sub(1);
                        if fresh65 == 0 {
                            badfieldspec(
                                optarg,
                                b"field number is zero\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                        if *s as libc::c_int == '.' as i32 {
                            s = parse_field_count(
                                s.offset(1 as libc::c_int as isize),
                                &mut (*key).echar,
                                b"invalid number after '.'\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                        s = set_ordering(s, key, bl_end);
                    }
                    if *s != 0 {
                        badfieldspec(
                            optarg,
                            b"stray character in field spec\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    insertkey(key);
                    current_block_151 = 5832582820025303349;
                }
                109 => {
                    mergeonly = 1 as libc::c_int != 0;
                    current_block_151 = 5832582820025303349;
                }
                260 => {
                    specify_nmerge(oi, c as libc::c_char, optarg);
                    current_block_151 = 5832582820025303349;
                }
                111 => {
                    if !outfile.is_null()
                        && !(strcmp(outfile, optarg) == 0 as libc::c_int)
                    {
                        if 0 != 0 {
                            error(
                                SORT_FAILURE as libc::c_int,
                                0 as libc::c_int,
                                gettext(
                                    b"multiple output files specified\0" as *const u8
                                        as *const libc::c_char,
                                ),
                            );
                            if SORT_FAILURE as libc::c_int != 0 as libc::c_int {
                                unreachable!();
                            } else {};
                        } else {
                            ({
                                let __errstatus: libc::c_int = SORT_FAILURE as libc::c_int;
                                error(
                                    __errstatus,
                                    0 as libc::c_int,
                                    gettext(
                                        b"multiple output files specified\0" as *const u8
                                            as *const libc::c_char,
                                    ),
                                );
                                if __errstatus != 0 as libc::c_int {
                                    unreachable!();
                                } else {};
                                
                            });
                            ({
                                let __errstatus: libc::c_int = SORT_FAILURE as libc::c_int;
                                error(
                                    __errstatus,
                                    0 as libc::c_int,
                                    gettext(
                                        b"multiple output files specified\0" as *const u8
                                            as *const libc::c_char,
                                    ),
                                );
                                if __errstatus != 0 as libc::c_int {
                                    unreachable!();
                                } else {};
                                
                            });
                        };
                    }
                    outfile = optarg;
                    current_block_151 = 5832582820025303349;
                }
                261 => {
                    if !random_source.is_null()
                        && !(strcmp(random_source, optarg) == 0 as libc::c_int)
                    {
                        if 0 != 0 {
                            error(
                                SORT_FAILURE as libc::c_int,
                                0 as libc::c_int,
                                gettext(
                                    b"multiple random sources specified\0" as *const u8
                                        as *const libc::c_char,
                                ),
                            );
                            if SORT_FAILURE as libc::c_int != 0 as libc::c_int {
                                unreachable!();
                            } else {};
                        } else {
                            ({
                                let __errstatus: libc::c_int = SORT_FAILURE as libc::c_int;
                                error(
                                    __errstatus,
                                    0 as libc::c_int,
                                    gettext(
                                        b"multiple random sources specified\0" as *const u8
                                            as *const libc::c_char,
                                    ),
                                );
                                if __errstatus != 0 as libc::c_int {
                                    unreachable!();
                                } else {};
                                
                            });
                            ({
                                let __errstatus: libc::c_int = SORT_FAILURE as libc::c_int;
                                error(
                                    __errstatus,
                                    0 as libc::c_int,
                                    gettext(
                                        b"multiple random sources specified\0" as *const u8
                                            as *const libc::c_char,
                                    ),
                                );
                                if __errstatus != 0 as libc::c_int {
                                    unreachable!();
                                } else {};
                                
                            });
                        };
                    }
                    random_source = optarg;
                    current_block_151 = 5832582820025303349;
                }
                115 => {
                    stable = 1 as libc::c_int != 0;
                    current_block_151 = 5832582820025303349;
                }
                83 => {
                    specify_sort_size(oi, c as libc::c_char, optarg);
                    current_block_151 = 5832582820025303349;
                }
                116 => {
                    let mut newtab: libc::c_char = *optarg
                        .offset(0 as libc::c_int as isize);
                    if newtab == 0 {
                        if 0 != 0 {
                            error(
                                SORT_FAILURE as libc::c_int,
                                0 as libc::c_int,
                                gettext(b"empty tab\0" as *const u8 as *const libc::c_char),
                            );
                            if SORT_FAILURE as libc::c_int != 0 as libc::c_int {
                                unreachable!();
                            } else {};
                        } else {
                            ({
                                let __errstatus: libc::c_int = SORT_FAILURE as libc::c_int;
                                error(
                                    __errstatus,
                                    0 as libc::c_int,
                                    gettext(b"empty tab\0" as *const u8 as *const libc::c_char),
                                );
                                if __errstatus != 0 as libc::c_int {
                                    unreachable!();
                                } else {};
                                
                            });
                            ({
                                let __errstatus: libc::c_int = SORT_FAILURE as libc::c_int;
                                error(
                                    __errstatus,
                                    0 as libc::c_int,
                                    gettext(b"empty tab\0" as *const u8 as *const libc::c_char),
                                );
                                if __errstatus != 0 as libc::c_int {
                                    unreachable!();
                                } else {};
                                
                            });
                        };
                    }
                    if *optarg.offset(1 as libc::c_int as isize) != 0 {
                        if strcmp(optarg, b"\\0\0" as *const u8 as *const libc::c_char)
                            == 0 as libc::c_int
                        {
                            newtab = '\0' as i32 as libc::c_char;
                        } else {
                            if 0 != 0 {
                                error(
                                    SORT_FAILURE as libc::c_int,
                                    0 as libc::c_int,
                                    gettext(
                                        b"multi-character tab %s\0" as *const u8
                                            as *const libc::c_char,
                                    ),
                                    quote(optarg),
                                );
                                if SORT_FAILURE as libc::c_int != 0 as libc::c_int {
                                    unreachable!();
                                } else {};
                            } else {
                                ({
                                    let __errstatus: libc::c_int = SORT_FAILURE as libc::c_int;
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
                                    let __errstatus: libc::c_int = SORT_FAILURE as libc::c_int;
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
                    }
                    if tab != TAB_DEFAULT as libc::c_int && tab != newtab as libc::c_int
                    {
                        if 0 != 0 {
                            error(
                                SORT_FAILURE as libc::c_int,
                                0 as libc::c_int,
                                gettext(
                                    b"incompatible tabs\0" as *const u8 as *const libc::c_char,
                                ),
                            );
                            if SORT_FAILURE as libc::c_int != 0 as libc::c_int {
                                unreachable!();
                            } else {};
                        } else {
                            ({
                                let __errstatus: libc::c_int = SORT_FAILURE as libc::c_int;
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
                                let __errstatus: libc::c_int = SORT_FAILURE as libc::c_int;
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
                    tab = newtab as libc::c_int;
                    current_block_151 = 5832582820025303349;
                }
                84 => {
                    add_temp_dir(optarg);
                    current_block_151 = 5832582820025303349;
                }
                263 => {
                    nthreads = specify_nthreads(oi, c as libc::c_char, optarg);
                    current_block_151 = 5832582820025303349;
                }
                117 => {
                    unique = 1 as libc::c_int != 0;
                    current_block_151 = 5832582820025303349;
                }
                121 => {
                    if optarg == *argv.offset((optind - 1 as libc::c_int) as isize) {
                        let mut p: *const libc::c_char = 0 as *const libc::c_char;
                        p = optarg;
                        while (*p as libc::c_uint)
                            .wrapping_sub('0' as i32 as libc::c_uint)
                            <= 9 as libc::c_int as libc::c_uint
                        {
                            p = p.offset(1);
                            p;
                        }
                        optind -= (*p as libc::c_int != '\0' as i32) as libc::c_int;
                    }
                    current_block_151 = 5832582820025303349;
                }
                122 => {
                    eolchar = 0 as libc::c_int as libc::c_char;
                    current_block_151 = 5832582820025303349;
                }
                -2 => {
                    usage(0 as libc::c_int);
                    current_block_151 = 5832582820025303349;
                }
                -3 => {
                    version_etc(
                        stdout,
                        b"sort\0" as *const u8 as *const libc::c_char,
                        b"GNU coreutils\0" as *const u8 as *const libc::c_char,
                        Version,
                        proper_name_lite(
                            b"Mike Haertel\0" as *const u8 as *const libc::c_char,
                            b"Mike Haertel\0" as *const u8 as *const libc::c_char,
                        ),
                        proper_name_lite(
                            b"Paul Eggert\0" as *const u8 as *const libc::c_char,
                            b"Paul Eggert\0" as *const u8 as *const libc::c_char,
                        ),
                        0 as *mut libc::c_void as *mut libc::c_char,
                    );
                    exit(0 as libc::c_int);
                }
                _ => {
                    usage(SORT_FAILURE as libc::c_int);
                    current_block_151 = 5832582820025303349;
                }
            }
            match current_block_151 {
                17089386665052453066 => {
                    if checkonly as libc::c_int != 0 && checkonly as libc::c_int != c {
                        incompatible_options(
                            b"cC\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    checkonly = c as libc::c_char;
                }
                4415811450919398055 => {
                    let mut str: [libc::c_char; 2] = [0; 2];
                    str[0 as libc::c_int as usize] = c as libc::c_char;
                    str[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
                    set_ordering(str.as_mut_ptr(), &mut gkey, bl_both);
                }
                _ => {}
            }
        }
    }
    if !files_from.is_null() {
        if nfiles != 0 {
            if 0 != 0 {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    gettext(b"extra operand %s\0" as *const u8 as *const libc::c_char),
                    quotearg_style(
                        shell_escape_always_quoting_style,
                        *files.offset(0 as libc::c_int as isize),
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
                            b"extra operand %s\0" as *const u8 as *const libc::c_char,
                        ),
                        quotearg_style(
                            shell_escape_always_quoting_style,
                            *files.offset(0 as libc::c_int as isize),
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
                            b"extra operand %s\0" as *const u8 as *const libc::c_char,
                        ),
                        quotearg_style(
                            shell_escape_always_quoting_style,
                            *files.offset(0 as libc::c_int as isize),
                        ),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
            fprintf(
                stderr,
                b"%s\n\0" as *const u8 as *const libc::c_char,
                gettext(
                    b"file operands cannot be combined with --files0-from\0" as *const u8
                        as *const libc::c_char,
                ),
            );
            usage(SORT_FAILURE as libc::c_int);
        }
        let mut stream: *mut FILE = xfopen(
            files_from,
            b"r\0" as *const u8 as *const libc::c_char,
        );
        readtokens0_init(&mut tok);
        if !readtokens0(stream, &mut tok) {
            if 0 != 0 {
                error(
                    SORT_FAILURE as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"cannot read file names from %s\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quotearg_style(shell_escape_always_quoting_style, files_from),
                );
                if SORT_FAILURE as libc::c_int != 0 as libc::c_int {
                    unreachable!();
                } else {};
            } else {
                ({
                    let __errstatus: libc::c_int = SORT_FAILURE as libc::c_int;
                    error(
                        __errstatus,
                        0 as libc::c_int,
                        gettext(
                            b"cannot read file names from %s\0" as *const u8
                                as *const libc::c_char,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, files_from),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
                ({
                    let __errstatus: libc::c_int = SORT_FAILURE as libc::c_int;
                    error(
                        __errstatus,
                        0 as libc::c_int,
                        gettext(
                            b"cannot read file names from %s\0" as *const u8
                                as *const libc::c_char,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, files_from),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
        xfclose(stream, files_from);
        if tok.n_tok != 0 {
            free(files as *mut libc::c_void);
            files = tok.tok;
            nfiles = tok.n_tok;
            let mut i_0: size_t = 0 as libc::c_int as size_t;
            while i_0 < nfiles {
                if strcmp(
                    *files.offset(i_0 as isize),
                    b"-\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    if 0 != 0 {
                        error(
                            SORT_FAILURE as libc::c_int,
                            0 as libc::c_int,
                            gettext(
                                b"when reading file names from stdin, no file name of %s allowed\0"
                                    as *const u8 as *const libc::c_char,
                            ),
                            quotearg_style(
                                shell_escape_always_quoting_style,
                                *files.offset(i_0 as isize),
                            ),
                        );
                        if SORT_FAILURE as libc::c_int != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                    } else {
                        ({
                            let __errstatus: libc::c_int = SORT_FAILURE as libc::c_int;
                            error(
                                __errstatus,
                                0 as libc::c_int,
                                gettext(
                                    b"when reading file names from stdin, no file name of %s allowed\0"
                                        as *const u8 as *const libc::c_char,
                                ),
                                quotearg_style(
                                    shell_escape_always_quoting_style,
                                    *files.offset(i_0 as isize),
                                ),
                            );
                            if __errstatus != 0 as libc::c_int {
                                unreachable!();
                            } else {};
                            
                        });
                        ({
                            let __errstatus: libc::c_int = SORT_FAILURE as libc::c_int;
                            error(
                                __errstatus,
                                0 as libc::c_int,
                                gettext(
                                    b"when reading file names from stdin, no file name of %s allowed\0"
                                        as *const u8 as *const libc::c_char,
                                ),
                                quotearg_style(
                                    shell_escape_always_quoting_style,
                                    *files.offset(i_0 as isize),
                                ),
                            );
                            if __errstatus != 0 as libc::c_int {
                                unreachable!();
                            } else {};
                            
                        });
                    };
                } else if *(*files.offset(i_0 as isize))
                    .offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
                {
                    let mut file_number: libc::c_ulong = i_0
                        .wrapping_add(1 as libc::c_int as libc::c_ulong);
                    if 0 != 0 {
                        error(
                            SORT_FAILURE as libc::c_int,
                            0 as libc::c_int,
                            gettext(
                                b"%s:%lu: invalid zero-length file name\0" as *const u8
                                    as *const libc::c_char,
                            ),
                            quotearg_n_style_colon(
                                0 as libc::c_int,
                                shell_escape_quoting_style,
                                files_from,
                            ),
                            file_number,
                        );
                        if SORT_FAILURE as libc::c_int != 0 as libc::c_int {
                            unreachable!();
                        } else {};
                    } else {
                        ({
                            let __errstatus: libc::c_int = SORT_FAILURE as libc::c_int;
                            error(
                                __errstatus,
                                0 as libc::c_int,
                                gettext(
                                    b"%s:%lu: invalid zero-length file name\0" as *const u8
                                        as *const libc::c_char,
                                ),
                                quotearg_n_style_colon(
                                    0 as libc::c_int,
                                    shell_escape_quoting_style,
                                    files_from,
                                ),
                                file_number,
                            );
                            if __errstatus != 0 as libc::c_int {
                                unreachable!();
                            } else {};
                            
                        });
                        ({
                            let __errstatus: libc::c_int = SORT_FAILURE as libc::c_int;
                            error(
                                __errstatus,
                                0 as libc::c_int,
                                gettext(
                                    b"%s:%lu: invalid zero-length file name\0" as *const u8
                                        as *const libc::c_char,
                                ),
                                quotearg_n_style_colon(
                                    0 as libc::c_int,
                                    shell_escape_quoting_style,
                                    files_from,
                                ),
                                file_number,
                            );
                            if __errstatus != 0 as libc::c_int {
                                unreachable!();
                            } else {};
                            
                        });
                    };
                }
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
        } else {
            if 0 != 0 {
                error(
                    SORT_FAILURE as libc::c_int,
                    0 as libc::c_int,
                    gettext(b"no input from %s\0" as *const u8 as *const libc::c_char),
                    quotearg_style(shell_escape_always_quoting_style, files_from),
                );
                if SORT_FAILURE as libc::c_int != 0 as libc::c_int {
                    unreachable!();
                } else {};
            } else {
                ({
                    let __errstatus: libc::c_int = SORT_FAILURE as libc::c_int;
                    error(
                        __errstatus,
                        0 as libc::c_int,
                        gettext(
                            b"no input from %s\0" as *const u8 as *const libc::c_char,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, files_from),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
                ({
                    let __errstatus: libc::c_int = SORT_FAILURE as libc::c_int;
                    error(
                        __errstatus,
                        0 as libc::c_int,
                        gettext(
                            b"no input from %s\0" as *const u8 as *const libc::c_char,
                        ),
                        quotearg_style(shell_escape_always_quoting_style, files_from),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
    }
    key = keylist;
    while !key.is_null() {
        if default_key_compare(key) as libc::c_int != 0 && !(*key).reverse {
            (*key).ignore = gkey.ignore;
            (*key).translate = gkey.translate;
            (*key).skipsblanks = gkey.skipsblanks;
            (*key).skipeblanks = gkey.skipeblanks;
            (*key).month = gkey.month;
            (*key).numeric = gkey.numeric;
            (*key).general_numeric = gkey.general_numeric;
            (*key).human_numeric = gkey.human_numeric;
            (*key).version = gkey.version;
            (*key).random = gkey.random;
            (*key).reverse = gkey.reverse;
        }
        need_random = (need_random as libc::c_int | (*key).random as libc::c_int)
            as bool;
        key = (*key).next;
    }
    if keylist.is_null() && !default_key_compare(&mut gkey) {
        gkey_only = 1 as libc::c_int != 0;
        insertkey(&mut gkey);
        need_random = (need_random as libc::c_int | gkey.random as libc::c_int) as bool;
    }
    check_ordering_compatibility();
    if debug {
        if checkonly as libc::c_int != 0 || !outfile.is_null() {
            static mut opts: [libc::c_char; 10] = unsafe {
                *::core::mem::transmute::<
                    &[u8; 10],
                    &mut [libc::c_char; 10],
                >(b"X --debug\0")
            };
            opts[0 as libc::c_int
                as usize] = (if checkonly as libc::c_int != 0 {
                checkonly as libc::c_int
            } else {
                'o' as i32
            }) as libc::c_char;
            incompatible_options(opts.as_mut_ptr());
        }
        if locale_ok {
            locale_ok = !(setlocale(
                3 as libc::c_int,
                b"\0" as *const u8 as *const libc::c_char,
            ))
                .is_null();
        }
        if !locale_ok {
            if 0 != 0 {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    gettext(
                        b"failed to set locale\0" as *const u8 as *const libc::c_char,
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
                        b"%s\0" as *const u8 as *const libc::c_char,
                        gettext(
                            b"failed to set locale\0" as *const u8 as *const libc::c_char,
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
                        b"%s\0" as *const u8 as *const libc::c_char,
                        gettext(
                            b"failed to set locale\0" as *const u8 as *const libc::c_char,
                        ),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
        if hard_LC_COLLATE {
            if 0 != 0 {
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"text ordering performed using %s sorting rules\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quote(setlocale(3 as libc::c_int, 0 as *const libc::c_char)),
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
                            b"text ordering performed using %s sorting rules\0"
                                as *const u8 as *const libc::c_char,
                        ),
                        quote(setlocale(3 as libc::c_int, 0 as *const libc::c_char)),
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
                            b"text ordering performed using %s sorting rules\0"
                                as *const u8 as *const libc::c_char,
                        ),
                        quote(setlocale(3 as libc::c_int, 0 as *const libc::c_char)),
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
                    b"%s\0" as *const u8 as *const libc::c_char,
                    gettext(
                        b"text ordering performed using simple byte comparison\0"
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
                        b"%s\0" as *const u8 as *const libc::c_char,
                        gettext(
                            b"text ordering performed using simple byte comparison\0"
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
                        b"%s\0" as *const u8 as *const libc::c_char,
                        gettext(
                            b"text ordering performed using simple byte comparison\0"
                                as *const u8 as *const libc::c_char,
                        ),
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
        key_warnings(&mut gkey, gkey_only);
    }
    reverse = gkey.reverse;
    if need_random {
        random_md5_state_init(random_source);
    }
    if temp_dir_count == 0 as libc::c_int as libc::c_ulong {
        let mut tmp_dir: *const libc::c_char = getenv(
            b"TMPDIR\0" as *const u8 as *const libc::c_char,
        );
        add_temp_dir(
            if !tmp_dir.is_null() {
                tmp_dir
            } else {
                b"/tmp\0" as *const u8 as *const libc::c_char
            },
        );
    }
    if nfiles == 0 as libc::c_int as libc::c_ulong {
        nfiles = 1 as libc::c_int as size_t;
        free(files as *mut libc::c_void);
        files = xmalloc(::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            as *mut *mut libc::c_char;
        *files = b"-\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (0 as libc::c_int as libc::c_ulong) < sort_size {
        sort_size = if sort_size
            > (nmerge as libc::c_ulong)
                .wrapping_mul(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_add(::core::mem::size_of::<line>() as libc::c_ulong),
                )
        {
            sort_size
        } else {
            (nmerge as libc::c_ulong)
                .wrapping_mul(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_add(::core::mem::size_of::<line>() as libc::c_ulong),
                )
        };
    }
    if checkonly != 0 {
        if nfiles > 1 as libc::c_int as libc::c_ulong {
            if 0 != 0 {
                error(
                    SORT_FAILURE as libc::c_int,
                    0 as libc::c_int,
                    gettext(
                        b"extra operand %s not allowed with -%c\0" as *const u8
                            as *const libc::c_char,
                    ),
                    quotearg_style(
                        shell_escape_always_quoting_style,
                        *files.offset(1 as libc::c_int as isize),
                    ),
                    checkonly as libc::c_int,
                );
                if SORT_FAILURE as libc::c_int != 0 as libc::c_int {
                    unreachable!();
                } else {};
            } else {
                ({
                    let __errstatus: libc::c_int = SORT_FAILURE as libc::c_int;
                    error(
                        __errstatus,
                        0 as libc::c_int,
                        gettext(
                            b"extra operand %s not allowed with -%c\0" as *const u8
                                as *const libc::c_char,
                        ),
                        quotearg_style(
                            shell_escape_always_quoting_style,
                            *files.offset(1 as libc::c_int as isize),
                        ),
                        checkonly as libc::c_int,
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
                ({
                    let __errstatus: libc::c_int = SORT_FAILURE as libc::c_int;
                    error(
                        __errstatus,
                        0 as libc::c_int,
                        gettext(
                            b"extra operand %s not allowed with -%c\0" as *const u8
                                as *const libc::c_char,
                        ),
                        quotearg_style(
                            shell_escape_always_quoting_style,
                            *files.offset(1 as libc::c_int as isize),
                        ),
                        checkonly as libc::c_int,
                    );
                    if __errstatus != 0 as libc::c_int {
                        unreachable!();
                    } else {};
                    
                });
            };
        }
        if !outfile.is_null() {
            static mut opts_0: [libc::c_char; 3] = [
                0 as libc::c_int as libc::c_char,
                'o' as i32 as libc::c_char,
                0 as libc::c_int as libc::c_char,
            ];
            opts_0[0 as libc::c_int as usize] = checkonly;
            incompatible_options(opts_0.as_mut_ptr());
        }
        exit(
            if check(*files.offset(0 as libc::c_int as isize), checkonly) as libc::c_int
                != 0
            {
                0 as libc::c_int
            } else {
                SORT_OUT_OF_ORDER as libc::c_int
            },
        );
    }
    check_inputs(files, nfiles);
    check_output(outfile);
    if mergeonly {
        let mut sortfiles: *mut sortfile = xcalloc(
            nfiles,
            ::core::mem::size_of::<sortfile>() as libc::c_ulong,
        ) as *mut sortfile;
        let mut i_1: size_t = 0 as libc::c_int as size_t;
        while i_1 < nfiles {
            let ref mut fresh66 = (*sortfiles.offset(i_1 as isize)).name;
            *fresh66 = *files.offset(i_1 as isize);
            i_1 = i_1.wrapping_add(1);
            i_1;
        }
        merge(sortfiles, 0 as libc::c_int as size_t, nfiles, outfile);
    } else {
        if nthreads == 0 {
            let mut np: libc::c_ulong = num_processors(NPROC_CURRENT_OVERRIDABLE);
            nthreads = if np < DEFAULT_MAX_THREADS as libc::c_int as libc::c_ulong {
                np
            } else {
                DEFAULT_MAX_THREADS as libc::c_int as libc::c_ulong
            };
        }
        let mut nthreads_max: size_t = (18446744073709551615 as libc::c_ulong)
            .wrapping_div(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<merge_node>() as libc::c_ulong),
            );
        nthreads = if nthreads < nthreads_max { nthreads } else { nthreads_max };
        sort(files, nfiles, outfile, nthreads);
    }
    if have_read_stdin as libc::c_int != 0 && rpl_fclose(stdin) == -(1 as libc::c_int) {
        sort_die(
            gettext(b"close failed\0" as *const u8 as *const libc::c_char),
            b"-\0" as *const u8 as *const libc::c_char,
        );
    }
    exit(0 as libc::c_int);
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
    merge_buffer_size = if (2 as libc::c_int as libc::c_ulong)
        .wrapping_add(::core::mem::size_of::<line>() as libc::c_ulong)
        > (256 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong
    {
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<line>() as libc::c_ulong)
    } else {
        (256 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
