use ::libc;
extern "C" {
    fn current_timespec() -> timespec;
    fn clock_getres(__clock_id: clockid_t, __res: *mut timespec) -> libc::c_int;
}
pub type __time_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type clockid_t = __clockid_t;
pub type C2RustUnnamed = libc::c_uint;
pub const TIMESPEC_HZ: C2RustUnnamed = 1000000000;
unsafe extern "C" fn gcd(mut a: libc::c_long, mut b: libc::c_long) -> libc::c_long {
    while b != 0 as libc::c_int as libc::c_long {
        let mut r: libc::c_long = a % b;
        a = b;
        b = r;
    }
    return a;
}
#[no_mangle]
pub unsafe extern "C" fn gettime_res() -> libc::c_long {
    let mut res: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    clock_getres(0 as libc::c_int, &mut res);
    let mut hz: libc::c_long = TIMESPEC_HZ as libc::c_int as libc::c_long;
    let mut r: libc::c_long = if res.tv_nsec <= 0 as libc::c_int as libc::c_long {
        hz
    } else {
        res.tv_nsec
    };
    let mut earlier: timespec = {
        let mut init = timespec {
            tv_sec: 0,
            tv_nsec: -(1 as libc::c_int) as __syscall_slong_t,
        };
        init
    };
    let mut nsamples: libc::c_int = 32 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while (1 as libc::c_int as libc::c_long) < r && i < nsamples {
        let mut now: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
        loop {
            now = current_timespec();
            if earlier.tv_nsec != now.tv_nsec || earlier.tv_sec != now.tv_sec {
                break;
            }
            i = nsamples;
        }
        earlier = now;
        if (0 as libc::c_int as libc::c_long) < now.tv_nsec {
            r = gcd(r, now.tv_nsec);
        }
        i += 1;
        i;
    }
    return r;
}
