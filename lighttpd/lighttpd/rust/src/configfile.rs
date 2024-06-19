use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type pcre2_real_match_data_8;
    pub type http_dispatch;
    pub type fdevents;
    pub type pcre2_real_general_context_8;
    pub type dirent;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn chdir(__path: *const libc::c_char) -> libc::c_int;
    fn getcwd(__buf: *mut libc::c_char, __size: size_t) -> *mut libc::c_char;
    fn dup(__fd: libc::c_int) -> libc::c_int;
    fn getpid() -> __pid_t;
    fn buffer_init() -> *mut buffer;
    fn buffer_free(b: *mut buffer);
    fn buffer_string_prepare_append(b: *mut buffer, size: size_t) -> *mut libc::c_char;
    fn buffer_extend(b: *mut buffer, x: size_t) -> *mut libc::c_char;
    fn buffer_commit(b: *mut buffer, size: size_t);
    fn buffer_copy_string(b: *mut buffer, s: *const libc::c_char);
    fn buffer_copy_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn buffer_append_string(b: *mut buffer, s: *const libc::c_char);
    fn buffer_append_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn buffer_append_str3(
        b: *mut buffer,
        s1: *const libc::c_char,
        len1: size_t,
        s2: *const libc::c_char,
        len2: size_t,
        s3: *const libc::c_char,
        len3: size_t,
    );
    fn buffer_append_int(b: *mut buffer, val: intmax_t);
    fn buffer_eq_slen(
        b: *const buffer,
        s: *const libc::c_char,
        slen: size_t,
    ) -> libc::c_int;
    fn buffer_is_equal(a: *const buffer, b: *const buffer) -> libc::c_int;
    fn buffer_to_lower(b: *mut buffer);
    fn buffer_to_upper(b: *mut buffer);
    fn buffer_append_path_len(b: *mut buffer, a: *const libc::c_char, alen: size_t);
    fn buffer_copy_path_len2(
        b: *mut buffer,
        s1: *const libc::c_char,
        len1: size_t,
        s2: *const libc::c_char,
        len2: size_t,
    );
    fn ck_memclear_s(s: *mut libc::c_void, smax: rsize_t, n: rsize_t) -> errno_t;
    fn ck_malloc(nbytes: size_t) -> *mut libc::c_void;
    fn ck_calloc(nmemb: size_t, elt_sz: size_t) -> *mut libc::c_void;
    fn ck_assert_failed(
        filename: *const libc::c_char,
        line: libc::c_uint,
        msg: *const libc::c_char,
    ) -> !;
    fn array_init(n: uint32_t) -> *mut array;
    fn array_copy_array(dst: *mut array, src: *const array);
    fn array_free_data(a: *mut array);
    fn array_free(a: *mut array);
    fn array_insert_unique(a: *mut array, entry: *mut data_unset);
    fn array_get_element_klen(
        a: *const array,
        key: *const libc::c_char,
        klen: uint32_t,
    ) -> *const data_unset;
    fn array_get_data_unset(
        a: *const array,
        key: *const libc::c_char,
        klen: uint32_t,
    ) -> *mut data_unset;
    fn array_get_int_ptr(
        a: *mut array,
        k: *const libc::c_char,
        klen: uint32_t,
    ) -> *mut libc::c_int;
    fn array_get_buf_ptr(
        a: *mut array,
        k: *const libc::c_char,
        klen: uint32_t,
    ) -> *mut buffer;
    fn array_insert_value(a: *mut array, v: *const libc::c_char, vlen: uint32_t);
    fn fdlog_open(fn_0: *const libc::c_char) -> *mut fdlog_st;
    fn fdlog_closeall(errh: *mut fdlog_st);
    fn fdlog_pipe_serrh(fd: libc::c_int);
    fn fdlog_closelog();
    fn fdlog_openlog(errh: *mut fdlog_st, syslog_facility: *const buffer);
    fn chunkqueue_set_chunk_size(sz: size_t);
    fn chunkqueue_set_tempdirs_default(
        tempdirs: *const array,
        upload_temp_file_size: off_t,
    );
    fn chunkqueue_env_tmpdir() -> *const libc::c_char;
    fn sock_addr_inet_pton(
        saddr: *mut sock_addr,
        str: *const libc::c_char,
        family: libc::c_int,
        port: libc::c_ushort,
    ) -> libc::c_int;
    fn http_request_host_normalize(
        b: *mut buffer,
        scheme_port: libc::c_int,
    ) -> libc::c_int;
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn fdevent_config(
        event_handler_name: *mut *const libc::c_char,
        errh: *mut log_error_st,
    ) -> libc::c_int;
    fn fdevent_setfd_cloexec(fd: libc::c_int);
    fn fdevent_pipe_cloexec(
        fds: *mut libc::c_int,
        bufsz_hint: libc::c_uint,
    ) -> libc::c_int;
    fn fdevent_open_devnull() -> libc::c_int;
    fn fdevent_set_stdin_stdout_stderr(
        fdin: libc::c_int,
        fdout: libc::c_int,
        fderr: libc::c_int,
    ) -> libc::c_int;
    fn fdevent_sh_exec(
        cmdstr: *const libc::c_char,
        envp: *mut *mut libc::c_char,
        fdin: libc::c_int,
        fdout: libc::c_int,
        fderr: libc::c_int,
    ) -> pid_t;
    fn fdevent_waitpid(pid: pid_t, status: *mut libc::c_int, nb: libc::c_int) -> pid_t;
    fn fdevent_load_file(
        fn_0: *const libc::c_char,
        lim: *mut off_t,
        errh: *mut log_error_st,
        malloc_fn: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
        free_fn: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    ) -> *mut libc::c_char;
    fn pcre_keyvalue_burl_normalize_key(k: *mut buffer, t: *mut buffer);
    fn log_pri(
        errh: *mut log_error_st,
        filename: *const libc::c_char,
        line: libc::c_uint,
        pri: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
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
    fn log_set_global_errh(
        errh: *mut log_error_st,
        ts_high_precision: libc::c_int,
    ) -> *mut log_error_st;
    fn data_config_init() -> *mut data_config;
    fn data_config_pcre_compile(
        dc: *mut data_config,
        pcre_jit: libc::c_int,
        errh: *mut log_error_st,
    ) -> libc::c_int;
    fn configparser(
        yyp: *mut libc::c_void,
        yymajor: libc::c_int,
        yyminor: *mut buffer,
        ctx: *mut config_t,
    );
    fn configparserFree(
        p: *mut libc::c_void,
        freeProc: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    );
    fn configparserAlloc(
        mallocProc: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    ) -> *mut libc::c_void;
    fn config_plugin_values_init(
        srv: *mut server,
        p_d: *mut libc::c_void,
        cpk: *const config_plugin_keys_t,
        mname: *const libc::c_char,
    ) -> libc::c_int;
    fn config_plugin_value_to_bool(
        du: *const data_unset,
        default_value: libc::c_int,
    ) -> libc::c_int;
    fn config_check_cond(r: *mut request_st, context_ndx: libc::c_int) -> libc::c_int;
    fn config_feature_bool(
        srv: *const server,
        feature: *const libc::c_char,
        default_value: libc::c_int,
    ) -> libc::c_int;
    fn request_config_set_defaults(config_defaults: *const request_config);
    fn stat_cache_choose_engine(
        stat_cache_string: *const buffer,
        errh: *mut log_error_st,
    ) -> libc::c_int;
    fn stat_cache_xattrname(name: *const libc::c_char);
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn glob(
        __pattern: *const libc::c_char,
        __flags: libc::c_int,
        __errfunc: Option::<
            unsafe extern "C" fn(*const libc::c_char, libc::c_int) -> libc::c_int,
        >,
        __pglob: *mut glob_t,
    ) -> libc::c_int;
    fn globfree(__pglob: *mut glob_t);
    fn pcre2_match_data_create_8(
        _: uint32_t,
        _: *mut pcre2_general_context_8,
    ) -> *mut pcre2_match_data_8;
    fn pcre2_match_data_free_8(_: *mut pcre2_match_data_8);
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __intmax_t = libc::c_long;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino64_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_uint;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_int;
pub type __blkcnt64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type ino_t = __ino64_t;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type off_t = __off64_t;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type rsize_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type uint_fast32_t = libc::c_ulong;
pub type uintptr_t = libc::c_ulong;
pub type intmax_t = __intmax_t;
pub type unix_time64_t = time_t;
pub type unix_timespec64_t = timespec;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct server {
    pub plugin_slots: *mut libc::c_void,
    pub config_context: *mut array,
    pub config_captures: libc::c_int,
    pub ev: *mut fdevents,
    pub network_backend_write: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *mut chunkqueue,
            off_t,
            *mut log_error_st,
        ) -> libc::c_int,
    >,
    pub request_env: Option::<unsafe extern "C" fn(*mut request_st) -> handler_t>,
    pub tmp_buf: *mut buffer,
    pub max_fds: libc::c_int,
    pub max_fds_lowat: libc::c_int,
    pub max_fds_hiwat: libc::c_int,
    pub cur_fds: libc::c_int,
    pub sockets_disabled: libc::c_int,
    pub lim_conns: uint32_t,
    pub conns: *mut connection,
    pub conns_pool: *mut connection,
    pub errh: *mut log_error_st,
    pub loadts: unix_time64_t,
    pub loadavg: [libc::c_double; 3],
    pub plugins_request_reset: Option::<
        unsafe extern "C" fn(*mut request_st) -> handler_t,
    >,
    pub srvconf: server_config,
    pub config_data_base: *mut libc::c_void,
    pub srv_sockets: server_socket_array,
    pub srv_sockets_inherited: server_socket_array,
    pub plugins: C2RustUnnamed,
    pub startup_ts: unix_time64_t,
    pub graceful_expire_ts: unix_time64_t,
    pub uid: uid_t,
    pub gid: gid_t,
    pub pid: pid_t,
    pub stdin_fd: libc::c_int,
    pub default_server_tag: *const buffer,
    pub argv: *mut *mut libc::c_char,
    pub match_data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buffer {
    pub ptr: *mut libc::c_char,
    pub used: uint32_t,
    pub size: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub ptr: *mut libc::c_void,
    pub used: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct server_socket_array {
    pub ptr: *mut *mut server_socket,
    pub used: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct server_socket {
    pub addr: sock_addr,
    pub fd: libc::c_int,
    pub is_ssl: uint8_t,
    pub srv_token_colon: uint8_t,
    pub sidx: libc::c_ushort,
    pub fdn: *mut fdnode,
    pub srv: *mut server,
    pub srv_token: *mut buffer,
}
pub type fdnode = fdnode_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fdnode_st {
    pub handler: fdevent_handler,
    pub ctx: *mut libc::c_void,
    pub fd: libc::c_int,
    pub events: libc::c_int,
    pub fde_ndx: libc::c_int,
}
pub type fdevent_handler = Option::<
    unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> handler_t,
>;
pub type handler_t = libc::c_uint;
pub const HANDLER_ERROR: handler_t = 4;
pub const HANDLER_COMEBACK: handler_t = 3;
pub const HANDLER_WAIT_FOR_EVENT: handler_t = 2;
pub const HANDLER_FINISHED: handler_t = 1;
pub const HANDLER_GO_ON: handler_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sock_addr {
    pub ipv6: sockaddr_in6,
    pub ipv4: sockaddr_in,
    pub un: sockaddr_un,
    pub plain: sockaddr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [libc::c_char; 108],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: uint32_t,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct server_config {
    pub max_request_field_size: uint32_t,
    pub log_request_header_on_error: libc::c_uchar,
    pub http_header_strict: libc::c_uchar,
    pub http_host_strict: libc::c_uchar,
    pub http_host_normalize: libc::c_uchar,
    pub http_method_get_body: libc::c_uchar,
    pub high_precision_timestamps: libc::c_uchar,
    pub h2proto: libc::c_uchar,
    pub absolute_dir_redirect: libc::c_uchar,
    pub http_url_normalize: libc::c_ushort,
    pub max_worker: libc::c_ushort,
    pub max_fds: libc::c_ushort,
    pub max_conns: libc::c_ushort,
    pub port: libc::c_ushort,
    pub upload_temp_file_size: libc::c_uint,
    pub upload_tempdirs: *mut array,
    pub dont_daemonize: libc::c_uchar,
    pub preflight_check: libc::c_uchar,
    pub enable_cores: libc::c_uchar,
    pub compat_module_load: libc::c_uchar,
    pub config_deprecated: libc::c_uchar,
    pub config_unsupported: libc::c_uchar,
    pub systemd_socket_activation: libc::c_uchar,
    pub errorlog_use_syslog: libc::c_uchar,
    pub syslog_facility: *const buffer,
    pub bindhost: *const buffer,
    pub changeroot: *const buffer,
    pub username: *const buffer,
    pub groupname: *const buffer,
    pub network_backend: *const buffer,
    pub feature_flags: *const array,
    pub event_handler: *const libc::c_char,
    pub modules_dir: *const libc::c_char,
    pub pid_file: *mut buffer,
    pub modules: *mut array,
    pub config_touched: *mut array,
    pub mimetypes_default: array,
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
pub struct request_st {
    pub state: request_state_t,
    pub http_status: libc::c_int,
    pub x: C2RustUnnamed_4,
    pub http_method: http_method_t,
    pub http_version: http_version_t,
    pub handler_module: *const plugin,
    pub plugin_ctx: *mut *mut libc::c_void,
    pub con: *mut connection,
    pub conditional_is_valid: uint32_t,
    pub cond_cache: *mut cond_cache_t,
    pub cond_match: *mut *mut cond_match_t,
    pub cond_match_data: *mut cond_match_t,
    pub conf: request_config,
    pub rqst_header_len: uint32_t,
    pub rqst_htags: uint64_t,
    pub rqst_headers: array,
    pub uri: request_uri,
    pub physical: physical,
    pub env: array,
    pub reqbody_length: off_t,
    pub resp_body_scratchpad: off_t,
    pub http_host: *mut buffer,
    pub server_name: *const buffer,
    pub target: buffer,
    pub target_orig: buffer,
    pub pathinfo: buffer,
    pub server_name_buf: buffer,
    pub dst_addr: *mut libc::c_void,
    pub dst_addr_buf: *mut buffer,
    pub resp_header_len: uint32_t,
    pub resp_htags: uint64_t,
    pub resp_headers: array,
    pub resp_body_finished: libc::c_char,
    pub resp_body_started: libc::c_char,
    pub resp_send_chunked: libc::c_char,
    pub resp_decode_chunked: libc::c_char,
    pub resp_header_repeated: libc::c_char,
    pub loops_per_request: libc::c_char,
    pub keep_alive: int8_t,
    pub async_callback: libc::c_char,
    pub tmp_buf: *mut buffer,
    pub gw_dechunk: *mut response_dechunk,
    pub start_hp: unix_timespec64_t,
    pub error_handler_saved_status: libc::c_int,
    pub error_handler_saved_method: http_method_t,
    pub write_queue: chunkqueue,
    pub read_queue: chunkqueue,
    pub reqbody_queue: chunkqueue,
    pub tmp_sce: *mut stat_cache_entry,
    pub cond_captures: libc::c_int,
    pub h2_connect_ext: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat_cache_entry {
    pub name: buffer,
    pub stat_ts: unix_time64_t,
    pub fd: libc::c_int,
    pub refcnt: libc::c_int,
    pub fam_dir: *mut libc::c_void,
    pub etag: buffer,
    pub content_type: buffer,
    pub st: stat,
}
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
pub struct chunk {
    pub next: *mut chunk,
    pub type_0: C2RustUnnamed_2,
    pub mem: *mut buffer,
    pub offset: off_t,
    pub file: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub length: off_t,
    pub fd: libc::c_int,
    pub is_temp: uint8_t,
    pub busy: uint8_t,
    pub flagmask: uint8_t,
    pub view: *mut chunk_file_view,
    pub ref_0: *mut libc::c_void,
    pub refchg: Option::<unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct chunk_file_view {
    pub mptr: *mut libc::c_char,
    pub mlen: off_t,
    pub foff: off_t,
    pub refcnt: libc::c_int,
}
pub type C2RustUnnamed_2 = libc::c_uint;
pub const FILE_CHUNK: C2RustUnnamed_2 = 1;
pub const MEM_CHUNK: C2RustUnnamed_2 = 0;
pub type http_method_t = libc::c_int;
pub const HTTP_METHOD_VERSION_CONTROL: http_method_t = 38;
pub const HTTP_METHOD_UPDATEREDIRECTREF: http_method_t = 37;
pub const HTTP_METHOD_UPDATE: http_method_t = 36;
pub const HTTP_METHOD_UNLOCK: http_method_t = 35;
pub const HTTP_METHOD_UNLINK: http_method_t = 34;
pub const HTTP_METHOD_UNCHECKOUT: http_method_t = 33;
pub const HTTP_METHOD_UNBIND: http_method_t = 32;
pub const HTTP_METHOD_SEARCH: http_method_t = 31;
pub const HTTP_METHOD_REPORT: http_method_t = 30;
pub const HTTP_METHOD_REBIND: http_method_t = 29;
pub const HTTP_METHOD_PROPPATCH: http_method_t = 28;
pub const HTTP_METHOD_PROPFIND: http_method_t = 27;
pub const HTTP_METHOD_PATCH: http_method_t = 26;
pub const HTTP_METHOD_ORDERPATCH: http_method_t = 25;
pub const HTTP_METHOD_MOVE: http_method_t = 24;
pub const HTTP_METHOD_MKWORKSPACE: http_method_t = 23;
pub const HTTP_METHOD_MKREDIRECTREF: http_method_t = 22;
pub const HTTP_METHOD_MKCOL: http_method_t = 21;
pub const HTTP_METHOD_MKCALENDAR: http_method_t = 20;
pub const HTTP_METHOD_MKACTIVITY: http_method_t = 19;
pub const HTTP_METHOD_MERGE: http_method_t = 18;
pub const HTTP_METHOD_LOCK: http_method_t = 17;
pub const HTTP_METHOD_LINK: http_method_t = 16;
pub const HTTP_METHOD_LABEL: http_method_t = 15;
pub const HTTP_METHOD_COPY: http_method_t = 14;
pub const HTTP_METHOD_CHECKOUT: http_method_t = 13;
pub const HTTP_METHOD_CHECKIN: http_method_t = 12;
pub const HTTP_METHOD_BIND: http_method_t = 11;
pub const HTTP_METHOD_BASELINE_CONTROL: http_method_t = 10;
pub const HTTP_METHOD_ACL: http_method_t = 9;
pub const HTTP_METHOD_TRACE: http_method_t = 8;
pub const HTTP_METHOD_OPTIONS: http_method_t = 7;
pub const HTTP_METHOD_CONNECT: http_method_t = 6;
pub const HTTP_METHOD_DELETE: http_method_t = 5;
pub const HTTP_METHOD_PUT: http_method_t = 4;
pub const HTTP_METHOD_POST: http_method_t = 3;
pub const HTTP_METHOD_QUERY: http_method_t = 2;
pub const HTTP_METHOD_HEAD: http_method_t = 1;
pub const HTTP_METHOD_GET: http_method_t = 0;
pub const HTTP_METHOD_UNSET: http_method_t = -1;
pub const HTTP_METHOD_PRI: http_method_t = -2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct response_dechunk {
    pub gw_chunked: off_t,
    pub b: buffer,
    pub done: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct physical {
    pub path: buffer,
    pub basedir: buffer,
    pub doc_root: buffer,
    pub rel_path: buffer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct request_uri {
    pub scheme: buffer,
    pub authority: buffer,
    pub path: buffer,
    pub query: buffer,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct request_config {
    pub errh: *mut fdlog_st,
    pub http_parseopts: libc::c_uint,
    pub max_request_field_size: uint32_t,
    pub mimetypes: *const array,
    pub document_root: *const buffer,
    pub server_name: *const buffer,
    pub server_tag: *const buffer,
    pub max_request_size: libc::c_uint,
    pub max_keep_alive_requests: libc::c_ushort,
    pub max_keep_alive_idle: libc::c_ushort,
    pub max_read_idle: libc::c_ushort,
    pub max_write_idle: libc::c_ushort,
    pub stream_request_body: libc::c_ushort,
    pub stream_response_body: libc::c_ushort,
    #[bitfield(name = "high_precision_timestamps", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "allow_http11", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "range_requests", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "follow_symlink", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "etag_flags", ty = "libc::c_uint", bits = "4..=6")]
    #[bitfield(name = "use_xattr", ty = "libc::c_uint", bits = "7..=7")]
    #[bitfield(name = "force_lowercase_filenames", ty = "libc::c_uint", bits = "8..=9")]
    #[bitfield(name = "error_intercept", ty = "libc::c_uint", bits = "10..=10")]
    #[bitfield(name = "h2proto", ty = "libc::c_uint", bits = "11..=12")]
    #[bitfield(name = "http_pathinfo", ty = "libc::c_uint", bits = "13..=13")]
    #[bitfield(name = "http_dummy", ty = "libc::c_uint", bits = "14..=15")]
    #[bitfield(name = "log_request_handling", ty = "libc::c_uint", bits = "16..=16")]
    #[bitfield(name = "log_state_handling", ty = "libc::c_uint", bits = "17..=17")]
    #[bitfield(name = "log_condition_handling", ty = "libc::c_uint", bits = "18..=18")]
    #[bitfield(name = "log_response_header", ty = "libc::c_uint", bits = "19..=19")]
    #[bitfield(name = "log_request_header", ty = "libc::c_uint", bits = "20..=20")]
    #[bitfield(name = "log_request_header_on_error", ty = "libc::c_uint", bits = "21..=21")]
    #[bitfield(name = "log_file_not_found", ty = "libc::c_uint", bits = "22..=22")]
    #[bitfield(name = "log_timeouts", ty = "libc::c_uint", bits = "23..=23")]
    pub high_precision_timestamps_allow_http11_range_requests_follow_symlink_etag_flags_use_xattr_force_lowercase_filenames_error_intercept_h2proto_http_pathinfo_http_dummy_log_request_handling_log_state_handling_log_condition_handling_log_response_header_log_request_header_log_request_header_on_error_log_file_not_found_log_timeouts: [u8; 3],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub bytes_per_second: libc::c_uint,
    pub global_bytes_per_second: libc::c_uint,
    pub global_bytes_per_second_cnt_ptr: *mut off_t,
    pub error_handler: *const buffer,
    pub error_handler_404: *const buffer,
    pub errorfile_prefix: *const buffer,
    pub serrh: *mut fdlog_st,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fdlog_st {
    pub mode: C2RustUnnamed_3,
    pub fd: libc::c_int,
    pub b: buffer,
    pub fn_0: *const libc::c_char,
}
pub type C2RustUnnamed_3 = libc::c_uint;
pub const FDLOG_PIPE: C2RustUnnamed_3 = 3;
pub const FDLOG_SYSLOG: C2RustUnnamed_3 = 2;
pub const FDLOG_FD: C2RustUnnamed_3 = 1;
pub const FDLOG_FILE: C2RustUnnamed_3 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cond_match_t {
    pub comp_value: *const buffer,
    pub match_data: *mut pcre2_real_match_data_8,
    pub captures: libc::c_int,
    pub matches: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cond_cache_t {
    pub result: int8_t,
    pub local_result: int8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct connection {
    pub request: request_st,
    pub hx: *mut hxcon,
    pub fd: libc::c_int,
    pub fdn: *mut fdnode,
    pub jqnext: *mut connection,
    pub is_readable: libc::c_schar,
    pub is_writable: libc::c_schar,
    pub is_ssl_sock: libc::c_char,
    pub traffic_limit_reached: libc::c_char,
    pub revents_err: uint16_t,
    pub proto_default_port: uint16_t,
    pub write_queue: *mut chunkqueue,
    pub read_queue: *mut chunkqueue,
    pub bytes_written_cur_second: off_t,
    pub network_write: Option::<
        unsafe extern "C" fn(*mut connection, *mut chunkqueue, off_t) -> libc::c_int,
    >,
    pub network_read: Option::<
        unsafe extern "C" fn(*mut connection, *mut chunkqueue, off_t) -> libc::c_int,
    >,
    pub reqbody_read: Option::<unsafe extern "C" fn(*mut request_st) -> handler_t>,
    pub fn_0: *const http_dispatch,
    pub srv: *mut server,
    pub plugin_slots: *mut libc::c_void,
    pub plugin_ctx: *mut *mut libc::c_void,
    pub config_data_base: *mut libc::c_void,
    pub dst_addr: sock_addr,
    pub dst_addr_buf: buffer,
    pub srv_socket: *const server_socket,
    pub read_idle_ts: unix_time64_t,
    pub close_timeout_ts: unix_time64_t,
    pub write_request_ts: unix_time64_t,
    pub connection_start: unix_time64_t,
    pub request_count: uint32_t,
    pub keep_alive_idle: libc::c_int,
    pub next: *mut connection,
    pub prev: *mut connection,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hxcon {
    pub r: [*mut request_st; 8],
    pub rused: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plugin {
    pub data: *mut libc::c_void,
    pub handle_uri_raw: Option::<
        unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    >,
    pub handle_uri_clean: Option::<
        unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    >,
    pub handle_docroot: Option::<
        unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    >,
    pub handle_physical: Option::<
        unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    >,
    pub handle_request_env: Option::<
        unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    >,
    pub handle_request_done: Option::<
        unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    >,
    pub handle_subrequest_start: Option::<
        unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    >,
    pub handle_subrequest: Option::<
        unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    >,
    pub handle_response_start: Option::<
        unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    >,
    pub handle_request_reset: Option::<
        unsafe extern "C" fn(*mut request_st, *mut libc::c_void) -> handler_t,
    >,
    pub handle_connection_accept: Option::<
        unsafe extern "C" fn(*mut connection, *mut libc::c_void) -> handler_t,
    >,
    pub handle_connection_shut_wr: Option::<
        unsafe extern "C" fn(*mut connection, *mut libc::c_void) -> handler_t,
    >,
    pub handle_connection_close: Option::<
        unsafe extern "C" fn(*mut connection, *mut libc::c_void) -> handler_t,
    >,
    pub handle_trigger: Option::<
        unsafe extern "C" fn(*mut server, *mut libc::c_void) -> handler_t,
    >,
    pub handle_sighup: Option::<
        unsafe extern "C" fn(*mut server, *mut libc::c_void) -> handler_t,
    >,
    pub handle_waitpid: Option::<
        unsafe extern "C" fn(
            *mut server,
            *mut libc::c_void,
            pid_t,
            libc::c_int,
        ) -> handler_t,
    >,
    pub init: Option::<unsafe extern "C" fn() -> *mut libc::c_void>,
    pub priv_defaults: Option::<
        unsafe extern "C" fn(*mut server, *mut libc::c_void) -> handler_t,
    >,
    pub set_defaults: Option::<
        unsafe extern "C" fn(*mut server, *mut libc::c_void) -> handler_t,
    >,
    pub worker_init: Option::<
        unsafe extern "C" fn(*mut server, *mut libc::c_void) -> handler_t,
    >,
    pub cleanup: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub name: *const libc::c_char,
    pub version: size_t,
    pub lib: *mut libc::c_void,
}
pub type http_version_t = libc::c_int;
pub const HTTP_VERSION_3: http_version_t = 3;
pub const HTTP_VERSION_2: http_version_t = 2;
pub const HTTP_VERSION_1_1: http_version_t = 1;
pub const HTTP_VERSION_1_0: http_version_t = 0;
pub const HTTP_VERSION_UNSET: http_version_t = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub h2: C2RustUnnamed_6,
    pub h1: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub bytes_written_ckpt: off_t,
    pub bytes_read_ckpt: off_t,
    pub te_chunked: off_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub state: uint32_t,
    pub id: uint32_t,
    pub rwin: int32_t,
    pub swin: int32_t,
    pub rwin_fudge: int16_t,
    pub prio: uint8_t,
}
pub type request_state_t = libc::c_uint;
pub const CON_STATE_CLOSE: request_state_t = 10;
pub const CON_STATE_ERROR: request_state_t = 9;
pub const CON_STATE_RESPONSE_END: request_state_t = 8;
pub const CON_STATE_WRITE: request_state_t = 7;
pub const CON_STATE_RESPONSE_START: request_state_t = 6;
pub const CON_STATE_HANDLE_REQUEST: request_state_t = 5;
pub const CON_STATE_READ_POST: request_state_t = 4;
pub const CON_STATE_REQUEST_END: request_state_t = 3;
pub const CON_STATE_READ: request_state_t = 2;
pub const CON_STATE_REQUEST_START: request_state_t = 1;
pub const CON_STATE_CONNECT: request_state_t = 0;
pub type log_error_st = fdlog_st;
pub type errno_t = libc::c_int;
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
pub struct data_array {
    pub key: buffer,
    pub fn_0: *const data_methods,
    pub type_0: data_type_t,
    pub value: array,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct data_integer {
    pub key: buffer,
    pub fn_0: *const data_methods,
    pub type_0: data_type_t,
    pub value: libc::c_int,
}
pub type burl_opts_e = libc::c_uint;
pub const HTTP_PARSEOPT_METHOD_GET_BODY: burl_opts_e = 32768;
pub const HTTP_PARSEOPT_URL_NORMALIZE_INVALID_UTF8_REJECT: burl_opts_e = 8192;
pub const HTTP_PARSEOPT_URL_NORMALIZE_QUERY_20_PLUS: burl_opts_e = 4096;
pub const HTTP_PARSEOPT_URL_NORMALIZE_PATH_DOTSEG_REJECT: burl_opts_e = 2048;
pub const HTTP_PARSEOPT_URL_NORMALIZE_PATH_DOTSEG_REMOVE: burl_opts_e = 1024;
pub const HTTP_PARSEOPT_URL_NORMALIZE_PATH_2F_REJECT: burl_opts_e = 512;
pub const HTTP_PARSEOPT_URL_NORMALIZE_PATH_2F_DECODE: burl_opts_e = 256;
pub const HTTP_PARSEOPT_URL_NORMALIZE_PATH_BACKSLASH_TRANS: burl_opts_e = 128;
pub const HTTP_PARSEOPT_URL_NORMALIZE_CTRLS_REJECT: burl_opts_e = 64;
pub const HTTP_PARSEOPT_URL_NORMALIZE_REQUIRED: burl_opts_e = 32;
pub const HTTP_PARSEOPT_URL_NORMALIZE_UNRESERVED: burl_opts_e = 16;
pub const HTTP_PARSEOPT_URL_NORMALIZE: burl_opts_e = 8;
pub const HTTP_PARSEOPT_HOST_NORMALIZE: burl_opts_e = 4;
pub const HTTP_PARSEOPT_HOST_STRICT: burl_opts_e = 2;
pub const HTTP_PARSEOPT_HEADER_STRICT: burl_opts_e = 1;
pub type C2RustUnnamed_7 = libc::c_uint;
pub const ETAG_USE_SIZE: C2RustUnnamed_7 = 4;
pub const ETAG_USE_MTIME: C2RustUnnamed_7 = 2;
pub const ETAG_USE_INODE: C2RustUnnamed_7 = 1;
pub type config_cond_t = libc::c_uint;
pub const CONFIG_COND_ELSE: config_cond_t = 7;
pub const CONFIG_COND_SUFFIX: config_cond_t = 6;
pub const CONFIG_COND_PREFIX: config_cond_t = 5;
pub const CONFIG_COND_NOMATCH: config_cond_t = 4;
pub const CONFIG_COND_NE: config_cond_t = 3;
pub const CONFIG_COND_MATCH: config_cond_t = 2;
pub const CONFIG_COND_EQ: config_cond_t = 1;
pub const CONFIG_COND_UNSET: config_cond_t = 0;
pub type comp_key_t = libc::c_uint;
pub const COMP_LAST_ELEMENT: comp_key_t = 13;
pub const COMP_HTTP_REQUEST_HEADER: comp_key_t = 12;
pub const COMP_HTTP_REQUEST_METHOD: comp_key_t = 11;
pub const COMP_HTTP_SCHEME: comp_key_t = 10;
pub const COMP_HTTP_QUERY_STRING: comp_key_t = 9;
pub const COMP_HTTP_REMOTE_IP: comp_key_t = 8;
pub const COMP_HTTP_COOKIE: comp_key_t = 7;
pub const COMP_HTTP_LANGUAGE: comp_key_t = 6;
pub const COMP_HTTP_USER_AGENT: comp_key_t = 5;
pub const COMP_HTTP_REFERER: comp_key_t = 4;
pub const COMP_HTTP_HOST: comp_key_t = 3;
pub const COMP_HTTP_URL: comp_key_t = 2;
pub const COMP_SERVER_SOCKET: comp_key_t = 1;
pub const COMP_UNSET: comp_key_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct data_config {
    pub key: buffer,
    pub fn_0: *const data_methods,
    pub type_0: data_type_t,
    pub context_ndx: libc::c_int,
    pub comp: comp_key_t,
    pub cond: config_cond_t,
    pub parent: *mut data_config,
    pub prev: *mut data_config,
    pub next: *mut data_config,
    pub string: buffer,
    pub code: *mut libc::c_void,
    pub match_data: *mut pcre2_real_match_data_8,
    pub capture_idx: libc::c_int,
    pub ext: libc::c_int,
    pub comp_tag: buffer,
    pub comp_key: *const libc::c_char,
    pub children: data_config_list,
    pub value: *mut array,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct data_config_list {
    pub data: *mut *mut data_config,
    pub used: uint32_t,
    pub size: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config_data_base {
    pub id: libc::c_int,
    pub nconfig: libc::c_int,
    pub cvlist: *mut config_plugin_value_t,
    pub self_0: *mut plugin,
    pub defaults: request_config,
}
pub type config_plugin_value_t = config_plugin_value;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config_plugin_value {
    pub k_id: libc::c_int,
    pub vtype: config_values_type_t,
    pub v: v_u,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union v_u {
    pub v: *mut libc::c_void,
    pub a: *const array,
    pub b: *const buffer,
    pub s: *const libc::c_char,
    pub u: libc::c_uint,
    pub shrt: libc::c_ushort,
    pub d: libc::c_double,
    pub o: off_t,
    pub u2: [uint32_t; 2],
}
pub type config_values_type_t = libc::c_uint;
pub const T_CONFIG_UNSUPPORTED: config_values_type_t = 12;
pub const T_CONFIG_DEPRECATED: config_values_type_t = 11;
pub const T_CONFIG_LOCAL: config_values_type_t = 10;
pub const T_CONFIG_ARRAY_VLIST: config_values_type_t = 9;
pub const T_CONFIG_ARRAY_KVSTRING: config_values_type_t = 8;
pub const T_CONFIG_ARRAY_KVARRAY: config_values_type_t = 7;
pub const T_CONFIG_ARRAY_KVANY: config_values_type_t = 6;
pub const T_CONFIG_ARRAY: config_values_type_t = 5;
pub const T_CONFIG_BOOL: config_values_type_t = 4;
pub const T_CONFIG_INT: config_values_type_t = 3;
pub const T_CONFIG_SHORT: config_values_type_t = 2;
pub const T_CONFIG_STRING: config_values_type_t = 1;
pub const T_CONFIG_UNSET: config_values_type_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config_plugin_keys_t {
    pub k: *const libc::c_char,
    pub klen: uint8_t,
    pub ktype: uint8_t,
    pub scope: uint8_t,
}
pub const T_CONFIG_SCOPE_UNSET: C2RustUnnamed_8 = 0;
pub const T_CONFIG_SCOPE_CONNECTION: C2RustUnnamed_8 = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plugin_data_base {
    pub id: libc::c_int,
    pub nconfig: libc::c_int,
    pub cvlist: *mut config_plugin_value_t,
    pub self_0: *mut plugin,
}
pub const T_CONFIG_SCOPE_SERVER: C2RustUnnamed_8 = 1;
pub const T_CONFIG_SCOPE_SOCKET: C2RustUnnamed_8 = 3;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config_t {
    pub srv: *mut server,
    pub ok: libc::c_int,
    pub all_configs: *mut array,
    pub configs_stack: data_config_list,
    pub current: *mut data_config,
    pub basedir: *mut buffer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tokenizer_t {
    pub offset: libc::c_int,
    pub size: libc::c_int,
    pub input: *const libc::c_char,
    pub token: *mut buffer,
    pub in_key: libc::c_char,
    pub parens: libc::c_char,
    pub in_cond: libc::c_char,
    pub simulate_eol: libc::c_char,
    pub tid: libc::c_int,
    pub line_pos: libc::c_int,
    pub line: libc::c_int,
    pub source: *const libc::c_char,
    pub errh: *mut log_error_st,
}
pub type pcre2_match_data_8 = pcre2_real_match_data_8;
pub type pcre2_general_context_8 = pcre2_real_general_context_8;
pub type C2RustUnnamed_8 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glob_t {
    pub gl_pathc: __size_t,
    pub gl_pathv: *mut *mut libc::c_char,
    pub gl_offs: __size_t,
    pub gl_flags: libc::c_int,
    pub gl_closedir: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub gl_readdir: Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut dirent>,
    pub gl_opendir: Option::<
        unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_void,
    >,
    pub gl_lstat: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut stat) -> libc::c_int,
    >,
    pub gl_stat: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut stat) -> libc::c_int,
    >,
}
pub type __size_t = libc::c_ulong;
#[inline]
unsafe extern "C" fn buffer_clear(mut b: *mut buffer) {
    (*b).used = 0 as libc::c_int as uint32_t;
}
#[inline]
unsafe extern "C" fn light_isdigit(mut c: libc::c_int) -> libc::c_int {
    return ((c as uint32_t).wrapping_sub('0' as i32 as libc::c_uint)
        <= ('9' as i32 - '0' as i32) as libc::c_uint) as libc::c_int;
}
#[inline]
unsafe extern "C" fn light_isalpha(mut c: libc::c_int) -> libc::c_int {
    return ((c as uint32_t | 0x20 as libc::c_int as libc::c_uint)
        .wrapping_sub('a' as i32 as libc::c_uint)
        <= ('z' as i32 - 'a' as i32) as libc::c_uint) as libc::c_int;
}
#[inline]
unsafe extern "C" fn light_isalnum(mut c: libc::c_int) -> libc::c_int {
    return (light_isdigit(c) != 0 || light_isalpha(c) != 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn buffer_is_unset(mut b: *const buffer) -> libc::c_int {
    return (0 as libc::c_int as libc::c_uint == (*b).used) as libc::c_int;
}
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
unsafe extern "C" fn buffer_append_char(mut b: *mut buffer, mut c: libc::c_char) {
    *buffer_extend(b, 1 as libc::c_int as size_t) = c;
}
#[inline]
unsafe extern "C" fn ck_memzero(mut s: *mut libc::c_void, mut n: rsize_t) -> errno_t {
    return ck_memclear_s(s, n, n);
}
#[inline]
unsafe extern "C" fn array_set_key_value(
    a: *mut array,
    k: *const libc::c_char,
    klen: uint32_t,
    v: *const libc::c_char,
    vlen: uint32_t,
) {
    buffer_copy_string_len(array_get_buf_ptr(a, k, klen), v, vlen as size_t);
}
unsafe extern "C" fn config_free_config(p_d: *mut libc::c_void) {
    let p: *mut plugin_data_base = p_d as *mut plugin_data_base;
    if p.is_null() {
        return;
    }
    if ((*p).cvlist).is_null() {
        free(p as *mut libc::c_void);
        return;
    }
    let mut i: libc::c_int = ((*((*p).cvlist).offset(0 as libc::c_int as isize))
        .v
        .u2[1 as libc::c_int as usize] == 0) as libc::c_int;
    let mut used: libc::c_int = (*p).nconfig;
    while i < used {
        let mut cpv: *mut config_plugin_value_t = ((*p).cvlist)
            .offset(
                (*((*p).cvlist).offset(i as isize)).v.u2[0 as libc::c_int as usize]
                    as isize,
            );
        while -(1 as libc::c_int) != (*cpv).k_id {
            match (*cpv).k_id {
                18 => {
                    if (*cpv).vtype as libc::c_uint
                        == T_CONFIG_LOCAL as libc::c_int as libc::c_uint
                    {
                        free((*cpv).v.v);
                    }
                }
                _ => {}
            }
            cpv = cpv.offset(1);
            cpv;
        }
        i += 1;
        i;
    }
    free((*p).cvlist as *mut libc::c_void);
    free(p as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn config_reset_config_bytes_sec(p_d: *mut libc::c_void) {
    let p: *mut plugin_data_base = p_d as *mut plugin_data_base;
    if ((*p).cvlist).is_null() {
        return;
    }
    let mut i: libc::c_int = ((*((*p).cvlist).offset(0 as libc::c_int as isize))
        .v
        .u2[1 as libc::c_int as usize] == 0) as libc::c_int;
    let mut used: libc::c_int = (*p).nconfig;
    while i < used {
        let mut cpv: *mut config_plugin_value_t = ((*p).cvlist)
            .offset(
                (*((*p).cvlist).offset(i as isize)).v.u2[0 as libc::c_int as usize]
                    as isize,
            );
        while -(1 as libc::c_int) != (*cpv).k_id {
            match (*cpv).k_id {
                18 => {
                    if (*cpv).vtype as libc::c_uint
                        == T_CONFIG_LOCAL as libc::c_int as libc::c_uint
                    {
                        *((*cpv).v.v as *mut off_t)
                            .offset(
                                0 as libc::c_int as isize,
                            ) = 0 as libc::c_int as off_t;
                    }
                }
                _ => {}
            }
            cpv = cpv.offset(1);
            cpv;
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn config_merge_config_cpv(
    pconf: *mut request_config,
    cpv: *const config_plugin_value_t,
) {
    match (*cpv).k_id {
        0 => {
            (*pconf).document_root = (*cpv).v.b;
        }
        1 => {
            (*pconf).server_name = (*cpv).v.b;
        }
        2 => {
            (*pconf).server_tag = (*cpv).v.b;
        }
        3 => {
            (*pconf).max_request_size = (*cpv).v.u;
        }
        4 => {
            (*pconf).max_keep_alive_requests = (*cpv).v.shrt;
        }
        5 => {
            (*pconf).max_keep_alive_idle = (*cpv).v.shrt;
        }
        6 => {
            (*pconf).max_read_idle = (*cpv).v.shrt;
        }
        7 => {
            (*pconf).max_write_idle = (*cpv).v.shrt;
        }
        8 => {
            (*pconf).errorfile_prefix = (*cpv).v.b;
        }
        9 => {
            (*pconf).error_handler = (*cpv).v.b;
        }
        10 => {
            (*pconf).error_handler_404 = (*cpv).v.b;
        }
        11 => {
            (*pconf)
                .set_error_intercept(
                    (0 as libc::c_int as libc::c_uint != (*cpv).v.u) as libc::c_int
                        as libc::c_uint,
                );
        }
        12 => {
            (*pconf)
                .set_force_lowercase_filenames(
                    (0 as libc::c_int as libc::c_uint != (*cpv).v.u) as libc::c_int
                        as libc::c_uint,
                );
        }
        13 => {
            (*pconf)
                .set_follow_symlink(
                    (0 as libc::c_int as libc::c_uint != (*cpv).v.u) as libc::c_int
                        as libc::c_uint,
                );
        }
        14 => {
            (*pconf)
                .set_allow_http11(
                    (0 as libc::c_int as libc::c_uint != (*cpv).v.u) as libc::c_int
                        as libc::c_uint,
                );
        }
        15 => {
            (*pconf)
                .set_range_requests(
                    (0 as libc::c_int as libc::c_uint != (*cpv).v.u) as libc::c_int
                        as libc::c_uint,
                );
        }
        16 => {
            (*pconf).stream_request_body = (*cpv).v.shrt;
        }
        17 => {
            (*pconf).stream_response_body = (*cpv).v.shrt;
        }
        18 => {
            (*pconf)
                .global_bytes_per_second = *((*cpv).v.v as *mut off_t)
                .offset(1 as libc::c_int as isize) as libc::c_uint;
            (*pconf).global_bytes_per_second_cnt_ptr = (*cpv).v.v as *mut off_t;
        }
        19 => {
            (*pconf)
                .bytes_per_second = ((*cpv).v.shrt as libc::c_uint) << 10 as libc::c_int;
        }
        20 => {
            (*pconf).mimetypes = (*cpv).v.a;
        }
        21 => {
            (*pconf)
                .set_use_xattr(
                    (0 as libc::c_int as libc::c_uint != (*cpv).v.u) as libc::c_int
                        as libc::c_uint,
                );
        }
        22 => {
            if (*cpv).v.u != 0 {
                (*pconf)
                    .set_etag_flags(
                        (*pconf).etag_flags()
                            | ETAG_USE_INODE as libc::c_int as libc::c_uint,
                    );
            } else {
                (*pconf)
                    .set_etag_flags(
                        (*pconf).etag_flags()
                            & !(ETAG_USE_INODE as libc::c_int) as libc::c_uint,
                    );
            };
        }
        23 => {
            if (*cpv).v.u != 0 {
                (*pconf)
                    .set_etag_flags(
                        (*pconf).etag_flags()
                            | ETAG_USE_MTIME as libc::c_int as libc::c_uint,
                    );
            } else {
                (*pconf)
                    .set_etag_flags(
                        (*pconf).etag_flags()
                            & !(ETAG_USE_MTIME as libc::c_int) as libc::c_uint,
                    );
            };
        }
        24 => {
            if (*cpv).v.u != 0 {
                (*pconf)
                    .set_etag_flags(
                        (*pconf).etag_flags()
                            | ETAG_USE_SIZE as libc::c_int as libc::c_uint,
                    );
            } else {
                (*pconf)
                    .set_etag_flags(
                        (*pconf).etag_flags()
                            & !(ETAG_USE_SIZE as libc::c_int) as libc::c_uint,
                    );
            };
        }
        25 => {
            (*pconf)
                .set_log_condition_handling(
                    (0 as libc::c_int as libc::c_uint != (*cpv).v.u) as libc::c_int
                        as libc::c_uint,
                );
        }
        26 => {
            (*pconf)
                .set_log_file_not_found(
                    (0 as libc::c_int as libc::c_uint != (*cpv).v.u) as libc::c_int
                        as libc::c_uint,
                );
        }
        27 => {
            (*pconf)
                .set_log_request_handling(
                    (0 as libc::c_int as libc::c_uint != (*cpv).v.u) as libc::c_int
                        as libc::c_uint,
                );
        }
        28 => {
            (*pconf)
                .set_log_request_header(
                    (0 as libc::c_int as libc::c_uint != (*cpv).v.u) as libc::c_int
                        as libc::c_uint,
                );
        }
        29 => {
            (*pconf)
                .set_log_response_header(
                    (0 as libc::c_int as libc::c_uint != (*cpv).v.u) as libc::c_int
                        as libc::c_uint,
                );
        }
        30 => {
            (*pconf)
                .set_log_timeouts(
                    (0 as libc::c_int as libc::c_uint != (*cpv).v.u) as libc::c_int
                        as libc::c_uint,
                );
        }
        31 => {
            (*pconf)
                .set_log_state_handling(
                    (0 as libc::c_int as libc::c_uint != (*cpv).v.u) as libc::c_int
                        as libc::c_uint,
                );
        }
        32 => {
            if (*cpv).vtype as libc::c_uint
                == T_CONFIG_LOCAL as libc::c_int as libc::c_uint
            {
                (*pconf).errh = (*cpv).v.v as *mut fdlog_st;
            }
        }
        33 => {
            if (*cpv).vtype as libc::c_uint
                == T_CONFIG_LOCAL as libc::c_int as libc::c_uint
            {
                (*pconf).serrh = (*cpv).v.v as *mut fdlog_st;
            }
        }
        _ => return,
    };
}
unsafe extern "C" fn config_merge_config(
    pconf: *mut request_config,
    mut cpv: *const config_plugin_value_t,
) {
    loop {
        config_merge_config_cpv(pconf, cpv);
        cpv = cpv.offset(1);
        if !((*cpv).k_id != -(1 as libc::c_int)) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn config_patch_config(r: *mut request_st) {
    let p: *mut config_data_base = (*(*r).con).config_data_base as *mut config_data_base;
    let mut i: libc::c_int = 1 as libc::c_int;
    let mut used: libc::c_int = (*p).nconfig;
    while i < used {
        if config_check_cond(
            r,
            (*((*p).cvlist).offset(i as isize)).k_id as uint32_t as libc::c_int,
        ) != 0
        {
            config_merge_config(
                &mut (*r).conf,
                ((*p).cvlist)
                    .offset(
                        (*((*p).cvlist).offset(i as isize))
                            .v
                            .u2[0 as libc::c_int as usize] as isize,
                    ),
            );
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn config_burl_normalize_cond(srv: *mut server) {
    let tb: *mut buffer = (*srv).tmp_buf;
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*(*srv).config_context).used {
        let config: *mut data_config = *((*(*srv).config_context).data)
            .offset(i as isize) as *mut data_config;
        if !(COMP_HTTP_QUERY_STRING as libc::c_int as libc::c_uint
            != (*config).comp as libc::c_uint)
        {
            match (*config).cond as libc::c_uint {
                3 | 1 | 5 | 6 => {
                    pcre_keyvalue_burl_normalize_key(&mut (*config).string, tb);
                }
                4 | 2 => {
                    pcre_keyvalue_burl_normalize_key(&mut (*config).string, tb);
                }
                _ => {}
            }
        }
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn config_pcre_keyvalue(srv: *mut server) -> libc::c_int {
    let pcre_jit: libc::c_int = config_feature_bool(
        srv,
        b"server.pcre_jit\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*(*srv).config_context).used {
        let dc: *mut data_config = *((*(*srv).config_context).data).offset(i as isize)
            as *mut data_config;
        if !((*dc).cond as libc::c_uint
            != CONFIG_COND_NOMATCH as libc::c_int as libc::c_uint
            && (*dc).cond as libc::c_uint
                != CONFIG_COND_MATCH as libc::c_int as libc::c_uint)
        {
            if data_config_pcre_compile(dc, pcre_jit, (*srv).errh) == 0 {
                return 0 as libc::c_int;
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn config_check_module_duplicates(mut srv: *mut server) {
    let mut dups: libc::c_int = 0 as libc::c_int;
    let data: *mut *mut data_string = (*(*srv).srvconf.modules).data
        as *mut *mut data_string;
    let used: uint32_t = (*(*srv).srvconf.modules).used;
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < used {
        let m: *const buffer = &mut (**data.offset(i as isize)).value;
        let mut j: uint32_t = i.wrapping_add(1 as libc::c_int as libc::c_uint);
        while j < used {
            if buffer_is_equal(m, &mut (**data.offset(j as isize)).value) != 0 {
                dups += 1;
                dups;
                break;
            } else {
                j = j.wrapping_add(1);
                j;
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    if dups == 0 {
        return;
    }
    let modules: *mut array = array_init(used.wrapping_sub(dups as libc::c_uint));
    let mut i_0: uint32_t = 0 as libc::c_int as uint32_t;
    while i_0 < used {
        let m_0: *const buffer = &mut (**data.offset(i_0 as isize)).value;
        let mut j_0: uint32_t = 0;
        j_0 = 0 as libc::c_int as uint32_t;
        while j_0 < (*modules).used {
            let mut n: *mut buffer = &mut (*(*((*modules).data).offset(j_0 as isize)
                as *mut data_string))
                .value;
            if buffer_is_equal(m_0, n) != 0 {
                break;
            }
            j_0 = j_0.wrapping_add(1);
            j_0;
        }
        if j_0 == (*modules).used {
            array_insert_value(modules, (*m_0).ptr, buffer_clen(m_0));
        }
        i_0 = i_0.wrapping_add(1);
        i_0;
    }
    array_free((*srv).srvconf.modules);
    (*srv).srvconf.modules = modules;
}
#[inline(never)]
unsafe extern "C" fn config_has_opt_enabled(
    srv: *const server,
    opt: *const libc::c_char,
    olen: uint32_t,
) -> libc::c_int {
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*(*srv).config_context).used {
        let config: *const data_config = *((*(*srv).config_context).data)
            .offset(i as isize) as *const data_config;
        let du: *const data_unset = array_get_data_unset((*config).value, opt, olen);
        if !du.is_null() {
            if if (*du).type_0 as libc::c_uint
                == TYPE_ARRAY as libc::c_int as libc::c_uint
            {
                ((*(du as *mut data_array)).value.used
                    != 0 as libc::c_int as libc::c_uint) as libc::c_int
            } else {
                config_plugin_value_to_bool(du, 0 as libc::c_int)
            } != 0
            {
                return 1 as libc::c_int;
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int;
}
#[inline(never)]
unsafe extern "C" fn config_has_opt_and_value(
    srv: *const server,
    opt: *const libc::c_char,
    olen: uint32_t,
    v: *const libc::c_char,
    vlen: uint32_t,
) -> *const libc::c_char {
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*(*srv).config_context).used {
        let config: *const data_config = *((*(*srv).config_context).data)
            .offset(i as isize) as *const data_config;
        let ds: *const data_string = array_get_element_klen((*config).value, opt, olen)
            as *mut data_string;
        if !ds.is_null()
            && (*ds).type_0 as libc::c_uint == TYPE_STRING as libc::c_int as libc::c_uint
            && buffer_eq_slen(&(*ds).value, v, vlen as size_t) != 0
        {
            return v;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as *const libc::c_char;
}
#[inline(never)]
unsafe extern "C" fn config_compat_module_remove(
    mut srv: *mut server,
    mut module: *const libc::c_char,
    mut len: uint32_t,
) {
    let mut modules: *mut array = array_init((*(*srv).srvconf.modules).used);
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*(*srv).srvconf.modules).used {
        let mut ds: *const data_string = *((*(*srv).srvconf.modules).data)
            .offset(i as isize) as *mut data_string;
        if buffer_eq_slen(&(*ds).value, module, len as size_t) == 0 {
            array_insert_value(modules, (*ds).value.ptr, buffer_clen(&(*ds).value));
        }
        i = i.wrapping_add(1);
        i;
    }
    array_free((*srv).srvconf.modules);
    (*srv).srvconf.modules = modules;
}
#[inline(never)]
unsafe extern "C" fn config_compat_module_prepend(
    mut srv: *mut server,
    mut module: *const libc::c_char,
    mut len: uint32_t,
) {
    let mut modules: *mut array = array_init(
        ((*(*srv).srvconf.modules).used).wrapping_add(4 as libc::c_int as libc::c_uint),
    );
    array_insert_value(modules, module, len);
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*(*srv).srvconf.modules).used {
        let mut ds: *mut data_string = *((*(*srv).srvconf.modules).data)
            .offset(i as isize) as *mut data_string;
        array_insert_value(modules, (*ds).value.ptr, buffer_clen(&mut (*ds).value));
        i = i.wrapping_add(1);
        i;
    }
    array_free((*srv).srvconf.modules);
    (*srv).srvconf.modules = modules;
}
unsafe extern "C" fn config_warn_authn_module(
    mut srv: *mut server,
    mut module: *const libc::c_char,
    mut len: uint32_t,
    mut v: *const libc::c_char,
) {
    let tb: *mut buffer = (*srv).tmp_buf;
    buffer_copy_string_len(
        tb,
        b"mod_authn_\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    buffer_append_string_len(tb, module, len as size_t);
    array_insert_value((*srv).srvconf.modules, (*tb).ptr, buffer_clen(tb));
    log_pri(
        (*srv).errh,
        b"configfile.c\0" as *const u8 as *const libc::c_char,
        387 as libc::c_int as libc::c_uint,
        4 as libc::c_int,
        b"Warning: please add \"mod_authn_%s\" to server.modules list in lighttpd.conf.  A future release of lighttpd 1.4.x will not automatically load mod_authn_%s and lighttpd will fail to start up since your lighttpd.conf uses auth.backend = \"%s\".\0"
            as *const u8 as *const libc::c_char,
        module,
        module,
        v,
    );
}
unsafe extern "C" fn config_compat_module_load(mut srv: *mut server) {
    let mut prepend_mod_indexfile: libc::c_int = 1 as libc::c_int;
    let mut append_mod_dirlisting: libc::c_int = 1 as libc::c_int;
    let mut append_mod_staticfile: libc::c_int = 1 as libc::c_int;
    let mut append_mod_authn_file: libc::c_int = 1 as libc::c_int;
    let mut append_mod_authn_ldap: libc::c_int = 1 as libc::c_int;
    let mut append_mod_openssl: libc::c_int = 1 as libc::c_int;
    let mut contains_mod_auth: libc::c_int = 0 as libc::c_int;
    let mut prepend_mod_auth: libc::c_int = 0 as libc::c_int;
    let mut prepend_mod_vhostdb: libc::c_int = 0 as libc::c_int;
    let mut dyn_name: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*(*srv).srvconf.modules).used {
        let mut m: *mut buffer = &mut (*(*((*(*srv).srvconf.modules).data)
            .offset(i as isize) as *mut data_string))
            .value;
        if buffer_eq_slen(
            m,
            b"mod_indexfile\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        ) != 0
        {
            prepend_mod_indexfile = 0 as libc::c_int;
        } else if buffer_eq_slen(
            m,
            b"mod_staticfile\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        ) != 0
        {
            append_mod_staticfile = 0 as libc::c_int;
        } else if buffer_eq_slen(
            m,
            b"mod_dirlisting\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        ) != 0
        {
            append_mod_dirlisting = 0 as libc::c_int;
        } else if buffer_eq_slen(
            m,
            b"mod_gnutls\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        ) != 0
        {
            append_mod_openssl = 0 as libc::c_int;
        } else if buffer_eq_slen(
            m,
            b"mod_mbedtls\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        ) != 0
        {
            append_mod_openssl = 0 as libc::c_int;
        } else if buffer_eq_slen(
            m,
            b"mod_nss\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        ) != 0
        {
            append_mod_openssl = 0 as libc::c_int;
        } else if buffer_eq_slen(
            m,
            b"mod_openssl\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        ) != 0
        {
            append_mod_openssl = 0 as libc::c_int;
        } else if buffer_eq_slen(
            m,
            b"mod_wolfssl\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        ) != 0
        {
            append_mod_openssl = 0 as libc::c_int;
        } else if 0 as libc::c_int
            == strncmp(
                (*m).ptr,
                b"mod_auth\0" as *const u8 as *const libc::c_char,
                (::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            )
        {
            if buffer_eq_slen(
                m,
                b"mod_auth\0" as *const u8 as *const libc::c_char,
                (::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            ) != 0
            {
                if contains_mod_auth == 0 {
                    contains_mod_auth = 1 as libc::c_int;
                    if !dyn_name.is_null() {
                        log_pri(
                            (*srv).errh,
                            b"configfile.c\0" as *const u8 as *const libc::c_char,
                            431 as libc::c_int as libc::c_uint,
                            4 as libc::c_int,
                            b"Warning: mod_auth should be listed in server.modules before dynamic backends such as %s\0"
                                as *const u8 as *const libc::c_char,
                            dyn_name,
                        );
                    }
                }
            } else if contains_mod_auth == 0 {
                prepend_mod_auth = 1 as libc::c_int;
            }
            if buffer_eq_slen(
                m,
                b"mod_authn_file\0" as *const u8 as *const libc::c_char,
                (::core::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            ) != 0
            {
                append_mod_authn_file = 0 as libc::c_int;
            } else if buffer_eq_slen(
                m,
                b"mod_authn_ldap\0" as *const u8 as *const libc::c_char,
                (::core::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            ) != 0
            {
                append_mod_authn_ldap = 0 as libc::c_int;
            }
        } else if 0 as libc::c_int
            == strncmp(
                (*m).ptr,
                b"mod_vhostdb\0" as *const u8 as *const libc::c_char,
                (::core::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            )
        {
            if buffer_eq_slen(
                m,
                b"mod_vhostdb\0" as *const u8 as *const libc::c_char,
                (::core::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            ) != 0
            {
                prepend_mod_vhostdb |= 2 as libc::c_int;
            } else if prepend_mod_vhostdb & 2 as libc::c_int == 0 {
                prepend_mod_vhostdb |= 1 as libc::c_int;
            }
        } else if 0 as libc::c_int
            == strncmp(
                (*m).ptr,
                b"mod_ajp13\0" as *const u8 as *const libc::c_char,
                (::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            )
            || 0 as libc::c_int
                == strncmp(
                    (*m).ptr,
                    b"mod_cgi\0" as *const u8 as *const libc::c_char,
                    (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                )
            || 0 as libc::c_int
                == strncmp(
                    (*m).ptr,
                    b"mod_fastcgi\0" as *const u8 as *const libc::c_char,
                    (::core::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                )
            || 0 as libc::c_int
                == strncmp(
                    (*m).ptr,
                    b"mod_proxy\0" as *const u8 as *const libc::c_char,
                    (::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                )
            || 0 as libc::c_int
                == strncmp(
                    (*m).ptr,
                    b"mod_scgi\0" as *const u8 as *const libc::c_char,
                    (::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                )
            || 0 as libc::c_int
                == strncmp(
                    (*m).ptr,
                    b"mod_sockproxy\0" as *const u8 as *const libc::c_char,
                    (::core::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                )
            || 0 as libc::c_int
                == strncmp(
                    (*m).ptr,
                    b"mod_wstunnel\0" as *const u8 as *const libc::c_char,
                    (::core::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                )
        {
            if dyn_name.is_null() {
                dyn_name = (*m).ptr;
            }
            if append_mod_staticfile == 0 {
                log_pri(
                    (*srv).errh,
                    b"configfile.c\0" as *const u8 as *const libc::c_char,
                    467 as libc::c_int as libc::c_uint,
                    4 as libc::c_int,
                    b"Warning: %s should be listed in server.modules before mod_staticfile\0"
                        as *const u8 as *const libc::c_char,
                    (*m).ptr,
                );
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    if config_has_opt_enabled(
        srv,
        b"index-file.names\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    ) == 0
        && config_has_opt_enabled(
            srv,
            b"server.indexfiles\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        ) == 0
    {
        if prepend_mod_indexfile == 0 {
            config_compat_module_remove(
                srv,
                b"mod_indexfile\0" as *const u8 as *const libc::c_char,
                (::core::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
            );
        }
        prepend_mod_indexfile = 0 as libc::c_int;
    }
    if config_has_opt_enabled(
        srv,
        b"dir-listing.activate\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    ) == 0
        && config_has_opt_enabled(
            srv,
            b"server.dir-listing\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        ) == 0
    {
        if append_mod_dirlisting == 0 {
            config_compat_module_remove(
                srv,
                b"mod_dirlisting\0" as *const u8 as *const libc::c_char,
                (::core::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
            );
        }
        append_mod_dirlisting = 0 as libc::c_int;
    }
    if prepend_mod_indexfile != 0 {
        config_compat_module_prepend(
            srv,
            b"mod_indexfile\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
    }
    if append_mod_dirlisting != 0 {
        array_insert_value(
            (*srv).srvconf.modules,
            b"mod_dirlisting\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
    }
    if append_mod_staticfile != 0 {
        array_insert_value(
            (*srv).srvconf.modules,
            b"mod_staticfile\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
    }
    append_mod_openssl != 0;
    if contains_mod_auth != 0 {
        if append_mod_authn_file != 0 {
            let mut v: *const libc::c_char = 0 as *const libc::c_char;
            v = config_has_opt_and_value(
                srv,
                b"auth.backend\0" as *const u8 as *const libc::c_char,
                (::core::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                b"htdigest\0" as *const u8 as *const libc::c_char,
                (::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
            );
            if !v.is_null()
                || {
                    v = config_has_opt_and_value(
                        srv,
                        b"auth.backend\0" as *const u8 as *const libc::c_char,
                        (::core::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint),
                        b"htpasswd\0" as *const u8 as *const libc::c_char,
                        (::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint),
                    );
                    !v.is_null()
                }
                || {
                    v = config_has_opt_and_value(
                        srv,
                        b"auth.backend\0" as *const u8 as *const libc::c_char,
                        (::core::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint),
                        b"plain\0" as *const u8 as *const libc::c_char,
                        (::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint),
                    );
                    !v.is_null()
                }
            {
                config_warn_authn_module(
                    srv,
                    b"file\0" as *const u8 as *const libc::c_char,
                    (::core::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint),
                    v,
                );
            }
        }
        append_mod_authn_ldap != 0;
    }
    if prepend_mod_auth != 0 {
        config_compat_module_prepend(
            srv,
            b"mod_auth\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
    }
    if prepend_mod_vhostdb & 1 as libc::c_int != 0 {
        config_compat_module_prepend(
            srv,
            b"mod_vhostdb\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
    }
}
unsafe extern "C" fn config_deprecate_module_compress(mut srv: *mut server) {
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*(*srv).srvconf.modules).used {
        let mut m: *mut buffer = &mut (*(*((*(*srv).srvconf.modules).data)
            .offset(i as isize) as *mut data_string))
            .value;
        if buffer_eq_slen(
            m,
            b"mod_compress\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        ) != 0
        {
            buffer_copy_string_len(
                m,
                b"mod_deflate\0" as *const u8 as *const libc::c_char,
                (::core::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
        }
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn config_http_parseopts(
    mut srv: *mut server,
    mut a: *const array,
) -> libc::c_int {
    let mut opts: libc::c_ushort = (*srv).srvconf.http_url_normalize;
    let mut decode_2f: uint8_t = 1 as libc::c_int as uint8_t;
    let mut url_normalize: uint8_t = 1 as libc::c_int as uint8_t;
    let mut rc: libc::c_int = 1 as libc::c_int;
    let mut current_block_30: u64;
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < (*a).used as libc::c_ulong {
        let ds: *const data_string = *((*a).data).offset(i as isize)
            as *const data_string;
        let mut k: *const buffer = &(*ds).key;
        let mut opt: libc::c_ushort = 0;
        let mut val: libc::c_int = config_plugin_value_to_bool(
            ds as *mut data_unset,
            2 as libc::c_int,
        );
        if 2 as libc::c_int == val {
            log_error(
                (*srv).errh,
                b"configfile.c\0" as *const u8 as *const libc::c_char,
                566 as libc::c_int as libc::c_uint,
                b"unrecognized value for server.http-parseopts: %s => %s (expect \"[enable|disable]\")\0"
                    as *const u8 as *const libc::c_char,
                (*k).ptr,
                (*ds).value.ptr,
            );
            rc = 0 as libc::c_int;
        }
        if buffer_eq_slen(
            k,
            b"url-normalize\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        ) != 0
        {
            opt = HTTP_PARSEOPT_URL_NORMALIZE as libc::c_int as libc::c_ushort;
            current_block_30 = 17184638872671510253;
        } else if buffer_eq_slen(
            k,
            b"url-normalize-unreserved\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        ) != 0
        {
            opt = HTTP_PARSEOPT_URL_NORMALIZE_UNRESERVED as libc::c_int
                as libc::c_ushort;
            current_block_30 = 17184638872671510253;
        } else if buffer_eq_slen(
            k,
            b"url-normalize-required\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        ) != 0
        {
            opt = HTTP_PARSEOPT_URL_NORMALIZE_REQUIRED as libc::c_int as libc::c_ushort;
            current_block_30 = 17184638872671510253;
        } else if buffer_eq_slen(
            k,
            b"url-ctrls-reject\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        ) != 0
        {
            opt = HTTP_PARSEOPT_URL_NORMALIZE_CTRLS_REJECT as libc::c_int
                as libc::c_ushort;
            current_block_30 = 17184638872671510253;
        } else if buffer_eq_slen(
            k,
            b"url-path-backslash-trans\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        ) != 0
        {
            opt = HTTP_PARSEOPT_URL_NORMALIZE_PATH_BACKSLASH_TRANS as libc::c_int
                as libc::c_ushort;
            current_block_30 = 17184638872671510253;
        } else if buffer_eq_slen(
            k,
            b"url-path-2f-decode\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        ) != 0
        {
            opt = HTTP_PARSEOPT_URL_NORMALIZE_PATH_2F_DECODE as libc::c_int
                as libc::c_ushort;
            current_block_30 = 17184638872671510253;
        } else if buffer_eq_slen(
            k,
            b"url-path-2f-reject\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        ) != 0
        {
            opt = HTTP_PARSEOPT_URL_NORMALIZE_PATH_2F_REJECT as libc::c_int
                as libc::c_ushort;
            current_block_30 = 17184638872671510253;
        } else if buffer_eq_slen(
            k,
            b"url-path-dotseg-remove\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        ) != 0
        {
            opt = HTTP_PARSEOPT_URL_NORMALIZE_PATH_DOTSEG_REMOVE as libc::c_int
                as libc::c_ushort;
            current_block_30 = 17184638872671510253;
        } else if buffer_eq_slen(
            k,
            b"url-path-dotseg-reject\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        ) != 0
        {
            opt = HTTP_PARSEOPT_URL_NORMALIZE_PATH_DOTSEG_REJECT as libc::c_int
                as libc::c_ushort;
            current_block_30 = 17184638872671510253;
        } else if buffer_eq_slen(
            k,
            b"url-query-20-plus\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        ) != 0
        {
            opt = HTTP_PARSEOPT_URL_NORMALIZE_QUERY_20_PLUS as libc::c_int
                as libc::c_ushort;
            current_block_30 = 17184638872671510253;
        } else if buffer_eq_slen(
            k,
            b"url-invalid-utf8-reject\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        ) != 0
        {
            opt = HTTP_PARSEOPT_URL_NORMALIZE_INVALID_UTF8_REJECT as libc::c_int
                as libc::c_ushort;
            current_block_30 = 17184638872671510253;
        } else {
            if buffer_eq_slen(
                k,
                b"header-strict\0" as *const u8 as *const libc::c_char,
                (::core::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            ) != 0
            {
                (*srv).srvconf.http_header_strict = val as libc::c_uchar;
            } else if buffer_eq_slen(
                k,
                b"host-strict\0" as *const u8 as *const libc::c_char,
                (::core::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            ) != 0
            {
                (*srv).srvconf.http_host_strict = val as libc::c_uchar;
            } else if buffer_eq_slen(
                k,
                b"host-normalize\0" as *const u8 as *const libc::c_char,
                (::core::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            ) != 0
            {
                (*srv).srvconf.http_host_normalize = val as libc::c_uchar;
            } else if buffer_eq_slen(
                k,
                b"method-get-body\0" as *const u8 as *const libc::c_char,
                (::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            ) != 0
            {
                (*srv).srvconf.http_method_get_body = val as libc::c_uchar;
            } else {
                log_error(
                    (*srv).errh,
                    b"configfile.c\0" as *const u8 as *const libc::c_char,
                    610 as libc::c_int as libc::c_uint,
                    b"unrecognized key for server.http-parseopts: %s\0" as *const u8
                        as *const libc::c_char,
                    (*k).ptr,
                );
                rc = 0 as libc::c_int;
            }
            current_block_30 = 8258075665625361029;
        }
        match current_block_30 {
            17184638872671510253 => {
                if val != 0 {
                    opts = (opts as libc::c_int | opt as libc::c_int) as libc::c_ushort;
                } else {
                    opts = (opts as libc::c_int & !(opt as libc::c_int))
                        as libc::c_ushort;
                    if opt as libc::c_int == HTTP_PARSEOPT_URL_NORMALIZE as libc::c_int {
                        url_normalize = 0 as libc::c_int as uint8_t;
                    }
                    if opt as libc::c_int
                        == HTTP_PARSEOPT_URL_NORMALIZE_PATH_2F_DECODE as libc::c_int
                    {
                        decode_2f = 0 as libc::c_int as uint8_t;
                    }
                }
            }
            _ => {}
        }
        i = i.wrapping_add(1);
        i;
    }
    if url_normalize == 0 {
        opts = 0 as libc::c_int as libc::c_ushort;
    }
    if opts as libc::c_int != 0 as libc::c_int {
        opts = (opts as libc::c_int | HTTP_PARSEOPT_URL_NORMALIZE as libc::c_int)
            as libc::c_ushort;
        if opts as libc::c_int
            & (HTTP_PARSEOPT_URL_NORMALIZE_PATH_2F_DECODE as libc::c_int
                | HTTP_PARSEOPT_URL_NORMALIZE_PATH_2F_REJECT as libc::c_int)
            == HTTP_PARSEOPT_URL_NORMALIZE_PATH_2F_DECODE as libc::c_int
                | HTTP_PARSEOPT_URL_NORMALIZE_PATH_2F_REJECT as libc::c_int
        {
            log_error(
                (*srv).errh,
                b"configfile.c\0" as *const u8 as *const libc::c_char,
                635 as libc::c_int as libc::c_uint,
                b"conflicting options in server.http-parseopts:url-path-2f-decode, url-path-2f-reject\0"
                    as *const u8 as *const libc::c_char,
            );
            rc = 0 as libc::c_int;
        }
        if opts as libc::c_int
            & (HTTP_PARSEOPT_URL_NORMALIZE_PATH_DOTSEG_REMOVE as libc::c_int
                | HTTP_PARSEOPT_URL_NORMALIZE_PATH_DOTSEG_REJECT as libc::c_int)
            == HTTP_PARSEOPT_URL_NORMALIZE_PATH_DOTSEG_REMOVE as libc::c_int
                | HTTP_PARSEOPT_URL_NORMALIZE_PATH_DOTSEG_REJECT as libc::c_int
        {
            log_error(
                (*srv).errh,
                b"configfile.c\0" as *const u8 as *const libc::c_char,
                644 as libc::c_int as libc::c_uint,
                b"conflicting options in server.http-parseopts:url-path-dotseg-remove, url-path-dotseg-reject\0"
                    as *const u8 as *const libc::c_char,
            );
            rc = 0 as libc::c_int;
        }
        if opts as libc::c_int
            & (HTTP_PARSEOPT_URL_NORMALIZE_UNRESERVED as libc::c_int
                | HTTP_PARSEOPT_URL_NORMALIZE_REQUIRED as libc::c_int) == 0
        {
            opts = (opts as libc::c_int
                | (HTTP_PARSEOPT_URL_NORMALIZE_UNRESERVED as libc::c_int
                    | HTTP_PARSEOPT_URL_NORMALIZE_INVALID_UTF8_REJECT as libc::c_int))
                as libc::c_ushort;
            if decode_2f as libc::c_int != 0
                && opts as libc::c_int
                    & HTTP_PARSEOPT_URL_NORMALIZE_PATH_2F_REJECT as libc::c_int == 0
            {
                opts = (opts as libc::c_int
                    | HTTP_PARSEOPT_URL_NORMALIZE_PATH_2F_DECODE as libc::c_int)
                    as libc::c_ushort;
            }
        }
    }
    (*srv).srvconf.http_url_normalize = opts;
    return rc;
}
static mut cpk: [config_plugin_keys_t; 34] = [config_plugin_keys_t {
    k: 0 as *const libc::c_char,
    klen: 0,
    ktype: 0,
    scope: 0,
}; 34];
unsafe extern "C" fn config_insert_srvconf(mut srv: *mut server) -> libc::c_int {
    (*srv).srvconf.h2proto = 2 as libc::c_int as libc::c_uchar;
    let mut rc: libc::c_int = 0 as libc::c_int;
    let mut srvplug: plugin_data_base = plugin_data_base {
        id: 0,
        nconfig: 0,
        cvlist: 0 as *mut config_plugin_value_t,
        self_0: 0 as *mut plugin,
    };
    memset(
        &mut srvplug as *mut plugin_data_base as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<plugin_data_base>() as libc::c_ulong,
    );
    let p: *mut plugin_data_base = &mut srvplug;
    if config_plugin_values_init(
        srv,
        p as *mut libc::c_void,
        cpk.as_ptr(),
        b"global\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        return HANDLER_ERROR as libc::c_int;
    }
    let mut ssl_enabled: libc::c_int = 0 as libc::c_int;
    if (*((*p).cvlist).offset(0 as libc::c_int as isize)).v.u2[1 as libc::c_int as usize]
        != 0
    {
        let mut cpv: *mut config_plugin_value_t = ((*p).cvlist)
            .offset(
                (*((*p).cvlist).offset(0 as libc::c_int as isize))
                    .v
                    .u2[0 as libc::c_int as usize] as isize,
            );
        while -(1 as libc::c_int) != (*cpv).k_id {
            match (*cpv).k_id {
                0 => {
                    array_copy_array((*srv).srvconf.modules, (*cpv).v.a);
                }
                1 => {
                    (*srv)
                        .srvconf
                        .compat_module_load = (*cpv).v.u as libc::c_ushort
                        as libc::c_uchar;
                }
                2 => {
                    (*srv)
                        .srvconf
                        .systemd_socket_activation = (*cpv).v.u as libc::c_ushort
                        as libc::c_uchar;
                }
                3 => {
                    (*srv).srvconf.port = (*cpv).v.shrt;
                }
                4 => {
                    if buffer_is_blank((*cpv).v.b) == 0 {
                        (*srv).srvconf.bindhost = (*cpv).v.b;
                    }
                }
                5 => {
                    if buffer_is_blank((*cpv).v.b) == 0 {
                        (*srv).srvconf.network_backend = (*cpv).v.b;
                    }
                }
                6 => {
                    if buffer_is_blank((*cpv).v.b) == 0 {
                        (*srv).srvconf.changeroot = (*cpv).v.b;
                    }
                }
                7 => {
                    if buffer_is_blank((*cpv).v.b) == 0 {
                        (*srv).srvconf.username = (*cpv).v.b;
                    }
                }
                8 => {
                    if buffer_is_blank((*cpv).v.b) == 0 {
                        (*srv).srvconf.groupname = (*cpv).v.b;
                    }
                }
                9 => {}
                10 => {}
                11 => {
                    (*srv)
                        .srvconf
                        .errorlog_use_syslog = (*cpv).v.u as libc::c_ushort
                        as libc::c_uchar;
                }
                12 => {
                    if buffer_is_blank((*cpv).v.b) == 0 {
                        (*srv).srvconf.syslog_facility = (*cpv).v.b;
                    }
                }
                13 => {
                    (*srv)
                        .srvconf
                        .enable_cores = (*cpv).v.u as libc::c_ushort as libc::c_uchar;
                }
                14 => {
                    (*srv).srvconf.event_handler = (*(*cpv).v.b).ptr;
                }
                15 => {
                    if buffer_is_blank((*cpv).v.b) == 0 {
                        let ref mut fresh0 = *(&mut (*srv).srvconf.pid_file
                            as *mut *mut buffer as *mut *const buffer);
                        *fresh0 = (*cpv).v.b;
                    }
                }
                16 => {
                    (*srv).srvconf.max_worker = (*cpv).v.u as libc::c_ushort;
                }
                17 => {
                    (*srv).srvconf.max_fds = (*cpv).v.u as libc::c_ushort;
                }
                18 => {
                    (*srv).srvconf.max_conns = (*cpv).v.u as libc::c_ushort;
                }
                19 => {
                    (*srv).srvconf.max_request_field_size = (*cpv).v.shrt as uint32_t;
                }
                20 => {
                    chunkqueue_set_chunk_size((*cpv).v.u as size_t);
                }
                21 => {
                    (*srv).srvconf.upload_temp_file_size = (*cpv).v.u;
                }
                22 => {
                    array_copy_array((*srv).srvconf.upload_tempdirs, (*cpv).v.a);
                }
                23 => {
                    if config_http_parseopts(srv, (*cpv).v.a) == 0 {
                        rc = HANDLER_ERROR as libc::c_int;
                    }
                }
                24 => {
                    (*srv)
                        .srvconf
                        .http_header_strict = (0 as libc::c_int as libc::c_uint
                        != (*cpv).v.u) as libc::c_int as libc::c_uchar;
                }
                25 => {
                    (*srv)
                        .srvconf
                        .http_host_strict = (0 as libc::c_int as libc::c_uint
                        != (*cpv).v.u) as libc::c_int as libc::c_uchar;
                }
                26 => {
                    (*srv)
                        .srvconf
                        .http_host_normalize = (0 as libc::c_int as libc::c_uint
                        != (*cpv).v.u) as libc::c_int as libc::c_uchar;
                }
                27 => {}
                28 => {
                    if 0 as libc::c_int
                        != stat_cache_choose_engine((*cpv).v.b, (*srv).errh)
                    {
                        rc = HANDLER_ERROR as libc::c_int;
                    }
                }
                29 => {
                    stat_cache_xattrname((*(*cpv).v.b).ptr);
                }
                30 => {
                    ssl_enabled = (0 as libc::c_int as libc::c_uint != (*cpv).v.u)
                        as libc::c_int;
                    if ssl_enabled != 0 {
                        log_error(
                            (*srv).errh,
                            b"configfile.c\0" as *const u8 as *const libc::c_char,
                            892 as libc::c_int as libc::c_uint,
                            b"ssl support is missing; recompile with e.g. --with-openssl\0"
                                as *const u8 as *const libc::c_char,
                        );
                        rc = HANDLER_ERROR as libc::c_int;
                    }
                }
                31 => {
                    (*srv)
                        .srvconf
                        .log_request_header_on_error = (0 as libc::c_int as libc::c_uint
                        != (*cpv).v.u) as libc::c_int as libc::c_uchar;
                }
                32 => {
                    (*srv).srvconf.feature_flags = (*cpv).v.a;
                    (*srv)
                        .srvconf
                        .h2proto = config_plugin_value_to_bool(
                        array_get_element_klen(
                            (*cpv).v.a,
                            b"server.h2proto\0" as *const u8 as *const libc::c_char,
                            (::core::mem::size_of::<[libc::c_char; 15]>()
                                as libc::c_ulong as uint32_t)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint),
                        ),
                        1 as libc::c_int,
                    ) as libc::c_uchar;
                    if (*srv).srvconf.h2proto != 0 {
                        (*srv)
                            .srvconf
                            .h2proto = ((*srv).srvconf.h2proto as libc::c_int
                            + config_plugin_value_to_bool(
                                array_get_element_klen(
                                    (*cpv).v.a,
                                    b"server.h2c\0" as *const u8 as *const libc::c_char,
                                    (::core::mem::size_of::<[libc::c_char; 11]>()
                                        as libc::c_ulong as uint32_t)
                                        .wrapping_sub(1 as libc::c_int as libc::c_uint),
                                ),
                                1 as libc::c_int,
                            )) as libc::c_uchar;
                    }
                    (*srv)
                        .srvconf
                        .absolute_dir_redirect = config_plugin_value_to_bool(
                        array_get_element_klen(
                            (*cpv).v.a,
                            b"server.absolute-dir-redirect\0" as *const u8
                                as *const libc::c_char,
                            (::core::mem::size_of::<[libc::c_char; 29]>()
                                as libc::c_ulong as uint32_t)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint),
                        ),
                        0 as libc::c_int,
                    ) as libc::c_uchar;
                }
                _ => {}
            }
            cpv = cpv.offset(1);
            cpv;
        }
    }
    if 0 as libc::c_int == (*srv).srvconf.port as libc::c_int {
        (*srv)
            .srvconf
            .port = (if ssl_enabled != 0 {
            443 as libc::c_int
        } else {
            80 as libc::c_int
        }) as libc::c_ushort;
    }
    if config_feature_bool(
        srv,
        b"server.h2proto\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    ) != 0
    {
        array_insert_value(
            (*srv).srvconf.modules,
            b"mod_h2\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
    }
    config_deprecate_module_compress(srv);
    config_check_module_duplicates(srv);
    if (*srv).srvconf.compat_module_load != 0 {
        config_compat_module_load(srv);
    }
    if (*srv).srvconf.http_url_normalize != 0 {
        config_burl_normalize_cond(srv);
    }
    if config_pcre_keyvalue(srv) == 0 {
        rc = HANDLER_ERROR as libc::c_int;
    }
    free(srvplug.cvlist as *mut libc::c_void);
    return rc;
}
#[cold]
unsafe extern "C" fn config_mimetypes_default(a: *mut array) {
    static mut mimetypes_default: [*const libc::c_char; 168] = [
        b".html\0" as *const u8 as *const libc::c_char,
        b"text/html\0" as *const u8 as *const libc::c_char,
        b".htm\0" as *const u8 as *const libc::c_char,
        b"text/html\0" as *const u8 as *const libc::c_char,
        b".txt\0" as *const u8 as *const libc::c_char,
        b"text/plain;charset=utf-8\0" as *const u8 as *const libc::c_char,
        b".text\0" as *const u8 as *const libc::c_char,
        b"text/plain;charset=utf-8\0" as *const u8 as *const libc::c_char,
        b".css\0" as *const u8 as *const libc::c_char,
        b"text/css;charset=utf-8\0" as *const u8 as *const libc::c_char,
        b".js\0" as *const u8 as *const libc::c_char,
        b"text/javascript\0" as *const u8 as *const libc::c_char,
        b".mjs\0" as *const u8 as *const libc::c_char,
        b"text/javascript\0" as *const u8 as *const libc::c_char,
        b".xml\0" as *const u8 as *const libc::c_char,
        b"text/xml\0" as *const u8 as *const libc::c_char,
        b".aac\0" as *const u8 as *const libc::c_char,
        b"audio/aac\0" as *const u8 as *const libc::c_char,
        b".flac\0" as *const u8 as *const libc::c_char,
        b"audio/flac\0" as *const u8 as *const libc::c_char,
        b".m4a\0" as *const u8 as *const libc::c_char,
        b"audio/mp4\0" as *const u8 as *const libc::c_char,
        b".mp3\0" as *const u8 as *const libc::c_char,
        b"audio/mpeg\0" as *const u8 as *const libc::c_char,
        b".oga\0" as *const u8 as *const libc::c_char,
        b"audio/ogg\0" as *const u8 as *const libc::c_char,
        b".ogg\0" as *const u8 as *const libc::c_char,
        b"audio/ogg\0" as *const u8 as *const libc::c_char,
        b".opus\0" as *const u8 as *const libc::c_char,
        b"audio/opus\0" as *const u8 as *const libc::c_char,
        b".wav\0" as *const u8 as *const libc::c_char,
        b"audio/x-wav\0" as *const u8 as *const libc::c_char,
        b".weba\0" as *const u8 as *const libc::c_char,
        b"audio/webm\0" as *const u8 as *const libc::c_char,
        b".ogx\0" as *const u8 as *const libc::c_char,
        b"application/ogg\0" as *const u8 as *const libc::c_char,
        b".apng\0" as *const u8 as *const libc::c_char,
        b"image/apng\0" as *const u8 as *const libc::c_char,
        b".avif\0" as *const u8 as *const libc::c_char,
        b"image/avif\0" as *const u8 as *const libc::c_char,
        b".bmp\0" as *const u8 as *const libc::c_char,
        b"image/bmp\0" as *const u8 as *const libc::c_char,
        b".gif\0" as *const u8 as *const libc::c_char,
        b"image/gif\0" as *const u8 as *const libc::c_char,
        b".jpeg\0" as *const u8 as *const libc::c_char,
        b"image/jpeg\0" as *const u8 as *const libc::c_char,
        b".jpg\0" as *const u8 as *const libc::c_char,
        b"image/jpeg\0" as *const u8 as *const libc::c_char,
        b".png\0" as *const u8 as *const libc::c_char,
        b"image/png\0" as *const u8 as *const libc::c_char,
        b".svg\0" as *const u8 as *const libc::c_char,
        b"image/svg+xml\0" as *const u8 as *const libc::c_char,
        b".svgz\0" as *const u8 as *const libc::c_char,
        b"image/svg+xml\0" as *const u8 as *const libc::c_char,
        b".tiff\0" as *const u8 as *const libc::c_char,
        b"image/tiff\0" as *const u8 as *const libc::c_char,
        b".webp\0" as *const u8 as *const libc::c_char,
        b"image/webp\0" as *const u8 as *const libc::c_char,
        b".avi\0" as *const u8 as *const libc::c_char,
        b"video/x-msvideo\0" as *const u8 as *const libc::c_char,
        b".mkv\0" as *const u8 as *const libc::c_char,
        b"video/x-matroska\0" as *const u8 as *const libc::c_char,
        b".m4v\0" as *const u8 as *const libc::c_char,
        b"video/mp4\0" as *const u8 as *const libc::c_char,
        b".mp4\0" as *const u8 as *const libc::c_char,
        b"video/mp4\0" as *const u8 as *const libc::c_char,
        b".mpeg\0" as *const u8 as *const libc::c_char,
        b"video/mpeg\0" as *const u8 as *const libc::c_char,
        b".mpg\0" as *const u8 as *const libc::c_char,
        b"video/mpeg\0" as *const u8 as *const libc::c_char,
        b".ogv\0" as *const u8 as *const libc::c_char,
        b"video/ogg\0" as *const u8 as *const libc::c_char,
        b".mov\0" as *const u8 as *const libc::c_char,
        b"video/quicktime\0" as *const u8 as *const libc::c_char,
        b".qt\0" as *const u8 as *const libc::c_char,
        b"video/quicktime\0" as *const u8 as *const libc::c_char,
        b".webm\0" as *const u8 as *const libc::c_char,
        b"video/webm\0" as *const u8 as *const libc::c_char,
        b".json\0" as *const u8 as *const libc::c_char,
        b"application/json\0" as *const u8 as *const libc::c_char,
        b".dtd\0" as *const u8 as *const libc::c_char,
        b"application/xml-dtd\0" as *const u8 as *const libc::c_char,
        b".pdf\0" as *const u8 as *const libc::c_char,
        b"application/pdf\0" as *const u8 as *const libc::c_char,
        b".xhtml\0" as *const u8 as *const libc::c_char,
        b"application/xhtml+xml\0" as *const u8 as *const libc::c_char,
        b".eot\0" as *const u8 as *const libc::c_char,
        b"application/vnd.ms-fontobject\0" as *const u8 as *const libc::c_char,
        b".otf\0" as *const u8 as *const libc::c_char,
        b"font/otf\0" as *const u8 as *const libc::c_char,
        b".sfnt\0" as *const u8 as *const libc::c_char,
        b"font/sfnt\0" as *const u8 as *const libc::c_char,
        b".ttc\0" as *const u8 as *const libc::c_char,
        b"font/collection\0" as *const u8 as *const libc::c_char,
        b".ttf\0" as *const u8 as *const libc::c_char,
        b"font/ttf\0" as *const u8 as *const libc::c_char,
        b".woff\0" as *const u8 as *const libc::c_char,
        b"font/woff\0" as *const u8 as *const libc::c_char,
        b".woff2\0" as *const u8 as *const libc::c_char,
        b"font/woff2\0" as *const u8 as *const libc::c_char,
        b".conf\0" as *const u8 as *const libc::c_char,
        b"text/plain\0" as *const u8 as *const libc::c_char,
        b".log\0" as *const u8 as *const libc::c_char,
        b"text/plain\0" as *const u8 as *const libc::c_char,
        b".csv\0" as *const u8 as *const libc::c_char,
        b"text/csv\0" as *const u8 as *const libc::c_char,
        b".rtf\0" as *const u8 as *const libc::c_char,
        b"text/rtf\0" as *const u8 as *const libc::c_char,
        b".ics\0" as *const u8 as *const libc::c_char,
        b"text/calendar\0" as *const u8 as *const libc::c_char,
        b".md\0" as *const u8 as *const libc::c_char,
        b"text/markdown;charset=utf-8\0" as *const u8 as *const libc::c_char,
        b".ico\0" as *const u8 as *const libc::c_char,
        b"image/vnd.microsoft.icon\0" as *const u8 as *const libc::c_char,
        b".7z\0" as *const u8 as *const libc::c_char,
        b"application/x-7z-compressed\0" as *const u8 as *const libc::c_char,
        b".bz2\0" as *const u8 as *const libc::c_char,
        b"application/x-bzip2\0" as *const u8 as *const libc::c_char,
        b".gz\0" as *const u8 as *const libc::c_char,
        b"application/gzip\0" as *const u8 as *const libc::c_char,
        b".rar\0" as *const u8 as *const libc::c_char,
        b"application/vnd.rar\0" as *const u8 as *const libc::c_char,
        b".tar\0" as *const u8 as *const libc::c_char,
        b"application/x-tar\0" as *const u8 as *const libc::c_char,
        b".tar.gz\0" as *const u8 as *const libc::c_char,
        b"application/x-gtar-compressed\0" as *const u8 as *const libc::c_char,
        b".tgz\0" as *const u8 as *const libc::c_char,
        b"application/x-gtar-compressed\0" as *const u8 as *const libc::c_char,
        b".xz\0" as *const u8 as *const libc::c_char,
        b"application/x-xz\0" as *const u8 as *const libc::c_char,
        b".zip\0" as *const u8 as *const libc::c_char,
        b"application/zip\0" as *const u8 as *const libc::c_char,
        b".zst\0" as *const u8 as *const libc::c_char,
        b"application/zstd\0" as *const u8 as *const libc::c_char,
        b".bin\0" as *const u8 as *const libc::c_char,
        b"application/octet-stream\0" as *const u8 as *const libc::c_char,
        b".class\0" as *const u8 as *const libc::c_char,
        b"application/java-vm\0" as *const u8 as *const libc::c_char,
        b".dll\0" as *const u8 as *const libc::c_char,
        b"application/x-msdos-program\0" as *const u8 as *const libc::c_char,
        b".exe\0" as *const u8 as *const libc::c_char,
        b"application/x-msdos-program\0" as *const u8 as *const libc::c_char,
        b".img\0" as *const u8 as *const libc::c_char,
        b"application/octet-stream\0" as *const u8 as *const libc::c_char,
        b".iso\0" as *const u8 as *const libc::c_char,
        b"application/x-iso9660-image\0" as *const u8 as *const libc::c_char,
        b".jar\0" as *const u8 as *const libc::c_char,
        b"application/java-archive\0" as *const u8 as *const libc::c_char,
        b".lha\0" as *const u8 as *const libc::c_char,
        b"application/x-lha\0" as *const u8 as *const libc::c_char,
        b".lhz\0" as *const u8 as *const libc::c_char,
        b"application/x-lzh\0" as *const u8 as *const libc::c_char,
        b".so\0" as *const u8 as *const libc::c_char,
        b"application/octet-stream\0" as *const u8 as *const libc::c_char,
        b".torrent\0" as *const u8 as *const libc::c_char,
        b"application/x-bittorrent\0" as *const u8 as *const libc::c_char,
        b".deb\0" as *const u8 as *const libc::c_char,
        b"application/vnd.debian.binary-package\0" as *const u8 as *const libc::c_char,
        b".dmg\0" as *const u8 as *const libc::c_char,
        b"application/x-apple-diskimage\0" as *const u8 as *const libc::c_char,
        b".rpm\0" as *const u8 as *const libc::c_char,
        b"application/x-redhat-package-manager\0" as *const u8 as *const libc::c_char,
        b".sig\0" as *const u8 as *const libc::c_char,
        b"application/pgp-signature\0" as *const u8 as *const libc::c_char,
        b"README\0" as *const u8 as *const libc::c_char,
        b"text/plain;charset=utf-8\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
        b"application/octet-stream\0" as *const u8 as *const libc::c_char,
    ];
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    loop {
        array_set_key_value(
            a,
            mimetypes_default[i as usize],
            strlen(mimetypes_default[i as usize]) as uint32_t,
            mimetypes_default[i.wrapping_add(1 as libc::c_int as libc::c_uint) as usize],
            strlen(
                mimetypes_default[i.wrapping_add(1 as libc::c_int as libc::c_uint)
                    as usize],
            ) as uint32_t,
        );
        i = (i as libc::c_uint).wrapping_add(2 as libc::c_int as libc::c_uint)
            as uint32_t as uint32_t;
        if !((i as libc::c_ulong)
            < (::core::mem::size_of::<[*const libc::c_char; 168]>() as libc::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
                ))
        {
            break;
        }
    };
}
static mut cpk_0: [config_plugin_keys_t; 35] = [config_plugin_keys_t {
    k: 0 as *const libc::c_char,
    klen: 0,
    ktype: 0,
    scope: 0,
}; 35];
unsafe extern "C" fn config_insert(mut srv: *mut server) -> libc::c_int {
    let mut rc: libc::c_int = 0 as libc::c_int;
    let p: *mut config_data_base = ck_calloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<config_data_base>() as libc::c_ulong,
    ) as *mut config_data_base;
    (*srv).config_data_base = p as *mut libc::c_void;
    if config_plugin_values_init(
        srv,
        p as *mut libc::c_void,
        cpk_0.as_ptr(),
        b"base\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        return HANDLER_ERROR as libc::c_int;
    }
    let mut i: libc::c_int = ((*((*p).cvlist).offset(0 as libc::c_int as isize))
        .v
        .u2[1 as libc::c_int as usize] == 0) as libc::c_int;
    while i < (*p).nconfig {
        let mut cpv: *mut config_plugin_value_t = ((*p).cvlist)
            .offset(
                (*((*p).cvlist).offset(i as isize)).v.u2[0 as libc::c_int as usize]
                    as isize,
            );
        while -(1 as libc::c_int) != (*cpv).k_id {
            let mut current_block_36: u64;
            match (*cpv).k_id {
                0 => {
                    current_block_36 = 13826291924415791078;
                }
                1 => {
                    if buffer_is_blank((*cpv).v.b) != 0 {
                        (*cpv).v.b = 0 as *const buffer;
                    }
                    current_block_36 = 13826291924415791078;
                }
                2 => {
                    if buffer_is_blank((*cpv).v.b) == 0 {
                        let mut b: *mut buffer = 0 as *mut buffer;
                        let ref mut fresh1 = *(&mut b as *mut *mut buffer
                            as *mut *const buffer);
                        *fresh1 = (*cpv).v.b;
                        let mut t: *mut libc::c_char = strchr((*b).ptr, '\n' as i32);
                        while !t.is_null() {
                            if !(*t.offset(1 as libc::c_int as isize) as libc::c_int
                                == ' ' as i32
                                || *t.offset(1 as libc::c_int as isize) as libc::c_int
                                    == '\t' as i32)
                            {
                                let mut off: off_t = t.offset_from((*b).ptr)
                                    as libc::c_long;
                                let mut len: size_t = buffer_clen(b) as size_t;
                                buffer_string_prepare_append(b, 1 as libc::c_int as size_t);
                                t = ((*b).ptr).offset(off as isize);
                                memmove(
                                    t.offset(2 as libc::c_int as isize) as *mut libc::c_void,
                                    t.offset(1 as libc::c_int as isize) as *const libc::c_void,
                                    len
                                        .wrapping_sub(off as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                );
                                *t
                                    .offset(
                                        1 as libc::c_int as isize,
                                    ) = ' ' as i32 as libc::c_char;
                                buffer_commit(b, 1 as libc::c_int as size_t);
                            }
                            t = strchr(t.offset(2 as libc::c_int as isize), '\n' as i32);
                        }
                        let mut t_0: *mut libc::c_char = (*b).ptr;
                        while *t_0 as libc::c_int == ' ' as i32
                            || *t_0 as libc::c_int == '\t' as i32
                            || *t_0 as libc::c_int == '\r' as i32
                            || *t_0 as libc::c_int == '\n' as i32
                        {
                            t_0 = t_0.offset(1);
                            t_0;
                        }
                        if *t_0 as libc::c_int == '\0' as i32 {
                            buffer_truncate(b, 0 as libc::c_int as uint32_t);
                        }
                        if buffer_is_blank(b) != 0 && 0 as libc::c_int != i {
                            (*cpv).v.b = 0 as *const buffer;
                        } else {
                            buffer_string_prepare_append(b, 6 as libc::c_int as size_t);
                            memcpy(
                                ((*b).ptr)
                                    .offset(buffer_clen(b) as isize)
                                    .offset(1 as libc::c_int as isize) as *mut libc::c_void,
                                b"server\0" as *const u8 as *const libc::c_char
                                    as *const libc::c_void,
                                6 as libc::c_int as libc::c_ulong,
                            );
                        }
                    } else if 0 as libc::c_int != i {
                        (*cpv).v.b = 0 as *const buffer;
                    }
                    current_block_36 = 13826291924415791078;
                }
                3 => {
                    current_block_36 = 15673671134019560096;
                }
                4 => {
                    current_block_36 = 15673671134019560096;
                }
                5 => {
                    current_block_36 = 4044967834681222298;
                }
                6 => {
                    current_block_36 = 12019201726191173971;
                }
                7 => {
                    current_block_36 = 7325936209844867276;
                }
                8 => {
                    current_block_36 = 17391114015930030077;
                }
                9 | 10 => {
                    current_block_36 = 17391114015930030077;
                }
                11 => {
                    current_block_36 = 14915700679857323593;
                }
                12 => {
                    current_block_36 = 14915700679857323593;
                }
                13 => {
                    current_block_36 = 13826291924415791078;
                }
                14 => {
                    current_block_36 = 260221932837729860;
                }
                15 => {
                    current_block_36 = 260221932837729860;
                }
                16 => {
                    if (*cpv).v.shrt as libc::c_int
                        & (1 as libc::c_int) << 1 as libc::c_int != 0
                    {
                        (*cpv)
                            .v
                            .shrt = ((*cpv).v.shrt as libc::c_int
                            | (1 as libc::c_int) << 0 as libc::c_int) as libc::c_ushort;
                    }
                    current_block_36 = 13826291924415791078;
                }
                17 => {
                    if (*cpv).v.shrt as libc::c_int
                        & (1 as libc::c_int) << 1 as libc::c_int != 0
                    {
                        (*cpv)
                            .v
                            .shrt = ((*cpv).v.shrt as libc::c_int
                            | (1 as libc::c_int) << 0 as libc::c_int) as libc::c_ushort;
                    }
                    current_block_36 = 13826291924415791078;
                }
                18 => {
                    let cnt: *mut off_t = ck_malloc(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<off_t>() as libc::c_ulong,
                            ),
                    ) as *mut off_t;
                    *cnt.offset(0 as libc::c_int as isize) = 0 as libc::c_int as off_t;
                    *cnt
                        .offset(
                            1 as libc::c_int as isize,
                        ) = ((*cpv).v.shrt as off_t) << 10 as libc::c_int;
                    (*cpv).v.v = cnt as *mut libc::c_void;
                    (*cpv).vtype = T_CONFIG_LOCAL;
                    current_block_36 = 13826291924415791078;
                }
                19 => {
                    current_block_36 = 13826291924415791078;
                }
                20 => {
                    let ds: *mut data_string = array_get_data_unset(
                        (*cpv).v.a,
                        b".js\0" as *const u8 as *const libc::c_char,
                        (::core::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint),
                    ) as *mut data_string;
                    if !ds.is_null()
                        && buffer_eq_slen(
                            &mut (*ds).value,
                            b"application/javascript\0" as *const u8
                                as *const libc::c_char,
                            (::core::mem::size_of::<[libc::c_char; 23]>()
                                as libc::c_ulong as uint32_t)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                        ) != 0
                    {
                        buffer_copy_string_len(
                            &mut (*ds).value,
                            b"text/javascript\0" as *const u8 as *const libc::c_char,
                            (::core::mem::size_of::<[libc::c_char; 16]>()
                                as libc::c_ulong as uint32_t)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                        );
                    }
                    current_block_36 = 13826291924415791078;
                }
                21 => {
                    current_block_36 = 6952961785245484067;
                }
                22 => {
                    current_block_36 = 6952961785245484067;
                }
                23 => {
                    current_block_36 = 7624374721444664992;
                }
                24 => {
                    current_block_36 = 16344026168134789597;
                }
                25 => {
                    current_block_36 = 13978907414850918344;
                }
                26 => {
                    current_block_36 = 6950096536176871466;
                }
                27 => {
                    current_block_36 = 17223499384403972148;
                }
                28 => {
                    current_block_36 = 7226479899448509951;
                }
                29 => {
                    current_block_36 = 11704601705159938947;
                }
                30 => {
                    current_block_36 = 1959906895173515371;
                }
                31 => {
                    current_block_36 = 14119566731743225857;
                }
                32 => {
                    current_block_36 = 3292440606114030865;
                }
                33 => {
                    current_block_36 = 16452509741658398146;
                }
                _ => {
                    current_block_36 = 13826291924415791078;
                }
            }
            match current_block_36 {
                15673671134019560096 => {
                    current_block_36 = 4044967834681222298;
                }
                17391114015930030077 => {
                    if buffer_is_blank((*cpv).v.b) != 0 {
                        (*cpv).v.b = 0 as *const buffer;
                    }
                    current_block_36 = 13826291924415791078;
                }
                14915700679857323593 => {
                    current_block_36 = 13826291924415791078;
                }
                260221932837729860 => {
                    current_block_36 = 13826291924415791078;
                }
                6952961785245484067 => {
                    current_block_36 = 7624374721444664992;
                }
                _ => {}
            }
            match current_block_36 {
                4044967834681222298 => {
                    current_block_36 = 12019201726191173971;
                }
                7624374721444664992 => {
                    current_block_36 = 16344026168134789597;
                }
                _ => {}
            }
            match current_block_36 {
                12019201726191173971 => {
                    current_block_36 = 7325936209844867276;
                }
                16344026168134789597 => {
                    current_block_36 = 13978907414850918344;
                }
                _ => {}
            }
            match current_block_36 {
                7325936209844867276 => {
                    current_block_36 = 13826291924415791078;
                }
                13978907414850918344 => {
                    current_block_36 = 6950096536176871466;
                }
                _ => {}
            }
            match current_block_36 {
                6950096536176871466 => {
                    current_block_36 = 17223499384403972148;
                }
                _ => {}
            }
            match current_block_36 {
                17223499384403972148 => {
                    current_block_36 = 7226479899448509951;
                }
                _ => {}
            }
            match current_block_36 {
                7226479899448509951 => {
                    current_block_36 = 11704601705159938947;
                }
                _ => {}
            }
            match current_block_36 {
                11704601705159938947 => {
                    current_block_36 = 1959906895173515371;
                }
                _ => {}
            }
            match current_block_36 {
                1959906895173515371 => {
                    current_block_36 = 14119566731743225857;
                }
                _ => {}
            }
            match current_block_36 {
                14119566731743225857 => {
                    current_block_36 = 3292440606114030865;
                }
                _ => {}
            }
            match current_block_36 {
                3292440606114030865 => {
                    current_block_36 = 16452509741658398146;
                }
                _ => {}
            }
            match current_block_36 {
                16452509741658398146 => {}
                _ => {}
            }
            cpv = cpv.offset(1);
            cpv;
        }
        i += 1;
        i;
    }
    (*p).defaults.errh = (*srv).errh;
    (*p).defaults.max_keep_alive_requests = 1000 as libc::c_int as libc::c_ushort;
    (*p).defaults.max_keep_alive_idle = 5 as libc::c_int as libc::c_ushort;
    (*p).defaults.max_read_idle = 60 as libc::c_int as libc::c_ushort;
    (*p).defaults.max_write_idle = 360 as libc::c_int as libc::c_ushort;
    ((*p).defaults).set_follow_symlink(1 as libc::c_int as libc::c_uint);
    ((*p).defaults).set_allow_http11(1 as libc::c_int as libc::c_uint);
    ((*p).defaults)
        .set_etag_flags(
            (ETAG_USE_INODE as libc::c_int | ETAG_USE_MTIME as libc::c_int
                | ETAG_USE_SIZE as libc::c_int) as libc::c_uint,
        );
    ((*p).defaults).set_range_requests(1 as libc::c_int as libc::c_uint);
    ((*p).defaults).set_force_lowercase_filenames(2 as libc::c_int as libc::c_uint);
    (*p)
        .defaults
        .http_parseopts = ((if (*srv).srvconf.http_header_strict as libc::c_int != 0 {
        HTTP_PARSEOPT_HEADER_STRICT as libc::c_int
    } else {
        0 as libc::c_int
    })
        | (if (*srv).srvconf.http_host_strict as libc::c_int != 0 {
            HTTP_PARSEOPT_HOST_STRICT as libc::c_int
                | HTTP_PARSEOPT_HOST_NORMALIZE as libc::c_int
        } else {
            0 as libc::c_int
        })
        | (if (*srv).srvconf.http_host_normalize as libc::c_int != 0 {
            HTTP_PARSEOPT_HOST_NORMALIZE as libc::c_int
        } else {
            0 as libc::c_int
        })
        | (if (*srv).srvconf.http_method_get_body as libc::c_int != 0 {
            HTTP_PARSEOPT_METHOD_GET_BODY as libc::c_int
        } else {
            0 as libc::c_int
        })) as libc::c_uint;
    (*p).defaults.http_parseopts |= (*srv).srvconf.http_url_normalize as libc::c_uint;
    (*p).defaults.mimetypes = &mut (*srv).srvconf.mimetypes_default;
    ((*p).defaults).set_h2proto((*srv).srvconf.h2proto as libc::c_uint);
    ((*p).defaults)
        .set_http_pathinfo(
            config_feature_bool(
                srv,
                b"server.http-pathinfo\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int,
            ) as libc::c_uint,
        );
    if (*p).nconfig > 0 as libc::c_int
        && (*(*p).cvlist).v.u2[1 as libc::c_int as usize] != 0
    {
        let mut cpv_0: *const config_plugin_value_t = ((*p).cvlist)
            .offset((*(*p).cvlist).v.u2[0 as libc::c_int as usize] as isize);
        if -(1 as libc::c_int) != (*cpv_0).k_id {
            config_merge_config(&mut (*p).defaults, cpv_0);
        }
    }
    if (*p).defaults.mimetypes
        == &mut (*srv).srvconf.mimetypes_default as *mut array as *const array
    {
        config_mimetypes_default(&mut (*srv).srvconf.mimetypes_default);
    }
    (*p).defaults.max_request_field_size = (*srv).srvconf.max_request_field_size;
    ((*p).defaults)
        .set_log_request_header_on_error(
            (*srv).srvconf.log_request_header_on_error as libc::c_uint,
        );
    if ((*p).defaults).log_request_handling() as libc::c_int != 0
        || ((*p).defaults).log_request_header() as libc::c_int != 0
    {
        ((*p).defaults)
            .set_log_request_header_on_error(1 as libc::c_int as libc::c_uint);
    }
    request_config_set_defaults(&mut (*p).defaults);
    return rc;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn config_finalize(
    mut srv: *mut server,
    mut default_server_tag: *const buffer,
) -> libc::c_int {
    let p: *mut config_data_base = (*srv).config_data_base as *mut config_data_base;
    (*srv)
        .srvconf
        .high_precision_timestamps = config_feature_bool(
        srv,
        b"server.metrics-high-precision\0" as *const u8 as *const libc::c_char,
        (*srv).srvconf.high_precision_timestamps as libc::c_int,
    ) as libc::c_uchar;
    ((*p).defaults)
        .set_high_precision_timestamps(
            (*srv).srvconf.high_precision_timestamps as libc::c_uint,
        );
    ((*p).defaults).set_h2proto((*srv).srvconf.h2proto as libc::c_uint);
    if ((*p).defaults.server_tag).is_null() {
        (*p).defaults.server_tag = default_server_tag;
    } else if buffer_is_blank((*p).defaults.server_tag) != 0 {
        (*p).defaults.server_tag = 0 as *const buffer;
    }
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*(*srv).config_context).used {
        let mut config: *mut array = (*(*((*(*srv).config_context).data)
            .offset(i as isize) as *mut data_config))
            .value;
        let mut j: uint32_t = 0 as libc::c_int as uint32_t;
        while !config.is_null() && j < (*config).used {
            let k: *const buffer = &mut (**((*config).data).offset(j as isize)).key;
            if !(strncmp(
                (*k).ptr,
                b"var.\0" as *const u8 as *const libc::c_char,
                (::core::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) == 0 as libc::c_int)
            {
                if !(strncmp(
                    (*k).ptr,
                    b"dir-listing.\0" as *const u8 as *const libc::c_char,
                    (::core::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                ) == 0 as libc::c_int
                    && strcmp(
                        (*k).ptr,
                        b"dir-listing.activate\0" as *const u8 as *const libc::c_char,
                    ) != 0 as libc::c_int)
                {
                    if (array_get_element_klen(
                        (*srv).srvconf.config_touched,
                        (*k).ptr,
                        buffer_clen(k),
                    ))
                        .is_null()
                    {
                        log_pri(
                            (*srv).errh,
                            b"configfile.c\0" as *const u8 as *const libc::c_char,
                            1398 as libc::c_int as libc::c_uint,
                            4 as libc::c_int,
                            b"WARNING: unknown config-key: %s (ignored)\0" as *const u8
                                as *const libc::c_char,
                            (*k).ptr,
                        );
                    }
                }
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    array_free((*srv).srvconf.config_touched);
    (*srv).srvconf.config_touched = 0 as *mut array;
    if (*srv).srvconf.config_unsupported as libc::c_int != 0
        || (*srv).srvconf.config_deprecated as libc::c_int != 0
    {
        if (*srv).srvconf.config_unsupported != 0 {
            log_error(
                (*srv).errh,
                b"configfile.c\0" as *const u8 as *const libc::c_char,
                1408 as libc::c_int as libc::c_uint,
                b"Configuration contains unsupported keys. Going down.\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if (*srv).srvconf.config_deprecated != 0 {
            log_error(
                (*srv).errh,
                b"configfile.c\0" as *const u8 as *const libc::c_char,
                1411 as libc::c_int as libc::c_uint,
                b"Configuration contains deprecated keys. Going down.\0" as *const u8
                    as *const libc::c_char,
            );
        }
        return 0 as libc::c_int;
    }
    if ((*srv).config_captures != 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        let mut i_0: uint32_t = 1 as libc::c_int as uint32_t;
        while i_0 < (*(*srv).config_context).used {
            let dc: *mut data_config = *((*(*srv).config_context).data)
                .offset(i_0 as isize) as *mut data_config;
            if !((0 as libc::c_int == (*dc).capture_idx) as libc::c_int as libc::c_long
                != 0)
            {
                match (*dc).cond as libc::c_uint {
                    1 | 5 | 6 => {
                        let b: *mut buffer = &mut (*dc).string;
                        if (*dc).cond as libc::c_uint
                            != CONFIG_COND_SUFFIX as libc::c_int as libc::c_uint
                            || *((*b).ptr).offset(0 as libc::c_int as isize)
                                as libc::c_int == '.' as i32
                        {
                            buffer_extend(b, 1 as libc::c_int as size_t);
                            memmove(
                                ((*b).ptr).offset(1 as libc::c_int as isize)
                                    as *mut libc::c_void,
                                (*b).ptr as *const libc::c_void,
                                (buffer_clen(b))
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                    as libc::c_ulong,
                            );
                            *((*b).ptr)
                                .offset(
                                    0 as libc::c_int as isize,
                                ) = (if (*dc).cond as libc::c_uint
                                == CONFIG_COND_SUFFIX as libc::c_int as libc::c_uint
                            {
                                '\\' as i32
                            } else {
                                '^' as i32
                            }) as libc::c_char;
                        }
                        if (*dc).cond as libc::c_uint
                            != CONFIG_COND_PREFIX as libc::c_int as libc::c_uint
                        {
                            buffer_append_char(b, '$' as i32 as libc::c_char);
                        }
                        (*dc).cond = CONFIG_COND_MATCH;
                        let pcre_jit: libc::c_int = config_feature_bool(
                            srv,
                            b"server.pcre_jit\0" as *const u8 as *const libc::c_char,
                            1 as libc::c_int,
                        );
                        if data_config_pcre_compile(dc, pcre_jit, (*srv).errh) == 0 {
                            return 0 as libc::c_int;
                        }
                    }
                    _ => {}
                }
            }
            i_0 = i_0.wrapping_add(1);
            i_0;
        }
    }
    let mut i_1: uint32_t = 1 as libc::c_int as uint32_t;
    while i_1 < (*(*srv).config_context).used {
        let dc_0: *mut data_config = *((*(*srv).config_context).data)
            .offset(i_1 as isize) as *mut data_config;
        if ((*dc_0).cond as libc::c_uint
            == CONFIG_COND_MATCH as libc::c_int as libc::c_uint
            || (*dc_0).cond as libc::c_uint
                == CONFIG_COND_NOMATCH as libc::c_int as libc::c_uint)
            && 0 as libc::c_int == (*dc_0).capture_idx
        {
            if (0 as *mut libc::c_void == (*srv).match_data) as libc::c_int
                as libc::c_long != 0
            {
                let mut ovec_max: uint32_t = 10 as libc::c_int as uint32_t;
                (*srv)
                    .match_data = pcre2_match_data_create_8(
                    ovec_max,
                    0 as *mut pcre2_general_context_8,
                ) as *mut libc::c_void;
                if ((*srv).match_data).is_null() {
                    ck_assert_failed(
                        b"configfile.c\0" as *const u8 as *const libc::c_char,
                        1483 as libc::c_int as libc::c_uint,
                        b"srv->match_data\0" as *const u8 as *const libc::c_char,
                    );
                }
            }
            (*dc_0).match_data = (*srv).match_data as *mut pcre2_real_match_data_8;
        }
        i_1 = i_1.wrapping_add(1);
        i_1;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn config_print_indent(mut b: *mut buffer, mut depth: libc::c_int) {
    depth <<= 2 as libc::c_int;
    memset(
        buffer_extend(b, depth as size_t) as *mut libc::c_void,
        ' ' as i32,
        depth as libc::c_ulong,
    );
}
unsafe extern "C" fn config_print_array_max_klen(a: *const array) -> uint32_t {
    let mut maxlen: uint32_t = 0 as libc::c_int as uint32_t;
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*a).used {
        let mut len: uint32_t = buffer_clen(&mut (**((*a).data).offset(i as isize)).key);
        if maxlen < len {
            maxlen = len;
        }
        i = i.wrapping_add(1);
        i;
    }
    return maxlen;
}
unsafe extern "C" fn config_print_array(
    a: *const array,
    b: *mut buffer,
    mut depth: libc::c_int,
) {
    if (*a).used <= 5 as libc::c_int as libc::c_uint
        && ((*a).used == 0
            || buffer_is_unset(
                &mut (**((*a).data).offset(0 as libc::c_int as isize)).key,
            ) != 0)
    {
        let mut oneline: libc::c_int = 1 as libc::c_int;
        let mut i: uint32_t = 0 as libc::c_int as uint32_t;
        while i < (*a).used {
            let mut du: *mut data_unset = *((*a).data).offset(i as isize);
            if (*du).type_0 as libc::c_uint != TYPE_STRING as libc::c_int as libc::c_uint
                && (*du).type_0 as libc::c_uint
                    != TYPE_INTEGER as libc::c_int as libc::c_uint
            {
                oneline = 0 as libc::c_int;
                break;
            } else {
                i = i.wrapping_add(1);
                i;
            }
        }
        if oneline != 0 {
            buffer_append_string(b, b"(\0" as *const u8 as *const libc::c_char);
            let mut i_0: uint32_t = 0 as libc::c_int as uint32_t;
            while i_0 < (*a).used {
                if i_0 != 0 as libc::c_int as libc::c_uint {
                    buffer_append_string(b, b", \0" as *const u8 as *const libc::c_char);
                }
                config_print_by_type(
                    *((*a).data).offset(i_0 as isize),
                    b,
                    depth + 1 as libc::c_int,
                );
                i_0 = i_0.wrapping_add(1);
                i_0;
            }
            buffer_append_string(b, b")\0" as *const u8 as *const libc::c_char);
            return;
        }
    }
    let maxlen: uint32_t = config_print_array_max_klen(a);
    buffer_append_string(b, b"(\n\0" as *const u8 as *const libc::c_char);
    let mut i_1: uint32_t = 0 as libc::c_int as uint32_t;
    while i_1 < (*a).used {
        config_print_indent(b, depth + 1 as libc::c_int);
        let mut du_0: *mut data_unset = *((*a).data).offset(i_1 as isize);
        if buffer_is_unset(&mut (*du_0).key) == 0 {
            buffer_append_str3(
                b,
                b"\"\0" as *const u8 as *const libc::c_char,
                (::core::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                (*du_0).key.ptr,
                buffer_clen(&mut (*du_0).key) as size_t,
                b"\"\0" as *const u8 as *const libc::c_char,
                (::core::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
            let mut indent: libc::c_int = maxlen
                .wrapping_sub(buffer_clen(&mut (*du_0).key)) as libc::c_int;
            if indent > 0 as libc::c_int {
                memset(
                    buffer_extend(b, indent as size_t) as *mut libc::c_void,
                    ' ' as i32,
                    indent as libc::c_ulong,
                );
            }
            buffer_append_string(b, b" => \0" as *const u8 as *const libc::c_char);
        }
        config_print_by_type(du_0, b, depth + 1 as libc::c_int);
        buffer_append_string(b, b",\n\0" as *const u8 as *const libc::c_char);
        i_1 = i_1.wrapping_add(1);
        i_1;
    }
    config_print_indent(b, depth);
    buffer_append_string(b, b")\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn config_print_config(
    mut d: *const data_unset,
    b: *mut buffer,
    mut depth: libc::c_int,
) {
    let mut dc: *mut data_config = d as *mut data_config;
    let mut a: *mut array = (*dc).value;
    if 0 as libc::c_int == (*dc).context_ndx {
        buffer_append_string(b, b"config {\n\0" as *const u8 as *const libc::c_char);
    } else {
        if (*dc).cond as libc::c_uint != CONFIG_COND_ELSE as libc::c_int as libc::c_uint
        {
            buffer_append_string(b, b"if \0" as *const u8 as *const libc::c_char);
            buffer_append_string(b, (*dc).comp_key);
            buffer_append_string(b, b" \0" as *const u8 as *const libc::c_char);
        }
        buffer_append_string(b, b"{\n\0" as *const u8 as *const libc::c_char);
        config_print_indent(b, depth + 1 as libc::c_int);
        buffer_append_string(b, b"# block \0" as *const u8 as *const libc::c_char);
        buffer_append_int(b, (*dc).context_ndx as intmax_t);
        buffer_append_string(b, b"\n\0" as *const u8 as *const libc::c_char);
    }
    depth += 1;
    depth;
    let maxlen: uint32_t = config_print_array_max_klen(a);
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*a).used {
        config_print_indent(b, depth);
        let mut du: *mut data_unset = *((*a).data).offset(i as isize);
        buffer_append_buffer(b, &mut (*du).key);
        let mut indent: libc::c_int = maxlen.wrapping_sub(buffer_clen(&mut (*du).key))
            as libc::c_int;
        if indent > 0 as libc::c_int {
            memset(
                buffer_extend(b, indent as size_t) as *mut libc::c_void,
                ' ' as i32,
                indent as libc::c_ulong,
            );
        }
        buffer_append_string(b, b" = \0" as *const u8 as *const libc::c_char);
        config_print_by_type(du, b, depth);
        buffer_append_string(b, b"\n\0" as *const u8 as *const libc::c_char);
        i = i.wrapping_add(1);
        i;
    }
    buffer_append_string(b, b"\n\0" as *const u8 as *const libc::c_char);
    let mut i_0: uint32_t = 0 as libc::c_int as uint32_t;
    while i_0 < (*dc).children.used {
        let mut dcc: *mut data_config = *((*dc).children.data).offset(i_0 as isize);
        if ((*dcc).prev).is_null() {
            buffer_append_string(b, b"\n\0" as *const u8 as *const libc::c_char);
            config_print_indent(b, depth);
            config_print_by_type(dcc as *mut data_unset, b, depth);
            buffer_append_string(b, b"\n\0" as *const u8 as *const libc::c_char);
        }
        i_0 = i_0.wrapping_add(1);
        i_0;
    }
    depth -= 1;
    depth;
    config_print_indent(b, depth);
    buffer_append_string(b, b"}\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int != (*dc).context_ndx {
        buffer_append_string(b, b" # end of \0" as *const u8 as *const libc::c_char);
        buffer_append_string(
            b,
            if (*dc).cond as libc::c_uint
                != CONFIG_COND_ELSE as libc::c_int as libc::c_uint
            {
                (*dc).comp_key
            } else {
                b"else\0" as *const u8 as *const libc::c_char
            },
        );
    }
    if !((*dc).next).is_null() {
        buffer_append_string(b, b"\n\0" as *const u8 as *const libc::c_char);
        config_print_indent(b, depth);
        buffer_append_string(b, b"else \0" as *const u8 as *const libc::c_char);
        config_print_by_type((*dc).next as *mut data_unset, b, depth);
    }
}
unsafe extern "C" fn config_print_string(mut du: *const data_unset, b: *mut buffer) {
    let vb: *const buffer = &mut (*(du as *mut data_string)).value;
    let mut dst: *mut libc::c_char = buffer_string_prepare_append(
        b,
        (buffer_clen(vb)).wrapping_mul(2 as libc::c_int as libc::c_uint) as size_t,
    );
    let mut n: uint32_t = 0 as libc::c_int as uint32_t;
    let fresh2 = n;
    n = n.wrapping_add(1);
    *dst.offset(fresh2 as isize) = '"' as i32 as libc::c_char;
    if !((*vb).ptr).is_null() {
        let mut p: *const libc::c_char = (*vb).ptr;
        while *p != 0 {
            if *p as libc::c_int == '"' as i32 {
                let fresh3 = n;
                n = n.wrapping_add(1);
                *dst.offset(fresh3 as isize) = '\\' as i32 as libc::c_char;
            }
            let fresh4 = n;
            n = n.wrapping_add(1);
            *dst.offset(fresh4 as isize) = *p;
            p = p.offset(1);
            p;
        }
    }
    let fresh5 = n;
    n = n.wrapping_add(1);
    *dst.offset(fresh5 as isize) = '"' as i32 as libc::c_char;
    buffer_commit(b, n as size_t);
}
#[cold]
unsafe extern "C" fn config_print_by_type(
    du: *const data_unset,
    b: *mut buffer,
    mut depth: libc::c_int,
) {
    match (*du).type_0 as libc::c_uint {
        0 => {
            config_print_string(du, b);
        }
        2 => {
            buffer_append_int(b, (*(du as *mut data_integer)).value as intmax_t);
        }
        1 => {
            config_print_array(&mut (*(du as *mut data_array)).value, b, depth);
        }
        3 => {
            config_print_config(du, b, depth);
        }
        _ => {}
    };
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn config_print(mut srv: *mut server) {
    buffer_clear((*srv).tmp_buf);
    let mut dc: *mut data_unset = *((*(*srv).config_context).data)
        .offset(0 as libc::c_int as isize);
    config_print_by_type(dc, (*srv).tmp_buf, 0 as libc::c_int);
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn config_free(mut srv: *mut server) {
    config_free_config((*srv).config_data_base);
    array_free((*srv).config_context);
    array_free((*srv).srvconf.config_touched);
    array_free((*srv).srvconf.modules);
    array_free((*srv).srvconf.upload_tempdirs);
    array_free_data(&mut (*srv).srvconf.mimetypes_default);
    if ((*srv).match_data).is_null() {
        pcre2_match_data_free_8((*srv).match_data as *mut pcre2_match_data_8);
    }
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn config_init(mut srv: *mut server) {
    (*srv).config_context = array_init(16 as libc::c_int as uint32_t);
    (*srv).srvconf.config_touched = array_init(128 as libc::c_int as uint32_t);
    (*srv).srvconf.port = 0 as libc::c_int as libc::c_ushort;
    (*srv).srvconf.dont_daemonize = 0 as libc::c_int as libc::c_uchar;
    (*srv).srvconf.preflight_check = 0 as libc::c_int as libc::c_uchar;
    (*srv).srvconf.compat_module_load = 1 as libc::c_int as libc::c_uchar;
    (*srv).srvconf.systemd_socket_activation = 0 as libc::c_int as libc::c_uchar;
    (*srv).srvconf.high_precision_timestamps = 0 as libc::c_int as libc::c_uchar;
    (*srv).srvconf.max_request_field_size = 8192 as libc::c_int as uint32_t;
    (*srv).srvconf.http_header_strict = 1 as libc::c_int as libc::c_uchar;
    (*srv).srvconf.http_host_strict = 1 as libc::c_int as libc::c_uchar;
    (*srv).srvconf.http_host_normalize = 0 as libc::c_int as libc::c_uchar;
    (*srv)
        .srvconf
        .http_url_normalize = (HTTP_PARSEOPT_URL_NORMALIZE as libc::c_int
        | HTTP_PARSEOPT_URL_NORMALIZE_UNRESERVED as libc::c_int
        | HTTP_PARSEOPT_URL_NORMALIZE_CTRLS_REJECT as libc::c_int
        | HTTP_PARSEOPT_URL_NORMALIZE_PATH_2F_DECODE as libc::c_int
        | HTTP_PARSEOPT_URL_NORMALIZE_PATH_DOTSEG_REMOVE as libc::c_int
        | HTTP_PARSEOPT_URL_NORMALIZE_INVALID_UTF8_REJECT as libc::c_int)
        as libc::c_ushort;
    (*srv).srvconf.modules = array_init(16 as libc::c_int as uint32_t);
    (*srv).srvconf.modules_dir = b"/usr/local/lib\0" as *const u8 as *const libc::c_char;
    (*srv).srvconf.upload_tempdirs = array_init(2 as libc::c_int as uint32_t);
}
unsafe extern "C" fn config_log_error_open_syslog(
    mut srv: *mut server,
    mut errh: *mut log_error_st,
    mut syslog_facility: *const buffer,
) {
    (*errh).mode = FDLOG_SYSLOG;
    (*errh).fd = -(1 as libc::c_int);
    fdlog_openlog((*srv).errh, syslog_facility);
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn config_log_error_open(mut srv: *mut server) -> libc::c_int {
    let p: *mut config_data_base = (*srv).config_data_base as *mut config_data_base;
    let mut serrh: *mut log_error_st = 0 as *mut log_error_st;
    let mut i: libc::c_int = ((*((*p).cvlist).offset(0 as libc::c_int as isize))
        .v
        .u2[1 as libc::c_int as usize] == 0) as libc::c_int;
    while i < (*p).nconfig {
        let mut cpv: *mut config_plugin_value_t = ((*p).cvlist)
            .offset(
                (*((*p).cvlist).offset(i as isize)).v.u2[0 as libc::c_int as usize]
                    as isize,
            );
        let mut current_block_15: u64;
        while -(1 as libc::c_int) != (*cpv).k_id {
            let mut fn_0: *const libc::c_char = 0 as *const libc::c_char;
            let mut errh: *mut log_error_st = 0 as *mut log_error_st;
            match (*cpv).k_id {
                32 => {
                    if 0 as libc::c_int == i {
                        if (*srv).srvconf.errorlog_use_syslog != 0 {
                            current_block_15 = 16658872821858055392;
                        } else {
                            errh = (*srv).errh;
                            current_block_15 = 1505205890512701289;
                        }
                    } else {
                        current_block_15 = 1505205890512701289;
                    }
                }
                33 => {
                    current_block_15 = 1505205890512701289;
                }
                _ => {
                    current_block_15 = 1856101646708284338;
                }
            }
            match current_block_15 {
                1505205890512701289 => {
                    if buffer_is_blank((*cpv).v.b) == 0 {
                        fn_0 = (*(*cpv).v.b).ptr;
                    }
                    current_block_15 = 1856101646708284338;
                }
                _ => {}
            }
            match current_block_15 {
                1856101646708284338 => {
                    if !fn_0.is_null() {
                        let fdlog: *mut fdlog_st = fdlog_open(fn_0);
                        if fdlog.is_null() {
                            log_perror(
                                (*srv).errh,
                                b"configfile.c\0" as *const u8 as *const libc::c_char,
                                1774 as libc::c_int as libc::c_uint,
                                b"opening errorlog '%s' failed\0" as *const u8
                                    as *const libc::c_char,
                                fn_0,
                            );
                            return -(1 as libc::c_int);
                        }
                        if !errh.is_null() {
                            (*srv).errh = fdlog;
                            (*p).defaults.errh = (*srv).errh;
                            log_set_global_errh((*srv).errh, 0 as libc::c_int);
                        }
                        errh = fdlog;
                        (*cpv).v.v = errh as *mut libc::c_void;
                        (*cpv).vtype = T_CONFIG_LOCAL;
                        if 0 as libc::c_int == i && errh != (*srv).errh {
                            serrh = errh;
                        }
                    }
                }
                _ => {}
            }
            cpv = cpv.offset(1);
            cpv;
        }
        i += 1;
        i;
    }
    if config_feature_bool(
        srv,
        b"server.errorlog-high-precision\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    ) != 0
    {
        log_set_global_errh((*srv).errh, 1 as libc::c_int);
    }
    if (*srv).srvconf.errorlog_use_syslog != 0 {
        config_log_error_open_syslog(srv, (*srv).errh, (*srv).srvconf.syslog_facility);
    } else if (*(*srv).errh).mode as libc::c_uint
        == FDLOG_FD as libc::c_int as libc::c_uint && (*srv).srvconf.dont_daemonize == 0
    {
        (*(*srv).errh).fd = -(1 as libc::c_int);
    }
    let mut errfd: libc::c_int = 0;
    if !serrh.is_null() {
        if (*(*srv).errh).mode as libc::c_uint == FDLOG_FD as libc::c_int as libc::c_uint
        {
            (*(*srv).errh).fd = dup(2 as libc::c_int);
            fdevent_setfd_cloexec((*(*srv).errh).fd);
        }
        errfd = (*serrh).fd;
        if *(*serrh).fn_0 as libc::c_int == '|' as i32 {
            fdlog_pipe_serrh(errfd);
        }
    } else if (*srv).srvconf.dont_daemonize == 0 {
        errfd = fdevent_open_devnull();
        if -(1 as libc::c_int) == errfd {
            log_perror(
                (*srv).errh,
                b"configfile.c\0" as *const u8 as *const libc::c_char,
                1825 as libc::c_int as libc::c_uint,
                b"opening /dev/null failed\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    } else {
        errfd = -(1 as libc::c_int);
    }
    if 0 as libc::c_int
        != fdevent_set_stdin_stdout_stderr(
            -(1 as libc::c_int),
            -(1 as libc::c_int),
            errfd,
        )
    {
        log_perror(
            (*srv).errh,
            b"configfile.c\0" as *const u8 as *const libc::c_char,
            1847 as libc::c_int as libc::c_uint,
            b"setting stderr failed\0" as *const u8 as *const libc::c_char,
        );
        if -(1 as libc::c_int) != errfd && serrh.is_null() {
            close(errfd);
        }
        return -(1 as libc::c_int);
    }
    if -(1 as libc::c_int) != errfd && serrh.is_null() {
        close(errfd);
    }
    if !serrh.is_null() {
        close(errfd);
        (*serrh).fd = 2 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn config_log_error_close(mut srv: *mut server) {
    let p: *mut config_data_base = (*srv).config_data_base as *mut config_data_base;
    if p.is_null() {
        return;
    }
    (*p).defaults.serrh = 0 as *mut fdlog_st;
    fdlog_closeall((*srv).errh);
    if (*(*srv).errh).mode as libc::c_uint == FDLOG_SYSLOG as libc::c_int as libc::c_uint
    {
        (*(*srv).errh).mode = FDLOG_FD;
        (*(*srv).errh).fd = 2 as libc::c_int;
    }
    fdlog_closelog();
}
unsafe extern "C" fn config_skip_newline(t: *const tokenizer_t) -> libc::c_int {
    let s: *const libc::c_char = ((*t).input).offset((*t).offset as isize);
    return 1 as libc::c_int
        + (*s.offset(0 as libc::c_int as isize) as libc::c_int == '\r' as i32
            && *s.offset(1 as libc::c_int as isize) as libc::c_int == '\n' as i32)
            as libc::c_int;
}
unsafe extern "C" fn config_skip_comment(t: *const tokenizer_t) -> libc::c_int {
    let mut s: *const libc::c_char = ((*t).input).offset((*t).offset as isize);
    loop {
        s = s.offset(1);
        s;
        if !(*s as libc::c_int != 0 && *s as libc::c_int != '\r' as i32
            && *s as libc::c_int != '\n' as i32)
        {
            break;
        }
    }
    return s.offset_from((*t).input) as libc::c_long as libc::c_int;
}
#[cold]
unsafe extern "C" fn config_tokenizer_err(
    mut t: *mut tokenizer_t,
    mut file: *const libc::c_char,
    mut line: libc::c_uint,
    mut msg: *const libc::c_char,
) -> libc::c_int {
    log_error(
        (*t).errh,
        file,
        line,
        b"source: %s line: %d pos: %d %s\0" as *const u8 as *const libc::c_char,
        (*t).source,
        (*t).line,
        (*t).offset - (*t).line_pos,
        msg,
    );
    return -(1 as libc::c_int);
}
#[inline(never)]
unsafe extern "C" fn config_tokenizer(mut t: *mut tokenizer_t) -> libc::c_int {
    let token: *mut buffer = (*t).token;
    if (*t).simulate_eol != 0 {
        (*t).simulate_eol = 0 as libc::c_int as libc::c_char;
        (*t).in_key = 1 as libc::c_int as libc::c_char;
        buffer_copy_string_len(
            token,
            b"(EOL)\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
        return 1 as libc::c_int;
    }
    while (*t).offset < (*t).size {
        let s: *const libc::c_char = ((*t).input).offset((*t).offset as isize);
        match *s.offset(0 as libc::c_int as isize) as libc::c_int {
            9 | 32 => {
                (*t).offset += 1;
                (*t).offset;
            }
            61 => {
                if (*t).parens != 0 {
                    if *s.offset(1 as libc::c_int as isize) as libc::c_int != '>' as i32
                    {
                        return config_tokenizer_err(
                            t,
                            b"configfile.c\0" as *const u8 as *const libc::c_char,
                            1945 as libc::c_int as libc::c_uint,
                            b"use => for assignments in arrays\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    (*t).offset += 2 as libc::c_int;
                    buffer_copy_string_len(token, s, 2 as libc::c_int as size_t);
                    return 12 as libc::c_int;
                } else if (*t).in_cond != 0 {
                    let mut tid: libc::c_int = 0;
                    match *s.offset(1 as libc::c_int as isize) as libc::c_int {
                        61 => {
                            tid = 21 as libc::c_int;
                        }
                        126 => {
                            tid = 22 as libc::c_int;
                        }
                        94 => {
                            tid = 25 as libc::c_int;
                        }
                        36 => {
                            tid = 26 as libc::c_int;
                        }
                        _ => {
                            return config_tokenizer_err(
                                t,
                                b"configfile.c\0" as *const u8 as *const libc::c_char,
                                1959 as libc::c_int as libc::c_uint,
                                b"only == =~ =^ =$ are allowed in the condition\0"
                                    as *const u8 as *const libc::c_char,
                            );
                        }
                    }
                    (*t).offset += 2 as libc::c_int;
                    (*t).in_key = 1 as libc::c_int as libc::c_char;
                    (*t).in_cond = 0 as libc::c_int as libc::c_char;
                    buffer_copy_string_len(token, s, 2 as libc::c_int as size_t);
                    return tid;
                } else if (*t).in_key != 0 {
                    (*t).offset += 1;
                    (*t).offset;
                    buffer_copy_string_len(token, s, 1 as libc::c_int as size_t);
                    return 2 as libc::c_int;
                } else {
                    return config_tokenizer_err(
                        t,
                        b"configfile.c\0" as *const u8 as *const libc::c_char,
                        1974 as libc::c_int as libc::c_uint,
                        b"unexpected equal-sign: =\0" as *const u8 as *const libc::c_char,
                    )
                }
            }
            33 => {
                if (*t).in_cond != 0 {
                    let mut tid_0: libc::c_int = 0;
                    match *s.offset(1 as libc::c_int as isize) as libc::c_int {
                        61 => {
                            tid_0 = 23 as libc::c_int;
                        }
                        126 => {
                            tid_0 = 24 as libc::c_int;
                        }
                        _ => {
                            return config_tokenizer_err(
                                t,
                                b"configfile.c\0" as *const u8 as *const libc::c_char,
                                1983 as libc::c_int as libc::c_uint,
                                b"only !~ and != are allowed in the condition\0"
                                    as *const u8 as *const libc::c_char,
                            );
                        }
                    }
                    (*t).offset += 2 as libc::c_int;
                    (*t).in_key = 1 as libc::c_int as libc::c_char;
                    (*t).in_cond = 0 as libc::c_int as libc::c_char;
                    buffer_copy_string_len(token, s, 2 as libc::c_int as size_t);
                    return tid_0;
                } else {
                    return config_tokenizer_err(
                        t,
                        b"configfile.c\0" as *const u8 as *const libc::c_char,
                        1993 as libc::c_int as libc::c_uint,
                        b"unexpected exclamation-marks: !\0" as *const u8
                            as *const libc::c_char,
                    )
                }
            }
            10 | 13 => {
                loop {
                    match *((*t).input).offset((*t).offset as isize) as libc::c_int {
                        13 | 10 => {
                            (*t).offset += config_skip_newline(t);
                            (*t).line_pos = (*t).offset;
                            (*t).line += 1;
                            (*t).line;
                        }
                        35 => {
                            (*t).offset = config_skip_comment(t);
                        }
                        9 | 32 => {
                            (*t).offset += 1;
                            (*t).offset;
                        }
                        _ => {
                            break;
                        }
                    }
                    if !((*t).offset < (*t).size) {
                        break;
                    }
                }
                if (*t).parens == 0 {
                    (*t).in_key = 1 as libc::c_int as libc::c_char;
                    buffer_copy_string_len(
                        token,
                        b"(EOL)\0" as *const u8 as *const libc::c_char,
                        (::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                    );
                    return 1 as libc::c_int;
                }
            }
            44 => {
                (*t).offset += 1;
                (*t).offset;
                if (*t).parens != 0 {
                    buffer_copy_string_len(
                        token,
                        b"(COMMA)\0" as *const u8 as *const libc::c_char,
                        (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                    );
                    return 11 as libc::c_int;
                }
            }
            34 => {
                if (*t).tid == 7 as libc::c_int {
                    return config_tokenizer_err(
                        t,
                        b"configfile.c\0" as *const u8 as *const libc::c_char,
                        2034 as libc::c_int as libc::c_uint,
                        b"strings may be combined with '+' or separated with ',' or '=>' in lists\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
                let mut start: *const libc::c_char = s.offset(1 as libc::c_int as isize);
                buffer_copy_string_len(
                    token,
                    b"\0" as *const u8 as *const libc::c_char,
                    (::core::mem::size_of::<[libc::c_char; 1]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                );
                let mut i: libc::c_int = 0;
                i = 1 as libc::c_int;
                while *s.offset(i as isize) as libc::c_int != 0
                    && *s.offset(i as isize) as libc::c_int != '"' as i32
                {
                    if *s.offset(i as isize) as libc::c_int == '\\' as i32
                        && *s.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                            == '"' as i32
                    {
                        buffer_append_string_len(
                            token,
                            start,
                            s.offset(i as isize).offset_from(start) as libc::c_long
                                as size_t,
                        );
                        i += 1;
                        start = s.offset(i as isize);
                    }
                    i += 1;
                    i;
                }
                if *s.offset(i as isize) as libc::c_int == '\0' as i32 {
                    return config_tokenizer_err(
                        t,
                        b"configfile.c\0" as *const u8 as *const libc::c_char,
                        2051 as libc::c_int as libc::c_uint,
                        b"missing closing quote\0" as *const u8 as *const libc::c_char,
                    );
                }
                (*t).offset += i + 1 as libc::c_int;
                buffer_append_string_len(
                    token,
                    start,
                    s.offset(i as isize).offset_from(start) as libc::c_long as size_t,
                );
                return 7 as libc::c_int;
            }
            40 => {
                (*t).offset += 1;
                (*t).offset;
                (*t).parens += 1;
                (*t).parens;
                buffer_copy_string_len(token, s, 1 as libc::c_int as size_t);
                return 9 as libc::c_int;
            }
            41 => {
                if (*t).parens == 0 {
                    return config_tokenizer_err(
                        t,
                        b"configfile.c\0" as *const u8 as *const libc::c_char,
                        2066 as libc::c_int as libc::c_uint,
                        b"close-parens without open-parens\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                (*t).offset += 1;
                (*t).offset;
                (*t).parens -= 1;
                (*t).parens;
                buffer_copy_string_len(token, s, 1 as libc::c_int as size_t);
                return 10 as libc::c_int;
            }
            36 => {
                (*t).offset += 1;
                (*t).offset;
                (*t).in_cond = 1 as libc::c_int as libc::c_char;
                (*t).in_key = 0 as libc::c_int as libc::c_char;
                buffer_copy_string_len(token, s, 1 as libc::c_int as size_t);
                return 17 as libc::c_int;
            }
            43 => {
                if *s.offset(1 as libc::c_int as isize) as libc::c_int == '=' as i32 {
                    (*t).offset += 2 as libc::c_int;
                    buffer_copy_string_len(token, s, 2 as libc::c_int as size_t);
                    return 4 as libc::c_int;
                } else {
                    (*t).offset += 1;
                    (*t).offset;
                    buffer_copy_string_len(token, s, 1 as libc::c_int as size_t);
                    return 6 as libc::c_int;
                }
            }
            58 => {
                if *s.offset(1 as libc::c_int as isize) as libc::c_int != '=' as i32 {
                    return config_tokenizer_err(
                        t,
                        b"configfile.c\0" as *const u8 as *const libc::c_char,
                        2091 as libc::c_int as libc::c_uint,
                        b"unexpected character ':'\0" as *const u8 as *const libc::c_char,
                    );
                }
                (*t).offset += 2 as libc::c_int;
                buffer_copy_string_len(token, s, 2 as libc::c_int as size_t);
                return 3 as libc::c_int;
            }
            123 => {
                (*t).offset += 1;
                (*t).offset;
                buffer_copy_string_len(token, s, 1 as libc::c_int as size_t);
                return 14 as libc::c_int;
            }
            125 => {
                loop {
                    (*t).offset += 1;
                    if !((*t).offset < (*t).size) {
                        break;
                    }
                    let mut c: libc::c_int = *((*t).input).offset((*t).offset as isize)
                        as libc::c_int;
                    if c == '\r' as i32 || c == '\n' as i32 {
                        break;
                    }
                    if c == '#' as i32 {
                        (*t).offset = config_skip_comment(t);
                        break;
                    } else {
                        if !(c != ' ' as i32 && c != '\t' as i32) {
                            continue;
                        }
                        (*t).simulate_eol = 1 as libc::c_int as libc::c_char;
                        break;
                    }
                }
                buffer_copy_string_len(token, s, 1 as libc::c_int as size_t);
                return 15 as libc::c_int;
            }
            91 => {
                (*t).offset += 1;
                (*t).offset;
                buffer_copy_string_len(token, s, 1 as libc::c_int as size_t);
                return 19 as libc::c_int;
            }
            93 => {
                (*t).offset += 1;
                (*t).offset;
                buffer_copy_string_len(token, s, 1 as libc::c_int as size_t);
                return 20 as libc::c_int;
            }
            35 => {
                (*t).offset = config_skip_comment(t);
            }
            0 => {
                config_tokenizer_err(
                    t,
                    b"configfile.c\0" as *const u8 as *const libc::c_char,
                    2129 as libc::c_int as libc::c_uint,
                    b"stray NUL\0" as *const u8 as *const libc::c_char,
                );
                return 0 as libc::c_int;
            }
            _ => {
                if (*t).in_cond != 0 {
                    let mut i_0: libc::c_int = 0 as libc::c_int;
                    while light_isalpha(*s.offset(i_0 as isize) as libc::c_int) != 0
                        || *s.offset(i_0 as isize) as libc::c_int == '_' as i32
                    {
                        i_0 += 1;
                        i_0;
                    }
                    if i_0 != 0 && *s.offset(i_0 as isize) as libc::c_int != 0 {
                        (*t).offset += i_0;
                        buffer_copy_string_len(token, s, i_0 as size_t);
                        return 18 as libc::c_int;
                    } else {
                        return config_tokenizer_err(
                            t,
                            b"configfile.c\0" as *const u8 as *const libc::c_char,
                            2141 as libc::c_int as libc::c_uint,
                            b"invalid character in condition\0" as *const u8
                                as *const libc::c_char,
                        )
                    }
                } else if light_isdigit(
                    *s.offset(0 as libc::c_int as isize) as libc::c_int,
                ) != 0
                {
                    let mut i_1: libc::c_int = 1 as libc::c_int;
                    while light_isdigit(*s.offset(i_1 as isize) as libc::c_int) != 0 {
                        i_1 += 1;
                        i_1;
                    }
                    (*t).offset += i_1;
                    buffer_copy_string_len(token, s, i_1 as size_t);
                    return 8 as libc::c_int;
                } else {
                    let mut i_2: libc::c_int = 0 as libc::c_int;
                    while light_isalnum(*s.offset(i_2 as isize) as libc::c_int) != 0
                        || *s.offset(i_2 as isize) as libc::c_int == '.' as i32
                        || *s.offset(i_2 as isize) as libc::c_int == '_' as i32
                        || *s.offset(i_2 as isize) as libc::c_int == '-' as i32
                    {
                        i_2 += 1;
                        i_2;
                    }
                    if i_2 != 0 && *s.offset(i_2 as isize) as libc::c_int != 0 {
                        (*t).offset += i_2;
                        buffer_copy_string_len(token, s, i_2 as size_t);
                        if 0 as libc::c_int
                            == strcmp(
                                (*token).ptr,
                                b"include\0" as *const u8 as *const libc::c_char,
                            )
                        {
                            return 27 as libc::c_int
                        } else if 0 as libc::c_int
                            == strcmp(
                                (*token).ptr,
                                b"include_shell\0" as *const u8 as *const libc::c_char,
                            )
                        {
                            return 28 as libc::c_int
                        } else if 0 as libc::c_int
                            == strcmp(
                                (*token).ptr,
                                b"global\0" as *const u8 as *const libc::c_char,
                            )
                        {
                            return 13 as libc::c_int
                        } else if 0 as libc::c_int
                            == strcmp(
                                (*token).ptr,
                                b"if\0" as *const u8 as *const libc::c_char,
                            )
                        {
                            let mut j: libc::c_int = i_2;
                            while *s.offset(j as isize) as libc::c_int == ' ' as i32
                                || *s.offset(j as isize) as libc::c_int == '\t' as i32
                            {
                                j += 1;
                                j;
                            }
                            *s.offset(j as isize) as libc::c_int == '$' as i32;
                        } else if 0 as libc::c_int
                            == strcmp(
                                (*token).ptr,
                                b"elif\0" as *const u8 as *const libc::c_char,
                            )
                            || 0 as libc::c_int
                                == strcmp(
                                    (*token).ptr,
                                    b"elsif\0" as *const u8 as *const libc::c_char,
                                )
                            || 0 as libc::c_int
                                == strcmp(
                                    (*token).ptr,
                                    b"elseif\0" as *const u8 as *const libc::c_char,
                                )
                        {
                            return 16 as libc::c_int
                        } else if 0 as libc::c_int
                            == strcmp(
                                (*token).ptr,
                                b"else\0" as *const u8 as *const libc::c_char,
                            )
                        {
                            return 16 as libc::c_int
                        } else {
                            return 5 as libc::c_int
                        }
                    } else if 0 as libc::c_int == i_2
                        && *(s as *mut uint8_t).offset(0 as libc::c_int as isize)
                            as libc::c_int == 0xc2 as libc::c_int
                        && *(s as *mut uint8_t).offset(1 as libc::c_int as isize)
                            as libc::c_int == 0xa0 as libc::c_int
                    {
                        (*t).offset += 2 as libc::c_int;
                    } else {
                        return config_tokenizer_err(
                            t,
                            b"configfile.c\0" as *const u8 as *const libc::c_char,
                            2194 as libc::c_int as libc::c_uint,
                            b"invalid character in variable name\0" as *const u8
                                as *const libc::c_char,
                        )
                    }
                }
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn config_parse(
    mut srv: *mut server,
    mut context: *mut config_t,
    mut source: *const libc::c_char,
    mut input: *const libc::c_char,
    mut isize: libc::c_int,
) -> libc::c_int {
    let mut t: tokenizer_t = tokenizer_t {
        offset: 0,
        size: 0,
        input: 0 as *const libc::c_char,
        token: 0 as *mut buffer,
        in_key: 0,
        parens: 0,
        in_cond: 0,
        simulate_eol: 0,
        tid: 0,
        line_pos: 0,
        line: 0,
        source: 0 as *const libc::c_char,
        errh: 0 as *mut log_error_st,
    };
    t.source = source;
    t.input = input;
    t.size = isize;
    t.offset = 0 as libc::c_int;
    t.line = 1 as libc::c_int;
    t.line_pos = 0 as libc::c_int;
    t.in_key = 1 as libc::c_int as libc::c_char;
    t.parens = 0 as libc::c_int as libc::c_char;
    t.in_cond = 0 as libc::c_int as libc::c_char;
    t.simulate_eol = 0 as libc::c_int as libc::c_char;
    t.tid = -(1 as libc::c_int);
    t.errh = (*srv).errh;
    t.token = buffer_init();
    let pParser: *mut libc::c_void = configparserAlloc(
        Some(malloc as unsafe extern "C" fn(libc::c_ulong) -> *mut libc::c_void),
    );
    if pParser.is_null() {
        ck_assert_failed(
            b"configfile.c\0" as *const u8 as *const libc::c_char,
            2221 as libc::c_int as libc::c_uint,
            b"pParser\0" as *const u8 as *const libc::c_char,
        );
    }
    while (*context).ok != 0
        && {
            t.tid = config_tokenizer(&mut t);
            t.tid > 0 as libc::c_int
        }
    {
        let token: *mut buffer = buffer_init();
        buffer_copy_buffer(token, t.token);
        configparser(pParser, t.tid, token, context);
    }
    if t.tid != -(1 as libc::c_int) && (*context).ok != 0 {
        let token_0: *mut buffer = buffer_init();
        buffer_copy_string(token_0, b"(EOL)\0" as *const u8 as *const libc::c_char);
        configparser(pParser, 1 as libc::c_int, token_0, context);
        if (*context).ok != 0 {
            configparser(pParser, 0 as libc::c_int, 0 as *mut buffer, context);
        }
    }
    configparserFree(
        pParser,
        Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
    if t.tid == -(1 as libc::c_int) {
        log_error(
            t.errh,
            b"configfile.c\0" as *const u8 as *const libc::c_char,
            2242 as libc::c_int as libc::c_uint,
            b"configfile parser failed at: %s\0" as *const u8 as *const libc::c_char,
            (*t.token).ptr,
        );
    } else if (*context).ok == 0 as libc::c_int {
        log_error(
            t.errh,
            b"configfile.c\0" as *const u8 as *const libc::c_char,
            2245 as libc::c_int as libc::c_uint,
            b"source: %s line: %d pos: %d parser failed somehow near here: %s\0"
                as *const u8 as *const libc::c_char,
            t.source,
            t.line,
            t.offset - t.line_pos,
            (*t.token).ptr,
        );
        t.tid = -(1 as libc::c_int);
    }
    buffer_free(t.token);
    return if t.tid == -(1 as libc::c_int) {
        -(1 as libc::c_int)
    } else {
        0 as libc::c_int
    };
}
#[cold]
unsafe extern "C" fn config_parse_stdin(
    mut srv: *mut server,
    mut context: *mut config_t,
) -> libc::c_int {
    static mut config_mem: *mut libc::c_char = 0 as *const libc::c_char
        as *mut libc::c_char;
    static mut lim: off_t = 0;
    if config_mem.is_null() {
        lim = (32 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int) as off_t;
        config_mem = fdevent_load_file(
            b"/dev/stdin\0" as *const u8 as *const libc::c_char,
            &mut lim,
            (*srv).errh,
            Some(malloc as unsafe extern "C" fn(libc::c_ulong) -> *mut libc::c_void),
            Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        );
        if config_mem.is_null() {
            log_perror(
                (*srv).errh,
                b"configfile.c\0" as *const u8 as *const libc::c_char,
                2267 as libc::c_int as libc::c_uint,
                b"config read from stdin\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        let mut fd: libc::c_int = fdevent_open_devnull();
        if fd > 0 as libc::c_int {
            close(fd);
        }
    }
    return if lim != 0 {
        config_parse(
            srv,
            context,
            b"-\0" as *const u8 as *const libc::c_char,
            config_mem,
            lim as libc::c_int,
        )
    } else {
        0 as libc::c_int
    };
}
unsafe extern "C" fn config_parse_file_stream(
    mut srv: *mut server,
    mut context: *mut config_t,
    mut fn_0: *const libc::c_char,
) -> libc::c_int {
    let mut dlen: off_t = (32 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int)
        as off_t;
    let mut data: *mut libc::c_char = fdevent_load_file(
        fn_0,
        &mut dlen,
        0 as *mut log_error_st,
        Some(malloc as unsafe extern "C" fn(libc::c_ulong) -> *mut libc::c_void),
        Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
    if data.is_null() {
        log_perror(
            (*srv).errh,
            b"configfile.c\0" as *const u8 as *const libc::c_char,
            2285 as libc::c_int as libc::c_uint,
            b"opening configfile %s failed\0" as *const u8 as *const libc::c_char,
            fn_0,
        );
        return -(1 as libc::c_int);
    }
    let mut rc: libc::c_int = 0 as libc::c_int;
    if dlen != 0 {
        rc = config_parse(srv, context, fn_0, data, dlen as libc::c_int);
        ck_memzero(data as *mut libc::c_void, dlen as size_t);
    }
    free(data as *mut libc::c_void);
    return rc;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn config_parse_file(
    mut srv: *mut server,
    mut context: *mut config_t,
    mut fn_0: *const libc::c_char,
) -> libc::c_int {
    let filename: *mut buffer = buffer_init();
    let fnlen: size_t = strlen(fn_0);
    let mut ret: libc::c_int = -(1 as libc::c_int);
    let mut flags: libc::c_int = (1 as libc::c_int) << 10 as libc::c_int;
    let mut gl: glob_t = glob_t {
        gl_pathc: 0,
        gl_pathv: 0 as *mut *mut libc::c_char,
        gl_offs: 0,
        gl_flags: 0,
        gl_closedir: None,
        gl_readdir: None,
        gl_opendir: None,
        gl_lstat: None,
        gl_stat: None,
    };
    if buffer_is_blank((*context).basedir) != 0
        || (*fn_0.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
            || *fn_0.offset(0 as libc::c_int as isize) as libc::c_int == '\\' as i32)
        || *fn_0.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
            && (*fn_0.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
                || *fn_0.offset(1 as libc::c_int as isize) as libc::c_int == '\\' as i32)
        || *fn_0.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
            && *fn_0.offset(1 as libc::c_int as isize) as libc::c_int == '.' as i32
            && (*fn_0.offset(2 as libc::c_int as isize) as libc::c_int == '/' as i32
                || *fn_0.offset(2 as libc::c_int as isize) as libc::c_int == '\\' as i32)
        || light_isalpha(*fn_0.offset(0 as libc::c_int as isize) as libc::c_int) != 0
            && *fn_0.offset(1 as libc::c_int as isize) as libc::c_int == ':' as i32
            && (*fn_0.offset(2 as libc::c_int as isize) as libc::c_int == '/' as i32
                || *fn_0.offset(2 as libc::c_int as isize) as libc::c_int == '\\' as i32)
    {
        buffer_copy_string_len(filename, fn_0, fnlen);
    } else {
        buffer_copy_path_len2(
            filename,
            (*(*context).basedir).ptr,
            buffer_clen((*context).basedir) as size_t,
            fn_0,
            fnlen,
        );
    }
    match glob((*filename).ptr, flags, None, &mut gl) {
        0 => {
            let mut i: size_t = 0 as libc::c_int as size_t;
            while i < gl.gl_pathc {
                ret = config_parse_file_stream(
                    srv,
                    context,
                    *(gl.gl_pathv).offset(i as isize),
                );
                if 0 as libc::c_int != ret {
                    break;
                }
                i = i.wrapping_add(1);
                i;
            }
            globfree(&mut gl);
        }
        3 => {
            if *((*filename).ptr)
                .offset(
                    strcspn(
                        (*filename).ptr,
                        b"*?[]{}\0" as *const u8 as *const libc::c_char,
                    ) as isize,
                ) as libc::c_int != '\0' as i32
            {
                ret = 0 as libc::c_int;
            } else {
                log_error(
                    (*srv).errh,
                    b"configfile.c\0" as *const u8 as *const libc::c_char,
                    2438 as libc::c_int as libc::c_uint,
                    b"include file not found: %s\0" as *const u8 as *const libc::c_char,
                    (*filename).ptr,
                );
            }
        }
        2 | 1 => {
            log_perror(
                (*srv).errh,
                b"configfile.c\0" as *const u8 as *const libc::c_char,
                2443 as libc::c_int as libc::c_uint,
                b"glob() %s failed\0" as *const u8 as *const libc::c_char,
                (*filename).ptr,
            );
        }
        _ => {}
    }
    buffer_free(filename);
    return ret;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn config_parse_cmd(
    mut srv: *mut server,
    mut context: *mut config_t,
    mut cmd: *const libc::c_char,
) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut fds: [libc::c_int; 2] = [0; 2];
    let mut oldpwd: [libc::c_char; 4096] = [0; 4096];
    if (getcwd(
        oldpwd.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
    ))
        .is_null()
    {
        log_perror(
            (*srv).errh,
            b"configfile.c\0" as *const u8 as *const libc::c_char,
            2473 as libc::c_int as libc::c_uint,
            b"getcwd()\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if buffer_is_blank((*context).basedir) == 0 {
        if 0 as libc::c_int != chdir((*(*context).basedir).ptr) {
            log_perror(
                (*srv).errh,
                b"configfile.c\0" as *const u8 as *const libc::c_char,
                2479 as libc::c_int as libc::c_uint,
                b"cannot change directory to %s\0" as *const u8 as *const libc::c_char,
                (*(*context).basedir).ptr,
            );
            return -(1 as libc::c_int);
        }
    }
    if fdevent_pipe_cloexec(fds.as_mut_ptr(), 65536 as libc::c_int as libc::c_uint) != 0
    {
        log_perror(
            (*srv).errh,
            b"configfile.c\0" as *const u8 as *const libc::c_char,
            2486 as libc::c_int as libc::c_uint,
            b"pipe()\0" as *const u8 as *const libc::c_char,
        );
        ret = -(1 as libc::c_int);
    } else {
        let mut pid: pid_t = fdevent_sh_exec(
            cmd,
            0 as *mut *mut libc::c_char,
            -(1 as libc::c_int),
            fds[1 as libc::c_int as usize],
            -(1 as libc::c_int),
        );
        if -(1 as libc::c_int) == pid {
            log_perror(
                (*srv).errh,
                b"configfile.c\0" as *const u8 as *const libc::c_char,
                2492 as libc::c_int as libc::c_uint,
                b"fork/exec(%s)\0" as *const u8 as *const libc::c_char,
                cmd,
            );
            ret = -(1 as libc::c_int);
        } else {
            let mut rd: ssize_t = 0;
            let mut wstatus: libc::c_int = 0 as libc::c_int;
            let mut out: *mut buffer = buffer_init();
            close(fds[1 as libc::c_int as usize]);
            fds[1 as libc::c_int as usize] = -(1 as libc::c_int);
            loop {
                rd = read(
                    fds[0 as libc::c_int as usize],
                    buffer_string_prepare_append(out, 1023 as libc::c_int as size_t)
                        as *mut libc::c_void,
                    1023 as libc::c_int as size_t,
                );
                if rd >= 0 as libc::c_int as libc::c_long {
                    buffer_commit(out, rd as size_t);
                }
                if !(rd > 0 as libc::c_int as libc::c_long
                    || -(1 as libc::c_int) as libc::c_long == rd
                        && *__errno_location() == 4 as libc::c_int)
                {
                    break;
                }
            }
            if 0 as libc::c_int as libc::c_long != rd {
                log_perror(
                    (*srv).errh,
                    b"configfile.c\0" as *const u8 as *const libc::c_char,
                    2506 as libc::c_int as libc::c_uint,
                    b"read \"%s\"\0" as *const u8 as *const libc::c_char,
                    cmd,
                );
                ret = -(1 as libc::c_int);
            }
            close(fds[0 as libc::c_int as usize]);
            fds[0 as libc::c_int as usize] = -(1 as libc::c_int);
            if pid != fdevent_waitpid(pid, &mut wstatus, 0 as libc::c_int) {
                log_perror(
                    (*srv).errh,
                    b"configfile.c\0" as *const u8 as *const libc::c_char,
                    2512 as libc::c_int as libc::c_uint,
                    b"waitpid \"%s\"\0" as *const u8 as *const libc::c_char,
                    cmd,
                );
                ret = -(1 as libc::c_int);
            }
            if 0 as libc::c_int != wstatus {
                log_error(
                    (*srv).errh,
                    b"configfile.c\0" as *const u8 as *const libc::c_char,
                    2516 as libc::c_int as libc::c_uint,
                    b"command \"%s\" exited non-zero: %d\0" as *const u8
                        as *const libc::c_char,
                    cmd,
                    (wstatus & 0xff00 as libc::c_int) >> 8 as libc::c_int,
                );
                ret = -(1 as libc::c_int);
            }
            if -(1 as libc::c_int) != ret {
                ret = config_parse(
                    srv,
                    context,
                    cmd,
                    (*out).ptr,
                    buffer_clen(out) as libc::c_int,
                );
            }
            buffer_free(out);
        }
        if -(1 as libc::c_int) != fds[0 as libc::c_int as usize] {
            close(fds[0 as libc::c_int as usize]);
        }
        if -(1 as libc::c_int) != fds[1 as libc::c_int as usize] {
            close(fds[1 as libc::c_int as usize]);
        }
    }
    if 0 as libc::c_int != chdir(oldpwd.as_mut_ptr()) {
        log_perror(
            (*srv).errh,
            b"configfile.c\0" as *const u8 as *const libc::c_char,
            2532 as libc::c_int as libc::c_uint,
            b"cannot change directory to %s\0" as *const u8 as *const libc::c_char,
            oldpwd.as_mut_ptr(),
        );
        ret = -(1 as libc::c_int);
    }
    return ret;
}
unsafe extern "C" fn config_remoteip_normalize_ipv6(
    b: *mut buffer,
    tb: *mut buffer,
) -> libc::c_int {
    buffer_clear(tb);
    if *((*b).ptr).offset(0 as libc::c_int as isize) as libc::c_int != '[' as i32 {
        buffer_append_str3(
            tb,
            b"[\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            (*b).ptr,
            buffer_clen(b) as size_t,
            b"]\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
    } else {
        buffer_append_buffer(tb, b);
    }
    let mut rc: libc::c_int = http_request_host_normalize(tb, 0 as libc::c_int);
    if 0 as libc::c_int == rc {
        let mut blen: size_t = buffer_clen(tb) as size_t;
        if blen > 1 as libc::c_int as libc::c_ulong {
            buffer_copy_string_len(
                b,
                ((*tb).ptr).offset(1 as libc::c_int as isize),
                blen.wrapping_sub(2 as libc::c_int as libc::c_ulong),
            );
        }
    }
    return rc;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn config_remoteip_normalize(
    b: *mut buffer,
    tb: *mut buffer,
) -> libc::c_int {
    if *((*b).ptr).offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32 {
        return 1 as libc::c_int;
    }
    let slash: *const libc::c_char = strchr((*b).ptr, '/' as i32);
    let colon: *const libc::c_char = strchr((*b).ptr, ':' as i32);
    let mut nm_bits: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    if !slash.is_null() {
        let mut nptr: *mut libc::c_char = 0 as *mut libc::c_char;
        nm_bits = strtoul(
            slash.offset(1 as libc::c_int as isize),
            &mut nptr,
            10 as libc::c_int,
        );
        if *nptr as libc::c_int != 0 || 0 as libc::c_int as libc::c_ulong == nm_bits
            || nm_bits
                > (if !colon.is_null() {
                    128 as libc::c_uint
                } else {
                    32 as libc::c_uint
                }) as libc::c_ulong
        {
            return -(1 as libc::c_int);
        }
        buffer_truncate(
            b,
            slash.offset_from((*b).ptr) as libc::c_long as size_t as uint32_t,
        );
    }
    let mut family: libc::c_int = if !colon.is_null() {
        10 as libc::c_int
    } else {
        2 as libc::c_int
    };
    let mut rc: libc::c_int = if family == 2 as libc::c_int {
        http_request_host_normalize(b, 0 as libc::c_int)
    } else {
        config_remoteip_normalize_ipv6(b, tb)
    };
    let mut len: uint32_t = buffer_clen(b);
    if nm_bits != 0 {
        buffer_append_char(b, '/' as i32 as libc::c_char);
        buffer_append_int(b, nm_bits as libc::c_int as intmax_t);
    }
    if 0 as libc::c_int != rc {
        return -(1 as libc::c_int);
    }
    let mut after: *mut libc::c_char = buffer_string_prepare_append(
        b,
        (1 as libc::c_int + 7 as libc::c_int + 28 as libc::c_int) as size_t,
    );
    after = after.offset(1);
    after;
    *(after as *mut libc::c_uchar) = nm_bits as libc::c_uchar;
    let addr: *mut sock_addr = ((after as uintptr_t)
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        .wrapping_add(7 as libc::c_int as libc::c_ulong)
        & !(7 as libc::c_int) as libc::c_ulong) as *mut sock_addr;
    if nm_bits != 0 {
        *((*b).ptr).offset(len as isize) = '\0' as i32 as libc::c_char;
    }
    rc = sock_addr_inet_pton(addr, (*b).ptr, family, 0 as libc::c_int as libc::c_ushort);
    if nm_bits != 0 {
        *((*b).ptr).offset(len as isize) = '/' as i32 as libc::c_char;
    }
    return (1 as libc::c_int == rc) as libc::c_int;
}
unsafe extern "C" fn context_init(mut srv: *mut server, mut context: *mut config_t) {
    (*context).srv = srv;
    (*context).ok = 1 as libc::c_int;
    (*context).configs_stack.data = 0 as *mut *mut data_config;
    (*context).configs_stack.used = 0 as libc::c_int as uint32_t;
    (*context).configs_stack.size = 0 as libc::c_int as uint32_t;
    (*context).basedir = buffer_init();
}
unsafe extern "C" fn context_free(mut context: *mut config_t) {
    free((*context).configs_stack.data as *mut libc::c_void);
    buffer_free((*context).basedir);
}
#[inline(never)]
unsafe extern "C" fn config_vars_init(mut a: *mut array) {
    let mut dcwd: [libc::c_char; 4096] = [0; 4096];
    if !(getcwd(
        dcwd.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
    ))
        .is_null()
    {
        array_set_key_value(
            a,
            b"var.CWD\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
            dcwd.as_mut_ptr(),
            strlen(dcwd.as_mut_ptr()) as uint32_t,
        );
    }
    *array_get_int_ptr(
        a,
        b"var.PID\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    ) = getpid();
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn config_read(
    mut srv: *mut server,
    mut fn_0: *const libc::c_char,
) -> libc::c_int {
    let mut context: config_t = config_t {
        srv: 0 as *mut server,
        ok: 0,
        all_configs: 0 as *mut array,
        configs_stack: data_config_list {
            data: 0 as *mut *mut data_config,
            used: 0,
            size: 0,
        },
        current: 0 as *mut data_config,
        basedir: 0 as *mut buffer,
    };
    let mut dc: *mut data_config = 0 as *mut data_config;
    let mut ret: libc::c_int = 0;
    let mut pos: *mut libc::c_char = 0 as *mut libc::c_char;
    context_init(srv, &mut context);
    context.all_configs = (*srv).config_context;
    pos = strrchr(fn_0, '/' as i32);
    if !pos.is_null() {
        buffer_copy_string_len(
            context.basedir,
            fn_0,
            (pos.offset_from(fn_0) as libc::c_long + 1 as libc::c_int as libc::c_long)
                as size_t,
        );
    }
    dc = data_config_init();
    buffer_copy_string_len(
        &mut (*dc).key,
        b"global\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    config_vars_init((*dc).value);
    if !((*context.all_configs).used == 0 as libc::c_int as libc::c_uint) {
        ck_assert_failed(
            b"configfile.c\0" as *const u8 as *const libc::c_char,
            2652 as libc::c_int as libc::c_uint,
            b"context.all_configs->used == 0\0" as *const u8 as *const libc::c_char,
        );
    }
    (*dc).context_ndx = (*context.all_configs).used as libc::c_int;
    array_insert_unique(context.all_configs, dc as *mut data_unset);
    context.current = dc;
    ret = if 0 as libc::c_int != strcmp(fn_0, b"-\0" as *const u8 as *const libc::c_char)
    {
        config_parse_file_stream(srv, &mut context, fn_0)
    } else {
        config_parse_stdin(srv, &mut context)
    };
    if 0 as libc::c_int == ret && context.ok != 0
        && 0 as libc::c_int as libc::c_uint != context.configs_stack.used
    {
        ck_assert_failed(
            b"configfile.c\0" as *const u8 as *const libc::c_char,
            2662 as libc::c_int as libc::c_uint,
            b"!(0 == ret && context.ok && 0 != context.configs_stack.used)\0"
                as *const u8 as *const libc::c_char,
        );
    }
    context_free(&mut context);
    if 0 as libc::c_int != ret {
        return ret;
    }
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*(*srv).config_context).used {
        dc = *((*(*srv).config_context).data).offset(i as isize) as *mut data_config;
        if !((*dc).context_ndx == i as libc::c_int) {
            let mut j: uint32_t = i;
            while j < (*(*srv).config_context).used {
                dc = *((*(*srv).config_context).data).offset(j as isize)
                    as *mut data_config;
                if (*dc).context_ndx == i as libc::c_int {
                    let ref mut fresh6 = *((*(*srv).config_context).data)
                        .offset(j as isize);
                    *fresh6 = *((*(*srv).config_context).data).offset(i as isize);
                    let ref mut fresh7 = *((*(*srv).config_context).data)
                        .offset(i as isize);
                    *fresh7 = dc as *mut data_unset;
                    break;
                } else {
                    j = j.wrapping_add(1);
                    j;
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    if 0 as libc::c_int != config_insert_srvconf(srv) {
        return -(1 as libc::c_int);
    }
    if 0 as libc::c_int != config_insert(srv) {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[inline(never)]
unsafe extern "C" fn config_stat_isdir(
    path: *const libc::c_char,
    st: *mut stat,
) -> libc::c_int {
    return !(-(1 as libc::c_int) == stat(path, st)
        || (if (*st).st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o40000 as libc::c_int as libc::c_uint
        {
            0 as libc::c_int
        } else {
            let ref mut fresh8 = *__errno_location();
            *fresh8 = 20 as libc::c_int;
            *fresh8
        }) != 0) as libc::c_int;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn config_set_defaults(srv: *mut server) -> libc::c_int {
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
    if fdevent_config(&mut (*srv).srvconf.event_handler, (*srv).errh) <= 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    chunkqueue_set_tempdirs_default(
        (*srv).srvconf.upload_tempdirs,
        (*srv).srvconf.upload_temp_file_size as off_t,
    );
    if (*(*srv).srvconf.upload_tempdirs).used == 0 {
        let mut tmpdir: *const libc::c_char = chunkqueue_env_tmpdir();
        array_insert_value(
            (*srv).srvconf.upload_tempdirs,
            tmpdir,
            strlen(tmpdir) as uint32_t,
        );
    }
    let tb: *mut buffer = (*srv).tmp_buf;
    buffer_clear(tb);
    if !((*srv).srvconf.changeroot).is_null() {
        buffer_copy_buffer(tb, (*srv).srvconf.changeroot);
        if config_stat_isdir((*tb).ptr, &mut st) == 0 {
            log_perror(
                (*srv).errh,
                b"configfile.c\0" as *const u8 as *const libc::c_char,
                2734 as libc::c_int as libc::c_uint,
                b"server.chroot %s\0" as *const u8 as *const libc::c_char,
                (*tb).ptr,
            );
            return -(1 as libc::c_int);
        }
    }
    let len: uint_fast32_t = buffer_clen(tb) as uint_fast32_t;
    let mut i: uint_fast32_t = 0 as libc::c_int as uint_fast32_t;
    while i < (*(*srv).srvconf.upload_tempdirs).used as libc::c_ulong {
        let mut value: *const buffer = &mut (*(*((*(*srv).srvconf.upload_tempdirs).data)
            .offset(i as isize) as *mut data_string))
            .value;
        if len != 0 {
            buffer_truncate(tb, len as uint32_t);
            buffer_append_path_len(tb, (*value).ptr, buffer_clen(value) as size_t);
            value = tb;
        }
        if config_stat_isdir((*value).ptr, &mut st) == 0 {
            log_perror(
                (*srv).errh,
                b"configfile.c\0" as *const u8 as *const libc::c_char,
                2750 as libc::c_int as libc::c_uint,
                b"server.upload-dirs %s\0" as *const u8 as *const libc::c_char,
                (*value).ptr,
            );
        }
        i = i.wrapping_add(1);
        i;
    }
    let s: *mut request_config = &mut (*((*srv).config_data_base
        as *mut config_data_base))
        .defaults;
    if ((*s).document_root).is_null() || buffer_is_blank((*s).document_root) != 0 {
        log_error(
            (*srv).errh,
            b"configfile.c\0" as *const u8 as *const libc::c_char,
            2759 as libc::c_int as libc::c_uint,
            b"server.document-root is not set\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if 2 as libc::c_int == (*s).force_lowercase_filenames() as libc::c_int {
        (*s).set_force_lowercase_filenames(0 as libc::c_int as libc::c_uint);
        let tb_0: *mut buffer = (*srv).tmp_buf;
        buffer_copy_buffer(tb_0, (*s).document_root);
        buffer_to_upper(tb_0);
        if 0 as libc::c_int == stat((*tb_0).ptr, &mut st) {
            let st_ino: ino_t = st.st_ino;
            let is_upper_eq: libc::c_int = buffer_is_equal(tb_0, (*s).document_root);
            buffer_to_lower(tb_0);
            if is_upper_eq != 0 && buffer_is_equal(tb_0, (*s).document_root) != 0 {
                (*s).set_force_lowercase_filenames(0 as libc::c_int as libc::c_uint);
            } else if 0 as libc::c_int == stat((*tb_0).ptr, &mut st) {
                (*s)
                    .set_force_lowercase_filenames(
                        (st_ino == st.st_ino) as libc::c_int as libc::c_uint,
                    );
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn run_static_initializers() {
    cpk = [
        {
            let mut init = config_plugin_keys_t {
                k: b"server.modules\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_ARRAY_VLIST as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.compat-module-load\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 26]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.systemd-socket-activation\0" as *const u8
                    as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.port\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_SHORT as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.bind\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.network-backend\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.chroot\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.username\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.groupname\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.errorlog-placeholder-moved-to-config-insert\0" as *const u8
                    as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 51]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.breakagelog-placeholder-moved-to-config-insert\0"
                    as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 54]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.errorlog-use-syslog\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.syslog-facility\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.core-files\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.event-handler\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.pid-file\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.max-worker\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_SHORT as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.max-fds\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_SHORT as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.max-connections\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_SHORT as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.max-request-field-size\0" as *const u8
                    as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_SHORT as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.chunkqueue-chunk-sz\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_INT as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.upload-temp-file-size\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_INT as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.upload-dirs\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_ARRAY_VLIST as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.http-parseopts\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_ARRAY_KVSTRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.http-parseopt-header-strict\0" as *const u8
                    as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 35]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.http-parseopt-host-strict\0" as *const u8
                    as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.http-parseopt-host-normalize\0" as *const u8
                    as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.reject-expect-100-with-417\0" as *const u8
                    as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 34]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.stat-cache-engine\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"mimetype.xattr-name\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"ssl.engine\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SOCKET as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"debug.log-request-header-on-error\0" as *const u8
                    as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 34]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.feature-flags\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_ARRAY_KVANY as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_SERVER as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: 0 as *const libc::c_char,
                klen: 0 as libc::c_int as uint8_t,
                ktype: T_CONFIG_UNSET as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_UNSET as libc::c_int as uint8_t,
            };
            init
        },
    ];
    cpk_0 = [
        {
            let mut init = config_plugin_keys_t {
                k: b"server.document-root\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.name\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.tag\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.max-request-size\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_INT as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.max-keep-alive-requests\0" as *const u8
                    as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 31]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_SHORT as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.max-keep-alive-idle\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_SHORT as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.max-read-idle\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_SHORT as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.max-write-idle\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_SHORT as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.errorfile-prefix\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.error-handler\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.error-handler-404\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.error-intercept\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.force-lowercase-filenames\0" as *const u8
                    as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.follow-symlink\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.protocol-http11\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.range-requests\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 22]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.stream-request-body\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_SHORT as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.stream-response-body\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 28]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_SHORT as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.kbytes-per-second\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_SHORT as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"connection.kbytes-per-second\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_SHORT as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"mimetype.assign\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_ARRAY_KVSTRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"mimetype.use-xattr\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"etag.use-inode\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"etag.use-mtime\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"etag.use-size\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"debug.log-condition-handling\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 29]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"debug.log-file-not-found\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"debug.log-request-handling\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"debug.log-request-header\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"debug.log-response-header\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 26]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"debug.log-timeouts\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"debug.log-state-handling\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_BOOL as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.errorlog\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: b"server.breakagelog\0" as *const u8 as *const libc::c_char,
                klen: (::core::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t,
                ktype: T_CONFIG_STRING as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_CONNECTION as libc::c_int as uint8_t,
            };
            init
        },
        {
            let mut init = config_plugin_keys_t {
                k: 0 as *const libc::c_char,
                klen: 0 as libc::c_int as uint8_t,
                ktype: T_CONFIG_UNSET as libc::c_int as uint8_t,
                scope: T_CONFIG_SCOPE_UNSET as libc::c_int as uint8_t,
            };
            init
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
