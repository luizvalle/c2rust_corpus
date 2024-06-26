use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type fdlog_st;
    pub type sockaddr_x25;
    pub type sockaddr_ns;
    pub type sockaddr_iso;
    pub type sockaddr_ipx;
    pub type sockaddr_inarp;
    pub type sockaddr_eon;
    pub type sockaddr_dl;
    pub type sockaddr_ax25;
    pub type sockaddr_at;
    pub type __spawn_action;
    fn accept(
        __fd: libc::c_int,
        __addr: __SOCKADDR_ARG,
        __addr_len: *mut socklen_t,
    ) -> libc::c_int;
    fn accept4(
        __fd: libc::c_int,
        __addr: __SOCKADDR_ARG,
        __addr_len: *mut socklen_t,
        __flags: libc::c_int,
    ) -> libc::c_int;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn recv(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
    ) -> ssize_t;
    fn getsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *mut libc::c_void,
        __optlen: *mut socklen_t,
    ) -> libc::c_int;
    fn setsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *const libc::c_void,
        __optlen: socklen_t,
    ) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn lseek(__fd: libc::c_int, __offset: __off64_t, __whence: libc::c_int) -> __off64_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
    fn pipe2(__pipedes: *mut libc::c_int, __flags: libc::c_int) -> libc::c_int;
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    static mut environ: *mut *mut libc::c_char;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn mkostemp(__template: *mut libc::c_char, __flags: libc::c_int) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn ck_memclear_s(s: *mut libc::c_void, smax: rsize_t, n: rsize_t) -> errno_t;
    fn ck_assert_failed(
        filename: *const libc::c_char,
        line: libc::c_uint,
        msg: *const libc::c_char,
    ) -> !;
    fn rename(__old: *const libc::c_char, __new: *const libc::c_char) -> libc::c_int;
    fn sigfillset(__set: *mut sigset_t) -> libc::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn posix_spawn(
        __pid: *mut pid_t,
        __path: *const libc::c_char,
        __file_actions: *const posix_spawn_file_actions_t,
        __attrp: *const posix_spawnattr_t,
        __argv: *const *mut libc::c_char,
        __envp: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn posix_spawnattr_init(__attr: *mut posix_spawnattr_t) -> libc::c_int;
    fn posix_spawnattr_destroy(__attr: *mut posix_spawnattr_t) -> libc::c_int;
    fn posix_spawnattr_setsigdefault(
        __attr: *mut posix_spawnattr_t,
        __sigdefault: *const sigset_t,
    ) -> libc::c_int;
    fn posix_spawnattr_setsigmask(
        __attr: *mut posix_spawnattr_t,
        __sigmask: *const sigset_t,
    ) -> libc::c_int;
    fn posix_spawnattr_setflags(
        _attr: *mut posix_spawnattr_t,
        __flags: libc::c_short,
    ) -> libc::c_int;
    fn posix_spawn_file_actions_init(
        __file_actions: *mut posix_spawn_file_actions_t,
    ) -> libc::c_int;
    fn posix_spawn_file_actions_destroy(
        __file_actions: *mut posix_spawn_file_actions_t,
    ) -> libc::c_int;
    fn posix_spawn_file_actions_addclosefrom_np(
        _: *mut posix_spawn_file_actions_t,
        __from: libc::c_int,
    ) -> libc::c_int;
    fn posix_spawn_file_actions_addfchdir_np(
        _: *mut posix_spawn_file_actions_t,
        __fd: libc::c_int,
    ) -> libc::c_int;
    fn posix_spawn_file_actions_adddup2(
        __file_actions: *mut posix_spawn_file_actions_t,
        __fd: libc::c_int,
        __newfd: libc::c_int,
    ) -> libc::c_int;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
    ) -> __pid_t;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn log_perror(
        errh: *mut log_error_st,
        filename: *const libc::c_char,
        line: libc::c_uint,
        fmt: *const libc::c_char,
        _: ...
    );
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
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
pub type __socklen_t = libc::c_uint;
pub type __sig_atomic_t = libc::c_int;
pub type mode_t = __mode_t;
pub type off_t = __off64_t;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
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
pub type log_error_st = fdlog_st;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_STREAM: __socket_type = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
pub type sa_family_t = libc::c_ushort;
pub type socklen_t = __socklen_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __SOCKADDR_ARG {
    pub __sockaddr__: *mut sockaddr,
    pub __sockaddr_at__: *mut sockaddr_at,
    pub __sockaddr_ax25__: *mut sockaddr_ax25,
    pub __sockaddr_dl__: *mut sockaddr_dl,
    pub __sockaddr_eon__: *mut sockaddr_eon,
    pub __sockaddr_in__: *mut sockaddr_in,
    pub __sockaddr_in6__: *mut sockaddr_in6,
    pub __sockaddr_inarp__: *mut sockaddr_inarp,
    pub __sockaddr_ipx__: *mut sockaddr_ipx,
    pub __sockaddr_iso__: *mut sockaddr_iso,
    pub __sockaddr_ns__: *mut sockaddr_ns,
    pub __sockaddr_un__: *mut sockaddr_un,
    pub __sockaddr_x25__: *mut sockaddr_x25,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [libc::c_char; 108],
}
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
    pub __in6_u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
pub type in_port_t = uint16_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct posix_spawnattr_t {
    pub __flags: libc::c_short,
    pub __pgrp: pid_t,
    pub __sd: sigset_t,
    pub __ss: sigset_t,
    pub __sp: sched_param,
    pub __policy: libc::c_int,
    pub __pad: [libc::c_int; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sched_param {
    pub sched_priority: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct posix_spawn_file_actions_t {
    pub __allocated: libc::c_int,
    pub __used: libc::c_int,
    pub __actions: *mut __spawn_action,
    pub __pad: [libc::c_int; 16],
}
pub type sig_atomic_t = __sig_atomic_t;
pub const MSG_NOSIGNAL: C2RustUnnamed_0 = 16384;
pub const MSG_DONTWAIT: C2RustUnnamed_0 = 64;
pub const MSG_TRUNC: C2RustUnnamed_0 = 32;
pub const TCP_CLOSE_WAIT: C2RustUnnamed_2 = 8;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct tcp_info {
    pub tcpi_state: uint8_t,
    pub tcpi_ca_state: uint8_t,
    pub tcpi_retransmits: uint8_t,
    pub tcpi_probes: uint8_t,
    pub tcpi_backoff: uint8_t,
    pub tcpi_options: uint8_t,
    #[bitfield(name = "tcpi_snd_wscale", ty = "uint8_t", bits = "0..=3")]
    #[bitfield(name = "tcpi_rcv_wscale", ty = "uint8_t", bits = "4..=7")]
    pub tcpi_snd_wscale_tcpi_rcv_wscale: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub tcpi_rto: uint32_t,
    pub tcpi_ato: uint32_t,
    pub tcpi_snd_mss: uint32_t,
    pub tcpi_rcv_mss: uint32_t,
    pub tcpi_unacked: uint32_t,
    pub tcpi_sacked: uint32_t,
    pub tcpi_lost: uint32_t,
    pub tcpi_retrans: uint32_t,
    pub tcpi_fackets: uint32_t,
    pub tcpi_last_data_sent: uint32_t,
    pub tcpi_last_ack_sent: uint32_t,
    pub tcpi_last_data_recv: uint32_t,
    pub tcpi_last_ack_recv: uint32_t,
    pub tcpi_pmtu: uint32_t,
    pub tcpi_rcv_ssthresh: uint32_t,
    pub tcpi_rtt: uint32_t,
    pub tcpi_rttvar: uint32_t,
    pub tcpi_snd_ssthresh: uint32_t,
    pub tcpi_snd_cwnd: uint32_t,
    pub tcpi_advmss: uint32_t,
    pub tcpi_reordering: uint32_t,
    pub tcpi_rcv_rtt: uint32_t,
    pub tcpi_rcv_space: uint32_t,
    pub tcpi_total_retrans: uint32_t,
}
pub const IPPROTO_TCP: C2RustUnnamed_1 = 6;
pub type errno_t = libc::c_int;
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
pub type __socket_type = libc::c_uint;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const MSG_CMSG_CLOEXEC: C2RustUnnamed_0 = 1073741824;
pub const MSG_FASTOPEN: C2RustUnnamed_0 = 536870912;
pub const MSG_ZEROCOPY: C2RustUnnamed_0 = 67108864;
pub const MSG_BATCH: C2RustUnnamed_0 = 262144;
pub const MSG_WAITFORONE: C2RustUnnamed_0 = 65536;
pub const MSG_MORE: C2RustUnnamed_0 = 32768;
pub const MSG_ERRQUEUE: C2RustUnnamed_0 = 8192;
pub const MSG_RST: C2RustUnnamed_0 = 4096;
pub const MSG_CONFIRM: C2RustUnnamed_0 = 2048;
pub const MSG_SYN: C2RustUnnamed_0 = 1024;
pub const MSG_FIN: C2RustUnnamed_0 = 512;
pub const MSG_WAITALL: C2RustUnnamed_0 = 256;
pub const MSG_EOR: C2RustUnnamed_0 = 128;
pub const MSG_PROXY: C2RustUnnamed_0 = 16;
pub const MSG_CTRUNC: C2RustUnnamed_0 = 8;
pub const MSG_TRYHARD: C2RustUnnamed_0 = 4;
pub const MSG_DONTROUTE: C2RustUnnamed_0 = 4;
pub const MSG_PEEK: C2RustUnnamed_0 = 2;
pub const MSG_OOB: C2RustUnnamed_0 = 1;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const IPPROTO_MAX: C2RustUnnamed_1 = 263;
pub const IPPROTO_MPTCP: C2RustUnnamed_1 = 262;
pub const IPPROTO_RAW: C2RustUnnamed_1 = 255;
pub const IPPROTO_ETHERNET: C2RustUnnamed_1 = 143;
pub const IPPROTO_MPLS: C2RustUnnamed_1 = 137;
pub const IPPROTO_UDPLITE: C2RustUnnamed_1 = 136;
pub const IPPROTO_SCTP: C2RustUnnamed_1 = 132;
pub const IPPROTO_COMP: C2RustUnnamed_1 = 108;
pub const IPPROTO_PIM: C2RustUnnamed_1 = 103;
pub const IPPROTO_ENCAP: C2RustUnnamed_1 = 98;
pub const IPPROTO_BEETPH: C2RustUnnamed_1 = 94;
pub const IPPROTO_MTP: C2RustUnnamed_1 = 92;
pub const IPPROTO_AH: C2RustUnnamed_1 = 51;
pub const IPPROTO_ESP: C2RustUnnamed_1 = 50;
pub const IPPROTO_GRE: C2RustUnnamed_1 = 47;
pub const IPPROTO_RSVP: C2RustUnnamed_1 = 46;
pub const IPPROTO_IPV6: C2RustUnnamed_1 = 41;
pub const IPPROTO_DCCP: C2RustUnnamed_1 = 33;
pub const IPPROTO_TP: C2RustUnnamed_1 = 29;
pub const IPPROTO_IDP: C2RustUnnamed_1 = 22;
pub const IPPROTO_UDP: C2RustUnnamed_1 = 17;
pub const IPPROTO_PUP: C2RustUnnamed_1 = 12;
pub const IPPROTO_EGP: C2RustUnnamed_1 = 8;
pub const IPPROTO_IPIP: C2RustUnnamed_1 = 4;
pub const IPPROTO_IGMP: C2RustUnnamed_1 = 2;
pub const IPPROTO_ICMP: C2RustUnnamed_1 = 1;
pub const IPPROTO_IP: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const TCP_CLOSING: C2RustUnnamed_2 = 11;
pub const TCP_LISTEN: C2RustUnnamed_2 = 10;
pub const TCP_LAST_ACK: C2RustUnnamed_2 = 9;
pub const TCP_CLOSE: C2RustUnnamed_2 = 7;
pub const TCP_TIME_WAIT: C2RustUnnamed_2 = 6;
pub const TCP_FIN_WAIT2: C2RustUnnamed_2 = 5;
pub const TCP_FIN_WAIT1: C2RustUnnamed_2 = 4;
pub const TCP_SYN_RECV: C2RustUnnamed_2 = 3;
pub const TCP_SYN_SENT: C2RustUnnamed_2 = 2;
pub const TCP_ESTABLISHED: C2RustUnnamed_2 = 1;
#[inline]
unsafe extern "C" fn ck_memzero(mut s: *mut libc::c_void, mut n: rsize_t) -> errno_t {
    return ck_memclear_s(s, n, n);
}
static mut use_sock_cloexec: libc::c_int = 0;
static mut use_sock_nonblock: libc::c_int = 0;
#[no_mangle]
#[cold]
pub unsafe extern "C" fn fdevent_socket_nb_cloexec_init() {
    if use_sock_cloexec != 0 {
        return;
    }
    let mut fd: libc::c_int = socket(
        2 as libc::c_int,
        SOCK_STREAM as libc::c_int | SOCK_CLOEXEC as libc::c_int
            | SOCK_NONBLOCK as libc::c_int,
        0 as libc::c_int,
    );
    if fd >= 0 as libc::c_int {
        let mut flags: libc::c_int = fcntl(fd, 3 as libc::c_int, 0 as libc::c_int);
        use_sock_nonblock = (-(1 as libc::c_int) != flags
            && flags & 0o4000 as libc::c_int != 0) as libc::c_int;
        use_sock_cloexec = 1 as libc::c_int;
        close(fd);
    }
}
#[no_mangle]
pub unsafe extern "C" fn fdevent_setfd_cloexec(mut fd: libc::c_int) {
    if fd < 0 as libc::c_int {
        return;
    }
    if !(-(1 as libc::c_int) != fcntl(fd, 2 as libc::c_int, 1 as libc::c_int)) {
        ck_assert_failed(
            b"fdevent.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int as libc::c_uint,
            b"-1 != fcntl(fd, 2, 1)\0" as *const u8 as *const libc::c_char,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn fdevent_clrfd_cloexec(mut fd: libc::c_int) {
    if fd >= 0 as libc::c_int {
        if !(-(1 as libc::c_int) != fcntl(fd, 2 as libc::c_int, 0 as libc::c_int)) {
            ck_assert_failed(
                b"fdevent.c\0" as *const u8 as *const libc::c_char,
                65 as libc::c_int as libc::c_uint,
                b"-1 != fcntl(fd, 2, 0)\0" as *const u8 as *const libc::c_char,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn fdevent_fcntl_set_nb(mut fd: libc::c_int) -> libc::c_int {
    return fcntl(fd, 4 as libc::c_int, 0o4000 as libc::c_int | 0o2 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn fdevent_fcntl_set_nb_cloexec(
    mut fd: libc::c_int,
) -> libc::c_int {
    fdevent_setfd_cloexec(fd);
    return fdevent_fcntl_set_nb(fd);
}
#[no_mangle]
pub unsafe extern "C" fn fdevent_fcntl_set_nb_cloexec_sock(
    mut fd: libc::c_int,
) -> libc::c_int {
    if use_sock_cloexec != 0 && use_sock_nonblock != 0 {
        return 0 as libc::c_int;
    }
    return fdevent_fcntl_set_nb_cloexec(fd);
}
#[no_mangle]
pub unsafe extern "C" fn fdevent_socket_cloexec(
    mut domain: libc::c_int,
    mut type_0: libc::c_int,
    mut protocol: libc::c_int,
) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    if use_sock_cloexec != 0 {
        return socket(domain, type_0 | SOCK_CLOEXEC as libc::c_int, protocol);
    }
    fd = socket(domain, type_0, protocol);
    if -(1 as libc::c_int) != fd {
        if !(-(1 as libc::c_int) != fcntl(fd, 2 as libc::c_int, 1 as libc::c_int)) {
            ck_assert_failed(
                b"fdevent.c\0" as *const u8 as *const libc::c_char,
                101 as libc::c_int as libc::c_uint,
                b"-1 != fcntl(fd, 2, 1)\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    return fd;
}
#[no_mangle]
pub unsafe extern "C" fn fdevent_socket_nb_cloexec(
    mut domain: libc::c_int,
    mut type_0: libc::c_int,
    mut protocol: libc::c_int,
) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    if use_sock_cloexec != 0 && use_sock_nonblock != 0 {
        return socket(
            domain,
            type_0 | SOCK_CLOEXEC as libc::c_int | SOCK_NONBLOCK as libc::c_int,
            protocol,
        );
    }
    fd = socket(domain, type_0, protocol);
    if -(1 as libc::c_int) != fd {
        if !(-(1 as libc::c_int) != fcntl(fd, 2 as libc::c_int, 1 as libc::c_int)) {
            ck_assert_failed(
                b"fdevent.c\0" as *const u8 as *const libc::c_char,
                125 as libc::c_int as libc::c_uint,
                b"-1 != fcntl(fd, 2, 1)\0" as *const u8 as *const libc::c_char,
            );
        }
        if !(-(1 as libc::c_int)
            != fcntl(fd, 4 as libc::c_int, 0o4000 as libc::c_int | 0o2 as libc::c_int))
        {
            ck_assert_failed(
                b"fdevent.c\0" as *const u8 as *const libc::c_char,
                128 as libc::c_int as libc::c_uint,
                b"-1 != fcntl(fd, 4, 04000 | 02)\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    return fd;
}
#[no_mangle]
pub unsafe extern "C" fn fdevent_dup_cloexec(mut fd: libc::c_int) -> libc::c_int {
    return fcntl(fd, 1030 as libc::c_int, 3 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn fdevent_open_cloexec(
    mut pathname: *const libc::c_char,
    mut symlinks: libc::c_int,
    mut flags: libc::c_int,
    mut mode: mode_t,
) -> libc::c_int {
    if symlinks == 0 {
        flags |= 0o100000 as libc::c_int;
    }
    return open(
        pathname,
        flags | 0o2000000 as libc::c_int
            | (0 as libc::c_int | 0 as libc::c_int | 0o400 as libc::c_int
                | 0o4000 as libc::c_int),
        mode,
    );
}
#[no_mangle]
pub unsafe extern "C" fn fdevent_open_devnull() -> libc::c_int {
    return fdevent_open_cloexec(
        b"/dev/null\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        0o2 as libc::c_int,
        0 as libc::c_int as mode_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn fdevent_open_dirname(
    mut path: *mut libc::c_char,
    mut symlinks: libc::c_int,
) -> libc::c_int {
    let c: *mut libc::c_char = strrchr(path, '/' as i32);
    let dname: *const libc::c_char = if !c.is_null() {
        if c == path {
            b"/\0" as *const u8 as *const libc::c_char
        } else {
            path as *const libc::c_char
        }
    } else {
        b".\0" as *const u8 as *const libc::c_char
    };
    let mut dfd: libc::c_int = 0;
    let mut flags: libc::c_int = 0 as libc::c_int;
    flags |= 0o40000 as libc::c_int;
    if !c.is_null() {
        *c = '\0' as i32 as libc::c_char;
    }
    dfd = fdevent_open_cloexec(dname, symlinks, flags, 0 as libc::c_int as mode_t);
    if !c.is_null() {
        *c = '/' as i32 as libc::c_char;
    }
    return dfd;
}
#[no_mangle]
pub unsafe extern "C" fn fdevent_pipe_cloexec(
    fds: *mut libc::c_int,
    bufsz_hint: libc::c_uint,
) -> libc::c_int {
    if 0 as libc::c_int != pipe2(fds, 0o2000000 as libc::c_int) {
        if 0 as libc::c_int != pipe(fds)
            || 0 as libc::c_int
                != fcntl(
                    *fds.offset(0 as libc::c_int as isize),
                    2 as libc::c_int,
                    1 as libc::c_int,
                )
            || 0 as libc::c_int
                != fcntl(
                    *fds.offset(1 as libc::c_int as isize),
                    2 as libc::c_int,
                    1 as libc::c_int,
                )
        {
            return -(1 as libc::c_int);
        }
    }
    if bufsz_hint > 65536 as libc::c_int as libc::c_uint {
        0 as libc::c_int
            != fcntl(
                *fds.offset(1 as libc::c_int as isize),
                1031 as libc::c_int,
                bufsz_hint,
            );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fdevent_socket_close(mut fd: libc::c_int) -> libc::c_int {
    return close(fd);
}
#[no_mangle]
pub unsafe extern "C" fn fdevent_mkostemp(
    mut path: *mut libc::c_char,
    mut flags: libc::c_int,
) -> libc::c_int {
    return mkostemp(path, 0o2000000 as libc::c_int | flags);
}
#[no_mangle]
pub unsafe extern "C" fn fdevent_accept_listenfd(
    mut listenfd: libc::c_int,
    mut addr: *mut sockaddr,
    mut addrlen: *mut size_t,
) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    let mut len: socklen_t = *addrlen as socklen_t;
    let mut sock_cloexec: libc::c_int = use_sock_cloexec;
    if sock_cloexec != 0 {
        fd = accept4(
            listenfd,
            __SOCKADDR_ARG {
                __sockaddr__: addr,
            },
            &mut len,
            SOCK_CLOEXEC as libc::c_int | SOCK_NONBLOCK as libc::c_int,
        );
        if fd >= 0 as libc::c_int {
            if use_sock_nonblock == 0 {
                if 0 as libc::c_int != fdevent_fcntl_set_nb(fd) {
                    close(fd);
                    fd = -(1 as libc::c_int);
                }
            }
        } else {
            match *__errno_location() {
                38 | 95 | 1 => {
                    fd = accept(
                        listenfd,
                        __SOCKADDR_ARG {
                            __sockaddr__: addr,
                        },
                        &mut len,
                    );
                    sock_cloexec = 0 as libc::c_int;
                }
                _ => {}
            }
        }
    } else {
        fd = accept(
            listenfd,
            __SOCKADDR_ARG {
                __sockaddr__: addr,
            },
            &mut len,
        );
    }
    if fd >= 0 as libc::c_int {
        *addrlen = len as size_t;
        if sock_cloexec == 0 && 0 as libc::c_int != fdevent_fcntl_set_nb_cloexec(fd) {
            close(fd);
            fd = -(1 as libc::c_int);
        }
    }
    return fd;
}
#[no_mangle]
pub unsafe extern "C" fn fdevent_environ() -> *mut *mut libc::c_char {
    return environ;
}
unsafe extern "C" fn fdevent_dup2_close_clrfd_cloexec(
    mut oldfd: libc::c_int,
    mut newfd: libc::c_int,
) -> libc::c_int {
    if oldfd >= 0 as libc::c_int {
        if oldfd != newfd {
            if !(oldfd > 2 as libc::c_int) {
                ck_assert_failed(
                    b"fdevent.c\0" as *const u8 as *const libc::c_char,
                    386 as libc::c_int as libc::c_uint,
                    b"oldfd > 2\0" as *const u8 as *const libc::c_char,
                );
            }
            if newfd != dup2(oldfd, newfd) {
                return -(1 as libc::c_int);
            }
        } else {
            fdevent_clrfd_cloexec(newfd);
        }
    }
    return newfd;
}
#[no_mangle]
pub unsafe extern "C" fn fdevent_set_stdin_stdout_stderr(
    mut fdin: libc::c_int,
    mut fdout: libc::c_int,
    mut fderr: libc::c_int,
) -> libc::c_int {
    if 0 as libc::c_int != fdevent_dup2_close_clrfd_cloexec(fdin, 0 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if 1 as libc::c_int != fdevent_dup2_close_clrfd_cloexec(fdout, 1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if 2 as libc::c_int != fdevent_dup2_close_clrfd_cloexec(fderr, 2 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fdevent_rename(
    mut oldpath: *const libc::c_char,
    mut newpath: *const libc::c_char,
) -> libc::c_int {
    return rename(oldpath, newpath);
}
#[no_mangle]
pub unsafe extern "C" fn fdevent_fork_execve(
    mut name: *const libc::c_char,
    mut argv: *mut *mut libc::c_char,
    mut envp: *mut *mut libc::c_char,
    mut fdin: libc::c_int,
    mut fdout: libc::c_int,
    mut fderr: libc::c_int,
    mut dfd: libc::c_int,
) -> pid_t {
    let mut sigs: sigset_t = sigset_t { __val: [0; 16] };
    let mut file_actions: posix_spawn_file_actions_t = posix_spawn_file_actions_t {
        __allocated: 0,
        __used: 0,
        __actions: 0 as *mut __spawn_action,
        __pad: [0; 16],
    };
    let mut attr: posix_spawnattr_t = posix_spawnattr_t {
        __flags: 0,
        __pgrp: 0,
        __sd: sigset_t { __val: [0; 16] },
        __ss: sigset_t { __val: [0; 16] },
        __sp: sched_param { sched_priority: 0 },
        __policy: 0,
        __pad: [0; 16],
    };
    let mut rc: libc::c_int = 0;
    let mut pid: pid_t = -(1 as libc::c_int);
    rc = posix_spawn_file_actions_init(&mut file_actions);
    if 0 as libc::c_int != rc {
        return pid;
    }
    rc = posix_spawnattr_init(&mut attr);
    if 0 as libc::c_int != rc {
        posix_spawn_file_actions_destroy(&mut file_actions);
        return pid;
    }
    rc = if fdin >= 0 as libc::c_int {
        posix_spawn_file_actions_adddup2(&mut file_actions, fdin, 0 as libc::c_int)
    } else {
        0 as libc::c_int
    };
    if 0 as libc::c_int == rc
        && {
            rc = if fdout >= 0 as libc::c_int {
                posix_spawn_file_actions_adddup2(
                    &mut file_actions,
                    fdout,
                    1 as libc::c_int,
                )
            } else {
                0 as libc::c_int
            };
            0 as libc::c_int == rc
        }
        && {
            rc = if fderr >= 0 as libc::c_int {
                posix_spawn_file_actions_adddup2(
                    &mut file_actions,
                    fderr,
                    2 as libc::c_int,
                )
            } else {
                0 as libc::c_int
            };
            0 as libc::c_int == rc
        }
        && {
            rc = if -(1 as libc::c_int) != dfd {
                posix_spawn_file_actions_addfchdir_np(&mut file_actions, dfd)
            } else {
                0 as libc::c_int
            };
            0 as libc::c_int == rc
        }
        && {
            rc = posix_spawnattr_setflags(
                &mut attr,
                (0x4 as libc::c_int | 0x8 as libc::c_int) as libc::c_short,
            );
            0 as libc::c_int == rc
        }
        && {
            rc = sigemptyset(&mut sigs);
            0 as libc::c_int == rc
        }
        && {
            rc = posix_spawnattr_setsigmask(&mut attr, &mut sigs);
            0 as libc::c_int == rc
        }
        && {
            rc = sigfillset(&mut sigs);
            0 as libc::c_int == rc
        }
        && {
            rc = posix_spawnattr_setsigdefault(&mut attr, &mut sigs);
            0 as libc::c_int == rc
        }
    {
        static mut trace_children: sig_atomic_t = 0;
        if trace_children == 0 {
            posix_spawn_file_actions_addclosefrom_np(
                &mut file_actions,
                3 as libc::c_int,
            );
        }
        rc = posix_spawn(
            &mut pid,
            name,
            &mut file_actions,
            &mut attr,
            argv as *const *mut libc::c_char,
            (if !envp.is_null() { envp } else { environ }) as *const *mut libc::c_char,
        );
        if 0 as libc::c_int != rc {
            pid = -(1 as libc::c_int);
        }
    }
    posix_spawn_file_actions_destroy(&mut file_actions);
    posix_spawnattr_destroy(&mut attr);
    return pid;
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn fdevent_sh_exec(
    mut cmdstr: *const libc::c_char,
    mut envp: *mut *mut libc::c_char,
    mut fdin: libc::c_int,
    mut fdout: libc::c_int,
    mut fderr: libc::c_int,
) -> pid_t {
    let mut args: [*mut libc::c_char; 4] = [0 as *mut libc::c_char; 4];
    if *cmdstr as libc::c_int == '/' as i32
        && *cmdstr
            .offset(
                strcspn(
                    cmdstr,
                    b"\t\n !\"#$&'()*;<>?[\\]`{|}\0" as *const u8 as *const libc::c_char,
                ) as isize,
            ) as libc::c_int == '\0' as i32
    {
        let ref mut fresh0 = *(&mut *args.as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut *mut libc::c_char as *mut *const libc::c_char);
        *fresh0 = cmdstr;
        args[1 as libc::c_int as usize] = 0 as *mut libc::c_char;
    } else {
        let mut shell: *const libc::c_char = getenv(
            b"SHELL\0" as *const u8 as *const libc::c_char,
        );
        if !shell.is_null()
            && (0 as libc::c_int
                == strcmp(shell, b"/usr/bin/false\0" as *const u8 as *const libc::c_char)
                || 0 as libc::c_int
                    == strcmp(
                        shell,
                        b"/bin/false\0" as *const u8 as *const libc::c_char,
                    ))
        {
            shell = 0 as *const libc::c_char;
        }
        let ref mut fresh1 = *(&mut *args.as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut *mut libc::c_char as *mut *const libc::c_char);
        *fresh1 = if !shell.is_null() {
            shell
        } else {
            b"/bin/sh\0" as *const u8 as *const libc::c_char
        };
        let ref mut fresh2 = *(&mut *args.as_mut_ptr().offset(1 as libc::c_int as isize)
            as *mut *mut libc::c_char as *mut *const libc::c_char);
        *fresh2 = b"-c\0" as *const u8 as *const libc::c_char;
        let ref mut fresh3 = *(&mut *args.as_mut_ptr().offset(2 as libc::c_int as isize)
            as *mut *mut libc::c_char as *mut *const libc::c_char);
        *fresh3 = cmdstr;
        args[3 as libc::c_int as usize] = 0 as *mut libc::c_char;
    }
    return fdevent_fork_execve(
        args[0 as libc::c_int as usize],
        args.as_mut_ptr(),
        envp,
        fdin,
        fdout,
        fderr,
        -(1 as libc::c_int),
    );
}
#[no_mangle]
pub unsafe extern "C" fn fdevent_kill(
    mut pid: pid_t,
    mut sig: libc::c_int,
) -> libc::c_int {
    return kill(pid, sig);
}
#[no_mangle]
pub unsafe extern "C" fn fdevent_waitpid(
    mut pid: pid_t,
    status: *mut libc::c_int,
    mut nb: libc::c_int,
) -> pid_t {
    let flags: libc::c_int = if nb != 0 { 1 as libc::c_int } else { 0 as libc::c_int };
    let mut rv: pid_t = 0;
    loop {
        rv = waitpid(pid, status, flags);
        if !(-(1 as libc::c_int) == rv && *__errno_location() == 4 as libc::c_int) {
            break;
        }
    }
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn fdevent_waitpid_intr(
    mut pid: pid_t,
    status: *mut libc::c_int,
) -> pid_t {
    return waitpid(pid, status, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn fdevent_socket_read_discard(
    mut fd: libc::c_int,
    mut buf: *mut libc::c_char,
    mut sz: size_t,
    mut family: libc::c_int,
    mut so_type: libc::c_int,
) -> ssize_t {
    if (family == 2 as libc::c_int || family == 10 as libc::c_int)
        && so_type == SOCK_STREAM as libc::c_int
    {
        let mut len: ssize_t = recv(
            fd,
            buf as *mut libc::c_void,
            sz,
            MSG_TRUNC as libc::c_int | MSG_DONTWAIT as libc::c_int
                | MSG_NOSIGNAL as libc::c_int,
        );
        if len >= 0 as libc::c_int as libc::c_long
            || *__errno_location() != 22 as libc::c_int
        {
            return len;
        }
    }
    return read(fd, buf as *mut libc::c_void, sz);
}
#[no_mangle]
pub unsafe extern "C" fn fdevent_ioctl_fionread(
    mut fd: libc::c_int,
    mut fdfmt: libc::c_int,
    mut toread: *mut libc::c_int,
) -> libc::c_int {
    return ioctl(fd, 0x541b as libc::c_int as libc::c_ulong, toread);
}
#[no_mangle]
pub unsafe extern "C" fn fdevent_connect_status(mut fd: libc::c_int) -> libc::c_int {
    let mut opt: libc::c_int = 0;
    let mut len: socklen_t = ::core::mem::size_of::<libc::c_int>() as libc::c_ulong
        as socklen_t;
    return if 0 as libc::c_int
        == getsockopt(
            fd,
            1 as libc::c_int,
            4 as libc::c_int,
            &mut opt as *mut libc::c_int as *mut libc::c_void,
            &mut len,
        )
    {
        opt
    } else {
        *__errno_location()
    };
}
#[no_mangle]
pub unsafe extern "C" fn fdevent_is_tcp_half_closed(mut fd: libc::c_int) -> libc::c_int {
    let mut tcpi: tcp_info = tcp_info {
        tcpi_state: 0,
        tcpi_ca_state: 0,
        tcpi_retransmits: 0,
        tcpi_probes: 0,
        tcpi_backoff: 0,
        tcpi_options: 0,
        tcpi_snd_wscale_tcpi_rcv_wscale: [0; 1],
        c2rust_padding: [0; 1],
        tcpi_rto: 0,
        tcpi_ato: 0,
        tcpi_snd_mss: 0,
        tcpi_rcv_mss: 0,
        tcpi_unacked: 0,
        tcpi_sacked: 0,
        tcpi_lost: 0,
        tcpi_retrans: 0,
        tcpi_fackets: 0,
        tcpi_last_data_sent: 0,
        tcpi_last_ack_sent: 0,
        tcpi_last_data_recv: 0,
        tcpi_last_ack_recv: 0,
        tcpi_pmtu: 0,
        tcpi_rcv_ssthresh: 0,
        tcpi_rtt: 0,
        tcpi_rttvar: 0,
        tcpi_snd_ssthresh: 0,
        tcpi_snd_cwnd: 0,
        tcpi_advmss: 0,
        tcpi_reordering: 0,
        tcpi_rcv_rtt: 0,
        tcpi_rcv_space: 0,
        tcpi_total_retrans: 0,
    };
    let mut tlen: socklen_t = ::core::mem::size_of::<tcp_info>() as libc::c_ulong
        as socklen_t;
    return (0 as libc::c_int
        == getsockopt(
            fd,
            6 as libc::c_int,
            11 as libc::c_int,
            &mut tcpi as *mut tcp_info as *mut libc::c_void,
            &mut tlen,
        ) && tcpi.tcpi_state as libc::c_int == TCP_CLOSE_WAIT as libc::c_int)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fdevent_set_tcp_nodelay(
    fd: libc::c_int,
    opt: libc::c_int,
) -> libc::c_int {
    return setsockopt(
        fd,
        IPPROTO_TCP as libc::c_int,
        1 as libc::c_int,
        &opt as *const libc::c_int as *const libc::c_void,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn fdevent_set_so_reuseaddr(
    fd: libc::c_int,
    opt: libc::c_int,
) -> libc::c_int {
    return setsockopt(
        fd,
        1 as libc::c_int,
        2 as libc::c_int,
        &opt as *const libc::c_int as *const libc::c_void,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    );
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn fdevent_load_file(
    fn_0: *const libc::c_char,
    mut lim: *mut off_t,
    mut errh: *mut log_error_st,
    mut malloc_fn: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    mut free_fn: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
) -> *mut libc::c_char {
    let mut fd: libc::c_int = 0;
    let mut sz: off_t = 0 as libc::c_int as off_t;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut current_block_30: u64;
    fd = if 0 as libc::c_int
        != strcmp(fn_0, b"/dev/stdin\0" as *const u8 as *const libc::c_char)
    {
        fdevent_open_cloexec(
            fn_0,
            1 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int as mode_t,
        )
    } else {
        0 as libc::c_int
    };
    if !(fd < 0 as libc::c_int) {
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
        if !(0 as libc::c_int != fstat(fd, &mut st)) {
            if st.st_mode & 0o170000 as libc::c_int as libc::c_uint
                == 0o100000 as libc::c_int as libc::c_uint
            {
                sz = st.st_size;
                if ::core::mem::size_of::<off_t>() as libc::c_ulong
                    > ::core::mem::size_of::<size_t>() as libc::c_ulong
                    && sz >= !(0 as libc::c_uint as size_t) as off_t
                    || *lim != 0 as libc::c_int as libc::c_long && sz >= *lim
                {
                    *__errno_location() = 75 as libc::c_int;
                    current_block_30 = 4567019141635105728;
                } else {
                    buf = malloc_fn
                        .expect(
                            "non-null function pointer",
                        )((sz as size_t).wrapping_add(1 as libc::c_int as libc::c_ulong))
                        as *mut libc::c_char;
                    if buf.is_null() {
                        current_block_30 = 4567019141635105728;
                    } else if sz != 0 {
                        let mut rd: ssize_t = 0 as libc::c_int as ssize_t;
                        let mut off: off_t = 0 as libc::c_int as off_t;
                        loop {
                            rd = read(
                                fd,
                                buf.offset(off as isize) as *mut libc::c_void,
                                (sz - off) as size_t,
                            );
                            if !(if rd > 0 as libc::c_int as libc::c_long {
                                off += rd;
                                (off != sz) as libc::c_int
                            } else {
                                (rd < 0 as libc::c_int as libc::c_long
                                    && *__errno_location() == 4 as libc::c_int) as libc::c_int
                            } != 0)
                            {
                                break;
                            }
                        }
                        if off != sz {
                            if rd >= 0 as libc::c_int as libc::c_long {
                                *__errno_location() = 5 as libc::c_int;
                            }
                            current_block_30 = 4567019141635105728;
                        } else {
                            current_block_30 = 980989089337379490;
                        }
                    } else {
                        current_block_30 = 980989089337379490;
                    }
                }
            } else {
                fcntl(
                    fd,
                    4 as libc::c_int,
                    (0 as libc::c_int
                        | (0 as libc::c_int | 0 as libc::c_int | 0o400 as libc::c_int
                            | 0o4000 as libc::c_int)) & !(0o4000 as libc::c_int),
                ) != 0;
                let mut rd_0: ssize_t = 0;
                let mut bsz: off_t = 0 as libc::c_int as off_t;
                if *lim == 0 as libc::c_int as libc::c_long {
                    *lim = (32 as libc::c_int * 1024 as libc::c_int
                        * 1024 as libc::c_int) as off_t;
                }
                loop {
                    if bsz <= sz + 2 as libc::c_int as libc::c_long {
                        if bsz >= *lim {
                            rd_0 = -(1 as libc::c_int) as ssize_t;
                            *__errno_location() = 75 as libc::c_int;
                            break;
                        } else {
                            bsz = if bsz != 0 {
                                bsz << 1 as libc::c_int
                            } else {
                                65536 as libc::c_int as libc::c_long
                            };
                            if bsz > *lim {
                                bsz = *lim;
                            }
                            let mut nbuf: *mut libc::c_char = malloc_fn
                                .expect("non-null function pointer")(bsz as size_t)
                                as *mut libc::c_char;
                            if nbuf.is_null() {
                                rd_0 = -(1 as libc::c_int) as ssize_t;
                                break;
                            } else {
                                if !buf.is_null() {
                                    memcpy(
                                        nbuf as *mut libc::c_void,
                                        buf as *const libc::c_void,
                                        sz as libc::c_ulong,
                                    );
                                    ck_memzero(buf as *mut libc::c_void, sz as size_t);
                                    free_fn
                                        .expect(
                                            "non-null function pointer",
                                        )(buf as *mut libc::c_void);
                                }
                                buf = nbuf;
                            }
                        }
                    }
                    rd_0 = read(
                        fd,
                        buf.offset(sz as isize) as *mut libc::c_void,
                        (bsz - sz - 1 as libc::c_int as libc::c_long) as size_t,
                    );
                    if !(if rd_0 > 0 as libc::c_int as libc::c_long {
                        sz += rd_0;
                        sz
                    } else {
                        (rd_0 < 0 as libc::c_int as libc::c_long
                            && *__errno_location() == 4 as libc::c_int) as libc::c_int
                            as libc::c_long
                    } != 0)
                    {
                        break;
                    }
                }
                if rd_0 != 0 as libc::c_int as libc::c_long {
                    current_block_30 = 4567019141635105728;
                } else {
                    current_block_30 = 980989089337379490;
                }
            }
            match current_block_30 {
                4567019141635105728 => {}
                _ => {
                    *buf.offset(sz as isize) = '\0' as i32 as libc::c_char;
                    *lim = sz;
                    close(fd);
                    return buf;
                }
            }
        }
    }
    let mut errnum: libc::c_int = *__errno_location();
    if !errh.is_null() {
        log_perror(
            errh,
            b"fdevent.c\0" as *const u8 as *const libc::c_char,
            946 as libc::c_int as libc::c_uint,
            b"%s() %s\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 18],
                &[libc::c_char; 18],
            >(b"fdevent_load_file\0"))
                .as_ptr(),
            fn_0,
        );
    }
    if fd >= 0 as libc::c_int {
        close(fd);
    }
    if !buf.is_null() {
        ck_memzero(buf as *mut libc::c_void, sz as size_t);
        free_fn.expect("non-null function pointer")(buf as *mut libc::c_void);
    }
    *lim = 0 as libc::c_int as off_t;
    *__errno_location() = errnum;
    return 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn fdevent_load_file_bytes(
    buf: *mut libc::c_char,
    sz: off_t,
    mut off: off_t,
    fn_0: *const libc::c_char,
    mut errh: *mut log_error_st,
) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    fd = fdevent_open_cloexec(
        fn_0,
        1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int as mode_t,
    );
    if !(fd < 0 as libc::c_int) {
        if !(0 as libc::c_int as libc::c_long != off
            && -(1 as libc::c_int) as off_t == lseek(fd, off, 0 as libc::c_int))
        {
            off = 0 as libc::c_int as off_t;
            let mut rd: ssize_t = 0 as libc::c_int as ssize_t;
            loop {
                rd = read(
                    fd,
                    buf.offset(off as isize) as *mut libc::c_void,
                    (sz - off) as size_t,
                );
                if !(if rd > 0 as libc::c_int as libc::c_long {
                    off += rd;
                    (off != sz) as libc::c_int
                } else {
                    (rd < 0 as libc::c_int as libc::c_long
                        && *__errno_location() == 4 as libc::c_int) as libc::c_int
                } != 0)
                {
                    break;
                }
            }
            if off != sz {
                if rd >= 0 as libc::c_int as libc::c_long {
                    *__errno_location() = 5 as libc::c_int;
                }
            } else {
                close(fd);
                return 0 as libc::c_int;
            }
        }
    }
    let mut errnum: libc::c_int = *__errno_location();
    if !errh.is_null() {
        log_perror(
            errh,
            b"fdevent.c\0" as *const u8 as *const libc::c_char,
            983 as libc::c_int as libc::c_uint,
            b"%s() %s\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 24],
                &[libc::c_char; 24],
            >(b"fdevent_load_file_bytes\0"))
                .as_ptr(),
            fn_0,
        );
    }
    if fd >= 0 as libc::c_int {
        close(fd);
    }
    ck_memzero(buf as *mut libc::c_void, sz as size_t);
    *__errno_location() = errnum;
    return -(1 as libc::c_int);
}
