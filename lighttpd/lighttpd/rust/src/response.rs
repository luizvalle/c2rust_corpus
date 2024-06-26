use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type pcre2_real_match_data_8;
    pub type fdevents;
    fn array_reset_data_strings(a: *mut array);
    fn http_response_body_clear(r: *mut request_st, preserve_length: libc::c_int);
    fn http_response_redirect_to_directory(
        r: *mut request_st,
        status: libc::c_int,
    ) -> libc::c_int;
    fn buffer_init() -> *mut buffer;
    fn buffer_free(b: *mut buffer);
    fn buffer_free_ptr(b: *mut buffer);
    fn buffer_copy_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn buffer_copy_string_len_lc(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn buffer_append_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn buffer_append_uint_hex_lc(b: *mut buffer, len: uintmax_t);
    fn buffer_append_int(b: *mut buffer, val: intmax_t);
    fn buffer_eq_icase_ssn(
        a: *const libc::c_char,
        b: *const libc::c_char,
        len: size_t,
    ) -> libc::c_int;
    fn buffer_copy_path_len2(
        b: *mut buffer,
        s1: *const libc::c_char,
        len1: size_t,
        s2: *const libc::c_char,
        len2: size_t,
    );
    fn chunkqueue_append_mem(cq: *mut chunkqueue, mem: *const libc::c_char, len: size_t);
    fn http_request_parse_target(
        r: *mut request_st,
        scheme_port: libc::c_int,
    ) -> libc::c_int;
    fn chunkqueue_prepend_buffer_commit(cq: *mut chunkqueue);
    fn chunkqueue_prepend_buffer_open(cq: *mut chunkqueue) -> *mut buffer;
    fn chunkqueue_append_buffer_commit(cq: *mut chunkqueue);
    fn chunkqueue_append_buffer_open(cq: *mut chunkqueue) -> *mut buffer;
    fn http_status_append(b: *mut buffer, status: libc::c_int);
    fn chunkqueue_reset(cq: *mut chunkqueue);
    fn request_config_reset(r: *mut request_st);
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn http_header_hkey_get(s: *const libc::c_char, slen: size_t) -> http_header_e;
    fn http_header_response_get(
        r: *const request_st,
        id: http_header_e,
        k: *const libc::c_char,
        klen: uint32_t,
    ) -> *mut buffer;
    fn http_header_response_set_ptr(
        r: *mut request_st,
        id: http_header_e,
        k: *const libc::c_char,
        klen: uint32_t,
    ) -> *mut buffer;
    fn http_header_response_unset(
        r: *mut request_st,
        id: http_header_e,
        k: *const libc::c_char,
        klen: uint32_t,
    );
    fn http_header_response_set(
        r: *mut request_st,
        id: http_header_e,
        k: *const libc::c_char,
        klen: uint32_t,
        v: *const libc::c_char,
        vlen: uint32_t,
    );
    fn http_header_response_append(
        r: *mut request_st,
        id: http_header_e,
        k: *const libc::c_char,
        klen: uint32_t,
        v: *const libc::c_char,
        vlen: uint32_t,
    );
    fn http_header_response_insert(
        r: *mut request_st,
        id: http_header_e,
        k: *const libc::c_char,
        klen: uint32_t,
        v: *const libc::c_char,
        vlen: uint32_t,
    );
    fn http_header_request_unset(
        r: *mut request_st,
        id: http_header_e,
        k: *const libc::c_char,
        klen: uint32_t,
    );
    fn http_header_env_set_ptr(
        r: *mut request_st,
        k: *const libc::c_char,
        klen: uint32_t,
    ) -> *mut buffer;
    fn log_pri(
        errh: *mut log_error_st,
        filename: *const libc::c_char,
        line: libc::c_uint,
        pri: libc::c_int,
        fmt: *const libc::c_char,
        _: ...
    );
    fn log_debug(
        errh: *mut log_error_st,
        filename: *const libc::c_char,
        line: libc::c_uint,
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
    fn stat_cache_content_type_get_by_ext(
        sce: *mut stat_cache_entry,
        mimetypes: *const array,
    ) -> *const buffer;
    fn stat_cache_get_entry(name: *const buffer) -> *mut stat_cache_entry;
    fn stat_cache_get_entry_open(
        name: *const buffer,
        symlinks: libc::c_int,
    ) -> *mut stat_cache_entry;
    fn stat_cache_path_contains_symlink(
        name: *const buffer,
        errh: *mut log_error_st,
    ) -> libc::c_int;
    fn http_chunk_append_file_ref(
        r: *mut request_st,
        sce: *mut stat_cache_entry,
    ) -> libc::c_int;
    fn http_range_rfc7233(r: *mut request_st) -> libc::c_int;
    fn config_patch_config(r: *mut request_st);
    fn config_cond_cache_reset(r: *mut request_st);
    fn plugins_call_handle_uri_clean(r: *mut request_st) -> handler_t;
    fn plugins_call_handle_subrequest_start(r: *mut request_st) -> handler_t;
    fn plugins_call_handle_response_start(r: *mut request_st) -> handler_t;
    fn plugins_call_handle_docroot(r: *mut request_st) -> handler_t;
    fn plugins_call_handle_physical(r: *mut request_st) -> handler_t;
    fn plugins_call_handle_request_reset(r: *mut request_st) -> handler_t;
    fn __errno_location() -> *mut libc::c_int;
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __intmax_t = libc::c_long;
pub type __uintmax_t = libc::c_ulong;
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
pub type __syscall_slong_t = libc::c_long;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type off_t = __off64_t;
pub type pid_t = __pid_t;
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
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
pub type unix_time64_t = time_t;
pub type unix_timespec64_t = timespec;
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
pub struct http_dispatch {
    pub process_streams: Option::<
        unsafe extern "C" fn(
            *mut connection,
            Option::<unsafe extern "C" fn(*mut request_st) -> handler_t>,
            Option::<
                unsafe extern "C" fn(*mut request_st, *mut connection) -> libc::c_int,
            >,
        ) -> libc::c_int,
    >,
    pub upgrade_h2: Option::<
        unsafe extern "C" fn(*mut request_st, *mut connection) -> (),
    >,
    pub upgrade_h2c: Option::<
        unsafe extern "C" fn(*mut request_st, *mut connection) -> (),
    >,
    pub send_100_continue: Option::<
        unsafe extern "C" fn(*mut request_st, *mut connection) -> libc::c_int,
    >,
    pub send_1xx: Option::<
        unsafe extern "C" fn(*mut request_st, *mut connection) -> libc::c_int,
    >,
    pub check_timeout: Option::<
        unsafe extern "C" fn(*mut connection, unix_time64_t) -> libc::c_int,
    >,
    pub goaway_graceful: Option::<unsafe extern "C" fn(*mut connection) -> libc::c_int>,
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
pub const COMP_HTTP_REMOTE_IP: C2RustUnnamed_7 = 8;
pub const COMP_SERVER_SOCKET: C2RustUnnamed_7 = 1;
pub const COMP_HTTP_REQUEST_HEADER: C2RustUnnamed_7 = 12;
pub const COMP_HTTP_QUERY_STRING: C2RustUnnamed_7 = 9;
pub const COMP_HTTP_URL: C2RustUnnamed_7 = 2;
pub const COMP_HTTP_REQUEST_METHOD: C2RustUnnamed_7 = 11;
pub const COMP_HTTP_HOST: C2RustUnnamed_7 = 3;
pub const COMP_HTTP_SCHEME: C2RustUnnamed_7 = 10;
pub type http_header_e = libc::c_uint;
pub const HTTP_HEADER_X_XSS_PROTECTION: http_header_e = 59;
pub const HTTP_HEADER_X_FRAME_OPTIONS: http_header_e = 58;
pub const HTTP_HEADER_X_FORWARDED_PROTO: http_header_e = 57;
pub const HTTP_HEADER_X_FORWARDED_FOR: http_header_e = 56;
pub const HTTP_HEADER_X_CONTENT_TYPE_OPTIONS: http_header_e = 55;
pub const HTTP_HEADER_WWW_AUTHENTICATE: http_header_e = 54;
pub const HTTP_HEADER_VARY: http_header_e = 53;
pub const HTTP_HEADER_USER_AGENT: http_header_e = 52;
pub const HTTP_HEADER_UPGRADE_INSECURE_REQUESTS: http_header_e = 51;
pub const HTTP_HEADER_UPGRADE: http_header_e = 50;
pub const HTTP_HEADER_TRANSFER_ENCODING: http_header_e = 49;
pub const HTTP_HEADER_TE: http_header_e = 48;
pub const HTTP_HEADER_STRICT_TRANSPORT_SECURITY: http_header_e = 47;
pub const HTTP_HEADER_STATUS: http_header_e = 46;
pub const HTTP_HEADER_SET_COOKIE: http_header_e = 45;
pub const HTTP_HEADER_SERVER: http_header_e = 44;
pub const HTTP_HEADER_REFERRER_POLICY: http_header_e = 43;
pub const HTTP_HEADER_REFERER: http_header_e = 42;
pub const HTTP_HEADER_RANGE: http_header_e = 41;
pub const HTTP_HEADER_PRIORITY: http_header_e = 40;
pub const HTTP_HEADER_PRAGMA: http_header_e = 39;
pub const HTTP_HEADER_P3P: http_header_e = 38;
pub const HTTP_HEADER_ONION_LOCATION: http_header_e = 37;
pub const HTTP_HEADER_LOCATION: http_header_e = 36;
pub const HTTP_HEADER_LINK: http_header_e = 35;
pub const HTTP_HEADER_LAST_MODIFIED: http_header_e = 34;
pub const HTTP_HEADER_IF_UNMODIFIED_SINCE: http_header_e = 33;
pub const HTTP_HEADER_IF_RANGE: http_header_e = 32;
pub const HTTP_HEADER_IF_NONE_MATCH: http_header_e = 31;
pub const HTTP_HEADER_IF_MODIFIED_SINCE: http_header_e = 30;
pub const HTTP_HEADER_IF_MATCH: http_header_e = 29;
pub const HTTP_HEADER_HTTP2_SETTINGS: http_header_e = 28;
pub const HTTP_HEADER_HOST: http_header_e = 27;
pub const HTTP_HEADER_FORWARDED: http_header_e = 26;
pub const HTTP_HEADER_EXPIRES: http_header_e = 25;
pub const HTTP_HEADER_EXPECT_CT: http_header_e = 24;
pub const HTTP_HEADER_EXPECT: http_header_e = 23;
pub const HTTP_HEADER_ETAG: http_header_e = 22;
pub const HTTP_HEADER_DNT: http_header_e = 21;
pub const HTTP_HEADER_DATE: http_header_e = 20;
pub const HTTP_HEADER_COOKIE: http_header_e = 19;
pub const HTTP_HEADER_CONTENT_TYPE: http_header_e = 18;
pub const HTTP_HEADER_CONTENT_SECURITY_POLICY: http_header_e = 17;
pub const HTTP_HEADER_CONTENT_RANGE: http_header_e = 16;
pub const HTTP_HEADER_CONTENT_LOCATION: http_header_e = 15;
pub const HTTP_HEADER_CONTENT_LENGTH: http_header_e = 14;
pub const HTTP_HEADER_CONTENT_ENCODING: http_header_e = 13;
pub const HTTP_HEADER_CONNECTION: http_header_e = 12;
pub const HTTP_HEADER_CACHE_CONTROL: http_header_e = 11;
pub const HTTP_HEADER_AUTHORIZATION: http_header_e = 10;
pub const HTTP_HEADER_ALT_USED: http_header_e = 9;
pub const HTTP_HEADER_ALT_SVC: http_header_e = 8;
pub const HTTP_HEADER_ALLOW: http_header_e = 7;
pub const HTTP_HEADER_AGE: http_header_e = 6;
pub const HTTP_HEADER_ACCESS_CONTROL_ALLOW_ORIGIN: http_header_e = 5;
pub const HTTP_HEADER_ACCEPT_RANGES: http_header_e = 4;
pub const HTTP_HEADER_ACCEPT_LANGUAGE: http_header_e = 3;
pub const HTTP_HEADER_ACCEPT_ENCODING: http_header_e = 2;
pub const HTTP_HEADER_ACCEPT: http_header_e = 1;
pub const HTTP_HEADER_OTHER: http_header_e = 0;
pub type C2RustUnnamed_7 = libc::c_uint;
pub const COMP_LAST_ELEMENT: C2RustUnnamed_7 = 13;
pub const COMP_HTTP_COOKIE: C2RustUnnamed_7 = 7;
pub const COMP_HTTP_LANGUAGE: C2RustUnnamed_7 = 6;
pub const COMP_HTTP_USER_AGENT: C2RustUnnamed_7 = 5;
pub const COMP_HTTP_REFERER: C2RustUnnamed_7 = 4;
pub const COMP_UNSET: C2RustUnnamed_7 = 0;
#[inline]
unsafe extern "C" fn buffer_clear(mut b: *mut buffer) {
    (*b).used = 0 as libc::c_int as uint32_t;
}
#[inline]
unsafe extern "C" fn buffer_reset(mut b: *mut buffer) {
    (*b).used = 0 as libc::c_int as uint32_t;
    if (*b).size > 4096 as libc::c_int as libc::c_uint {
        buffer_free_ptr(b);
    }
}
#[inline]
unsafe extern "C" fn buffer_has_slash_suffix(b: *const buffer) -> libc::c_int {
    return ((*b).used > 1 as libc::c_int as libc::c_uint
        && *((*b).ptr)
            .offset(((*b).used).wrapping_sub(2 as libc::c_int as libc::c_uint) as isize)
            as libc::c_int == '/' as i32) as libc::c_int;
}
#[inline]
unsafe extern "C" fn buffer_has_pathsep_suffix(b: *const buffer) -> libc::c_int {
    return ((*b).used > 1 as libc::c_int as libc::c_uint
        && *((*b).ptr)
            .offset(((*b).used).wrapping_sub(2 as libc::c_int as libc::c_uint) as isize)
            as libc::c_int == '/' as i32) as libc::c_int;
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
unsafe extern "C" fn buffer_truncate(mut b: *mut buffer, mut len: uint32_t) {
    *((*b).ptr).offset(len as isize) = '\0' as i32 as libc::c_char;
    (*b).used = len.wrapping_add(1 as libc::c_int as libc::c_uint);
}
#[inline]
unsafe extern "C" fn chunkqueue_length(mut cq: *const chunkqueue) -> off_t {
    return (*cq).bytes_in - (*cq).bytes_out;
}
unsafe extern "C" fn http_response_physical_pathinfo(
    r: *mut request_st,
) -> *mut stat_cache_entry {
    let mut pathinfo: *mut libc::c_char = ((*r).physical.path.ptr)
        .offset(buffer_clen(&mut (*r).physical.basedir) as isize)
        .offset(-(buffer_has_pathsep_suffix(&mut (*r).physical.basedir) as isize));
    if '/' as i32 != *pathinfo as libc::c_int {
        pathinfo = 0 as *mut libc::c_char;
    } else if pathinfo == (*r).physical.path.ptr {
        pathinfo = strchr(pathinfo.offset(1 as libc::c_int as isize), '/' as i32);
    }
    let mut sce: *mut stat_cache_entry = 0 as *mut stat_cache_entry;
    let pathused: uint32_t = (*r).physical.path.used;
    let mut pprev: *mut libc::c_char = pathinfo;
    while !pathinfo.is_null() {
        (*r)
            .physical
            .path
            .used = (pathinfo.offset_from((*r).physical.path.ptr) as libc::c_long
            + 1 as libc::c_int as libc::c_long) as uint32_t;
        *pathinfo = '\0' as i32 as libc::c_char;
        let nsce: *mut stat_cache_entry = stat_cache_get_entry(&mut (*r).physical.path);
        *pathinfo = '/' as i32 as libc::c_char;
        (*r).physical.path.used = pathused;
        if nsce.is_null() {
            pathinfo = if pathinfo != pprev { pprev } else { 0 as *mut libc::c_char };
            break;
        } else {
            sce = nsce;
            if !((*sce).st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o40000 as libc::c_int as libc::c_uint)
            {
                break;
            }
            pprev = pathinfo;
            pathinfo = strchr(pathinfo.offset(1 as libc::c_int as isize), '/' as i32);
        }
    }
    if pathinfo.is_null()
        || !((*sce).st.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o100000 as libc::c_int as libc::c_uint)
    {
        return 0 as *mut stat_cache_entry;
    }
    let mut len: size_t = ((*r).physical.path.ptr)
        .offset(pathused as isize)
        .offset(-(1 as libc::c_int as isize))
        .offset_from(pathinfo) as libc::c_long as size_t;
    let mut reqlen: size_t = 0;
    let ptr: *const libc::c_char = if ((*r).conf).force_lowercase_filenames()
        as libc::c_int != 0
        && {
            reqlen = buffer_clen(&mut (*r).target) as size_t;
            len <= reqlen
        }
        && buffer_eq_icase_ssn(
            ((*r).target.ptr).offset(reqlen as isize).offset(-(len as isize)),
            pathinfo,
            len,
        ) != 0
    {
        ((*r).target.ptr).offset(reqlen as isize).offset(-(len as isize))
    } else {
        pathinfo
    };
    buffer_copy_string_len(&mut (*r).pathinfo, ptr, len);
    buffer_truncate(
        &mut (*r).uri.path,
        (buffer_clen(&mut (*r).uri.path) as libc::c_ulong).wrapping_sub(len) as uint32_t,
    );
    buffer_truncate(
        &mut (*r).physical.path,
        pathinfo.offset_from((*r).physical.path.ptr) as libc::c_long as uint32_t,
    );
    return sce;
}
#[cold]
unsafe extern "C" fn http_response_physical_path_error(
    r: *mut request_st,
    code: libc::c_int,
    msg: *const libc::c_char,
) -> handler_t {
    (*r).http_status = code;
    if code == 404 as libc::c_int && ((*r).conf).log_file_not_found() as libc::c_int != 0
        || ((*r).conf).log_request_handling() as libc::c_int != 0
    {
        if msg.is_null() {
            log_pri(
                (*r).conf.errh,
                b"response.c\0" as *const u8 as *const libc::c_char,
                115 as libc::c_int as libc::c_uint,
                (1 as libc::c_int) << 8 as libc::c_int | 7 as libc::c_int,
                b"-- \0" as *const u8 as *const libc::c_char,
            );
        } else {
            log_debug(
                (*r).conf.errh,
                b"response.c\0" as *const u8 as *const libc::c_char,
                117 as libc::c_int as libc::c_uint,
                b"%s\0" as *const u8 as *const libc::c_char,
                msg,
            );
        }
        log_debug(
            (*r).conf.errh,
            b"response.c\0" as *const u8 as *const libc::c_char,
            118 as libc::c_int as libc::c_uint,
            b"Path         : %s\0" as *const u8 as *const libc::c_char,
            (*r).physical.path.ptr,
        );
        log_debug(
            (*r).conf.errh,
            b"response.c\0" as *const u8 as *const libc::c_char,
            120 as libc::c_int as libc::c_uint,
            b"URI          : %s\0" as *const u8 as *const libc::c_char,
            (*r).uri.path.ptr,
        );
    }
    return HANDLER_FINISHED;
}
unsafe extern "C" fn http_response_physical_path_check(r: *mut request_st) -> handler_t {
    let mut sce: *mut stat_cache_entry = stat_cache_get_entry(&mut (*r).physical.path);
    if !((sce != 0 as *mut libc::c_void as *mut stat_cache_entry) as libc::c_int
        as libc::c_long != 0)
    {
        's_48: {
            match *__errno_location() {
                20 => {
                    break 's_48;
                }
                13 => {
                    return http_response_physical_path_error(
                        r,
                        403 as libc::c_int,
                        0 as *const libc::c_char,
                    );
                }
                2 => {
                    if (*r).http_method as libc::c_int
                        == HTTP_METHOD_OPTIONS as libc::c_int
                        && (*r).resp_htags
                            & (1 as libc::c_ulong) << HTTP_HEADER_ALLOW as libc::c_int
                            != 0
                    {
                        (*r).http_status = 200 as libc::c_int;
                        return HANDLER_FINISHED;
                    }
                }
                36 => {}
                _ => {
                    return http_response_physical_path_error(
                        r,
                        500 as libc::c_int,
                        0 as *const libc::c_char,
                    );
                }
            }
            return http_response_physical_path_error(
                r,
                404 as libc::c_int,
                0 as *const libc::c_char,
            );
        }
        if ((*r).conf).http_pathinfo() == 0
            || {
                sce = http_response_physical_pathinfo(r);
                sce.is_null()
            }
        {
            return http_response_physical_path_error(
                r,
                404 as libc::c_int,
                b"-- file not found\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    if ((*r).conf).follow_symlink() == 0
        && 0 as libc::c_int
            != stat_cache_path_contains_symlink(&mut (*r).physical.path, (*r).conf.errh)
    {
        return http_response_physical_path_error(
            r,
            403 as libc::c_int,
            b"-- access denied due to symlink restriction\0" as *const u8
                as *const libc::c_char,
        );
    }
    (*r).tmp_sce = sce;
    if (*sce).st.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o100000 as libc::c_int as libc::c_uint
    {
        return HANDLER_GO_ON;
    }
    if (*sce).st.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o40000 as libc::c_int as libc::c_uint
    {
        if buffer_has_slash_suffix(&mut (*r).uri.path) == 0 {
            http_response_redirect_to_directory(r, 301 as libc::c_int);
            return HANDLER_FINISHED;
        }
    }
    return HANDLER_GO_ON;
}
#[cold]
#[inline(never)]
unsafe extern "C" fn http_status_set_error_close(
    r: *mut request_st,
    mut status: libc::c_int,
) -> handler_t {
    (*r).keep_alive = 0 as libc::c_int as int8_t;
    (*r).resp_body_finished = 1 as libc::c_int as libc::c_char;
    (*r).handler_module = 0 as *const plugin;
    (*r).http_status = status;
    return HANDLER_FINISHED;
}
#[cold]
unsafe extern "C" fn http_response_prepare_options_star(
    r: *mut request_st,
) -> handler_t {
    (*r).http_status = 200 as libc::c_int;
    (*r).resp_body_finished = 1 as libc::c_int as libc::c_char;
    http_header_response_append(
        r,
        HTTP_HEADER_ALLOW,
        b"Allow\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
        b"OPTIONS, GET, HEAD, POST\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    return HANDLER_FINISHED;
}
#[cold]
unsafe extern "C" fn http_response_prepare_connect(r: *mut request_st) -> handler_t {
    return (if !((*r).handler_module).is_null() {
        HANDLER_GO_ON as libc::c_int as libc::c_uint
    } else {
        http_status_set_error_close(r, 405 as libc::c_int) as libc::c_uint
    }) as handler_t;
}
unsafe extern "C" fn http_response_config(r: *mut request_st) -> handler_t {
    config_cond_cache_reset(r);
    config_patch_config(r);
    (*r)
        .server_name = if !((*r).conf.server_name).is_null() {
        (*r).conf.server_name
    } else {
        &mut (*r).uri.authority as *mut buffer as *const buffer
    };
    if (((*r).conf).allow_http11() == 0) as libc::c_int as libc::c_long != 0
        && (*r).http_version as libc::c_int == HTTP_VERSION_1_1 as libc::c_int
    {
        (*r).http_version = HTTP_VERSION_1_0;
        (*r).keep_alive = 0 as libc::c_int as int8_t;
        http_header_request_unset(
            r,
            HTTP_HEADER_UPGRADE,
            b"upgrade\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
    }
    if ((*r).reqbody_length > 0 as libc::c_int as libc::c_long) as libc::c_int
        as libc::c_long != 0
        && 0 as libc::c_int as libc::c_uint != (*r).conf.max_request_size
        && (*r).reqbody_length
            > ((*r).conf.max_request_size as off_t) << 10 as libc::c_int
    {
        log_error(
            (*r).conf.errh,
            b"response.c\0" as *const u8 as *const libc::c_char,
            247 as libc::c_int as libc::c_uint,
            b"request-size too long: %lld -> 413\0" as *const u8 as *const libc::c_char,
            (*r).reqbody_length as libc::c_longlong,
        );
        return http_status_set_error_close(r, 413 as libc::c_int);
    }
    return HANDLER_GO_ON;
}
#[inline(never)]
unsafe extern "C" fn http_response_prepare(r: *mut request_st) -> handler_t {
    let mut rc: handler_t = HANDLER_GO_ON;
    if ((*r).http_status > 200 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        if 0 as libc::c_int == (*r).resp_body_finished as libc::c_int {
            http_response_body_clear(r, 0 as libc::c_int);
        }
        return HANDLER_FINISHED;
    }
    if buffer_is_unset(&mut (*r).physical.path) as libc::c_long != 0 {
        rc = http_response_config(r);
        if HANDLER_GO_ON as libc::c_int as libc::c_uint != rc as libc::c_uint {
            return rc;
        }
        if ((*r).conf).log_request_handling() != 0 {
            log_debug(
                (*r).conf.errh,
                b"response.c\0" as *const u8 as *const libc::c_char,
                297 as libc::c_int as libc::c_uint,
                b"-- parsed Request-URI\0" as *const u8 as *const libc::c_char,
            );
            log_debug(
                (*r).conf.errh,
                b"response.c\0" as *const u8 as *const libc::c_char,
                299 as libc::c_int as libc::c_uint,
                b"Request-URI     : %s\0" as *const u8 as *const libc::c_char,
                (*r).target.ptr,
            );
            log_debug(
                (*r).conf.errh,
                b"response.c\0" as *const u8 as *const libc::c_char,
                301 as libc::c_int as libc::c_uint,
                b"URI-scheme      : %s\0" as *const u8 as *const libc::c_char,
                (*r).uri.scheme.ptr,
            );
            log_debug(
                (*r).conf.errh,
                b"response.c\0" as *const u8 as *const libc::c_char,
                303 as libc::c_int as libc::c_uint,
                b"URI-authority   : %s\0" as *const u8 as *const libc::c_char,
                (*r).uri.authority.ptr,
            );
            log_debug(
                (*r).conf.errh,
                b"response.c\0" as *const u8 as *const libc::c_char,
                305 as libc::c_int as libc::c_uint,
                b"URI-path (clean): %s\0" as *const u8 as *const libc::c_char,
                (*r).uri.path.ptr,
            );
            log_debug(
                (*r).conf.errh,
                b"response.c\0" as *const u8 as *const libc::c_char,
                307 as libc::c_int as libc::c_uint,
                b"URI-query       : %.*s\0" as *const u8 as *const libc::c_char,
                buffer_clen(&mut (*r).uri.query) as libc::c_int,
                (*r).uri.query.ptr,
            );
        }
        rc = plugins_call_handle_uri_clean(r);
        if HANDLER_GO_ON as libc::c_int as libc::c_uint != rc as libc::c_uint {
            return rc;
        }
        if ((*r).http_method as libc::c_int == HTTP_METHOD_OPTIONS as libc::c_int)
            as libc::c_int as libc::c_long != 0
            && *((*r).uri.path.ptr).offset(0 as libc::c_int as isize) as libc::c_int
                == '*' as i32
            && *((*r).uri.path.ptr).offset(1 as libc::c_int as isize) as libc::c_int
                == '\0' as i32
        {
            return http_response_prepare_options_star(r);
        }
        if ((*r).http_method as libc::c_int == HTTP_METHOD_CONNECT as libc::c_int)
            as libc::c_int as libc::c_long != 0
            && (!((*r).handler_module).is_null() || (*r).h2_connect_ext == 0)
        {
            return http_response_prepare_connect(r);
        }
        buffer_clear(&mut (*r).physical.doc_root);
        rc = plugins_call_handle_docroot(r);
        if HANDLER_GO_ON as libc::c_int as libc::c_uint != rc as libc::c_uint {
            return rc;
        }
        if (((*r).conf).force_lowercase_filenames() == 0) as libc::c_int as libc::c_long
            != 0
        {
            buffer_copy_buffer(&mut (*r).physical.rel_path, &mut (*r).uri.path);
        } else {
            buffer_copy_string_len_lc(
                &mut (*r).physical.rel_path,
                (*r).uri.path.ptr,
                buffer_clen(&mut (*r).uri.path) as size_t,
            );
        };
        if buffer_is_unset(&mut (*r).physical.doc_root) != 0 {
            buffer_copy_buffer(&mut (*r).physical.doc_root, (*r).conf.document_root);
        }
        buffer_copy_buffer(&mut (*r).physical.basedir, &mut (*r).physical.doc_root);
        buffer_copy_path_len2(
            &mut (*r).physical.path,
            (*r).physical.doc_root.ptr,
            buffer_clen(&mut (*r).physical.doc_root) as size_t,
            (*r).physical.rel_path.ptr,
            buffer_clen(&mut (*r).physical.rel_path) as size_t,
        );
        rc = plugins_call_handle_physical(r);
        if HANDLER_GO_ON as libc::c_int as libc::c_uint != rc as libc::c_uint {
            return rc;
        }
        if ((*r).conf).log_request_handling() != 0 {
            log_debug(
                (*r).conf.errh,
                b"response.c\0" as *const u8 as *const libc::c_char,
                386 as libc::c_int as libc::c_uint,
                b"-- logical -> physical\0" as *const u8 as *const libc::c_char,
            );
            log_debug(
                (*r).conf.errh,
                b"response.c\0" as *const u8 as *const libc::c_char,
                388 as libc::c_int as libc::c_uint,
                b"Doc-Root     : %s\0" as *const u8 as *const libc::c_char,
                (*r).physical.doc_root.ptr,
            );
            log_debug(
                (*r).conf.errh,
                b"response.c\0" as *const u8 as *const libc::c_char,
                390 as libc::c_int as libc::c_uint,
                b"Basedir      : %s\0" as *const u8 as *const libc::c_char,
                (*r).physical.basedir.ptr,
            );
            log_debug(
                (*r).conf.errh,
                b"response.c\0" as *const u8 as *const libc::c_char,
                392 as libc::c_int as libc::c_uint,
                b"Rel-Path     : %s\0" as *const u8 as *const libc::c_char,
                (*r).physical.rel_path.ptr,
            );
            log_debug(
                (*r).conf.errh,
                b"response.c\0" as *const u8 as *const libc::c_char,
                394 as libc::c_int as libc::c_uint,
                b"Path         : %s\0" as *const u8 as *const libc::c_char,
                (*r).physical.path.ptr,
            );
        }
    }
    if !((*r).handler_module).is_null() {
        return HANDLER_GO_ON;
    }
    rc = http_response_physical_path_check(r);
    if HANDLER_GO_ON as libc::c_int as libc::c_uint != rc as libc::c_uint {
        return rc;
    }
    if ((*r).conf).log_request_handling() != 0 {
        log_debug(
            (*r).conf.errh,
            b"response.c\0" as *const u8 as *const libc::c_char,
            406 as libc::c_int as libc::c_uint,
            b"-- handling subrequest\0" as *const u8 as *const libc::c_char,
        );
        log_debug(
            (*r).conf.errh,
            b"response.c\0" as *const u8 as *const libc::c_char,
            408 as libc::c_int as libc::c_uint,
            b"Path         : %s\0" as *const u8 as *const libc::c_char,
            (*r).physical.path.ptr,
        );
        log_debug(
            (*r).conf.errh,
            b"response.c\0" as *const u8 as *const libc::c_char,
            410 as libc::c_int as libc::c_uint,
            b"URI          : %s\0" as *const u8 as *const libc::c_char,
            (*r).uri.path.ptr,
        );
        log_debug(
            (*r).conf.errh,
            b"response.c\0" as *const u8 as *const libc::c_char,
            412 as libc::c_int as libc::c_uint,
            b"Pathinfo     : %.*s\0" as *const u8 as *const libc::c_char,
            buffer_clen(&mut (*r).pathinfo) as libc::c_int,
            (*r).pathinfo.ptr,
        );
    }
    rc = plugins_call_handle_subrequest_start(r);
    if HANDLER_GO_ON as libc::c_int as libc::c_uint != rc as libc::c_uint {
        return rc;
    }
    if !((*r).handler_module).is_null() {
        return HANDLER_GO_ON;
    }
    if (0 as libc::c_int == (*r).http_status) as libc::c_int as libc::c_long != 0 {
        if (*r).http_method as libc::c_int == HTTP_METHOD_OPTIONS as libc::c_int {
            http_response_body_clear(r, 0 as libc::c_int);
            http_response_prepare_options_star(r);
        } else if (*r).http_method as libc::c_int == HTTP_METHOD_CONNECT as libc::c_int {
            return http_status_set_error_close(r, 405 as libc::c_int)
        } else if !((*r).http_method as libc::c_int <= HTTP_METHOD_POST as libc::c_int) {
            (*r).http_status = 501 as libc::c_int;
        } else {
            (*r).http_status = 403 as libc::c_int;
        }
    }
    return HANDLER_FINISHED;
}
#[cold]
#[inline(never)]
unsafe extern "C" fn http_response_comeback(r: *mut request_st) -> handler_t {
    if !((*r).handler_module).is_null() || buffer_is_unset(&mut (*r).physical.path) == 0
    {
        return HANDLER_GO_ON;
    }
    request_config_reset(r);
    if ((*r).http_host != 0 as *mut libc::c_void as *mut buffer) as libc::c_int
        as libc::c_long != 0
    {
        buffer_copy_string_len_lc(
            &mut (*r).uri.authority,
            (*(*r).http_host).ptr,
            buffer_clen((*r).http_host) as size_t,
        );
    } else {
        buffer_copy_string_len(
            &mut (*r).uri.authority,
            b"\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 1]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
    }
    let mut status: libc::c_int = http_request_parse_target(
        r,
        (*(*r).con).proto_default_port as libc::c_int,
    );
    if 0 as libc::c_int == status {
        (*r)
            .conditional_is_valid = ((1 as libc::c_int)
            << COMP_SERVER_SOCKET as libc::c_int
            | (1 as libc::c_int) << COMP_HTTP_SCHEME as libc::c_int
            | (1 as libc::c_int) << COMP_HTTP_HOST as libc::c_int
            | (1 as libc::c_int) << COMP_HTTP_REMOTE_IP as libc::c_int
            | (1 as libc::c_int) << COMP_HTTP_REQUEST_METHOD as libc::c_int
            | (1 as libc::c_int) << COMP_HTTP_URL as libc::c_int
            | (1 as libc::c_int) << COMP_HTTP_QUERY_STRING as libc::c_int
            | (1 as libc::c_int) << COMP_HTTP_REQUEST_HEADER as libc::c_int) as uint32_t;
        return HANDLER_GO_ON;
    } else {
        (*r)
            .conditional_is_valid = ((1 as libc::c_int)
            << COMP_SERVER_SOCKET as libc::c_int
            | (1 as libc::c_int) << COMP_HTTP_REMOTE_IP as libc::c_int) as uint32_t;
        config_cond_cache_reset(r);
        return http_status_set_error_close(r, status);
    };
}
#[cold]
unsafe extern "C" fn http_response_errdoc_init(r: *mut request_st) {
    let mut www_auth: *mut buffer = 0 as *mut buffer;
    if 401 as libc::c_int == (*r).http_status {
        let vb: *const buffer = http_header_response_get(
            r,
            HTTP_HEADER_WWW_AUTHENTICATE,
            b"WWW-Authenticate\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
        if !vb.is_null() {
            www_auth = buffer_init();
            buffer_copy_buffer(www_auth, vb);
        }
    }
    buffer_reset(&mut (*r).physical.path);
    (*r).resp_htags = 0 as libc::c_int as uint64_t;
    array_reset_data_strings(&mut (*r).resp_headers);
    http_response_body_clear(r, 0 as libc::c_int);
    if !www_auth.is_null() {
        http_header_response_set(
            r,
            HTTP_HEADER_WWW_AUTHENTICATE,
            b"WWW-Authenticate\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
            (*www_auth).ptr,
            buffer_clen(www_auth),
        );
        buffer_free(www_auth);
    }
}
#[cold]
unsafe extern "C" fn http_response_static_errdoc(r: *mut request_st) {
    if if ((*r).handler_module).is_null() {
        ((*r).error_handler_saved_status >= 65535 as libc::c_int) as libc::c_int
    } else {
        (((*r).conf).error_intercept() == 0 || (*r).error_handler_saved_status != 0)
            as libc::c_int
    } != 0
    {
        return;
    }
    http_response_errdoc_init(r);
    (*r).resp_body_finished = 1 as libc::c_int as libc::c_char;
    if !((*r).conf.errorfile_prefix).is_null() {
        buffer_copy_buffer(&mut (*r).physical.path, (*r).conf.errorfile_prefix);
        buffer_append_int(&mut (*r).physical.path, (*r).http_status as intmax_t);
        buffer_append_string_len(
            &mut (*r).physical.path,
            b".html\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        );
        let mut sce: *mut stat_cache_entry = stat_cache_get_entry_open(
            &mut (*r).physical.path,
            ((*r).conf).follow_symlink() as libc::c_int,
        );
        if !sce.is_null() && 0 as libc::c_int == http_chunk_append_file_ref(r, sce) {
            let mut content_type: *const buffer = stat_cache_content_type_get_by_ext(
                sce,
                (*r).conf.mimetypes,
            );
            if !content_type.is_null() {
                http_header_response_set(
                    r,
                    HTTP_HEADER_CONTENT_TYPE,
                    b"Content-Type\0" as *const u8 as *const libc::c_char,
                    (::core::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint),
                    (*content_type).ptr,
                    buffer_clen(content_type),
                );
            }
            return;
        }
        buffer_clear(&mut (*r).physical.path);
    }
    let b: *mut buffer = chunkqueue_append_buffer_open(&mut (*r).write_queue);
    buffer_copy_string_len(
        b,
        b"<!DOCTYPE html>\n<html lang=\"en\">\n <head>\n  <meta charset=\"UTF-8\" />\n  <title>\0"
            as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 78]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    http_status_append(b, (*r).http_status);
    buffer_append_string_len(
        b,
        b"</title>\n </head>\n <body>\n  <h1>\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    http_status_append(b, (*r).http_status);
    buffer_append_string_len(
        b,
        b"</h1>\n </body>\n</html>\n\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    chunkqueue_append_buffer_commit(&mut (*r).write_queue);
    http_header_response_set(
        r,
        HTTP_HEADER_CONTENT_TYPE,
        b"Content-Type\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
        b"text/html\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
}
#[cold]
unsafe extern "C" fn http_response_merge_trailers(r: *mut request_st) {
    if buffer_is_blank(&mut (*(*r).gw_dechunk).b) != 0 {
        return;
    }
    let done: libc::c_int = (*(*r).gw_dechunk).done;
    if done == 0 {
        return;
    }
    if done < 400 as libc::c_int && (*r).http_status >= 400 as libc::c_int {
        return;
    }
    let mut k: *const libc::c_char = strchr((*(*r).gw_dechunk).b.ptr, '\n' as i32);
    if k.is_null() {
        return;
    }
    k = k.offset(1);
    k;
    let mut v: *const libc::c_char = 0 as *const libc::c_char;
    let mut e: *const libc::c_char = 0 as *const libc::c_char;
    loop {
        e = strchr(k, '\n' as i32);
        if e.is_null() {
            break;
        }
        v = memchr(
            k as *const libc::c_void,
            ':' as i32,
            e.offset_from(k) as libc::c_long as size_t,
        ) as *const libc::c_char;
        if !(v.is_null() || v == k || *k as libc::c_int == ' ' as i32
            || *k as libc::c_int == '\t' as i32)
        {
            let mut klen: uint32_t = v.offset_from(k) as libc::c_long as uint32_t;
            loop {
                v = v.offset(1);
                v;
                if !(*v as libc::c_int == ' ' as i32 || *v as libc::c_int == '\t' as i32)
                {
                    break;
                }
            }
            if !(*v as libc::c_int == '\r' as i32 || *v as libc::c_int == '\n' as i32) {
                let mut id: http_header_e = http_header_hkey_get(k, klen as size_t);
                http_header_response_insert(
                    r,
                    id,
                    k,
                    klen,
                    v,
                    e.offset_from(v) as libc::c_long as size_t as uint32_t,
                );
            }
        }
        k = e.offset(1 as libc::c_int as isize);
    }
    http_header_response_unset(
        r,
        HTTP_HEADER_OTHER,
        b"Trailer\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    buffer_clear(&mut (*(*r).gw_dechunk).b);
}
#[inline(never)]
unsafe extern "C" fn http_response_write_prepare(r: *mut request_st) -> handler_t {
    let mut current_block_5: u64;
    match (*r).http_status {
        200 => {
            current_block_5 = 7815301370352969686;
        }
        204 | 205 => {
            http_header_response_unset(
                r,
                HTTP_HEADER_CONTENT_LENGTH,
                b"Content-Length\0" as *const u8 as *const libc::c_char,
                (::core::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
            );
            current_block_5 = 1885571931232418223;
        }
        304 => {
            current_block_5 = 1885571931232418223;
        }
        _ => {
            if (*r).http_status >= 400 as libc::c_int
                && (*r).http_status < 600 as libc::c_int
            {
                http_response_static_errdoc(r);
            }
            current_block_5 = 7815301370352969686;
        }
    }
    match current_block_5 {
        1885571931232418223 => {
            http_response_body_clear(r, 1 as libc::c_int);
            (*r).resp_body_finished = 1 as libc::c_int as libc::c_char;
        }
        _ => {}
    }
    if !((*r).gw_dechunk).is_null() {
        http_response_merge_trailers(r);
    }
    match plugins_call_handle_response_start(r) as libc::c_uint {
        0 | 1 => {}
        _ => return HANDLER_ERROR,
    }
    if (*r).resp_body_finished != 0 {
        if ((*r).conf).range_requests() as libc::c_int != 0
            && (*r).http_status == 200 as libc::c_int
            && http_range_rfc7233(r) >= 400 as libc::c_int
        {
            http_response_static_errdoc(r);
        }
        if (*r).resp_htags
            & ((1 as libc::c_ulong) << HTTP_HEADER_CONTENT_LENGTH as libc::c_int
                | (1 as libc::c_ulong) << HTTP_HEADER_TRANSFER_ENCODING as libc::c_int)
            == 0
        {
            let mut qlen: off_t = chunkqueue_length(&mut (*r).write_queue);
            if qlen > 0 as libc::c_int as libc::c_long {
                buffer_append_int(
                    http_header_response_set_ptr(
                        r,
                        HTTP_HEADER_CONTENT_LENGTH,
                        b"Content-Length\0" as *const u8 as *const libc::c_char,
                        (::core::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint),
                    ),
                    qlen,
                );
            } else if (*r).http_method as libc::c_int != HTTP_METHOD_HEAD as libc::c_int
                && (*r).http_status != 204 as libc::c_int
                && (*r).http_status != 304 as libc::c_int
            {
                http_header_response_set(
                    r,
                    HTTP_HEADER_CONTENT_LENGTH,
                    b"Content-Length\0" as *const u8 as *const libc::c_char,
                    (::core::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint),
                    b"0\0" as *const u8 as *const libc::c_char,
                    (::core::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                        as uint32_t)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint),
                );
            }
        }
    } else if !((*r).http_version as libc::c_int >= HTTP_VERSION_2 as libc::c_int) {
        if (*r).resp_htags
            & ((1 as libc::c_ulong) << HTTP_HEADER_CONTENT_LENGTH as libc::c_int
                | (1 as libc::c_ulong) << HTTP_HEADER_TRANSFER_ENCODING as libc::c_int
                | (1 as libc::c_ulong) << HTTP_HEADER_UPGRADE as libc::c_int) == 0
        {
            if !((*r).http_method as libc::c_int == HTTP_METHOD_CONNECT as libc::c_int
                && (*r).http_status == 200 as libc::c_int)
            {
                if (*r).http_version as libc::c_int == HTTP_VERSION_1_1 as libc::c_int {
                    let mut qlen_0: off_t = chunkqueue_length(&mut (*r).write_queue);
                    (*r).resp_send_chunked = 1 as libc::c_int as libc::c_char;
                    if (*r).resp_decode_chunked != 0 {
                        let mut gw_chunked: off_t = (*(*r).gw_dechunk).gw_chunked;
                        if gw_chunked >= 2 as libc::c_int as libc::c_long {
                            qlen_0 += gw_chunked - 2 as libc::c_int as libc::c_long;
                        } else if 1 as libc::c_int as libc::c_long == gw_chunked {
                            chunkqueue_append_mem(
                                &mut (*r).write_queue,
                                b"\r\0" as *const u8 as *const libc::c_char,
                                (::core::mem::size_of::<[libc::c_char; 2]>()
                                    as libc::c_ulong as uint32_t)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                            );
                        } else {
                            if qlen_0 != 0 {
                                chunkqueue_append_mem(
                                    &mut (*r).write_queue,
                                    b"\r\n\0" as *const u8 as *const libc::c_char,
                                    (::core::mem::size_of::<[libc::c_char; 3]>()
                                        as libc::c_ulong as uint32_t)
                                        .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                                );
                            }
                            let hdr: *const buffer = &mut (*(*r).gw_dechunk).b;
                            if buffer_is_blank(hdr) == 0 {
                                chunkqueue_append_mem(
                                    &mut (*r).write_queue,
                                    (*hdr).ptr,
                                    buffer_clen(hdr) as size_t,
                                );
                            }
                        }
                    } else if qlen_0 != 0 {
                        chunkqueue_append_mem(
                            &mut (*r).write_queue,
                            b"\r\n\0" as *const u8 as *const libc::c_char,
                            (::core::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong
                                as uint32_t)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                        );
                    }
                    if qlen_0 != 0 {
                        let b: *mut buffer = chunkqueue_prepend_buffer_open(
                            &mut (*r).write_queue,
                        );
                        buffer_append_uint_hex_lc(b, qlen_0 as uintmax_t);
                        buffer_append_string_len(
                            b,
                            b"\r\n\0" as *const u8 as *const libc::c_char,
                            (::core::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong
                                as uint32_t)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                        );
                        chunkqueue_prepend_buffer_commit(&mut (*r).write_queue);
                    }
                    http_header_response_append(
                        r,
                        HTTP_HEADER_TRANSFER_ENCODING,
                        b"Transfer-Encoding\0" as *const u8 as *const libc::c_char,
                        (::core::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint),
                        b"chunked\0" as *const u8 as *const libc::c_char,
                        (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong
                            as uint32_t)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint),
                    );
                } else {
                    (*r).keep_alive = 0 as libc::c_int as int8_t;
                }
            }
        }
    }
    if (*r).http_method as libc::c_int == HTTP_METHOD_HEAD as libc::c_int {
        http_response_body_clear(r, 1 as libc::c_int);
        (*r).resp_body_finished = 1 as libc::c_int as libc::c_char;
    }
    return HANDLER_GO_ON;
}
#[cold]
unsafe extern "C" fn http_response_call_error_handler(
    r: *mut request_st,
    error_handler: *const buffer,
) -> libc::c_int {
    buffer_append_int(
        http_header_env_set_ptr(
            r,
            b"REDIRECT_STATUS\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        ),
        (*r).http_status as intmax_t,
    );
    if error_handler == (*r).conf.error_handler {
        plugins_call_handle_request_reset(r);
        if (*r).reqbody_length != 0 {
            if (*r).reqbody_length != (*r).reqbody_queue.bytes_in {
                (*r).keep_alive = 0 as libc::c_int as int8_t;
            }
            (*r).reqbody_length = 0 as libc::c_int as off_t;
            chunkqueue_reset(&mut (*r).reqbody_queue);
        }
        (*r).error_handler_saved_status = (*r).http_status;
        (*r).error_handler_saved_method = (*r).http_method;
        (*r).http_method = HTTP_METHOD_GET;
    } else {
        (*r).error_handler_saved_status = -(*r).http_status;
    }
    if (*r).http_version as libc::c_int == HTTP_VERSION_UNSET as libc::c_int {
        (*r).http_version = HTTP_VERSION_1_0;
    }
    buffer_copy_buffer(&mut (*r).target, error_handler);
    http_response_errdoc_init(r);
    (*r).http_status = 0 as libc::c_int;
    if (*r).rqst_htags & (1 as libc::c_ulong) << HTTP_HEADER_UPGRADE as libc::c_int != 0
    {
        http_header_request_unset(
            r,
            HTTP_HEADER_UPGRADE,
            b"upgrade\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
    }
    (*r).h2_connect_ext = 0 as libc::c_int;
    return 1 as libc::c_int;
}
#[cold]
#[inline(never)]
unsafe extern "C" fn http_response_has_error_handler(r: *mut request_st) -> libc::c_int {
    if (*r).error_handler_saved_status > 0 as libc::c_int {
        (*r).http_method = (*r).error_handler_saved_method;
    }
    if ((*r).handler_module).is_null()
        || ((*r).conf).error_intercept() as libc::c_int != 0
    {
        if (*r).error_handler_saved_status as libc::c_long != 0 {
            let subreq_status: libc::c_int = (*r).http_status;
            if (*r).error_handler_saved_status > 0 as libc::c_int {
                (*r).http_status = (*r).error_handler_saved_status;
            } else if (*r).http_status == 404 as libc::c_int
                || (*r).http_status == 403 as libc::c_int
            {
                (*r).http_status = -(*r).error_handler_saved_status;
            }
            if 200 as libc::c_int <= subreq_status && subreq_status <= 299 as libc::c_int
            {
                (*r).error_handler_saved_status = 65535 as libc::c_int;
            }
        } else if ((*r).http_status >= 400 as libc::c_int) as libc::c_int as libc::c_long
            != 0
        {
            let mut error_handler: *const buffer = 0 as *const buffer;
            if !((*r).conf.error_handler).is_null() {
                error_handler = (*r).conf.error_handler;
            } else if ((*r).http_status == 404 as libc::c_int
                || (*r).http_status == 403 as libc::c_int)
                && !((*r).conf.error_handler_404).is_null()
            {
                error_handler = (*r).conf.error_handler_404;
            }
            if !error_handler.is_null() {
                return http_response_call_error_handler(r, error_handler);
            }
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn http_response_handler(r: *mut request_st) -> handler_t {
    let mut rc: libc::c_int = 0;
    let mut current_block_12: u64;
    loop {
        let mut p: *const plugin = (*r).handler_module;
        if !p.is_null()
            || {
                rc = http_response_prepare(r) as libc::c_int;
                rc == HANDLER_GO_ON as libc::c_int
                    && {
                        p = (*r).handler_module;
                        !p.is_null()
                    }
            }
        {
            rc = ((*p).handle_subrequest)
                .expect("non-null function pointer")(r, (*p).data) as libc::c_int;
        }
        match rc {
            2 => {
                if (*r).resp_body_finished == 0
                    && ((*r).resp_body_started == 0
                        || 0 as libc::c_int
                            == (*r).conf.stream_response_body as libc::c_int
                                & ((1 as libc::c_int) << 0 as libc::c_int
                                    | (1 as libc::c_int) << 1 as libc::c_int))
                {
                    return HANDLER_WAIT_FOR_EVENT;
                }
                current_block_12 = 4388226512632805436;
            }
            0 | 1 => {
                current_block_12 = 4388226512632805436;
            }
            3 => {
                current_block_12 = 6433861111731890583;
            }
            _ => return HANDLER_ERROR,
        }
        match current_block_12 {
            4388226512632805436 => {
                if (*r).http_status == 0 as libc::c_int {
                    (*r).http_status = 200 as libc::c_int;
                }
                if ((*r).http_status < 400 as libc::c_int) as libc::c_int as libc::c_long
                    != 0
                    && (0 as libc::c_int == (*r).error_handler_saved_status)
                        as libc::c_int as libc::c_long != 0
                    || (http_response_has_error_handler(r) == 0) as libc::c_int
                        as libc::c_long != 0
                {
                    return http_response_write_prepare(r);
                }
            }
            _ => {}
        }
        http_response_comeback(r);
        rc = HANDLER_COMEBACK as libc::c_int;
        if !(rc == HANDLER_COMEBACK as libc::c_int) {
            break;
        }
    }
    return HANDLER_ERROR;
}
