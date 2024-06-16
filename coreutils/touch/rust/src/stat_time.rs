use ::libc;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_int;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
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
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn stat_time_normalize(
    mut result: libc::c_int,
    mut st: *mut stat,
) -> libc::c_int {
    return result;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn get_stat_birthtime(mut st: *const stat) -> timespec {
    let mut t: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    t = {
        let mut init = timespec {
            tv_sec: -(1 as libc::c_int) as __time_t,
            tv_nsec: -(1 as libc::c_int) as __syscall_slong_t,
        };
        init
    };
    return t;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn get_stat_mtime(mut st: *const stat) -> timespec {
    return (*st).st_mtim;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn get_stat_ctime(mut st: *const stat) -> timespec {
    return (*st).st_ctim;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn get_stat_atime(mut st: *const stat) -> timespec {
    return (*st).st_atim;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn get_stat_birthtime_ns(mut st: *const stat) -> libc::c_long {
    return 0 as libc::c_int as libc::c_long;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn get_stat_mtime_ns(mut st: *const stat) -> libc::c_long {
    return (*st).st_mtim.tv_nsec;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn get_stat_ctime_ns(mut st: *const stat) -> libc::c_long {
    return (*st).st_ctim.tv_nsec;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn get_stat_atime_ns(mut st: *const stat) -> libc::c_long {
    return (*st).st_atim.tv_nsec;
}
