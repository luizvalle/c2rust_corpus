use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type fdnode_st;
    pub type stat_cache_entry;
    pub type pcre2_real_match_data_8;
    pub type http_dispatch;
    pub type plugin;
    pub type fdevents;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn buffer_init() -> *mut buffer;
    fn buffer_free(b: *mut buffer);
    fn buffer_move(b: *mut buffer, src: *mut buffer);
    fn buffer_copy_string(b: *mut buffer, s: *const libc::c_char);
    fn buffer_copy_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn buffer_append_string(b: *mut buffer, s: *const libc::c_char);
    fn buffer_append_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn buffer_append_str2(
        b: *mut buffer,
        s1: *const libc::c_char,
        len1: size_t,
        s2: *const libc::c_char,
        len2: size_t,
    );
    fn buffer_append_iovec(b: *mut buffer, iov: *const const_iovec, n: size_t);
    fn buffer_append_int(b: *mut buffer, val: intmax_t);
    fn buffer_eq_slen(
        b: *const buffer,
        s: *const libc::c_char,
        slen: size_t,
    ) -> libc::c_int;
    fn buffer_is_equal(a: *const buffer, b: *const buffer) -> libc::c_int;
    fn ck_realloc_u32(
        list: *mut *mut libc::c_void,
        n: size_t,
        x: size_t,
        elt_sz: size_t,
    ) -> *mut libc::c_void;
    fn ck_assert_failed(
        filename: *const libc::c_char,
        line: libc::c_uint,
        msg: *const libc::c_char,
    ) -> !;
    fn array_data_string_init() -> *mut data_string;
    fn array_data_array_init() -> *mut data_array;
    fn array_data_integer_init() -> *mut data_integer;
    fn array_init(n: uint32_t) -> *mut array;
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
    fn array_extract_element_klen(
        a: *mut array,
        key: *const libc::c_char,
        klen: uint32_t,
    ) -> *mut data_unset;
    fn array_replace(a: *mut array, entry: *mut data_unset);
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
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn data_config_init() -> *mut data_config;
    fn config_parse_file(
        srv: *mut server,
        context: *mut config_t,
        fn_0: *const libc::c_char,
    ) -> libc::c_int;
    fn config_parse_cmd(
        srv: *mut server,
        context: *mut config_t,
        cmd: *const libc::c_char,
    ) -> libc::c_int;
    fn config_remoteip_normalize(b: *mut buffer, tb: *mut buffer) -> libc::c_int;
    fn http_header_hkey_get(s: *const libc::c_char, slen: size_t) -> http_header_e;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __errno_location() -> *mut libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __intmax_t = libc::c_long;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
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
pub type handler_t = libc::c_uint;
pub const HANDLER_ERROR: handler_t = 4;
pub const HANDLER_COMEBACK: handler_t = 3;
pub const HANDLER_WAIT_FOR_EVENT: handler_t = 2;
pub const HANDLER_FINISHED: handler_t = 1;
pub const HANDLER_GO_ON: handler_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hxcon {
    pub r: [*mut request_st; 8],
    pub rused: uint32_t,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct const_iovec {
    pub iov_base: *const libc::c_void,
    pub iov_len: size_t,
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
pub struct yyParser {
    pub yytos: *mut yyStackEntry,
    pub yyerrcnt: libc::c_int,
    pub ctx: *mut config_t,
    pub yystack: [yyStackEntry; 100],
    pub yystackEnd: *mut yyStackEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yyStackEntry {
    pub stateno: libc::c_uchar,
    pub major: libc::c_uchar,
    pub minor: YYMINORTYPE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union YYMINORTYPE {
    pub yyinit: libc::c_int,
    pub yy0: *mut buffer,
    pub yy20: *mut data_config,
    pub yy35: *mut data_unset,
    pub yy59: config_cond_t,
    pub yy66: *mut array,
}
pub type FILE = _IO_FILE;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub comp: comp_key_t,
    pub len: uint32_t,
    pub comp_tag: *const libc::c_char,
}
pub const _ISspace: C2RustUnnamed_8 = 8192;
pub type C2RustUnnamed_8 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_8 = 8;
pub const _ISpunct: C2RustUnnamed_8 = 4;
pub const _IScntrl: C2RustUnnamed_8 = 2;
pub const _ISblank: C2RustUnnamed_8 = 1;
pub const _ISgraph: C2RustUnnamed_8 = 32768;
pub const _ISprint: C2RustUnnamed_8 = 16384;
pub const _ISxdigit: C2RustUnnamed_8 = 4096;
pub const _ISdigit: C2RustUnnamed_8 = 2048;
pub const _ISalpha: C2RustUnnamed_8 = 1024;
pub const _ISlower: C2RustUnnamed_8 = 512;
pub const _ISupper: C2RustUnnamed_8 = 256;
#[inline]
unsafe extern "C" fn buffer_clear(mut b: *mut buffer) {
    (*b).used = 0 as libc::c_int as uint32_t;
}
#[inline]
unsafe extern "C" fn buffer_is_unset(mut b: *const buffer) -> libc::c_int {
    return (0 as libc::c_int as libc::c_uint == (*b).used) as libc::c_int;
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
unsafe extern "C" fn configparser_get_data_config(
    mut a: *const array,
    mut k: *const libc::c_char,
    klen: size_t,
) -> *mut data_config {
    return array_get_data_unset(a, k, klen as uint32_t) as *mut data_config;
}
#[inline(never)]
unsafe extern "C" fn configparser_push_data_config_list(
    mut v: *mut data_config_list,
    mut dc: *mut data_config,
) {
    if (*v).size == (*v).used {
        ck_realloc_u32(
            &mut (*v).data as *mut *mut *mut data_config as *mut *mut libc::c_void,
            (*v).size as size_t,
            4 as libc::c_int as size_t,
            ::core::mem::size_of::<*mut data_config>() as libc::c_ulong,
        );
        (*v)
            .size = ((*v).size as libc::c_uint)
            .wrapping_add(4 as libc::c_int as libc::c_uint) as uint32_t as uint32_t;
    }
    let fresh0 = (*v).used;
    (*v).used = ((*v).used).wrapping_add(1);
    let ref mut fresh1 = *((*v).data).offset(fresh0 as isize);
    *fresh1 = dc;
}
unsafe extern "C" fn configparser_push(
    mut ctx: *mut config_t,
    mut dc: *mut data_config,
    mut isnew: libc::c_int,
) {
    if isnew != 0 {
        (*dc).context_ndx = (*(*ctx).all_configs).used as libc::c_int;
        if !((*dc).context_ndx > (*(*ctx).current).context_ndx) {
            ck_assert_failed(
                b"./configparser.y\0" as *const u8 as *const libc::c_char,
                51 as libc::c_int as libc::c_uint,
                b"dc->context_ndx > ctx->current->context_ndx\0" as *const u8
                    as *const libc::c_char,
            );
        }
        array_insert_unique((*ctx).all_configs, dc as *mut data_unset);
        (*dc).parent = (*ctx).current;
        configparser_push_data_config_list(&mut (*(*dc).parent).children, dc);
    }
    if (*ctx).configs_stack.used > 0 as libc::c_int as libc::c_uint
        && (*(*ctx).current).context_ndx == 0 as libc::c_int
    {
        fprintf(
            stderr,
            b"Cannot use conditionals inside a global { ... } block\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(-(1 as libc::c_int));
    }
    configparser_push_data_config_list(&mut (*ctx).configs_stack, (*ctx).current);
    (*ctx).current = dc;
}
unsafe extern "C" fn configparser_pop(mut ctx: *mut config_t) -> *mut data_config {
    let mut old: *mut data_config = (*ctx).current;
    (*ctx)
        .current = if (*ctx).configs_stack.used > 0 as libc::c_int as libc::c_uint {
        (*ctx).configs_stack.used = ((*ctx).configs_stack.used).wrapping_sub(1);
        *((*ctx).configs_stack.data).offset((*ctx).configs_stack.used as isize)
    } else {
        0 as *mut data_config
    };
    if !(!old.is_null() && !((*ctx).current).is_null()) {
        ck_assert_failed(
            b"./configparser.y\0" as *const u8 as *const libc::c_char,
            69 as libc::c_int as libc::c_uint,
            b"old && ctx->current\0" as *const u8 as *const libc::c_char,
        );
    }
    return old;
}
unsafe extern "C" fn configparser_get_variable(
    mut ctx: *mut config_t,
    mut key: *const buffer,
) -> *mut data_unset {
    let mut du: *const data_unset = 0 as *const data_unset;
    let mut dc: *mut data_config = 0 as *mut data_config;
    dc = (*ctx).current;
    while !dc.is_null() {
        du = array_get_element_klen((*dc).value, (*key).ptr, buffer_clen(key));
        if !du.is_null() {
            let mut du_copy: *mut data_unset = ((*(*du).fn_0).copy)
                .expect("non-null function pointer")(du);
            buffer_clear(&mut (*du_copy).key);
            return du_copy;
        }
        dc = (*dc).parent;
    }
    return 0 as *mut data_unset;
}
unsafe extern "C" fn configparser_merge_data(
    mut op1: *mut data_unset,
    mut op2: *const data_unset,
) -> *mut data_unset {
    if (*op1).type_0 as libc::c_uint != (*op2).type_0 as libc::c_uint {
        if (*op1).type_0 as libc::c_uint == TYPE_STRING as libc::c_int as libc::c_uint
            && (*op2).type_0 as libc::c_uint
                == TYPE_INTEGER as libc::c_int as libc::c_uint
        {
            let mut ds: *mut data_string = op1 as *mut data_string;
            buffer_append_int(
                &mut (*ds).value,
                (*(op2 as *mut data_integer)).value as intmax_t,
            );
            return op1;
        } else if (*op1).type_0 as libc::c_uint
            == TYPE_INTEGER as libc::c_int as libc::c_uint
            && (*op2).type_0 as libc::c_uint
                == TYPE_STRING as libc::c_int as libc::c_uint
        {
            let mut ds_0: *mut data_string = array_data_string_init();
            buffer_append_int(
                &mut (*ds_0).value,
                (*(op1 as *mut data_integer)).value as intmax_t,
            );
            buffer_append_buffer(
                &mut (*ds_0).value,
                &mut (*(op2 as *mut data_string)).value,
            );
            ((*(*op1).fn_0).free).expect("non-null function pointer")(op1);
            return ds_0 as *mut data_unset;
        } else {
            fprintf(
                stderr,
                b"data type mismatch, cannot merge\n\0" as *const u8
                    as *const libc::c_char,
            );
            ((*(*op1).fn_0).free).expect("non-null function pointer")(op1);
            return 0 as *mut data_unset;
        }
    }
    match (*op1).type_0 as libc::c_uint {
        0 => {
            buffer_append_buffer(
                &mut (*(op1 as *mut data_string)).value,
                &mut (*(op2 as *mut data_string)).value,
            );
        }
        2 => {
            (*(op1 as *mut data_integer)).value += (*(op2 as *mut data_integer)).value;
        }
        1 => {
            let mut dst: *mut array = &mut (*(op1 as *mut data_array)).value;
            let mut src: *mut array = &mut (*(op2 as *mut data_array)).value;
            let mut du: *const data_unset = 0 as *const data_unset;
            let mut ddu: *const data_unset = 0 as *const data_unset;
            let mut i: size_t = 0;
            let mut current_block_24: u64;
            i = 0 as libc::c_int as size_t;
            while i < (*src).used as libc::c_ulong {
                du = *((*src).data).offset(i as isize);
                if !du.is_null() {
                    if buffer_is_unset(&(*du).key) != 0
                        || {
                            ddu = array_get_element_klen(
                                dst,
                                (*du).key.ptr,
                                buffer_clen(&(*du).key),
                            );
                            ddu.is_null()
                        }
                    {
                        array_insert_unique(
                            dst,
                            ((*(*du).fn_0).copy).expect("non-null function pointer")(du),
                        );
                    } else {
                        fprintf(
                            stderr,
                            b"Duplicate array-key '%s'\n\0" as *const u8
                                as *const libc::c_char,
                            (*du).key.ptr,
                        );
                        if (*ddu).type_0 as libc::c_uint == (*du).type_0 as libc::c_uint
                        {
                            if (*du).type_0 as libc::c_uint
                                == TYPE_STRING as libc::c_int as libc::c_uint
                                && buffer_is_equal(
                                    &mut (*(du as *mut data_string)).value,
                                    &mut (*(ddu as *mut data_string)).value,
                                ) != 0
                            {
                                current_block_24 = 5143058163439228106;
                            } else if (*du).type_0 as libc::c_uint
                                == TYPE_INTEGER as libc::c_int as libc::c_uint
                                && (*(du as *mut data_integer)).value
                                    == (*(ddu as *mut data_integer)).value
                            {
                                current_block_24 = 5143058163439228106;
                            } else {
                                current_block_24 = 9828876828309294594;
                            }
                        } else {
                            current_block_24 = 9828876828309294594;
                        }
                        match current_block_24 {
                            5143058163439228106 => {}
                            _ => {
                                ((*(*op1).fn_0).free)
                                    .expect("non-null function pointer")(op1);
                                return 0 as *mut data_unset;
                            }
                        }
                    }
                }
                i = i.wrapping_add(1);
                i;
            }
        }
        _ => {
            ck_assert_failed(
                b"./configparser.y\0" as *const u8 as *const libc::c_char,
                149 as libc::c_int as libc::c_uint,
                b"unexpected enum value\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    return op1;
}
static mut comps: [C2RustUnnamed_7; 13] = [C2RustUnnamed_7 {
    comp: COMP_UNSET,
    len: 0,
    comp_tag: 0 as *const libc::c_char,
}; 13];
unsafe extern "C" fn configparser_comp_key_id(
    obj_tag: *const buffer,
    comp_tag: *const buffer,
) -> comp_key_t {
    if buffer_eq_slen(
        obj_tag,
        b"REQUEST_HEADER\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    ) != 0
    {
        return COMP_HTTP_REQUEST_HEADER
    } else if buffer_eq_slen(
        obj_tag,
        b"SERVER\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    ) != 0
    {
        return (if buffer_eq_slen(
            comp_tag,
            b"socket\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
        ) != 0
        {
            COMP_SERVER_SOCKET as libc::c_int
        } else {
            COMP_UNSET as libc::c_int
        }) as comp_key_t
    } else if buffer_eq_slen(
        obj_tag,
        b"HTTP\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    ) != 0
    {
        let mut i: uint32_t = 0 as libc::c_int as uint32_t;
        while (i as libc::c_ulong)
            < (::core::mem::size_of::<[C2RustUnnamed_7; 13]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<C2RustUnnamed_7>() as libc::c_ulong)
        {
            if buffer_eq_slen(
                comp_tag,
                comps[i as usize].comp_tag,
                comps[i as usize].len as size_t,
            ) != 0
            {
                return comps[i as usize].comp;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return COMP_UNSET;
}
unsafe extern "C" fn configparser_simplify_regex(b: *mut buffer) -> config_cond_t {
    let mut len: uint32_t = buffer_clen(b);
    let mut cond: config_cond_t = CONFIG_COND_MATCH;
    let mut off: libc::c_int = 0 as libc::c_int;
    if len != 0
        && *((*b).ptr)
            .offset(len.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
            as libc::c_int == '$' as i32
    {
        cond = CONFIG_COND_SUFFIX;
        if *((*b).ptr).offset(0 as libc::c_int as isize) as libc::c_int == '\\' as i32
            && *((*b).ptr).offset(1 as libc::c_int as isize) as libc::c_int == '.' as i32
        {
            off = 2 as libc::c_int;
        } else if *((*b).ptr).offset(0 as libc::c_int as isize) as libc::c_int
            == '^' as i32
        {
            off = 1 as libc::c_int;
            cond = CONFIG_COND_EQ;
        }
        len = len.wrapping_sub(1);
        len;
    } else if *((*b).ptr).offset(0 as libc::c_int as isize) as libc::c_int == '^' as i32
    {
        off = 1 as libc::c_int;
        cond = CONFIG_COND_PREFIX;
    } else {
        return CONFIG_COND_MATCH
    }
    static mut regex_chars: [libc::c_char; 15] = unsafe {
        *::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"\\^$.|?*+()[]{}\0")
    };
    if strcspn(((*b).ptr).offset(off as isize), regex_chars.as_ptr())
        != len.wrapping_sub(off as libc::c_uint) as libc::c_ulong
    {
        return CONFIG_COND_MATCH;
    }
    if off != 0 {
        memmove(
            (*b).ptr as *mut libc::c_void,
            ((*b).ptr).offset(1 as libc::c_int as isize) as *const libc::c_void,
            len.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_ulong,
        );
        len = len.wrapping_sub(1);
        len;
    }
    buffer_truncate(b, len);
    return cond;
}
unsafe extern "C" fn configparser_parse_condition(
    ctx: *mut config_t,
    obj_tag: *const buffer,
    comp_tag: *const buffer,
    mut cond: config_cond_t,
    rvalue: *mut buffer,
) {
    let comp: comp_key_t = configparser_comp_key_id(obj_tag, comp_tag);
    if cond as libc::c_uint == CONFIG_COND_MATCH as libc::c_int as libc::c_uint
        && comp as libc::c_uint != COMP_SERVER_SOCKET as libc::c_int as libc::c_uint
    {
        cond = configparser_simplify_regex(rvalue);
    }
    let mut op: *const libc::c_char = 0 as *const libc::c_char;
    match cond as libc::c_uint {
        3 => {
            op = b"!=\0" as *const u8 as *const libc::c_char;
        }
        1 => {
            op = b"==\0" as *const u8 as *const libc::c_char;
        }
        4 => {
            op = b"!~\0" as *const u8 as *const libc::c_char;
        }
        2 => {
            op = b"=~\0" as *const u8 as *const libc::c_char;
        }
        5 => {
            op = b"=^\0" as *const u8 as *const libc::c_char;
        }
        6 => {
            op = b"=$\0" as *const u8 as *const libc::c_char;
        }
        _ => {
            ck_assert_failed(
                b"./configparser.y\0" as *const u8 as *const libc::c_char,
                251 as libc::c_int as libc::c_uint,
                b"unexpected enum value\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    let comp_offset: uint32_t = (buffer_clen(&mut (*(*ctx).current).key))
        .wrapping_add(3 as libc::c_int as libc::c_uint);
    let tb: *mut buffer = (*(*ctx).srv).tmp_buf;
    buffer_clear(tb);
    let mut iov: [const_iovec; 11] = [
        {
            let mut init = const_iovec {
                iov_base: (*(*ctx).current).key.ptr as *const libc::c_void,
                iov_len: buffer_clen(&mut (*(*ctx).current).key) as size_t,
            };
            init
        },
        {
            let mut init = const_iovec {
                iov_base: b" / \0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                iov_len: (::core::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            };
            init
        },
        {
            let mut init = const_iovec {
                iov_base: b"$\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                iov_len: (::core::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            };
            init
        },
        {
            let mut init = const_iovec {
                iov_base: (*obj_tag).ptr as *const libc::c_void,
                iov_len: buffer_clen(obj_tag) as size_t,
            };
            init
        },
        {
            let mut init = const_iovec {
                iov_base: b"[\"\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                iov_len: (::core::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            };
            init
        },
        {
            let mut init = const_iovec {
                iov_base: (*comp_tag).ptr as *const libc::c_void,
                iov_len: buffer_clen(comp_tag) as size_t,
            };
            init
        },
        {
            let mut init = const_iovec {
                iov_base: b"\"] \0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                iov_len: (::core::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            };
            init
        },
        {
            let mut init = const_iovec {
                iov_base: op as *const libc::c_void,
                iov_len: 2 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = const_iovec {
                iov_base: b" \"\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                iov_len: (::core::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            };
            init
        },
        {
            let mut init = const_iovec {
                iov_base: (*rvalue).ptr as *const libc::c_void,
                iov_len: buffer_clen(rvalue) as size_t,
            };
            init
        },
        {
            let mut init = const_iovec {
                iov_base: b"\"\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                iov_len: (::core::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            };
            init
        },
    ];
    buffer_append_iovec(
        tb,
        iov.as_mut_ptr(),
        (::core::mem::size_of::<[const_iovec; 11]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<const_iovec>() as libc::c_ulong),
    );
    let mut dc: *mut data_config = 0 as *mut data_config;
    dc = configparser_get_data_config(
        (*ctx).all_configs,
        (*tb).ptr,
        buffer_clen(tb) as size_t,
    );
    if !dc.is_null() {
        configparser_push(ctx, dc, 0 as libc::c_int);
    } else {
        dc = data_config_init();
        (*dc).cond = cond;
        (*dc).comp = comp;
        buffer_copy_buffer(&mut (*dc).key, tb);
        buffer_copy_buffer(&mut (*dc).comp_tag, comp_tag);
        (*dc).comp_key = ((*dc).key.ptr).offset(comp_offset as isize);
        if COMP_UNSET as libc::c_int as libc::c_uint == (*dc).comp as libc::c_uint {
            fprintf(
                stderr,
                b"error comp_key %s\n\0" as *const u8 as *const libc::c_char,
                (*dc).comp_key,
            );
            (*ctx).ok = 0 as libc::c_int;
        } else if COMP_HTTP_LANGUAGE as libc::c_int as libc::c_uint
            == (*dc).comp as libc::c_uint
        {
            (*dc).comp = COMP_HTTP_REQUEST_HEADER;
            buffer_copy_string_len(
                &mut (*dc).comp_tag,
                b"Accept-Language\0" as *const u8 as *const libc::c_char,
                (::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
        } else if COMP_HTTP_USER_AGENT as libc::c_int as libc::c_uint
            == (*dc).comp as libc::c_uint
        {
            (*dc).comp = COMP_HTTP_REQUEST_HEADER;
            buffer_copy_string_len(
                &mut (*dc).comp_tag,
                b"User-Agent\0" as *const u8 as *const libc::c_char,
                (::core::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
        } else if COMP_HTTP_REMOTE_IP as libc::c_int as libc::c_uint
            == (*dc).comp as libc::c_uint
            && ((*dc).cond as libc::c_uint
                == CONFIG_COND_EQ as libc::c_int as libc::c_uint
                || (*dc).cond as libc::c_uint
                    == CONFIG_COND_NE as libc::c_int as libc::c_uint
                || (*dc).cond as libc::c_uint
                    == CONFIG_COND_PREFIX as libc::c_int as libc::c_uint
                || (*dc).cond as libc::c_uint
                    == CONFIG_COND_SUFFIX as libc::c_int as libc::c_uint)
        {
            if config_remoteip_normalize(rvalue, tb) == 0 {
                fprintf(
                    stderr,
                    b"invalid IP addr: %s\n\0" as *const u8 as *const libc::c_char,
                    (*rvalue).ptr,
                );
                (*ctx).ok = 0 as libc::c_int;
            }
        } else if COMP_SERVER_SOCKET as libc::c_int as libc::c_uint
            == (*dc).comp as libc::c_uint
        {
            if *((*rvalue).ptr).offset(0 as libc::c_int as isize) as libc::c_int
                != ':' as i32
                && !(*((*rvalue).ptr).offset(0 as libc::c_int as isize) as libc::c_int
                    == '[' as i32
                    && *((*rvalue).ptr).offset(1 as libc::c_int as isize) as libc::c_int
                        == ']' as i32)
                && !(*((*rvalue).ptr).offset(0 as libc::c_int as isize) as libc::c_int
                    == '/' as i32
                    || *((*rvalue).ptr).offset(0 as libc::c_int as isize) as libc::c_int
                        == '\\' as i32)
            {
                if http_request_host_normalize(rvalue, 0 as libc::c_int) != 0 {
                    fprintf(
                        stderr,
                        b"invalid IP addr: %s\n\0" as *const u8 as *const libc::c_char,
                        (*rvalue).ptr,
                    );
                    (*ctx).ok = 0 as libc::c_int;
                }
            }
        } else if COMP_HTTP_HOST as libc::c_int as libc::c_uint
            == (*dc).comp as libc::c_uint
        {
            if (*dc).cond as libc::c_uint
                == CONFIG_COND_EQ as libc::c_int as libc::c_uint
                || (*dc).cond as libc::c_uint
                    == CONFIG_COND_NE as libc::c_int as libc::c_uint
                || (*dc).cond as libc::c_uint
                    == CONFIG_COND_PREFIX as libc::c_int as libc::c_uint
                || (*dc).cond as libc::c_uint
                    == CONFIG_COND_SUFFIX as libc::c_int as libc::c_uint
            {
                if http_request_host_normalize(rvalue, 0 as libc::c_int) != 0 {
                    fprintf(
                        stderr,
                        b"invalid IP addr: %s\n\0" as *const u8 as *const libc::c_char,
                        (*rvalue).ptr,
                    );
                    (*ctx).ok = 0 as libc::c_int;
                }
            }
        }
        if COMP_HTTP_REQUEST_HEADER as libc::c_int as libc::c_uint
            == (*dc).comp as libc::c_uint
        {
            (*dc)
                .ext = http_header_hkey_get(
                (*dc).comp_tag.ptr,
                buffer_clen(&mut (*dc).comp_tag) as size_t,
            ) as libc::c_int;
        }
        buffer_move(&mut (*dc).string, rvalue);
        if (*ctx).ok != 0 {
            configparser_push(ctx, dc, 1 as libc::c_int);
        } else {
            ((*(*dc).fn_0).free)
                .expect("non-null function pointer")(dc as *mut data_unset);
        }
    };
}
unsafe extern "C" fn configparser_parse_else_condition(ctx: *mut config_t) {
    let dc: *mut data_config = data_config_init();
    (*dc).cond = CONFIG_COND_ELSE;
    buffer_append_str2(
        &mut (*dc).key,
        (*(*ctx).current).key.ptr,
        buffer_clen(&mut (*(*ctx).current).key) as size_t,
        b" / else_tmp_token\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 18]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    configparser_push(ctx, dc, 1 as libc::c_int);
}
static mut yy_action: [libc::c_uchar; 150] = [
    158 as libc::c_int as libc::c_uchar,
    158 as libc::c_int as libc::c_uchar,
    158 as libc::c_int as libc::c_uchar,
    18 as libc::c_int as libc::c_uchar,
    158 as libc::c_int as libc::c_uchar,
    158 as libc::c_int as libc::c_uchar,
    16 as libc::c_int as libc::c_uchar,
    115 as libc::c_int as libc::c_uchar,
    126 as libc::c_int as libc::c_uchar,
    143 as libc::c_int as libc::c_uchar,
    10 as libc::c_int as libc::c_uchar,
    76 as libc::c_int as libc::c_uchar,
    154 as libc::c_int as libc::c_uchar,
    22 as libc::c_int as libc::c_uchar,
    131 as libc::c_int as libc::c_uchar,
    128 as libc::c_int as libc::c_uchar,
    39 as libc::c_int as libc::c_uchar,
    31 as libc::c_int as libc::c_uchar,
    37 as libc::c_int as libc::c_uchar,
    90 as libc::c_int as libc::c_uchar,
    30 as libc::c_int as libc::c_uchar,
    96 as libc::c_int as libc::c_uchar,
    38 as libc::c_int as libc::c_uchar,
    30 as libc::c_int as libc::c_uchar,
    76 as libc::c_int as libc::c_uchar,
    115 as libc::c_int as libc::c_uchar,
    80 as libc::c_int as libc::c_uchar,
    81 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    76 as libc::c_int as libc::c_uchar,
    19 as libc::c_int as libc::c_uchar,
    115 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    7 as libc::c_int as libc::c_uchar,
    76 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    90 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    91 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    30 as libc::c_int as libc::c_uchar,
    16 as libc::c_int as libc::c_uchar,
    90 as libc::c_int as libc::c_uchar,
    15 as libc::c_int as libc::c_uchar,
    95 as libc::c_int as libc::c_uchar,
    17 as libc::c_int as libc::c_uchar,
    30 as libc::c_int as libc::c_uchar,
    157 as libc::c_int as libc::c_uchar,
    115 as libc::c_int as libc::c_uchar,
    29 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    7 as libc::c_int as libc::c_uchar,
    76 as libc::c_int as libc::c_uchar,
    14 as libc::c_int as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    11 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    7 as libc::c_int as libc::c_uchar,
    84 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    90 as libc::c_int as libc::c_uchar,
    13 as libc::c_int as libc::c_uchar,
    165 as libc::c_int as libc::c_uchar,
    76 as libc::c_int as libc::c_uchar,
    30 as libc::c_int as libc::c_uchar,
    80 as libc::c_int as libc::c_uchar,
    81 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    83 as libc::c_int as libc::c_uchar,
    20 as libc::c_int as libc::c_uchar,
    21 as libc::c_int as libc::c_uchar,
    127 as libc::c_int as libc::c_uchar,
    24 as libc::c_int as libc::c_uchar,
    136 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    7 as libc::c_int as libc::c_uchar,
    23 as libc::c_int as libc::c_uchar,
    131 as libc::c_int as libc::c_uchar,
    128 as libc::c_int as libc::c_uchar,
    35 as libc::c_int as libc::c_uchar,
    127 as libc::c_int as libc::c_uchar,
    24 as libc::c_int as libc::c_uchar,
    134 as libc::c_int as libc::c_uchar,
    119 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    120 as libc::c_int as libc::c_uchar,
    131 as libc::c_int as libc::c_uchar,
    128 as libc::c_int as libc::c_uchar,
    35 as libc::c_int as libc::c_uchar,
    120 as libc::c_int as libc::c_uchar,
    120 as libc::c_int as libc::c_uchar,
    120 as libc::c_int as libc::c_uchar,
    120 as libc::c_int as libc::c_uchar,
    99 as libc::c_int as libc::c_uchar,
    100 as libc::c_int as libc::c_uchar,
    101 as libc::c_int as libc::c_uchar,
    102 as libc::c_int as libc::c_uchar,
    103 as libc::c_int as libc::c_uchar,
    104 as libc::c_int as libc::c_uchar,
    120 as libc::c_int as libc::c_uchar,
    120 as libc::c_int as libc::c_uchar,
    127 as libc::c_int as libc::c_uchar,
    28 as libc::c_int as libc::c_uchar,
    120 as libc::c_int as libc::c_uchar,
    127 as libc::c_int as libc::c_uchar,
    28 as libc::c_int as libc::c_uchar,
    120 as libc::c_int as libc::c_uchar,
    131 as libc::c_int as libc::c_uchar,
    128 as libc::c_int as libc::c_uchar,
    156 as libc::c_int as libc::c_uchar,
    131 as libc::c_int as libc::c_uchar,
    128 as libc::c_int as libc::c_uchar,
    155 as libc::c_int as libc::c_uchar,
    127 as libc::c_int as libc::c_uchar,
    28 as libc::c_int as libc::c_uchar,
    120 as libc::c_int as libc::c_uchar,
    127 as libc::c_int as libc::c_uchar,
    26 as libc::c_int as libc::c_uchar,
    120 as libc::c_int as libc::c_uchar,
    131 as libc::c_int as libc::c_uchar,
    128 as libc::c_int as libc::c_uchar,
    27 as libc::c_int as libc::c_uchar,
    131 as libc::c_int as libc::c_uchar,
    128 as libc::c_int as libc::c_uchar,
    127 as libc::c_int as libc::c_uchar,
    32 as libc::c_int as libc::c_uchar,
    127 as libc::c_int as libc::c_uchar,
    33 as libc::c_int as libc::c_uchar,
    120 as libc::c_int as libc::c_uchar,
    120 as libc::c_int as libc::c_uchar,
    131 as libc::c_int as libc::c_uchar,
    128 as libc::c_int as libc::c_uchar,
    131 as libc::c_int as libc::c_uchar,
    128 as libc::c_int as libc::c_uchar,
    127 as libc::c_int as libc::c_uchar,
    34 as libc::c_int as libc::c_uchar,
    127 as libc::c_int as libc::c_uchar,
    36 as libc::c_int as libc::c_uchar,
    141 as libc::c_int as libc::c_uchar,
    142 as libc::c_int as libc::c_uchar,
    131 as libc::c_int as libc::c_uchar,
    128 as libc::c_int as libc::c_uchar,
    131 as libc::c_int as libc::c_uchar,
    128 as libc::c_int as libc::c_uchar,
    120 as libc::c_int as libc::c_uchar,
    120 as libc::c_int as libc::c_uchar,
    120 as libc::c_int as libc::c_uchar,
    37 as libc::c_int as libc::c_uchar,
    25 as libc::c_int as libc::c_uchar,
];
static mut yy_lookahead: [libc::c_uchar; 179] = [
    31 as libc::c_int as libc::c_uchar,
    32 as libc::c_int as libc::c_uchar,
    33 as libc::c_int as libc::c_uchar,
    34 as libc::c_int as libc::c_uchar,
    35 as libc::c_int as libc::c_uchar,
    36 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    37 as libc::c_int as libc::c_uchar,
    40 as libc::c_int as libc::c_uchar,
    46 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    44 as libc::c_int as libc::c_uchar,
    43 as libc::c_int as libc::c_uchar,
    44 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    48 as libc::c_int as libc::c_uchar,
    49 as libc::c_int as libc::c_uchar,
    13 as libc::c_int as libc::c_uchar,
    17 as libc::c_int as libc::c_uchar,
    15 as libc::c_int as libc::c_uchar,
    47 as libc::c_int as libc::c_uchar,
    17 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    7 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    14 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    30 as libc::c_int as libc::c_uchar,
    27 as libc::c_int as libc::c_uchar,
    28 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    19 as libc::c_int as libc::c_uchar,
    13 as libc::c_int as libc::c_uchar,
    30 as libc::c_int as libc::c_uchar,
    15 as libc::c_int as libc::c_uchar,
    30 as libc::c_int as libc::c_uchar,
    17 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    13 as libc::c_int as libc::c_uchar,
    16 as libc::c_int as libc::c_uchar,
    15 as libc::c_int as libc::c_uchar,
    20 as libc::c_int as libc::c_uchar,
    17 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    18 as libc::c_int as libc::c_uchar,
    27 as libc::c_int as libc::c_uchar,
    28 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    27 as libc::c_int as libc::c_uchar,
    28 as libc::c_int as libc::c_uchar,
    10 as libc::c_int as libc::c_uchar,
    11 as libc::c_int as libc::c_uchar,
    13 as libc::c_int as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    16 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    17 as libc::c_int as libc::c_uchar,
    7 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    10 as libc::c_int as libc::c_uchar,
    14 as libc::c_int as libc::c_uchar,
    14 as libc::c_int as libc::c_uchar,
    37 as libc::c_int as libc::c_uchar,
    38 as libc::c_int as libc::c_uchar,
    39 as libc::c_int as libc::c_uchar,
    27 as libc::c_int as libc::c_uchar,
    28 as libc::c_int as libc::c_uchar,
    42 as libc::c_int as libc::c_uchar,
    43 as libc::c_int as libc::c_uchar,
    44 as libc::c_int as libc::c_uchar,
    45 as libc::c_int as libc::c_uchar,
    37 as libc::c_int as libc::c_uchar,
    38 as libc::c_int as libc::c_uchar,
    39 as libc::c_int as libc::c_uchar,
    29 as libc::c_int as libc::c_uchar,
    30 as libc::c_int as libc::c_uchar,
    51 as libc::c_int as libc::c_uchar,
    43 as libc::c_int as libc::c_uchar,
    44 as libc::c_int as libc::c_uchar,
    45 as libc::c_int as libc::c_uchar,
    51 as libc::c_int as libc::c_uchar,
    51 as libc::c_int as libc::c_uchar,
    51 as libc::c_int as libc::c_uchar,
    51 as libc::c_int as libc::c_uchar,
    21 as libc::c_int as libc::c_uchar,
    22 as libc::c_int as libc::c_uchar,
    23 as libc::c_int as libc::c_uchar,
    24 as libc::c_int as libc::c_uchar,
    25 as libc::c_int as libc::c_uchar,
    26 as libc::c_int as libc::c_uchar,
    51 as libc::c_int as libc::c_uchar,
    51 as libc::c_int as libc::c_uchar,
    37 as libc::c_int as libc::c_uchar,
    38 as libc::c_int as libc::c_uchar,
    51 as libc::c_int as libc::c_uchar,
    37 as libc::c_int as libc::c_uchar,
    38 as libc::c_int as libc::c_uchar,
    51 as libc::c_int as libc::c_uchar,
    43 as libc::c_int as libc::c_uchar,
    44 as libc::c_int as libc::c_uchar,
    45 as libc::c_int as libc::c_uchar,
    43 as libc::c_int as libc::c_uchar,
    44 as libc::c_int as libc::c_uchar,
    45 as libc::c_int as libc::c_uchar,
    37 as libc::c_int as libc::c_uchar,
    38 as libc::c_int as libc::c_uchar,
    51 as libc::c_int as libc::c_uchar,
    37 as libc::c_int as libc::c_uchar,
    38 as libc::c_int as libc::c_uchar,
    51 as libc::c_int as libc::c_uchar,
    43 as libc::c_int as libc::c_uchar,
    44 as libc::c_int as libc::c_uchar,
    45 as libc::c_int as libc::c_uchar,
    43 as libc::c_int as libc::c_uchar,
    44 as libc::c_int as libc::c_uchar,
    37 as libc::c_int as libc::c_uchar,
    38 as libc::c_int as libc::c_uchar,
    37 as libc::c_int as libc::c_uchar,
    38 as libc::c_int as libc::c_uchar,
    51 as libc::c_int as libc::c_uchar,
    51 as libc::c_int as libc::c_uchar,
    43 as libc::c_int as libc::c_uchar,
    44 as libc::c_int as libc::c_uchar,
    43 as libc::c_int as libc::c_uchar,
    44 as libc::c_int as libc::c_uchar,
    37 as libc::c_int as libc::c_uchar,
    38 as libc::c_int as libc::c_uchar,
    37 as libc::c_int as libc::c_uchar,
    38 as libc::c_int as libc::c_uchar,
    40 as libc::c_int as libc::c_uchar,
    41 as libc::c_int as libc::c_uchar,
    43 as libc::c_int as libc::c_uchar,
    44 as libc::c_int as libc::c_uchar,
    43 as libc::c_int as libc::c_uchar,
    44 as libc::c_int as libc::c_uchar,
    51 as libc::c_int as libc::c_uchar,
    51 as libc::c_int as libc::c_uchar,
    51 as libc::c_int as libc::c_uchar,
    49 as libc::c_int as libc::c_uchar,
    50 as libc::c_int as libc::c_uchar,
    29 as libc::c_int as libc::c_uchar,
    29 as libc::c_int as libc::c_uchar,
    29 as libc::c_int as libc::c_uchar,
    29 as libc::c_int as libc::c_uchar,
    29 as libc::c_int as libc::c_uchar,
    29 as libc::c_int as libc::c_uchar,
    29 as libc::c_int as libc::c_uchar,
    29 as libc::c_int as libc::c_uchar,
    29 as libc::c_int as libc::c_uchar,
    29 as libc::c_int as libc::c_uchar,
    29 as libc::c_int as libc::c_uchar,
    29 as libc::c_int as libc::c_uchar,
    29 as libc::c_int as libc::c_uchar,
    29 as libc::c_int as libc::c_uchar,
    29 as libc::c_int as libc::c_uchar,
    29 as libc::c_int as libc::c_uchar,
    29 as libc::c_int as libc::c_uchar,
    29 as libc::c_int as libc::c_uchar,
    29 as libc::c_int as libc::c_uchar,
    29 as libc::c_int as libc::c_uchar,
    29 as libc::c_int as libc::c_uchar,
    29 as libc::c_int as libc::c_uchar,
    29 as libc::c_int as libc::c_uchar,
    29 as libc::c_int as libc::c_uchar,
    29 as libc::c_int as libc::c_uchar,
    29 as libc::c_int as libc::c_uchar,
    29 as libc::c_int as libc::c_uchar,
    29 as libc::c_int as libc::c_uchar,
    29 as libc::c_int as libc::c_uchar,
];
static mut yy_shift_ofst: [libc::c_uchar; 40] = [
    150 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    24 as libc::c_int as libc::c_uchar,
    30 as libc::c_int as libc::c_uchar,
    48 as libc::c_int as libc::c_uchar,
    59 as libc::c_int as libc::c_uchar,
    19 as libc::c_int as libc::c_uchar,
    19 as libc::c_int as libc::c_uchar,
    19 as libc::c_int as libc::c_uchar,
    19 as libc::c_int as libc::c_uchar,
    19 as libc::c_int as libc::c_uchar,
    19 as libc::c_int as libc::c_uchar,
    19 as libc::c_int as libc::c_uchar,
    19 as libc::c_int as libc::c_uchar,
    19 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    19 as libc::c_int as libc::c_uchar,
    73 as libc::c_int as libc::c_uchar,
    15 as libc::c_int as libc::c_uchar,
    150 as libc::c_int as libc::c_uchar,
    150 as libc::c_int as libc::c_uchar,
    150 as libc::c_int as libc::c_uchar,
    52 as libc::c_int as libc::c_uchar,
    49 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    16 as libc::c_int as libc::c_uchar,
    36 as libc::c_int as libc::c_uchar,
    26 as libc::c_int as libc::c_uchar,
    36 as libc::c_int as libc::c_uchar,
    17 as libc::c_int as libc::c_uchar,
    32 as libc::c_int as libc::c_uchar,
    56 as libc::c_int as libc::c_uchar,
    36 as libc::c_int as libc::c_uchar,
    36 as libc::c_int as libc::c_uchar,
    36 as libc::c_int as libc::c_uchar,
    50 as libc::c_int as libc::c_uchar,
    36 as libc::c_int as libc::c_uchar,
    57 as libc::c_int as libc::c_uchar,
    28 as libc::c_int as libc::c_uchar,
    47 as libc::c_int as libc::c_uchar,
];
static mut yy_reduce_ofst: [libc::c_schar; 22] = [
    55 as libc::c_int as libc::c_schar,
    -(31 as libc::c_int) as libc::c_schar,
    -(31 as libc::c_int) as libc::c_schar,
    -(31 as libc::c_int) as libc::c_schar,
    -(31 as libc::c_int) as libc::c_schar,
    35 as libc::c_int as libc::c_schar,
    44 as libc::c_int as libc::c_schar,
    65 as libc::c_int as libc::c_schar,
    68 as libc::c_int as libc::c_schar,
    77 as libc::c_int as libc::c_schar,
    80 as libc::c_int as libc::c_schar,
    88 as libc::c_int as libc::c_schar,
    90 as libc::c_int as libc::c_schar,
    98 as libc::c_int as libc::c_schar,
    100 as libc::c_int as libc::c_schar,
    99 as libc::c_int as libc::c_schar,
    -(29 as libc::c_int) as libc::c_schar,
    -(36 as libc::c_int) as libc::c_schar,
    -(25 as libc::c_int) as libc::c_schar,
    2 as libc::c_int as libc::c_schar,
    8 as libc::c_int as libc::c_schar,
    10 as libc::c_int as libc::c_schar,
];
static mut yy_default: [libc::c_uchar; 40] = [
    159 as libc::c_int as libc::c_uchar,
    118 as libc::c_int as libc::c_uchar,
    118 as libc::c_int as libc::c_uchar,
    118 as libc::c_int as libc::c_uchar,
    118 as libc::c_int as libc::c_uchar,
    118 as libc::c_int as libc::c_uchar,
    135 as libc::c_int as libc::c_uchar,
    118 as libc::c_int as libc::c_uchar,
    118 as libc::c_int as libc::c_uchar,
    118 as libc::c_int as libc::c_uchar,
    118 as libc::c_int as libc::c_uchar,
    118 as libc::c_int as libc::c_uchar,
    118 as libc::c_int as libc::c_uchar,
    118 as libc::c_int as libc::c_uchar,
    118 as libc::c_int as libc::c_uchar,
    147 as libc::c_int as libc::c_uchar,
    118 as libc::c_int as libc::c_uchar,
    118 as libc::c_int as libc::c_uchar,
    166 as libc::c_int as libc::c_uchar,
    159 as libc::c_int as libc::c_uchar,
    159 as libc::c_int as libc::c_uchar,
    159 as libc::c_int as libc::c_uchar,
    118 as libc::c_int as libc::c_uchar,
    118 as libc::c_int as libc::c_uchar,
    137 as libc::c_int as libc::c_uchar,
    118 as libc::c_int as libc::c_uchar,
    146 as libc::c_int as libc::c_uchar,
    118 as libc::c_int as libc::c_uchar,
    154 as libc::c_int as libc::c_uchar,
    118 as libc::c_int as libc::c_uchar,
    118 as libc::c_int as libc::c_uchar,
    118 as libc::c_int as libc::c_uchar,
    124 as libc::c_int as libc::c_uchar,
    123 as libc::c_int as libc::c_uchar,
    138 as libc::c_int as libc::c_uchar,
    118 as libc::c_int as libc::c_uchar,
    122 as libc::c_int as libc::c_uchar,
    118 as libc::c_int as libc::c_uchar,
    118 as libc::c_int as libc::c_uchar,
    121 as libc::c_int as libc::c_uchar,
];
unsafe extern "C" fn configparserInit(mut yypRawParser: *mut libc::c_void) {
    let mut yypParser: *mut yyParser = yypRawParser as *mut yyParser;
    (*yypParser).yyerrcnt = -(1 as libc::c_int);
    (*yypParser).yytos = ((*yypParser).yystack).as_mut_ptr();
    (*yypParser)
        .yystack[0 as libc::c_int as usize]
        .stateno = 0 as libc::c_int as libc::c_uchar;
    (*yypParser)
        .yystack[0 as libc::c_int as usize]
        .major = 0 as libc::c_int as libc::c_uchar;
    (*yypParser)
        .yystackEnd = &mut *((*yypParser).yystack)
        .as_mut_ptr()
        .offset((100 as libc::c_int - 1 as libc::c_int) as isize) as *mut yyStackEntry;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn configparserAlloc(
    mut mallocProc: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
) -> *mut libc::c_void {
    let mut yypParser: *mut yyParser = 0 as *mut yyParser;
    yypParser = (Some(mallocProc.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(::core::mem::size_of::<yyParser>() as libc::c_ulong) as *mut yyParser;
    if !yypParser.is_null() {
        configparserInit(yypParser as *mut libc::c_void);
    }
    return yypParser as *mut libc::c_void;
}
unsafe extern "C" fn yy_destructor(
    mut yypParser: *mut yyParser,
    mut yymajor: libc::c_uchar,
    mut yypminor: *mut YYMINORTYPE,
) {
    let mut ctx: *mut config_t = (*yypParser).ctx;
    let mut current_block_9: u64;
    match yymajor as libc::c_int {
        1 => {
            current_block_9 = 7159190435856046257;
        }
        2 => {
            current_block_9 = 7159190435856046257;
        }
        3 => {
            current_block_9 = 10293610489198370764;
        }
        4 => {
            current_block_9 = 13028940918391719814;
        }
        5 => {
            current_block_9 = 3376895679810870753;
        }
        6 => {
            current_block_9 = 2706921823919522293;
        }
        7 => {
            current_block_9 = 14577463518471206087;
        }
        8 => {
            current_block_9 = 9769981902644550996;
        }
        9 => {
            current_block_9 = 17954308555434947575;
        }
        10 => {
            current_block_9 = 7956493946251855999;
        }
        11 => {
            current_block_9 = 12336299542597011755;
        }
        12 => {
            current_block_9 = 14394517079513442205;
        }
        13 => {
            current_block_9 = 15670310399476789445;
        }
        14 => {
            current_block_9 = 17148705401394580873;
        }
        15 => {
            current_block_9 = 3439635089883991044;
        }
        16 => {
            current_block_9 = 16277711289989004608;
        }
        17 => {
            current_block_9 = 15722309537388380679;
        }
        18 => {
            current_block_9 = 6898679946311925052;
        }
        19 => {
            current_block_9 = 6104607799170323156;
        }
        20 => {
            current_block_9 = 4114335408742026738;
        }
        21 => {
            current_block_9 = 5790641323087711348;
        }
        22 => {
            current_block_9 = 3961168104591128949;
        }
        23 => {
            current_block_9 = 7821053025542062995;
        }
        24 => {
            current_block_9 = 16974864242504997560;
        }
        25 => {
            current_block_9 = 3397287415529778187;
        }
        26 => {
            current_block_9 = 10655304992323330286;
        }
        27 => {
            current_block_9 = 14968114887822604789;
        }
        28 => {
            current_block_9 = 10914145571469894186;
        }
        38 => {
            current_block_9 = 16848555411549253182;
        }
        37 | 39 => {
            current_block_9 = 16848555411549253182;
        }
        42 | 43 => {
            array_free((*yypminor).yy66);
            current_block_9 = 13472856163611868459;
        }
        44 | 45 => {
            buffer_free((*yypminor).yy0);
            current_block_9 = 13472856163611868459;
        }
        _ => {
            current_block_9 = 13472856163611868459;
        }
    }
    match current_block_9 {
        7159190435856046257 => {
            current_block_9 = 10293610489198370764;
        }
        16848555411549253182 => {
            if !((*yypminor).yy35).is_null() {
                ((*(*(*yypminor).yy35).fn_0).free)
                    .expect("non-null function pointer")((*yypminor).yy35);
            }
            current_block_9 = 13472856163611868459;
        }
        _ => {}
    }
    match current_block_9 {
        10293610489198370764 => {
            current_block_9 = 13028940918391719814;
        }
        _ => {}
    }
    match current_block_9 {
        13028940918391719814 => {
            current_block_9 = 3376895679810870753;
        }
        _ => {}
    }
    match current_block_9 {
        3376895679810870753 => {
            current_block_9 = 2706921823919522293;
        }
        _ => {}
    }
    match current_block_9 {
        2706921823919522293 => {
            current_block_9 = 14577463518471206087;
        }
        _ => {}
    }
    match current_block_9 {
        14577463518471206087 => {
            current_block_9 = 9769981902644550996;
        }
        _ => {}
    }
    match current_block_9 {
        9769981902644550996 => {
            current_block_9 = 17954308555434947575;
        }
        _ => {}
    }
    match current_block_9 {
        17954308555434947575 => {
            current_block_9 = 7956493946251855999;
        }
        _ => {}
    }
    match current_block_9 {
        7956493946251855999 => {
            current_block_9 = 12336299542597011755;
        }
        _ => {}
    }
    match current_block_9 {
        12336299542597011755 => {
            current_block_9 = 14394517079513442205;
        }
        _ => {}
    }
    match current_block_9 {
        14394517079513442205 => {
            current_block_9 = 15670310399476789445;
        }
        _ => {}
    }
    match current_block_9 {
        15670310399476789445 => {
            current_block_9 = 17148705401394580873;
        }
        _ => {}
    }
    match current_block_9 {
        17148705401394580873 => {
            current_block_9 = 3439635089883991044;
        }
        _ => {}
    }
    match current_block_9 {
        3439635089883991044 => {
            current_block_9 = 16277711289989004608;
        }
        _ => {}
    }
    match current_block_9 {
        16277711289989004608 => {
            current_block_9 = 15722309537388380679;
        }
        _ => {}
    }
    match current_block_9 {
        15722309537388380679 => {
            current_block_9 = 6898679946311925052;
        }
        _ => {}
    }
    match current_block_9 {
        6898679946311925052 => {
            current_block_9 = 6104607799170323156;
        }
        _ => {}
    }
    match current_block_9 {
        6104607799170323156 => {
            current_block_9 = 4114335408742026738;
        }
        _ => {}
    }
    match current_block_9 {
        4114335408742026738 => {
            current_block_9 = 5790641323087711348;
        }
        _ => {}
    }
    match current_block_9 {
        5790641323087711348 => {
            current_block_9 = 3961168104591128949;
        }
        _ => {}
    }
    match current_block_9 {
        3961168104591128949 => {
            current_block_9 = 7821053025542062995;
        }
        _ => {}
    }
    match current_block_9 {
        7821053025542062995 => {
            current_block_9 = 16974864242504997560;
        }
        _ => {}
    }
    match current_block_9 {
        16974864242504997560 => {
            current_block_9 = 3397287415529778187;
        }
        _ => {}
    }
    match current_block_9 {
        3397287415529778187 => {
            current_block_9 = 10655304992323330286;
        }
        _ => {}
    }
    match current_block_9 {
        10655304992323330286 => {
            current_block_9 = 14968114887822604789;
        }
        _ => {}
    }
    match current_block_9 {
        14968114887822604789 => {
            current_block_9 = 10914145571469894186;
        }
        _ => {}
    }
    match current_block_9 {
        10914145571469894186 => {
            buffer_free((*yypminor).yy0);
        }
        _ => {}
    };
}
unsafe extern "C" fn yy_pop_parser_stack(mut pParser: *mut yyParser) {
    let mut yytos: *mut yyStackEntry = 0 as *mut yyStackEntry;
    let fresh2 = (*pParser).yytos;
    (*pParser).yytos = ((*pParser).yytos).offset(-1);
    yytos = fresh2;
    yy_destructor(pParser, (*yytos).major, &mut (*yytos).minor);
}
unsafe extern "C" fn configparserFinalize(mut p: *mut libc::c_void) {
    let mut pParser: *mut yyParser = p as *mut yyParser;
    while (*pParser).yytos > ((*pParser).yystack).as_mut_ptr() {
        yy_pop_parser_stack(pParser);
    }
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn configparserFree(
    mut p: *mut libc::c_void,
    mut freeProc: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
) {
    if p.is_null() {
        return;
    }
    configparserFinalize(p);
    (Some(freeProc.expect("non-null function pointer")))
        .expect("non-null function pointer")(p);
}
unsafe extern "C" fn yy_find_shift_action(
    mut iLookAhead: libc::c_uchar,
    mut stateno: libc::c_uchar,
) -> libc::c_uchar {
    let mut i: libc::c_int = 0;
    if stateno as libc::c_int > 39 as libc::c_int {
        return stateno;
    }
    i = yy_shift_ofst[stateno as usize] as libc::c_int;
    i += iLookAhead as libc::c_int;
    if yy_lookahead[i as usize] as libc::c_int != iLookAhead as libc::c_int {
        return yy_default[stateno as usize]
    } else {
        return yy_action[i as usize]
    };
}
unsafe extern "C" fn yy_find_reduce_action(
    mut stateno: libc::c_uchar,
    mut iLookAhead: libc::c_uchar,
) -> libc::c_uchar {
    let mut i: libc::c_int = 0;
    i = yy_reduce_ofst[stateno as usize] as libc::c_int;
    i += iLookAhead as libc::c_int;
    return yy_action[i as usize];
}
unsafe extern "C" fn yyStackOverflow(mut yypParser: *mut yyParser) {
    let mut ctx: *mut config_t = (*yypParser).ctx;
    while (*yypParser).yytos > ((*yypParser).yystack).as_mut_ptr() {
        yy_pop_parser_stack(yypParser);
    }
    (*yypParser).ctx = ctx;
}
unsafe extern "C" fn yy_shift(
    mut yypParser: *mut yyParser,
    mut yyNewState: libc::c_uchar,
    mut yyMajor: libc::c_uchar,
    mut yyMinor: *mut buffer,
) {
    let mut yytos: *mut yyStackEntry = 0 as *mut yyStackEntry;
    (*yypParser).yytos = ((*yypParser).yytos).offset(1);
    (*yypParser).yytos;
    if (*yypParser).yytos > (*yypParser).yystackEnd {
        (*yypParser).yytos = ((*yypParser).yytos).offset(-1);
        (*yypParser).yytos;
        yyStackOverflow(yypParser);
        return;
    }
    if yyNewState as libc::c_int > 39 as libc::c_int {
        yyNewState = (yyNewState as libc::c_int
            + (121 as libc::c_int - 72 as libc::c_int)) as libc::c_uchar;
    }
    yytos = (*yypParser).yytos;
    (*yytos).stateno = yyNewState;
    (*yytos).major = yyMajor;
    (*yytos).minor.yy0 = yyMinor;
}
static mut yyRuleInfoLhs: [libc::c_uchar; 46] = [
    31 as libc::c_int as libc::c_uchar,
    32 as libc::c_int as libc::c_uchar,
    32 as libc::c_int as libc::c_uchar,
    32 as libc::c_int as libc::c_uchar,
    44 as libc::c_int as libc::c_uchar,
    38 as libc::c_int as libc::c_uchar,
    38 as libc::c_int as libc::c_uchar,
    37 as libc::c_int as libc::c_uchar,
    37 as libc::c_int as libc::c_uchar,
    37 as libc::c_int as libc::c_uchar,
    37 as libc::c_int as libc::c_uchar,
    43 as libc::c_int as libc::c_uchar,
    43 as libc::c_int as libc::c_uchar,
    42 as libc::c_int as libc::c_uchar,
    42 as libc::c_int as libc::c_uchar,
    42 as libc::c_int as libc::c_uchar,
    39 as libc::c_int as libc::c_uchar,
    39 as libc::c_int as libc::c_uchar,
    48 as libc::c_int as libc::c_uchar,
    33 as libc::c_int as libc::c_uchar,
    34 as libc::c_int as libc::c_uchar,
    34 as libc::c_int as libc::c_uchar,
    34 as libc::c_int as libc::c_uchar,
    40 as libc::c_int as libc::c_uchar,
    41 as libc::c_int as libc::c_uchar,
    49 as libc::c_int as libc::c_uchar,
    50 as libc::c_int as libc::c_uchar,
    46 as libc::c_int as libc::c_uchar,
    46 as libc::c_int as libc::c_uchar,
    46 as libc::c_int as libc::c_uchar,
    46 as libc::c_int as libc::c_uchar,
    46 as libc::c_int as libc::c_uchar,
    46 as libc::c_int as libc::c_uchar,
    45 as libc::c_int as libc::c_uchar,
    35 as libc::c_int as libc::c_uchar,
    36 as libc::c_int as libc::c_uchar,
    29 as libc::c_int as libc::c_uchar,
    30 as libc::c_int as libc::c_uchar,
    30 as libc::c_int as libc::c_uchar,
    31 as libc::c_int as libc::c_uchar,
    31 as libc::c_int as libc::c_uchar,
    31 as libc::c_int as libc::c_uchar,
    31 as libc::c_int as libc::c_uchar,
    31 as libc::c_int as libc::c_uchar,
    47 as libc::c_int as libc::c_uchar,
    47 as libc::c_int as libc::c_uchar,
];
static mut yyRuleInfoNRhs: [libc::c_schar; 46] = [
    -(2 as libc::c_int) as libc::c_schar,
    -(3 as libc::c_int) as libc::c_schar,
    -(3 as libc::c_int) as libc::c_schar,
    -(3 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(3 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(3 as libc::c_int) as libc::c_schar,
    -(3 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(3 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(4 as libc::c_int) as libc::c_schar,
    -(4 as libc::c_int) as libc::c_schar,
    -(4 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(4 as libc::c_int) as libc::c_schar,
    -(4 as libc::c_int) as libc::c_schar,
    -(7 as libc::c_int) as libc::c_schar,
    0 as libc::c_int as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(2 as libc::c_int) as libc::c_schar,
    0 as libc::c_int as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    0 as libc::c_int as libc::c_schar,
];
unsafe extern "C" fn yy_reduce(
    mut yypParser: *mut yyParser,
    mut yyruleno: libc::c_uint,
    mut yyLookahead: libc::c_int,
    mut yyLookaheadToken: *mut buffer,
) -> libc::c_uchar {
    let mut yygoto: libc::c_int = 0;
    let mut yyact: libc::c_uchar = 0;
    let mut yymsp: *mut yyStackEntry = 0 as *mut yyStackEntry;
    let mut yysize: libc::c_int = 0;
    let mut ctx: *mut config_t = (*yypParser).ctx;
    yymsp = (*yypParser).yytos;
    let mut yylhsminor: YYMINORTYPE = YYMINORTYPE { yyinit: 0 };
    let mut current_block_393: u64;
    match yyruleno {
        0 => {
            let ref mut fresh3 = (*yymsp.offset(-(1 as libc::c_int) as isize))
                .minor
                .yy20;
            *fresh3 = 0 as *mut data_config;
            yy_destructor(
                yypParser,
                1 as libc::c_int as libc::c_uchar,
                &mut (*yymsp.offset(0 as libc::c_int as isize)).minor,
            );
            current_block_393 = 10953711258009896266;
        }
        1 => {
            if (*ctx).ok != 0 {
                buffer_copy_buffer(
                    &mut (*(*yymsp.offset(0 as libc::c_int as isize)).minor.yy35).key,
                    (*yymsp.offset(-(2 as libc::c_int) as isize)).minor.yy0,
                );
                if strncmp(
                    (*(*yymsp.offset(-(2 as libc::c_int) as isize)).minor.yy0).ptr,
                    b"env.\0" as *const u8 as *const libc::c_char,
                    (::core::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                ) == 0 as libc::c_int
                {
                    fprintf(
                        stderr,
                        b"Setting env variable is not supported in conditional %d %s: %s\n\0"
                            as *const u8 as *const libc::c_char,
                        (*(*ctx).current).context_ndx,
                        (*(*ctx).current).key.ptr,
                        (*(*yymsp.offset(-(2 as libc::c_int) as isize)).minor.yy0).ptr,
                    );
                    (*ctx).ok = 0 as libc::c_int;
                } else if (array_get_element_klen(
                    (*(*ctx).current).value,
                    (*(*yymsp.offset(0 as libc::c_int as isize)).minor.yy35).key.ptr,
                    buffer_clen(
                        &mut (*(*yymsp.offset(0 as libc::c_int as isize)).minor.yy35).key,
                    ),
                ))
                    .is_null()
                {
                    array_insert_unique(
                        (*(*ctx).current).value,
                        (*yymsp.offset(0 as libc::c_int as isize)).minor.yy35,
                    );
                    let ref mut fresh4 = (*yymsp.offset(0 as libc::c_int as isize))
                        .minor
                        .yy35;
                    *fresh4 = 0 as *mut data_unset;
                } else {
                    fprintf(
                        stderr,
                        b"Duplicate config variable in conditional %d %s: %s\n\0"
                            as *const u8 as *const libc::c_char,
                        (*(*ctx).current).context_ndx,
                        (*(*ctx).current).key.ptr,
                        (*(*yymsp.offset(0 as libc::c_int as isize)).minor.yy35).key.ptr,
                    );
                    (*ctx).ok = 0 as libc::c_int;
                }
            }
            buffer_free((*yymsp.offset(-(2 as libc::c_int) as isize)).minor.yy0);
            let ref mut fresh5 = (*yymsp.offset(-(2 as libc::c_int) as isize)).minor.yy0;
            *fresh5 = 0 as *mut buffer;
            if !((*yymsp.offset(0 as libc::c_int as isize)).minor.yy35).is_null() {
                ((*(*(*yymsp.offset(0 as libc::c_int as isize)).minor.yy35).fn_0).free)
                    .expect(
                        "non-null function pointer",
                    )((*yymsp.offset(0 as libc::c_int as isize)).minor.yy35);
            }
            let ref mut fresh6 = (*yymsp.offset(0 as libc::c_int as isize)).minor.yy35;
            *fresh6 = 0 as *mut data_unset;
            yy_destructor(
                yypParser,
                2 as libc::c_int as libc::c_uchar,
                &mut (*yymsp.offset(-(1 as libc::c_int) as isize)).minor,
            );
            current_block_393 = 10953711258009896266;
        }
        2 => {
            if (*ctx).ok != 0 {
                if strncmp(
                    (*(*yymsp.offset(-(2 as libc::c_int) as isize)).minor.yy0).ptr,
                    b"env.\0" as *const u8 as *const libc::c_char,
                    (::core::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                ) == 0 as libc::c_int
                {
                    fprintf(
                        stderr,
                        b"Setting env variable is not supported in conditional %d %s: %s\n\0"
                            as *const u8 as *const libc::c_char,
                        (*(*ctx).current).context_ndx,
                        (*(*ctx).current).key.ptr,
                        (*(*yymsp.offset(-(2 as libc::c_int) as isize)).minor.yy0).ptr,
                    );
                    (*ctx).ok = 0 as libc::c_int;
                } else {
                    buffer_copy_buffer(
                        &mut (*(*yymsp.offset(0 as libc::c_int as isize)).minor.yy35)
                            .key,
                        (*yymsp.offset(-(2 as libc::c_int) as isize)).minor.yy0,
                    );
                    array_replace(
                        (*(*ctx).current).value,
                        (*yymsp.offset(0 as libc::c_int as isize)).minor.yy35,
                    );
                    let ref mut fresh7 = (*yymsp.offset(0 as libc::c_int as isize))
                        .minor
                        .yy35;
                    *fresh7 = 0 as *mut data_unset;
                }
            }
            buffer_free((*yymsp.offset(-(2 as libc::c_int) as isize)).minor.yy0);
            let ref mut fresh8 = (*yymsp.offset(-(2 as libc::c_int) as isize)).minor.yy0;
            *fresh8 = 0 as *mut buffer;
            if !((*yymsp.offset(0 as libc::c_int as isize)).minor.yy35).is_null() {
                ((*(*(*yymsp.offset(0 as libc::c_int as isize)).minor.yy35).fn_0).free)
                    .expect(
                        "non-null function pointer",
                    )((*yymsp.offset(0 as libc::c_int as isize)).minor.yy35);
            }
            let ref mut fresh9 = (*yymsp.offset(0 as libc::c_int as isize)).minor.yy35;
            *fresh9 = 0 as *mut data_unset;
            yy_destructor(
                yypParser,
                3 as libc::c_int as libc::c_uchar,
                &mut (*yymsp.offset(-(1 as libc::c_int) as isize)).minor,
            );
            current_block_393 = 10953711258009896266;
        }
        3 => {
            if (*ctx).ok != 0 {
                let mut vars: *mut array = (*(*ctx).current).value;
                let mut du: *mut data_unset = 0 as *mut data_unset;
                if strncmp(
                    (*(*yymsp.offset(-(2 as libc::c_int) as isize)).minor.yy0).ptr,
                    b"env.\0" as *const u8 as *const libc::c_char,
                    (::core::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                ) == 0 as libc::c_int
                {
                    fprintf(
                        stderr,
                        b"Appending env variable is not supported in conditional %d %s: %s\n\0"
                            as *const u8 as *const libc::c_char,
                        (*(*ctx).current).context_ndx,
                        (*(*ctx).current).key.ptr,
                        (*(*yymsp.offset(-(2 as libc::c_int) as isize)).minor.yy0).ptr,
                    );
                    (*ctx).ok = 0 as libc::c_int;
                } else {
                    du = array_extract_element_klen(
                        vars,
                        (*(*yymsp.offset(-(2 as libc::c_int) as isize)).minor.yy0).ptr,
                        buffer_clen(
                            (*yymsp.offset(-(2 as libc::c_int) as isize)).minor.yy0,
                        ),
                    );
                    if !du.is_null()
                        || {
                            du = configparser_get_variable(
                                ctx,
                                (*yymsp.offset(-(2 as libc::c_int) as isize)).minor.yy0,
                            );
                            !du.is_null()
                        }
                    {
                        du = configparser_merge_data(
                            du,
                            (*yymsp.offset(0 as libc::c_int as isize)).minor.yy35,
                        );
                        if du.is_null() {
                            (*ctx).ok = 0 as libc::c_int;
                        } else {
                            buffer_copy_buffer(
                                &mut (*du).key,
                                (*yymsp.offset(-(2 as libc::c_int) as isize)).minor.yy0,
                            );
                            array_insert_unique((*(*ctx).current).value, du);
                        }
                    } else {
                        buffer_copy_buffer(
                            &mut (*(*yymsp.offset(0 as libc::c_int as isize)).minor.yy35)
                                .key,
                            (*yymsp.offset(-(2 as libc::c_int) as isize)).minor.yy0,
                        );
                        array_insert_unique(
                            (*(*ctx).current).value,
                            (*yymsp.offset(0 as libc::c_int as isize)).minor.yy35,
                        );
                        let ref mut fresh10 = (*yymsp.offset(0 as libc::c_int as isize))
                            .minor
                            .yy35;
                        *fresh10 = 0 as *mut data_unset;
                    }
                }
            }
            buffer_free((*yymsp.offset(-(2 as libc::c_int) as isize)).minor.yy0);
            let ref mut fresh11 = (*yymsp.offset(-(2 as libc::c_int) as isize))
                .minor
                .yy0;
            *fresh11 = 0 as *mut buffer;
            if !((*yymsp.offset(0 as libc::c_int as isize)).minor.yy35).is_null() {
                ((*(*(*yymsp.offset(0 as libc::c_int as isize)).minor.yy35).fn_0).free)
                    .expect(
                        "non-null function pointer",
                    )((*yymsp.offset(0 as libc::c_int as isize)).minor.yy35);
            }
            let ref mut fresh12 = (*yymsp.offset(0 as libc::c_int as isize)).minor.yy35;
            *fresh12 = 0 as *mut data_unset;
            yy_destructor(
                yypParser,
                4 as libc::c_int as libc::c_uchar,
                &mut (*yymsp.offset(-(1 as libc::c_int) as isize)).minor,
            );
            current_block_393 = 10953711258009896266;
        }
        4 => {
            if (strchr(
                (*(*yymsp.offset(0 as libc::c_int as isize)).minor.yy0).ptr,
                '.' as i32,
            ))
                .is_null()
            {
                yylhsminor.yy0 = buffer_init();
                buffer_copy_string(
                    yylhsminor.yy0,
                    b"var.\0" as *const u8 as *const libc::c_char,
                );
                buffer_append_buffer(
                    yylhsminor.yy0,
                    (*yymsp.offset(0 as libc::c_int as isize)).minor.yy0,
                );
            } else {
                yylhsminor.yy0 = (*yymsp.offset(0 as libc::c_int as isize)).minor.yy0;
                let ref mut fresh13 = (*yymsp.offset(0 as libc::c_int as isize))
                    .minor
                    .yy0;
                *fresh13 = 0 as *mut buffer;
            }
            buffer_free((*yymsp.offset(0 as libc::c_int as isize)).minor.yy0);
            let ref mut fresh14 = (*yymsp.offset(0 as libc::c_int as isize)).minor.yy0;
            *fresh14 = 0 as *mut buffer;
            let ref mut fresh15 = (*yymsp.offset(0 as libc::c_int as isize)).minor.yy0;
            *fresh15 = yylhsminor.yy0;
            current_block_393 = 10953711258009896266;
        }
        5 => {
            yylhsminor.yy35 = 0 as *mut data_unset;
            if (*ctx).ok != 0 {
                yylhsminor
                    .yy35 = configparser_merge_data(
                    (*yymsp.offset(-(2 as libc::c_int) as isize)).minor.yy35,
                    (*yymsp.offset(0 as libc::c_int as isize)).minor.yy35,
                );
                let ref mut fresh16 = (*yymsp.offset(-(2 as libc::c_int) as isize))
                    .minor
                    .yy35;
                *fresh16 = 0 as *mut data_unset;
                if (yylhsminor.yy35).is_null() {
                    (*ctx).ok = 0 as libc::c_int;
                }
            }
            if !((*yymsp.offset(-(2 as libc::c_int) as isize)).minor.yy35).is_null() {
                ((*(*(*yymsp.offset(-(2 as libc::c_int) as isize)).minor.yy35).fn_0)
                    .free)
                    .expect(
                        "non-null function pointer",
                    )((*yymsp.offset(-(2 as libc::c_int) as isize)).minor.yy35);
            }
            let ref mut fresh17 = (*yymsp.offset(-(2 as libc::c_int) as isize))
                .minor
                .yy35;
            *fresh17 = 0 as *mut data_unset;
            if !((*yymsp.offset(0 as libc::c_int as isize)).minor.yy35).is_null() {
                ((*(*(*yymsp.offset(0 as libc::c_int as isize)).minor.yy35).fn_0).free)
                    .expect(
                        "non-null function pointer",
                    )((*yymsp.offset(0 as libc::c_int as isize)).minor.yy35);
            }
            let ref mut fresh18 = (*yymsp.offset(0 as libc::c_int as isize)).minor.yy35;
            *fresh18 = 0 as *mut data_unset;
            yy_destructor(
                yypParser,
                6 as libc::c_int as libc::c_uchar,
                &mut (*yymsp.offset(-(1 as libc::c_int) as isize)).minor,
            );
            let ref mut fresh19 = (*yymsp.offset(-(2 as libc::c_int) as isize))
                .minor
                .yy35;
            *fresh19 = yylhsminor.yy35;
            current_block_393 = 10953711258009896266;
        }
        6 => {
            current_block_393 = 2472048668343472511;
        }
        16 => {
            current_block_393 = 2472048668343472511;
        }
        7 => {
            yylhsminor.yy35 = 0 as *mut data_unset;
            if (*ctx).ok != 0 {
                if strncmp(
                    (*(*yymsp.offset(0 as libc::c_int as isize)).minor.yy0).ptr,
                    b"env.\0" as *const u8 as *const libc::c_char,
                    (::core::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                ) == 0 as libc::c_int
                {
                    let mut env: *mut libc::c_char = 0 as *mut libc::c_char;
                    env = getenv(
                        ((*(*yymsp.offset(0 as libc::c_int as isize)).minor.yy0).ptr)
                            .offset(4 as libc::c_int as isize),
                    );
                    if !env.is_null() {
                        let mut ds: *mut data_string = 0 as *mut data_string;
                        ds = array_data_string_init();
                        buffer_append_string(&mut (*ds).value, env);
                        yylhsminor.yy35 = ds as *mut data_unset;
                    } else {
                        fprintf(
                            stderr,
                            b"Undefined env variable: %s\n\0" as *const u8
                                as *const libc::c_char,
                            ((*(*yymsp.offset(0 as libc::c_int as isize)).minor.yy0).ptr)
                                .offset(4 as libc::c_int as isize),
                        );
                        (*ctx).ok = 0 as libc::c_int;
                    }
                } else {
                    yylhsminor
                        .yy35 = configparser_get_variable(
                        ctx,
                        (*yymsp.offset(0 as libc::c_int as isize)).minor.yy0,
                    );
                    if (yylhsminor.yy35).is_null() {
                        fprintf(
                            stderr,
                            b"Undefined config variable: %s\n\0" as *const u8
                                as *const libc::c_char,
                            (*(*yymsp.offset(0 as libc::c_int as isize)).minor.yy0).ptr,
                        );
                        (*ctx).ok = 0 as libc::c_int;
                    }
                }
            }
            buffer_free((*yymsp.offset(0 as libc::c_int as isize)).minor.yy0);
            let ref mut fresh22 = (*yymsp.offset(0 as libc::c_int as isize)).minor.yy0;
            *fresh22 = 0 as *mut buffer;
            let ref mut fresh23 = (*yymsp.offset(0 as libc::c_int as isize)).minor.yy35;
            *fresh23 = yylhsminor.yy35;
            current_block_393 = 10953711258009896266;
        }
        8 => {
            yylhsminor.yy35 = array_data_string_init() as *mut data_unset;
            memcpy(
                &mut (*(yylhsminor.yy35 as *mut data_string)).value as *mut buffer
                    as *mut libc::c_void,
                (*yymsp.offset(0 as libc::c_int as isize)).minor.yy0
                    as *const libc::c_void,
                ::core::mem::size_of::<buffer>() as libc::c_ulong,
            );
            free(
                (*yymsp.offset(0 as libc::c_int as isize)).minor.yy0 as *mut libc::c_void,
            );
            let ref mut fresh24 = (*yymsp.offset(0 as libc::c_int as isize)).minor.yy0;
            *fresh24 = 0 as *mut buffer;
            let ref mut fresh25 = (*yymsp.offset(0 as libc::c_int as isize)).minor.yy35;
            *fresh25 = yylhsminor.yy35;
            current_block_393 = 10953711258009896266;
        }
        9 => {
            let mut endptr: *mut libc::c_char = 0 as *mut libc::c_char;
            yylhsminor.yy35 = array_data_integer_init() as *mut data_unset;
            *__errno_location() = 0 as libc::c_int;
            (*(yylhsminor.yy35 as *mut data_integer))
                .value = strtol(
                (*(*yymsp.offset(0 as libc::c_int as isize)).minor.yy0).ptr,
                &mut endptr,
                10 as libc::c_int,
            ) as libc::c_int;
            if endptr != (*(*yymsp.offset(0 as libc::c_int as isize)).minor.yy0).ptr {
                while *(*__ctype_b_loc())
                    .offset(*(endptr as *mut libc::c_uchar) as libc::c_int as isize)
                    as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    endptr = endptr.offset(1);
                    endptr;
                }
            }
            if 0 as libc::c_int != *__errno_location()
                || *endptr as libc::c_int != '\0' as i32
            {
                fprintf(
                    stderr,
                    b"error parsing number: '%s'\n\0" as *const u8
                        as *const libc::c_char,
                    (*(*yymsp.offset(0 as libc::c_int as isize)).minor.yy0).ptr,
                );
                (*ctx).ok = 0 as libc::c_int;
            }
            buffer_free((*yymsp.offset(0 as libc::c_int as isize)).minor.yy0);
            let ref mut fresh26 = (*yymsp.offset(0 as libc::c_int as isize)).minor.yy0;
            *fresh26 = 0 as *mut buffer;
            let ref mut fresh27 = (*yymsp.offset(0 as libc::c_int as isize)).minor.yy35;
            *fresh27 = yylhsminor.yy35;
            current_block_393 = 10953711258009896266;
        }
        10 => {
            yylhsminor.yy35 = array_data_array_init() as *mut data_unset;
            memcpy(
                &mut (*(yylhsminor.yy35 as *mut data_array)).value as *mut array
                    as *mut libc::c_void,
                (*yymsp.offset(0 as libc::c_int as isize)).minor.yy66
                    as *const libc::c_void,
                ::core::mem::size_of::<array>() as libc::c_ulong,
            );
            free(
                (*yymsp.offset(0 as libc::c_int as isize)).minor.yy66
                    as *mut libc::c_void,
            );
            let ref mut fresh28 = (*yymsp.offset(0 as libc::c_int as isize)).minor.yy66;
            *fresh28 = 0 as *mut array;
            let ref mut fresh29 = (*yymsp.offset(0 as libc::c_int as isize)).minor.yy35;
            *fresh29 = yylhsminor.yy35;
            current_block_393 = 10953711258009896266;
        }
        11 => {
            yy_destructor(
                yypParser,
                9 as libc::c_int as libc::c_uchar,
                &mut (*yymsp.offset(-(1 as libc::c_int) as isize)).minor,
            );
            let ref mut fresh30 = (*yymsp.offset(-(1 as libc::c_int) as isize))
                .minor
                .yy66;
            *fresh30 = array_init(8 as libc::c_int as uint32_t);
            yy_destructor(
                yypParser,
                10 as libc::c_int as libc::c_uchar,
                &mut (*yymsp.offset(0 as libc::c_int as isize)).minor,
            );
            current_block_393 = 10953711258009896266;
        }
        12 => {
            yy_destructor(
                yypParser,
                9 as libc::c_int as libc::c_uchar,
                &mut (*yymsp.offset(-(2 as libc::c_int) as isize)).minor,
            );
            let ref mut fresh31 = (*yymsp.offset(-(2 as libc::c_int) as isize))
                .minor
                .yy66;
            *fresh31 = (*yymsp.offset(-(1 as libc::c_int) as isize)).minor.yy66;
            let ref mut fresh32 = (*yymsp.offset(-(1 as libc::c_int) as isize))
                .minor
                .yy66;
            *fresh32 = 0 as *mut array;
            yy_destructor(
                yypParser,
                10 as libc::c_int as libc::c_uchar,
                &mut (*yymsp.offset(0 as libc::c_int as isize)).minor,
            );
            current_block_393 = 10953711258009896266;
        }
        13 => {
            yylhsminor.yy66 = 0 as *mut array;
            if (*ctx).ok != 0 {
                if buffer_is_unset(
                    &mut (*(*yymsp.offset(0 as libc::c_int as isize)).minor.yy35).key,
                ) != 0
                    || (array_get_element_klen(
                        (*yymsp.offset(-(2 as libc::c_int) as isize)).minor.yy66,
                        (*(*yymsp.offset(0 as libc::c_int as isize)).minor.yy35).key.ptr,
                        buffer_clen(
                            &mut (*(*yymsp.offset(0 as libc::c_int as isize)).minor.yy35)
                                .key,
                        ),
                    ))
                        .is_null()
                {
                    array_insert_unique(
                        (*yymsp.offset(-(2 as libc::c_int) as isize)).minor.yy66,
                        (*yymsp.offset(0 as libc::c_int as isize)).minor.yy35,
                    );
                    let ref mut fresh33 = (*yymsp.offset(0 as libc::c_int as isize))
                        .minor
                        .yy35;
                    *fresh33 = 0 as *mut data_unset;
                } else {
                    fprintf(
                        stderr,
                        b"Error: duplicate array-key: %s. Please get rid of the duplicate entry.\n\0"
                            as *const u8 as *const libc::c_char,
                        (*(*yymsp.offset(0 as libc::c_int as isize)).minor.yy35).key.ptr,
                    );
                    (*ctx).ok = 0 as libc::c_int;
                }
                yylhsminor
                    .yy66 = (*yymsp.offset(-(2 as libc::c_int) as isize)).minor.yy66;
                let ref mut fresh34 = (*yymsp.offset(-(2 as libc::c_int) as isize))
                    .minor
                    .yy66;
                *fresh34 = 0 as *mut array;
            }
            array_free((*yymsp.offset(-(2 as libc::c_int) as isize)).minor.yy66);
            let ref mut fresh35 = (*yymsp.offset(-(2 as libc::c_int) as isize))
                .minor
                .yy66;
            *fresh35 = 0 as *mut array;
            if !((*yymsp.offset(0 as libc::c_int as isize)).minor.yy35).is_null() {
                ((*(*(*yymsp.offset(0 as libc::c_int as isize)).minor.yy35).fn_0).free)
                    .expect(
                        "non-null function pointer",
                    )((*yymsp.offset(0 as libc::c_int as isize)).minor.yy35);
            }
            let ref mut fresh36 = (*yymsp.offset(0 as libc::c_int as isize)).minor.yy35;
            *fresh36 = 0 as *mut data_unset;
            yy_destructor(
                yypParser,
                11 as libc::c_int as libc::c_uchar,
                &mut (*yymsp.offset(-(1 as libc::c_int) as isize)).minor,
            );
            let ref mut fresh37 = (*yymsp.offset(-(2 as libc::c_int) as isize))
                .minor
                .yy66;
            *fresh37 = yylhsminor.yy66;
            current_block_393 = 10953711258009896266;
        }
        14 => {
            yylhsminor.yy66 = (*yymsp.offset(-(1 as libc::c_int) as isize)).minor.yy66;
            let ref mut fresh38 = (*yymsp.offset(-(1 as libc::c_int) as isize))
                .minor
                .yy66;
            *fresh38 = 0 as *mut array;
            yy_destructor(
                yypParser,
                11 as libc::c_int as libc::c_uchar,
                &mut (*yymsp.offset(0 as libc::c_int as isize)).minor,
            );
            let ref mut fresh39 = (*yymsp.offset(-(1 as libc::c_int) as isize))
                .minor
                .yy66;
            *fresh39 = yylhsminor.yy66;
            current_block_393 = 10953711258009896266;
        }
        15 => {
            yylhsminor.yy66 = 0 as *mut array;
            if (*ctx).ok != 0 {
                yylhsminor.yy66 = array_init(4 as libc::c_int as uint32_t);
                array_insert_unique(
                    yylhsminor.yy66,
                    (*yymsp.offset(0 as libc::c_int as isize)).minor.yy35,
                );
                let ref mut fresh40 = (*yymsp.offset(0 as libc::c_int as isize))
                    .minor
                    .yy35;
                *fresh40 = 0 as *mut data_unset;
            }
            if !((*yymsp.offset(0 as libc::c_int as isize)).minor.yy35).is_null() {
                ((*(*(*yymsp.offset(0 as libc::c_int as isize)).minor.yy35).fn_0).free)
                    .expect(
                        "non-null function pointer",
                    )((*yymsp.offset(0 as libc::c_int as isize)).minor.yy35);
            }
            let ref mut fresh41 = (*yymsp.offset(0 as libc::c_int as isize)).minor.yy35;
            *fresh41 = 0 as *mut data_unset;
            let ref mut fresh42 = (*yymsp.offset(0 as libc::c_int as isize)).minor.yy66;
            *fresh42 = yylhsminor.yy66;
            current_block_393 = 10953711258009896266;
        }
        17 => {
            yylhsminor.yy35 = 0 as *mut data_unset;
            if (*ctx).ok != 0 {
                buffer_copy_buffer(
                    &mut (*(*yymsp.offset(0 as libc::c_int as isize)).minor.yy35).key,
                    (*yymsp.offset(-(2 as libc::c_int) as isize)).minor.yy0,
                );
                yylhsminor.yy35 = (*yymsp.offset(0 as libc::c_int as isize)).minor.yy35;
                let ref mut fresh43 = (*yymsp.offset(0 as libc::c_int as isize))
                    .minor
                    .yy35;
                *fresh43 = 0 as *mut data_unset;
            }
            if !((*yymsp.offset(0 as libc::c_int as isize)).minor.yy35).is_null() {
                ((*(*(*yymsp.offset(0 as libc::c_int as isize)).minor.yy35).fn_0).free)
                    .expect(
                        "non-null function pointer",
                    )((*yymsp.offset(0 as libc::c_int as isize)).minor.yy35);
            }
            let ref mut fresh44 = (*yymsp.offset(0 as libc::c_int as isize)).minor.yy35;
            *fresh44 = 0 as *mut data_unset;
            buffer_free((*yymsp.offset(-(2 as libc::c_int) as isize)).minor.yy0);
            let ref mut fresh45 = (*yymsp.offset(-(2 as libc::c_int) as isize))
                .minor
                .yy0;
            *fresh45 = 0 as *mut buffer;
            yy_destructor(
                yypParser,
                12 as libc::c_int as libc::c_uchar,
                &mut (*yymsp.offset(-(1 as libc::c_int) as isize)).minor,
            );
            let ref mut fresh46 = (*yymsp.offset(-(2 as libc::c_int) as isize))
                .minor
                .yy35;
            *fresh46 = yylhsminor.yy35;
            current_block_393 = 10953711258009896266;
        }
        18 => {
            yy_destructor(
                yypParser,
                13 as libc::c_int as libc::c_uchar,
                &mut (*yymsp.offset(0 as libc::c_int as isize)).minor,
            );
            let mut dc: *mut data_config = 0 as *mut data_config;
            dc = configparser_get_data_config(
                (*(*ctx).srv).config_context,
                b"global\0" as *const u8 as *const libc::c_char,
                (::core::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            );
            configparser_push(ctx, dc, 0 as libc::c_int);
            current_block_393 = 10953711258009896266;
        }
        19 => {
            configparser_pop(ctx);
            yy_destructor(
                yypParser,
                14 as libc::c_int as libc::c_uchar,
                &mut (*yymsp.offset(-(2 as libc::c_int) as isize)).minor,
            );
            yy_destructor(
                yypParser,
                15 as libc::c_int as libc::c_uchar,
                &mut (*yymsp.offset(0 as libc::c_int as isize)).minor,
            );
            current_block_393 = 10953711258009896266;
        }
        20 => {
            yylhsminor.yy20 = 0 as *mut data_config;
            if (*ctx).ok != 0 {
                if (*(*yymsp.offset(-(3 as libc::c_int) as isize)).minor.yy20)
                    .context_ndx
                    >= (*(*yymsp.offset(0 as libc::c_int as isize)).minor.yy20)
                        .context_ndx
                {
                    fprintf(
                        stderr,
                        b"unreachable else condition\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    (*ctx).ok = 0 as libc::c_int;
                }
                if (*(*yymsp.offset(-(3 as libc::c_int) as isize)).minor.yy20).cond
                    as libc::c_uint == CONFIG_COND_ELSE as libc::c_int as libc::c_uint
                {
                    fprintf(
                        stderr,
                        b"unreachable condition following else catch-all\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                    (*ctx).ok = 0 as libc::c_int;
                }
                let ref mut fresh47 = (*(*yymsp.offset(0 as libc::c_int as isize))
                    .minor
                    .yy20)
                    .prev;
                *fresh47 = (*yymsp.offset(-(3 as libc::c_int) as isize)).minor.yy20;
                let ref mut fresh48 = (*(*yymsp.offset(-(3 as libc::c_int) as isize))
                    .minor
                    .yy20)
                    .next;
                *fresh48 = (*yymsp.offset(0 as libc::c_int as isize)).minor.yy20;
                yylhsminor.yy20 = (*yymsp.offset(0 as libc::c_int as isize)).minor.yy20;
            }
            let ref mut fresh49 = (*yymsp.offset(-(3 as libc::c_int) as isize))
                .minor
                .yy20;
            *fresh49 = 0 as *mut data_config;
            let ref mut fresh50 = (*yymsp.offset(0 as libc::c_int as isize)).minor.yy20;
            *fresh50 = 0 as *mut data_config;
            yy_destructor(
                yypParser,
                16 as libc::c_int as libc::c_uchar,
                &mut (*yymsp.offset(-(1 as libc::c_int) as isize)).minor,
            );
            let ref mut fresh51 = (*yymsp.offset(-(3 as libc::c_int) as isize))
                .minor
                .yy20;
            *fresh51 = yylhsminor.yy20;
            current_block_393 = 10953711258009896266;
        }
        21 => {
            yylhsminor.yy20 = 0 as *mut data_config;
            if (*ctx).ok != 0 {
                if (*(*yymsp.offset(-(3 as libc::c_int) as isize)).minor.yy20)
                    .context_ndx
                    >= (*(*yymsp.offset(0 as libc::c_int as isize)).minor.yy20)
                        .context_ndx
                {
                    fprintf(
                        stderr,
                        b"unreachable else condition\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    (*ctx).ok = 0 as libc::c_int;
                }
                if (*(*yymsp.offset(-(3 as libc::c_int) as isize)).minor.yy20).cond
                    as libc::c_uint == CONFIG_COND_ELSE as libc::c_int as libc::c_uint
                {
                    fprintf(
                        stderr,
                        b"unreachable condition following else catch-all\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                    (*ctx).ok = 0 as libc::c_int;
                }
            }
            if (*ctx).ok != 0 {
                let mut dc_0: *mut data_config = 0 as *mut data_config;
                dc_0 = array_extract_element_klen(
                    (*ctx).all_configs,
                    (*(*yymsp.offset(0 as libc::c_int as isize)).minor.yy20).key.ptr,
                    buffer_clen(
                        &mut (*(*yymsp.offset(0 as libc::c_int as isize)).minor.yy20).key,
                    ),
                ) as *mut data_config;
                if (*yymsp.offset(0 as libc::c_int as isize)).minor.yy20 != dc_0 {
                    (*ctx).ok = 0 as libc::c_int;
                }
            }
            if (*ctx).ok != 0 {
                buffer_copy_buffer(
                    &mut (*(*yymsp.offset(0 as libc::c_int as isize)).minor.yy20).key,
                    &mut (*(*yymsp.offset(-(3 as libc::c_int) as isize)).minor.yy20).key,
                );
                let ref mut fresh52 = (*(*yymsp.offset(0 as libc::c_int as isize))
                    .minor
                    .yy20)
                    .comp_key;
                *fresh52 = ((*(*yymsp.offset(0 as libc::c_int as isize)).minor.yy20)
                    .key
                    .ptr)
                    .offset(
                        ((*(*yymsp.offset(-(3 as libc::c_int) as isize)).minor.yy20)
                            .comp_key)
                            .offset_from(
                                (*(*yymsp.offset(-(3 as libc::c_int) as isize)).minor.yy20)
                                    .key
                                    .ptr,
                            ) as libc::c_long as isize,
                    );
                (*(*yymsp.offset(0 as libc::c_int as isize)).minor.yy20)
                    .comp = (*(*yymsp.offset(-(3 as libc::c_int) as isize)).minor.yy20)
                    .comp;
                let mut pos: size_t = 0;
                pos = (buffer_clen(
                    &mut (*(*yymsp.offset(0 as libc::c_int as isize)).minor.yy20).key,
                ))
                    .wrapping_sub(
                        buffer_clen(
                            &mut (*(*yymsp.offset(-(3 as libc::c_int) as isize))
                                .minor
                                .yy20)
                                .string,
                        ),
                    )
                    .wrapping_sub(5 as libc::c_int as libc::c_uint) as size_t;
                match (*(*yymsp.offset(-(3 as libc::c_int) as isize)).minor.yy20).cond
                    as libc::c_uint
                {
                    3 => {
                        *((*(*yymsp.offset(0 as libc::c_int as isize)).minor.yy20)
                            .key
                            .ptr)
                            .offset(pos as isize) = '=' as i32 as libc::c_char;
                    }
                    1 => {
                        *((*(*yymsp.offset(0 as libc::c_int as isize)).minor.yy20)
                            .key
                            .ptr)
                            .offset(pos as isize) = '!' as i32 as libc::c_char;
                    }
                    4 => {
                        *((*(*yymsp.offset(0 as libc::c_int as isize)).minor.yy20)
                            .key
                            .ptr)
                            .offset(pos as isize) = '=' as i32 as libc::c_char;
                    }
                    2 => {
                        *((*(*yymsp.offset(0 as libc::c_int as isize)).minor.yy20)
                            .key
                            .ptr)
                            .offset(pos as isize) = '!' as i32 as libc::c_char;
                    }
                    5 => {
                        *((*(*yymsp.offset(0 as libc::c_int as isize)).minor.yy20)
                            .key
                            .ptr)
                            .offset(pos as isize) = '!' as i32 as libc::c_char;
                    }
                    6 => {
                        *((*(*yymsp.offset(0 as libc::c_int as isize)).minor.yy20)
                            .key
                            .ptr)
                            .offset(pos as isize) = '!' as i32 as libc::c_char;
                    }
                    _ => {
                        ck_assert_failed(
                            b"./configparser.y\0" as *const u8 as *const libc::c_char,
                            702 as libc::c_int as libc::c_uint,
                            b"unexpected enum value\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                }
                let mut dc_1: *mut data_config = 0 as *mut data_config;
                dc_1 = configparser_get_data_config(
                    (*ctx).all_configs,
                    (*(*yymsp.offset(0 as libc::c_int as isize)).minor.yy20).key.ptr,
                    buffer_clen(
                        &mut (*(*yymsp.offset(0 as libc::c_int as isize)).minor.yy20).key,
                    ) as size_t,
                );
                if dc_1.is_null() {
                    array_insert_unique(
                        (*ctx).all_configs,
                        (*yymsp.offset(0 as libc::c_int as isize)).minor.yy20
                            as *mut data_unset,
                    );
                    let ref mut fresh53 = (*(*yymsp.offset(0 as libc::c_int as isize))
                        .minor
                        .yy20)
                        .prev;
                    *fresh53 = (*yymsp.offset(-(3 as libc::c_int) as isize)).minor.yy20;
                    let ref mut fresh54 = (*(*yymsp.offset(-(3 as libc::c_int) as isize))
                        .minor
                        .yy20)
                        .next;
                    *fresh54 = (*yymsp.offset(0 as libc::c_int as isize)).minor.yy20;
                } else {
                    fprintf(
                        stderr,
                        b"unreachable else condition\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    (*ctx).ok = 0 as libc::c_int;
                    ((*(*(*yymsp.offset(0 as libc::c_int as isize)).minor.yy20).fn_0)
                        .free)
                        .expect(
                            "non-null function pointer",
                        )(
                        (*yymsp.offset(0 as libc::c_int as isize)).minor.yy20
                            as *mut data_unset,
                    );
                    let ref mut fresh55 = (*yymsp.offset(0 as libc::c_int as isize))
                        .minor
                        .yy20;
                    *fresh55 = dc_1;
                }
                yylhsminor.yy20 = (*yymsp.offset(0 as libc::c_int as isize)).minor.yy20;
            }
            let ref mut fresh56 = (*yymsp.offset(-(3 as libc::c_int) as isize))
                .minor
                .yy20;
            *fresh56 = 0 as *mut data_config;
            let ref mut fresh57 = (*yymsp.offset(0 as libc::c_int as isize)).minor.yy20;
            *fresh57 = 0 as *mut data_config;
            yy_destructor(
                yypParser,
                16 as libc::c_int as libc::c_uchar,
                &mut (*yymsp.offset(-(1 as libc::c_int) as isize)).minor,
            );
            let ref mut fresh58 = (*yymsp.offset(-(3 as libc::c_int) as isize))
                .minor
                .yy20;
            *fresh58 = yylhsminor.yy20;
            current_block_393 = 10953711258009896266;
        }
        22 => {
            yylhsminor.yy20 = (*yymsp.offset(0 as libc::c_int as isize)).minor.yy20;
            let ref mut fresh59 = (*yymsp.offset(0 as libc::c_int as isize)).minor.yy20;
            *fresh59 = 0 as *mut data_config;
            let ref mut fresh60 = (*yymsp.offset(0 as libc::c_int as isize)).minor.yy20;
            *fresh60 = yylhsminor.yy20;
            current_block_393 = 10953711258009896266;
        }
        23 => {
            current_block_393 = 7539340762994337730;
        }
        24 => {
            current_block_393 = 7539340762994337730;
        }
        25 => {
            yy_destructor(
                yypParser,
                17 as libc::c_int as libc::c_uchar,
                &mut (*yymsp.offset(-(6 as libc::c_int) as isize)).minor,
            );
            if (*ctx).ok != 0
                && (*(*yymsp.offset(0 as libc::c_int as isize)).minor.yy35).type_0
                    as libc::c_uint != TYPE_STRING as libc::c_int as libc::c_uint
            {
                fprintf(
                    stderr,
                    b"rvalue must be string\n\0" as *const u8 as *const libc::c_char,
                );
                (*ctx).ok = 0 as libc::c_int;
            }
            if (*ctx).ok != 0 {
                configparser_parse_condition(
                    ctx,
                    (*yymsp.offset(-(5 as libc::c_int) as isize)).minor.yy0,
                    (*yymsp.offset(-(3 as libc::c_int) as isize)).minor.yy0,
                    (*yymsp.offset(-(1 as libc::c_int) as isize)).minor.yy59,
                    &mut (*((*yymsp.offset(0 as libc::c_int as isize)).minor.yy35
                        as *mut data_string))
                        .value,
                );
            }
            buffer_free((*yymsp.offset(-(5 as libc::c_int) as isize)).minor.yy0);
            let ref mut fresh63 = (*yymsp.offset(-(5 as libc::c_int) as isize))
                .minor
                .yy0;
            *fresh63 = 0 as *mut buffer;
            buffer_free((*yymsp.offset(-(3 as libc::c_int) as isize)).minor.yy0);
            let ref mut fresh64 = (*yymsp.offset(-(3 as libc::c_int) as isize))
                .minor
                .yy0;
            *fresh64 = 0 as *mut buffer;
            if !((*yymsp.offset(0 as libc::c_int as isize)).minor.yy35).is_null() {
                ((*(*(*yymsp.offset(0 as libc::c_int as isize)).minor.yy35).fn_0).free)
                    .expect(
                        "non-null function pointer",
                    )((*yymsp.offset(0 as libc::c_int as isize)).minor.yy35);
            }
            let ref mut fresh65 = (*yymsp.offset(0 as libc::c_int as isize)).minor.yy35;
            *fresh65 = 0 as *mut data_unset;
            yy_destructor(
                yypParser,
                19 as libc::c_int as libc::c_uchar,
                &mut (*yymsp.offset(-(4 as libc::c_int) as isize)).minor,
            );
            yy_destructor(
                yypParser,
                20 as libc::c_int as libc::c_uchar,
                &mut (*yymsp.offset(-(2 as libc::c_int) as isize)).minor,
            );
            current_block_393 = 10953711258009896266;
        }
        26 => {
            if (*ctx).ok != 0 {
                configparser_parse_else_condition(ctx);
            }
            current_block_393 = 10953711258009896266;
        }
        27 => {
            yy_destructor(
                yypParser,
                21 as libc::c_int as libc::c_uchar,
                &mut (*yymsp.offset(0 as libc::c_int as isize)).minor,
            );
            (*yymsp.offset(0 as libc::c_int as isize)).minor.yy59 = CONFIG_COND_EQ;
            current_block_393 = 10953711258009896266;
        }
        28 => {
            yy_destructor(
                yypParser,
                22 as libc::c_int as libc::c_uchar,
                &mut (*yymsp.offset(0 as libc::c_int as isize)).minor,
            );
            (*yymsp.offset(0 as libc::c_int as isize)).minor.yy59 = CONFIG_COND_MATCH;
            current_block_393 = 10953711258009896266;
        }
        29 => {
            yy_destructor(
                yypParser,
                23 as libc::c_int as libc::c_uchar,
                &mut (*yymsp.offset(0 as libc::c_int as isize)).minor,
            );
            (*yymsp.offset(0 as libc::c_int as isize)).minor.yy59 = CONFIG_COND_NE;
            current_block_393 = 10953711258009896266;
        }
        30 => {
            yy_destructor(
                yypParser,
                24 as libc::c_int as libc::c_uchar,
                &mut (*yymsp.offset(0 as libc::c_int as isize)).minor,
            );
            (*yymsp.offset(0 as libc::c_int as isize)).minor.yy59 = CONFIG_COND_NOMATCH;
            current_block_393 = 10953711258009896266;
        }
        31 => {
            yy_destructor(
                yypParser,
                25 as libc::c_int as libc::c_uchar,
                &mut (*yymsp.offset(0 as libc::c_int as isize)).minor,
            );
            (*yymsp.offset(0 as libc::c_int as isize)).minor.yy59 = CONFIG_COND_PREFIX;
            current_block_393 = 10953711258009896266;
        }
        32 => {
            yy_destructor(
                yypParser,
                26 as libc::c_int as libc::c_uchar,
                &mut (*yymsp.offset(0 as libc::c_int as isize)).minor,
            );
            (*yymsp.offset(0 as libc::c_int as isize)).minor.yy59 = CONFIG_COND_SUFFIX;
            current_block_393 = 10953711258009896266;
        }
        33 => {
            yylhsminor.yy0 = 0 as *mut buffer;
            if (*ctx).ok != 0 {
                if (*(*yymsp.offset(0 as libc::c_int as isize)).minor.yy35).type_0
                    as libc::c_uint == TYPE_STRING as libc::c_int as libc::c_uint
                {
                    yylhsminor.yy0 = buffer_init();
                    buffer_copy_buffer(
                        yylhsminor.yy0,
                        &mut (*((*yymsp.offset(0 as libc::c_int as isize)).minor.yy35
                            as *mut data_string))
                            .value,
                    );
                } else if (*(*yymsp.offset(0 as libc::c_int as isize)).minor.yy35).type_0
                    as libc::c_uint == TYPE_INTEGER as libc::c_int as libc::c_uint
                {
                    yylhsminor.yy0 = buffer_init();
                    buffer_append_int(
                        yylhsminor.yy0,
                        (*((*yymsp.offset(0 as libc::c_int as isize)).minor.yy35
                            as *mut data_integer))
                            .value as intmax_t,
                    );
                } else {
                    fprintf(
                        stderr,
                        b"operand must be string\n\0" as *const u8 as *const libc::c_char,
                    );
                    (*ctx).ok = 0 as libc::c_int;
                }
            }
            if !((*yymsp.offset(0 as libc::c_int as isize)).minor.yy35).is_null() {
                ((*(*(*yymsp.offset(0 as libc::c_int as isize)).minor.yy35).fn_0).free)
                    .expect(
                        "non-null function pointer",
                    )((*yymsp.offset(0 as libc::c_int as isize)).minor.yy35);
            }
            let ref mut fresh66 = (*yymsp.offset(0 as libc::c_int as isize)).minor.yy35;
            *fresh66 = 0 as *mut data_unset;
            let ref mut fresh67 = (*yymsp.offset(0 as libc::c_int as isize)).minor.yy0;
            *fresh67 = yylhsminor.yy0;
            current_block_393 = 10953711258009896266;
        }
        34 => {
            yy_destructor(
                yypParser,
                27 as libc::c_int as libc::c_uchar,
                &mut (*yymsp.offset(-(1 as libc::c_int) as isize)).minor,
            );
            if (*ctx).ok != 0 {
                if 0 as libc::c_int
                    != config_parse_file(
                        (*ctx).srv,
                        ctx,
                        (*(*yymsp.offset(0 as libc::c_int as isize)).minor.yy0).ptr,
                    )
                {
                    (*ctx).ok = 0 as libc::c_int;
                }
            }
            buffer_free((*yymsp.offset(0 as libc::c_int as isize)).minor.yy0);
            let ref mut fresh68 = (*yymsp.offset(0 as libc::c_int as isize)).minor.yy0;
            *fresh68 = 0 as *mut buffer;
            current_block_393 = 10953711258009896266;
        }
        35 => {
            yy_destructor(
                yypParser,
                28 as libc::c_int as libc::c_uchar,
                &mut (*yymsp.offset(-(1 as libc::c_int) as isize)).minor,
            );
            if (*ctx).ok != 0 {
                if 0 as libc::c_int
                    != config_parse_cmd(
                        (*ctx).srv,
                        ctx,
                        (*(*yymsp.offset(0 as libc::c_int as isize)).minor.yy0).ptr,
                    )
                {
                    (*ctx).ok = 0 as libc::c_int;
                }
            }
            buffer_free((*yymsp.offset(0 as libc::c_int as isize)).minor.yy0);
            let ref mut fresh69 = (*yymsp.offset(0 as libc::c_int as isize)).minor.yy0;
            *fresh69 = 0 as *mut buffer;
            current_block_393 = 10953711258009896266;
        }
        43 => {
            current_block_393 = 1297461190301222800;
        }
        44 => {
            current_block_393 = 1297461190301222800;
        }
        _ => {
            current_block_393 = 10953711258009896266;
        }
    }
    match current_block_393 {
        7539340762994337730 => {
            let ref mut fresh61 = (*yymsp.offset(-(3 as libc::c_int) as isize))
                .minor
                .yy20;
            *fresh61 = 0 as *mut data_config;
            if (*ctx).ok != 0 {
                let ref mut fresh62 = (*yymsp.offset(-(3 as libc::c_int) as isize))
                    .minor
                    .yy20;
                *fresh62 = configparser_pop(ctx);
            }
            yy_destructor(
                yypParser,
                14 as libc::c_int as libc::c_uchar,
                &mut (*yymsp.offset(-(2 as libc::c_int) as isize)).minor,
            );
            yy_destructor(
                yypParser,
                15 as libc::c_int as libc::c_uchar,
                &mut (*yymsp.offset(0 as libc::c_int as isize)).minor,
            );
        }
        2472048668343472511 => {
            yylhsminor.yy35 = (*yymsp.offset(0 as libc::c_int as isize)).minor.yy35;
            let ref mut fresh20 = (*yymsp.offset(0 as libc::c_int as isize)).minor.yy35;
            *fresh20 = 0 as *mut data_unset;
            let ref mut fresh21 = (*yymsp.offset(0 as libc::c_int as isize)).minor.yy35;
            *fresh21 = yylhsminor.yy35;
        }
        1297461190301222800 => {
            yy_destructor(
                yypParser,
                1 as libc::c_int as libc::c_uchar,
                &mut (*yymsp.offset(0 as libc::c_int as isize)).minor,
            );
        }
        _ => {}
    }
    yygoto = yyRuleInfoLhs[yyruleno as usize] as libc::c_int;
    yysize = yyRuleInfoNRhs[yyruleno as usize] as libc::c_int;
    yyact = yy_find_reduce_action(
        (*yymsp.offset(yysize as isize)).stateno,
        yygoto as libc::c_uchar,
    );
    yymsp = yymsp.offset((yysize + 1 as libc::c_int) as isize);
    (*yypParser).yytos = yymsp;
    (*yymsp).stateno = yyact;
    (*yymsp).major = yygoto as libc::c_uchar;
    return yyact;
}
unsafe extern "C" fn yy_parse_failed(mut yypParser: *mut yyParser) {
    let mut ctx: *mut config_t = (*yypParser).ctx;
    while (*yypParser).yytos > ((*yypParser).yystack).as_mut_ptr() {
        yy_pop_parser_stack(yypParser);
    }
    (*ctx).ok = 0 as libc::c_int;
    (*yypParser).ctx = ctx;
}
unsafe extern "C" fn yy_syntax_error(
    mut yypParser: *mut yyParser,
    mut yymajor: libc::c_int,
    mut yyminor: *mut buffer,
) {
    let mut ctx: *mut config_t = (*yypParser).ctx;
    (*yypParser).ctx = ctx;
}
unsafe extern "C" fn yy_accept(mut yypParser: *mut yyParser) {
    let mut ctx: *mut config_t = (*yypParser).ctx;
    (*yypParser).yyerrcnt = -(1 as libc::c_int);
    (*yypParser).ctx = ctx;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn configparser(
    mut yyp: *mut libc::c_void,
    mut yymajor: libc::c_int,
    mut yyminor: *mut buffer,
    mut ctx: *mut config_t,
) {
    let mut yyminorunion: YYMINORTYPE = YYMINORTYPE { yyinit: 0 };
    let mut yyact: libc::c_uchar = 0;
    let mut yyendofinput: libc::c_int = 0;
    let mut yypParser: *mut yyParser = yyp as *mut yyParser;
    (*yypParser).ctx = ctx;
    yyendofinput = (yymajor == 0 as libc::c_int) as libc::c_int;
    yyact = (*(*yypParser).yytos).stateno;
    loop {
        yyact = yy_find_shift_action(yymajor as libc::c_uchar, yyact);
        if yyact as libc::c_int >= 121 as libc::c_int {
            let mut yyruleno: libc::c_uint = (yyact as libc::c_int - 121 as libc::c_int)
                as libc::c_uint;
            if yyRuleInfoNRhs[yyruleno as usize] as libc::c_int == 0 as libc::c_int {
                if (*yypParser).yytos >= (*yypParser).yystackEnd {
                    yyStackOverflow(yypParser);
                    break;
                }
            }
            yyact = yy_reduce(yypParser, yyruleno, yymajor, yyminor);
        } else if yyact as libc::c_int <= 117 as libc::c_int {
            yy_shift(yypParser, yyact, yymajor as libc::c_uchar, yyminor);
            (*yypParser).yyerrcnt -= 1;
            (*yypParser).yyerrcnt;
            break;
        } else if yyact as libc::c_int == 119 as libc::c_int {
            (*yypParser).yytos = ((*yypParser).yytos).offset(-1);
            (*yypParser).yytos;
            yy_accept(yypParser);
            return;
        } else {
            yyminorunion.yy0 = yyminor;
            if (*yypParser).yyerrcnt <= 0 as libc::c_int {
                yy_syntax_error(yypParser, yymajor, yyminor);
            }
            (*yypParser).yyerrcnt = 3 as libc::c_int;
            yy_destructor(yypParser, yymajor as libc::c_uchar, &mut yyminorunion);
            if yyendofinput != 0 {
                yy_parse_failed(yypParser);
                (*yypParser).yyerrcnt = -(1 as libc::c_int);
            }
            break;
        }
    };
}
unsafe extern "C" fn run_static_initializers() {
    comps = [
        {
            let mut init = C2RustUnnamed_7 {
                comp: COMP_HTTP_URL,
                len: (::core::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                comp_tag: b"url\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_7 {
                comp: COMP_HTTP_HOST,
                len: (::core::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                comp_tag: b"host\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_7 {
                comp: COMP_HTTP_REQUEST_HEADER,
                len: (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                comp_tag: b"referer\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_7 {
                comp: COMP_HTTP_USER_AGENT,
                len: (::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                comp_tag: b"useragent\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_7 {
                comp: COMP_HTTP_REQUEST_HEADER,
                len: (::core::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                comp_tag: b"user-agent\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_7 {
                comp: COMP_HTTP_LANGUAGE,
                len: (::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                comp_tag: b"language\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_7 {
                comp: COMP_HTTP_REQUEST_HEADER,
                len: (::core::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                comp_tag: b"cookie\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_7 {
                comp: COMP_HTTP_REMOTE_IP,
                len: (::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                comp_tag: b"remoteip\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_7 {
                comp: COMP_HTTP_REMOTE_IP,
                len: (::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                comp_tag: b"remote-ip\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_7 {
                comp: COMP_HTTP_QUERY_STRING,
                len: (::core::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                comp_tag: b"querystring\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_7 {
                comp: COMP_HTTP_QUERY_STRING,
                len: (::core::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                comp_tag: b"query-string\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_7 {
                comp: COMP_HTTP_REQUEST_METHOD,
                len: (::core::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                comp_tag: b"request-method\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_7 {
                comp: COMP_HTTP_SCHEME,
                len: (::core::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                comp_tag: b"scheme\0" as *const u8 as *const libc::c_char,
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
