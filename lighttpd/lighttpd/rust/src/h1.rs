use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type stat_cache_entry;
    pub type pcre2_real_match_data_8;
    pub type plugin;
    pub type fdevents;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn buffer_eq_icase_slen(
        b: *const buffer,
        s: *const libc::c_char,
        slen: size_t,
    ) -> libc::c_int;
    fn chunkqueue_append_mem(cq: *mut chunkqueue, mem: *const libc::c_char, len: size_t);
    fn ck_assert_failed(
        filename: *const libc::c_char,
        line: libc::c_uint,
        msg: *const libc::c_char,
    ) -> !;
    fn hex2int(c: libc::c_uchar) -> libc::c_char;
    fn chunkqueue_append_buffer_open(cq: *mut chunkqueue) -> *mut buffer;
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
    fn http_request_headers_process(
        r: *mut request_st,
        hdrs: *mut libc::c_char,
        hoff: *const libc::c_ushort,
        scheme_port: libc::c_int,
    );
    fn buffer_string_prepare_append(b: *mut buffer, size: size_t) -> *mut libc::c_char;
    fn buffer_extend(b: *mut buffer, x: size_t) -> *mut libc::c_char;
    fn chunkqueue_prepend_buffer_open(cq: *mut chunkqueue) -> *mut buffer;
    fn chunkqueue_prepend_buffer_commit(cq: *mut chunkqueue);
    fn buffer_copy_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn http_status_append(b: *mut buffer, status: libc::c_int);
    fn buffer_append_str2(
        b: *mut buffer,
        s1: *const libc::c_char,
        len1: size_t,
        s2: *const libc::c_char,
        len2: size_t,
    );
    fn buffer_append_string_len(b: *mut buffer, s: *const libc::c_char, len: size_t);
    fn chunkqueue_append_buffer_commit(cq: *mut chunkqueue);
    fn chunkqueue_init(cq: *mut chunkqueue) -> *mut chunkqueue;
    fn chunkqueue_append_chunkqueue(cq: *mut chunkqueue, src: *mut chunkqueue);
    fn chunkqueue_steal_with_tempfiles(
        dest: *mut chunkqueue,
        src: *mut chunkqueue,
        len: off_t,
        errh: *mut log_error_st,
    ) -> libc::c_int;
    fn chunkqueue_mark_written(cq: *mut chunkqueue, len: off_t);
    fn chunkqueue_remove_finished_chunks(cq: *mut chunkqueue);
    fn chunkqueue_steal(dest: *mut chunkqueue, src: *mut chunkqueue, len: off_t);
    fn chunkqueue_read_data(
        cq: *mut chunkqueue,
        data: *mut libc::c_char,
        dlen: uint32_t,
        errh: *mut log_error_st,
    ) -> libc::c_int;
    fn chunkqueue_free(cq: *mut chunkqueue);
    fn chunkqueue_compact_mem(cq: *mut chunkqueue, clen: size_t);
    fn chunkqueue_compact_mem_offset(cq: *mut chunkqueue);
    fn chunkqueue_small_resp_optim(cq: *mut chunkqueue);
    fn http_date_time_to_str(
        s: *mut libc::c_char,
        sz: size_t,
        t: unix_time64_t,
    ) -> uint32_t;
    fn http_header_str_contains_token(
        s: *const libc::c_char,
        slen: uint32_t,
        m: *const libc::c_char,
        mlen: uint32_t,
    ) -> libc::c_int;
    fn http_header_remove_token(
        b: *mut buffer,
        m: *const libc::c_char,
        mlen: uint32_t,
    ) -> libc::c_int;
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
    fn http_header_request_get(
        r: *const request_st,
        id: http_header_e,
        k: *const libc::c_char,
        klen: uint32_t,
    ) -> *mut buffer;
    fn http_header_request_unset(
        r: *mut request_st,
        id: http_header_e,
        k: *const libc::c_char,
        klen: uint32_t,
    );
    fn http_header_parse_hoff(
        n: *const libc::c_char,
        clen: uint32_t,
        hoff: *mut libc::c_ushort,
    ) -> uint32_t;
    static mut log_epoch_secs: unix_time64_t;
    static mut log_monotonic_secs: unix_time64_t;
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
    fn log_pri_multiline(
        errh: *mut log_error_st,
        filename: *const libc::c_char,
        line: libc::c_uint,
        pri: libc::c_int,
        multiline: *const libc::c_char,
        len: size_t,
        fmt: *const libc::c_char,
        _: ...
    );
    fn request_reset_ex(r: *mut request_st);
    static mut http_dispatch: [http_dispatch; 4];
    fn http_response_reqbody_read_error(
        r: *mut request_st,
        http_status: libc::c_int,
    ) -> handler_t;
    fn http_response_delay(con: *mut connection);
    fn http_response_omit_header(
        r: *mut request_st,
        ds: *const data_string,
    ) -> libc::c_int;
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
pub type __syscall_slong_t = libc::c_long;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type off_t = __off64_t;
pub type pid_t = __pid_t;
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
pub struct data_string {
    pub key: buffer,
    pub fn_0: *const data_methods,
    pub type_0: data_type_t,
    pub ext: libc::c_int,
    pub value: buffer,
}
pub const HTTP_HEADER_CONTENT_LENGTH: http_header_e = 14;
pub const HTTP_HEADER_SERVER: http_header_e = 44;
pub const HTTP_HEADER_DATE: http_header_e = 20;
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
pub const HTTP_HEADER_COOKIE: http_header_e = 19;
pub const HTTP_HEADER_CONTENT_TYPE: http_header_e = 18;
pub const HTTP_HEADER_CONTENT_SECURITY_POLICY: http_header_e = 17;
pub const HTTP_HEADER_CONTENT_RANGE: http_header_e = 16;
pub const HTTP_HEADER_CONTENT_LOCATION: http_header_e = 15;
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
pub const COMP_HTTP_REMOTE_IP: C2RustUnnamed_7 = 8;
pub const COMP_SERVER_SOCKET: C2RustUnnamed_7 = 1;
pub type C2RustUnnamed_7 = libc::c_uint;
pub const COMP_LAST_ELEMENT: C2RustUnnamed_7 = 13;
pub const COMP_HTTP_REQUEST_HEADER: C2RustUnnamed_7 = 12;
pub const COMP_HTTP_REQUEST_METHOD: C2RustUnnamed_7 = 11;
pub const COMP_HTTP_SCHEME: C2RustUnnamed_7 = 10;
pub const COMP_HTTP_QUERY_STRING: C2RustUnnamed_7 = 9;
pub const COMP_HTTP_COOKIE: C2RustUnnamed_7 = 7;
pub const COMP_HTTP_LANGUAGE: C2RustUnnamed_7 = 6;
pub const COMP_HTTP_USER_AGENT: C2RustUnnamed_7 = 5;
pub const COMP_HTTP_REFERER: C2RustUnnamed_7 = 4;
pub const COMP_HTTP_HOST: C2RustUnnamed_7 = 3;
pub const COMP_HTTP_URL: C2RustUnnamed_7 = 2;
pub const COMP_UNSET: C2RustUnnamed_7 = 0;
#[inline]
unsafe extern "C" fn buffer_truncate(mut b: *mut buffer, mut len: uint32_t) {
    *((*b).ptr).offset(len as isize) = '\0' as i32 as libc::c_char;
    (*b).used = len.wrapping_add(1 as libc::c_int as libc::c_uint);
}
#[inline]
unsafe extern "C" fn buffer_clen(mut b: *const buffer) -> uint32_t {
    return ((*b).used)
        .wrapping_sub(
            (0 as libc::c_int as libc::c_uint != (*b).used) as libc::c_int
                as libc::c_uint,
        );
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
unsafe extern "C" fn chunkqueue_is_empty(mut cq: *const chunkqueue) -> libc::c_int {
    return (0 as *mut libc::c_void as *mut chunk == (*cq).first) as libc::c_int;
}
#[inline]
unsafe extern "C" fn chunkqueue_length(mut cq: *const chunkqueue) -> off_t {
    return (*cq).bytes_in - (*cq).bytes_out;
}
unsafe extern "C" fn h1_send_1xx_info(
    r: *mut request_st,
    con: *mut connection,
) -> libc::c_int {
    let cq: *mut chunkqueue = (*con).write_queue;
    let mut written: off_t = (*cq).bytes_out;
    let mut rc: libc::c_int = ((*con).network_write)
        .expect(
            "non-null function pointer",
        )(con, cq, (256 as libc::c_int * 1024 as libc::c_int) as off_t);
    written = (*cq).bytes_out - written;
    (*con).bytes_written_cur_second += written;
    if !((*r).conf.global_bytes_per_second_cnt_ptr).is_null() {
        *(*r).conf.global_bytes_per_second_cnt_ptr += written;
    }
    if rc < 0 as libc::c_int {
        request_set_state_error(r, CON_STATE_ERROR);
        return 0 as libc::c_int;
    }
    if chunkqueue_is_empty(cq) == 0 {
        (*con).is_writable = 0 as libc::c_int as libc::c_schar;
        if cq == &mut (*r).write_queue as *mut chunkqueue {
            (*con).write_queue = chunkqueue_init(0 as *mut chunkqueue);
            (*(*con).write_queue).bytes_in = (*cq).bytes_out;
            (*(*con).write_queue).bytes_out = (*cq).bytes_out;
            chunkqueue_append_chunkqueue((*con).write_queue, cq);
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn h1_send_1xx(
    r: *mut request_st,
    con: *mut connection,
) -> libc::c_int {
    let cq: *mut chunkqueue = (*con).write_queue;
    let b: *mut buffer = chunkqueue_append_buffer_open(cq);
    buffer_copy_string_len(
        b,
        b"HTTP/1.1 \0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    http_status_append(b, (*r).http_status);
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*r).resp_headers.used {
        let ds: *const data_string = *((*r).resp_headers.data).offset(i as isize)
            as *mut data_string;
        let klen: uint32_t = buffer_clen(&(*ds).key);
        let vlen: uint32_t = buffer_clen(&(*ds).value);
        if !(0 as libc::c_int as libc::c_uint == klen
            || 0 as libc::c_int as libc::c_uint == vlen)
        {
            buffer_append_str2(
                b,
                b"\r\n\0" as *const u8 as *const libc::c_char,
                (::core::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                (*ds).key.ptr,
                klen as size_t,
            );
            buffer_append_str2(
                b,
                b": \0" as *const u8 as *const libc::c_char,
                (::core::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong
                    as uint32_t)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
                (*ds).value.ptr,
                vlen as size_t,
            );
        }
        i = i.wrapping_add(1);
        i;
    }
    buffer_append_string_len(
        b,
        b"\r\n\r\n\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    chunkqueue_append_buffer_commit(cq);
    if (*con).traffic_limit_reached != 0 {
        return 1 as libc::c_int;
    }
    return h1_send_1xx_info(r, con);
}
unsafe extern "C" fn h1_send_100_continue(
    r: *mut request_st,
    con: *mut connection,
) -> libc::c_int {
    static mut http_100_continue: [libc::c_char; 26] = unsafe {
        *::core::mem::transmute::<
            &[u8; 26],
            &[libc::c_char; 26],
        >(b"HTTP/1.1 100 Continue\r\n\r\n\0")
    };
    if (*con).traffic_limit_reached != 0 {
        return 1 as libc::c_int;
    }
    let cq: *mut chunkqueue = (*con).write_queue;
    chunkqueue_append_mem(
        cq,
        http_100_continue.as_ptr(),
        (::core::mem::size_of::<[libc::c_char; 26]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    return h1_send_1xx_info(r, con);
}
#[cold]
unsafe extern "C" fn h1_send_headers_partial_1xx(r: *mut request_st, b: *mut buffer) {
    let con: *mut connection = (*r).con;
    let cq: *mut chunkqueue = (*con).write_queue;
    (*con).write_queue = &mut (*r).write_queue;
    let mut len: uint32_t = chunkqueue_length(cq) as uint32_t;
    if chunkqueue_read_data(
        cq,
        buffer_string_prepare_append(b, len as size_t),
        len,
        (*r).conf.errh,
    ) < 0 as libc::c_int
    {
        len = 0 as libc::c_int as uint32_t;
    }
    buffer_truncate(b, len);
    chunkqueue_free(cq);
}
#[no_mangle]
pub unsafe extern "C" fn h1_send_headers(r: *mut request_st) {
    (*(*r).con).keep_alive_idle = (*r).conf.max_keep_alive_idle as libc::c_int;
    if (0 as libc::c_int == (*r).conf.max_keep_alive_idle as libc::c_int) as libc::c_int
        as libc::c_long != 0
        || (*(*r).con).request_count > (*r).conf.max_keep_alive_requests as libc::c_uint
    {
        (*r).keep_alive = 0 as libc::c_int as int8_t;
    } else if 0 as libc::c_int as libc::c_long != (*r).reqbody_length
        && (*r).reqbody_length != (*r).reqbody_queue.bytes_in
        && (((*r).handler_module).is_null()
            || 0 as libc::c_int
                == (*r).conf.stream_request_body as libc::c_int
                    & ((1 as libc::c_int) << 0 as libc::c_int
                        | (1 as libc::c_int) << 1 as libc::c_int))
    {
        (*r).keep_alive = 0 as libc::c_int as int8_t;
    }
    if (*r).resp_htags & (1 as libc::c_ulong) << HTTP_HEADER_UPGRADE as libc::c_int != 0
        && (*r).http_version as libc::c_int == HTTP_VERSION_1_1 as libc::c_int
    {
        http_header_response_set(
            r,
            HTTP_HEADER_CONNECTION,
            b"Connection\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
            b"upgrade\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
    } else if (*r).keep_alive as libc::c_int <= 0 as libc::c_int {
        if ((*r).keep_alive as libc::c_int) < 0 as libc::c_int {
            http_response_delay((*r).con);
        }
        http_header_response_set(
            r,
            HTTP_HEADER_CONNECTION,
            b"Connection\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
            b"close\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
    } else if (*r).http_version as libc::c_int == HTTP_VERSION_1_0 as libc::c_int {
        http_header_response_set(
            r,
            HTTP_HEADER_CONNECTION,
            b"Connection\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
            b"keep-alive\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
    }
    if 304 as libc::c_int == (*r).http_status
        && (*r).resp_htags
            & (1 as libc::c_ulong) << HTTP_HEADER_CONTENT_ENCODING as libc::c_int != 0
    {
        http_header_response_unset(
            r,
            HTTP_HEADER_CONTENT_ENCODING,
            b"Content-Encoding\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
    }
    let cq: *mut chunkqueue = &mut (*r).write_queue;
    let b: *mut buffer = chunkqueue_prepend_buffer_open(cq);
    if cq != (*(*r).con).write_queue {
        h1_send_headers_partial_1xx(r, b);
    }
    buffer_append_string_len(
        b,
        if (*r).http_version as libc::c_int == HTTP_VERSION_1_1 as libc::c_int {
            b"HTTP/1.1 \0" as *const u8 as *const libc::c_char
        } else {
            b"HTTP/1.0 \0" as *const u8 as *const libc::c_char
        },
        (::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    http_status_append(b, (*r).http_status);
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut used: size_t = (*r).resp_headers.used as size_t;
    while i < used {
        let ds: *const data_string = *((*r).resp_headers.data).offset(i as isize)
            as *mut data_string;
        let klen: uint32_t = buffer_clen(&(*ds).key);
        let vlen: uint32_t = buffer_clen(&(*ds).value);
        if !((0 as libc::c_int as libc::c_uint == klen) as libc::c_int as libc::c_long
            != 0)
        {
            if !((0 as libc::c_int as libc::c_uint == vlen) as libc::c_int
                as libc::c_long != 0)
            {
                if !(*((*ds).key.ptr).offset(0 as libc::c_int as isize) as libc::c_int
                    & 0xdf as libc::c_int == 'X' as i32
                    && http_response_omit_header(r, ds) != 0)
                {
                    let mut s: *mut libc::c_char = buffer_extend(
                        b,
                        klen
                            .wrapping_add(vlen)
                            .wrapping_add(4 as libc::c_int as libc::c_uint) as size_t,
                    );
                    *s.offset(0 as libc::c_int as isize) = '\r' as i32 as libc::c_char;
                    *s.offset(1 as libc::c_int as isize) = '\n' as i32 as libc::c_char;
                    memcpy(
                        s.offset(2 as libc::c_int as isize) as *mut libc::c_void,
                        (*ds).key.ptr as *const libc::c_void,
                        klen as libc::c_ulong,
                    );
                    s = s
                        .offset(
                            (2 as libc::c_int as libc::c_uint).wrapping_add(klen)
                                as isize,
                        );
                    *s.offset(0 as libc::c_int as isize) = ':' as i32 as libc::c_char;
                    *s.offset(1 as libc::c_int as isize) = ' ' as i32 as libc::c_char;
                    memcpy(
                        s.offset(2 as libc::c_int as isize) as *mut libc::c_void,
                        (*ds).value.ptr as *const libc::c_void,
                        vlen as libc::c_ulong,
                    );
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    if (*r).resp_htags & (1 as libc::c_ulong) << HTTP_HEADER_DATE as libc::c_int == 0 {
        static mut tlast: unix_time64_t = 0 as libc::c_int as unix_time64_t;
        static mut tstr: [libc::c_char; 40] = unsafe {
            *::core::mem::transmute::<
                &[u8; 40],
                &mut [libc::c_char; 40],
            >(
                b"\r\nDate: \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            )
        };
        let cur_ts: unix_time64_t = log_epoch_secs;
        if (tlast != cur_ts) as libc::c_int as libc::c_long != 0 {
            tlast = cur_ts;
            http_date_time_to_str(
                tstr.as_mut_ptr().offset(8 as libc::c_int as isize),
                (::core::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong)
                    .wrapping_sub(8 as libc::c_int as libc::c_ulong),
                tlast,
            );
        }
        buffer_append_string_len(b, tstr.as_mut_ptr(), 37 as libc::c_int as size_t);
    }
    if (*r).resp_htags & (1 as libc::c_ulong) << HTTP_HEADER_SERVER as libc::c_int == 0
        && !((*r).conf.server_tag).is_null()
    {
        buffer_append_str2(
            b,
            b"\r\nServer: \0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
            (*(*r).conf.server_tag).ptr,
            buffer_clen((*r).conf.server_tag) as size_t,
        );
    }
    buffer_append_string_len(
        b,
        b"\r\n\r\n\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    (*r).resp_header_len = buffer_clen(b);
    if ((*r).conf).log_response_header() != 0 {
        log_pri_multiline(
            (*r).conf.errh,
            b"h1.c\0" as *const u8 as *const libc::c_char,
            248 as libc::c_int as libc::c_uint,
            7 as libc::c_int,
            (*b).ptr,
            buffer_clen(b) as size_t,
            b"fd:%d resp: \0" as *const u8 as *const libc::c_char,
            (*(*r).con).fd,
        );
    }
    chunkqueue_prepend_buffer_commit(cq);
    let mut cqlen: off_t = 0;
    if (*r).resp_body_finished as libc::c_int != 0
        && (*r).resp_htags
            & (1 as libc::c_ulong) << HTTP_HEADER_CONTENT_LENGTH as libc::c_int != 0
        && {
            cqlen = chunkqueue_length(cq) - (*r).resp_header_len as libc::c_long;
            cqlen > 0 as libc::c_int as libc::c_long
        } && cqlen < 16384 as libc::c_int as libc::c_long
    {
        chunkqueue_small_resp_optim(cq);
    }
}
#[cold]
unsafe extern "C" fn h1_discard_blank_line(
    cq: *mut chunkqueue,
    mut header_len: uint32_t,
) -> *mut chunk {
    chunkqueue_mark_written(cq, header_len as off_t);
    return (*cq).first;
}
unsafe extern "C" fn h1_recv_headers_more(
    con: *mut connection,
    cq: *mut chunkqueue,
    mut c: *mut chunk,
    olen: size_t,
) -> *mut chunk {
    if (c.is_null() || ((*c).next).is_null())
        && (*con).is_readable as libc::c_int > 0 as libc::c_int
    {
        (*con).read_idle_ts = log_monotonic_secs;
        if 0 as libc::c_int
            != ((*con).network_read)
                .expect(
                    "non-null function pointer",
                )(con, cq, (256 as libc::c_int * 1024 as libc::c_int) as off_t)
        {
            let r: *mut request_st = &mut (*con).request;
            request_set_state_error(r, CON_STATE_ERROR);
        }
        let r_0: *mut request_st = &mut (*con).request;
        if (*r_0).http_version as libc::c_int == HTTP_VERSION_2 as libc::c_int {
            return 0 as *mut chunk;
        }
    }
    if (*cq).first != (*cq).last && 0 as libc::c_int as libc::c_ulong != olen {
        let clen: size_t = chunkqueue_length(cq) as size_t;
        let mut block: size_t = olen
            .wrapping_add((16384 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
            & !(16384 as libc::c_int - 1 as libc::c_int) as libc::c_ulong;
        block = (block as libc::c_ulong)
            .wrapping_add(
                (if block.wrapping_sub(olen) > 1024 as libc::c_int as libc::c_ulong {
                    0 as libc::c_int
                } else {
                    16384 as libc::c_int
                }) as libc::c_ulong,
            ) as size_t as size_t;
        chunkqueue_compact_mem(cq, if block > clen { clen } else { block });
    }
    c = (*cq).first;
    return if !c.is_null()
        && ((*c).offset as size_t).wrapping_add(olen)
            < buffer_clen((*c).mem) as libc::c_ulong
    {
        c
    } else {
        0 as *mut chunk
    };
}
#[cold]
unsafe extern "C" fn h1_check_upgrade(
    r: *mut request_st,
    con: *mut connection,
) -> libc::c_int {
    let mut upgrade: *mut buffer = http_header_request_get(
        r,
        HTTP_HEADER_UPGRADE,
        b"Upgrade\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    let http_connection: *mut buffer = http_header_request_get(
        r,
        HTTP_HEADER_CONNECTION,
        b"Connection\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    if http_connection.is_null() {
        http_header_request_unset(
            r,
            HTTP_HEADER_UPGRADE,
            b"Upgrade\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
        return 0 as libc::c_int;
    }
    if (*r).http_version as libc::c_int == HTTP_VERSION_1_1 as libc::c_int {
        if http_header_str_contains_token(
            (*upgrade).ptr,
            buffer_clen(upgrade),
            b"h2c\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        ) == 0
        {
            return 0 as libc::c_int;
        }
        if http_header_str_contains_token(
            (*http_connection).ptr,
            buffer_clen(http_connection),
            b"HTTP2-Settings\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        ) != 0
        {
            if (http_dispatch[HTTP_VERSION_2 as libc::c_int as usize].upgrade_h2c)
                .is_some()
            {
                (http_dispatch[HTTP_VERSION_2 as libc::c_int as usize].upgrade_h2c)
                    .expect("non-null function pointer")(r, con);
            }
        }
        http_header_request_unset(
            r,
            HTTP_HEADER_HTTP2_SETTINGS,
            b"HTTP2-Settings\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
        http_header_remove_token(
            http_connection,
            b"HTTP2-Settings\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong as uint32_t)
                .wrapping_sub(1 as libc::c_int as libc::c_uint),
        );
    }
    http_header_request_unset(
        r,
        HTTP_HEADER_UPGRADE,
        b"Upgrade\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    http_header_remove_token(
        http_connection,
        b"Upgrade\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    if (*r).http_version as libc::c_int != HTTP_VERSION_2 as libc::c_int {
        return 0 as libc::c_int;
    }
    (*r)
        .conditional_is_valid = ((1 as libc::c_int) << COMP_SERVER_SOCKET as libc::c_int
        | (1 as libc::c_int) << COMP_HTTP_REMOTE_IP as libc::c_int) as uint32_t;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn h1_recv_headers(
    r: *mut request_st,
    con: *mut connection,
) -> libc::c_int {
    let cq: *mut chunkqueue = (*con).read_queue;
    let mut c: *mut chunk = (*cq).first;
    let mut clen: uint32_t = 0 as libc::c_int as uint32_t;
    let mut header_len: uint32_t = 0 as libc::c_int as uint32_t;
    let mut keepalive_request_start: uint8_t = 0 as libc::c_int as uint8_t;
    let mut pipelined_request_start: uint8_t = 0 as libc::c_int as uint8_t;
    let mut discard_blank: uint8_t = 0 as libc::c_int as uint8_t;
    let mut hoff: [libc::c_ushort; 8192] = [0; 8192];
    if (*con).request_count > 1 as libc::c_int as libc::c_uint {
        discard_blank = 1 as libc::c_int as uint8_t;
        if (*cq).bytes_in == (*r).x.h1.bytes_read_ckpt {
            keepalive_request_start = 1 as libc::c_int as uint8_t;
            if !c.is_null() {
                pipelined_request_start = 1 as libc::c_int as uint8_t;
                (*con).is_readable = 1 as libc::c_int as libc::c_schar;
            }
        }
    }
    let mut current_block_30: u64;
    loop {
        if !c.is_null() {
            clen = (buffer_clen((*c).mem) as libc::c_long - (*c).offset) as uint32_t;
            if !(0 as libc::c_int as libc::c_uint == clen) {
                if ((*c).offset
                    > (32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                        as libc::c_long) as libc::c_int as libc::c_long != 0
                {
                    chunkqueue_compact_mem_offset(cq);
                }
                hoff[0 as libc::c_int as usize] = 1 as libc::c_int as libc::c_ushort;
                hoff[1 as libc::c_int as usize] = (*c).offset as libc::c_ushort;
                header_len = http_header_parse_hoff(
                    ((*(*c).mem).ptr).offset((*c).offset as isize),
                    clen,
                    hoff.as_mut_ptr(),
                );
                let max_request_field_size: uint32_t = (*r).conf.max_request_field_size;
                if (if header_len != 0 { header_len } else { clen })
                    > max_request_field_size
                    || hoff[0 as libc::c_int as usize] as libc::c_ulong
                        >= (::core::mem::size_of::<[libc::c_ushort; 8192]>()
                            as libc::c_ulong)
                            .wrapping_div(
                                ::core::mem::size_of::<libc::c_ushort>() as libc::c_ulong,
                            )
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                {
                    log_error(
                        (*r).conf.errh,
                        b"h1.c\0" as *const u8 as *const libc::c_char,
                        413 as libc::c_int as libc::c_uint,
                        b"%s\0" as *const u8 as *const libc::c_char,
                        b"oversized request-header -> sending Status 431\0" as *const u8
                            as *const libc::c_char,
                    );
                    (*r).http_status = 431 as libc::c_int;
                    (*r).keep_alive = 0 as libc::c_int as int8_t;
                    return 1 as libc::c_int;
                }
                if (0 as libc::c_int as libc::c_uint != header_len) as libc::c_int
                    as libc::c_long != 0
                {
                    if (hoff[0 as libc::c_int as usize] as libc::c_int
                        > 1 as libc::c_int) as libc::c_int as libc::c_long != 0
                    {
                        break;
                    }
                    if discard_blank != 0 {
                        if header_len == clen {
                            current_block_30 = 17965632435239708295;
                        } else {
                            let ch: libc::c_int = *((*(*c).mem).ptr)
                                .offset(((*c).offset + header_len as libc::c_long) as isize)
                                as libc::c_int;
                            if ch != '\r' as i32 && ch != '\n' as i32 {
                                discard_blank = 0 as libc::c_int as uint8_t;
                                clen = 0 as libc::c_int as uint32_t;
                                c = h1_discard_blank_line(cq, header_len);
                                current_block_30 = 17965632435239708295;
                            } else {
                                current_block_30 = 11932355480408055363;
                            }
                        }
                    } else {
                        current_block_30 = 11932355480408055363;
                    }
                } else {
                    current_block_30 = 11932355480408055363;
                }
                match current_block_30 {
                    17965632435239708295 => {}
                    _ => {
                        if (*((*(*c).mem).ptr as *mut libc::c_uchar)
                            .offset((*c).offset as isize) as libc::c_int)
                            < 32 as libc::c_int
                        {
                            log_error(
                                (*r).conf.errh,
                                b"h1.c\0" as *const u8 as *const libc::c_char,
                                441 as libc::c_int as libc::c_uint,
                                b"%s (%s)\0" as *const u8 as *const libc::c_char,
                                if *((*(*c).mem).ptr).offset((*c).offset as isize)
                                    as libc::c_int == 0x16 as libc::c_int
                                {
                                    b"unexpected TLS ClientHello on clear port\0" as *const u8
                                        as *const libc::c_char
                                } else {
                                    b"invalid request-line -> sending Status 400\0" as *const u8
                                        as *const libc::c_char
                                },
                                (*con).dst_addr_buf.ptr,
                            );
                            (*r).http_status = 400 as libc::c_int;
                            (*r).keep_alive = 0 as libc::c_int as int8_t;
                            return 1 as libc::c_int;
                        }
                    }
                }
            }
        }
        c = h1_recv_headers_more(con, cq, c, clen as size_t);
        if c.is_null() {
            break;
        }
    }
    if keepalive_request_start != 0 {
        if (*cq).bytes_in > (*r).x.h1.bytes_read_ckpt {
            (*r).start_hp.tv_sec = log_epoch_secs;
            if ((*r).conf).high_precision_timestamps() != 0 {
                clock_gettime(0 as libc::c_int, &mut (*r).start_hp);
            }
        }
        if pipelined_request_start as libc::c_int != 0 && !c.is_null() {
            (*con).read_idle_ts = log_monotonic_secs;
        }
    }
    if c.is_null() {
        return 0 as libc::c_int;
    }
    let hdrs: *mut libc::c_char = ((*(*c).mem).ptr)
        .offset(hoff[1 as libc::c_int as usize] as libc::c_int as isize);
    if (*con).request_count > 1 as libc::c_int as libc::c_uint {
        (*r).x.h1.bytes_read_ckpt = (*cq).bytes_out;
        request_reset_ex(r);
    } else if (*con).is_ssl_sock == 0 && ((*r).conf).h2proto() as libc::c_int != 0
        && hoff[0 as libc::c_int as usize] as libc::c_int == 2 as libc::c_int
        && hoff[2 as libc::c_int as usize] as libc::c_int == 16 as libc::c_int
        && *hdrs.offset(0 as libc::c_int as isize) as libc::c_int == 'P' as i32
        && *hdrs.offset(1 as libc::c_int as isize) as libc::c_int == 'R' as i32
        && *hdrs.offset(2 as libc::c_int as isize) as libc::c_int == 'I' as i32
        && *hdrs.offset(3 as libc::c_int as isize) as libc::c_int == ' ' as i32
    {
        (*r).http_version = HTTP_VERSION_2;
        return 0 as libc::c_int;
    }
    (*r).rqst_header_len = header_len;
    if ((*r).conf).log_request_header() != 0 {
        log_pri_multiline(
            (*r).conf.errh,
            b"h1.c\0" as *const u8 as *const libc::c_char,
            497 as libc::c_int as libc::c_uint,
            7 as libc::c_int,
            hdrs,
            header_len as size_t,
            b"fd:%d rqst: \0" as *const u8 as *const libc::c_char,
            (*con).fd,
        );
    }
    http_request_headers_process(
        r,
        hdrs,
        hoff.as_mut_ptr(),
        (*con).proto_default_port as libc::c_int,
    );
    chunkqueue_mark_written(cq, (*r).rqst_header_len as off_t);
    if (*r).rqst_htags & (1 as libc::c_ulong) << HTTP_HEADER_UPGRADE as libc::c_int != 0
        && 0 as libc::c_int == (*r).http_status && h1_check_upgrade(r, con) != 0
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[cold]
unsafe extern "C" fn h1_check_expect_100(
    r: *mut request_st,
    con: *mut connection,
) -> libc::c_int {
    if (*con).is_writable as libc::c_int <= 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    let vb: *const buffer = http_header_request_get(
        r,
        HTTP_HEADER_EXPECT,
        b"Expect\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    if vb.is_null() {
        return 1 as libc::c_int;
    }
    let mut rc: libc::c_int = buffer_eq_icase_slen(
        vb,
        b"100-continue\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 13]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as size_t,
    );
    http_header_request_unset(
        r,
        HTTP_HEADER_EXPECT,
        b"Expect\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong as uint32_t)
            .wrapping_sub(1 as libc::c_int as libc::c_uint),
    );
    if rc == 0 || 0 as libc::c_int as libc::c_long != (*r).reqbody_queue.bytes_in
        || chunkqueue_is_empty(&mut (*r).read_queue) == 0
        || chunkqueue_is_empty(&mut (*r).write_queue) == 0
        || (*r).http_version as libc::c_int == HTTP_VERSION_1_0 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    return h1_send_100_continue(r, con);
}
unsafe extern "C" fn h1_cq_compact(cq: *mut chunkqueue) -> libc::c_int {
    let mut c: *mut chunk = (*cq).first;
    if c.is_null() {
        return 0 as libc::c_int;
    }
    let mlen: uint32_t = (buffer_clen((*c).mem) as libc::c_ulong)
        .wrapping_sub((*c).offset as size_t) as uint32_t;
    loop {
        c = (*c).next;
        if c.is_null() {
            break;
        }
        let blen: uint32_t = (buffer_clen((*c).mem) as libc::c_ulong)
            .wrapping_sub((*c).offset as size_t) as uint32_t;
        if 0 as libc::c_int as libc::c_uint == blen {
            continue;
        }
        chunkqueue_compact_mem(cq, mlen.wrapping_add(blen) as size_t);
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn h1_chunked_crlf(cq: *mut chunkqueue) -> libc::c_int {
    let mut c: *mut chunk = 0 as *mut chunk;
    let mut b: *mut buffer = 0 as *mut buffer;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    if chunkqueue_is_empty(cq) != 0 {
        return 0 as libc::c_int;
    }
    c = (*cq).first;
    b = (*c).mem;
    p = ((*b).ptr).offset((*c).offset as isize);
    if *p.offset(0 as libc::c_int as isize) as libc::c_int != '\r' as i32 {
        return -(1 as libc::c_int);
    }
    if *p.offset(1 as libc::c_int as isize) as libc::c_int == '\n' as i32 {
        return 1 as libc::c_int;
    }
    len = (buffer_clen(b) as libc::c_ulong).wrapping_sub((*c).offset as size_t);
    if 1 as libc::c_int as libc::c_ulong != len {
        return -(1 as libc::c_int);
    }
    loop {
        c = (*c).next;
        if c.is_null() {
            break;
        }
        b = (*c).mem;
        len = (buffer_clen(b) as libc::c_ulong).wrapping_sub((*c).offset as size_t);
        if 0 as libc::c_int as libc::c_ulong == len {
            continue;
        }
        p = ((*b).ptr).offset((*c).offset as isize);
        return if *p.offset(0 as libc::c_int as isize) as libc::c_int == '\n' as i32 {
            1 as libc::c_int
        } else {
            -(1 as libc::c_int)
        };
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn h1_chunked(
    r: *mut request_st,
    cq: *mut chunkqueue,
    dst_cq: *mut chunkqueue,
) -> handler_t {
    let max_request_size: off_t = ((*r).conf.max_request_size as off_t)
        << 10 as libc::c_int;
    let mut te_chunked: off_t = (*r).x.h1.te_chunked;
    loop {
        let mut len: off_t = chunkqueue_length(cq);
        while 0 as libc::c_int as libc::c_long == te_chunked {
            let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut c: *mut chunk = (*cq).first;
            if c.is_null() {
                break;
            }
            if !((*c).type_0 as libc::c_uint == MEM_CHUNK as libc::c_int as libc::c_uint)
            {
                ck_assert_failed(
                    b"h1.c\0" as *const u8 as *const libc::c_char,
                    606 as libc::c_int as libc::c_uint,
                    b"c->type == MEM_CHUNK\0" as *const u8 as *const libc::c_char,
                );
            }
            p = strchr(((*(*c).mem).ptr).offset((*c).offset as isize), '\n' as i32);
            if !p.is_null() {
                let mut hsz: off_t = p
                    .offset(1 as libc::c_int as isize)
                    .offset_from(((*(*c).mem).ptr).offset((*c).offset as isize))
                    as libc::c_long;
                let mut s: *mut libc::c_uchar = ((*(*c).mem).ptr as *mut libc::c_uchar)
                    .offset((*c).offset as isize);
                let mut u: libc::c_uchar = 0;
                loop {
                    u = hex2int(*s) as libc::c_uchar;
                    if !(u as libc::c_int != 0xff as libc::c_int) {
                        break;
                    }
                    if te_chunked
                        > ((1 as libc::c_ulonglong)
                            << (8 as libc::c_int as libc::c_ulong)
                                .wrapping_mul(
                                    ::core::mem::size_of::<off_t>() as libc::c_ulong,
                                )
                                .wrapping_sub(5 as libc::c_int as libc::c_ulong)) as off_t
                            - 1 as libc::c_int as libc::c_long
                            - 2 as libc::c_int as libc::c_long
                    {
                        log_error(
                            (*r).conf.errh,
                            b"h1.c\0" as *const u8 as *const libc::c_char,
                            613 as libc::c_int as libc::c_uint,
                            b"chunked data size too large -> 400\0" as *const u8
                                as *const libc::c_char,
                        );
                        return http_response_reqbody_read_error(r, 400 as libc::c_int);
                    }
                    te_chunked <<= 4 as libc::c_int;
                    te_chunked |= u as libc::c_long;
                    s = s.offset(1);
                    s;
                }
                if s
                    == ((*(*c).mem).ptr as *mut libc::c_uchar)
                        .offset((*c).offset as isize)
                {
                    log_error(
                        (*r).conf.errh,
                        b"h1.c\0" as *const u8 as *const libc::c_char,
                        622 as libc::c_int as libc::c_uint,
                        b"chunked header invalid chars -> 400\0" as *const u8
                            as *const libc::c_char,
                    );
                    return http_response_reqbody_read_error(r, 400 as libc::c_int);
                }
                while *s as libc::c_int == ' ' as i32 || *s as libc::c_int == '\t' as i32
                {
                    s = s.offset(1);
                    s;
                }
                if *s as libc::c_int != '\r' as i32 && *s as libc::c_int != ';' as i32 {
                    log_error(
                        (*r).conf.errh,
                        b"h1.c\0" as *const u8 as *const libc::c_char,
                        629 as libc::c_int as libc::c_uint,
                        b"chunked header invalid chars -> 400\0" as *const u8
                            as *const libc::c_char,
                    );
                    return http_response_reqbody_read_error(r, 400 as libc::c_int);
                }
                if hsz >= 1024 as libc::c_int as libc::c_long {
                    log_error(
                        (*r).conf.errh,
                        b"h1.c\0" as *const u8 as *const libc::c_char,
                        638 as libc::c_int as libc::c_uint,
                        b"chunked header line too long -> 400\0" as *const u8
                            as *const libc::c_char,
                    );
                    return http_response_reqbody_read_error(r, 400 as libc::c_int);
                }
                if 0 as libc::c_int as libc::c_long == te_chunked {
                    if *p.offset(0 as libc::c_int as isize) as libc::c_int == '\r' as i32
                        && *p.offset(1 as libc::c_int as isize) as libc::c_int
                            == '\n' as i32
                    {
                        hsz += 2 as libc::c_int as libc::c_long;
                    } else {
                        hsz -= 2 as libc::c_int as libc::c_long;
                        loop {
                            c = (*cq).first;
                            p = strstr(
                                ((*(*c).mem).ptr)
                                    .offset((*c).offset as isize)
                                    .offset(hsz as isize),
                                b"\r\n\r\n\0" as *const u8 as *const libc::c_char,
                            );
                            if !(p.is_null() && h1_cq_compact(cq) != 0) {
                                break;
                            }
                        }
                        if p.is_null() {
                            if buffer_clen((*c).mem) as off_t - (*c).offset
                                < (*r).conf.max_request_field_size as off_t
                            {
                                break;
                            }
                            (*r).keep_alive = 0 as libc::c_int as int8_t;
                            p = ((*(*c).mem).ptr)
                                .offset(buffer_clen((*c).mem) as isize)
                                .offset(-(4 as libc::c_int as isize));
                        }
                        hsz = p
                            .offset(4 as libc::c_int as isize)
                            .offset_from(((*(*c).mem).ptr).offset((*c).offset as isize))
                            as libc::c_long;
                    }
                    chunkqueue_mark_written(cq, hsz as size_t as off_t);
                    (*r).reqbody_length = (*dst_cq).bytes_in;
                    break;
                } else {
                    chunkqueue_mark_written(cq, hsz as size_t as off_t);
                    len = chunkqueue_length(cq);
                    if 0 as libc::c_int as libc::c_long != max_request_size
                        && (max_request_size < te_chunked
                            || max_request_size - te_chunked < (*dst_cq).bytes_in)
                    {
                        log_error(
                            (*r).conf.errh,
                            b"h1.c\0" as *const u8 as *const libc::c_char,
                            695 as libc::c_int as libc::c_uint,
                            b"request-size too long: %lld -> 413\0" as *const u8
                                as *const libc::c_char,
                            ((*dst_cq).bytes_in + te_chunked) as libc::c_longlong,
                        );
                        return http_response_reqbody_read_error(r, 413 as libc::c_int);
                    }
                    te_chunked += 2 as libc::c_int as libc::c_long;
                    break;
                }
            } else if buffer_clen((*c).mem) as off_t - (*c).offset
                >= 1024 as libc::c_int as libc::c_long
            {
                log_error(
                    (*r).conf.errh,
                    b"h1.c\0" as *const u8 as *const libc::c_char,
                    710 as libc::c_int as libc::c_uint,
                    b"chunked header line too long -> 400\0" as *const u8
                        as *const libc::c_char,
                );
                return http_response_reqbody_read_error(r, 400 as libc::c_int);
            } else if h1_cq_compact(cq) == 0 {
                break;
            }
        }
        if 0 as libc::c_int as libc::c_long == te_chunked {
            break;
        }
        if te_chunked > 2 as libc::c_int as libc::c_long {
            if len > te_chunked - 2 as libc::c_int as libc::c_long {
                len = te_chunked - 2 as libc::c_int as libc::c_long;
            }
            if (*dst_cq).bytes_in + te_chunked
                <= (64 as libc::c_int * 1024 as libc::c_int) as libc::c_long
            {
                chunkqueue_steal(dst_cq, cq, len);
            } else if 0 as libc::c_int
                != chunkqueue_steal_with_tempfiles(dst_cq, cq, len, (*r).conf.errh)
            {
                return http_response_reqbody_read_error(r, 500 as libc::c_int)
            }
            te_chunked -= len;
            len = chunkqueue_length(cq);
        }
        if len < te_chunked {
            break;
        }
        if 2 as libc::c_int as libc::c_long == te_chunked {
            if -(1 as libc::c_int) == h1_chunked_crlf(cq) {
                log_error(
                    (*r).conf.errh,
                    b"h1.c\0" as *const u8 as *const libc::c_char,
                    740 as libc::c_int as libc::c_uint,
                    b"chunked data missing end CRLF -> 400\0" as *const u8
                        as *const libc::c_char,
                );
                return http_response_reqbody_read_error(r, 400 as libc::c_int);
            }
            chunkqueue_mark_written(cq, 2 as libc::c_int as off_t);
            te_chunked -= 2 as libc::c_int as libc::c_long;
        }
        if !(chunkqueue_is_empty(cq) == 0) {
            break;
        }
    }
    (*r).x.h1.te_chunked = te_chunked;
    return HANDLER_GO_ON;
}
unsafe extern "C" fn h1_read_body_unknown(
    r: *mut request_st,
    cq: *mut chunkqueue,
    dst_cq: *mut chunkqueue,
) -> handler_t {
    let max_request_size: off_t = ((*r).conf.max_request_size as off_t)
        << 10 as libc::c_int;
    chunkqueue_append_chunkqueue(dst_cq, cq);
    if 0 as libc::c_int as libc::c_long != max_request_size
        && (*dst_cq).bytes_in > max_request_size
    {
        log_error(
            (*r).conf.errh,
            b"h1.c\0" as *const u8 as *const libc::c_char,
            763 as libc::c_int as libc::c_uint,
            b"request-size too long: %lld -> 413\0" as *const u8 as *const libc::c_char,
            (*dst_cq).bytes_in as libc::c_longlong,
        );
        return http_response_reqbody_read_error(r, 413 as libc::c_int);
    }
    return HANDLER_GO_ON;
}
#[no_mangle]
pub unsafe extern "C" fn h1_reqbody_read(r: *mut request_st) -> handler_t {
    let con: *mut connection = (*r).con;
    let cq: *mut chunkqueue = &mut (*r).read_queue;
    let dst_cq: *mut chunkqueue = &mut (*r).reqbody_queue;
    let mut is_closed: libc::c_int = 0 as libc::c_int;
    if (*con).is_readable as libc::c_int > 0 as libc::c_int {
        (*con).read_idle_ts = log_monotonic_secs;
        let max_per_read: off_t = (if (*r).conf.stream_request_body as libc::c_int
            & ((1 as libc::c_int) << 0 as libc::c_int
                | (1 as libc::c_int) << 1 as libc::c_int) == 0
        {
            256 as libc::c_int * 1024 as libc::c_int
        } else if (*r).conf.stream_request_body as libc::c_int
            & (1 as libc::c_int) << 1 as libc::c_int != 0
        {
            16384 as libc::c_int
        } else {
            65536 as libc::c_int
        }) as off_t;
        match ((*con).network_read)
            .expect("non-null function pointer")(con, cq, max_per_read)
        {
            -1 => {
                request_set_state_error(r, CON_STATE_ERROR);
                return HANDLER_ERROR;
            }
            -2 => {
                is_closed = 1 as libc::c_int;
            }
            _ => {}
        }
        chunkqueue_remove_finished_chunks(cq);
    }
    if (*r).rqst_htags & (1 as libc::c_ulong) << HTTP_HEADER_EXPECT as libc::c_int != 0
        && h1_check_expect_100(r, con) == 0
    {
        return HANDLER_ERROR;
    }
    if (*r).reqbody_length < 0 as libc::c_int as libc::c_long {
        let mut rc: handler_t = (if -(1 as libc::c_int) as libc::c_long
            == (*r).reqbody_length
        {
            h1_chunked(r, cq, dst_cq) as libc::c_uint
        } else {
            h1_read_body_unknown(r, cq, dst_cq) as libc::c_uint
        }) as handler_t;
        if HANDLER_GO_ON as libc::c_int as libc::c_uint != rc as libc::c_uint {
            return rc;
        }
        chunkqueue_remove_finished_chunks(cq);
    } else {
        let mut len: off_t = (*r).reqbody_length - (*dst_cq).bytes_in;
        if (*r).reqbody_length
            <= (64 as libc::c_int * 1024 as libc::c_int) as libc::c_long
        {
            chunkqueue_steal(dst_cq, cq, len);
        } else if chunkqueue_length(dst_cq) + len
            <= (64 as libc::c_int * 1024 as libc::c_int) as libc::c_long
            && (((*dst_cq).first).is_null()
                || (*(*dst_cq).first).type_0 as libc::c_uint
                    == MEM_CHUNK as libc::c_int as libc::c_uint)
        {
            chunkqueue_steal(dst_cq, cq, len);
        } else if 0 as libc::c_int
            != chunkqueue_steal_with_tempfiles(dst_cq, cq, len, (*r).conf.errh)
        {
            return http_response_reqbody_read_error(r, 500 as libc::c_int)
        }
        chunkqueue_remove_finished_chunks(cq);
    }
    if (*dst_cq).bytes_in == (*r).reqbody_length {
        (*r)
            .conf
            .stream_request_body = ((*r).conf.stream_request_body as libc::c_int
            & !((1 as libc::c_int) << 15 as libc::c_int)) as libc::c_ushort;
        if (*r).state as libc::c_uint
            == CON_STATE_READ_POST as libc::c_int as libc::c_uint
        {
            (*r).state = CON_STATE_HANDLE_REQUEST;
        }
        return HANDLER_GO_ON;
    } else if is_closed != 0 {
        return HANDLER_ERROR
    } else {
        (*r)
            .conf
            .stream_request_body = ((*r).conf.stream_request_body as libc::c_int
            | (1 as libc::c_int) << 15 as libc::c_int) as libc::c_ushort;
        return (if (*r).conf.stream_request_body as libc::c_int
            & (1 as libc::c_int) << 0 as libc::c_int != 0
        {
            HANDLER_GO_ON as libc::c_int
        } else {
            HANDLER_WAIT_FOR_EVENT as libc::c_int
        }) as handler_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn h1_check_timeout(
    con: *mut connection,
    cur_ts: unix_time64_t,
) -> libc::c_int {
    let r: *mut request_st = &mut (*con).request;
    let waitevents: libc::c_int = if !((*con).fdn).is_null() {
        (*(*con).fdn).events
    } else {
        0 as libc::c_int
    };
    let mut changed: libc::c_int = 0 as libc::c_int;
    if (*r).state as libc::c_uint == CON_STATE_CLOSE as libc::c_int as libc::c_uint {
        if cur_ts - (*con).close_timeout_ts > 5 as libc::c_int as libc::c_long {
            changed = 1 as libc::c_int;
        }
    } else if waitevents & 0x1 as libc::c_int != 0 {
        let mut keep_alive: libc::c_int = ((*con).request_count
            != 1 as libc::c_int as libc::c_uint
            && (*r).state as libc::c_uint
                == CON_STATE_READ as libc::c_int as libc::c_uint) as libc::c_int;
        let mut idle_timeout: libc::c_int = if keep_alive != 0 {
            (*con).keep_alive_idle
        } else {
            (*r).conf.max_read_idle as libc::c_int
        };
        if cur_ts - (*con).read_idle_ts > idle_timeout as libc::c_long {
            if ((*r).conf).log_timeouts() != 0 {
                log_debug(
                    (*r).conf.errh,
                    b"h1.c\0" as *const u8 as *const libc::c_char,
                    881 as libc::c_int as libc::c_uint,
                    b"connection closed - %s timeout: %d\0" as *const u8
                        as *const libc::c_char,
                    if keep_alive != 0 {
                        b"keep-alive\0" as *const u8 as *const libc::c_char
                    } else {
                        b"read\0" as *const u8 as *const libc::c_char
                    },
                    (*con).fd,
                );
            }
            request_set_state_error(r, CON_STATE_ERROR);
            changed = 1 as libc::c_int;
        }
    }
    if (*r).http_version as libc::c_int <= HTTP_VERSION_1_1 as libc::c_int
        && (*r).state as libc::c_uint == CON_STATE_WRITE as libc::c_int as libc::c_uint
        && (*con).write_request_ts != 0 as libc::c_int as libc::c_long
    {
        if cur_ts - (*con).write_request_ts > (*r).conf.max_write_idle as libc::c_long {
            if ((*r).conf).log_timeouts() != 0 {
                log_debug(
                    (*r).conf.errh,
                    b"h1.c\0" as *const u8 as *const libc::c_char,
                    914 as libc::c_int as libc::c_uint,
                    b"NOTE: a request from %s for %.*s timed out after writing %lld bytes. We waited %d seconds. If this is a problem, increase server.max-write-idle\0"
                        as *const u8 as *const libc::c_char,
                    (*(*r).dst_addr_buf).ptr,
                    buffer_clen(&mut (*r).target) as libc::c_int,
                    (*r).target.ptr,
                    (*(*con).write_queue).bytes_out as libc::c_longlong,
                    (*r).conf.max_write_idle as libc::c_int,
                );
            }
            request_set_state_error(r, CON_STATE_ERROR);
            changed = 1 as libc::c_int;
        }
    }
    return changed;
}
