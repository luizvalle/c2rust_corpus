use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type stat_cache_entry;
    pub type pcre2_real_match_data_8;
    pub type fdevents;
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
    fn buffer_copy_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn ck_calloc(nmemb: size_t, elt_sz: size_t) -> *mut libc::c_void;
    fn chunkqueue_get_memory(cq: *mut chunkqueue, len: *mut size_t) -> *mut libc::c_char;
    fn chunkqueue_use_memory(cq: *mut chunkqueue, ckpt: *mut chunk, len: size_t);
    fn chunkqueue_free(cq: *mut chunkqueue);
    fn chunkqueue_reset(cq: *mut chunkqueue);
    fn setsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *const libc::c_void,
        __optlen: socklen_t,
    ) -> libc::c_int;
    fn shutdown(__fd: libc::c_int, __how: libc::c_int) -> libc::c_int;
    static mut log_con_jqueue: *mut connection;
    static mut log_epoch_secs: unix_time64_t;
    static mut log_monotonic_secs: unix_time64_t;
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
    fn fdevent_fdnode_event_del(ev: *mut fdevents, fdn: *mut fdnode);
    fn fdevent_fdnode_event_set(
        ev: *mut fdevents,
        fdn: *mut fdnode,
        events: libc::c_int,
    );
    fn fdevent_register(
        ev: *mut fdevents,
        fd: libc::c_int,
        handler: fdevent_handler,
        ctx: *mut libc::c_void,
    ) -> *mut fdnode;
    fn fdevent_unregister(ev: *mut fdevents, fdn: *mut fdnode);
    fn fdevent_socket_close(fd: libc::c_int) -> libc::c_int;
    fn fdevent_socket_read_discard(
        fd: libc::c_int,
        buf: *mut libc::c_char,
        sz: size_t,
        family: libc::c_int,
        so_type: libc::c_int,
    ) -> ssize_t;
    fn fdevent_ioctl_fionread(
        fd: libc::c_int,
        fdfmt: libc::c_int,
        toread: *mut libc::c_int,
    ) -> libc::c_int;
    fn fdevent_is_tcp_half_closed(fd: libc::c_int) -> libc::c_int;
    fn h1_send_headers(r: *mut request_st);
    fn h1_recv_headers(r: *mut request_st, con: *mut connection) -> libc::c_int;
    fn h1_reqbody_read(r: *mut request_st) -> handler_t;
    fn h1_check_timeout(con: *mut connection, cur_ts: unix_time64_t) -> libc::c_int;
    fn request_init_data(r: *mut request_st, con: *mut connection, srv: *mut server);
    fn request_reset(r: *mut request_st);
    fn request_reset_ex(r: *mut request_st);
    fn request_free_data(r: *mut request_st);
    static mut http_dispatch: [http_dispatch; 4];
    fn http_response_handler(r: *mut request_st) -> handler_t;
    fn config_cond_cache_reset(r: *mut request_st);
    fn plugins_call_handle_request_done(r: *mut request_st) -> handler_t;
    fn plugins_call_handle_connection_accept(con: *mut connection) -> handler_t;
    fn plugins_call_handle_connection_shut_wr(con: *mut connection) -> handler_t;
    fn plugins_call_handle_connection_close(con: *mut connection) -> handler_t;
    fn sock_addr_cache_inet_ntop_copy_buffer(
        b: *mut buffer,
        saddr: *const sock_addr,
    ) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn free(_: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type off_t = __off64_t;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
pub type clockid_t = __clockid_t;
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
pub type socklen_t = __socklen_t;
pub type __socket_type = libc::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
pub type C2RustUnnamed_7 = libc::c_uint;
pub const SHUT_RDWR: C2RustUnnamed_7 = 2;
pub const SHUT_WR: C2RustUnnamed_7 = 1;
pub const SHUT_RD: C2RustUnnamed_7 = 0;
pub type C2RustUnnamed_8 = libc::c_uint;
pub const IPPROTO_MAX: C2RustUnnamed_8 = 263;
pub const IPPROTO_MPTCP: C2RustUnnamed_8 = 262;
pub const IPPROTO_RAW: C2RustUnnamed_8 = 255;
pub const IPPROTO_ETHERNET: C2RustUnnamed_8 = 143;
pub const IPPROTO_MPLS: C2RustUnnamed_8 = 137;
pub const IPPROTO_UDPLITE: C2RustUnnamed_8 = 136;
pub const IPPROTO_SCTP: C2RustUnnamed_8 = 132;
pub const IPPROTO_COMP: C2RustUnnamed_8 = 108;
pub const IPPROTO_PIM: C2RustUnnamed_8 = 103;
pub const IPPROTO_ENCAP: C2RustUnnamed_8 = 98;
pub const IPPROTO_BEETPH: C2RustUnnamed_8 = 94;
pub const IPPROTO_MTP: C2RustUnnamed_8 = 92;
pub const IPPROTO_AH: C2RustUnnamed_8 = 51;
pub const IPPROTO_ESP: C2RustUnnamed_8 = 50;
pub const IPPROTO_GRE: C2RustUnnamed_8 = 47;
pub const IPPROTO_RSVP: C2RustUnnamed_8 = 46;
pub const IPPROTO_IPV6: C2RustUnnamed_8 = 41;
pub const IPPROTO_DCCP: C2RustUnnamed_8 = 33;
pub const IPPROTO_TP: C2RustUnnamed_8 = 29;
pub const IPPROTO_IDP: C2RustUnnamed_8 = 22;
pub const IPPROTO_UDP: C2RustUnnamed_8 = 17;
pub const IPPROTO_PUP: C2RustUnnamed_8 = 12;
pub const IPPROTO_EGP: C2RustUnnamed_8 = 8;
pub const IPPROTO_TCP: C2RustUnnamed_8 = 6;
pub const IPPROTO_IPIP: C2RustUnnamed_8 = 4;
pub const IPPROTO_IGMP: C2RustUnnamed_8 = 2;
pub const IPPROTO_ICMP: C2RustUnnamed_8 = 1;
pub const IPPROTO_IP: C2RustUnnamed_8 = 0;
pub const COMP_HTTP_REQUEST_METHOD: C2RustUnnamed_9 = 11;
pub const COMP_HTTP_REMOTE_IP: C2RustUnnamed_9 = 8;
pub const COMP_SERVER_SOCKET: C2RustUnnamed_9 = 1;
pub type C2RustUnnamed_9 = libc::c_uint;
pub const COMP_LAST_ELEMENT: C2RustUnnamed_9 = 13;
pub const COMP_HTTP_REQUEST_HEADER: C2RustUnnamed_9 = 12;
pub const COMP_HTTP_SCHEME: C2RustUnnamed_9 = 10;
pub const COMP_HTTP_QUERY_STRING: C2RustUnnamed_9 = 9;
pub const COMP_HTTP_COOKIE: C2RustUnnamed_9 = 7;
pub const COMP_HTTP_LANGUAGE: C2RustUnnamed_9 = 6;
pub const COMP_HTTP_USER_AGENT: C2RustUnnamed_9 = 5;
pub const COMP_HTTP_REFERER: C2RustUnnamed_9 = 4;
pub const COMP_HTTP_HOST: C2RustUnnamed_9 = 3;
pub const COMP_HTTP_URL: C2RustUnnamed_9 = 2;
pub const COMP_UNSET: C2RustUnnamed_9 = 0;
#[inline]
unsafe extern "C" fn chunkqueue_length(mut cq: *const chunkqueue) -> off_t {
    return (*cq).bytes_in - (*cq).bytes_out;
}
#[inline]
unsafe extern "C" fn chunkqueue_is_empty(mut cq: *const chunkqueue) -> libc::c_int {
    return (0 as *mut libc::c_void as *mut chunk == (*cq).first) as libc::c_int;
}
#[cold]
#[inline]
unsafe extern "C" fn request_set_state_error(
    r: *mut request_st,
    state: request_state_t,
) {
    (*r).state = state;
}
#[inline]
unsafe extern "C" fn sock_addr_get_family(mut saddr: *const sock_addr) -> libc::c_int {
    return (*saddr).plain.sa_family as libc::c_int;
}
#[inline]
unsafe extern "C" fn connection_jq_append(con: *mut connection) {
    if ((*con).jqnext).is_null() {
        (*con).jqnext = log_con_jqueue;
        log_con_jqueue = con;
    }
}
unsafe extern "C" fn connections_get_new_connection(
    mut srv: *mut server,
) -> *mut connection {
    let mut con: *mut connection = 0 as *mut connection;
    (*srv).lim_conns = ((*srv).lim_conns).wrapping_sub(1);
    (*srv).lim_conns;
    if !((*srv).conns_pool).is_null() {
        con = (*srv).conns_pool;
        (*srv).conns_pool = (*con).next;
    } else {
        con = connection_init(srv);
        connection_reset(con);
    }
    (*con).next = (*srv).conns;
    if !((*con).next).is_null() {
        (*(*con).next).prev = con;
    }
    (*srv).conns = con;
    return (*srv).conns;
}
unsafe extern "C" fn connection_del(mut srv: *mut server, mut con: *mut connection) {
    if !((*con).next).is_null() {
        (*(*con).next).prev = (*con).prev;
    }
    if !((*con).prev).is_null() {
        (*(*con).prev).next = (*con).next;
    } else {
        (*srv).conns = (*con).next;
    }
    (*con).prev = 0 as *mut connection;
    (*con).next = (*srv).conns_pool;
    (*srv).conns_pool = con;
    (*srv).lim_conns = ((*srv).lim_conns).wrapping_add(1);
    (*srv).lim_conns;
}
unsafe extern "C" fn connection_close(mut con: *mut connection) {
    if (*con).fd < 0 as libc::c_int {
        (*con).fd = -(*con).fd;
    }
    plugins_call_handle_connection_close(con);
    let srv: *mut server = (*con).srv;
    let r: *mut request_st = &mut (*con).request;
    request_reset_ex(r);
    (*r).state = CON_STATE_CONNECT;
    chunkqueue_reset((*con).read_queue);
    (*con).request_count = 0 as libc::c_int as uint32_t;
    (*con).is_ssl_sock = 0 as libc::c_int as libc::c_char;
    (*con).traffic_limit_reached = 0 as libc::c_int as libc::c_char;
    (*con).revents_err = 0 as libc::c_int as uint16_t;
    fdevent_fdnode_event_del((*srv).ev, (*con).fdn);
    fdevent_unregister((*srv).ev, (*con).fdn);
    (*con).fdn = 0 as *mut fdnode;
    if 0 as libc::c_int != fdevent_socket_close((*con).fd) {
        log_perror(
            (*r).conf.errh,
            b"connections.c\0" as *const u8 as *const libc::c_char,
            94 as libc::c_int as libc::c_uint,
            b"(warning) close: %d\0" as *const u8 as *const libc::c_char,
            (*con).fd,
        );
    }
    (*con).fd = -(1 as libc::c_int);
    (*srv).cur_fds -= 1;
    (*srv).cur_fds;
    connection_del(srv, con);
}
unsafe extern "C" fn connection_read_for_eos_plain(con: *mut connection) {
    let mut len: ssize_t = 0;
    let type_0: libc::c_int = sock_addr_get_family(&mut (*con).dst_addr);
    let mut buf: [libc::c_char; 16384] = [0; 16384];
    loop {
        len = fdevent_socket_read_discard(
            (*con).fd,
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 16384]>() as libc::c_ulong,
            type_0,
            SOCK_STREAM as libc::c_int,
        );
        if !(len > 0 as libc::c_int as libc::c_long
            || len < 0 as libc::c_int as libc::c_long
                && *__errno_location() == 4 as libc::c_int)
        {
            break;
        }
    }
    if len < 0 as libc::c_int as libc::c_long && *__errno_location() == 11 as libc::c_int
    {
        return;
    }
    (*con)
        .close_timeout_ts = log_monotonic_secs
        - (5 as libc::c_int + 1 as libc::c_int) as libc::c_long;
}
unsafe extern "C" fn connection_read_for_eos_ssl(con: *mut connection) {
    if ((*con).network_read)
        .expect(
            "non-null function pointer",
        )(con, (*con).read_queue, (256 as libc::c_int * 1024 as libc::c_int) as off_t)
        < 0 as libc::c_int
    {
        (*con)
            .close_timeout_ts = log_monotonic_secs
            - (5 as libc::c_int + 1 as libc::c_int) as libc::c_long;
    }
    chunkqueue_reset((*con).read_queue);
}
unsafe extern "C" fn connection_read_for_eos(con: *mut connection) {
    if (*con).is_ssl_sock == 0 {
        connection_read_for_eos_plain(con);
    } else {
        connection_read_for_eos_ssl(con);
    };
}
unsafe extern "C" fn connection_handle_close_state(mut con: *mut connection) {
    connection_read_for_eos(con);
    if log_monotonic_secs - (*con).close_timeout_ts > 5 as libc::c_int as libc::c_long {
        connection_close(con);
    }
}
unsafe extern "C" fn connection_handle_shutdown(mut con: *mut connection) {
    plugins_call_handle_connection_shut_wr(con);
    connection_reset(con);
    if (*con).fd >= 0 as libc::c_int
        && ((*con).is_ssl_sock as libc::c_int != 0
            || 0 as libc::c_int == shutdown((*con).fd, SHUT_WR as libc::c_int))
    {
        (*con).close_timeout_ts = log_monotonic_secs;
        let r: *mut request_st = &mut (*con).request;
        (*r).state = CON_STATE_CLOSE;
    } else {
        connection_close(con);
    };
}
unsafe extern "C" fn connection_handle_request_start_state(
    r: *mut request_st,
    con: *mut connection,
) {
    (*con).request_count = ((*con).request_count).wrapping_add(1);
    (*con).request_count;
    (*con).read_idle_ts = log_monotonic_secs;
    (*r).start_hp.tv_sec = log_epoch_secs;
    (*r).loops_per_request = 0 as libc::c_int as libc::c_char;
    if ((*r).conf).high_precision_timestamps() != 0 {
        clock_gettime(0 as libc::c_int, &mut (*r).start_hp);
    }
}
unsafe extern "C" fn connection_handle_response_end_state(
    r: *mut request_st,
    con: *mut connection,
) {
    if (*r).http_version as libc::c_int > HTTP_VERSION_1_1 as libc::c_int {
        (*r).keep_alive = 0 as libc::c_int as int8_t;
        (*r).http_status = 100 as libc::c_int;
        plugins_call_handle_request_done(r);
        connection_handle_shutdown(con);
        return;
    }
    if (*r).http_status != 0 {
        plugins_call_handle_request_done(r);
    }
    if (*r).reqbody_length != (*r).reqbody_queue.bytes_in
        || (*r).state as libc::c_uint == CON_STATE_ERROR as libc::c_int as libc::c_uint
    {
        (*r).keep_alive = 0 as libc::c_int as int8_t;
        if &mut (*r).write_queue as *mut chunkqueue != (*con).write_queue {
            chunkqueue_free((*con).write_queue);
            (*con).write_queue = &mut (*r).write_queue;
        }
    }
    if (*r).keep_alive as libc::c_int > 0 as libc::c_int {
        request_reset(r);
        (*con).is_readable = 1 as libc::c_int as libc::c_schar;
        (*r).x.h1.bytes_read_ckpt = (*r).read_queue.bytes_in;
        (*r).x.h1.bytes_written_ckpt = (*r).write_queue.bytes_out;
        (*r).state = CON_STATE_REQUEST_START;
    } else {
        connection_handle_shutdown(con);
    };
}
unsafe extern "C" fn connection_write_throttle(
    con: *const connection,
    mut max_bytes: off_t,
) -> off_t {
    let rconf: *const request_config = &(*con).request.conf;
    if (0 as libc::c_int as libc::c_uint
        == (*rconf).global_bytes_per_second | (*rconf).bytes_per_second) as libc::c_int
        as libc::c_long != 0
    {
        return max_bytes;
    }
    if (*rconf).global_bytes_per_second != 0 {
        let mut limit: off_t = (*rconf).global_bytes_per_second as off_t
            - *(*rconf).global_bytes_per_second_cnt_ptr;
        if max_bytes > limit {
            max_bytes = limit;
        }
    }
    if (*rconf).bytes_per_second != 0 {
        let mut limit_0: off_t = (*rconf).bytes_per_second as off_t
            - (*con).bytes_written_cur_second;
        if max_bytes > limit_0 {
            max_bytes = limit_0;
        }
    }
    return if max_bytes > 0 as libc::c_int as libc::c_long {
        max_bytes
    } else {
        0 as libc::c_int as libc::c_long
    };
}
unsafe extern "C" fn connection_write_chunkqueue(
    con: *mut connection,
    cq: *mut chunkqueue,
    mut max_bytes: off_t,
) -> libc::c_int {
    (*con).write_request_ts = log_monotonic_secs;
    max_bytes = connection_write_throttle(con, max_bytes);
    if (0 as libc::c_int as libc::c_long == max_bytes) as libc::c_int as libc::c_long
        != 0
    {
        (*con).traffic_limit_reached = 1 as libc::c_int as libc::c_char;
        return (*con).traffic_limit_reached as libc::c_int;
    }
    let mut written: off_t = (*cq).bytes_out;
    let mut ret: libc::c_int = 0;
    let mut corked: libc::c_int = 0 as libc::c_int;
    if !((*(*cq).first).next).is_null()
        && (*(*cq).first).type_0 as libc::c_uint
            == MEM_CHUNK as libc::c_int as libc::c_uint
    {
        let mut c: *const chunk = (*cq).first;
        loop {
            c = (*c).next;
            if !(!c.is_null()
                && (*c).type_0 as libc::c_uint
                    == MEM_CHUNK as libc::c_int as libc::c_uint)
            {
                break;
            }
        }
        if !c.is_null()
            || (*con).is_ssl_sock as libc::c_int != 0
                && chunkqueue_length(cq) > 16384 as libc::c_int as libc::c_long
        {
            let sa_family: libc::c_int = sock_addr_get_family(
                &(*(*con).srv_socket).addr,
            );
            if sa_family == 2 as libc::c_int || sa_family == 10 as libc::c_int {
                corked = 1 as libc::c_int;
                setsockopt(
                    (*con).fd,
                    IPPROTO_TCP as libc::c_int,
                    3 as libc::c_int,
                    &mut corked as *mut libc::c_int as *const libc::c_void,
                    ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
                );
            }
        }
    }
    ret = ((*con).network_write).expect("non-null function pointer")(con, cq, max_bytes);
    if corked != 0 {
        corked = 0 as libc::c_int;
        setsockopt(
            (*con).fd,
            IPPROTO_TCP as libc::c_int,
            3 as libc::c_int,
            &mut corked as *mut libc::c_int as *const libc::c_void,
            ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
        );
    }
    written = (*cq).bytes_out - written;
    (*con).bytes_written_cur_second += written;
    let r: *mut request_st = &mut (*con).request;
    if !((*r).conf.global_bytes_per_second_cnt_ptr).is_null() {
        *(*r).conf.global_bytes_per_second_cnt_ptr += written;
    }
    return if ret >= 0 as libc::c_int {
        (chunkqueue_is_empty(cq) == 0 && (*cq).bytes_out != 0) as libc::c_int
    } else {
        ret
    };
}
#[inline(never)]
unsafe extern "C" fn connection_handle_write(
    r: *mut request_st,
    con: *mut connection,
) -> libc::c_int {
    if (*con).is_writable as libc::c_int <= 0 as libc::c_int {
        return CON_STATE_WRITE as libc::c_int;
    }
    let mut rc: libc::c_int = connection_write_chunkqueue(
        con,
        (*con).write_queue,
        (256 as libc::c_int * 1024 as libc::c_int) as off_t,
    );
    match rc {
        -1 | -2 => {
            request_set_state_error(r, CON_STATE_ERROR);
            return CON_STATE_ERROR as libc::c_int;
        }
        1 => {
            (*con).is_writable = 0 as libc::c_int as libc::c_schar;
        }
        0 | _ => {}
    }
    return CON_STATE_WRITE as libc::c_int;
}
unsafe extern "C" fn connection_handle_write_state(
    r: *mut request_st,
    con: *mut connection,
) -> libc::c_int {
    let mut loop_once: libc::c_int = 0 as libc::c_int;
    loop {
        if chunkqueue_is_empty(&mut (*r).write_queue) == 0 {
            let mut rc: libc::c_int = connection_handle_write(r, con);
            if rc != CON_STATE_WRITE as libc::c_int {
                return rc;
            }
        } else if (*r).resp_body_finished != 0 {
            (*r).state = CON_STATE_RESPONSE_END;
            return CON_STATE_RESPONSE_END as libc::c_int;
        }
        if !((*r).handler_module).is_null() && (*r).resp_body_finished == 0 {
            let p: *const plugin = (*r).handler_module;
            if ((*p).handle_subrequest).expect("non-null function pointer")(r, (*p).data)
                as libc::c_uint > HANDLER_WAIT_FOR_EVENT as libc::c_int as libc::c_uint
            {
                request_set_state_error(r, CON_STATE_ERROR);
                return CON_STATE_ERROR as libc::c_int;
            }
        }
        if !(if chunkqueue_is_empty(&mut (*r).write_queue) == 0 {
            ((*con).is_writable as libc::c_int > 0 as libc::c_int
                && 0 as libc::c_int == (*con).traffic_limit_reached as libc::c_int
                && {
                    loop_once += 1;
                    1 as libc::c_int == loop_once
                }) as libc::c_int
        } else {
            (*r).resp_body_finished as libc::c_int
        } != 0)
        {
            break;
        }
    }
    if 2 as libc::c_int == loop_once {
        connection_jq_append(con);
    }
    return CON_STATE_WRITE as libc::c_int;
}
#[cold]
unsafe extern "C" fn connection_init(mut srv: *mut server) -> *mut connection {
    let con: *mut connection = ck_calloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<connection>() as libc::c_ulong,
    ) as *mut connection;
    (*con).srv = srv;
    (*con).plugin_slots = (*srv).plugin_slots;
    (*con).config_data_base = (*srv).config_data_base;
    let r: *mut request_st = &mut (*con).request;
    request_init_data(r, con, srv);
    (*con).write_queue = &mut (*r).write_queue;
    (*con).read_queue = &mut (*r).read_queue;
    (*con)
        .plugin_ctx = ck_calloc(
        ((*srv).plugins.used).wrapping_add(1 as libc::c_int as libc::c_uint) as size_t,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
    ) as *mut *mut libc::c_void;
    return con;
}
unsafe extern "C" fn connection_free(con: *mut connection) {
    let r: *mut request_st = &mut (*con).request;
    connection_reset(con);
    if (*con).write_queue != &mut (*r).write_queue as *mut chunkqueue {
        chunkqueue_free((*con).write_queue);
    }
    if (*con).read_queue != &mut (*r).read_queue as *mut chunkqueue {
        chunkqueue_free((*con).read_queue);
    }
    request_free_data(r);
    free((*con).plugin_ctx as *mut libc::c_void);
    free((*con).dst_addr_buf.ptr as *mut libc::c_void);
    free(con as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn connections_pool_clear(srv: *mut server) {
    let mut con: *mut connection = 0 as *mut connection;
    loop {
        con = (*srv).conns_pool;
        if con.is_null() {
            break;
        }
        (*srv).conns_pool = (*con).next;
        connection_free(con);
    };
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn connections_free(mut srv: *mut server) {
    connections_pool_clear(srv);
    let mut con: *mut connection = 0 as *mut connection;
    loop {
        con = (*srv).conns;
        if con.is_null() {
            break;
        }
        (*srv).conns = (*con).next;
        connection_free(con);
    };
}
#[inline(never)]
unsafe extern "C" fn connection_reset(mut con: *mut connection) {
    let r: *mut request_st = &mut (*con).request;
    request_reset(r);
    (*con).is_readable = 1 as libc::c_int as libc::c_schar;
    (*con).bytes_written_cur_second = 0 as libc::c_int as off_t;
    (*con).fn_0 = 0 as *const http_dispatch;
}
#[cold]
unsafe extern "C" fn connection_transition_h2(
    h2r: *mut request_st,
    con: *mut connection,
) {
    buffer_copy_string_len(
        &mut (*h2r).target,
        b"*\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    buffer_copy_string_len(
        &mut (*h2r).target_orig,
        b"*\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    buffer_copy_string_len(
        &mut (*h2r).uri.path,
        b"*\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    (*h2r).http_method = HTTP_METHOD_PRI;
    (*h2r).conditional_is_valid
        |= ((1 as libc::c_int) << COMP_HTTP_REQUEST_METHOD as libc::c_int)
            as libc::c_uint;
    (*h2r).reqbody_length = -(1 as libc::c_int) as off_t;
    (*h2r)
        .conf
        .stream_request_body = ((*h2r).conf.stream_request_body as libc::c_int
        | (1 as libc::c_int) << 15 as libc::c_int) as libc::c_ushort;
    if (*h2r).state as libc::c_uint == CON_STATE_ERROR as libc::c_int as libc::c_uint {
        return;
    }
    (*h2r).state = CON_STATE_WRITE;
    if ((*con).hx).is_null() {
        (http_dispatch[HTTP_VERSION_2 as libc::c_int as usize].upgrade_h2)
            .expect("non-null function pointer")(h2r, con);
    }
}
unsafe extern "C" fn connection_handle_fdevent(
    context: *mut libc::c_void,
    revents: libc::c_int,
) -> handler_t {
    let mut con: *mut connection = context as *mut connection;
    let is_ssl_sock: libc::c_int = (*con).is_ssl_sock as libc::c_int;
    connection_jq_append(con);
    if revents & !(0x1 as libc::c_int | 0x4 as libc::c_int) != 0 {
        (*con)
            .revents_err = ((*con).revents_err as libc::c_int
            | revents & !(0x1 as libc::c_int | 0x4 as libc::c_int)) as uint16_t;
    }
    if revents & (0x1 as libc::c_int | 0x4 as libc::c_int) != 0 {
        if is_ssl_sock != 0 {
            (*con).is_writable = 1 as libc::c_int as libc::c_schar;
            (*con).is_readable = (*con).is_writable;
        } else {
            if revents & 0x1 as libc::c_int != 0 {
                (*con).is_readable = 1 as libc::c_int as libc::c_schar;
            }
            if revents & 0x4 as libc::c_int != 0 {
                (*con).is_writable = 1 as libc::c_int as libc::c_schar;
            }
        }
    }
    return HANDLER_FINISHED;
}
#[cold]
unsafe extern "C" fn connection_read_cq_err(mut con: *mut connection) -> libc::c_int {
    let r: *mut request_st = &mut (*con).request;
    match *__errno_location() {
        11 => return 0 as libc::c_int,
        4 => {
            (*con).is_readable = 1 as libc::c_int as libc::c_schar;
            return 0 as libc::c_int;
        }
        104 => {}
        _ => {
            log_perror(
                (*r).conf.errh,
                b"connections.c\0" as *const u8 as *const libc::c_char,
                528 as libc::c_int as libc::c_uint,
                b"connection closed - read failed\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    request_set_state_error(r, CON_STATE_ERROR);
    return -(1 as libc::c_int);
}
unsafe extern "C" fn connection_read_cq(
    mut con: *mut connection,
    mut cq: *mut chunkqueue,
    mut max_bytes: off_t,
) -> libc::c_int {
    let mut len: ssize_t = 0;
    let mut mem_len: size_t = 0 as libc::c_int as size_t;
    loop {
        let mut ckpt: *mut chunk = (*cq).last;
        let mem: *mut libc::c_char = chunkqueue_get_memory(cq, &mut mem_len);
        if mem_len > max_bytes as size_t {
            mem_len = max_bytes as size_t;
        }
        len = read((*con).fd, mem as *mut libc::c_void, mem_len);
        chunkqueue_use_memory(
            cq,
            ckpt,
            (if len > 0 as libc::c_int as libc::c_long {
                len
            } else {
                0 as libc::c_int as libc::c_long
            }) as size_t,
        );
        if len != mem_len as ssize_t {
            (*con).is_readable = 0 as libc::c_int as libc::c_schar;
            return if len > 0 as libc::c_int as libc::c_long {
                0 as libc::c_int
            } else if 0 as libc::c_int as libc::c_long == len {
                -(2 as libc::c_int)
            } else {
                connection_read_cq_err(con)
            };
        }
        max_bytes -= len;
        let mut frd: libc::c_int = 0;
        mem_len = if 0 as libc::c_int
            == fdevent_ioctl_fionread((*con).fd, 0o140000 as libc::c_int, &mut frd)
        {
            if (frd as libc::c_long) < max_bytes {
                frd as size_t
            } else {
                max_bytes as size_t
            }
        } else {
            0 as libc::c_int as libc::c_ulong
        };
        if !(max_bytes != 0) {
            break;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn connection_write_cq(
    mut con: *mut connection,
    mut cq: *mut chunkqueue,
    mut max_bytes: off_t,
) -> libc::c_int {
    let r: *mut request_st = &mut (*con).request;
    return ((*(*con).srv).network_backend_write)
        .expect("non-null function pointer")((*con).fd, cq, max_bytes, (*r).conf.errh);
}
#[no_mangle]
pub unsafe extern "C" fn connection_accepted(
    mut srv: *mut server,
    mut srv_socket: *const server_socket,
    mut cnt_addr: *mut sock_addr,
    mut cnt: libc::c_int,
) -> *mut connection {
    let mut con: *mut connection = 0 as *mut connection;
    (*srv).cur_fds += 1;
    (*srv).cur_fds;
    con = connections_get_new_connection(srv);
    (*con).fd = cnt;
    (*con)
        .fdn = fdevent_register(
        (*srv).ev,
        (*con).fd,
        Some(
            connection_handle_fdevent
                as unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> handler_t,
        ),
        con as *mut libc::c_void,
    );
    (*con)
        .network_read = Some(
        connection_read_cq
            as unsafe extern "C" fn(
                *mut connection,
                *mut chunkqueue,
                off_t,
            ) -> libc::c_int,
    );
    (*con)
        .network_write = Some(
        connection_write_cq
            as unsafe extern "C" fn(
                *mut connection,
                *mut chunkqueue,
                off_t,
            ) -> libc::c_int,
    );
    (*con)
        .reqbody_read = Some(
        h1_reqbody_read as unsafe extern "C" fn(*mut request_st) -> handler_t,
    );
    let r: *mut request_st = &mut (*con).request;
    (*r).state = CON_STATE_REQUEST_START;
    (*con).connection_start = log_monotonic_secs;
    (*con).dst_addr = *cnt_addr;
    sock_addr_cache_inet_ntop_copy_buffer(
        &mut (*con).dst_addr_buf,
        &mut (*con).dst_addr,
    );
    (*con).srv_socket = srv_socket;
    (*con).is_writable = 1 as libc::c_int as libc::c_schar;
    (*con).is_ssl_sock = (*srv_socket).is_ssl as libc::c_char;
    (*con).proto_default_port = 80 as libc::c_int as uint16_t;
    config_cond_cache_reset(r);
    (*r)
        .conditional_is_valid = ((1 as libc::c_int) << COMP_SERVER_SOCKET as libc::c_int
        | (1 as libc::c_int) << COMP_HTTP_REMOTE_IP as libc::c_int) as uint32_t;
    if HANDLER_GO_ON as libc::c_int as libc::c_uint
        != plugins_call_handle_connection_accept(con) as libc::c_uint
    {
        connection_reset(con);
        connection_close(con);
        return 0 as *mut connection;
    }
    if (*r).http_status < 0 as libc::c_int {
        (*r).state = CON_STATE_WRITE;
    }
    return con;
}
unsafe extern "C" fn connection_state_machine_loop(
    r: *mut request_st,
    con: *mut connection,
) {
    let mut ostate: request_state_t = CON_STATE_CONNECT;
    let mut current_block_21: u64;
    loop {
        ostate = (*r).state;
        match ostate as libc::c_uint {
            1 => {
                connection_handle_request_start_state(r, con);
                (*r).state = CON_STATE_READ;
                current_block_21 = 15433940732114890699;
            }
            2 => {
                current_block_21 = 15433940732114890699;
            }
            4 | 5 => {
                current_block_21 = 17577394992057524319;
            }
            7 => {
                current_block_21 = 16449988584484264064;
            }
            8 | 9 => {
                current_block_21 = 1321270241106104922;
            }
            10 => {
                connection_handle_close_state(con);
                return;
            }
            0 => return,
            _ => return,
        }
        match current_block_21 {
            15433940732114890699 => {
                if h1_recv_headers(r, con) == 0 {
                    if (*r).http_version as libc::c_int == HTTP_VERSION_2 as libc::c_int
                    {
                        connection_transition_h2(r, con);
                        connection_state_machine(con);
                        ostate = CON_STATE_WRITE;
                        current_block_21 = 792017965103506125;
                    } else {
                        current_block_21 = 792017965103506125;
                    }
                } else {
                    (*r)
                        .state = (if 0 as libc::c_int as libc::c_long
                        == (*r).reqbody_length
                    {
                        CON_STATE_HANDLE_REQUEST as libc::c_int
                    } else {
                        CON_STATE_READ_POST as libc::c_int
                    }) as request_state_t;
                    current_block_21 = 17577394992057524319;
                }
            }
            _ => {}
        }
        match current_block_21 {
            17577394992057524319 => {
                match http_response_handler(r) as libc::c_uint {
                    0 | 1 => {
                        h1_send_headers(r);
                        (*r).state = CON_STATE_WRITE;
                        current_block_21 = 16449988584484264064;
                    }
                    2 => return,
                    _ => {
                        request_set_state_error(r, CON_STATE_ERROR);
                        current_block_21 = 792017965103506125;
                    }
                }
            }
            _ => {}
        }
        match current_block_21 {
            16449988584484264064 => {
                if connection_handle_write_state(r, con)
                    == CON_STATE_WRITE as libc::c_int
                {
                    return;
                }
                current_block_21 = 1321270241106104922;
            }
            _ => {}
        }
        match current_block_21 {
            1321270241106104922 => {
                connection_handle_response_end_state(r, con);
                ostate = CON_STATE_RESPONSE_END;
            }
            _ => {}
        }
        if !(ostate as libc::c_uint != (*r).state as libc::c_uint) {
            break;
        }
    };
}
#[cold]
unsafe extern "C" fn connection_revents_err(r: *mut request_st, con: *mut connection) {
    let revents: libc::c_int = (*con).revents_err as libc::c_int;
    (*con).revents_err = 0 as libc::c_int as uint16_t;
    if (*r).state as libc::c_uint == CON_STATE_CLOSE as libc::c_int as libc::c_uint {
        (*con)
            .close_timeout_ts = log_monotonic_secs
            - (5 as libc::c_int + 1 as libc::c_int) as libc::c_long;
    } else if revents & 0x10 as libc::c_int != 0 {
        request_set_state_error(r, CON_STATE_ERROR);
    } else if revents & 0x2000 as libc::c_int != 0 {
        let mut events: libc::c_int = if !((*con).fdn).is_null() {
            (*(*con).fdn).events
        } else {
            0 as libc::c_int
        };
        events &= !(0x1 as libc::c_int | 0x2000 as libc::c_int);
        (*r)
            .conf
            .stream_request_body = ((*r).conf.stream_request_body as libc::c_int
            & !((1 as libc::c_int) << 1 as libc::c_int
                | (1 as libc::c_int) << 15 as libc::c_int)) as libc::c_ushort;
        (*r)
            .conf
            .stream_request_body = ((*r).conf.stream_request_body as libc::c_int
            | (1 as libc::c_int) << 12 as libc::c_int) as libc::c_ushort;
        (*con).is_readable = 1 as libc::c_int as libc::c_schar;
        if chunkqueue_is_empty((*con).read_queue) != 0 {
            (*r).keep_alive = 0 as libc::c_int as int8_t;
        }
        if (*r).reqbody_length < -(1 as libc::c_int) as libc::c_long {
            (*r).reqbody_length = (*r).reqbody_queue.bytes_in;
        }
        if sock_addr_get_family(&mut (*con).dst_addr) == 1 as libc::c_int {
            fdevent_fdnode_event_set((*(*con).srv).ev, (*con).fdn, events);
        } else if fdevent_is_tcp_half_closed((*con).fd) != 0 {
            (*r)
                .conf
                .stream_request_body = ((*r).conf.stream_request_body as libc::c_int
                | (1 as libc::c_int) << 13 as libc::c_int) as libc::c_ushort;
            fdevent_fdnode_event_set((*(*con).srv).ev, (*con).fdn, events);
        } else {
            request_set_state_error(r, CON_STATE_ERROR);
        }
    } else if revents & 0x8 as libc::c_int != 0 {
        request_set_state_error(r, CON_STATE_ERROR);
    } else {
        log_error(
            (*r).conf.errh,
            b"connections.c\0" as *const u8 as *const libc::c_char,
            749 as libc::c_int as libc::c_uint,
            b"connection closed: poll() -> ??? %d\0" as *const u8 as *const libc::c_char,
            revents,
        );
    };
}
unsafe extern "C" fn connection_set_fdevent_interest(
    r: *mut request_st,
    con: *mut connection,
) {
    if (*con).fd < 0 as libc::c_int {
        return;
    }
    if (*con).revents_err as libc::c_int != 0
        && (*r).state as libc::c_uint != CON_STATE_ERROR as libc::c_int as libc::c_uint
    {
        connection_revents_err(r, con);
        connection_state_machine(con);
        return;
    }
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut current_block_18: u64;
    match (*r).state as libc::c_uint {
        2 => {
            n = 0x1 as libc::c_int;
            if (*r).conf.stream_request_body as libc::c_int
                & (1 as libc::c_int) << 12 as libc::c_int == 0
            {
                n |= 0x2000 as libc::c_int;
            }
            current_block_18 = 6009453772311597924;
        }
        7 => {
            if chunkqueue_is_empty((*con).write_queue) == 0
                && 0 as libc::c_int == (*con).is_writable as libc::c_int
                && 0 as libc::c_int == (*con).traffic_limit_reached as libc::c_int
            {
                n |= 0x4 as libc::c_int;
            }
            current_block_18 = 3395033283095938687;
        }
        4 => {
            current_block_18 = 3395033283095938687;
        }
        10 => {
            n = 0x1 as libc::c_int;
            current_block_18 = 6009453772311597924;
        }
        0 => return,
        _ => {
            current_block_18 = 6009453772311597924;
        }
    }
    match current_block_18 {
        3395033283095938687 => {
            if (*r).conf.stream_request_body as libc::c_int
                & (1 as libc::c_int) << 15 as libc::c_int != 0
            {
                n |= 0x1 as libc::c_int;
            }
            if (*r).conf.stream_request_body as libc::c_int
                & (1 as libc::c_int) << 12 as libc::c_int == 0
            {
                n |= 0x2000 as libc::c_int;
            }
        }
        _ => {}
    }
    let events: libc::c_int = if !((*con).fdn).is_null() {
        (*(*con).fdn).events
    } else {
        0 as libc::c_int
    };
    if ((*con).is_readable as libc::c_int) < 0 as libc::c_int {
        (*con).is_readable = 0 as libc::c_int as libc::c_schar;
        n |= 0x1 as libc::c_int;
    }
    if ((*con).is_writable as libc::c_int) < 0 as libc::c_int {
        (*con).is_writable = 0 as libc::c_int as libc::c_schar;
        n |= 0x4 as libc::c_int;
    }
    if events & 0x2000 as libc::c_int != 0 {
        n |= 0x2000 as libc::c_int;
    }
    if n == events {
        return;
    }
    if n & 0x1 as libc::c_int != 0 && events & 0x1 as libc::c_int == 0 {
        (*con).read_idle_ts = log_monotonic_secs;
    }
    if n & 0x4 as libc::c_int != 0 && events & 0x4 as libc::c_int == 0 {
        (*con).write_request_ts = log_monotonic_secs;
    }
    fdevent_fdnode_event_set((*(*con).srv).ev, (*con).fdn, n);
}
#[no_mangle]
pub unsafe extern "C" fn connection_state_machine(con: *mut connection) {
    let mut rc: libc::c_int = (((*con).fn_0).is_null()
        || ((*(*con).fn_0).process_streams)
            .expect(
                "non-null function pointer",
            )(
            con,
            Some(
                http_response_handler
                    as unsafe extern "C" fn(*mut request_st) -> handler_t,
            ),
            Some(
                connection_handle_write
                    as unsafe extern "C" fn(
                        *mut request_st,
                        *mut connection,
                    ) -> libc::c_int,
            ),
        ) != 0) as libc::c_int;
    let r: *mut request_st = &mut (*con).request;
    if rc != 0 {
        connection_state_machine_loop(r, con);
    }
    connection_set_fdevent_interest(r, con);
}
unsafe extern "C" fn connection_check_timeout(
    con: *mut connection,
    cur_ts: unix_time64_t,
) {
    let mut changed: libc::c_int = if !((*con).fn_0).is_null()
        && ((*(*con).fn_0).check_timeout).is_some()
    {
        ((*(*con).fn_0).check_timeout).expect("non-null function pointer")(con, cur_ts)
    } else {
        h1_check_timeout(con, cur_ts)
    };
    (*con).bytes_written_cur_second = 0 as libc::c_int as off_t;
    if ((*con).traffic_limit_reached as libc::c_int != 0 as libc::c_int) as libc::c_int
        as libc::c_long != 0
    {
        let r: *const request_st = &mut (*con).request;
        let t_diff: libc::c_int = (cur_ts - (*con).connection_start) as libc::c_int;
        if (*r).conf.bytes_per_second == 0 as libc::c_int as libc::c_uint
            || (*(*con).write_queue).bytes_out
                < (*r).conf.bytes_per_second as off_t
                    * (if t_diff != 0 { t_diff } else { 1 as libc::c_int })
                        as libc::c_long
        {
            (*con).traffic_limit_reached = 0 as libc::c_int as libc::c_char;
            changed = 1 as libc::c_int;
        }
    }
    if changed != 0 {
        connection_state_machine(con);
    }
}
#[no_mangle]
pub unsafe extern "C" fn connection_periodic_maint(
    srv: *mut server,
    cur_ts: unix_time64_t,
) {
    let mut con: *mut connection = (*srv).conns;
    let mut tc: *mut connection = 0 as *mut connection;
    while !con.is_null() {
        tc = (*con).next;
        connection_check_timeout(con, cur_ts);
        con = tc;
    }
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn connection_graceful_shutdown_maint(srv: *mut server) {
    let graceful_expire: libc::c_int = ((*srv).graceful_expire_ts != 0
        && (*srv).graceful_expire_ts < log_monotonic_secs) as libc::c_int;
    let mut con: *mut connection = (*srv).conns;
    let mut tc: *mut connection = 0 as *mut connection;
    while !con.is_null() {
        tc = (*con).next;
        let mut changed: libc::c_int = 0 as libc::c_int;
        let r: *mut request_st = &mut (*con).request;
        if (*r).state as libc::c_uint == CON_STATE_CLOSE as libc::c_int as libc::c_uint {
            if 5 as libc::c_int > 1 as libc::c_int {
                (*con).close_timeout_ts
                    -= (5 as libc::c_int - 1 as libc::c_int) as libc::c_long;
            }
            (*con).close_timeout_ts
                -= (graceful_expire << 1 as libc::c_int) as libc::c_long;
            if log_monotonic_secs - (*con).close_timeout_ts
                > 5 as libc::c_int as libc::c_long
            {
                changed = 1 as libc::c_int;
            }
        } else if !((*con).fn_0).is_null() {
            if ((*(*con).fn_0).goaway_graceful).expect("non-null function pointer")(con)
                != 0
            {
                changed = 1 as libc::c_int;
            }
        } else if (*r).state as libc::c_uint
            == CON_STATE_READ as libc::c_int as libc::c_uint
            && (*con).request_count > 1 as libc::c_int as libc::c_uint
            && chunkqueue_is_empty((*con).read_queue) != 0
        {
            request_set_state_error(r, CON_STATE_ERROR);
            changed = 1 as libc::c_int;
        } else if (*r).reqbody_length == -(2 as libc::c_int) as libc::c_long
            && (*r).conf.stream_request_body as libc::c_int
                & (1 as libc::c_int) << 13 as libc::c_int == 0
        {
            (*r)
                .conf
                .stream_request_body = ((*r).conf.stream_request_body as libc::c_int
                | (1 as libc::c_int) << 13 as libc::c_int) as libc::c_ushort;
            changed = 1 as libc::c_int;
        }
        if graceful_expire != 0
            && (*r).state as libc::c_uint
                != CON_STATE_CLOSE as libc::c_int as libc::c_uint
        {
            request_set_state_error(r, CON_STATE_ERROR);
            changed = 1 as libc::c_int;
        }
        (*r).keep_alive = 0 as libc::c_int as int8_t;
        (*r).conf.bytes_per_second = 0 as libc::c_int as libc::c_uint;
        (*r).conf.global_bytes_per_second = 0 as libc::c_int as libc::c_uint;
        if (*con).traffic_limit_reached != 0 {
            (*con).traffic_limit_reached = 0 as libc::c_int as libc::c_char;
            changed = 1 as libc::c_int;
        }
        if changed != 0 {
            connection_state_machine(con);
        }
        con = tc;
    }
}
