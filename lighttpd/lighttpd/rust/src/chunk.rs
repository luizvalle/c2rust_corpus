use ::libc;
extern "C" {
    fn buffer_init() -> *mut buffer;
    fn buffer_free(b: *mut buffer);
    fn buffer_move(b: *mut buffer, src: *mut buffer);
    fn buffer_string_prepare_copy(b: *mut buffer, size: size_t) -> *mut libc::c_char;
    fn buffer_extend(b: *mut buffer, x: size_t) -> *mut libc::c_char;
    fn buffer_commit(b: *mut buffer, size: size_t);
    fn buffer_append_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn buffer_copy_path_len2(
        b: *mut buffer,
        s1: *const libc::c_char,
        len1: size_t,
        s2: *const libc::c_char,
        len2: size_t,
    );
    fn ck_calloc(nmemb: size_t, elt_sz: size_t) -> *mut libc::c_void;
    fn ck_assert_failed(
        filename: *const libc::c_char,
        line: libc::c_uint,
        msg: *const libc::c_char,
    ) -> !;
    fn buffer_copy_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn fdevent_dup_cloexec(fd: libc::c_int) -> libc::c_int;
    fn fdevent_open_cloexec(
        pathname: *const libc::c_char,
        symlinks: libc::c_int,
        flags: libc::c_int,
        mode: mode_t,
    ) -> libc::c_int;
    fn fdevent_pipe_cloexec(
        fds: *mut libc::c_int,
        bufsz_hint: libc::c_uint,
    ) -> libc::c_int;
    fn fdevent_mkostemp(path: *mut libc::c_char, flags: libc::c_int) -> libc::c_int;
    fn log_error(
        errh: *mut log_error_st,
        filename: *const libc::c_char,
        line: libc::c_uint,
        fmt: *const libc::c_char,
        _: ...
    );
    fn log_perror(
        errh: *mut log_error_st,
        filename: *const libc::c_char,
        line: libc::c_uint,
        fmt: *const libc::c_char,
        _: ...
    );
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn munmap(__addr: *mut libc::c_void, __len: size_t) -> libc::c_int;
    fn mmap(
        __addr: *mut libc::c_void,
        __len: size_t,
        __prot: libc::c_int,
        __flags: libc::c_int,
        __fd: libc::c_int,
        __offset: __off64_t,
    ) -> *mut libc::c_void;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn pread(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __nbytes: size_t,
        __offset: __off64_t,
    ) -> ssize_t;
    fn pwrite(
        __fd: libc::c_int,
        __buf: *const libc::c_void,
        __nbytes: size_t,
        __offset: __off64_t,
    ) -> ssize_t;
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn splice(
        __fdin: libc::c_int,
        __offin: *mut __off64_t,
        __fdout: libc::c_int,
        __offout: *mut __off64_t,
        __len: size_t,
        __flags: libc::c_uint,
    ) -> __ssize_t;
    fn __errno_location() -> *mut libc::c_int;
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
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn pwritev(
        __fd: libc::c_int,
        __iovec: *const iovec,
        __count: libc::c_int,
        __offset: __off64_t,
    ) -> ssize_t;
    fn preadv2(
        __fd: libc::c_int,
        __iovec: *const iovec,
        __count: libc::c_int,
        __offset: __off64_t,
        __flags: libc::c_int,
    ) -> ssize_t;
    fn sendfile(
        __out_fd: libc::c_int,
        __in_fd: libc::c_int,
        __offset: *mut __off64_t,
        __count: size_t,
    ) -> ssize_t;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino64_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_uint;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_int;
pub type __blkcnt64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __loff_t = __off64_t;
pub type loff_t = __loff_t;
pub type mode_t = __mode_t;
pub type off_t = __off64_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buffer {
    pub ptr: *mut libc::c_char,
    pub used: uint32_t,
    pub size: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct data_unset {
    pub key: buffer,
    pub fn_0: *const data_methods,
    pub type_0: data_type_t,
}
pub type data_type_t = libc::c_uint;
pub const TYPE_OTHER: data_type_t = 4;
pub const TYPE_CONFIG: data_type_t = 3;
pub const TYPE_INTEGER: data_type_t = 2;
pub const TYPE_ARRAY: data_type_t = 1;
pub const TYPE_STRING: data_type_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct data_methods {
    pub copy: Option::<unsafe extern "C" fn(*const data_unset) -> *mut data_unset>,
    pub free: Option::<unsafe extern "C" fn(*mut data_unset) -> ()>,
    pub insert_dup: Option::<
        unsafe extern "C" fn(*mut data_unset, *mut data_unset) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct array {
    pub data: *mut *mut data_unset,
    pub sorted: *mut *mut data_unset,
    pub used: uint32_t,
    pub size: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct data_string {
    pub key: buffer,
    pub fn_0: *const data_methods,
    pub type_0: data_type_t,
    pub ext: libc::c_int,
    pub value: buffer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fdlog_st {
    pub mode: C2RustUnnamed,
    pub fd: libc::c_int,
    pub b: buffer,
    pub fn_0: *const libc::c_char,
}
pub type C2RustUnnamed = libc::c_uint;
pub const FDLOG_PIPE: C2RustUnnamed = 3;
pub const FDLOG_SYSLOG: C2RustUnnamed = 2;
pub const FDLOG_FD: C2RustUnnamed = 1;
pub const FDLOG_FILE: C2RustUnnamed = 0;
pub type log_error_st = fdlog_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct chunk_file_view {
    pub mptr: *mut libc::c_char,
    pub mlen: off_t,
    pub foff: off_t,
    pub refcnt: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct chunk {
    pub next: *mut chunk,
    pub type_0: C2RustUnnamed_1,
    pub mem: *mut buffer,
    pub offset: off_t,
    pub file: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub length: off_t,
    pub fd: libc::c_int,
    pub is_temp: uint8_t,
    pub busy: uint8_t,
    pub flagmask: uint8_t,
    pub view: *mut chunk_file_view,
    pub ref_0: *mut libc::c_void,
    pub refchg: Option::<unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> ()>,
}
pub type C2RustUnnamed_1 = libc::c_uint;
pub const FILE_CHUNK: C2RustUnnamed_1 = 1;
pub const MEM_CHUNK: C2RustUnnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct chunkqueue {
    pub first: *mut chunk,
    pub last: *mut chunk,
    pub bytes_in: off_t,
    pub bytes_out: off_t,
    pub upload_temp_file_size: off_t,
    pub tempdir_idx: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut libc::c_void,
    pub iov_len: size_t,
}
pub const _SC_PAGESIZE: C2RustUnnamed_2 = 30;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino64_t,
    pub st_mode: __mode_t,
    pub st_nlink: __nlink_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub st_rdev: __dev_t,
    pub __pad1: __dev_t,
    pub st_size: __off64_t,
    pub st_blksize: __blksize_t,
    pub __pad2: libc::c_int,
    pub st_blocks: __blkcnt64_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [libc::c_int; 2],
}
pub type C2RustUnnamed_2 = libc::c_uint;
pub const _SC_SIGSTKSZ: C2RustUnnamed_2 = 250;
pub const _SC_MINSIGSTKSZ: C2RustUnnamed_2 = 249;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: C2RustUnnamed_2 = 248;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: C2RustUnnamed_2 = 247;
pub const _SC_XOPEN_STREAMS: C2RustUnnamed_2 = 246;
pub const _SC_TRACE_USER_EVENT_MAX: C2RustUnnamed_2 = 245;
pub const _SC_TRACE_SYS_MAX: C2RustUnnamed_2 = 244;
pub const _SC_TRACE_NAME_MAX: C2RustUnnamed_2 = 243;
pub const _SC_TRACE_EVENT_NAME_MAX: C2RustUnnamed_2 = 242;
pub const _SC_SS_REPL_MAX: C2RustUnnamed_2 = 241;
pub const _SC_V7_LPBIG_OFFBIG: C2RustUnnamed_2 = 240;
pub const _SC_V7_LP64_OFF64: C2RustUnnamed_2 = 239;
pub const _SC_V7_ILP32_OFFBIG: C2RustUnnamed_2 = 238;
pub const _SC_V7_ILP32_OFF32: C2RustUnnamed_2 = 237;
pub const _SC_RAW_SOCKETS: C2RustUnnamed_2 = 236;
pub const _SC_IPV6: C2RustUnnamed_2 = 235;
pub const _SC_LEVEL4_CACHE_LINESIZE: C2RustUnnamed_2 = 199;
pub const _SC_LEVEL4_CACHE_ASSOC: C2RustUnnamed_2 = 198;
pub const _SC_LEVEL4_CACHE_SIZE: C2RustUnnamed_2 = 197;
pub const _SC_LEVEL3_CACHE_LINESIZE: C2RustUnnamed_2 = 196;
pub const _SC_LEVEL3_CACHE_ASSOC: C2RustUnnamed_2 = 195;
pub const _SC_LEVEL3_CACHE_SIZE: C2RustUnnamed_2 = 194;
pub const _SC_LEVEL2_CACHE_LINESIZE: C2RustUnnamed_2 = 193;
pub const _SC_LEVEL2_CACHE_ASSOC: C2RustUnnamed_2 = 192;
pub const _SC_LEVEL2_CACHE_SIZE: C2RustUnnamed_2 = 191;
pub const _SC_LEVEL1_DCACHE_LINESIZE: C2RustUnnamed_2 = 190;
pub const _SC_LEVEL1_DCACHE_ASSOC: C2RustUnnamed_2 = 189;
pub const _SC_LEVEL1_DCACHE_SIZE: C2RustUnnamed_2 = 188;
pub const _SC_LEVEL1_ICACHE_LINESIZE: C2RustUnnamed_2 = 187;
pub const _SC_LEVEL1_ICACHE_ASSOC: C2RustUnnamed_2 = 186;
pub const _SC_LEVEL1_ICACHE_SIZE: C2RustUnnamed_2 = 185;
pub const _SC_TRACE_LOG: C2RustUnnamed_2 = 184;
pub const _SC_TRACE_INHERIT: C2RustUnnamed_2 = 183;
pub const _SC_TRACE_EVENT_FILTER: C2RustUnnamed_2 = 182;
pub const _SC_TRACE: C2RustUnnamed_2 = 181;
pub const _SC_HOST_NAME_MAX: C2RustUnnamed_2 = 180;
pub const _SC_V6_LPBIG_OFFBIG: C2RustUnnamed_2 = 179;
pub const _SC_V6_LP64_OFF64: C2RustUnnamed_2 = 178;
pub const _SC_V6_ILP32_OFFBIG: C2RustUnnamed_2 = 177;
pub const _SC_V6_ILP32_OFF32: C2RustUnnamed_2 = 176;
pub const _SC_2_PBS_CHECKPOINT: C2RustUnnamed_2 = 175;
pub const _SC_STREAMS: C2RustUnnamed_2 = 174;
pub const _SC_SYMLOOP_MAX: C2RustUnnamed_2 = 173;
pub const _SC_2_PBS_TRACK: C2RustUnnamed_2 = 172;
pub const _SC_2_PBS_MESSAGE: C2RustUnnamed_2 = 171;
pub const _SC_2_PBS_LOCATE: C2RustUnnamed_2 = 170;
pub const _SC_2_PBS_ACCOUNTING: C2RustUnnamed_2 = 169;
pub const _SC_2_PBS: C2RustUnnamed_2 = 168;
pub const _SC_USER_GROUPS_R: C2RustUnnamed_2 = 167;
pub const _SC_USER_GROUPS: C2RustUnnamed_2 = 166;
pub const _SC_TYPED_MEMORY_OBJECTS: C2RustUnnamed_2 = 165;
pub const _SC_TIMEOUTS: C2RustUnnamed_2 = 164;
pub const _SC_SYSTEM_DATABASE_R: C2RustUnnamed_2 = 163;
pub const _SC_SYSTEM_DATABASE: C2RustUnnamed_2 = 162;
pub const _SC_THREAD_SPORADIC_SERVER: C2RustUnnamed_2 = 161;
pub const _SC_SPORADIC_SERVER: C2RustUnnamed_2 = 160;
pub const _SC_SPAWN: C2RustUnnamed_2 = 159;
pub const _SC_SIGNALS: C2RustUnnamed_2 = 158;
pub const _SC_SHELL: C2RustUnnamed_2 = 157;
pub const _SC_REGEX_VERSION: C2RustUnnamed_2 = 156;
pub const _SC_REGEXP: C2RustUnnamed_2 = 155;
pub const _SC_SPIN_LOCKS: C2RustUnnamed_2 = 154;
pub const _SC_READER_WRITER_LOCKS: C2RustUnnamed_2 = 153;
pub const _SC_NETWORKING: C2RustUnnamed_2 = 152;
pub const _SC_SINGLE_PROCESS: C2RustUnnamed_2 = 151;
pub const _SC_MULTI_PROCESS: C2RustUnnamed_2 = 150;
pub const _SC_MONOTONIC_CLOCK: C2RustUnnamed_2 = 149;
pub const _SC_FILE_SYSTEM: C2RustUnnamed_2 = 148;
pub const _SC_FILE_LOCKING: C2RustUnnamed_2 = 147;
pub const _SC_FILE_ATTRIBUTES: C2RustUnnamed_2 = 146;
pub const _SC_PIPE: C2RustUnnamed_2 = 145;
pub const _SC_FIFO: C2RustUnnamed_2 = 144;
pub const _SC_FD_MGMT: C2RustUnnamed_2 = 143;
pub const _SC_DEVICE_SPECIFIC_R: C2RustUnnamed_2 = 142;
pub const _SC_DEVICE_SPECIFIC: C2RustUnnamed_2 = 141;
pub const _SC_DEVICE_IO: C2RustUnnamed_2 = 140;
pub const _SC_THREAD_CPUTIME: C2RustUnnamed_2 = 139;
pub const _SC_CPUTIME: C2RustUnnamed_2 = 138;
pub const _SC_CLOCK_SELECTION: C2RustUnnamed_2 = 137;
pub const _SC_C_LANG_SUPPORT_R: C2RustUnnamed_2 = 136;
pub const _SC_C_LANG_SUPPORT: C2RustUnnamed_2 = 135;
pub const _SC_BASE: C2RustUnnamed_2 = 134;
pub const _SC_BARRIERS: C2RustUnnamed_2 = 133;
pub const _SC_ADVISORY_INFO: C2RustUnnamed_2 = 132;
pub const _SC_XOPEN_REALTIME_THREADS: C2RustUnnamed_2 = 131;
pub const _SC_XOPEN_REALTIME: C2RustUnnamed_2 = 130;
pub const _SC_XOPEN_LEGACY: C2RustUnnamed_2 = 129;
pub const _SC_XBS5_LPBIG_OFFBIG: C2RustUnnamed_2 = 128;
pub const _SC_XBS5_LP64_OFF64: C2RustUnnamed_2 = 127;
pub const _SC_XBS5_ILP32_OFFBIG: C2RustUnnamed_2 = 126;
pub const _SC_XBS5_ILP32_OFF32: C2RustUnnamed_2 = 125;
pub const _SC_NL_TEXTMAX: C2RustUnnamed_2 = 124;
pub const _SC_NL_SETMAX: C2RustUnnamed_2 = 123;
pub const _SC_NL_NMAX: C2RustUnnamed_2 = 122;
pub const _SC_NL_MSGMAX: C2RustUnnamed_2 = 121;
pub const _SC_NL_LANGMAX: C2RustUnnamed_2 = 120;
pub const _SC_NL_ARGMAX: C2RustUnnamed_2 = 119;
pub const _SC_USHRT_MAX: C2RustUnnamed_2 = 118;
pub const _SC_ULONG_MAX: C2RustUnnamed_2 = 117;
pub const _SC_UINT_MAX: C2RustUnnamed_2 = 116;
pub const _SC_UCHAR_MAX: C2RustUnnamed_2 = 115;
pub const _SC_SHRT_MIN: C2RustUnnamed_2 = 114;
pub const _SC_SHRT_MAX: C2RustUnnamed_2 = 113;
pub const _SC_SCHAR_MIN: C2RustUnnamed_2 = 112;
pub const _SC_SCHAR_MAX: C2RustUnnamed_2 = 111;
pub const _SC_SSIZE_MAX: C2RustUnnamed_2 = 110;
pub const _SC_NZERO: C2RustUnnamed_2 = 109;
pub const _SC_MB_LEN_MAX: C2RustUnnamed_2 = 108;
pub const _SC_WORD_BIT: C2RustUnnamed_2 = 107;
pub const _SC_LONG_BIT: C2RustUnnamed_2 = 106;
pub const _SC_INT_MIN: C2RustUnnamed_2 = 105;
pub const _SC_INT_MAX: C2RustUnnamed_2 = 104;
pub const _SC_CHAR_MIN: C2RustUnnamed_2 = 103;
pub const _SC_CHAR_MAX: C2RustUnnamed_2 = 102;
pub const _SC_CHAR_BIT: C2RustUnnamed_2 = 101;
pub const _SC_XOPEN_XPG4: C2RustUnnamed_2 = 100;
pub const _SC_XOPEN_XPG3: C2RustUnnamed_2 = 99;
pub const _SC_XOPEN_XPG2: C2RustUnnamed_2 = 98;
pub const _SC_2_UPE: C2RustUnnamed_2 = 97;
pub const _SC_2_C_VERSION: C2RustUnnamed_2 = 96;
pub const _SC_2_CHAR_TERM: C2RustUnnamed_2 = 95;
pub const _SC_XOPEN_SHM: C2RustUnnamed_2 = 94;
pub const _SC_XOPEN_ENH_I18N: C2RustUnnamed_2 = 93;
pub const _SC_XOPEN_CRYPT: C2RustUnnamed_2 = 92;
pub const _SC_XOPEN_UNIX: C2RustUnnamed_2 = 91;
pub const _SC_XOPEN_XCU_VERSION: C2RustUnnamed_2 = 90;
pub const _SC_XOPEN_VERSION: C2RustUnnamed_2 = 89;
pub const _SC_PASS_MAX: C2RustUnnamed_2 = 88;
pub const _SC_ATEXIT_MAX: C2RustUnnamed_2 = 87;
pub const _SC_AVPHYS_PAGES: C2RustUnnamed_2 = 86;
pub const _SC_PHYS_PAGES: C2RustUnnamed_2 = 85;
pub const _SC_NPROCESSORS_ONLN: C2RustUnnamed_2 = 84;
pub const _SC_NPROCESSORS_CONF: C2RustUnnamed_2 = 83;
pub const _SC_THREAD_PROCESS_SHARED: C2RustUnnamed_2 = 82;
pub const _SC_THREAD_PRIO_PROTECT: C2RustUnnamed_2 = 81;
pub const _SC_THREAD_PRIO_INHERIT: C2RustUnnamed_2 = 80;
pub const _SC_THREAD_PRIORITY_SCHEDULING: C2RustUnnamed_2 = 79;
pub const _SC_THREAD_ATTR_STACKSIZE: C2RustUnnamed_2 = 78;
pub const _SC_THREAD_ATTR_STACKADDR: C2RustUnnamed_2 = 77;
pub const _SC_THREAD_THREADS_MAX: C2RustUnnamed_2 = 76;
pub const _SC_THREAD_STACK_MIN: C2RustUnnamed_2 = 75;
pub const _SC_THREAD_KEYS_MAX: C2RustUnnamed_2 = 74;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: C2RustUnnamed_2 = 73;
pub const _SC_TTY_NAME_MAX: C2RustUnnamed_2 = 72;
pub const _SC_LOGIN_NAME_MAX: C2RustUnnamed_2 = 71;
pub const _SC_GETPW_R_SIZE_MAX: C2RustUnnamed_2 = 70;
pub const _SC_GETGR_R_SIZE_MAX: C2RustUnnamed_2 = 69;
pub const _SC_THREAD_SAFE_FUNCTIONS: C2RustUnnamed_2 = 68;
pub const _SC_THREADS: C2RustUnnamed_2 = 67;
pub const _SC_T_IOV_MAX: C2RustUnnamed_2 = 66;
pub const _SC_PII_OSI_M: C2RustUnnamed_2 = 65;
pub const _SC_PII_OSI_CLTS: C2RustUnnamed_2 = 64;
pub const _SC_PII_OSI_COTS: C2RustUnnamed_2 = 63;
pub const _SC_PII_INTERNET_DGRAM: C2RustUnnamed_2 = 62;
pub const _SC_PII_INTERNET_STREAM: C2RustUnnamed_2 = 61;
pub const _SC_IOV_MAX: C2RustUnnamed_2 = 60;
pub const _SC_UIO_MAXIOV: C2RustUnnamed_2 = 60;
pub const _SC_SELECT: C2RustUnnamed_2 = 59;
pub const _SC_POLL: C2RustUnnamed_2 = 58;
pub const _SC_PII_OSI: C2RustUnnamed_2 = 57;
pub const _SC_PII_INTERNET: C2RustUnnamed_2 = 56;
pub const _SC_PII_SOCKET: C2RustUnnamed_2 = 55;
pub const _SC_PII_XTI: C2RustUnnamed_2 = 54;
pub const _SC_PII: C2RustUnnamed_2 = 53;
pub const _SC_2_LOCALEDEF: C2RustUnnamed_2 = 52;
pub const _SC_2_SW_DEV: C2RustUnnamed_2 = 51;
pub const _SC_2_FORT_RUN: C2RustUnnamed_2 = 50;
pub const _SC_2_FORT_DEV: C2RustUnnamed_2 = 49;
pub const _SC_2_C_DEV: C2RustUnnamed_2 = 48;
pub const _SC_2_C_BIND: C2RustUnnamed_2 = 47;
pub const _SC_2_VERSION: C2RustUnnamed_2 = 46;
pub const _SC_CHARCLASS_NAME_MAX: C2RustUnnamed_2 = 45;
pub const _SC_RE_DUP_MAX: C2RustUnnamed_2 = 44;
pub const _SC_LINE_MAX: C2RustUnnamed_2 = 43;
pub const _SC_EXPR_NEST_MAX: C2RustUnnamed_2 = 42;
pub const _SC_EQUIV_CLASS_MAX: C2RustUnnamed_2 = 41;
pub const _SC_COLL_WEIGHTS_MAX: C2RustUnnamed_2 = 40;
pub const _SC_BC_STRING_MAX: C2RustUnnamed_2 = 39;
pub const _SC_BC_SCALE_MAX: C2RustUnnamed_2 = 38;
pub const _SC_BC_DIM_MAX: C2RustUnnamed_2 = 37;
pub const _SC_BC_BASE_MAX: C2RustUnnamed_2 = 36;
pub const _SC_TIMER_MAX: C2RustUnnamed_2 = 35;
pub const _SC_SIGQUEUE_MAX: C2RustUnnamed_2 = 34;
pub const _SC_SEM_VALUE_MAX: C2RustUnnamed_2 = 33;
pub const _SC_SEM_NSEMS_MAX: C2RustUnnamed_2 = 32;
pub const _SC_RTSIG_MAX: C2RustUnnamed_2 = 31;
pub const _SC_VERSION: C2RustUnnamed_2 = 29;
pub const _SC_MQ_PRIO_MAX: C2RustUnnamed_2 = 28;
pub const _SC_MQ_OPEN_MAX: C2RustUnnamed_2 = 27;
pub const _SC_DELAYTIMER_MAX: C2RustUnnamed_2 = 26;
pub const _SC_AIO_PRIO_DELTA_MAX: C2RustUnnamed_2 = 25;
pub const _SC_AIO_MAX: C2RustUnnamed_2 = 24;
pub const _SC_AIO_LISTIO_MAX: C2RustUnnamed_2 = 23;
pub const _SC_SHARED_MEMORY_OBJECTS: C2RustUnnamed_2 = 22;
pub const _SC_SEMAPHORES: C2RustUnnamed_2 = 21;
pub const _SC_MESSAGE_PASSING: C2RustUnnamed_2 = 20;
pub const _SC_MEMORY_PROTECTION: C2RustUnnamed_2 = 19;
pub const _SC_MEMLOCK_RANGE: C2RustUnnamed_2 = 18;
pub const _SC_MEMLOCK: C2RustUnnamed_2 = 17;
pub const _SC_MAPPED_FILES: C2RustUnnamed_2 = 16;
pub const _SC_FSYNC: C2RustUnnamed_2 = 15;
pub const _SC_SYNCHRONIZED_IO: C2RustUnnamed_2 = 14;
pub const _SC_PRIORITIZED_IO: C2RustUnnamed_2 = 13;
pub const _SC_ASYNCHRONOUS_IO: C2RustUnnamed_2 = 12;
pub const _SC_TIMERS: C2RustUnnamed_2 = 11;
pub const _SC_PRIORITY_SCHEDULING: C2RustUnnamed_2 = 10;
pub const _SC_REALTIME_SIGNALS: C2RustUnnamed_2 = 9;
pub const _SC_SAVED_IDS: C2RustUnnamed_2 = 8;
pub const _SC_JOB_CONTROL: C2RustUnnamed_2 = 7;
pub const _SC_TZNAME_MAX: C2RustUnnamed_2 = 6;
pub const _SC_STREAM_MAX: C2RustUnnamed_2 = 5;
pub const _SC_OPEN_MAX: C2RustUnnamed_2 = 4;
pub const _SC_NGROUPS_MAX: C2RustUnnamed_2 = 3;
pub const _SC_CLK_TCK: C2RustUnnamed_2 = 2;
pub const _SC_CHILD_MAX: C2RustUnnamed_2 = 1;
pub const _SC_ARG_MAX: C2RustUnnamed_2 = 0;
#[inline]
unsafe extern "C" fn buffer_is_blank(mut b: *const buffer) -> libc::c_int {
    return ((*b).used < 2 as libc::c_int as libc::c_uint) as libc::c_int;
}
#[inline]
unsafe extern "C" fn buffer_clen(mut b: *const buffer) -> uint32_t {
    return ((*b).used)
        .wrapping_sub(
            (0 as libc::c_int as libc::c_uint != (*b).used) as libc::c_int
                as libc::c_uint,
        );
}
#[inline]
unsafe extern "C" fn buffer_string_space(mut b: *const buffer) -> uint32_t {
    return if (*b).size != 0 {
        ((*b).size)
            .wrapping_sub(
                (*b).used
                    | (0 as libc::c_int as libc::c_uint == (*b).used) as libc::c_int
                        as libc::c_uint,
            )
    } else {
        0 as libc::c_int as libc::c_uint
    };
}
#[inline]
unsafe extern "C" fn buffer_copy_buffer(mut b: *mut buffer, mut src: *const buffer) {
    buffer_copy_string_len(b, (*src).ptr, buffer_clen(src) as size_t);
}
#[inline]
unsafe extern "C" fn buffer_append_buffer(mut b: *mut buffer, mut src: *const buffer) {
    buffer_append_string_len(b, (*src).ptr, buffer_clen(src) as size_t);
}
#[inline]
unsafe extern "C" fn buffer_truncate(mut b: *mut buffer, mut len: uint32_t) {
    *((*b).ptr).offset(len as isize) = '\0' as i32 as libc::c_char;
    (*b).used = len.wrapping_add(1 as libc::c_int as libc::c_uint);
}
#[inline]
unsafe extern "C" fn chunkqueue_length(mut cq: *const chunkqueue) -> off_t {
    return (*cq).bytes_in - (*cq).bytes_out;
}
#[inline]
unsafe extern "C" fn buffer_clear(mut b: *mut buffer) {
    (*b).used = 0 as libc::c_int as uint32_t;
}
#[cold]
unsafe extern "C" fn mmap_pagemask() -> off_t {
    let mut pagesize: libc::c_long = sysconf(_SC_PAGESIZE as libc::c_int);
    if -(1 as libc::c_int) as libc::c_long == pagesize {
        pagesize = 4096 as libc::c_int as libc::c_long;
    }
    if !(pagesize < (512 as libc::c_int * 1024 as libc::c_int) as libc::c_long) {
        ck_assert_failed(
            b"chunk.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int as libc::c_uint,
            b"pagesize < (512*1024)\0" as *const u8 as *const libc::c_char,
        );
    }
    return !(pagesize - 1 as libc::c_int as libc::c_long);
}
static mut chunk_pagemask: off_t = 0 as libc::c_int as off_t;
static mut chunk_mmap_flags: libc::c_int = 0x1 as libc::c_int;
static mut chunk_buf_sz: size_t = 8192 as libc::c_int as size_t;
static mut chunks: *mut chunk = 0 as *const chunk as *mut chunk;
static mut chunks_oversized: *mut chunk = 0 as *const chunk as *mut chunk;
static mut chunks_filechunk: *mut chunk = 0 as *const chunk as *mut chunk;
static mut chunk_buffers: *mut chunk = 0 as *const chunk as *mut chunk;
static mut chunks_oversized_n: libc::c_int = 0;
static mut chunkqueue_default_tempdirs: *const array = 0 as *const array;
static mut chunkqueue_default_tempfile_size: off_t = (1 as libc::c_int
    * 1024 as libc::c_int * 1024 as libc::c_int) as off_t;
static mut env_tmpdir: *const libc::c_char = 0 as *const libc::c_char;
#[no_mangle]
#[cold]
pub unsafe extern "C" fn chunkqueue_set_chunk_size(mut sz: size_t) {
    let mut x: size_t = 1024 as libc::c_int as size_t;
    while x < sz && x < ((1 as libc::c_uint) << 30 as libc::c_int) as libc::c_ulong {
        x <<= 1 as libc::c_int;
    }
    chunk_buf_sz = if sz > 0 as libc::c_int as libc::c_ulong {
        x
    } else {
        8192 as libc::c_int as libc::c_ulong
    };
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn chunkqueue_set_tempdirs_default_reset() {
    chunk_buf_sz = 8192 as libc::c_int as size_t;
    chunkqueue_default_tempdirs = 0 as *const array;
    chunkqueue_default_tempfile_size = (1 as libc::c_int * 1024 as libc::c_int
        * 1024 as libc::c_int) as off_t;
    if 0 as libc::c_int as libc::c_long == chunk_pagemask {
        chunk_pagemask = mmap_pagemask();
    }
    chunk_mmap_flags = 0x1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_init(mut cq: *mut chunkqueue) -> *mut chunkqueue {
    if cq.is_null() {
        cq = ck_calloc(
            1 as libc::c_int as size_t,
            ::core::mem::size_of::<chunkqueue>() as libc::c_ulong,
        ) as *mut chunkqueue;
    }
    (*cq).upload_temp_file_size = chunkqueue_default_tempfile_size;
    return cq;
}
unsafe extern "C" fn chunk_init() -> *mut chunk {
    let c: *mut chunk = ck_calloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<chunk>() as libc::c_ulong,
    ) as *mut chunk;
    (*c).file.fd = -(1 as libc::c_int);
    (*c).mem = buffer_init();
    return c;
}
#[inline(never)]
unsafe extern "C" fn chunk_init_sz(mut sz: size_t) -> *mut chunk {
    let c: *mut chunk = chunk_init();
    buffer_string_prepare_copy(
        (*c).mem,
        sz.wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    return c;
}
unsafe extern "C" fn chunk_file_view_init() -> *mut libc::c_void {
    let cfv: *mut chunk_file_view = ck_calloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<chunk_file_view>() as libc::c_ulong,
    ) as *mut chunk_file_view;
    (*cfv).mptr = -(1 as libc::c_int) as *mut libc::c_void as *mut libc::c_char;
    (*cfv).refcnt = 1 as libc::c_int;
    return cfv as *mut libc::c_void;
}
unsafe extern "C" fn chunk_file_view_release(
    mut cfv: *mut chunk_file_view,
) -> *mut chunk_file_view {
    (*cfv).refcnt -= 1;
    if 0 as libc::c_int == (*cfv).refcnt {
        if -(1 as libc::c_int) as *mut libc::c_void != (*cfv).mptr as *mut libc::c_void {
            munmap((*cfv).mptr as *mut libc::c_void, (*cfv).mlen as size_t);
        }
        free(cfv as *mut libc::c_void);
    }
    return 0 as *mut chunk_file_view;
}
#[cold]
#[inline(never)]
unsafe extern "C" fn chunk_file_view_failed(
    mut cfv: *mut chunk_file_view,
) -> *mut chunk_file_view {
    return chunk_file_view_release(cfv);
}
#[no_mangle]
pub unsafe extern "C" fn chunk_file_pread(
    mut fd: libc::c_int,
    mut buf: *mut libc::c_void,
    mut count: size_t,
    mut offset: off_t,
) -> ssize_t {
    let mut rd: ssize_t = 0;
    loop {
        rd = pread(fd, buf, count, offset);
        if !(-(1 as libc::c_int) as libc::c_long == rd
            && *__errno_location() == 4 as libc::c_int)
        {
            break;
        }
    }
    return rd;
}
unsafe extern "C" fn chunk_file_preadv2_flags(mut c: *mut chunk) -> ssize_t {
    if 0 as libc::c_int == (*c).file.flagmask as libc::c_int {
        (*c).file.flagmask = 0x8 as libc::c_int as uint8_t;
        let fn_0: *const libc::c_char = (*(*c).mem).ptr;
        if buffer_clen((*c).mem) > 5 as libc::c_int as libc::c_uint
            && *fn_0.offset(4 as libc::c_int as isize) as libc::c_int == '/' as i32
            && (*fn_0.offset(1 as libc::c_int as isize) as libc::c_int == 't' as i32
                && *fn_0.offset(2 as libc::c_int as isize) as libc::c_int == 'm' as i32
                && *fn_0.offset(3 as libc::c_int as isize) as libc::c_int == 'p' as i32
                || *fn_0.offset(1 as libc::c_int as isize) as libc::c_int == 'd' as i32
                    && *fn_0.offset(2 as libc::c_int as isize) as libc::c_int
                        == 'e' as i32
                    && *fn_0.offset(3 as libc::c_int as isize) as libc::c_int
                        == 'v' as i32)
        {
            (*c).file.flagmask = !(0x8 as libc::c_int) as uint8_t;
        }
    }
    return (0x8 as libc::c_int & (*c).file.flagmask as libc::c_int) as ssize_t;
}
#[no_mangle]
pub unsafe extern "C" fn chunk_file_pread_chunk(
    mut c: *mut chunk,
    mut buf: *mut libc::c_void,
    mut count: size_t,
) -> ssize_t {
    let mut iov: [iovec; 1] = [
        {
            let mut init = iovec {
                iov_base: buf,
                iov_len: count,
            };
            init
        },
    ];
    let flags: libc::c_int = (if (*c).file.busy == 0 {
        chunk_file_preadv2_flags(c)
    } else {
        0 as libc::c_int as libc::c_long
    }) as libc::c_int;
    (*c).file.busy = 0 as libc::c_int as uint8_t;
    let mut rd: ssize_t = preadv2(
        (*c).file.fd,
        iov.as_mut_ptr(),
        1 as libc::c_int,
        (*c).offset,
        flags,
    );
    if (rd > 0 as libc::c_int as libc::c_long) as libc::c_int as libc::c_long != 0 {
        return rd;
    }
    if (rd < 0 as libc::c_int as libc::c_long) as libc::c_int as libc::c_long != 0 {
        let mut errnum: libc::c_int = *__errno_location();
        if errnum == 95 as libc::c_int {
            (*c).file.flagmask = !(0x8 as libc::c_int) as uint8_t;
            return chunk_file_pread_chunk(c, buf, count);
        }
        (*c)
            .file
            .busy = (errnum == 11 as libc::c_int || errnum == 4 as libc::c_int)
            as libc::c_int as uint8_t;
        return rd;
    }
    return chunk_file_pread((*c).file.fd, buf, count, (*c).offset);
}
unsafe extern "C" fn chunk_reset_file_chunk(mut c: *mut chunk) {
    if (*c).file.is_temp != 0 {
        (*c).file.is_temp = 0 as libc::c_int as uint8_t;
        if buffer_is_blank((*c).mem) == 0 {
            unlink((*(*c).mem).ptr);
        }
    }
    if ((*c).file.refchg).is_some() {
        ((*c).file.refchg)
            .expect("non-null function pointer")((*c).file.ref_0, -(1 as libc::c_int));
        (*c).file.refchg = None;
        (*c).file.ref_0 = 0 as *mut libc::c_void;
    } else if (*c).file.fd != -(1 as libc::c_int) {
        close((*c).file.fd);
    }
    if !((*c).file.view).is_null() {
        (*c).file.view = chunk_file_view_release((*c).file.view);
    }
    (*c).file.fd = -(1 as libc::c_int);
    (*c).file.length = 0 as libc::c_int as off_t;
    (*c).file.busy = 0 as libc::c_int as uint8_t;
    (*c).file.flagmask = 0 as libc::c_int as uint8_t;
    (*c).type_0 = MEM_CHUNK;
}
unsafe extern "C" fn chunk_reset(mut c: *mut chunk) {
    if (*c).type_0 as libc::c_uint == FILE_CHUNK as libc::c_int as libc::c_uint {
        chunk_reset_file_chunk(c);
    }
    buffer_clear((*c).mem);
    (*c).offset = 0 as libc::c_int as off_t;
}
unsafe extern "C" fn chunk_free(mut c: *mut chunk) {
    if (*c).type_0 as libc::c_uint == FILE_CHUNK as libc::c_int as libc::c_uint {
        chunk_reset_file_chunk(c);
    }
    buffer_free((*c).mem);
    free(c as *mut libc::c_void);
}
unsafe extern "C" fn chunk_pop_oversized(mut sz: size_t) -> *mut chunk {
    if !chunks_oversized.is_null()
        && (*(*chunks_oversized).mem).size as libc::c_ulong >= sz
    {
        chunks_oversized_n -= 1;
        chunks_oversized_n;
        let mut c: *mut chunk = chunks_oversized;
        chunks_oversized = (*c).next;
        return c;
    }
    return 0 as *mut chunk;
}
unsafe extern "C" fn chunk_push_oversized(c: *mut chunk, sz: size_t) {
    if chunks_oversized_n < 64 as libc::c_int
        && chunk_buf_sz >= 4096 as libc::c_int as libc::c_ulong
    {
        chunks_oversized_n += 1;
        chunks_oversized_n;
        let mut co: *mut *mut chunk = &mut chunks_oversized;
        while !(*co).is_null() && sz < (*(**co).mem).size as libc::c_ulong {
            co = &mut (**co).next;
        }
        (*c).next = *co;
        *co = c;
    } else {
        let tb: *mut buffer = if !chunks_oversized.is_null() {
            (*chunks_oversized).mem
        } else {
            0 as *mut buffer
        };
        if !tb.is_null() && ((*tb).size as libc::c_ulong) < sz {
            (*chunks_oversized).mem = (*c).mem;
            (*c).mem = tb;
        }
        chunk_free(c);
    };
}
#[inline(never)]
unsafe extern "C" fn chunk_buffer_acquire_sz(sz: size_t) -> *mut buffer {
    let mut c: *mut chunk = 0 as *mut chunk;
    let mut b: *mut buffer = 0 as *mut buffer;
    if sz <= chunk_buf_sz | 1 as libc::c_int as libc::c_ulong {
        if !chunks.is_null() {
            c = chunks;
            chunks = (*c).next;
        } else {
            c = chunk_init_sz(chunk_buf_sz);
        }
    } else {
        c = chunk_pop_oversized(sz);
        if c.is_null() {
            c = chunk_init_sz(
                (sz & !(1 as libc::c_ulong))
                    .wrapping_add(
                        chunk_buf_sz.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ) & !chunk_buf_sz.wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
        }
    }
    (*c).next = chunk_buffers;
    chunk_buffers = c;
    b = (*c).mem;
    (*c).mem = 0 as *mut buffer;
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn chunk_buffer_acquire() -> *mut buffer {
    return chunk_buffer_acquire_sz(chunk_buf_sz);
}
#[no_mangle]
pub unsafe extern "C" fn chunk_buffer_release(mut b: *mut buffer) {
    if b.is_null() {
        return;
    }
    if !chunk_buffers.is_null() {
        let mut c: *mut chunk = chunk_buffers;
        chunk_buffers = (*c).next;
        (*c).mem = b;
        buffer_clear(b);
        if (*b).size as libc::c_ulong == chunk_buf_sz | 1 as libc::c_int as libc::c_ulong
        {
            (*c).next = chunks;
            chunks = c;
        } else if (*b).size as libc::c_ulong > chunk_buf_sz {
            chunk_push_oversized(c, (*b).size as size_t);
        } else {
            chunk_free(c);
        }
    } else {
        buffer_free(b);
    };
}
#[no_mangle]
pub unsafe extern "C" fn chunk_buffer_yield(mut b: *mut buffer) {
    if (*b).size as libc::c_ulong == chunk_buf_sz | 1 as libc::c_int as libc::c_ulong {
        return;
    }
    let cb: *mut buffer = chunk_buffer_acquire_sz(chunk_buf_sz);
    let mut tb: buffer = *b;
    *b = *cb;
    *cb = tb;
    chunk_buffer_release(cb);
}
#[no_mangle]
pub unsafe extern "C" fn chunk_buffer_prepare_append(
    b: *mut buffer,
    mut sz: size_t,
) -> size_t {
    if sz > buffer_string_space(b) as libc::c_ulong {
        sz = (sz as libc::c_ulong)
            .wrapping_add(
                (if (*b).used != 0 {
                    (*b).used
                } else {
                    1 as libc::c_int as libc::c_uint
                }) as libc::c_ulong,
            ) as size_t as size_t;
        let cb: *mut buffer = chunk_buffer_acquire_sz(sz);
        let mut tb: buffer = *b;
        *b = *cb;
        *cb = tb;
        (*b).used = tb.used;
        if (*b).used != 0 {
            memcpy(
                (*b).ptr as *mut libc::c_void,
                tb.ptr as *const libc::c_void,
                tb.used as libc::c_ulong,
            );
        }
        chunk_buffer_release(cb);
    }
    return buffer_string_space(b) as size_t;
}
#[inline(never)]
unsafe extern "C" fn chunk_acquire(mut sz: size_t) -> *mut chunk {
    if sz <= chunk_buf_sz | 1 as libc::c_int as libc::c_ulong {
        if !chunks.is_null() {
            let mut c: *mut chunk = chunks;
            chunks = (*c).next;
            return c;
        }
        sz = chunk_buf_sz;
    } else {
        sz = sz
            .wrapping_add(chunk_buf_sz.wrapping_sub(1 as libc::c_int as libc::c_ulong))
            & !chunk_buf_sz.wrapping_sub(1 as libc::c_int as libc::c_ulong);
        let mut c_0: *mut chunk = chunk_pop_oversized(sz);
        if !c_0.is_null() {
            return c_0;
        }
    }
    return chunk_init_sz(sz);
}
unsafe extern "C" fn chunk_release(mut c: *mut chunk) {
    let sz: size_t = (*(*c).mem).size as size_t;
    if sz == chunk_buf_sz | 1 as libc::c_int as libc::c_ulong {
        chunk_reset(c);
        (*c).next = chunks;
        chunks = c;
    } else if sz > chunk_buf_sz {
        chunk_reset(c);
        chunk_push_oversized(c, sz);
    } else if (*c).type_0 as libc::c_uint == FILE_CHUNK as libc::c_int as libc::c_uint {
        chunk_reset(c);
        (*c).next = chunks_filechunk;
        chunks_filechunk = c;
    } else {
        chunk_free(c);
    };
}
unsafe extern "C" fn chunk_acquire_filechunk() -> *mut chunk {
    if !chunks_filechunk.is_null() {
        let mut c: *mut chunk = chunks_filechunk;
        chunks_filechunk = (*c).next;
        return c;
    }
    return chunk_init();
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_chunk_pool_clear() {
    let mut next: *mut chunk = 0 as *mut chunk;
    let mut c: *mut chunk = chunks;
    while !c.is_null() {
        next = (*c).next;
        chunk_free(c);
        c = next;
    }
    chunks = 0 as *mut chunk;
    let mut next_0: *mut chunk = 0 as *mut chunk;
    let mut c_0: *mut chunk = chunks_oversized;
    while !c_0.is_null() {
        next_0 = (*c_0).next;
        chunk_free(c_0);
        c_0 = next_0;
    }
    chunks_oversized = 0 as *mut chunk;
    chunks_oversized_n = 0 as libc::c_int;
    let mut next_1: *mut chunk = 0 as *mut chunk;
    let mut c_1: *mut chunk = chunks_filechunk;
    while !c_1.is_null() {
        next_1 = (*c_1).next;
        chunk_free(c_1);
        c_1 = next_1;
    }
    chunks_filechunk = 0 as *mut chunk;
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_chunk_pool_free() {
    chunkqueue_chunk_pool_clear();
    let mut next: *mut chunk = 0 as *mut chunk;
    let mut c: *mut chunk = chunk_buffers;
    while !c.is_null() {
        next = (*c).next;
        free(c as *mut libc::c_void);
        c = next;
    }
    chunk_buffers = 0 as *mut chunk;
}
unsafe extern "C" fn chunk_remaining_length(mut c: *const chunk) -> off_t {
    return (if (*c).type_0 as libc::c_uint == MEM_CHUNK as libc::c_int as libc::c_uint {
        buffer_clen((*c).mem) as off_t
    } else {
        (*c).file.length
    }) - (*c).offset;
}
unsafe extern "C" fn chunkqueue_release_chunks(mut cq: *mut chunkqueue) {
    (*cq).last = 0 as *mut chunk;
    let mut c: *mut chunk = 0 as *mut chunk;
    loop {
        c = (*cq).first;
        if c.is_null() {
            break;
        }
        (*cq).first = (*c).next;
        chunk_release(c);
    };
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn chunkqueue_free(mut cq: *mut chunkqueue) {
    if cq.is_null() {
        return;
    }
    chunkqueue_release_chunks(cq);
    free(cq as *mut libc::c_void);
}
unsafe extern "C" fn chunkqueue_prepend_chunk(cq: *mut chunkqueue, c: *mut chunk) {
    (*c).next = (*cq).first;
    if ((*c).next).is_null() {
        (*cq).last = c;
    }
    (*cq).first = c;
}
unsafe extern "C" fn chunkqueue_append_chunk(cq: *mut chunkqueue, c: *mut chunk) {
    (*c).next = 0 as *mut chunk;
    let ref mut fresh0 = *if !((*cq).last).is_null() {
        &mut (*(*cq).last).next
    } else {
        &mut (*cq).first
    };
    *fresh0 = c;
    (*cq).last = c;
}
unsafe extern "C" fn chunkqueue_prepend_mem_chunk(
    mut cq: *mut chunkqueue,
    mut sz: size_t,
) -> *mut chunk {
    let mut c: *mut chunk = chunk_acquire(sz);
    chunkqueue_prepend_chunk(cq, c);
    return c;
}
unsafe extern "C" fn chunkqueue_append_mem_chunk(
    mut cq: *mut chunkqueue,
    mut sz: size_t,
) -> *mut chunk {
    let mut c: *mut chunk = chunk_acquire(sz);
    chunkqueue_append_chunk(cq, c);
    return c;
}
unsafe extern "C" fn chunkqueue_append_file_chunk(
    cq: *mut chunkqueue,
    fn_0: *const buffer,
    mut offset: off_t,
    mut len: off_t,
) -> *mut chunk {
    let c: *mut chunk = chunk_acquire_filechunk();
    chunkqueue_append_chunk(cq, c);
    (*c).type_0 = FILE_CHUNK;
    (*c).offset = offset;
    (*c).file.length = offset + len;
    (*cq).bytes_in += len;
    buffer_copy_buffer((*c).mem, fn_0);
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_reset(mut cq: *mut chunkqueue) {
    chunkqueue_release_chunks(cq);
    (*cq).bytes_in = 0 as libc::c_int as off_t;
    (*cq).bytes_out = 0 as libc::c_int as off_t;
    (*cq).tempdir_idx = 0 as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_append_file_fd(
    cq: *mut chunkqueue,
    fn_0: *const buffer,
    mut fd: libc::c_int,
    mut offset: off_t,
    mut len: off_t,
) {
    if len > 0 as libc::c_int as libc::c_long {
        (*chunkqueue_append_file_chunk(cq, fn_0, offset, len)).file.fd = fd;
    } else {
        close(fd);
    };
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_append_file(
    cq: *mut chunkqueue,
    fn_0: *const buffer,
    mut offset: off_t,
    mut len: off_t,
) {
    if len > 0 as libc::c_int as libc::c_long {
        chunkqueue_append_file_chunk(cq, fn_0, offset, len);
    }
}
unsafe extern "C" fn chunkqueue_append_mem_extend_chunk(
    cq: *mut chunkqueue,
    mem: *const libc::c_char,
    mut len: size_t,
) -> libc::c_int {
    let mut c: *mut chunk = (*cq).last;
    if 0 as libc::c_int as libc::c_ulong == len {
        return 1 as libc::c_int;
    }
    if !c.is_null()
        && (*c).type_0 as libc::c_uint == MEM_CHUNK as libc::c_int as libc::c_uint
        && buffer_string_space((*c).mem) as libc::c_ulong >= len
    {
        buffer_append_string_len((*c).mem, mem, len);
        (*cq)
            .bytes_in = ((*cq).bytes_in as libc::c_ulong).wrapping_add(len) as off_t
            as off_t;
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_append_buffer(
    cq: *mut chunkqueue,
    mem: *mut buffer,
) {
    let mut c: *mut chunk = 0 as *mut chunk;
    let len: size_t = buffer_clen(mem) as size_t;
    if len < 1024 as libc::c_int as libc::c_ulong
        && chunkqueue_append_mem_extend_chunk(cq, (*mem).ptr, len) != 0
    {
        buffer_clear(mem);
        return;
    }
    c = chunkqueue_append_mem_chunk(cq, chunk_buf_sz);
    (*cq)
        .bytes_in = ((*cq).bytes_in as libc::c_ulong).wrapping_add(len) as off_t
        as off_t;
    buffer_move((*c).mem, mem);
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_append_mem(
    cq: *mut chunkqueue,
    mem: *const libc::c_char,
    mut len: size_t,
) {
    let mut c: *mut chunk = 0 as *mut chunk;
    if len < chunk_buf_sz && chunkqueue_append_mem_extend_chunk(cq, mem, len) != 0 {
        return;
    }
    c = chunkqueue_append_mem_chunk(
        cq,
        len.wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    (*cq)
        .bytes_in = ((*cq).bytes_in as libc::c_ulong).wrapping_add(len) as off_t
        as off_t;
    buffer_copy_string_len((*c).mem, mem, len);
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_append_mem_min(
    cq: *mut chunkqueue,
    mem: *const libc::c_char,
    mut len: size_t,
) {
    let mut c: *mut chunk = 0 as *mut chunk;
    if len < chunk_buf_sz && chunkqueue_append_mem_extend_chunk(cq, mem, len) != 0 {
        return;
    }
    c = chunk_init_sz(len.wrapping_add(1 as libc::c_int as libc::c_ulong));
    chunkqueue_append_chunk(cq, c);
    (*cq)
        .bytes_in = ((*cq).bytes_in as libc::c_ulong).wrapping_add(len) as off_t
        as off_t;
    buffer_copy_string_len((*c).mem, mem, len);
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_append_chunkqueue(
    cq: *mut chunkqueue,
    src: *mut chunkqueue,
) {
    if ((*src).first).is_null() {
        return;
    }
    if ((*cq).first).is_null() {
        (*cq).first = (*src).first;
    } else {
        (*(*cq).last).next = (*src).first;
    }
    (*cq).last = (*src).last;
    (*cq).bytes_in += chunkqueue_length(src);
    (*src).first = 0 as *mut chunk;
    (*src).last = 0 as *mut chunk;
    (*src).bytes_out = (*src).bytes_in;
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_prepend_buffer_open_sz(
    mut cq: *mut chunkqueue,
    mut sz: size_t,
) -> *mut buffer {
    let c: *mut chunk = chunkqueue_prepend_mem_chunk(cq, sz);
    return (*c).mem;
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_prepend_buffer_open(
    mut cq: *mut chunkqueue,
) -> *mut buffer {
    return chunkqueue_prepend_buffer_open_sz(cq, chunk_buf_sz);
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_prepend_buffer_commit(mut cq: *mut chunkqueue) {
    (*cq).bytes_in += buffer_clen((*(*cq).first).mem) as libc::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_append_buffer_open_sz(
    mut cq: *mut chunkqueue,
    mut sz: size_t,
) -> *mut buffer {
    let c: *mut chunk = chunkqueue_append_mem_chunk(cq, sz);
    return (*c).mem;
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_append_buffer_open(
    mut cq: *mut chunkqueue,
) -> *mut buffer {
    return chunkqueue_append_buffer_open_sz(cq, chunk_buf_sz);
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_append_buffer_commit(mut cq: *mut chunkqueue) {
    (*cq).bytes_in += buffer_clen((*(*cq).last).mem) as libc::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_get_memory(
    cq: *mut chunkqueue,
    len: *mut size_t,
) -> *mut libc::c_char {
    let mut sz: size_t = if *len != 0 { *len } else { chunk_buf_sz >> 1 as libc::c_int };
    let mut b: *mut buffer = 0 as *mut buffer;
    let mut c: *mut chunk = (*cq).last;
    if !c.is_null()
        && MEM_CHUNK as libc::c_int as libc::c_uint == (*c).type_0 as libc::c_uint
    {
        let mut avail: size_t = buffer_string_space((*c).mem) as size_t;
        if avail >= sz {
            *len = avail;
            b = (*c).mem;
            return ((*b).ptr).offset(buffer_clen(b) as isize);
        }
    }
    b = chunkqueue_append_buffer_open_sz(cq, sz);
    *len = buffer_string_space(b) as size_t;
    return (*b).ptr;
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_use_memory(
    cq: *mut chunkqueue,
    mut ckpt: *mut chunk,
    mut len: size_t,
) {
    let mut b: *mut buffer = (*(*cq).last).mem;
    if (len > 0 as libc::c_int as libc::c_ulong) as libc::c_int as libc::c_long != 0 {
        buffer_commit(b, len);
        (*cq)
            .bytes_in = ((*cq).bytes_in as libc::c_ulong).wrapping_add(len) as off_t
            as off_t;
        if (*cq).last == ckpt || ckpt.is_null()
            || MEM_CHUNK as libc::c_int as libc::c_uint != (*ckpt).type_0 as libc::c_uint
            || len > buffer_string_space((*ckpt).mem) as libc::c_ulong
        {
            return;
        }
        buffer_append_buffer((*ckpt).mem, b);
    } else if buffer_is_blank(b) == 0 {
        return
    }
    chunk_release((*cq).last);
    (*cq).last = ckpt;
    let ref mut fresh1 = *if !ckpt.is_null() {
        &mut (*ckpt).next
    } else {
        &mut (*cq).first
    };
    *fresh1 = 0 as *mut chunk;
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_update_file(
    cq: *mut chunkqueue,
    mut c: *mut chunk,
    mut len: off_t,
) {
    (*c).file.length += len;
    (*cq).bytes_in += len;
    if 0 as libc::c_int as libc::c_long == chunk_remaining_length(c) {
        chunkqueue_remove_empty_chunks(cq);
    }
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn chunkqueue_set_tempdirs_default(
    mut tempdirs: *const array,
    mut upload_temp_file_size: off_t,
) {
    if upload_temp_file_size == 0 as libc::c_int as libc::c_long {
        upload_temp_file_size = (1 as libc::c_int * 1024 as libc::c_int
            * 1024 as libc::c_int) as off_t;
    }
    chunkqueue_default_tempdirs = tempdirs;
    chunkqueue_default_tempfile_size = upload_temp_file_size;
    env_tmpdir = getenv(b"TMPDIR\0" as *const u8 as *const libc::c_char);
    if env_tmpdir.is_null() {
        env_tmpdir = b"/var/tmp\0" as *const u8 as *const libc::c_char;
    }
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_set_tempdirs(
    cq: *mut chunkqueue,
    mut upload_temp_file_size: off_t,
) {
    if upload_temp_file_size == 0 as libc::c_int as libc::c_long {
        upload_temp_file_size = chunkqueue_default_tempfile_size;
    }
    (*cq).upload_temp_file_size = upload_temp_file_size;
    (*cq).tempdir_idx = 0 as libc::c_int as libc::c_uint;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn chunkqueue_env_tmpdir() -> *const libc::c_char {
    return env_tmpdir;
}
#[inline(never)]
unsafe extern "C" fn chunkqueue_dup_file_chunk_fd(d: *mut chunk, c: *const chunk) {
    if (*c).file.fd >= 0 as libc::c_int {
        if ((*c).file.refchg).is_some() {
            (*d).file.fd = (*c).file.fd;
            (*d).file.ref_0 = (*c).file.ref_0;
            (*d).file.refchg = (*c).file.refchg;
            ((*d).file.refchg)
                .expect("non-null function pointer")((*d).file.ref_0, 1 as libc::c_int);
        } else {
            (*d).file.fd = fdevent_dup_cloexec((*c).file.fd);
        }
        (*d).file.view = (*c).file.view;
        if !((*d).file.view).is_null() {
            (*(*d).file.view).refcnt += 1;
            (*(*d).file.view).refcnt;
        }
    }
}
#[inline(never)]
unsafe extern "C" fn chunkqueue_steal_partial_file_chunk(
    dest: *mut chunkqueue,
    c: *const chunk,
    len: off_t,
) {
    chunkqueue_append_file(dest, (*c).mem, (*c).offset, len);
    chunkqueue_dup_file_chunk_fd((*dest).last, c);
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_steal(
    dest: *mut chunkqueue,
    src: *mut chunkqueue,
    mut len: off_t,
) {
    let mut clen: off_t = 0;
    loop {
        let c: *mut chunk = (*src).first;
        if (0 as *mut libc::c_void as *mut chunk == c) as libc::c_int as libc::c_long
            != 0
        {
            break;
        }
        clen = chunk_remaining_length(c);
        if len >= clen {
            (*src).first = (*c).next;
            if c == (*src).last {
                (*src).last = 0 as *mut chunk;
            }
            if (0 as libc::c_int as libc::c_long != clen) as libc::c_int as libc::c_long
                != 0
            {
                chunkqueue_append_chunk(dest, c);
                (*dest).bytes_in += clen;
            } else {
                chunk_release(c);
            }
        } else {
            match (*c).type_0 as libc::c_uint {
                0 => {
                    chunkqueue_append_mem(
                        dest,
                        ((*(*c).mem).ptr).offset((*c).offset as isize),
                        len as size_t,
                    );
                }
                1 => {
                    chunkqueue_steal_partial_file_chunk(dest, c, len);
                }
                _ => {}
            }
            (*c).offset += len;
            clen = len;
        }
        (*src).bytes_out += clen;
        len -= clen;
        if !(len != 0) {
            break;
        }
    };
}
unsafe extern "C" fn chunkqueue_get_append_mkstemp(
    b: *mut buffer,
    mut path: *const libc::c_char,
    len: uint32_t,
) -> libc::c_int {
    buffer_copy_path_len2(
        b,
        path,
        len as size_t,
        b"lighttpd-upload-XXXXXX\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    return fdevent_mkostemp((*b).ptr, 0 as libc::c_int);
}
unsafe extern "C" fn chunkqueue_get_append_newtempfile(
    cq: *mut chunkqueue,
    errh: *mut log_error_st,
) -> *mut chunk {
    static mut emptyb: buffer = {
        let mut init = buffer {
            ptr: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            used: 0 as libc::c_int as uint32_t,
            size: 0 as libc::c_int as uint32_t,
        };
        init
    };
    let last: *mut chunk = (*cq).last;
    let c: *mut chunk = chunkqueue_append_file_chunk(
        cq,
        &emptyb,
        0 as libc::c_int as off_t,
        0 as libc::c_int as off_t,
    );
    let tempdirs: *const array = chunkqueue_default_tempdirs;
    let template: *mut buffer = (*c).mem;
    (*c).file.is_temp = 1 as libc::c_int as uint8_t;
    (*c).file.flagmask = !(0x8 as libc::c_int) as uint8_t;
    if !tempdirs.is_null() && (*tempdirs).used != 0 {
        *__errno_location() = 5 as libc::c_int;
        while (*cq).tempdir_idx < (*tempdirs).used {
            let mut ds: *mut data_string = *((*tempdirs).data)
                .offset((*cq).tempdir_idx as isize) as *mut data_string;
            (*c)
                .file
                .fd = chunkqueue_get_append_mkstemp(
                template,
                (*ds).value.ptr,
                buffer_clen(&mut (*ds).value),
            );
            if -(1 as libc::c_int) != (*c).file.fd {
                return c;
            }
            (*cq).tempdir_idx = ((*cq).tempdir_idx).wrapping_add(1);
            (*cq).tempdir_idx;
        }
    } else {
        let mut tmpdir: *const libc::c_char = chunkqueue_env_tmpdir();
        (*c)
            .file
            .fd = chunkqueue_get_append_mkstemp(
            template,
            tmpdir,
            strlen(tmpdir) as uint32_t,
        );
        if -(1 as libc::c_int) != (*c).file.fd {
            return c;
        }
    }
    log_perror(
        errh,
        b"chunk.c\0" as *const u8 as *const libc::c_char,
        885 as libc::c_int as libc::c_uint,
        b"opening temp-file failed: %s\0" as *const u8 as *const libc::c_char,
        (*template).ptr,
    );
    (*c).file.is_temp = 0 as libc::c_int as uint8_t;
    (*cq).last = last;
    if !((*cq).last).is_null() {
        (*last).next = 0 as *mut chunk;
    } else {
        (*cq).first = 0 as *mut chunk;
    }
    chunk_release(c);
    return 0 as *mut chunk;
}
#[cold]
unsafe extern "C" fn chunkqueue_close_tempchunk(
    c: *mut chunk,
    errh: *mut log_error_st,
) -> libc::c_int {
    if ((*c).file.refchg).is_some() {
        ck_assert_failed(
            b"chunk.c\0" as *const u8 as *const libc::c_char,
            899 as libc::c_int as libc::c_uint,
            b"0 == c->file.refchg\0" as *const u8 as *const libc::c_char,
        );
    }
    let mut rc: libc::c_int = close((*c).file.fd);
    (*c).file.fd = -(1 as libc::c_int);
    if 0 as libc::c_int != rc {
        log_perror(
            errh,
            b"chunk.c\0" as *const u8 as *const libc::c_char,
            903 as libc::c_int as libc::c_uint,
            b"close() temp-file %s failed\0" as *const u8 as *const libc::c_char,
            (*(*c).mem).ptr,
        );
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn chunkqueue_get_append_tempfile(
    cq: *mut chunkqueue,
    errh: *mut log_error_st,
) -> *mut chunk {
    let c: *mut chunk = (*cq).last;
    if !c.is_null() && (*c).file.is_temp as libc::c_int != 0
        && (*c).file.fd >= 0 as libc::c_int
    {
        let mut upload_temp_file_size: off_t = if (*cq).upload_temp_file_size != 0 {
            (*cq).upload_temp_file_size
        } else {
            chunkqueue_default_tempfile_size
        };
        if (*c).file.length < upload_temp_file_size {
            return c;
        }
        if chunkqueue_close_tempchunk(c, errh) == 0 {
            return 0 as *mut chunk;
        }
    }
    return chunkqueue_get_append_newtempfile(cq, errh);
}
#[cold]
unsafe extern "C" fn chunkqueue_append_tempfile_err(
    cq: *mut chunkqueue,
    errh: *mut log_error_st,
    c: *mut chunk,
) -> libc::c_int {
    let errnum: libc::c_int = *__errno_location();
    if errnum == 4 as libc::c_int {
        return 1 as libc::c_int;
    }
    let tempdirs: *const array = chunkqueue_default_tempdirs;
    let mut retry: libc::c_int = (errnum == 28 as libc::c_int && !tempdirs.is_null()
        && {
            (*cq).tempdir_idx = ((*cq).tempdir_idx).wrapping_add(1);
            (*cq).tempdir_idx < (*tempdirs).used
        }) as libc::c_int;
    if retry == 0 {
        log_perror(
            errh,
            b"chunk.c\0" as *const u8 as *const libc::c_char,
            944 as libc::c_int as libc::c_uint,
            b"write() temp-file %s failed\0" as *const u8 as *const libc::c_char,
            (*(*c).mem).ptr,
        );
    }
    if 0 as libc::c_int as libc::c_long == chunk_remaining_length(c) {
        chunkqueue_remove_empty_chunks(cq);
    } else if chunkqueue_close_tempchunk(c, errh) == 0 {
        retry = 0 as libc::c_int;
    }
    return retry;
}
#[cold]
#[inline(never)]
unsafe extern "C" fn chunkqueue_to_tempfiles(
    dest: *mut chunkqueue,
    errh: *mut log_error_st,
) -> libc::c_int {
    let cqlen: off_t = chunkqueue_length(dest);
    let mut src: chunkqueue = *dest;
    (*dest).last = 0 as *mut chunk;
    (*dest).first = (*dest).last;
    (*dest).bytes_in -= cqlen;
    if 0 as libc::c_int == chunkqueue_steal_with_tempfiles(dest, &mut src, cqlen, errh) {
        return 0 as libc::c_int
    } else {
        let errnum: libc::c_int = *__errno_location();
        chunkqueue_release_chunks(&mut src);
        return -errnum;
    };
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_append_mem_to_tempfile(
    dest: *mut chunkqueue,
    mut mem: *const libc::c_char,
    mut len: size_t,
    errh: *mut log_error_st,
) -> libc::c_int {
    let mut dst_c: *mut chunk = (*dest).first;
    if !dst_c.is_null()
        && (*dst_c).type_0 as libc::c_uint == MEM_CHUNK as libc::c_int as libc::c_uint
        && 0 as libc::c_int != chunkqueue_to_tempfiles(dest, errh)
    {
        return -(1 as libc::c_int);
    }
    loop {
        dst_c = chunkqueue_get_append_tempfile(dest, errh);
        if dst_c.is_null() {
            return -(1 as libc::c_int);
        }
        if 0 as libc::c_int as libc::c_ulong == len {
            return 0 as libc::c_int;
        }
        let written: ssize_t = pwrite(
            (*dst_c).file.fd,
            mem as *const libc::c_void,
            len,
            (*dst_c).file.length,
        );
        if written as size_t == len {
            (*dst_c)
                .file
                .length = ((*dst_c).file.length as libc::c_ulong).wrapping_add(len)
                as off_t as off_t;
            (*dest)
                .bytes_in = ((*dest).bytes_in as libc::c_ulong).wrapping_add(len)
                as off_t as off_t;
            return 0 as libc::c_int;
        } else {
            if written >= 0 as libc::c_int as libc::c_long {
                (*dest).bytes_in += written;
                mem = mem.offset(written as isize);
                len = (len as libc::c_ulong).wrapping_sub(written as size_t) as size_t
                    as size_t;
                (*dst_c)
                    .file
                    .length = ((*dst_c).file.length as libc::c_ulong)
                    .wrapping_add(written as size_t) as off_t as off_t;
            } else if chunkqueue_append_tempfile_err(dest, errh, dst_c) == 0 {
                break;
            }
            if !(len != 0) {
                break;
            }
        }
    }
    return -(1 as libc::c_int);
}
#[cold]
#[inline(never)]
unsafe extern "C" fn chunkqueue_append_cqmem_to_tempfile_partial(
    dest: *mut chunkqueue,
    c: *mut chunk,
    mut wr: ssize_t,
    errh: *mut log_error_st,
) -> ssize_t {
    let mut ckpt: *mut chunk = (*dest).first;
    while (*ckpt).next != c {
        ckpt = (*ckpt).next;
    }
    (*ckpt).next = 0 as *mut chunk;
    (*dest).last = ckpt;
    (*dest).bytes_in -= wr;
    (*dest).bytes_out -= wr;
    chunkqueue_mark_written(dest, wr);
    (*c).next = (*dest).first;
    (*dest).first = c;
    return (if 0 as libc::c_int == chunkqueue_to_tempfiles(dest, errh) {
        0 as libc::c_int
    } else {
        -(1 as libc::c_int)
    }) as ssize_t;
}
unsafe extern "C" fn chunkqueue_append_cqmem_to_tempfile(
    dest: *mut chunkqueue,
    src: *mut chunkqueue,
    mut len: off_t,
    errh: *mut log_error_st,
) -> ssize_t {
    let mut iovcnt: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut iov: [iovec; 16] = [iovec {
        iov_base: 0 as *mut libc::c_void,
        iov_len: 0,
    }; 16];
    let mut dlen: off_t = 0 as libc::c_int as off_t;
    let mut c: *mut chunk = 0 as *mut chunk;
    c = (*dest).first;
    while !c.is_null()
        && (*c).type_0 as libc::c_uint == MEM_CHUNK as libc::c_int as libc::c_uint
    {
        let clen: off_t = chunk_remaining_length(c);
        iov[iovcnt as usize]
            .iov_base = ((*(*c).mem).ptr).offset((*c).offset as isize)
            as *mut libc::c_void;
        iov[iovcnt as usize].iov_len = clen as size_t;
        dlen += clen;
        iovcnt = iovcnt.wrapping_add(1);
        iovcnt;
        if (iovcnt as libc::c_ulong
            == (::core::mem::size_of::<[iovec; 16]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<iovec>() as libc::c_ulong))
            as libc::c_int as libc::c_long != 0
        {
            break;
        }
        c = (*c).next;
    }
    if (c != 0 as *mut libc::c_void as *mut chunk) as libc::c_int as libc::c_long != 0
        && (*(*dest).first).type_0 as libc::c_uint
            == MEM_CHUNK as libc::c_int as libc::c_uint
    {
        if 0 as libc::c_int != chunkqueue_to_tempfiles(dest, errh) {
            return -(1 as libc::c_int) as ssize_t;
        }
        dlen = 0 as libc::c_int as off_t;
        iovcnt = 0 as libc::c_int as libc::c_uint;
    }
    if ((iovcnt as libc::c_ulong)
        < (::core::mem::size_of::<[iovec; 16]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<iovec>() as libc::c_ulong))
        as libc::c_int as libc::c_long != 0
    {
        c = (*src).first;
        while !c.is_null()
            && (*c).type_0 as libc::c_uint == MEM_CHUNK as libc::c_int as libc::c_uint
        {
            let mut clen_0: off_t = chunk_remaining_length(c);
            if clen_0 > len {
                clen_0 = len;
            }
            iov[iovcnt as usize]
                .iov_base = ((*(*c).mem).ptr).offset((*c).offset as isize)
                as *mut libc::c_void;
            iov[iovcnt as usize].iov_len = clen_0 as size_t;
            len -= clen_0;
            iovcnt = iovcnt.wrapping_add(1);
            iovcnt;
            if 0 as libc::c_int as libc::c_long == len {
                break;
            }
            if (iovcnt as libc::c_ulong
                == (::core::mem::size_of::<[iovec; 16]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<iovec>() as libc::c_ulong))
                as libc::c_int as libc::c_long != 0
            {
                break;
            }
            c = (*c).next;
        }
    }
    if (0 as libc::c_int as libc::c_uint == iovcnt) as libc::c_int as libc::c_long != 0 {
        return 0 as libc::c_int as ssize_t;
    }
    c = chunkqueue_get_append_tempfile(dest, errh);
    if c.is_null() {
        return -(1 as libc::c_int) as ssize_t;
    }
    let mut wr: ssize_t = pwritev(
        (*c).file.fd,
        iov.as_mut_ptr(),
        iovcnt as libc::c_int,
        (*c).file.length,
    );
    if wr >= 0 as libc::c_int as libc::c_long {
        (*c).file.length += wr;
        (*dest).bytes_in += wr;
        if dlen != 0 {
            if (wr < dlen) as libc::c_int as libc::c_long != 0 {
                return chunkqueue_append_cqmem_to_tempfile_partial(dest, c, wr, errh);
            }
            wr -= dlen;
            (*dest).bytes_in -= dlen;
            (*dest).bytes_out -= dlen;
            chunkqueue_mark_written(dest, dlen);
        }
    } else if chunkqueue_append_tempfile_err(dest, errh, c) != 0 {
        wr = 0 as libc::c_int as ssize_t;
    }
    return wr;
}
#[cold]
#[inline(never)]
unsafe extern "C" fn chunkqueue_append_drain_pipe_tempfile(
    cq: *mut chunkqueue,
    fd: libc::c_int,
    mut len: libc::c_uint,
    errh: *mut log_error_st,
) -> ssize_t {
    let mut buf: [libc::c_char; 16384] = [0; 16384];
    let mut rd: ssize_t = 0;
    loop {
        loop {
            rd = read(
                fd,
                buf.as_mut_ptr() as *mut libc::c_void,
                ::core::mem::size_of::<[libc::c_char; 16384]>() as libc::c_ulong,
            );
            if !(rd < 0 as libc::c_int as libc::c_long
                && *__errno_location() == 4 as libc::c_int)
            {
                break;
            }
        }
        if rd < 0 as libc::c_int as libc::c_long {
            break;
        }
        if 0 as libc::c_int
            != chunkqueue_append_mem_to_tempfile(
                cq,
                buf.as_mut_ptr(),
                rd as size_t,
                errh,
            )
        {
            break;
        }
        len = len.wrapping_sub(rd as libc::c_uint);
        if !(len != 0) {
            break;
        }
    }
    if 0 as libc::c_int as libc::c_uint == len {
        return 0 as libc::c_int as ssize_t
    } else {
        let errnum: libc::c_int = *__errno_location();
        if !((*cq).last).is_null()
            && 0 as libc::c_int as libc::c_long == chunk_remaining_length((*cq).last)
        {
            chunkqueue_remove_empty_chunks(cq);
        }
        return -errnum as ssize_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_append_splice_pipe_tempfile(
    cq: *mut chunkqueue,
    fd: libc::c_int,
    mut len: libc::c_uint,
    errh: *mut log_error_st,
) -> ssize_t {
    if !((*cq).first).is_null()
        && (*(*cq).first).type_0 as libc::c_uint
            == MEM_CHUNK as libc::c_int as libc::c_uint
    {
        let mut rc: libc::c_int = chunkqueue_to_tempfiles(cq, errh);
        if (0 as libc::c_int != rc) as libc::c_int as libc::c_long != 0 {
            return rc as ssize_t;
        }
    }
    let mut total: ssize_t = 0 as libc::c_int as ssize_t;
    loop {
        let c: *mut chunk = chunkqueue_get_append_tempfile(cq, errh);
        if (0 as *mut libc::c_void as *mut chunk == c) as libc::c_int as libc::c_long
            != 0
        {
            return -*__errno_location() as ssize_t;
        }
        let mut off: loff_t = (*c).file.length;
        let mut wr: ssize_t = splice(
            fd,
            0 as *mut __off64_t,
            (*c).file.fd,
            &mut off,
            len as size_t,
            (1 as libc::c_int | 2 as libc::c_int) as libc::c_uint,
        );
        if (wr as size_t == len as libc::c_ulong) as libc::c_int as libc::c_long != 0 {
            (*c).file.length += len as libc::c_long;
            (*cq).bytes_in += len as libc::c_long;
            return total + len as libc::c_long;
        } else if wr >= 0 as libc::c_int as libc::c_long {
            (*cq).bytes_in += wr;
            total += wr;
            len = (len as libc::c_ulong).wrapping_sub(wr as size_t) as libc::c_uint
                as libc::c_uint;
            (*c)
                .file
                .length = ((*c).file.length as libc::c_ulong).wrapping_add(wr as size_t)
                as off_t as off_t;
        } else {
            let errnum: libc::c_int = *__errno_location();
            match errnum {
                11 => {
                    if 0 as libc::c_int as libc::c_long == chunk_remaining_length(c) {
                        chunkqueue_remove_empty_chunks(cq);
                    }
                    return total;
                }
                22 => {
                    wr = chunkqueue_append_drain_pipe_tempfile(cq, fd, len, errh);
                    return if 0 as libc::c_int as libc::c_long == wr {
                        total + len as ssize_t
                    } else {
                        wr
                    };
                }
                _ => {
                    if chunkqueue_append_tempfile_err(cq, errh, c) == 0 {
                        return -errnum as ssize_t;
                    }
                }
            }
        }
        if !(len != 0) {
            break;
        }
    }
    return -(5 as libc::c_int) as ssize_t;
}
static mut cqpipes: [libc::c_int; 2] = [-(1 as libc::c_int), -(1 as libc::c_int)];
#[no_mangle]
#[cold]
#[inline(never)]
pub unsafe extern "C" fn chunkqueue_internal_pipes(mut init: libc::c_int) {
    if -(1 as libc::c_int) != cqpipes[0 as libc::c_int as usize] {
        close(cqpipes[0 as libc::c_int as usize]);
        cqpipes[0 as libc::c_int as usize] = -(1 as libc::c_int);
    }
    if -(1 as libc::c_int) != cqpipes[1 as libc::c_int as usize] {
        close(cqpipes[1 as libc::c_int as usize]);
        cqpipes[1 as libc::c_int as usize] = -(1 as libc::c_int);
    }
    if init != 0 {
        0 as libc::c_int
            != fdevent_pipe_cloexec(
                cqpipes.as_mut_ptr(),
                262144 as libc::c_int as libc::c_uint,
            );
    }
}
#[cold]
#[inline(never)]
unsafe extern "C" fn chunkqueue_pipe_read_discard() {
    let mut buf: [libc::c_char; 16384] = [0; 16384];
    let mut rd: ssize_t = 0;
    loop {
        rd = read(
            cqpipes[0 as libc::c_int as usize],
            buf.as_mut_ptr() as *mut libc::c_void,
            ::core::mem::size_of::<[libc::c_char; 16384]>() as libc::c_ulong,
        );
        if !(rd > 0 as libc::c_int as libc::c_long
            || rd < 0 as libc::c_int as libc::c_long
                && *__errno_location() == 4 as libc::c_int)
        {
            break;
        }
    }
    if rd < 0 as libc::c_int as libc::c_long && *__errno_location() != 11 as libc::c_int
    {
        chunkqueue_internal_pipes(1 as libc::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_append_splice_sock_tempfile(
    cq: *mut chunkqueue,
    fd: libc::c_int,
    mut len: libc::c_uint,
    errh: *mut log_error_st,
) -> ssize_t {
    let pipes: *mut libc::c_int = cqpipes.as_mut_ptr();
    if -(1 as libc::c_int) == *pipes.offset(1 as libc::c_int as isize) {
        return -(22 as libc::c_int) as ssize_t;
    }
    let mut wr: ssize_t = splice(
        fd,
        0 as *mut __off64_t,
        *pipes.offset(1 as libc::c_int as isize),
        0 as *mut __off64_t,
        len as size_t,
        (1 as libc::c_int | 2 as libc::c_int) as libc::c_uint,
    );
    if (wr <= 0 as libc::c_int as libc::c_long) as libc::c_int as libc::c_long != 0 {
        return -(22 as libc::c_int) as ssize_t;
    }
    len = wr as libc::c_uint;
    wr = chunkqueue_append_splice_pipe_tempfile(
        cq,
        *pipes.offset(0 as libc::c_int as isize),
        len,
        errh,
    );
    if wr < 0 as libc::c_int as libc::c_long {
        chunkqueue_pipe_read_discard();
    }
    return wr;
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_steal_with_tempfiles(
    dest: *mut chunkqueue,
    src: *mut chunkqueue,
    mut len: off_t,
    errh: *mut log_error_st,
) -> libc::c_int {
    let mut clen: off_t = 0;
    loop {
        let c: *mut chunk = (*src).first;
        if (0 as *mut libc::c_void as *mut chunk == c) as libc::c_int as libc::c_long
            != 0
        {
            break;
        }
        if (*c).type_0 as libc::c_uint == MEM_CHUNK as libc::c_int as libc::c_uint {
            clen = chunkqueue_append_cqmem_to_tempfile(dest, src, len, errh);
            if (clen < 0 as libc::c_int as libc::c_long) as libc::c_int as libc::c_long
                != 0
            {
                return -(1 as libc::c_int);
            }
            chunkqueue_mark_written(src, clen);
        } else {
            clen = chunk_remaining_length(c);
            if len < clen {
                clen = len;
            }
            chunkqueue_steal(dest, src, clen);
        }
        len -= clen;
        if !(len != 0) {
            break;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_append_cq_range(
    dst: *mut chunkqueue,
    src: *const chunkqueue,
    mut offset: off_t,
    mut len: off_t,
) {
    let mut c: *const chunk = (*src).first;
    while len > 0 as libc::c_int as libc::c_long && !c.is_null() {
        let mut clen: off_t = chunk_remaining_length(c);
        if offset >= clen {
            offset -= clen;
        } else {
            clen -= offset;
            if len < clen {
                clen = len;
            }
            len -= clen;
            if (*c).type_0 as libc::c_uint == FILE_CHUNK as libc::c_int as libc::c_uint {
                chunkqueue_append_file(dst, (*c).mem, (*c).offset + offset, clen);
                chunkqueue_dup_file_chunk_fd((*dst).last, c);
            } else {
                chunkqueue_append_mem(
                    dst,
                    ((*(*c).mem).ptr)
                        .offset((*c).offset as isize)
                        .offset(offset as isize),
                    clen as size_t,
                );
            }
            offset = 0 as libc::c_int as off_t;
        }
        c = (*c).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_mark_written(
    mut cq: *mut chunkqueue,
    mut len: off_t,
) {
    (*cq).bytes_out += len;
    let mut c: *mut chunk = (*cq).first;
    while !c.is_null() {
        let mut c_len: off_t = chunk_remaining_length(c);
        if len >= c_len {
            let x: *mut chunk = c;
            c = (*c).next;
            len -= c_len;
            chunk_release(x);
        } else {
            (*c).offset += len;
            (*cq).first = c;
            return;
        }
    }
    (*cq).last = 0 as *mut chunk;
    (*cq).first = (*cq).last;
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_remove_finished_chunks(mut cq: *mut chunkqueue) {
    let mut c: *mut chunk = 0 as *mut chunk;
    loop {
        c = (*cq).first;
        if !(!c.is_null()
            && 0 as libc::c_int as libc::c_long == chunk_remaining_length(c))
        {
            break;
        }
        (*cq).first = (*c).next;
        if ((*cq).first).is_null() {
            (*cq).last = 0 as *mut chunk;
        }
        chunk_release(c);
    };
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn chunkqueue_remove_empty_chunks(mut cq: *mut chunkqueue) {
    let mut c: *mut chunk = 0 as *mut chunk;
    chunkqueue_remove_finished_chunks(cq);
    c = (*cq).first;
    while !c.is_null() && !((*c).next).is_null() {
        if 0 as libc::c_int as libc::c_long == chunk_remaining_length((*c).next) {
            let mut empty: *mut chunk = (*c).next;
            (*c).next = (*empty).next;
            if empty == (*cq).last {
                (*cq).last = c;
            }
            chunk_release(empty);
        }
        c = (*c).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_compact_mem_offset(cq: *mut chunkqueue) {
    let c: *mut chunk = (*cq).first;
    if 0 as libc::c_int as libc::c_long == (*c).offset {
        return;
    }
    if (*c).type_0 as libc::c_uint != MEM_CHUNK as libc::c_int as libc::c_uint {
        return;
    }
    let b: *mut buffer = (*c).mem;
    let mut len: size_t = (buffer_clen(b) as libc::c_long - (*c).offset) as size_t;
    memmove(
        (*b).ptr as *mut libc::c_void,
        ((*b).ptr).offset((*c).offset as isize) as *const libc::c_void,
        len,
    );
    (*c).offset = 0 as libc::c_int as off_t;
    buffer_truncate(b, len as uint32_t);
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_compact_mem(
    mut cq: *mut chunkqueue,
    mut clen: size_t,
) {
    let mut c: *mut chunk = (*cq).first;
    let mut b: *mut buffer = (*c).mem;
    let mut len: size_t = (buffer_clen(b) as libc::c_long - (*c).offset) as size_t;
    if len >= clen {
        return;
    }
    if (*b).size as libc::c_ulong > clen {
        if (buffer_string_space(b) as libc::c_ulong) < clen.wrapping_sub(len) {
            chunkqueue_compact_mem_offset(cq);
        }
    } else {
        b = chunkqueue_prepend_buffer_open_sz(
            cq,
            clen.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        buffer_append_string_len(b, ((*(*c).mem).ptr).offset((*c).offset as isize), len);
        (*(*cq).first).next = (*c).next;
        if ((*c).next).is_null() {
            (*cq).last = (*cq).first;
        }
        chunk_release(c);
        c = (*cq).first;
    }
    let mut fc: *mut chunk = c;
    loop {
        clen = (clen as libc::c_ulong).wrapping_sub(len) as size_t as size_t;
        if !(clen != 0
            && {
                c = (*fc).next;
                !c.is_null()
            })
        {
            break;
        }
        len = (buffer_clen((*c).mem) as libc::c_long - (*c).offset) as size_t;
        if len > clen {
            buffer_append_string_len(
                b,
                ((*(*c).mem).ptr).offset((*c).offset as isize),
                clen,
            );
            (*c)
                .offset = ((*c).offset as libc::c_ulong).wrapping_add(clen) as off_t
                as off_t;
            break;
        } else {
            buffer_append_string_len(
                b,
                ((*(*c).mem).ptr).offset((*c).offset as isize),
                len,
            );
            (*fc).next = (*c).next;
            if ((*c).next).is_null() {
                (*cq).last = fc;
            }
            chunk_release(c);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn chunk_open_file_chunk(
    c: *mut chunk,
    errh: *mut log_error_st,
) -> libc::c_int {
    if -(1 as libc::c_int) == (*c).file.fd {
        (*c)
            .file
            .fd = fdevent_open_cloexec(
            (*(*c).mem).ptr,
            1 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int as mode_t,
        );
        if -(1 as libc::c_int) == (*c).file.fd {
            log_perror(
                errh,
                b"chunk.c\0" as *const u8 as *const libc::c_char,
                1453 as libc::c_int as libc::c_uint,
                b"open failed: %s\0" as *const u8 as *const libc::c_char,
                (*(*c).mem).ptr,
            );
            return -(1 as libc::c_int);
        }
    }
    if (*c).file.is_temp != 0 {
        return 0 as libc::c_int;
    }
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
    if -(1 as libc::c_int) == fstat((*c).file.fd, &mut st) {
        log_perror(
            errh,
            b"chunk.c\0" as *const u8 as *const libc::c_char,
            1463 as libc::c_int as libc::c_uint,
            b"fstat failed\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (*c).file.length > st.st_size {
        log_error(
            errh,
            b"chunk.c\0" as *const u8 as *const libc::c_char,
            1469 as libc::c_int as libc::c_uint,
            b"file shrunk: %s\0" as *const u8 as *const libc::c_char,
            (*(*c).mem).ptr,
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn chunkqueue_write_data(
    fd: libc::c_int,
    mut buf: *const libc::c_void,
    mut len: size_t,
) -> ssize_t {
    let mut wr: ssize_t = 0 as libc::c_int as ssize_t;
    if len != 0 {
        loop {
            wr = write(fd, buf, len);
            if !(-(1 as libc::c_int) as libc::c_long == wr
                && *__errno_location() == 4 as libc::c_int)
            {
                break;
            }
        }
    }
    return wr;
}
#[cold]
#[inline(never)]
unsafe extern "C" fn chunkqueue_write_chunk_file_intermed(
    fd: libc::c_int,
    c: *mut chunk,
    errh: *mut log_error_st,
) -> ssize_t {
    let mut buf: [libc::c_char; 16384] = [0; 16384];
    let mut data: *mut libc::c_char = buf.as_mut_ptr();
    let len: off_t = (*c).file.length - (*c).offset;
    let mut dlen: uint32_t = (if len
        < ::core::mem::size_of::<[libc::c_char; 16384]>() as libc::c_ulong as off_t
    {
        len as uint32_t as libc::c_ulong
    } else {
        ::core::mem::size_of::<[libc::c_char; 16384]>() as libc::c_ulong
    }) as uint32_t;
    let mut cq: chunkqueue = {
        let mut init = chunkqueue {
            first: c,
            last: c,
            bytes_in: 0 as libc::c_int as off_t,
            bytes_out: 0 as libc::c_int as off_t,
            upload_temp_file_size: 0 as libc::c_int as off_t,
            tempdir_idx: 0 as libc::c_int as libc::c_uint,
        };
        init
    };
    if 0 as libc::c_int
        != chunkqueue_peek_data(&mut cq, &mut data, &mut dlen, errh, 0 as libc::c_int)
        && 0 as libc::c_int as libc::c_uint == dlen
    {
        return -(1 as libc::c_int) as ssize_t;
    }
    return chunkqueue_write_data(fd, data as *const libc::c_void, dlen as size_t);
}
unsafe extern "C" fn chunkqueue_write_chunk_file(
    fd: libc::c_int,
    c: *mut chunk,
    errh: *mut log_error_st,
) -> ssize_t {
    if 0 as libc::c_int != chunk_open_file_chunk(c, errh) {
        return -(1 as libc::c_int) as ssize_t;
    }
    let len: off_t = (*c).file.length - (*c).offset;
    if 0 as libc::c_int as libc::c_long == len {
        return 0 as libc::c_int as ssize_t;
    }
    let mut offset: off_t = (*c).offset;
    let wr: ssize_t = sendfile(
        fd,
        (*c).file.fd,
        &mut offset,
        (if len < 2147483647 as libc::c_int as libc::c_long {
            len
        } else {
            2147483647 as libc::c_int as libc::c_long
        }) as size_t,
    );
    if (wr >= 0 as libc::c_int as libc::c_long) as libc::c_int as libc::c_long != 0
        || *__errno_location() != 22 as libc::c_int
            && *__errno_location() != 38 as libc::c_int
    {
        return wr;
    }
    return chunkqueue_write_chunk_file_intermed(fd, c, errh);
}
unsafe extern "C" fn chunkqueue_write_chunk_mem(
    fd: libc::c_int,
    c: *const chunk,
) -> ssize_t {
    let buf: *const libc::c_void = ((*(*c).mem).ptr).offset((*c).offset as isize)
        as *const libc::c_void;
    let len: size_t = (buffer_clen((*c).mem) as libc::c_ulong)
        .wrapping_sub((*c).offset as size_t);
    return chunkqueue_write_data(fd, buf, len);
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_write_chunk(
    fd: libc::c_int,
    cq: *mut chunkqueue,
    errh: *mut log_error_st,
) -> ssize_t {
    let c: *mut chunk = (*cq).first;
    match (*c).type_0 as libc::c_uint {
        0 => return chunkqueue_write_chunk_mem(fd, c),
        1 => return chunkqueue_write_chunk_file(fd, c, errh),
        _ => {
            *__errno_location() = 22 as libc::c_int;
            return -(1 as libc::c_int) as ssize_t;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_write_chunk_to_pipe(
    fd: libc::c_int,
    cq: *mut chunkqueue,
    errh: *mut log_error_st,
) -> ssize_t {
    let c: *mut chunk = (*cq).first;
    if (*c).type_0 as libc::c_uint == FILE_CHUNK as libc::c_int as libc::c_uint {
        let len: size_t = ((*c).file.length - (*c).offset) as size_t;
        let mut abs_offset: loff_t = (*c).offset;
        if (0 as libc::c_int as libc::c_ulong == len) as libc::c_int as libc::c_long != 0
        {
            return 0 as libc::c_int as ssize_t;
        }
        return if 0 as libc::c_int == chunk_open_file_chunk(c, errh) {
            splice(
                (*c).file.fd,
                &mut abs_offset,
                fd,
                0 as *mut __off64_t,
                len,
                2 as libc::c_int as libc::c_uint,
            )
        } else {
            -(1 as libc::c_int) as libc::c_long
        };
    }
    return chunkqueue_write_chunk(fd, cq, errh);
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_small_resp_optim(cq: *mut chunkqueue) {
    let mut c: *mut chunk = (*cq).first;
    let filec: *mut chunk = (*c).next;
    if filec != (*cq).last
        || (*filec).type_0 as libc::c_uint != FILE_CHUNK as libc::c_int as libc::c_uint
        || (*filec).file.fd < 0 as libc::c_int
    {
        return;
    }
    let mut len: off_t = (*filec).file.length - (*filec).offset;
    if len as size_t > buffer_string_space((*c).mem) as libc::c_ulong {
        (*c)
            .next = chunk_acquire(
            (len as size_t).wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        c = (*c).next;
    }
    (*c).next = 0 as *mut chunk;
    (*cq).last = c;
    let mut rd: ssize_t = 0;
    let mut offset: off_t = 0 as libc::c_int as off_t;
    let ptr: *mut libc::c_char = buffer_extend((*c).mem, len as size_t);
    loop {
        rd = chunk_file_pread(
            (*filec).file.fd,
            ptr.offset(offset as isize) as *mut libc::c_void,
            len as size_t,
            (*filec).offset + offset,
        );
        if !(rd > 0 as libc::c_int as libc::c_long
            && {
                offset += rd;
                len -= rd;
                len != 0
            })
        {
            break;
        }
    }
    if (0 as libc::c_int as libc::c_long == len) as libc::c_int as libc::c_long != 0 {
        chunk_release(filec);
    } else {
        buffer_truncate(
            (*c).mem,
            ptr.offset(offset as isize).offset_from((*(*c).mem).ptr) as libc::c_long
                as uint32_t,
        );
        (*c).next = filec;
        (*cq).last = (*c).next;
        if offset != 0 {
            (*filec).offset += offset;
        } else if ((*cq).first != c) as libc::c_int as libc::c_long != 0 {
            (*(*cq).first).next = filec;
            chunk_release(c);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_peek_data(
    cq: *mut chunkqueue,
    data: *mut *mut libc::c_char,
    dlen: *mut uint32_t,
    errh: *mut log_error_st,
    mut nowait: libc::c_int,
) -> libc::c_int {
    let data_in: *mut libc::c_char = *data;
    let data_insz: uint32_t = *dlen;
    *dlen = 0 as libc::c_int as uint32_t;
    let mut c: *mut chunk = (*cq).first;
    while !c.is_null() {
        let space: uint32_t = data_insz.wrapping_sub(*dlen);
        match (*c).type_0 as libc::c_uint {
            0 => {
                let mut have: uint32_t = (buffer_clen((*c).mem))
                    .wrapping_sub((*c).offset as uint32_t);
                if !((0 as libc::c_int as libc::c_uint == have) as libc::c_int
                    as libc::c_long != 0)
                {
                    if have > space {
                        have = space;
                    }
                    if *dlen != 0 {
                        memcpy(
                            data_in.offset(*dlen as isize) as *mut libc::c_void,
                            ((*(*c).mem).ptr).offset((*c).offset as isize)
                                as *const libc::c_void,
                            have as libc::c_ulong,
                        );
                    } else {
                        *data = ((*(*c).mem).ptr).offset((*c).offset as isize);
                    }
                    *dlen = (*dlen as libc::c_uint).wrapping_add(have) as uint32_t
                        as uint32_t;
                }
            }
            1 => {
                if (*c).file.fd >= 0 as libc::c_int
                    || 0 as libc::c_int == chunk_open_file_chunk(c, errh)
                {
                    let mut len: off_t = (*c).file.length - (*c).offset;
                    if !((0 as libc::c_int as libc::c_long == len) as libc::c_int
                        as libc::c_long != 0)
                    {
                        if len > space as off_t {
                            len = space as off_t;
                        }
                        (*c)
                            .file
                            .busy = ((*c).file.busy as libc::c_int
                            | (nowait == 0) as libc::c_int) as uint8_t;
                        let mut rd: ssize_t = chunk_file_pread_chunk(
                            c,
                            data_in.offset(*dlen as isize) as *mut libc::c_void,
                            len as size_t,
                        );
                        if (rd <= 0 as libc::c_int as libc::c_long) as libc::c_int
                            as libc::c_long != 0
                        {
                            if nowait != 0 && (*c).file.busy as libc::c_int != 0 {
                                return 0 as libc::c_int;
                            }
                            log_perror(
                                errh,
                                b"chunk.c\0" as *const u8 as *const libc::c_char,
                                1742 as libc::c_int as libc::c_uint,
                                b"read(\"%s\")\0" as *const u8 as *const libc::c_char,
                                (*(*c).mem).ptr,
                            );
                            return -(1 as libc::c_int);
                        }
                        *dlen = (*dlen as libc::c_uint).wrapping_add(rd as uint32_t)
                            as uint32_t as uint32_t;
                        if nowait != 0 && rd != len {
                            return 0 as libc::c_int;
                        }
                    }
                } else {
                    (*c).file.busy = 0 as libc::c_int as uint8_t;
                    return -(1 as libc::c_int);
                }
            }
            _ => return -(1 as libc::c_int),
        }
        if *dlen == data_insz {
            break;
        }
        c = (*c).next;
        if c.is_null() {
            break;
        }
        if *dlen != 0 && *data != data_in {
            memcpy(
                data_in as *mut libc::c_void,
                *data as *const libc::c_void,
                *dlen as libc::c_ulong,
            );
            *data = data_in;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_read_data(
    cq: *mut chunkqueue,
    data: *mut libc::c_char,
    dlen: uint32_t,
    errh: *mut log_error_st,
) -> libc::c_int {
    let mut ptr: *mut libc::c_char = data;
    let mut len: uint32_t = dlen;
    if chunkqueue_peek_data(cq, &mut ptr, &mut len, errh, 0 as libc::c_int)
        < 0 as libc::c_int || len != dlen
    {
        return -(1 as libc::c_int);
    }
    if data != ptr {
        memcpy(
            data as *mut libc::c_void,
            ptr as *const libc::c_void,
            len as libc::c_ulong,
        );
    }
    chunkqueue_mark_written(cq, len as off_t);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_read_squash(
    cq: *mut chunkqueue,
    errh: *mut log_error_st,
) -> *mut chunk {
    let mut cqlen: off_t = chunkqueue_length(cq);
    if cqlen >= 4294967295 as libc::c_uint as libc::c_long {
        return 0 as *mut chunk;
    }
    if !((*cq).first).is_null() && ((*(*cq).first).next).is_null()
        && (*(*cq).first).type_0 as libc::c_uint
            == MEM_CHUNK as libc::c_int as libc::c_uint
    {
        return (*cq).first;
    }
    let c: *mut chunk = chunk_acquire(
        (cqlen as uint32_t).wrapping_add(1 as libc::c_int as libc::c_uint) as size_t,
    );
    let mut data: *mut libc::c_char = (*(*c).mem).ptr;
    let mut dlen: uint32_t = cqlen as uint32_t;
    let mut rc: libc::c_int = chunkqueue_peek_data(
        cq,
        &mut data,
        &mut dlen,
        errh,
        0 as libc::c_int,
    );
    if rc < 0 as libc::c_int {
        chunk_release(c);
        return 0 as *mut chunk;
    }
    buffer_truncate((*c).mem, dlen);
    chunkqueue_release_chunks(cq);
    chunkqueue_append_chunk(cq, c);
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn chunkqueue_chunk_file_viewadj(
    c: *mut chunk,
    mut n: off_t,
    mut errh: *mut log_error_st,
) -> *const chunk_file_view {
    if (*c).file.fd < 0 as libc::c_int
        && 0 as libc::c_int != chunk_open_file_chunk(c, errh)
    {
        return 0 as *const chunk_file_view;
    }
    let mut cfv: *mut chunk_file_view = (*c).file.view;
    if cfv.is_null() {
        (*c).file.view = chunk_file_view_init() as *mut chunk_file_view;
        cfv = (*c).file.view;
    } else if -(1 as libc::c_int) as *mut libc::c_void
        != (*cfv).mptr as *mut libc::c_void
    {
        munmap((*cfv).mptr as *mut libc::c_void, (*cfv).mlen as size_t);
    }
    (*cfv).foff = (*c).offset & chunk_pagemask;
    if 0 as libc::c_int as libc::c_long != n {
        (*cfv).mlen = (*c).offset - (*cfv).foff + n;
    } else {
        (*cfv).mlen = (512 as libc::c_int * 1024 as libc::c_int) as off_t;
    }
    if (*cfv).mlen < (512 as libc::c_int * 1024 as libc::c_int) as libc::c_long {
        (*cfv).mlen = (512 as libc::c_int * 1024 as libc::c_int) as off_t;
    }
    if (*cfv).mlen > (*c).file.length - (*cfv).foff {
        (*cfv).mlen = (*c).file.length - (*cfv).foff;
    }
    (*cfv)
        .mptr = mmap(
        0 as *mut libc::c_void,
        (*cfv).mlen as size_t,
        0x1 as libc::c_int,
        if (*c).file.is_temp as libc::c_int != 0 {
            0x2 as libc::c_int
        } else {
            chunk_mmap_flags
        },
        (*c).file.fd,
        (*cfv).foff,
    ) as *mut libc::c_char;
    if (-(1 as libc::c_int) as *mut libc::c_void == (*cfv).mptr as *mut libc::c_void)
        as libc::c_int as libc::c_long != 0
    {
        if (*__errno_location() == 22 as libc::c_int) as libc::c_int as libc::c_long != 0
        {
            chunk_mmap_flags &= !(0x1 as libc::c_int);
            chunk_mmap_flags |= 0x2 as libc::c_int;
            (*cfv)
                .mptr = mmap(
                0 as *mut libc::c_void,
                (*cfv).mlen as size_t,
                0x1 as libc::c_int,
                0x2 as libc::c_int,
                (*c).file.fd,
                (*cfv).foff,
            ) as *mut libc::c_char;
        }
        if (-(1 as libc::c_int) as *mut libc::c_void == (*cfv).mptr as *mut libc::c_void)
            as libc::c_int as libc::c_long != 0
        {
            (*c).file.view = chunk_file_view_failed(cfv);
            return 0 as *const chunk_file_view;
        }
    }
    return cfv;
}
