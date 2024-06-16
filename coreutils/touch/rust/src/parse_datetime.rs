use ::libc;
extern "C" {
    pub type tm_zone;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    fn tzalloc(__name: *const libc::c_char) -> timezone_t;
    fn tzfree(__tz: timezone_t);
    fn localtime_rz(
        __tz: timezone_t,
        __timer: *const time_t,
        __result: *mut tm,
    ) -> *mut tm;
    fn mktime_z(__tz: timezone_t, __tm: *mut tm) -> time_t;
    fn gettime(_: *mut timespec);
    fn nstrftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
        __tz: timezone_t,
        __ns: libc::c_int,
    ) -> size_t;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn rpl_vfprintf(
        fp: *mut FILE,
        format: *const libc::c_char,
        args: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn abs(_: libc::c_int) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn gettext(__msgid: *const libc::c_char) -> *mut libc::c_char;
}
pub type __builtin_va_list = __va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list {
    pub __stack: *mut libc::c_void,
    pub __gr_top: *mut libc::c_void,
    pub __vr_top: *mut libc::c_void,
    pub __gr_offs: libc::c_int,
    pub __vr_offs: libc::c_int,
}
pub type size_t = libc::c_ulong;
pub type __intmax_t = libc::c_long;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type ptrdiff_t = libc::c_long;
pub type timezone_t = *mut tm_zone;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct parser_control {
    pub input: *const libc::c_char,
    pub day_ordinal: intmax_t,
    pub day_number: libc::c_int,
    pub local_isdst: libc::c_int,
    pub time_zone: libc::c_int,
    pub meridian: libc::c_int,
    pub year: textint,
    pub month: intmax_t,
    pub day: intmax_t,
    pub hour: intmax_t,
    pub minutes: intmax_t,
    pub seconds: timespec,
    pub rel: relative_time,
    pub timespec_seen: bool,
    pub rels_seen: bool,
    pub dates_seen: idx_t,
    pub days_seen: idx_t,
    pub J_zones_seen: idx_t,
    pub local_zones_seen: idx_t,
    pub dsts_seen: idx_t,
    pub times_seen: idx_t,
    pub zones_seen: idx_t,
    pub year_seen: bool,
    pub parse_datetime_debug: bool,
    pub debug_dates_seen: bool,
    pub debug_days_seen: bool,
    pub debug_local_zones_seen: bool,
    pub debug_times_seen: bool,
    pub debug_zones_seen: bool,
    pub debug_year_seen: bool,
    pub debug_ordinal_day_seen: bool,
    pub local_time_zone_table: [table; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct table {
    pub name: *const libc::c_char,
    pub type_0: libc::c_int,
    pub value: libc::c_int,
}
pub type idx_t = ptrdiff_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct relative_time {
    pub year: intmax_t,
    pub month: intmax_t,
    pub day: intmax_t,
    pub hour: intmax_t,
    pub minutes: intmax_t,
    pub seconds: intmax_t,
    pub ns: libc::c_int,
}
pub type intmax_t = __intmax_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct textint {
    pub negative: bool,
    pub value: intmax_t,
    pub digits: idx_t,
}
pub type va_list = __builtin_va_list;
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
pub const BILLION: C2RustUnnamed_1 = 1000000000;
pub const TM_YEAR_BASE: C2RustUnnamed_2 = 1900;
pub const MERpm: C2RustUnnamed_0 = 1;
pub const MERam: C2RustUnnamed_0 = 0;
pub const MER24: C2RustUnnamed_0 = 2;
pub type yy_state_t = yytype_int8;
pub type yytype_int8 = libc::c_schar;
#[derive(Copy, Clone)]
#[repr(C)]
pub union YYSTYPE {
    pub intval: intmax_t,
    pub textintval: textint,
    pub timespec: timespec,
    pub rel: relative_time,
}
pub type yysymbol_kind_t = libc::c_int;
pub const YYSYMBOL_o_colon_minutes: yysymbol_kind_t = 54;
pub const YYSYMBOL_hybrid: yysymbol_kind_t = 53;
pub const YYSYMBOL_number: yysymbol_kind_t = 52;
pub const YYSYMBOL_unsigned_seconds: yysymbol_kind_t = 51;
pub const YYSYMBOL_signed_seconds: yysymbol_kind_t = 50;
pub const YYSYMBOL_seconds: yysymbol_kind_t = 49;
pub const YYSYMBOL_dayshift: yysymbol_kind_t = 48;
pub const YYSYMBOL_relunit_snumber: yysymbol_kind_t = 47;
pub const YYSYMBOL_relunit: yysymbol_kind_t = 46;
pub const YYSYMBOL_rel: yysymbol_kind_t = 45;
pub const YYSYMBOL_iso_8601_date: yysymbol_kind_t = 44;
pub const YYSYMBOL_date: yysymbol_kind_t = 43;
pub const YYSYMBOL_day: yysymbol_kind_t = 42;
pub const YYSYMBOL_zone: yysymbol_kind_t = 41;
pub const YYSYMBOL_local_zone: yysymbol_kind_t = 40;
pub const YYSYMBOL_zone_offset: yysymbol_kind_t = 39;
pub const YYSYMBOL_o_zone_offset: yysymbol_kind_t = 38;
pub const YYSYMBOL_iso_8601_time: yysymbol_kind_t = 37;
pub const YYSYMBOL_time: yysymbol_kind_t = 36;
pub const YYSYMBOL_iso_8601_datetime: yysymbol_kind_t = 35;
pub const YYSYMBOL_datetime: yysymbol_kind_t = 34;
pub const YYSYMBOL_item: yysymbol_kind_t = 33;
pub const YYSYMBOL_items: yysymbol_kind_t = 32;
pub const YYSYMBOL_timespec: yysymbol_kind_t = 31;
pub const YYSYMBOL_spec: yysymbol_kind_t = 30;
pub const YYSYMBOL_YYACCEPT: yysymbol_kind_t = 29;
pub const YYSYMBOL_28_: yysymbol_kind_t = 28;
pub const YYSYMBOL_27_: yysymbol_kind_t = 27;
pub const YYSYMBOL_26_: yysymbol_kind_t = 26;
pub const YYSYMBOL_25_T_: yysymbol_kind_t = 25;
pub const YYSYMBOL_24_J_: yysymbol_kind_t = 24;
pub const YYSYMBOL_23_: yysymbol_kind_t = 23;
pub const YYSYMBOL_tUDECIMAL_NUMBER: yysymbol_kind_t = 22;
pub const YYSYMBOL_tSDECIMAL_NUMBER: yysymbol_kind_t = 21;
pub const YYSYMBOL_tUNUMBER: yysymbol_kind_t = 20;
pub const YYSYMBOL_tSNUMBER: yysymbol_kind_t = 19;
pub const YYSYMBOL_tZONE: yysymbol_kind_t = 18;
pub const YYSYMBOL_tORDINAL: yysymbol_kind_t = 17;
pub const YYSYMBOL_tMONTH: yysymbol_kind_t = 16;
pub const YYSYMBOL_tMERIDIAN: yysymbol_kind_t = 15;
pub const YYSYMBOL_tLOCAL_ZONE: yysymbol_kind_t = 14;
pub const YYSYMBOL_tDAYZONE: yysymbol_kind_t = 13;
pub const YYSYMBOL_tDAY: yysymbol_kind_t = 12;
pub const YYSYMBOL_tDAY_SHIFT: yysymbol_kind_t = 11;
pub const YYSYMBOL_tDAY_UNIT: yysymbol_kind_t = 10;
pub const YYSYMBOL_tSEC_UNIT: yysymbol_kind_t = 9;
pub const YYSYMBOL_tMINUTE_UNIT: yysymbol_kind_t = 8;
pub const YYSYMBOL_tHOUR_UNIT: yysymbol_kind_t = 7;
pub const YYSYMBOL_tMONTH_UNIT: yysymbol_kind_t = 6;
pub const YYSYMBOL_tYEAR_UNIT: yysymbol_kind_t = 5;
pub const YYSYMBOL_tDST: yysymbol_kind_t = 4;
pub const YYSYMBOL_tAGO: yysymbol_kind_t = 3;
pub const YYSYMBOL_YYUNDEF: yysymbol_kind_t = 2;
pub const YYSYMBOL_YYerror: yysymbol_kind_t = 1;
pub const YYSYMBOL_YYEOF: yysymbol_kind_t = 0;
pub const YYSYMBOL_YYEMPTY: yysymbol_kind_t = -2;
pub const YYEMPTY: yytokentype = -2;
pub type yy_state_fast_t = libc::c_int;
pub const YYEOF: yytokentype = 0;
pub const YYUNDEF: yytokentype = 257;
pub const YYerror: yytokentype = 256;
pub const tDAYZONE: yytokentype = 268;
pub const tZONE: yytokentype = 273;
pub const tAGO: yytokentype = 258;
pub const tORDINAL: yytokentype = 272;
pub const tDAY_SHIFT: yytokentype = 266;
pub const tSEC_UNIT: yytokentype = 264;
pub const tMINUTE_UNIT: yytokentype = 263;
pub const tHOUR_UNIT: yytokentype = 262;
pub const tDAY_UNIT: yytokentype = 265;
pub const tMONTH_UNIT: yytokentype = 261;
pub const tYEAR_UNIT: yytokentype = 260;
pub const tDST: yytokentype = 259;
pub const tDAY: yytokentype = 267;
pub const tMONTH: yytokentype = 271;
pub const tMERIDIAN: yytokentype = 270;
pub const tUNUMBER: yytokentype = 275;
pub const tSNUMBER: yytokentype = 274;
pub const tUDECIMAL_NUMBER: yytokentype = 277;
pub const tSDECIMAL_NUMBER: yytokentype = 276;
pub const LOG10_BILLION: C2RustUnnamed_1 = 9;
#[derive(Copy, Clone)]
#[repr(C)]
pub union yyalloc {
    pub yyss_alloc: yy_state_t,
    pub yyvs_alloc: YYSTYPE,
}
pub const tLOCAL_ZONE: yytokentype = 269;
pub const TZBUFSIZE: C2RustUnnamed = 100;
pub type C2RustUnnamed = libc::c_uint;
pub type C2RustUnnamed_0 = libc::c_uint;
pub type C2RustUnnamed_1 = libc::c_uint;
pub type yytokentype = libc::c_int;
pub type C2RustUnnamed_2 = libc::c_uint;
#[inline]
unsafe extern "C" fn c_isalpha(mut c: libc::c_int) -> bool {
    match c {
        97 | 98 | 99 | 100 | 101 | 102 | 103 | 104 | 105 | 106 | 107 | 108 | 109 | 110
        | 111 | 112 | 113 | 114 | 115 | 116 | 117 | 118 | 119 | 120 | 121 | 122 | 65 | 66
        | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82
        | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
#[inline]
unsafe extern "C" fn c_isdigit(mut c: libc::c_int) -> bool {
    match c {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
#[inline]
unsafe extern "C" fn c_isspace(mut c: libc::c_int) -> bool {
    match c {
        32 | 9 | 10 | 11 | 12 | 13 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
#[inline]
unsafe extern "C" fn c_toupper(mut c: libc::c_int) -> libc::c_int {
    match c {
        97 | 98 | 99 | 100 | 101 | 102 | 103 | 104 | 105 | 106 | 107 | 108 | 109 | 110
        | 111 | 112 | 113 | 114 | 115 | 116 | 117 | 118 | 119 | 120 | 121 | 122 => {
            return c - 'a' as i32 + 'A' as i32;
        }
        _ => return c,
    };
}
unsafe extern "C" fn time_overflow(mut n: intmax_t) -> bool {
    return !((if !((0 as libc::c_int as time_t) < -(1 as libc::c_int) as time_t) {
        (!(if (0 as libc::c_int as time_t) < -(1 as libc::c_int) as time_t {
            -(1 as libc::c_int) as time_t
        } else {
            (((1 as libc::c_int as time_t)
                << (::core::mem::size_of::<time_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                - 1 as libc::c_int as libc::c_long) * 2 as libc::c_int as libc::c_long
                + 1 as libc::c_int as libc::c_long
        }) <= n) as libc::c_int
    } else {
        (0 as libc::c_int as libc::c_long <= n) as libc::c_int
    }) != 0
        && n
            <= (if (0 as libc::c_int as time_t) < -(1 as libc::c_int) as time_t {
                -(1 as libc::c_int) as time_t
            } else {
                (((1 as libc::c_int as time_t)
                    << (::core::mem::size_of::<time_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int as libc::c_long)
                    * 2 as libc::c_int as libc::c_long + 1 as libc::c_int as libc::c_long
            }));
}
unsafe extern "C" fn to_uchar(mut ch: libc::c_char) -> libc::c_uchar {
    return ch as libc::c_uchar;
}
unsafe extern "C" fn dbg_printf(mut msg: *const libc::c_char, mut args: ...) {
    let mut args_0: ::core::ffi::VaListImpl;
    fputs(b"date: \0" as *const u8 as *const libc::c_char, stderr);
    args_0 = args.clone();
    rpl_vfprintf(stderr, msg, args_0.as_va_list());
}
unsafe extern "C" fn debugging(mut pc: *const parser_control) -> bool {
    return (*pc).parse_datetime_debug;
}
unsafe extern "C" fn digits_to_date_time(
    mut pc: *mut parser_control,
    mut text_int: textint,
) {
    if (*pc).dates_seen != 0 && (*pc).year.digits == 0 && !(*pc).rels_seen
        && ((*pc).times_seen != 0
            || (2 as libc::c_int as libc::c_long) < text_int.digits)
    {
        (*pc).year_seen = 1 as libc::c_int != 0;
        (*pc).year = text_int;
    } else if (4 as libc::c_int as libc::c_long) < text_int.digits {
        (*pc).dates_seen += 1;
        (*pc).dates_seen;
        (*pc).day = text_int.value % 100 as libc::c_int as libc::c_long;
        (*pc)
            .month = text_int.value / 100 as libc::c_int as libc::c_long
            % 100 as libc::c_int as libc::c_long;
        (*pc).year.value = text_int.value / 10000 as libc::c_int as libc::c_long;
        (*pc).year.digits = text_int.digits - 4 as libc::c_int as libc::c_long;
    } else {
        (*pc).times_seen += 1;
        (*pc).times_seen;
        if text_int.digits <= 2 as libc::c_int as libc::c_long {
            (*pc).hour = text_int.value;
            (*pc).minutes = 0 as libc::c_int as intmax_t;
        } else {
            (*pc).hour = text_int.value / 100 as libc::c_int as libc::c_long;
            (*pc).minutes = text_int.value % 100 as libc::c_int as libc::c_long;
        }
        (*pc)
            .seconds = {
            let mut init = timespec {
                tv_sec: 0 as libc::c_int as __time_t,
                tv_nsec: 0,
            };
            init
        };
        (*pc).meridian = MER24 as libc::c_int;
    };
}
unsafe extern "C" fn apply_relative_time(
    mut pc: *mut parser_control,
    mut rel: relative_time,
    mut factor: libc::c_int,
) -> bool {
    if if factor < 0 as libc::c_int {
        let (fresh0, fresh1) = ((*pc).rel.ns).overflowing_sub(rel.ns);
        *(&mut (*pc).rel.ns as *mut libc::c_int) = fresh0;
        let (fresh2, fresh3) = ((*pc).rel.seconds).overflowing_sub(rel.seconds);
        *(&mut (*pc).rel.seconds as *mut intmax_t) = fresh2;
        let (fresh4, fresh5) = ((*pc).rel.minutes).overflowing_sub(rel.minutes);
        *(&mut (*pc).rel.minutes as *mut intmax_t) = fresh4;
        let (fresh6, fresh7) = ((*pc).rel.hour).overflowing_sub(rel.hour);
        *(&mut (*pc).rel.hour as *mut intmax_t) = fresh6;
        let (fresh8, fresh9) = ((*pc).rel.day).overflowing_sub(rel.day);
        *(&mut (*pc).rel.day as *mut intmax_t) = fresh8;
        let (fresh10, fresh11) = ((*pc).rel.month).overflowing_sub(rel.month);
        *(&mut (*pc).rel.month as *mut intmax_t) = fresh10;
        let (fresh12, fresh13) = ((*pc).rel.year).overflowing_sub(rel.year);
        *(&mut (*pc).rel.year as *mut intmax_t) = fresh12;
        fresh1 as libc::c_int | fresh3 as libc::c_int | fresh5 as libc::c_int
            | fresh7 as libc::c_int | fresh9 as libc::c_int | fresh11 as libc::c_int
            | fresh13 as libc::c_int
    } else {
        let (fresh14, fresh15) = ((*pc).rel.ns).overflowing_add(rel.ns);
        *(&mut (*pc).rel.ns as *mut libc::c_int) = fresh14;
        let (fresh16, fresh17) = ((*pc).rel.seconds).overflowing_add(rel.seconds);
        *(&mut (*pc).rel.seconds as *mut intmax_t) = fresh16;
        let (fresh18, fresh19) = ((*pc).rel.minutes).overflowing_add(rel.minutes);
        *(&mut (*pc).rel.minutes as *mut intmax_t) = fresh18;
        let (fresh20, fresh21) = ((*pc).rel.hour).overflowing_add(rel.hour);
        *(&mut (*pc).rel.hour as *mut intmax_t) = fresh20;
        let (fresh22, fresh23) = ((*pc).rel.day).overflowing_add(rel.day);
        *(&mut (*pc).rel.day as *mut intmax_t) = fresh22;
        let (fresh24, fresh25) = ((*pc).rel.month).overflowing_add(rel.month);
        *(&mut (*pc).rel.month as *mut intmax_t) = fresh24;
        let (fresh26, fresh27) = ((*pc).rel.year).overflowing_add(rel.year);
        *(&mut (*pc).rel.year as *mut intmax_t) = fresh26;
        fresh15 as libc::c_int | fresh17 as libc::c_int | fresh19 as libc::c_int
            | fresh21 as libc::c_int | fresh23 as libc::c_int | fresh25 as libc::c_int
            | fresh27 as libc::c_int
    } != 0
    {
        return 0 as libc::c_int != 0;
    }
    (*pc).rels_seen = 1 as libc::c_int != 0;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn set_hhmmss(
    mut pc: *mut parser_control,
    mut hour: intmax_t,
    mut minutes: intmax_t,
    mut sec: time_t,
    mut nsec: libc::c_int,
) {
    (*pc).hour = hour;
    (*pc).minutes = minutes;
    (*pc)
        .seconds = {
        let mut init = timespec {
            tv_sec: sec,
            tv_nsec: nsec as __syscall_slong_t,
        };
        init
    };
}
unsafe extern "C" fn str_days(
    mut pc: *mut parser_control,
    mut buffer: *mut libc::c_char,
    mut n: libc::c_int,
) -> *const libc::c_char {
    static mut ordinal_values: [[libc::c_char; 11]; 14] = unsafe {
        [
            *::core::mem::transmute::<
                &[u8; 11],
                &[libc::c_char; 11],
            >(b"last\0\0\0\0\0\0\0"),
            *::core::mem::transmute::<
                &[u8; 11],
                &[libc::c_char; 11],
            >(b"this\0\0\0\0\0\0\0"),
            *::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"next/first\0"),
            *::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"(SECOND)\0\0\0"),
            *::core::mem::transmute::<
                &[u8; 11],
                &[libc::c_char; 11],
            >(b"third\0\0\0\0\0\0"),
            *::core::mem::transmute::<
                &[u8; 11],
                &[libc::c_char; 11],
            >(b"fourth\0\0\0\0\0"),
            *::core::mem::transmute::<
                &[u8; 11],
                &[libc::c_char; 11],
            >(b"fifth\0\0\0\0\0\0"),
            *::core::mem::transmute::<
                &[u8; 11],
                &[libc::c_char; 11],
            >(b"sixth\0\0\0\0\0\0"),
            *::core::mem::transmute::<
                &[u8; 11],
                &[libc::c_char; 11],
            >(b"seventh\0\0\0\0"),
            *::core::mem::transmute::<
                &[u8; 11],
                &[libc::c_char; 11],
            >(b"eight\0\0\0\0\0\0"),
            *::core::mem::transmute::<
                &[u8; 11],
                &[libc::c_char; 11],
            >(b"ninth\0\0\0\0\0\0"),
            *::core::mem::transmute::<
                &[u8; 11],
                &[libc::c_char; 11],
            >(b"tenth\0\0\0\0\0\0"),
            *::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"eleventh\0\0\0"),
            *::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"twelfth\0\0\0\0"),
        ]
    };
    static mut days_values: [[libc::c_char; 4]; 7] = unsafe {
        [
            *::core::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"Sun\0"),
            *::core::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"Mon\0"),
            *::core::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"Tue\0"),
            *::core::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"Wed\0"),
            *::core::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"Thu\0"),
            *::core::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"Fri\0"),
            *::core::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"Sat\0"),
        ]
    };
    let mut len: libc::c_int = 0;
    if (*pc).debug_ordinal_day_seen {
        len = if -(1 as libc::c_int) as libc::c_long <= (*pc).day_ordinal
            && (*pc).day_ordinal <= 12 as libc::c_int as libc::c_long
        {
            snprintf(
                buffer,
                n as libc::c_ulong,
                b"%s\0" as *const u8 as *const libc::c_char,
                (ordinal_values[((*pc).day_ordinal + 1 as libc::c_int as libc::c_long)
                    as usize])
                    .as_ptr(),
            )
        } else {
            snprintf(
                buffer,
                n as libc::c_ulong,
                b"%ld\0" as *const u8 as *const libc::c_char,
                (*pc).day_ordinal,
            )
        };
    } else {
        *buffer.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        len = 0 as libc::c_int;
    }
    if 0 as libc::c_int <= (*pc).day_number && (*pc).day_number <= 6 as libc::c_int
        && 0 as libc::c_int <= len && len < n
    {
        snprintf(
            buffer.offset(len as isize),
            (n - len) as libc::c_ulong,
            &*(b" %s\0" as *const u8 as *const libc::c_char)
                .offset((len == 0 as libc::c_int) as libc::c_int as isize)
                as *const libc::c_char,
            (days_values[(*pc).day_number as usize]).as_ptr(),
        );
    }
    return buffer;
}
unsafe extern "C" fn time_zone_str(
    mut time_zone: libc::c_int,
    mut time_zone_buf: *mut libc::c_char,
) -> *const libc::c_char {
    let mut p: *mut libc::c_char = time_zone_buf;
    let mut sign: libc::c_char = (if time_zone < 0 as libc::c_int {
        '-' as i32
    } else {
        '+' as i32
    }) as libc::c_char;
    let mut hour: libc::c_int = abs(time_zone / (60 as libc::c_int * 60 as libc::c_int));
    p = p
        .offset(
            sprintf(
                time_zone_buf,
                b"%c%02d\0" as *const u8 as *const libc::c_char,
                sign as libc::c_int,
                hour,
            ) as isize,
        );
    let mut offset_from_hour: libc::c_int = abs(
        time_zone % (60 as libc::c_int * 60 as libc::c_int),
    );
    if offset_from_hour != 0 as libc::c_int {
        let mut mm: libc::c_int = offset_from_hour / 60 as libc::c_int;
        let mut ss: libc::c_int = offset_from_hour % 60 as libc::c_int;
        let fresh28 = p;
        p = p.offset(1);
        *fresh28 = ':' as i32 as libc::c_char;
        let fresh29 = p;
        p = p.offset(1);
        *fresh29 = ('0' as i32 + mm / 10 as libc::c_int) as libc::c_char;
        let fresh30 = p;
        p = p.offset(1);
        *fresh30 = ('0' as i32 + mm % 10 as libc::c_int) as libc::c_char;
        if ss != 0 {
            let fresh31 = p;
            p = p.offset(1);
            *fresh31 = ':' as i32 as libc::c_char;
            let fresh32 = p;
            p = p.offset(1);
            *fresh32 = ('0' as i32 + ss / 10 as libc::c_int) as libc::c_char;
            let fresh33 = p;
            p = p.offset(1);
            *fresh33 = ('0' as i32 + ss % 10 as libc::c_int) as libc::c_char;
        }
        *p = '\0' as i32 as libc::c_char;
    }
    return time_zone_buf as *const libc::c_char;
}
unsafe extern "C" fn debug_print_current_time(
    mut item: *const libc::c_char,
    mut pc: *mut parser_control,
) {
    let mut space: bool = 0 as libc::c_int != 0;
    if !debugging(pc) {
        return;
    }
    dbg_printf(gettext(b"parsed %s part: \0" as *const u8 as *const libc::c_char), item);
    if (*pc).dates_seen != 0 && !(*pc).debug_dates_seen {
        fprintf(
            stderr,
            b"(Y-M-D) %04ld-%02ld-%02ld\0" as *const u8 as *const libc::c_char,
            (*pc).year.value,
            (*pc).month,
            (*pc).day,
        );
        (*pc).debug_dates_seen = 1 as libc::c_int != 0;
        space = 1 as libc::c_int != 0;
    }
    if (*pc).year_seen as libc::c_int != (*pc).debug_year_seen as libc::c_int {
        if space {
            fputc(' ' as i32, stderr);
        }
        fprintf(
            stderr,
            gettext(b"year: %04ld\0" as *const u8 as *const libc::c_char),
            (*pc).year.value,
        );
        (*pc).debug_year_seen = (*pc).year_seen;
        space = 1 as libc::c_int != 0;
    }
    if (*pc).times_seen != 0 && !(*pc).debug_times_seen {
        let mut sec: intmax_t = (*pc).seconds.tv_sec;
        fprintf(
            stderr,
            &*(b" %02ld:%02ld:%02ld\0" as *const u8 as *const libc::c_char)
                .offset(!space as libc::c_int as isize) as *const libc::c_char,
            (*pc).hour,
            (*pc).minutes,
            sec,
        );
        if (*pc).seconds.tv_nsec != 0 as libc::c_int as libc::c_long {
            let mut nsec: libc::c_int = (*pc).seconds.tv_nsec as libc::c_int;
            fprintf(stderr, b".%09d\0" as *const u8 as *const libc::c_char, nsec);
        }
        if (*pc).meridian == MERpm as libc::c_int {
            fputs(b"pm\0" as *const u8 as *const libc::c_char, stderr);
        }
        (*pc).debug_times_seen = 1 as libc::c_int != 0;
        space = 1 as libc::c_int != 0;
    }
    if (*pc).days_seen != 0 && !(*pc).debug_days_seen {
        if space {
            fputc(' ' as i32, stderr);
        }
        let mut tmp: [libc::c_char; 100] = [0; 100];
        fprintf(
            stderr,
            gettext(
                b"%s (day ordinal=%ld number=%d)\0" as *const u8 as *const libc::c_char,
            ),
            str_days(
                pc,
                tmp.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong
                    as libc::c_int,
            ),
            (*pc).day_ordinal,
            (*pc).day_number,
        );
        (*pc).debug_days_seen = 1 as libc::c_int != 0;
        space = 1 as libc::c_int != 0;
    }
    if (*pc).local_zones_seen != 0 && !(*pc).debug_local_zones_seen {
        fprintf(
            stderr,
            &*(b" isdst=%d%s\0" as *const u8 as *const libc::c_char)
                .offset(!space as libc::c_int as isize) as *const libc::c_char,
            (*pc).local_isdst,
            if (*pc).dsts_seen != 0 {
                b" DST\0" as *const u8 as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        );
        (*pc).debug_local_zones_seen = 1 as libc::c_int != 0;
        space = 1 as libc::c_int != 0;
    }
    if (*pc).zones_seen != 0 && !(*pc).debug_zones_seen {
        let mut time_zone_buf: [libc::c_char; 27] = [0; 27];
        fprintf(
            stderr,
            &*(b" UTC%s\0" as *const u8 as *const libc::c_char)
                .offset(!space as libc::c_int as isize) as *const libc::c_char,
            time_zone_str((*pc).time_zone, time_zone_buf.as_mut_ptr()),
        );
        (*pc).debug_zones_seen = 1 as libc::c_int != 0;
        space = 1 as libc::c_int != 0;
    }
    if (*pc).timespec_seen {
        let mut sec_0: intmax_t = (*pc).seconds.tv_sec;
        if space {
            fputc(' ' as i32, stderr);
        }
        fprintf(
            stderr,
            gettext(b"number of seconds: %ld\0" as *const u8 as *const libc::c_char),
            sec_0,
        );
    }
    fputc('\n' as i32, stderr);
}
unsafe extern "C" fn print_rel_part(
    mut space: bool,
    mut val: intmax_t,
    mut name: *const libc::c_char,
) -> bool {
    if val == 0 as libc::c_int as libc::c_long {
        return space;
    }
    fprintf(
        stderr,
        &*(b" %+ld %s\0" as *const u8 as *const libc::c_char)
            .offset(!space as libc::c_int as isize) as *const libc::c_char,
        val,
        name,
    );
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn debug_print_relative_time(
    mut item: *const libc::c_char,
    mut pc: *const parser_control,
) {
    let mut space: bool = 0 as libc::c_int != 0;
    if !debugging(pc) {
        return;
    }
    dbg_printf(gettext(b"parsed %s part: \0" as *const u8 as *const libc::c_char), item);
    if (*pc).rel.year == 0 as libc::c_int as libc::c_long
        && (*pc).rel.month == 0 as libc::c_int as libc::c_long
        && (*pc).rel.day == 0 as libc::c_int as libc::c_long
        && (*pc).rel.hour == 0 as libc::c_int as libc::c_long
        && (*pc).rel.minutes == 0 as libc::c_int as libc::c_long
        && (*pc).rel.seconds == 0 as libc::c_int as libc::c_long
        && (*pc).rel.ns == 0 as libc::c_int
    {
        fputs(
            gettext(b"today/this/now\n\0" as *const u8 as *const libc::c_char),
            stderr,
        );
        return;
    }
    space = print_rel_part(
        space,
        (*pc).rel.year,
        b"year(s)\0" as *const u8 as *const libc::c_char,
    );
    space = print_rel_part(
        space,
        (*pc).rel.month,
        b"month(s)\0" as *const u8 as *const libc::c_char,
    );
    space = print_rel_part(
        space,
        (*pc).rel.day,
        b"day(s)\0" as *const u8 as *const libc::c_char,
    );
    space = print_rel_part(
        space,
        (*pc).rel.hour,
        b"hour(s)\0" as *const u8 as *const libc::c_char,
    );
    space = print_rel_part(
        space,
        (*pc).rel.minutes,
        b"minutes\0" as *const u8 as *const libc::c_char,
    );
    space = print_rel_part(
        space,
        (*pc).rel.seconds,
        b"seconds\0" as *const u8 as *const libc::c_char,
    );
    print_rel_part(
        space,
        (*pc).rel.ns as intmax_t,
        b"nanoseconds\0" as *const u8 as *const libc::c_char,
    );
    fputc('\n' as i32, stderr);
}
static mut meridian_table: [table; 5] = [
    {
        let mut init = table {
            name: b"AM\0" as *const u8 as *const libc::c_char,
            type_0: tMERIDIAN as libc::c_int,
            value: MERam as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"A.M.\0" as *const u8 as *const libc::c_char,
            type_0: tMERIDIAN as libc::c_int,
            value: MERam as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"PM\0" as *const u8 as *const libc::c_char,
            type_0: tMERIDIAN as libc::c_int,
            value: MERpm as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"P.M.\0" as *const u8 as *const libc::c_char,
            type_0: tMERIDIAN as libc::c_int,
            value: MERpm as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: 0 as *const libc::c_char,
            type_0: 0 as libc::c_int,
            value: 0 as libc::c_int,
        };
        init
    },
];
static mut dst_table: [table; 1] = [
    {
        let mut init = table {
            name: b"DST\0" as *const u8 as *const libc::c_char,
            type_0: tDST as libc::c_int,
            value: 0 as libc::c_int,
        };
        init
    },
];
static mut month_and_day_table: [table; 25] = [
    {
        let mut init = table {
            name: b"JANUARY\0" as *const u8 as *const libc::c_char,
            type_0: tMONTH as libc::c_int,
            value: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"FEBRUARY\0" as *const u8 as *const libc::c_char,
            type_0: tMONTH as libc::c_int,
            value: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"MARCH\0" as *const u8 as *const libc::c_char,
            type_0: tMONTH as libc::c_int,
            value: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"APRIL\0" as *const u8 as *const libc::c_char,
            type_0: tMONTH as libc::c_int,
            value: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"MAY\0" as *const u8 as *const libc::c_char,
            type_0: tMONTH as libc::c_int,
            value: 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"JUNE\0" as *const u8 as *const libc::c_char,
            type_0: tMONTH as libc::c_int,
            value: 6 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"JULY\0" as *const u8 as *const libc::c_char,
            type_0: tMONTH as libc::c_int,
            value: 7 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"AUGUST\0" as *const u8 as *const libc::c_char,
            type_0: tMONTH as libc::c_int,
            value: 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"SEPTEMBER\0" as *const u8 as *const libc::c_char,
            type_0: tMONTH as libc::c_int,
            value: 9 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"SEPT\0" as *const u8 as *const libc::c_char,
            type_0: tMONTH as libc::c_int,
            value: 9 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"OCTOBER\0" as *const u8 as *const libc::c_char,
            type_0: tMONTH as libc::c_int,
            value: 10 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"NOVEMBER\0" as *const u8 as *const libc::c_char,
            type_0: tMONTH as libc::c_int,
            value: 11 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"DECEMBER\0" as *const u8 as *const libc::c_char,
            type_0: tMONTH as libc::c_int,
            value: 12 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"SUNDAY\0" as *const u8 as *const libc::c_char,
            type_0: tDAY as libc::c_int,
            value: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"MONDAY\0" as *const u8 as *const libc::c_char,
            type_0: tDAY as libc::c_int,
            value: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"TUESDAY\0" as *const u8 as *const libc::c_char,
            type_0: tDAY as libc::c_int,
            value: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"TUES\0" as *const u8 as *const libc::c_char,
            type_0: tDAY as libc::c_int,
            value: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"WEDNESDAY\0" as *const u8 as *const libc::c_char,
            type_0: tDAY as libc::c_int,
            value: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"WEDNES\0" as *const u8 as *const libc::c_char,
            type_0: tDAY as libc::c_int,
            value: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"THURSDAY\0" as *const u8 as *const libc::c_char,
            type_0: tDAY as libc::c_int,
            value: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"THUR\0" as *const u8 as *const libc::c_char,
            type_0: tDAY as libc::c_int,
            value: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"THURS\0" as *const u8 as *const libc::c_char,
            type_0: tDAY as libc::c_int,
            value: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"FRIDAY\0" as *const u8 as *const libc::c_char,
            type_0: tDAY as libc::c_int,
            value: 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"SATURDAY\0" as *const u8 as *const libc::c_char,
            type_0: tDAY as libc::c_int,
            value: 6 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: 0 as *const libc::c_char,
            type_0: 0 as libc::c_int,
            value: 0 as libc::c_int,
        };
        init
    },
];
static mut time_units_table: [table; 11] = [
    {
        let mut init = table {
            name: b"YEAR\0" as *const u8 as *const libc::c_char,
            type_0: tYEAR_UNIT as libc::c_int,
            value: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"MONTH\0" as *const u8 as *const libc::c_char,
            type_0: tMONTH_UNIT as libc::c_int,
            value: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"FORTNIGHT\0" as *const u8 as *const libc::c_char,
            type_0: tDAY_UNIT as libc::c_int,
            value: 14 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"WEEK\0" as *const u8 as *const libc::c_char,
            type_0: tDAY_UNIT as libc::c_int,
            value: 7 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"DAY\0" as *const u8 as *const libc::c_char,
            type_0: tDAY_UNIT as libc::c_int,
            value: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"HOUR\0" as *const u8 as *const libc::c_char,
            type_0: tHOUR_UNIT as libc::c_int,
            value: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"MINUTE\0" as *const u8 as *const libc::c_char,
            type_0: tMINUTE_UNIT as libc::c_int,
            value: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"MIN\0" as *const u8 as *const libc::c_char,
            type_0: tMINUTE_UNIT as libc::c_int,
            value: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"SECOND\0" as *const u8 as *const libc::c_char,
            type_0: tSEC_UNIT as libc::c_int,
            value: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"SEC\0" as *const u8 as *const libc::c_char,
            type_0: tSEC_UNIT as libc::c_int,
            value: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: 0 as *const libc::c_char,
            type_0: 0 as libc::c_int,
            value: 0 as libc::c_int,
        };
        init
    },
];
static mut relative_time_table: [table; 21] = [
    {
        let mut init = table {
            name: b"TOMORROW\0" as *const u8 as *const libc::c_char,
            type_0: tDAY_SHIFT as libc::c_int,
            value: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"YESTERDAY\0" as *const u8 as *const libc::c_char,
            type_0: tDAY_SHIFT as libc::c_int,
            value: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = table {
            name: b"TODAY\0" as *const u8 as *const libc::c_char,
            type_0: tDAY_SHIFT as libc::c_int,
            value: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"NOW\0" as *const u8 as *const libc::c_char,
            type_0: tDAY_SHIFT as libc::c_int,
            value: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"LAST\0" as *const u8 as *const libc::c_char,
            type_0: tORDINAL as libc::c_int,
            value: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = table {
            name: b"THIS\0" as *const u8 as *const libc::c_char,
            type_0: tORDINAL as libc::c_int,
            value: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"NEXT\0" as *const u8 as *const libc::c_char,
            type_0: tORDINAL as libc::c_int,
            value: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"FIRST\0" as *const u8 as *const libc::c_char,
            type_0: tORDINAL as libc::c_int,
            value: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"THIRD\0" as *const u8 as *const libc::c_char,
            type_0: tORDINAL as libc::c_int,
            value: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"FOURTH\0" as *const u8 as *const libc::c_char,
            type_0: tORDINAL as libc::c_int,
            value: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"FIFTH\0" as *const u8 as *const libc::c_char,
            type_0: tORDINAL as libc::c_int,
            value: 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"SIXTH\0" as *const u8 as *const libc::c_char,
            type_0: tORDINAL as libc::c_int,
            value: 6 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"SEVENTH\0" as *const u8 as *const libc::c_char,
            type_0: tORDINAL as libc::c_int,
            value: 7 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"EIGHTH\0" as *const u8 as *const libc::c_char,
            type_0: tORDINAL as libc::c_int,
            value: 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"NINTH\0" as *const u8 as *const libc::c_char,
            type_0: tORDINAL as libc::c_int,
            value: 9 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"TENTH\0" as *const u8 as *const libc::c_char,
            type_0: tORDINAL as libc::c_int,
            value: 10 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"ELEVENTH\0" as *const u8 as *const libc::c_char,
            type_0: tORDINAL as libc::c_int,
            value: 11 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"TWELFTH\0" as *const u8 as *const libc::c_char,
            type_0: tORDINAL as libc::c_int,
            value: 12 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"AGO\0" as *const u8 as *const libc::c_char,
            type_0: tAGO as libc::c_int,
            value: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = table {
            name: b"HENCE\0" as *const u8 as *const libc::c_char,
            type_0: tAGO as libc::c_int,
            value: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: 0 as *const libc::c_char,
            type_0: 0 as libc::c_int,
            value: 0 as libc::c_int,
        };
        init
    },
];
static mut universal_time_zone_table: [table; 4] = [
    {
        let mut init = table {
            name: b"GMT\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 60 as libc::c_int * 60 as libc::c_int * 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"UT\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 60 as libc::c_int * 60 as libc::c_int * 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"UTC\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 60 as libc::c_int * 60 as libc::c_int * 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: 0 as *const libc::c_char,
            type_0: 0 as libc::c_int,
            value: 0 as libc::c_int,
        };
        init
    },
];
static mut time_zone_table: [table; 48] = [
    {
        let mut init = table {
            name: b"WET\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 60 as libc::c_int * 60 as libc::c_int * 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"WEST\0" as *const u8 as *const libc::c_char,
            type_0: tDAYZONE as libc::c_int,
            value: 60 as libc::c_int * 60 as libc::c_int * 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"BST\0" as *const u8 as *const libc::c_char,
            type_0: tDAYZONE as libc::c_int,
            value: 60 as libc::c_int * 60 as libc::c_int * 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"ART\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(60 as libc::c_int * 60 as libc::c_int * 3 as libc::c_int),
        };
        init
    },
    {
        let mut init = table {
            name: b"BRT\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(60 as libc::c_int * 60 as libc::c_int * 3 as libc::c_int),
        };
        init
    },
    {
        let mut init = table {
            name: b"BRST\0" as *const u8 as *const libc::c_char,
            type_0: tDAYZONE as libc::c_int,
            value: -(60 as libc::c_int * 60 as libc::c_int * 3 as libc::c_int),
        };
        init
    },
    {
        let mut init = table {
            name: b"NST\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(60 as libc::c_int * 60 as libc::c_int * 3 as libc::c_int
                + 30 as libc::c_int * 60 as libc::c_int),
        };
        init
    },
    {
        let mut init = table {
            name: b"NDT\0" as *const u8 as *const libc::c_char,
            type_0: tDAYZONE as libc::c_int,
            value: -(60 as libc::c_int * 60 as libc::c_int * 3 as libc::c_int
                + 30 as libc::c_int * 60 as libc::c_int),
        };
        init
    },
    {
        let mut init = table {
            name: b"AST\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(60 as libc::c_int * 60 as libc::c_int * 4 as libc::c_int),
        };
        init
    },
    {
        let mut init = table {
            name: b"ADT\0" as *const u8 as *const libc::c_char,
            type_0: tDAYZONE as libc::c_int,
            value: -(60 as libc::c_int * 60 as libc::c_int * 4 as libc::c_int),
        };
        init
    },
    {
        let mut init = table {
            name: b"CLT\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(60 as libc::c_int * 60 as libc::c_int * 4 as libc::c_int),
        };
        init
    },
    {
        let mut init = table {
            name: b"CLST\0" as *const u8 as *const libc::c_char,
            type_0: tDAYZONE as libc::c_int,
            value: -(60 as libc::c_int * 60 as libc::c_int * 4 as libc::c_int),
        };
        init
    },
    {
        let mut init = table {
            name: b"EST\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(60 as libc::c_int * 60 as libc::c_int * 5 as libc::c_int),
        };
        init
    },
    {
        let mut init = table {
            name: b"EDT\0" as *const u8 as *const libc::c_char,
            type_0: tDAYZONE as libc::c_int,
            value: -(60 as libc::c_int * 60 as libc::c_int * 5 as libc::c_int),
        };
        init
    },
    {
        let mut init = table {
            name: b"CST\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(60 as libc::c_int * 60 as libc::c_int * 6 as libc::c_int),
        };
        init
    },
    {
        let mut init = table {
            name: b"CDT\0" as *const u8 as *const libc::c_char,
            type_0: tDAYZONE as libc::c_int,
            value: -(60 as libc::c_int * 60 as libc::c_int * 6 as libc::c_int),
        };
        init
    },
    {
        let mut init = table {
            name: b"MST\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(60 as libc::c_int * 60 as libc::c_int * 7 as libc::c_int),
        };
        init
    },
    {
        let mut init = table {
            name: b"MDT\0" as *const u8 as *const libc::c_char,
            type_0: tDAYZONE as libc::c_int,
            value: -(60 as libc::c_int * 60 as libc::c_int * 7 as libc::c_int),
        };
        init
    },
    {
        let mut init = table {
            name: b"PST\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(60 as libc::c_int * 60 as libc::c_int * 8 as libc::c_int),
        };
        init
    },
    {
        let mut init = table {
            name: b"PDT\0" as *const u8 as *const libc::c_char,
            type_0: tDAYZONE as libc::c_int,
            value: -(60 as libc::c_int * 60 as libc::c_int * 8 as libc::c_int),
        };
        init
    },
    {
        let mut init = table {
            name: b"AKST\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(60 as libc::c_int * 60 as libc::c_int * 9 as libc::c_int),
        };
        init
    },
    {
        let mut init = table {
            name: b"AKDT\0" as *const u8 as *const libc::c_char,
            type_0: tDAYZONE as libc::c_int,
            value: -(60 as libc::c_int * 60 as libc::c_int * 9 as libc::c_int),
        };
        init
    },
    {
        let mut init = table {
            name: b"HST\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(60 as libc::c_int * 60 as libc::c_int * 10 as libc::c_int),
        };
        init
    },
    {
        let mut init = table {
            name: b"HAST\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(60 as libc::c_int * 60 as libc::c_int * 10 as libc::c_int),
        };
        init
    },
    {
        let mut init = table {
            name: b"HADT\0" as *const u8 as *const libc::c_char,
            type_0: tDAYZONE as libc::c_int,
            value: -(60 as libc::c_int * 60 as libc::c_int * 10 as libc::c_int),
        };
        init
    },
    {
        let mut init = table {
            name: b"SST\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(60 as libc::c_int * 60 as libc::c_int * 12 as libc::c_int),
        };
        init
    },
    {
        let mut init = table {
            name: b"WAT\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 60 as libc::c_int * 60 as libc::c_int * 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"CET\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 60 as libc::c_int * 60 as libc::c_int * 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"CEST\0" as *const u8 as *const libc::c_char,
            type_0: tDAYZONE as libc::c_int,
            value: 60 as libc::c_int * 60 as libc::c_int * 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"MET\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 60 as libc::c_int * 60 as libc::c_int * 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"MEZ\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 60 as libc::c_int * 60 as libc::c_int * 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"MEST\0" as *const u8 as *const libc::c_char,
            type_0: tDAYZONE as libc::c_int,
            value: 60 as libc::c_int * 60 as libc::c_int * 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"MESZ\0" as *const u8 as *const libc::c_char,
            type_0: tDAYZONE as libc::c_int,
            value: 60 as libc::c_int * 60 as libc::c_int * 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"EET\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 60 as libc::c_int * 60 as libc::c_int * 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"EEST\0" as *const u8 as *const libc::c_char,
            type_0: tDAYZONE as libc::c_int,
            value: 60 as libc::c_int * 60 as libc::c_int * 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"CAT\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 60 as libc::c_int * 60 as libc::c_int * 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"SAST\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 60 as libc::c_int * 60 as libc::c_int * 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"EAT\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 60 as libc::c_int * 60 as libc::c_int * 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"MSK\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 60 as libc::c_int * 60 as libc::c_int * 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"MSD\0" as *const u8 as *const libc::c_char,
            type_0: tDAYZONE as libc::c_int,
            value: 60 as libc::c_int * 60 as libc::c_int * 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"IST\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 60 as libc::c_int * 60 as libc::c_int * 5 as libc::c_int
                + 30 as libc::c_int * 60 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"SGT\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 60 as libc::c_int * 60 as libc::c_int * 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"KST\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 60 as libc::c_int * 60 as libc::c_int * 9 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"JST\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 60 as libc::c_int * 60 as libc::c_int * 9 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"GST\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 60 as libc::c_int * 60 as libc::c_int * 10 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"NZST\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 60 as libc::c_int * 60 as libc::c_int * 12 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"NZDT\0" as *const u8 as *const libc::c_char,
            type_0: tDAYZONE as libc::c_int,
            value: 60 as libc::c_int * 60 as libc::c_int * 12 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: 0 as *const libc::c_char,
            type_0: 0 as libc::c_int,
            value: 0 as libc::c_int,
        };
        init
    },
];
static mut yytranslate: [yytype_int8; 278] = [
    0 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
];
static mut military_table: [table; 27] = [
    {
        let mut init = table {
            name: b"A\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 60 as libc::c_int * 60 as libc::c_int * 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"B\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 60 as libc::c_int * 60 as libc::c_int * 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"C\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 60 as libc::c_int * 60 as libc::c_int * 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"D\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 60 as libc::c_int * 60 as libc::c_int * 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"E\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 60 as libc::c_int * 60 as libc::c_int * 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"F\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 60 as libc::c_int * 60 as libc::c_int * 6 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"G\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 60 as libc::c_int * 60 as libc::c_int * 7 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"H\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 60 as libc::c_int * 60 as libc::c_int * 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"I\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 60 as libc::c_int * 60 as libc::c_int * 9 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"J\0" as *const u8 as *const libc::c_char,
            type_0: 'J' as i32,
            value: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"K\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 60 as libc::c_int * 60 as libc::c_int * 10 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"L\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 60 as libc::c_int * 60 as libc::c_int * 11 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"M\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 60 as libc::c_int * 60 as libc::c_int * 12 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"N\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(60 as libc::c_int * 60 as libc::c_int * 1 as libc::c_int),
        };
        init
    },
    {
        let mut init = table {
            name: b"O\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(60 as libc::c_int * 60 as libc::c_int * 2 as libc::c_int),
        };
        init
    },
    {
        let mut init = table {
            name: b"P\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(60 as libc::c_int * 60 as libc::c_int * 3 as libc::c_int),
        };
        init
    },
    {
        let mut init = table {
            name: b"Q\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(60 as libc::c_int * 60 as libc::c_int * 4 as libc::c_int),
        };
        init
    },
    {
        let mut init = table {
            name: b"R\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(60 as libc::c_int * 60 as libc::c_int * 5 as libc::c_int),
        };
        init
    },
    {
        let mut init = table {
            name: b"S\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(60 as libc::c_int * 60 as libc::c_int * 6 as libc::c_int),
        };
        init
    },
    {
        let mut init = table {
            name: b"T\0" as *const u8 as *const libc::c_char,
            type_0: 'T' as i32,
            value: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: b"U\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(60 as libc::c_int * 60 as libc::c_int * 8 as libc::c_int),
        };
        init
    },
    {
        let mut init = table {
            name: b"V\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(60 as libc::c_int * 60 as libc::c_int * 9 as libc::c_int),
        };
        init
    },
    {
        let mut init = table {
            name: b"W\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(60 as libc::c_int * 60 as libc::c_int * 10 as libc::c_int),
        };
        init
    },
    {
        let mut init = table {
            name: b"X\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(60 as libc::c_int * 60 as libc::c_int * 11 as libc::c_int),
        };
        init
    },
    {
        let mut init = table {
            name: b"Y\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: -(60 as libc::c_int * 60 as libc::c_int * 12 as libc::c_int),
        };
        init
    },
    {
        let mut init = table {
            name: b"Z\0" as *const u8 as *const libc::c_char,
            type_0: tZONE as libc::c_int,
            value: 60 as libc::c_int * 60 as libc::c_int * 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = table {
            name: 0 as *const libc::c_char,
            type_0: 0 as libc::c_int,
            value: 0 as libc::c_int,
        };
        init
    },
];
unsafe extern "C" fn time_zone_hhmm(
    mut pc: *mut parser_control,
    mut s: textint,
    mut mm: intmax_t,
) -> bool {
    let mut n_minutes: intmax_t = 0;
    let mut overflow: bool = 0 as libc::c_int != 0;
    if s.digits <= 2 as libc::c_int as libc::c_long
        && mm < 0 as libc::c_int as libc::c_long
    {
        s.value *= 100 as libc::c_int as libc::c_long;
    }
    if mm < 0 as libc::c_int as libc::c_long {
        n_minutes = s.value / 100 as libc::c_int as libc::c_long
            * 60 as libc::c_int as libc::c_long
            + s.value % 100 as libc::c_int as libc::c_long;
    } else {
        overflow = (overflow as libc::c_int
            | (if (0 as libc::c_int as intmax_t) < -(1 as libc::c_int) as intmax_t
                && ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    s.value
                }) - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long
                && ((if 1 as libc::c_int != 0 {
                    0 as libc::c_int
                } else {
                    60 as libc::c_int
                }) - 1 as libc::c_int) < 0 as libc::c_int
                && (if (60 as libc::c_int) < 0 as libc::c_int {
                    if s.value < 0 as libc::c_int as libc::c_long {
                        if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                -(1 as libc::c_int) as intmax_t
                            }) + 60 as libc::c_int as libc::c_long
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            (s.value
                                < -(1 as libc::c_int) as intmax_t
                                    / 60 as libc::c_int as libc::c_long) as libc::c_int
                        } else {
                            ((if (if (if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                60 as libc::c_int
                            }) - 1 as libc::c_int) < 0 as libc::c_int
                            {
                                !(((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    60 as libc::c_int
                                }) + 1 as libc::c_int)
                                    << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    60 as libc::c_int
                                }) + 0 as libc::c_int
                            }) < 0 as libc::c_int
                            {
                                ((60 as libc::c_int)
                                    < -(if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        60 as libc::c_int
                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                    {
                                        ((((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            60 as libc::c_int
                                        }) + 1 as libc::c_int)
                                            << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            60 as libc::c_int
                                        }) - 1 as libc::c_int
                                    })) as libc::c_int
                            } else {
                                ((0 as libc::c_int) < 60 as libc::c_int) as libc::c_int
                            }) != 0
                            {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    60 as libc::c_int
                                }) as libc::c_long + -(1 as libc::c_int) as intmax_t
                                    >> (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            } else {
                                -(1 as libc::c_int) as intmax_t
                                    / -(60 as libc::c_int) as libc::c_long
                            }) <= -(1 as libc::c_int) as libc::c_long - s.value)
                                as libc::c_int
                        }
                    } else {
                        if (if (if ((if 1 as libc::c_int != 0 {
                            0 as libc::c_int as libc::c_long
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                60 as libc::c_int
                            }) as libc::c_long + 0 as libc::c_int as intmax_t
                        }) - 1 as libc::c_int as libc::c_long)
                            < 0 as libc::c_int as libc::c_long
                        {
                            !(((((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    60 as libc::c_int
                                }) as libc::c_long + 0 as libc::c_int as intmax_t
                            }) + 1 as libc::c_int as libc::c_long)
                                << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                - 1 as libc::c_int as libc::c_long)
                                * 2 as libc::c_int as libc::c_long
                                + 1 as libc::c_int as libc::c_long)
                        } else {
                            (if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    60 as libc::c_int
                                }) as libc::c_long + 0 as libc::c_int as intmax_t
                            }) + 0 as libc::c_int as libc::c_long
                        }) < 0 as libc::c_int as libc::c_long
                        {
                            (((if 1 as libc::c_int != 0 {
                                0 as libc::c_int
                            } else {
                                60 as libc::c_int
                            }) as libc::c_long + 0 as libc::c_int as intmax_t)
                                < -(if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        60 as libc::c_int
                                    }) as libc::c_long + 0 as libc::c_int as intmax_t
                                }) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    ((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            60 as libc::c_int
                                        }) as libc::c_long + 0 as libc::c_int as intmax_t
                                    }) + 1 as libc::c_int as libc::c_long)
                                        << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int as libc::c_long)
                                        * 2 as libc::c_int as libc::c_long
                                        + 1 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            60 as libc::c_int
                                        }) as libc::c_long + 0 as libc::c_int as intmax_t
                                    }) - 1 as libc::c_int as libc::c_long
                                })) as libc::c_int
                        } else {
                            ((0 as libc::c_int as libc::c_long)
                                < (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    60 as libc::c_int
                                }) as libc::c_long + 0 as libc::c_int as intmax_t)
                                as libc::c_int
                        }) != 0 && 60 as libc::c_int == -(1 as libc::c_int)
                        {
                            if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                s.value
                            }) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                ((0 as libc::c_int as libc::c_long)
                                    < s.value + 0 as libc::c_int as intmax_t) as libc::c_int
                            } else {
                                ((0 as libc::c_int as libc::c_long) < s.value
                                    && (-(1 as libc::c_int) as libc::c_long
                                        - 0 as libc::c_int as intmax_t)
                                        < s.value - 1 as libc::c_int as libc::c_long) as libc::c_int
                            }
                        } else {
                            ((0 as libc::c_int as intmax_t
                                / 60 as libc::c_int as libc::c_long) < s.value)
                                as libc::c_int
                        }
                    }
                } else {
                    if 60 as libc::c_int == 0 as libc::c_int {
                        0 as libc::c_int
                    } else {
                        if s.value < 0 as libc::c_int as libc::c_long {
                            if (if (if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    s.value
                                }) + 0 as libc::c_int as intmax_t
                            }) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                !(((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        s.value
                                    }) + 0 as libc::c_int as intmax_t
                                }) + 1 as libc::c_int as libc::c_long)
                                    << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long)
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        s.value
                                    }) + 0 as libc::c_int as intmax_t
                                }) + 0 as libc::c_int as libc::c_long
                            }) < 0 as libc::c_int as libc::c_long
                            {
                                (((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    s.value
                                }) + 0 as libc::c_int as intmax_t)
                                    < -(if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            s.value
                                        }) + 0 as libc::c_int as intmax_t
                                    }) - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        ((((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                s.value
                                            }) + 0 as libc::c_int as intmax_t
                                        }) + 1 as libc::c_int as libc::c_long)
                                            << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int as libc::c_long)
                                            * 2 as libc::c_int as libc::c_long
                                            + 1 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                s.value
                                            }) + 0 as libc::c_int as intmax_t
                                        }) - 1 as libc::c_int as libc::c_long
                                    })) as libc::c_int
                            } else {
                                ((0 as libc::c_int as libc::c_long)
                                    < (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        s.value
                                    }) + 0 as libc::c_int as intmax_t) as libc::c_int
                            }) != 0 && s.value == -(1 as libc::c_int) as libc::c_long
                            {
                                if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    60 as libc::c_int
                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                {
                                    ((0 as libc::c_int as libc::c_long)
                                        < 60 as libc::c_int as libc::c_long
                                            + 0 as libc::c_int as intmax_t) as libc::c_int
                                } else {
                                    ((-(1 as libc::c_int) as libc::c_long
                                        - 0 as libc::c_int as intmax_t)
                                        < (60 as libc::c_int - 1 as libc::c_int) as libc::c_long)
                                        as libc::c_int
                                }
                            } else {
                                (0 as libc::c_int as intmax_t / s.value
                                    < 60 as libc::c_int as libc::c_long) as libc::c_int
                            }
                        } else {
                            ((-(1 as libc::c_int) as intmax_t
                                / 60 as libc::c_int as libc::c_long) < s.value)
                                as libc::c_int
                        }
                    }
                }) != 0
            {
                let (fresh38, fresh39) = (s.value).overflowing_mul((60 as libc::c_int).into());
                *(&mut n_minutes as *mut intmax_t) = fresh38;
                1 as libc::c_int
            } else {
                let (fresh40, fresh41) = (s.value).overflowing_mul((60 as libc::c_int).into());
                *(&mut n_minutes as *mut intmax_t) = fresh40;
                fresh41 as libc::c_int
            } != 0) as libc::c_int) != 0;
        overflow = (overflow as libc::c_int
            | if s.negative as libc::c_int != 0 {
                let (fresh42, fresh43) = n_minutes.overflowing_sub(mm);
                *(&mut n_minutes as *mut intmax_t) = fresh42;
                fresh43 as libc::c_int
            } else {
                let (fresh44, fresh45) = n_minutes.overflowing_add(mm);
                *(&mut n_minutes as *mut intmax_t) = fresh44;
                fresh45 as libc::c_int
            }) != 0;
    }
    if overflow as libc::c_int != 0
        || !((-(24 as libc::c_int) * 60 as libc::c_int) as libc::c_long <= n_minutes
            && n_minutes <= (24 as libc::c_int * 60 as libc::c_int) as libc::c_long)
    {
        return 0 as libc::c_int != 0;
    }
    (*pc).time_zone = (n_minutes * 60 as libc::c_int as libc::c_long) as libc::c_int;
    return 1 as libc::c_int != 0;
}
static mut yypact: [yytype_int8; 115] = [
    -(14 as libc::c_int) as yytype_int8,
    7 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    37 as libc::c_int as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    14 as libc::c_int as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    64 as libc::c_int as yytype_int8,
    47 as libc::c_int as yytype_int8,
    67 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    82 as libc::c_int as yytype_int8,
    -(4 as libc::c_int) as yytype_int8,
    74 as libc::c_int as yytype_int8,
    75 as libc::c_int as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    76 as libc::c_int as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    69 as libc::c_int as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    93 as libc::c_int as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    79 as libc::c_int as yytype_int8,
    72 as libc::c_int as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    26 as libc::c_int as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    62 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    80 as libc::c_int as yytype_int8,
    81 as libc::c_int as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    83 as libc::c_int as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    84 as libc::c_int as yytype_int8,
    85 as libc::c_int as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    45 as libc::c_int as yytype_int8,
    86 as libc::c_int as yytype_int8,
    -(12 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    87 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    88 as libc::c_int as yytype_int8,
    89 as libc::c_int as yytype_int8,
    78 as libc::c_int as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    59 as libc::c_int as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    18 as libc::c_int as yytype_int8,
    91 as libc::c_int as yytype_int8,
];
unsafe extern "C" fn to_hour(
    mut hours: intmax_t,
    mut meridian: libc::c_int,
) -> libc::c_int {
    match meridian {
        2 => {}
        0 => {
            return (if (0 as libc::c_int as libc::c_long) < hours
                && hours < 12 as libc::c_int as libc::c_long
            {
                hours
            } else {
                (if hours == 12 as libc::c_int as libc::c_long {
                    0 as libc::c_int
                } else {
                    -(1 as libc::c_int)
                }) as libc::c_long
            }) as libc::c_int;
        }
        1 => {
            return (if (0 as libc::c_int as libc::c_long) < hours
                && hours < 12 as libc::c_int as libc::c_long
            {
                hours + 12 as libc::c_int as libc::c_long
            } else {
                (if hours == 12 as libc::c_int as libc::c_long {
                    12 as libc::c_int
                } else {
                    -(1 as libc::c_int)
                }) as libc::c_long
            }) as libc::c_int;
        }
        _ => {}
    }
    return (if 0 as libc::c_int as libc::c_long <= hours
        && hours < 24 as libc::c_int as libc::c_long
    {
        hours
    } else {
        -(1 as libc::c_int) as libc::c_long
    }) as libc::c_int;
}
static mut yydefact: [yytype_int8; 115] = [
    5 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    86 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    85 as libc::c_int as yytype_int8,
    87 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    83 as libc::c_int as yytype_int8,
    84 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    57 as libc::c_int as yytype_int8,
    60 as libc::c_int as yytype_int8,
    66 as libc::c_int as yytype_int8,
    69 as libc::c_int as yytype_int8,
    74 as libc::c_int as yytype_int8,
    63 as libc::c_int as yytype_int8,
    82 as libc::c_int as yytype_int8,
    38 as libc::c_int as yytype_int8,
    36 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    89 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    32 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    53 as libc::c_int as yytype_int8,
    75 as libc::c_int as yytype_int8,
    54 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    46 as libc::c_int as yytype_int8,
    55 as libc::c_int as yytype_int8,
    58 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    67 as libc::c_int as yytype_int8,
    70 as libc::c_int as yytype_int8,
    61 as libc::c_int as yytype_int8,
    40 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    91 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    76 as libc::c_int as yytype_int8,
    77 as libc::c_int as yytype_int8,
    79 as libc::c_int as yytype_int8,
    80 as libc::c_int as yytype_int8,
    81 as libc::c_int as yytype_int8,
    78 as libc::c_int as yytype_int8,
    56 as libc::c_int as yytype_int8,
    59 as libc::c_int as yytype_int8,
    65 as libc::c_int as yytype_int8,
    68 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    62 as libc::c_int as yytype_int8,
    41 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    48 as libc::c_int as yytype_int8,
    91 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    90 as libc::c_int as yytype_int8,
    72 as libc::c_int as yytype_int8,
    73 as libc::c_int as yytype_int8,
    34 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    52 as libc::c_int as yytype_int8,
    45 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    35 as libc::c_int as yytype_int8,
    44 as libc::c_int as yytype_int8,
    49 as libc::c_int as yytype_int8,
    51 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    42 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    47 as libc::c_int as yytype_int8,
    92 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    91 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
];
unsafe extern "C" fn tm_year_str(
    mut tm_year: libc::c_int,
    mut buf: *mut libc::c_char,
) -> *const libc::c_char {
    sprintf(
        buf,
        &*(b"-%02d%02d\0" as *const u8 as *const libc::c_char)
            .offset((-(TM_YEAR_BASE as libc::c_int) <= tm_year) as libc::c_int as isize)
            as *const libc::c_char,
        abs(
            tm_year / 100 as libc::c_int
                + TM_YEAR_BASE as libc::c_int / 100 as libc::c_int,
        ),
        abs(tm_year % 100 as libc::c_int),
    );
    return buf as *const libc::c_char;
}
static mut yypgoto: [yytype_int8; 26] = [
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    17 as libc::c_int as yytype_int8,
    -(28 as libc::c_int) as yytype_int8,
    -(27 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    38 as libc::c_int as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(90 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    -(91 as libc::c_int) as yytype_int8,
    46 as libc::c_int as yytype_int8,
];
static mut yydefgoto: [yytype_int8; 26] = [
    0 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    32 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    34 as libc::c_int as yytype_int8,
    35 as libc::c_int as yytype_int8,
    36 as libc::c_int as yytype_int8,
    104 as libc::c_int as yytype_int8,
    105 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    38 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    40 as libc::c_int as yytype_int8,
    41 as libc::c_int as yytype_int8,
    42 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    44 as libc::c_int as yytype_int8,
    45 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    46 as libc::c_int as yytype_int8,
    47 as libc::c_int as yytype_int8,
    94 as libc::c_int as yytype_int8,
];
unsafe extern "C" fn to_tm_year(
    mut textyear: textint,
    mut debug: bool,
    mut tm_year: *mut libc::c_int,
) -> bool {
    let mut year: intmax_t = textyear.value;
    if 0 as libc::c_int as libc::c_long <= year
        && textyear.digits == 2 as libc::c_int as libc::c_long
    {
        year
            += (if year < 69 as libc::c_int as libc::c_long {
                2000 as libc::c_int
            } else {
                1900 as libc::c_int
            }) as libc::c_long;
        if debug {
            dbg_printf(
                gettext(
                    b"warning: adjusting year value %ld to %ld\n\0" as *const u8
                        as *const libc::c_char,
                ),
                textyear.value,
                year,
            );
        }
    }
    if if year < 0 as libc::c_int as libc::c_long {
        let (fresh46, fresh47) = (-(TM_YEAR_BASE as libc::c_int)).overflowing_sub(year.try_into().unwrap());
        *tm_year = fresh46;
        fresh47 as libc::c_int
    } else {
        let (fresh48, fresh49) = year.overflowing_sub((TM_YEAR_BASE as libc::c_int).into());
        *tm_year = fresh48;
        fresh49 as libc::c_int
    } != 0
    {
        if debug {
            dbg_printf(
                gettext(
                    b"error: out-of-range year %ld\n\0" as *const u8
                        as *const libc::c_char,
                ),
                year,
            );
        }
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
static mut yytable: [yytype_int8; 115] = [
    80 as libc::c_int as yytype_int8,
    68 as libc::c_int as yytype_int8,
    69 as libc::c_int as yytype_int8,
    70 as libc::c_int as yytype_int8,
    71 as libc::c_int as yytype_int8,
    72 as libc::c_int as yytype_int8,
    73 as libc::c_int as yytype_int8,
    102 as libc::c_int as yytype_int8,
    74 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    59 as libc::c_int as yytype_int8,
    75 as libc::c_int as yytype_int8,
    76 as libc::c_int as yytype_int8,
    108 as libc::c_int as yytype_int8,
    107 as libc::c_int as yytype_int8,
    77 as libc::c_int as yytype_int8,
    62 as libc::c_int as yytype_int8,
    63 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    65 as libc::c_int as yytype_int8,
    66 as libc::c_int as yytype_int8,
    67 as libc::c_int as yytype_int8,
    78 as libc::c_int as yytype_int8,
    114 as libc::c_int as yytype_int8,
    79 as libc::c_int as yytype_int8,
    60 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    93 as libc::c_int as yytype_int8,
    62 as libc::c_int as yytype_int8,
    63 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    65 as libc::c_int as yytype_int8,
    66 as libc::c_int as yytype_int8,
    67 as libc::c_int as yytype_int8,
    89 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    48 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    89 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    101 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    61 as libc::c_int as yytype_int8,
    102 as libc::c_int as yytype_int8,
    81 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    51 as libc::c_int as yytype_int8,
    49 as libc::c_int as yytype_int8,
    84 as libc::c_int as yytype_int8,
    80 as libc::c_int as yytype_int8,
    103 as libc::c_int as yytype_int8,
    52 as libc::c_int as yytype_int8,
    53 as libc::c_int as yytype_int8,
    54 as libc::c_int as yytype_int8,
    55 as libc::c_int as yytype_int8,
    56 as libc::c_int as yytype_int8,
    57 as libc::c_int as yytype_int8,
    102 as libc::c_int as yytype_int8,
    58 as libc::c_int as yytype_int8,
    112 as libc::c_int as yytype_int8,
    91 as libc::c_int as yytype_int8,
    92 as libc::c_int as yytype_int8,
    82 as libc::c_int as yytype_int8,
    83 as libc::c_int as yytype_int8,
    113 as libc::c_int as yytype_int8,
    112 as libc::c_int as yytype_int8,
    62 as libc::c_int as yytype_int8,
    63 as libc::c_int as yytype_int8,
    64 as libc::c_int as yytype_int8,
    65 as libc::c_int as yytype_int8,
    66 as libc::c_int as yytype_int8,
    67 as libc::c_int as yytype_int8,
    111 as libc::c_int as yytype_int8,
    85 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    86 as libc::c_int as yytype_int8,
    102 as libc::c_int as yytype_int8,
    87 as libc::c_int as yytype_int8,
    88 as libc::c_int as yytype_int8,
    95 as libc::c_int as yytype_int8,
    96 as libc::c_int as yytype_int8,
    98 as libc::c_int as yytype_int8,
    97 as libc::c_int as yytype_int8,
    99 as libc::c_int as yytype_int8,
    100 as libc::c_int as yytype_int8,
    90 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    109 as libc::c_int as yytype_int8,
    110 as libc::c_int as yytype_int8,
    102 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    89 as libc::c_int as yytype_int8,
    106 as libc::c_int as yytype_int8,
];
static mut yycheck: [yytype_int8; 115] = [
    27 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    103 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    113 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    97 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    108 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    114 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    85 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    60 as libc::c_int as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    20 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    -(1 as libc::c_int) as yytype_int8,
    26 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
];
unsafe extern "C" fn lookup_zone(
    mut pc: *const parser_control,
    mut name: *const libc::c_char,
) -> *const table {
    let mut tp: *const table = 0 as *const table;
    tp = universal_time_zone_table.as_ptr();
    while !((*tp).name).is_null() {
        if strcmp(name, (*tp).name) == 0 as libc::c_int {
            return tp;
        }
        tp = tp.offset(1);
        tp;
    }
    tp = ((*pc).local_time_zone_table).as_ptr();
    while !((*tp).name).is_null() {
        if strcmp(name, (*tp).name) == 0 as libc::c_int {
            return tp;
        }
        tp = tp.offset(1);
        tp;
    }
    tp = time_zone_table.as_ptr();
    while !((*tp).name).is_null() {
        if strcmp(name, (*tp).name) == 0 as libc::c_int {
            return tp;
        }
        tp = tp.offset(1);
        tp;
    }
    return 0 as *const table;
}
static mut yystos: [yytype_int8; 115] = [
    0 as libc::c_int as yytype_int8,
    23 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    32 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    49 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    51 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    11 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    13 as libc::c_int as yytype_int8,
    14 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    17 as libc::c_int as yytype_int8,
    18 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    21 as libc::c_int as yytype_int8,
    22 as libc::c_int as yytype_int8,
    24 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    34 as libc::c_int as yytype_int8,
    35 as libc::c_int as yytype_int8,
    36 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    40 as libc::c_int as yytype_int8,
    41 as libc::c_int as yytype_int8,
    42 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    44 as libc::c_int as yytype_int8,
    45 as libc::c_int as yytype_int8,
    46 as libc::c_int as yytype_int8,
    47 as libc::c_int as yytype_int8,
    48 as libc::c_int as yytype_int8,
    52 as libc::c_int as yytype_int8,
    53 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    47 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    7 as libc::c_int as yytype_int8,
    8 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    10 as libc::c_int as yytype_int8,
    12 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    16 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    47 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    9 as libc::c_int as yytype_int8,
    47 as libc::c_int as yytype_int8,
    25 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    27 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    54 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    54 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    19 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    38 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    28 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    51 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    20 as libc::c_int as yytype_int8,
    15 as libc::c_int as yytype_int8,
    38 as libc::c_int as yytype_int8,
    26 as libc::c_int as yytype_int8,
    51 as libc::c_int as yytype_int8,
];
static mut yyr1: [yytype_int8; 93] = [
    0 as libc::c_int as yytype_int8,
    29 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    30 as libc::c_int as yytype_int8,
    31 as libc::c_int as yytype_int8,
    32 as libc::c_int as yytype_int8,
    32 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    33 as libc::c_int as yytype_int8,
    34 as libc::c_int as yytype_int8,
    35 as libc::c_int as yytype_int8,
    36 as libc::c_int as yytype_int8,
    36 as libc::c_int as yytype_int8,
    36 as libc::c_int as yytype_int8,
    36 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    37 as libc::c_int as yytype_int8,
    38 as libc::c_int as yytype_int8,
    38 as libc::c_int as yytype_int8,
    39 as libc::c_int as yytype_int8,
    40 as libc::c_int as yytype_int8,
    40 as libc::c_int as yytype_int8,
    41 as libc::c_int as yytype_int8,
    41 as libc::c_int as yytype_int8,
    41 as libc::c_int as yytype_int8,
    41 as libc::c_int as yytype_int8,
    41 as libc::c_int as yytype_int8,
    41 as libc::c_int as yytype_int8,
    41 as libc::c_int as yytype_int8,
    42 as libc::c_int as yytype_int8,
    42 as libc::c_int as yytype_int8,
    42 as libc::c_int as yytype_int8,
    42 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    43 as libc::c_int as yytype_int8,
    44 as libc::c_int as yytype_int8,
    45 as libc::c_int as yytype_int8,
    45 as libc::c_int as yytype_int8,
    45 as libc::c_int as yytype_int8,
    46 as libc::c_int as yytype_int8,
    46 as libc::c_int as yytype_int8,
    46 as libc::c_int as yytype_int8,
    46 as libc::c_int as yytype_int8,
    46 as libc::c_int as yytype_int8,
    46 as libc::c_int as yytype_int8,
    46 as libc::c_int as yytype_int8,
    46 as libc::c_int as yytype_int8,
    46 as libc::c_int as yytype_int8,
    46 as libc::c_int as yytype_int8,
    46 as libc::c_int as yytype_int8,
    46 as libc::c_int as yytype_int8,
    46 as libc::c_int as yytype_int8,
    46 as libc::c_int as yytype_int8,
    46 as libc::c_int as yytype_int8,
    46 as libc::c_int as yytype_int8,
    46 as libc::c_int as yytype_int8,
    46 as libc::c_int as yytype_int8,
    46 as libc::c_int as yytype_int8,
    46 as libc::c_int as yytype_int8,
    46 as libc::c_int as yytype_int8,
    47 as libc::c_int as yytype_int8,
    47 as libc::c_int as yytype_int8,
    47 as libc::c_int as yytype_int8,
    47 as libc::c_int as yytype_int8,
    47 as libc::c_int as yytype_int8,
    47 as libc::c_int as yytype_int8,
    48 as libc::c_int as yytype_int8,
    49 as libc::c_int as yytype_int8,
    49 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    50 as libc::c_int as yytype_int8,
    51 as libc::c_int as yytype_int8,
    51 as libc::c_int as yytype_int8,
    52 as libc::c_int as yytype_int8,
    53 as libc::c_int as yytype_int8,
    54 as libc::c_int as yytype_int8,
    54 as libc::c_int as yytype_int8,
];
static mut yyr2: [yytype_int8; 93] = [
    0 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    6 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    5 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    4 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    3 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    1 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
    0 as libc::c_int as yytype_int8,
    2 as libc::c_int as yytype_int8,
];
unsafe extern "C" fn lookup_word(
    mut pc: *const parser_control,
    mut word: *mut libc::c_char,
) -> *const table {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut wordlen: idx_t = 0;
    let mut tp: *const table = 0 as *const table;
    let mut period_found: bool = false;
    let mut abbrev: bool = false;
    p = word;
    while *p != 0 {
        *p = c_toupper(to_uchar(*p) as libc::c_int) as libc::c_char;
        p = p.offset(1);
        p;
    }
    tp = meridian_table.as_ptr();
    while !((*tp).name).is_null() {
        if strcmp(word, (*tp).name) == 0 as libc::c_int {
            return tp;
        }
        tp = tp.offset(1);
        tp;
    }
    wordlen = strlen(word) as idx_t;
    abbrev = wordlen == 3 as libc::c_int as libc::c_long
        || wordlen == 4 as libc::c_int as libc::c_long
            && *word.offset(3 as libc::c_int as isize) as libc::c_int == '.' as i32;
    tp = month_and_day_table.as_ptr();
    while !((*tp).name).is_null() {
        if (if abbrev as libc::c_int != 0 {
            strncmp(word, (*tp).name, 3 as libc::c_int as libc::c_ulong)
        } else {
            strcmp(word, (*tp).name)
        }) == 0 as libc::c_int
        {
            return tp;
        }
        tp = tp.offset(1);
        tp;
    }
    tp = lookup_zone(pc, word);
    if !tp.is_null() {
        return tp;
    }
    if strcmp(word, dst_table[0 as libc::c_int as usize].name) == 0 as libc::c_int {
        return dst_table.as_ptr();
    }
    tp = time_units_table.as_ptr();
    while !((*tp).name).is_null() {
        if strcmp(word, (*tp).name) == 0 as libc::c_int {
            return tp;
        }
        tp = tp.offset(1);
        tp;
    }
    if *word.offset((wordlen - 1 as libc::c_int as libc::c_long) as isize) as libc::c_int
        == 'S' as i32
    {
        *word
            .offset(
                (wordlen - 1 as libc::c_int as libc::c_long) as isize,
            ) = '\0' as i32 as libc::c_char;
        tp = time_units_table.as_ptr();
        while !((*tp).name).is_null() {
            if strcmp(word, (*tp).name) == 0 as libc::c_int {
                return tp;
            }
            tp = tp.offset(1);
            tp;
        }
        *word
            .offset(
                (wordlen - 1 as libc::c_int as libc::c_long) as isize,
            ) = 'S' as i32 as libc::c_char;
    }
    tp = relative_time_table.as_ptr();
    while !((*tp).name).is_null() {
        if strcmp(word, (*tp).name) == 0 as libc::c_int {
            return tp;
        }
        tp = tp.offset(1);
        tp;
    }
    if wordlen == 1 as libc::c_int as libc::c_long {
        tp = military_table.as_ptr();
        while !((*tp).name).is_null() {
            if *word.offset(0 as libc::c_int as isize) as libc::c_int
                == *((*tp).name).offset(0 as libc::c_int as isize) as libc::c_int
            {
                return tp;
            }
            tp = tp.offset(1);
            tp;
        }
    }
    period_found = 0 as libc::c_int != 0;
    q = word;
    p = q;
    loop {
        *p = *q;
        if !(*p != 0) {
            break;
        }
        if *q as libc::c_int == '.' as i32 {
            period_found = 1 as libc::c_int != 0;
        } else {
            p = p.offset(1);
            p;
        }
        q = q.offset(1);
        q;
    }
    if period_found as libc::c_int != 0
        && {
            tp = lookup_zone(pc, word);
            !tp.is_null()
        }
    {
        return tp;
    }
    return 0 as *const table;
}
unsafe extern "C" fn yylex(
    mut lvalp: *mut YYSTYPE,
    mut pc: *mut parser_control,
) -> libc::c_int {
    let mut c: libc::c_uchar = 0;
    loop {
        loop {
            c = *(*pc).input as libc::c_uchar;
            if !c_isspace(c as libc::c_int) {
                break;
            }
            (*pc).input = ((*pc).input).offset(1);
            (*pc).input;
        }
        if c_isdigit(c as libc::c_int) as libc::c_int != 0
            || c as libc::c_int == '-' as i32 || c as libc::c_int == '+' as i32
        {
            let mut p: *const libc::c_char = (*pc).input;
            let mut sign: libc::c_int = 0;
            if c as libc::c_int == '-' as i32 || c as libc::c_int == '+' as i32 {
                sign = if c as libc::c_int == '-' as i32 {
                    -(1 as libc::c_int)
                } else {
                    1 as libc::c_int
                };
                loop {
                    p = p.offset(1);
                    (*pc).input = p;
                    c = *(*pc).input as libc::c_uchar;
                    if !c_isspace(c as libc::c_int) {
                        break;
                    }
                }
                if !c_isdigit(c as libc::c_int) {
                    continue;
                }
            } else {
                sign = 0 as libc::c_int;
            }
            let mut value: time_t = 0 as libc::c_int as time_t;
            loop {
                if if (0 as libc::c_int as time_t) < -(1 as libc::c_int) as time_t
                    && ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int as libc::c_long
                    } else {
                        value
                    }) - 1 as libc::c_int as libc::c_long)
                        < 0 as libc::c_int as libc::c_long
                    && ((if 1 as libc::c_int != 0 {
                        0 as libc::c_int
                    } else {
                        10 as libc::c_int
                    }) - 1 as libc::c_int) < 0 as libc::c_int
                    && (if (10 as libc::c_int) < 0 as libc::c_int {
                        if value < 0 as libc::c_int as libc::c_long {
                            if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    -(1 as libc::c_int) as time_t
                                }) + 10 as libc::c_int as libc::c_long
                            }) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                (value
                                    < -(1 as libc::c_int) as time_t
                                        / 10 as libc::c_int as libc::c_long) as libc::c_int
                            } else {
                                ((if (if (if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    10 as libc::c_int
                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                {
                                    !(((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        10 as libc::c_int
                                    }) + 1 as libc::c_int)
                                        << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        10 as libc::c_int
                                    }) + 0 as libc::c_int
                                }) < 0 as libc::c_int
                                {
                                    ((10 as libc::c_int)
                                        < -(if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            10 as libc::c_int
                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                        {
                                            ((((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                10 as libc::c_int
                                            }) + 1 as libc::c_int)
                                                << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                10 as libc::c_int
                                            }) - 1 as libc::c_int
                                        })) as libc::c_int
                                } else {
                                    ((0 as libc::c_int) < 10 as libc::c_int) as libc::c_int
                                }) != 0
                                {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        10 as libc::c_int
                                    }) as libc::c_long + -(1 as libc::c_int) as time_t
                                        >> (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                } else {
                                    -(1 as libc::c_int) as time_t
                                        / -(10 as libc::c_int) as libc::c_long
                                }) <= -(1 as libc::c_int) as libc::c_long - value)
                                    as libc::c_int
                            }
                        } else {
                            if (if (if ((if 1 as libc::c_int != 0 {
                                0 as libc::c_int as libc::c_long
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    10 as libc::c_int
                                }) as libc::c_long + 0 as libc::c_int as time_t
                            }) - 1 as libc::c_int as libc::c_long)
                                < 0 as libc::c_int as libc::c_long
                            {
                                !(((((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        10 as libc::c_int
                                    }) as libc::c_long + 0 as libc::c_int as time_t
                                }) + 1 as libc::c_int as libc::c_long)
                                    << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                    - 1 as libc::c_int as libc::c_long)
                                    * 2 as libc::c_int as libc::c_long
                                    + 1 as libc::c_int as libc::c_long)
                            } else {
                                (if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        10 as libc::c_int
                                    }) as libc::c_long + 0 as libc::c_int as time_t
                                }) + 0 as libc::c_int as libc::c_long
                            }) < 0 as libc::c_int as libc::c_long
                            {
                                (((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    10 as libc::c_int
                                }) as libc::c_long + 0 as libc::c_int as time_t)
                                    < -(if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int
                                        } else {
                                            10 as libc::c_int
                                        }) as libc::c_long + 0 as libc::c_int as time_t
                                    }) - 1 as libc::c_int as libc::c_long)
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        ((((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                10 as libc::c_int
                                            }) as libc::c_long + 0 as libc::c_int as time_t
                                        }) + 1 as libc::c_int as libc::c_long)
                                            << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                            - 1 as libc::c_int as libc::c_long)
                                            * 2 as libc::c_int as libc::c_long
                                            + 1 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int
                                            } else {
                                                10 as libc::c_int
                                            }) as libc::c_long + 0 as libc::c_int as time_t
                                        }) - 1 as libc::c_int as libc::c_long
                                    })) as libc::c_int
                            } else {
                                ((0 as libc::c_int as libc::c_long)
                                    < (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        10 as libc::c_int
                                    }) as libc::c_long + 0 as libc::c_int as time_t)
                                    as libc::c_int
                            }) != 0 && 10 as libc::c_int == -(1 as libc::c_int)
                            {
                                if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    value
                                }) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    ((0 as libc::c_int as libc::c_long)
                                        < value + 0 as libc::c_int as time_t) as libc::c_int
                                } else {
                                    ((0 as libc::c_int as libc::c_long) < value
                                        && (-(1 as libc::c_int) as libc::c_long
                                            - 0 as libc::c_int as time_t)
                                            < value - 1 as libc::c_int as libc::c_long) as libc::c_int
                                }
                            } else {
                                ((0 as libc::c_int as time_t
                                    / 10 as libc::c_int as libc::c_long) < value) as libc::c_int
                            }
                        }
                    } else {
                        if 10 as libc::c_int == 0 as libc::c_int {
                            0 as libc::c_int
                        } else {
                            if value < 0 as libc::c_int as libc::c_long {
                                if (if (if ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        value
                                    }) + 0 as libc::c_int as time_t
                                }) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    !(((((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            value
                                        }) + 0 as libc::c_int as time_t
                                    }) + 1 as libc::c_int as libc::c_long)
                                        << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                        - 1 as libc::c_int as libc::c_long)
                                        * 2 as libc::c_int as libc::c_long
                                        + 1 as libc::c_int as libc::c_long)
                                } else {
                                    (if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            value
                                        }) + 0 as libc::c_int as time_t
                                    }) + 0 as libc::c_int as libc::c_long
                                }) < 0 as libc::c_int as libc::c_long
                                {
                                    (((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int as libc::c_long
                                    } else {
                                        value
                                    }) + 0 as libc::c_int as time_t)
                                        < -(if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                value
                                            }) + 0 as libc::c_int as time_t
                                        }) - 1 as libc::c_int as libc::c_long)
                                            < 0 as libc::c_int as libc::c_long
                                        {
                                            ((((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    value
                                                }) + 0 as libc::c_int as time_t
                                            }) + 1 as libc::c_int as libc::c_long)
                                                << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int as libc::c_long)
                                                * 2 as libc::c_int as libc::c_long
                                                + 1 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    value
                                                }) + 0 as libc::c_int as time_t
                                            }) - 1 as libc::c_int as libc::c_long
                                        })) as libc::c_int
                                } else {
                                    ((0 as libc::c_int as libc::c_long)
                                        < (if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            value
                                        }) + 0 as libc::c_int as time_t) as libc::c_int
                                }) != 0 && value == -(1 as libc::c_int) as libc::c_long
                                {
                                    if ((if 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        10 as libc::c_int
                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                    {
                                        ((0 as libc::c_int as libc::c_long)
                                            < 10 as libc::c_int as libc::c_long
                                                + 0 as libc::c_int as time_t) as libc::c_int
                                    } else {
                                        ((-(1 as libc::c_int) as libc::c_long
                                            - 0 as libc::c_int as time_t)
                                            < (10 as libc::c_int - 1 as libc::c_int) as libc::c_long)
                                            as libc::c_int
                                    }
                                } else {
                                    (0 as libc::c_int as time_t / value
                                        < 10 as libc::c_int as libc::c_long) as libc::c_int
                                }
                            } else {
                                ((-(1 as libc::c_int) as time_t
                                    / 10 as libc::c_int as libc::c_long) < value) as libc::c_int
                            }
                        }
                    }) != 0
                {
                    let (fresh54, fresh55) = value.overflowing_mul((10 as libc::c_int).into());
                    *(&mut value as *mut time_t) = fresh54;
                    1 as libc::c_int
                } else {
                    let (fresh56, fresh57) = value.overflowing_mul((10 as libc::c_int).into());
                    *(&mut value as *mut time_t) = fresh56;
                    fresh57 as libc::c_int
                } != 0
                {
                    return '?' as i32;
                }
                let (fresh58, fresh59) = value
                    .overflowing_add(
                        if sign < 0 as libc::c_int {
                            ('0' as i32 - c as libc::c_int).into()
                        } else {
                            (c as libc::c_int - '0' as i32).into()
                        },
                    );
                *(&mut value as *mut time_t) = fresh58;
                if fresh59 {
                    return '?' as i32;
                }
                p = p.offset(1);
                c = *p as libc::c_uchar;
                if !c_isdigit(c as libc::c_int) {
                    break;
                }
            }
            if (c as libc::c_int == '.' as i32 || c as libc::c_int == ',' as i32)
                && c_isdigit(*p.offset(1 as libc::c_int as isize) as libc::c_int)
                    as libc::c_int != 0
            {
                let mut s: time_t = value;
                let mut digits: libc::c_int = 0;
                p = p.offset(1);
                p;
                let fresh60 = p;
                p = p.offset(1);
                let mut ns: libc::c_int = *fresh60 as libc::c_int - '0' as i32;
                digits = 2 as libc::c_int;
                while digits <= LOG10_BILLION as libc::c_int {
                    ns *= 10 as libc::c_int;
                    if c_isdigit(*p as libc::c_int) {
                        let fresh61 = p;
                        p = p.offset(1);
                        ns += *fresh61 as libc::c_int - '0' as i32;
                    }
                    digits += 1;
                    digits;
                }
                if sign < 0 as libc::c_int {
                    while c_isdigit(*p as libc::c_int) {
                        if *p as libc::c_int != '0' as i32 {
                            ns += 1;
                            ns;
                            break;
                        } else {
                            p = p.offset(1);
                            p;
                        }
                    }
                }
                while c_isdigit(*p as libc::c_int) {
                    p = p.offset(1);
                    p;
                }
                if sign < 0 as libc::c_int && ns != 0 {
                    let (fresh62, fresh63) = s.overflowing_sub((1 as libc::c_int).into());
                    *(&mut s as *mut time_t) = fresh62;
                    if fresh63 {
                        return '?' as i32;
                    }
                    ns = BILLION as libc::c_int - ns;
                }
                (*lvalp)
                    .timespec = {
                    let mut init = timespec {
                        tv_sec: s,
                        tv_nsec: ns as __syscall_slong_t,
                    };
                    init
                };
                (*pc).input = p;
                return if sign != 0 {
                    tSDECIMAL_NUMBER as libc::c_int
                } else {
                    tUDECIMAL_NUMBER as libc::c_int
                };
            } else {
                (*lvalp).textintval.negative = sign < 0 as libc::c_int;
                (*lvalp).textintval.value = value;
                (*lvalp).textintval.digits = p.offset_from((*pc).input) as libc::c_long;
                (*pc).input = p;
                return if sign != 0 {
                    tSNUMBER as libc::c_int
                } else {
                    tUNUMBER as libc::c_int
                };
            }
        } else {
            if c_isalpha(c as libc::c_int) {
                let mut buff: [libc::c_char; 20] = [0; 20];
                let mut p_0: *mut libc::c_char = buff.as_mut_ptr();
                let mut tp: *const table = 0 as *const table;
                loop {
                    if p_0
                        < buff
                            .as_mut_ptr()
                            .offset(
                                ::core::mem::size_of::<[libc::c_char; 20]>()
                                    as libc::c_ulong as isize,
                            )
                            .offset(-(1 as libc::c_int as isize))
                    {
                        let fresh64 = p_0;
                        p_0 = p_0.offset(1);
                        *fresh64 = c as libc::c_char;
                    }
                    (*pc).input = ((*pc).input).offset(1);
                    c = *(*pc).input as libc::c_uchar;
                    if !(c_isalpha(c as libc::c_int) as libc::c_int != 0
                        || c as libc::c_int == '.' as i32)
                    {
                        break;
                    }
                }
                *p_0 = '\0' as i32 as libc::c_char;
                tp = lookup_word(pc, buff.as_mut_ptr());
                if tp.is_null() {
                    if debugging(pc) {
                        dbg_printf(
                            gettext(
                                b"error: unknown word '%s'\n\0" as *const u8
                                    as *const libc::c_char,
                            ),
                            buff.as_mut_ptr(),
                        );
                    }
                    return '?' as i32;
                }
                (*lvalp).intval = (*tp).value as intmax_t;
                return (*tp).type_0;
            }
            if c as libc::c_int != '(' as i32 {
                let fresh65 = (*pc).input;
                (*pc).input = ((*pc).input).offset(1);
                return to_uchar(*fresh65) as libc::c_int;
            }
            let mut count: idx_t = 0 as libc::c_int as idx_t;
            loop {
                let fresh66 = (*pc).input;
                (*pc).input = ((*pc).input).offset(1);
                c = *fresh66 as libc::c_uchar;
                if c as libc::c_int == '\0' as i32 {
                    return c as libc::c_int;
                }
                if c as libc::c_int == '(' as i32 {
                    count += 1;
                    count;
                } else if c as libc::c_int == ')' as i32 {
                    count -= 1;
                    count;
                }
                if !(count != 0 as libc::c_int as libc::c_long) {
                    break;
                }
            }
        }
    };
}
unsafe extern "C" fn yydestruct(
    mut yymsg: *const libc::c_char,
    mut yykind: yysymbol_kind_t,
    mut yyvaluep: *mut YYSTYPE,
    mut pc: *mut parser_control,
) {
    if yymsg.is_null() {
        yymsg = b"Deleting\0" as *const u8 as *const libc::c_char;
    }
}
unsafe extern "C" fn yyerror(mut pc: *const parser_control, mut s: *const libc::c_char) {}
unsafe extern "C" fn mktime_ok(mut tm0: *const tm, mut tm1: *const tm) -> bool {
    if (*tm1).tm_wday < 0 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    return (*tm0).tm_sec ^ (*tm1).tm_sec | (*tm0).tm_min ^ (*tm1).tm_min
        | (*tm0).tm_hour ^ (*tm1).tm_hour | (*tm0).tm_mday ^ (*tm1).tm_mday
        | (*tm0).tm_mon ^ (*tm1).tm_mon | (*tm0).tm_year ^ (*tm1).tm_year == 0;
}
#[no_mangle]
pub unsafe extern "C" fn yyparse(mut pc: *mut parser_control) -> libc::c_int {
    let mut current_block: u64;
    let mut yychar: libc::c_int = 0;
    static mut yyval_default: YYSTYPE = YYSTYPE { intval: 0 };
    let mut yylval: YYSTYPE = yyval_default;
    let mut yynerrs: libc::c_int = 0 as libc::c_int;
    let mut yystate: yy_state_fast_t = 0 as libc::c_int;
    let mut yyerrstatus: libc::c_int = 0 as libc::c_int;
    let mut yystacksize: libc::c_long = 20 as libc::c_int as libc::c_long;
    let mut yyssa: [yy_state_t; 20] = [0; 20];
    let mut yyss: *mut yy_state_t = yyssa.as_mut_ptr();
    let mut yyssp: *mut yy_state_t = yyss;
    let mut yyvsa: [YYSTYPE; 20] = [YYSTYPE { intval: 0 }; 20];
    let mut yyvs: *mut YYSTYPE = yyvsa.as_mut_ptr();
    let mut yyvsp: *mut YYSTYPE = yyvs;
    let mut yyn: libc::c_int = 0;
    let mut yyresult: libc::c_int = 0;
    let mut yytoken: yysymbol_kind_t = YYSYMBOL_YYEMPTY;
    let mut yyval: YYSTYPE = YYSTYPE { intval: 0 };
    let mut yylen: libc::c_int = 0 as libc::c_int;
    yychar = YYEMPTY as libc::c_int;
    's_54: loop {
        (0 as libc::c_int != 0
            && (0 as libc::c_int <= yystate && yystate < 115 as libc::c_int))
            as libc::c_int;
        *yyssp = yystate as yy_state_t;
        if yyss.offset(yystacksize as isize).offset(-(1 as libc::c_int as isize))
            <= yyssp
        {
            let mut yysize: libc::c_long = yyssp.offset_from(yyss) as libc::c_long
                + 1 as libc::c_int as libc::c_long;
            if 20 as libc::c_int as libc::c_long <= yystacksize {
                current_block = 2666335540292013992;
                break;
            }
            yystacksize *= 2 as libc::c_int as libc::c_long;
            if (20 as libc::c_int as libc::c_long) < yystacksize {
                yystacksize = 20 as libc::c_int as libc::c_long;
            }
            let mut yyss1: *mut yy_state_t = yyss;
            let mut yyptr: *mut yyalloc = malloc(
                (yystacksize
                    * (::core::mem::size_of::<yy_state_t>() as libc::c_ulong
                        as libc::c_long
                        + ::core::mem::size_of::<YYSTYPE>() as libc::c_ulong
                            as libc::c_long)
                    + (::core::mem::size_of::<yyalloc>() as libc::c_ulong as libc::c_long
                        - 1 as libc::c_int as libc::c_long)) as libc::c_ulong,
            ) as *mut yyalloc;
            if yyptr.is_null() {
                current_block = 2666335540292013992;
                break;
            }
            let mut yynewbytes: libc::c_long = 0;
            libc::memcpy(
                &mut (*yyptr).yyss_alloc as *mut yy_state_t as *mut libc::c_void,
                yyss as *const libc::c_void,
                (yysize as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<yy_state_t>() as libc::c_ulong)
                    as libc::size_t,
            );
            yyss = &mut (*yyptr).yyss_alloc;
            yynewbytes = yystacksize
                * ::core::mem::size_of::<yy_state_t>() as libc::c_ulong as libc::c_long
                + (::core::mem::size_of::<yyalloc>() as libc::c_ulong as libc::c_long
                    - 1 as libc::c_int as libc::c_long);
            yyptr = yyptr
                .offset(
                    (yynewbytes
                        / ::core::mem::size_of::<yyalloc>() as libc::c_ulong
                            as libc::c_long) as isize,
                );
            let mut yynewbytes_0: libc::c_long = 0;
            libc::memcpy(
                &mut (*yyptr).yyvs_alloc as *mut YYSTYPE as *mut libc::c_void,
                yyvs as *const libc::c_void,
                (yysize as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<YYSTYPE>() as libc::c_ulong)
                    as libc::size_t,
            );
            yyvs = &mut (*yyptr).yyvs_alloc;
            yynewbytes_0 = yystacksize
                * ::core::mem::size_of::<YYSTYPE>() as libc::c_ulong as libc::c_long
                + (::core::mem::size_of::<yyalloc>() as libc::c_ulong as libc::c_long
                    - 1 as libc::c_int as libc::c_long);
            yyptr = yyptr
                .offset(
                    (yynewbytes_0
                        / ::core::mem::size_of::<yyalloc>() as libc::c_ulong
                            as libc::c_long) as isize,
                );
            if yyss1 != yyssa.as_mut_ptr() {
                free(yyss1 as *mut libc::c_void);
            }
            yyssp = yyss.offset(yysize as isize).offset(-(1 as libc::c_int as isize));
            yyvsp = yyvs.offset(yysize as isize).offset(-(1 as libc::c_int as isize));
            if yyss.offset(yystacksize as isize).offset(-(1 as libc::c_int as isize))
                <= yyssp
            {
                current_block = 16695196076034203957;
                break;
            }
        }
        if yystate == 12 as libc::c_int {
            yyresult = 0 as libc::c_int;
            current_block = 7359207444195069603;
            break;
        } else {
            yyn = yypact[yystate as usize] as libc::c_int;
            if yyn == -(91 as libc::c_int) {
                current_block = 3354273763016320579;
            } else {
                if yychar == YYEMPTY as libc::c_int {
                    yychar = yylex(&mut yylval, pc);
                }
                if yychar <= YYEOF as libc::c_int {
                    yychar = YYEOF as libc::c_int;
                    yytoken = YYSYMBOL_YYEOF;
                    current_block = 1924505913685386279;
                } else if yychar == YYerror as libc::c_int {
                    yychar = YYUNDEF as libc::c_int;
                    yytoken = YYSYMBOL_YYerror;
                    current_block = 16915605972616362265;
                } else {
                    yytoken = (if 0 as libc::c_int <= yychar
                        && yychar <= 277 as libc::c_int
                    {
                        yytranslate[yychar as usize] as yysymbol_kind_t as libc::c_int
                    } else {
                        YYSYMBOL_YYUNDEF as libc::c_int
                    }) as yysymbol_kind_t;
                    current_block = 1924505913685386279;
                }
                match current_block {
                    16915605972616362265 => {}
                    _ => {
                        yyn += yytoken as libc::c_int;
                        if yyn < 0 as libc::c_int || (114 as libc::c_int) < yyn
                            || yycheck[yyn as usize] as libc::c_int
                                != yytoken as libc::c_int
                        {
                            current_block = 3354273763016320579;
                        } else {
                            yyn = yytable[yyn as usize] as libc::c_int;
                            if yyn <= 0 as libc::c_int {
                                yyn = -yyn;
                                current_block = 953755897124911993;
                            } else {
                                if yyerrstatus != 0 {
                                    yyerrstatus -= 1;
                                    yyerrstatus;
                                }
                                yystate = yyn;
                                yyvsp = yyvsp.offset(1);
                                *yyvsp = yylval;
                                yychar = YYEMPTY as libc::c_int;
                                current_block = 10346405911354504429;
                            }
                        }
                    }
                }
            }
            match current_block {
                3354273763016320579 => {
                    yyn = yydefact[yystate as usize] as libc::c_int;
                    if yyn == 0 as libc::c_int {
                        yytoken = (if yychar == YYEMPTY as libc::c_int {
                            YYSYMBOL_YYEMPTY as libc::c_int
                        } else if 0 as libc::c_int <= yychar
                            && yychar <= 277 as libc::c_int
                        {
                            yytranslate[yychar as usize] as yysymbol_kind_t
                                as libc::c_int
                        } else {
                            YYSYMBOL_YYUNDEF as libc::c_int
                        }) as yysymbol_kind_t;
                        if yyerrstatus == 0 {
                            yynerrs += 1;
                            yynerrs;
                            yyerror(
                                pc,
                                b"syntax error\0" as *const u8 as *const libc::c_char,
                            );
                        }
                        if yyerrstatus == 3 as libc::c_int {
                            if yychar <= YYEOF as libc::c_int {
                                if yychar == YYEOF as libc::c_int {
                                    current_block = 16695196076034203957;
                                    break;
                                }
                            } else {
                                yydestruct(
                                    b"Error: discarding\0" as *const u8 as *const libc::c_char,
                                    yytoken,
                                    &mut yylval,
                                    pc,
                                );
                                yychar = YYEMPTY as libc::c_int;
                            }
                        }
                        current_block = 16915605972616362265;
                    } else {
                        current_block = 953755897124911993;
                    }
                }
                _ => {}
            }
            match current_block {
                953755897124911993 => {
                    yylen = yyr2[yyn as usize] as libc::c_int;
                    yyval = *yyvsp.offset((1 as libc::c_int - yylen) as isize);
                    match yyn {
                        4 => {
                            (*pc)
                                .seconds = (*yyvsp.offset(0 as libc::c_int as isize))
                                .timespec;
                            (*pc).timespec_seen = 1 as libc::c_int != 0;
                            debug_print_current_time(
                                gettext(
                                    b"number of seconds\0" as *const u8 as *const libc::c_char,
                                ),
                                pc,
                            );
                        }
                        7 => {
                            (*pc).times_seen += 1;
                            (*pc).times_seen;
                            (*pc).dates_seen += 1;
                            (*pc).dates_seen;
                            debug_print_current_time(
                                gettext(b"datetime\0" as *const u8 as *const libc::c_char),
                                pc,
                            );
                        }
                        8 => {
                            (*pc).times_seen += 1;
                            (*pc).times_seen;
                            debug_print_current_time(
                                gettext(b"time\0" as *const u8 as *const libc::c_char),
                                pc,
                            );
                        }
                        9 => {
                            (*pc).local_zones_seen += 1;
                            (*pc).local_zones_seen;
                            debug_print_current_time(
                                gettext(
                                    b"local_zone\0" as *const u8 as *const libc::c_char,
                                ),
                                pc,
                            );
                        }
                        10 => {
                            (*pc).J_zones_seen += 1;
                            (*pc).J_zones_seen;
                            debug_print_current_time(
                                b"J\0" as *const u8 as *const libc::c_char,
                                pc,
                            );
                        }
                        11 => {
                            (*pc).zones_seen += 1;
                            (*pc).zones_seen;
                            debug_print_current_time(
                                gettext(b"zone\0" as *const u8 as *const libc::c_char),
                                pc,
                            );
                        }
                        12 => {
                            (*pc).dates_seen += 1;
                            (*pc).dates_seen;
                            debug_print_current_time(
                                gettext(b"date\0" as *const u8 as *const libc::c_char),
                                pc,
                            );
                        }
                        13 => {
                            (*pc).days_seen += 1;
                            (*pc).days_seen;
                            debug_print_current_time(
                                gettext(b"day\0" as *const u8 as *const libc::c_char),
                                pc,
                            );
                        }
                        14 => {
                            debug_print_relative_time(
                                gettext(b"relative\0" as *const u8 as *const libc::c_char),
                                pc,
                            );
                        }
                        15 => {
                            debug_print_current_time(
                                gettext(b"number\0" as *const u8 as *const libc::c_char),
                                pc,
                            );
                        }
                        16 => {
                            debug_print_relative_time(
                                gettext(b"hybrid\0" as *const u8 as *const libc::c_char),
                                pc,
                            );
                        }
                        19 => {
                            set_hhmmss(
                                pc,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                    .textintval
                                    .value,
                                0 as libc::c_int as intmax_t,
                                0 as libc::c_int as time_t,
                                0 as libc::c_int,
                            );
                            (*pc)
                                .meridian = (*yyvsp.offset(0 as libc::c_int as isize))
                                .intval as libc::c_int;
                        }
                        20 => {
                            set_hhmmss(
                                pc,
                                (*yyvsp.offset(-(3 as libc::c_int) as isize))
                                    .textintval
                                    .value,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                    .textintval
                                    .value,
                                0 as libc::c_int as time_t,
                                0 as libc::c_int,
                            );
                            (*pc)
                                .meridian = (*yyvsp.offset(0 as libc::c_int as isize))
                                .intval as libc::c_int;
                        }
                        21 => {
                            set_hhmmss(
                                pc,
                                (*yyvsp.offset(-(5 as libc::c_int) as isize))
                                    .textintval
                                    .value,
                                (*yyvsp.offset(-(3 as libc::c_int) as isize))
                                    .textintval
                                    .value,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                    .timespec
                                    .tv_sec,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                    .timespec
                                    .tv_nsec as libc::c_int,
                            );
                            (*pc)
                                .meridian = (*yyvsp.offset(0 as libc::c_int as isize))
                                .intval as libc::c_int;
                        }
                        23 => {
                            set_hhmmss(
                                pc,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                    .textintval
                                    .value,
                                0 as libc::c_int as intmax_t,
                                0 as libc::c_int as time_t,
                                0 as libc::c_int,
                            );
                            (*pc).meridian = MER24 as libc::c_int;
                        }
                        24 => {
                            set_hhmmss(
                                pc,
                                (*yyvsp.offset(-(3 as libc::c_int) as isize))
                                    .textintval
                                    .value,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                    .textintval
                                    .value,
                                0 as libc::c_int as time_t,
                                0 as libc::c_int,
                            );
                            (*pc).meridian = MER24 as libc::c_int;
                        }
                        25 => {
                            set_hhmmss(
                                pc,
                                (*yyvsp.offset(-(5 as libc::c_int) as isize))
                                    .textintval
                                    .value,
                                (*yyvsp.offset(-(3 as libc::c_int) as isize))
                                    .textintval
                                    .value,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                    .timespec
                                    .tv_sec,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                    .timespec
                                    .tv_nsec as libc::c_int,
                            );
                            (*pc).meridian = MER24 as libc::c_int;
                        }
                        28 => {
                            (*pc).zones_seen += 1;
                            (*pc).zones_seen;
                            if !time_zone_hhmm(
                                pc,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).textintval,
                                (*yyvsp.offset(0 as libc::c_int as isize)).intval,
                            ) {
                                current_block = 16695196076034203957;
                                break;
                            }
                        }
                        29 => {
                            (*pc)
                                .local_isdst = (*yyvsp.offset(0 as libc::c_int as isize))
                                .intval as libc::c_int;
                        }
                        30 => {
                            (*pc).local_isdst = 1 as libc::c_int;
                            (*pc).dsts_seen += 1;
                            (*pc).dsts_seen;
                        }
                        31 => {
                            (*pc)
                                .time_zone = (*yyvsp.offset(0 as libc::c_int as isize))
                                .intval as libc::c_int;
                        }
                        32 => {
                            (*pc)
                                .time_zone = -(60 as libc::c_int * 60 as libc::c_int
                                * 7 as libc::c_int);
                        }
                        33 => {
                            (*pc)
                                .time_zone = (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                .intval as libc::c_int;
                            if !apply_relative_time(
                                pc,
                                (*yyvsp.offset(0 as libc::c_int as isize)).rel,
                                1 as libc::c_int,
                            ) {
                                current_block = 16695196076034203957;
                                break;
                            }
                            debug_print_relative_time(
                                gettext(b"relative\0" as *const u8 as *const libc::c_char),
                                pc,
                            );
                        }
                        34 => {
                            (*pc)
                                .time_zone = -(60 as libc::c_int * 60 as libc::c_int
                                * 7 as libc::c_int);
                            if !apply_relative_time(
                                pc,
                                (*yyvsp.offset(0 as libc::c_int as isize)).rel,
                                1 as libc::c_int,
                            ) {
                                current_block = 16695196076034203957;
                                break;
                            }
                            debug_print_relative_time(
                                gettext(b"relative\0" as *const u8 as *const libc::c_char),
                                pc,
                            );
                        }
                        35 => {
                            if !time_zone_hhmm(
                                pc,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).textintval,
                                (*yyvsp.offset(0 as libc::c_int as isize)).intval,
                            ) {
                                current_block = 16695196076034203957;
                                break;
                            }
                            let (fresh67, fresh68) = ((*pc).time_zone)
                                .overflowing_add(
                                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).intval.try_into().unwrap(),
                                );
                            *(&mut (*pc).time_zone as *mut libc::c_int) = fresh67;
                            if fresh68 {
                                current_block = 16695196076034203957;
                                break;
                            }
                        }
                        36 => {
                            (*pc)
                                .time_zone = ((*yyvsp.offset(0 as libc::c_int as isize))
                                .intval
                                + (60 as libc::c_int * 60 as libc::c_int) as libc::c_long)
                                as libc::c_int;
                        }
                        37 => {
                            (*pc)
                                .time_zone = ((*yyvsp.offset(-(1 as libc::c_int) as isize))
                                .intval
                                + (60 as libc::c_int * 60 as libc::c_int) as libc::c_long)
                                as libc::c_int;
                        }
                        38 => {
                            (*pc).day_ordinal = 0 as libc::c_int as intmax_t;
                            (*pc)
                                .day_number = (*yyvsp.offset(0 as libc::c_int as isize))
                                .intval as libc::c_int;
                        }
                        39 => {
                            (*pc).day_ordinal = 0 as libc::c_int as intmax_t;
                            (*pc)
                                .day_number = (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                .intval as libc::c_int;
                        }
                        40 => {
                            (*pc)
                                .day_ordinal = (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                .intval;
                            (*pc)
                                .day_number = (*yyvsp.offset(0 as libc::c_int as isize))
                                .intval as libc::c_int;
                            (*pc).debug_ordinal_day_seen = 1 as libc::c_int != 0;
                        }
                        41 => {
                            (*pc)
                                .day_ordinal = (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                .textintval
                                .value;
                            (*pc)
                                .day_number = (*yyvsp.offset(0 as libc::c_int as isize))
                                .intval as libc::c_int;
                            (*pc).debug_ordinal_day_seen = 1 as libc::c_int != 0;
                        }
                        42 => {
                            (*pc)
                                .month = (*yyvsp.offset(-(2 as libc::c_int) as isize))
                                .textintval
                                .value;
                            (*pc)
                                .day = (*yyvsp.offset(0 as libc::c_int as isize))
                                .textintval
                                .value;
                        }
                        43 => {
                            if 4 as libc::c_int as libc::c_long
                                <= (*yyvsp.offset(-(4 as libc::c_int) as isize))
                                    .textintval
                                    .digits
                            {
                                if debugging(pc) {
                                    let mut digits: intmax_t = (*yyvsp
                                        .offset(-(4 as libc::c_int) as isize))
                                        .textintval
                                        .digits;
                                    dbg_printf(
                                        gettext(
                                            b"warning: value %ld has %ld digits. Assuming YYYY/MM/DD\n\0"
                                                as *const u8 as *const libc::c_char,
                                        ),
                                        (*yyvsp.offset(-(4 as libc::c_int) as isize))
                                            .textintval
                                            .value,
                                        digits,
                                    );
                                }
                                (*pc)
                                    .year = (*yyvsp.offset(-(4 as libc::c_int) as isize))
                                    .textintval;
                                (*pc)
                                    .month = (*yyvsp.offset(-(2 as libc::c_int) as isize))
                                    .textintval
                                    .value;
                                (*pc)
                                    .day = (*yyvsp.offset(0 as libc::c_int as isize))
                                    .textintval
                                    .value;
                            } else {
                                if debugging(pc) {
                                    dbg_printf(
                                        gettext(
                                            b"warning: value %ld has less than 4 digits. Assuming MM/DD/YY[YY]\n\0"
                                                as *const u8 as *const libc::c_char,
                                        ),
                                        (*yyvsp.offset(-(4 as libc::c_int) as isize))
                                            .textintval
                                            .value,
                                    );
                                }
                                (*pc)
                                    .month = (*yyvsp.offset(-(4 as libc::c_int) as isize))
                                    .textintval
                                    .value;
                                (*pc)
                                    .day = (*yyvsp.offset(-(2 as libc::c_int) as isize))
                                    .textintval
                                    .value;
                                (*pc)
                                    .year = (*yyvsp.offset(0 as libc::c_int as isize))
                                    .textintval;
                            }
                        }
                        44 => {
                            (*pc)
                                .day = (*yyvsp.offset(-(2 as libc::c_int) as isize))
                                .textintval
                                .value;
                            (*pc)
                                .month = (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                .intval;
                            let (fresh69, fresh70) = (0 as libc::c_int)
                                .overflowing_sub(
                                    (*yyvsp.offset(0 as libc::c_int as isize)).textintval.value.try_into().unwrap(),
                                );
                            *(&mut (*pc).year.value as *mut intmax_t) = fresh69;
                            if fresh70 {
                                current_block = 16695196076034203957;
                                break;
                            }
                            (*pc)
                                .year
                                .digits = (*yyvsp.offset(0 as libc::c_int as isize))
                                .textintval
                                .digits;
                        }
                        45 => {
                            (*pc)
                                .month = (*yyvsp.offset(-(2 as libc::c_int) as isize))
                                .intval;
                            let (fresh71, fresh72) = (0 as libc::c_int)
                                .overflowing_sub(
                                    (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                        .textintval
                                        .value.try_into().unwrap(),
                                );
                            *(&mut (*pc).day as *mut intmax_t) = fresh71;
                            if fresh72 {
                                current_block = 16695196076034203957;
                                break;
                            }
                            let (fresh73, fresh74) = (0 as libc::c_int)
                                .overflowing_sub(
                                    (*yyvsp.offset(0 as libc::c_int as isize)).textintval.value.try_into().unwrap(),
                                );
                            *(&mut (*pc).year.value as *mut intmax_t) = fresh73;
                            if fresh74 {
                                current_block = 16695196076034203957;
                                break;
                            }
                            (*pc)
                                .year
                                .digits = (*yyvsp.offset(0 as libc::c_int as isize))
                                .textintval
                                .digits;
                        }
                        46 => {
                            (*pc)
                                .month = (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                .intval;
                            (*pc)
                                .day = (*yyvsp.offset(0 as libc::c_int as isize))
                                .textintval
                                .value;
                        }
                        47 => {
                            (*pc)
                                .month = (*yyvsp.offset(-(3 as libc::c_int) as isize))
                                .intval;
                            (*pc)
                                .day = (*yyvsp.offset(-(2 as libc::c_int) as isize))
                                .textintval
                                .value;
                            (*pc)
                                .year = (*yyvsp.offset(0 as libc::c_int as isize))
                                .textintval;
                        }
                        48 => {
                            (*pc)
                                .day = (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                .textintval
                                .value;
                            (*pc)
                                .month = (*yyvsp.offset(0 as libc::c_int as isize)).intval;
                        }
                        49 => {
                            (*pc)
                                .day = (*yyvsp.offset(-(2 as libc::c_int) as isize))
                                .textintval
                                .value;
                            (*pc)
                                .month = (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                .intval;
                            (*pc)
                                .year = (*yyvsp.offset(0 as libc::c_int as isize))
                                .textintval;
                        }
                        51 => {
                            (*pc)
                                .year = (*yyvsp.offset(-(2 as libc::c_int) as isize))
                                .textintval;
                            let (fresh75, fresh76) = (0 as libc::c_int)
                                .overflowing_sub(
                                    (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                        .textintval
                                        .value.try_into().unwrap(),
                                );
                            *(&mut (*pc).month as *mut intmax_t) = fresh75;
                            if fresh76 {
                                current_block = 16695196076034203957;
                                break;
                            }
                            let (fresh77, fresh78) = (0 as libc::c_int)
                                .overflowing_sub(
                                    (*yyvsp.offset(0 as libc::c_int as isize)).textintval.value.try_into().unwrap(),
                                );
                            *(&mut (*pc).day as *mut intmax_t) = fresh77;
                            if fresh78 {
                                current_block = 16695196076034203957;
                                break;
                            }
                        }
                        52 => {
                            if !apply_relative_time(
                                pc,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).rel,
                                (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                    as libc::c_int,
                            ) {
                                current_block = 16695196076034203957;
                                break;
                            }
                        }
                        53 => {
                            if !apply_relative_time(
                                pc,
                                (*yyvsp.offset(0 as libc::c_int as isize)).rel,
                                1 as libc::c_int,
                            ) {
                                current_block = 16695196076034203957;
                                break;
                            }
                        }
                        54 => {
                            if !apply_relative_time(
                                pc,
                                (*yyvsp.offset(0 as libc::c_int as isize)).rel,
                                1 as libc::c_int,
                            ) {
                                current_block = 16695196076034203957;
                                break;
                            }
                        }
                        55 => {
                            yyval
                                .rel = {
                                let mut init = relative_time {
                                    year: 0 as libc::c_int as intmax_t,
                                    month: 0 as libc::c_int as intmax_t,
                                    day: 0 as libc::c_int as intmax_t,
                                    hour: 0 as libc::c_int as intmax_t,
                                    minutes: 0 as libc::c_int as intmax_t,
                                    seconds: 0 as libc::c_int as intmax_t,
                                    ns: 0 as libc::c_int,
                                };
                                init
                            };
                            yyval
                                .rel
                                .year = (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                .intval;
                        }
                        56 => {
                            yyval
                                .rel = {
                                let mut init = relative_time {
                                    year: 0 as libc::c_int as intmax_t,
                                    month: 0 as libc::c_int as intmax_t,
                                    day: 0 as libc::c_int as intmax_t,
                                    hour: 0 as libc::c_int as intmax_t,
                                    minutes: 0 as libc::c_int as intmax_t,
                                    seconds: 0 as libc::c_int as intmax_t,
                                    ns: 0 as libc::c_int,
                                };
                                init
                            };
                            yyval
                                .rel
                                .year = (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                .textintval
                                .value;
                        }
                        57 => {
                            yyval
                                .rel = {
                                let mut init = relative_time {
                                    year: 0 as libc::c_int as intmax_t,
                                    month: 0 as libc::c_int as intmax_t,
                                    day: 0 as libc::c_int as intmax_t,
                                    hour: 0 as libc::c_int as intmax_t,
                                    minutes: 0 as libc::c_int as intmax_t,
                                    seconds: 0 as libc::c_int as intmax_t,
                                    ns: 0 as libc::c_int,
                                };
                                init
                            };
                            yyval.rel.year = 1 as libc::c_int as intmax_t;
                        }
                        58 => {
                            yyval
                                .rel = {
                                let mut init = relative_time {
                                    year: 0 as libc::c_int as intmax_t,
                                    month: 0 as libc::c_int as intmax_t,
                                    day: 0 as libc::c_int as intmax_t,
                                    hour: 0 as libc::c_int as intmax_t,
                                    minutes: 0 as libc::c_int as intmax_t,
                                    seconds: 0 as libc::c_int as intmax_t,
                                    ns: 0 as libc::c_int,
                                };
                                init
                            };
                            yyval
                                .rel
                                .month = (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                .intval;
                        }
                        59 => {
                            yyval
                                .rel = {
                                let mut init = relative_time {
                                    year: 0 as libc::c_int as intmax_t,
                                    month: 0 as libc::c_int as intmax_t,
                                    day: 0 as libc::c_int as intmax_t,
                                    hour: 0 as libc::c_int as intmax_t,
                                    minutes: 0 as libc::c_int as intmax_t,
                                    seconds: 0 as libc::c_int as intmax_t,
                                    ns: 0 as libc::c_int,
                                };
                                init
                            };
                            yyval
                                .rel
                                .month = (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                .textintval
                                .value;
                        }
                        60 => {
                            yyval
                                .rel = {
                                let mut init = relative_time {
                                    year: 0 as libc::c_int as intmax_t,
                                    month: 0 as libc::c_int as intmax_t,
                                    day: 0 as libc::c_int as intmax_t,
                                    hour: 0 as libc::c_int as intmax_t,
                                    minutes: 0 as libc::c_int as intmax_t,
                                    seconds: 0 as libc::c_int as intmax_t,
                                    ns: 0 as libc::c_int,
                                };
                                init
                            };
                            yyval.rel.month = 1 as libc::c_int as intmax_t;
                        }
                        61 => {
                            yyval
                                .rel = {
                                let mut init = relative_time {
                                    year: 0 as libc::c_int as intmax_t,
                                    month: 0 as libc::c_int as intmax_t,
                                    day: 0 as libc::c_int as intmax_t,
                                    hour: 0 as libc::c_int as intmax_t,
                                    minutes: 0 as libc::c_int as intmax_t,
                                    seconds: 0 as libc::c_int as intmax_t,
                                    ns: 0 as libc::c_int,
                                };
                                init
                            };
                            if if (0 as libc::c_int as intmax_t)
                                < -(1 as libc::c_int) as intmax_t
                                && ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (*yyvsp.offset(-(1 as libc::c_int) as isize)).intval
                                }) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                && ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                }) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                && (if (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    if (*yyvsp.offset(-(1 as libc::c_int) as isize)).intval
                                        < 0 as libc::c_int as libc::c_long
                                    {
                                        if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                -(1 as libc::c_int) as intmax_t
                                            }) + (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                        }) - 1 as libc::c_int as libc::c_long)
                                            < 0 as libc::c_int as libc::c_long
                                        {
                                            ((*yyvsp.offset(-(1 as libc::c_int) as isize)).intval
                                                < -(1 as libc::c_int) as intmax_t
                                                    / (*yyvsp.offset(0 as libc::c_int as isize)).intval)
                                                as libc::c_int
                                        } else {
                                            ((if (if (if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                            }) - 1 as libc::c_int as libc::c_long)
                                                < 0 as libc::c_int as libc::c_long
                                            {
                                                !(((((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                }) + 1 as libc::c_int as libc::c_long)
                                                    << (::core::mem::size_of::<intmax_t>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    - 1 as libc::c_int as libc::c_long)
                                                    * 2 as libc::c_int as libc::c_long
                                                    + 1 as libc::c_int as libc::c_long)
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                }) + 0 as libc::c_int as libc::c_long
                                            }) < 0 as libc::c_int as libc::c_long
                                            {
                                                ((*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                    < -(if ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                    }) - 1 as libc::c_int as libc::c_long)
                                                        < 0 as libc::c_int as libc::c_long
                                                    {
                                                        ((((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                        }) + 1 as libc::c_int as libc::c_long)
                                                            << (::core::mem::size_of::<intmax_t>() as libc::c_ulong)
                                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                            - 1 as libc::c_int as libc::c_long)
                                                            * 2 as libc::c_int as libc::c_long
                                                            + 1 as libc::c_int as libc::c_long
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                        }) - 1 as libc::c_int as libc::c_long
                                                    })) as libc::c_int
                                            } else {
                                                ((0 as libc::c_int as libc::c_long)
                                                    < (*yyvsp.offset(0 as libc::c_int as isize)).intval)
                                                    as libc::c_int
                                            }) != 0
                                            {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                }) + -(1 as libc::c_int) as intmax_t
                                                    >> (::core::mem::size_of::<intmax_t>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            } else {
                                                -(1 as libc::c_int) as intmax_t
                                                    / -(*yyvsp.offset(0 as libc::c_int as isize)).intval
                                            })
                                                <= -(1 as libc::c_int) as libc::c_long
                                                    - (*yyvsp.offset(-(1 as libc::c_int) as isize)).intval)
                                                as libc::c_int
                                        }
                                    } else {
                                        if (if (if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                            }) + 0 as libc::c_int as intmax_t
                                        }) - 1 as libc::c_int as libc::c_long)
                                            < 0 as libc::c_int as libc::c_long
                                        {
                                            !(((((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                }) + 0 as libc::c_int as intmax_t
                                            }) + 1 as libc::c_int as libc::c_long)
                                                << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int as libc::c_long)
                                                * 2 as libc::c_int as libc::c_long
                                                + 1 as libc::c_int as libc::c_long)
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                }) + 0 as libc::c_int as intmax_t
                                            }) + 0 as libc::c_int as libc::c_long
                                        }) < 0 as libc::c_int as libc::c_long
                                        {
                                            (((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                            }) + 0 as libc::c_int as intmax_t)
                                                < -(if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                    }) + 0 as libc::c_int as intmax_t
                                                }) - 1 as libc::c_int as libc::c_long)
                                                    < 0 as libc::c_int as libc::c_long
                                                {
                                                    ((((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                        }) + 0 as libc::c_int as intmax_t
                                                    }) + 1 as libc::c_int as libc::c_long)
                                                        << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                        - 1 as libc::c_int as libc::c_long)
                                                        * 2 as libc::c_int as libc::c_long
                                                        + 1 as libc::c_int as libc::c_long
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                        }) + 0 as libc::c_int as intmax_t
                                                    }) - 1 as libc::c_int as libc::c_long
                                                })) as libc::c_int
                                        } else {
                                            ((0 as libc::c_int as libc::c_long)
                                                < (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                }) + 0 as libc::c_int as intmax_t) as libc::c_int
                                        }) != 0
                                            && (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                == -(1 as libc::c_int) as libc::c_long
                                        {
                                            if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).intval
                                            }) - 1 as libc::c_int as libc::c_long)
                                                < 0 as libc::c_int as libc::c_long
                                            {
                                                ((0 as libc::c_int as libc::c_long)
                                                    < (*yyvsp.offset(-(1 as libc::c_int) as isize)).intval
                                                        + 0 as libc::c_int as intmax_t) as libc::c_int
                                            } else {
                                                ((0 as libc::c_int as libc::c_long)
                                                    < (*yyvsp.offset(-(1 as libc::c_int) as isize)).intval
                                                    && (-(1 as libc::c_int) as libc::c_long
                                                        - 0 as libc::c_int as intmax_t)
                                                        < (*yyvsp.offset(-(1 as libc::c_int) as isize)).intval
                                                            - 1 as libc::c_int as libc::c_long) as libc::c_int
                                            }
                                        } else {
                                            (0 as libc::c_int as intmax_t
                                                / (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                < (*yyvsp.offset(-(1 as libc::c_int) as isize)).intval)
                                                as libc::c_int
                                        }
                                    }
                                } else {
                                    if (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                        == 0 as libc::c_int as libc::c_long
                                    {
                                        0 as libc::c_int
                                    } else {
                                        if (*yyvsp.offset(-(1 as libc::c_int) as isize)).intval
                                            < 0 as libc::c_int as libc::c_long
                                        {
                                            if (if (if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (*yyvsp.offset(-(1 as libc::c_int) as isize)).intval
                                                }) + 0 as libc::c_int as intmax_t
                                            }) - 1 as libc::c_int as libc::c_long)
                                                < 0 as libc::c_int as libc::c_long
                                            {
                                                !(((((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        (*yyvsp.offset(-(1 as libc::c_int) as isize)).intval
                                                    }) + 0 as libc::c_int as intmax_t
                                                }) + 1 as libc::c_int as libc::c_long)
                                                    << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    - 1 as libc::c_int as libc::c_long)
                                                    * 2 as libc::c_int as libc::c_long
                                                    + 1 as libc::c_int as libc::c_long)
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        (*yyvsp.offset(-(1 as libc::c_int) as isize)).intval
                                                    }) + 0 as libc::c_int as intmax_t
                                                }) + 0 as libc::c_int as libc::c_long
                                            }) < 0 as libc::c_int as libc::c_long
                                            {
                                                (((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (*yyvsp.offset(-(1 as libc::c_int) as isize)).intval
                                                }) + 0 as libc::c_int as intmax_t)
                                                    < -(if ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            (*yyvsp.offset(-(1 as libc::c_int) as isize)).intval
                                                        }) + 0 as libc::c_int as intmax_t
                                                    }) - 1 as libc::c_int as libc::c_long)
                                                        < 0 as libc::c_int as libc::c_long
                                                    {
                                                        ((((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_long
                                                            } else {
                                                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).intval
                                                            }) + 0 as libc::c_int as intmax_t
                                                        }) + 1 as libc::c_int as libc::c_long)
                                                            << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                            - 1 as libc::c_int as libc::c_long)
                                                            * 2 as libc::c_int as libc::c_long
                                                            + 1 as libc::c_int as libc::c_long
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_long
                                                            } else {
                                                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).intval
                                                            }) + 0 as libc::c_int as intmax_t
                                                        }) - 1 as libc::c_int as libc::c_long
                                                    })) as libc::c_int
                                            } else {
                                                ((0 as libc::c_int as libc::c_long)
                                                    < (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        (*yyvsp.offset(-(1 as libc::c_int) as isize)).intval
                                                    }) + 0 as libc::c_int as intmax_t) as libc::c_int
                                            }) != 0
                                                && (*yyvsp.offset(-(1 as libc::c_int) as isize)).intval
                                                    == -(1 as libc::c_int) as libc::c_long
                                            {
                                                if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                }) - 1 as libc::c_int as libc::c_long)
                                                    < 0 as libc::c_int as libc::c_long
                                                {
                                                    ((0 as libc::c_int as libc::c_long)
                                                        < (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                            + 0 as libc::c_int as intmax_t) as libc::c_int
                                                } else {
                                                    ((-(1 as libc::c_int) as libc::c_long
                                                        - 0 as libc::c_int as intmax_t)
                                                        < (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                            - 1 as libc::c_int as libc::c_long) as libc::c_int
                                                }
                                            } else {
                                                (0 as libc::c_int as intmax_t
                                                    / (*yyvsp.offset(-(1 as libc::c_int) as isize)).intval
                                                    < (*yyvsp.offset(0 as libc::c_int as isize)).intval)
                                                    as libc::c_int
                                            }
                                        } else {
                                            (-(1 as libc::c_int) as intmax_t
                                                / (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                < (*yyvsp.offset(-(1 as libc::c_int) as isize)).intval)
                                                as libc::c_int
                                        }
                                    }
                                }) != 0
                            {
                                let (fresh83, fresh84) = ((*yyvsp
                                    .offset(-(1 as libc::c_int) as isize))
                                    .intval)
                                    .overflowing_mul(
                                        (*yyvsp.offset(0 as libc::c_int as isize)).intval,
                                    );
                                *(&mut yyval.rel.day as *mut intmax_t) = fresh83;
                                1 as libc::c_int
                            } else {
                                let (fresh85, fresh86) = ((*yyvsp
                                    .offset(-(1 as libc::c_int) as isize))
                                    .intval)
                                    .overflowing_mul(
                                        (*yyvsp.offset(0 as libc::c_int as isize)).intval,
                                    );
                                *(&mut yyval.rel.day as *mut intmax_t) = fresh85;
                                fresh86 as libc::c_int
                            } != 0
                            {
                                current_block = 16695196076034203957;
                                break;
                            }
                        }
                        62 => {
                            yyval
                                .rel = {
                                let mut init = relative_time {
                                    year: 0 as libc::c_int as intmax_t,
                                    month: 0 as libc::c_int as intmax_t,
                                    day: 0 as libc::c_int as intmax_t,
                                    hour: 0 as libc::c_int as intmax_t,
                                    minutes: 0 as libc::c_int as intmax_t,
                                    seconds: 0 as libc::c_int as intmax_t,
                                    ns: 0 as libc::c_int,
                                };
                                init
                            };
                            if if (0 as libc::c_int as intmax_t)
                                < -(1 as libc::c_int) as intmax_t
                                && ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                        .textintval
                                        .value
                                }) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                && ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                }) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                && (if (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    if (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                        .textintval
                                        .value < 0 as libc::c_int as libc::c_long
                                    {
                                        if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                -(1 as libc::c_int) as intmax_t
                                            }) + (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                        }) - 1 as libc::c_int as libc::c_long)
                                            < 0 as libc::c_int as libc::c_long
                                        {
                                            ((*yyvsp.offset(-(1 as libc::c_int) as isize))
                                                .textintval
                                                .value
                                                < -(1 as libc::c_int) as intmax_t
                                                    / (*yyvsp.offset(0 as libc::c_int as isize)).intval)
                                                as libc::c_int
                                        } else {
                                            ((if (if (if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                            }) - 1 as libc::c_int as libc::c_long)
                                                < 0 as libc::c_int as libc::c_long
                                            {
                                                !(((((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                }) + 1 as libc::c_int as libc::c_long)
                                                    << (::core::mem::size_of::<intmax_t>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    - 1 as libc::c_int as libc::c_long)
                                                    * 2 as libc::c_int as libc::c_long
                                                    + 1 as libc::c_int as libc::c_long)
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                }) + 0 as libc::c_int as libc::c_long
                                            }) < 0 as libc::c_int as libc::c_long
                                            {
                                                ((*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                    < -(if ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                    }) - 1 as libc::c_int as libc::c_long)
                                                        < 0 as libc::c_int as libc::c_long
                                                    {
                                                        ((((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                        }) + 1 as libc::c_int as libc::c_long)
                                                            << (::core::mem::size_of::<intmax_t>() as libc::c_ulong)
                                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                            - 1 as libc::c_int as libc::c_long)
                                                            * 2 as libc::c_int as libc::c_long
                                                            + 1 as libc::c_int as libc::c_long
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                        }) - 1 as libc::c_int as libc::c_long
                                                    })) as libc::c_int
                                            } else {
                                                ((0 as libc::c_int as libc::c_long)
                                                    < (*yyvsp.offset(0 as libc::c_int as isize)).intval)
                                                    as libc::c_int
                                            }) != 0
                                            {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                }) + -(1 as libc::c_int) as intmax_t
                                                    >> (::core::mem::size_of::<intmax_t>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            } else {
                                                -(1 as libc::c_int) as intmax_t
                                                    / -(*yyvsp.offset(0 as libc::c_int as isize)).intval
                                            })
                                                <= -(1 as libc::c_int) as libc::c_long
                                                    - (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                                        .textintval
                                                        .value) as libc::c_int
                                        }
                                    } else {
                                        if (if (if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                            }) + 0 as libc::c_int as intmax_t
                                        }) - 1 as libc::c_int as libc::c_long)
                                            < 0 as libc::c_int as libc::c_long
                                        {
                                            !(((((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                }) + 0 as libc::c_int as intmax_t
                                            }) + 1 as libc::c_int as libc::c_long)
                                                << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int as libc::c_long)
                                                * 2 as libc::c_int as libc::c_long
                                                + 1 as libc::c_int as libc::c_long)
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                }) + 0 as libc::c_int as intmax_t
                                            }) + 0 as libc::c_int as libc::c_long
                                        }) < 0 as libc::c_int as libc::c_long
                                        {
                                            (((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                            }) + 0 as libc::c_int as intmax_t)
                                                < -(if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                    }) + 0 as libc::c_int as intmax_t
                                                }) - 1 as libc::c_int as libc::c_long)
                                                    < 0 as libc::c_int as libc::c_long
                                                {
                                                    ((((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                        }) + 0 as libc::c_int as intmax_t
                                                    }) + 1 as libc::c_int as libc::c_long)
                                                        << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                        - 1 as libc::c_int as libc::c_long)
                                                        * 2 as libc::c_int as libc::c_long
                                                        + 1 as libc::c_int as libc::c_long
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                        }) + 0 as libc::c_int as intmax_t
                                                    }) - 1 as libc::c_int as libc::c_long
                                                })) as libc::c_int
                                        } else {
                                            ((0 as libc::c_int as libc::c_long)
                                                < (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                }) + 0 as libc::c_int as intmax_t) as libc::c_int
                                        }) != 0
                                            && (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                == -(1 as libc::c_int) as libc::c_long
                                        {
                                            if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                                    .textintval
                                                    .value
                                            }) - 1 as libc::c_int as libc::c_long)
                                                < 0 as libc::c_int as libc::c_long
                                            {
                                                ((0 as libc::c_int as libc::c_long)
                                                    < (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                                        .textintval
                                                        .value + 0 as libc::c_int as intmax_t) as libc::c_int
                                            } else {
                                                ((0 as libc::c_int as libc::c_long)
                                                    < (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                                        .textintval
                                                        .value
                                                    && (-(1 as libc::c_int) as libc::c_long
                                                        - 0 as libc::c_int as intmax_t)
                                                        < (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                                            .textintval
                                                            .value - 1 as libc::c_int as libc::c_long) as libc::c_int
                                            }
                                        } else {
                                            (0 as libc::c_int as intmax_t
                                                / (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                < (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                                    .textintval
                                                    .value) as libc::c_int
                                        }
                                    }
                                } else {
                                    if (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                        == 0 as libc::c_int as libc::c_long
                                    {
                                        0 as libc::c_int
                                    } else {
                                        if (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                            .textintval
                                            .value < 0 as libc::c_int as libc::c_long
                                        {
                                            if (if (if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                                        .textintval
                                                        .value
                                                }) + 0 as libc::c_int as intmax_t
                                            }) - 1 as libc::c_int as libc::c_long)
                                                < 0 as libc::c_int as libc::c_long
                                            {
                                                !(((((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                                            .textintval
                                                            .value
                                                    }) + 0 as libc::c_int as intmax_t
                                                }) + 1 as libc::c_int as libc::c_long)
                                                    << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    - 1 as libc::c_int as libc::c_long)
                                                    * 2 as libc::c_int as libc::c_long
                                                    + 1 as libc::c_int as libc::c_long)
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                                            .textintval
                                                            .value
                                                    }) + 0 as libc::c_int as intmax_t
                                                }) + 0 as libc::c_int as libc::c_long
                                            }) < 0 as libc::c_int as libc::c_long
                                            {
                                                (((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                                        .textintval
                                                        .value
                                                }) + 0 as libc::c_int as intmax_t)
                                                    < -(if ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                                                .textintval
                                                                .value
                                                        }) + 0 as libc::c_int as intmax_t
                                                    }) - 1 as libc::c_int as libc::c_long)
                                                        < 0 as libc::c_int as libc::c_long
                                                    {
                                                        ((((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_long
                                                            } else {
                                                                (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                                                    .textintval
                                                                    .value
                                                            }) + 0 as libc::c_int as intmax_t
                                                        }) + 1 as libc::c_int as libc::c_long)
                                                            << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                            - 1 as libc::c_int as libc::c_long)
                                                            * 2 as libc::c_int as libc::c_long
                                                            + 1 as libc::c_int as libc::c_long
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_long
                                                            } else {
                                                                (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                                                    .textintval
                                                                    .value
                                                            }) + 0 as libc::c_int as intmax_t
                                                        }) - 1 as libc::c_int as libc::c_long
                                                    })) as libc::c_int
                                            } else {
                                                ((0 as libc::c_int as libc::c_long)
                                                    < (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                                            .textintval
                                                            .value
                                                    }) + 0 as libc::c_int as intmax_t) as libc::c_int
                                            }) != 0
                                                && (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                                    .textintval
                                                    .value == -(1 as libc::c_int) as libc::c_long
                                            {
                                                if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                }) - 1 as libc::c_int as libc::c_long)
                                                    < 0 as libc::c_int as libc::c_long
                                                {
                                                    ((0 as libc::c_int as libc::c_long)
                                                        < (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                            + 0 as libc::c_int as intmax_t) as libc::c_int
                                                } else {
                                                    ((-(1 as libc::c_int) as libc::c_long
                                                        - 0 as libc::c_int as intmax_t)
                                                        < (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                            - 1 as libc::c_int as libc::c_long) as libc::c_int
                                                }
                                            } else {
                                                (0 as libc::c_int as intmax_t
                                                    / (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                                        .textintval
                                                        .value < (*yyvsp.offset(0 as libc::c_int as isize)).intval)
                                                    as libc::c_int
                                            }
                                        } else {
                                            (-(1 as libc::c_int) as intmax_t
                                                / (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                < (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                                    .textintval
                                                    .value) as libc::c_int
                                        }
                                    }
                                }) != 0
                            {
                                let (fresh91, fresh92) = ((*yyvsp
                                    .offset(-(1 as libc::c_int) as isize))
                                    .textintval
                                    .value)
                                    .overflowing_mul(
                                        (*yyvsp.offset(0 as libc::c_int as isize)).intval,
                                    );
                                *(&mut yyval.rel.day as *mut intmax_t) = fresh91;
                                1 as libc::c_int
                            } else {
                                let (fresh93, fresh94) = ((*yyvsp
                                    .offset(-(1 as libc::c_int) as isize))
                                    .textintval
                                    .value)
                                    .overflowing_mul(
                                        (*yyvsp.offset(0 as libc::c_int as isize)).intval,
                                    );
                                *(&mut yyval.rel.day as *mut intmax_t) = fresh93;
                                fresh94 as libc::c_int
                            } != 0
                            {
                                current_block = 16695196076034203957;
                                break;
                            }
                        }
                        63 => {
                            yyval
                                .rel = {
                                let mut init = relative_time {
                                    year: 0 as libc::c_int as intmax_t,
                                    month: 0 as libc::c_int as intmax_t,
                                    day: 0 as libc::c_int as intmax_t,
                                    hour: 0 as libc::c_int as intmax_t,
                                    minutes: 0 as libc::c_int as intmax_t,
                                    seconds: 0 as libc::c_int as intmax_t,
                                    ns: 0 as libc::c_int,
                                };
                                init
                            };
                            yyval
                                .rel
                                .day = (*yyvsp.offset(0 as libc::c_int as isize)).intval;
                        }
                        64 => {
                            yyval
                                .rel = {
                                let mut init = relative_time {
                                    year: 0 as libc::c_int as intmax_t,
                                    month: 0 as libc::c_int as intmax_t,
                                    day: 0 as libc::c_int as intmax_t,
                                    hour: 0 as libc::c_int as intmax_t,
                                    minutes: 0 as libc::c_int as intmax_t,
                                    seconds: 0 as libc::c_int as intmax_t,
                                    ns: 0 as libc::c_int,
                                };
                                init
                            };
                            yyval
                                .rel
                                .hour = (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                .intval;
                        }
                        65 => {
                            yyval
                                .rel = {
                                let mut init = relative_time {
                                    year: 0 as libc::c_int as intmax_t,
                                    month: 0 as libc::c_int as intmax_t,
                                    day: 0 as libc::c_int as intmax_t,
                                    hour: 0 as libc::c_int as intmax_t,
                                    minutes: 0 as libc::c_int as intmax_t,
                                    seconds: 0 as libc::c_int as intmax_t,
                                    ns: 0 as libc::c_int,
                                };
                                init
                            };
                            yyval
                                .rel
                                .hour = (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                .textintval
                                .value;
                        }
                        66 => {
                            yyval
                                .rel = {
                                let mut init = relative_time {
                                    year: 0 as libc::c_int as intmax_t,
                                    month: 0 as libc::c_int as intmax_t,
                                    day: 0 as libc::c_int as intmax_t,
                                    hour: 0 as libc::c_int as intmax_t,
                                    minutes: 0 as libc::c_int as intmax_t,
                                    seconds: 0 as libc::c_int as intmax_t,
                                    ns: 0 as libc::c_int,
                                };
                                init
                            };
                            yyval.rel.hour = 1 as libc::c_int as intmax_t;
                        }
                        67 => {
                            yyval
                                .rel = {
                                let mut init = relative_time {
                                    year: 0 as libc::c_int as intmax_t,
                                    month: 0 as libc::c_int as intmax_t,
                                    day: 0 as libc::c_int as intmax_t,
                                    hour: 0 as libc::c_int as intmax_t,
                                    minutes: 0 as libc::c_int as intmax_t,
                                    seconds: 0 as libc::c_int as intmax_t,
                                    ns: 0 as libc::c_int,
                                };
                                init
                            };
                            yyval
                                .rel
                                .minutes = (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                .intval;
                        }
                        68 => {
                            yyval
                                .rel = {
                                let mut init = relative_time {
                                    year: 0 as libc::c_int as intmax_t,
                                    month: 0 as libc::c_int as intmax_t,
                                    day: 0 as libc::c_int as intmax_t,
                                    hour: 0 as libc::c_int as intmax_t,
                                    minutes: 0 as libc::c_int as intmax_t,
                                    seconds: 0 as libc::c_int as intmax_t,
                                    ns: 0 as libc::c_int,
                                };
                                init
                            };
                            yyval
                                .rel
                                .minutes = (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                .textintval
                                .value;
                        }
                        69 => {
                            yyval
                                .rel = {
                                let mut init = relative_time {
                                    year: 0 as libc::c_int as intmax_t,
                                    month: 0 as libc::c_int as intmax_t,
                                    day: 0 as libc::c_int as intmax_t,
                                    hour: 0 as libc::c_int as intmax_t,
                                    minutes: 0 as libc::c_int as intmax_t,
                                    seconds: 0 as libc::c_int as intmax_t,
                                    ns: 0 as libc::c_int,
                                };
                                init
                            };
                            yyval.rel.minutes = 1 as libc::c_int as intmax_t;
                        }
                        70 => {
                            yyval
                                .rel = {
                                let mut init = relative_time {
                                    year: 0 as libc::c_int as intmax_t,
                                    month: 0 as libc::c_int as intmax_t,
                                    day: 0 as libc::c_int as intmax_t,
                                    hour: 0 as libc::c_int as intmax_t,
                                    minutes: 0 as libc::c_int as intmax_t,
                                    seconds: 0 as libc::c_int as intmax_t,
                                    ns: 0 as libc::c_int,
                                };
                                init
                            };
                            yyval
                                .rel
                                .seconds = (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                .intval;
                        }
                        71 => {
                            yyval
                                .rel = {
                                let mut init = relative_time {
                                    year: 0 as libc::c_int as intmax_t,
                                    month: 0 as libc::c_int as intmax_t,
                                    day: 0 as libc::c_int as intmax_t,
                                    hour: 0 as libc::c_int as intmax_t,
                                    minutes: 0 as libc::c_int as intmax_t,
                                    seconds: 0 as libc::c_int as intmax_t,
                                    ns: 0 as libc::c_int,
                                };
                                init
                            };
                            yyval
                                .rel
                                .seconds = (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                .textintval
                                .value;
                        }
                        72 => {
                            yyval
                                .rel = {
                                let mut init = relative_time {
                                    year: 0 as libc::c_int as intmax_t,
                                    month: 0 as libc::c_int as intmax_t,
                                    day: 0 as libc::c_int as intmax_t,
                                    hour: 0 as libc::c_int as intmax_t,
                                    minutes: 0 as libc::c_int as intmax_t,
                                    seconds: 0 as libc::c_int as intmax_t,
                                    ns: 0 as libc::c_int,
                                };
                                init
                            };
                            yyval
                                .rel
                                .seconds = (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                .timespec
                                .tv_sec;
                            yyval
                                .rel
                                .ns = (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                .timespec
                                .tv_nsec as libc::c_int;
                        }
                        73 => {
                            yyval
                                .rel = {
                                let mut init = relative_time {
                                    year: 0 as libc::c_int as intmax_t,
                                    month: 0 as libc::c_int as intmax_t,
                                    day: 0 as libc::c_int as intmax_t,
                                    hour: 0 as libc::c_int as intmax_t,
                                    minutes: 0 as libc::c_int as intmax_t,
                                    seconds: 0 as libc::c_int as intmax_t,
                                    ns: 0 as libc::c_int,
                                };
                                init
                            };
                            yyval
                                .rel
                                .seconds = (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                .timespec
                                .tv_sec;
                            yyval
                                .rel
                                .ns = (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                .timespec
                                .tv_nsec as libc::c_int;
                        }
                        74 => {
                            yyval
                                .rel = {
                                let mut init = relative_time {
                                    year: 0 as libc::c_int as intmax_t,
                                    month: 0 as libc::c_int as intmax_t,
                                    day: 0 as libc::c_int as intmax_t,
                                    hour: 0 as libc::c_int as intmax_t,
                                    minutes: 0 as libc::c_int as intmax_t,
                                    seconds: 0 as libc::c_int as intmax_t,
                                    ns: 0 as libc::c_int,
                                };
                                init
                            };
                            yyval.rel.seconds = 1 as libc::c_int as intmax_t;
                        }
                        76 => {
                            yyval
                                .rel = {
                                let mut init = relative_time {
                                    year: 0 as libc::c_int as intmax_t,
                                    month: 0 as libc::c_int as intmax_t,
                                    day: 0 as libc::c_int as intmax_t,
                                    hour: 0 as libc::c_int as intmax_t,
                                    minutes: 0 as libc::c_int as intmax_t,
                                    seconds: 0 as libc::c_int as intmax_t,
                                    ns: 0 as libc::c_int,
                                };
                                init
                            };
                            yyval
                                .rel
                                .year = (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                .textintval
                                .value;
                        }
                        77 => {
                            yyval
                                .rel = {
                                let mut init = relative_time {
                                    year: 0 as libc::c_int as intmax_t,
                                    month: 0 as libc::c_int as intmax_t,
                                    day: 0 as libc::c_int as intmax_t,
                                    hour: 0 as libc::c_int as intmax_t,
                                    minutes: 0 as libc::c_int as intmax_t,
                                    seconds: 0 as libc::c_int as intmax_t,
                                    ns: 0 as libc::c_int,
                                };
                                init
                            };
                            yyval
                                .rel
                                .month = (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                .textintval
                                .value;
                        }
                        78 => {
                            yyval
                                .rel = {
                                let mut init = relative_time {
                                    year: 0 as libc::c_int as intmax_t,
                                    month: 0 as libc::c_int as intmax_t,
                                    day: 0 as libc::c_int as intmax_t,
                                    hour: 0 as libc::c_int as intmax_t,
                                    minutes: 0 as libc::c_int as intmax_t,
                                    seconds: 0 as libc::c_int as intmax_t,
                                    ns: 0 as libc::c_int,
                                };
                                init
                            };
                            if if (0 as libc::c_int as intmax_t)
                                < -(1 as libc::c_int) as intmax_t
                                && ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                        .textintval
                                        .value
                                }) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                && ((if 1 as libc::c_int != 0 {
                                    0 as libc::c_int as libc::c_long
                                } else {
                                    (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                }) - 1 as libc::c_int as libc::c_long)
                                    < 0 as libc::c_int as libc::c_long
                                && (if (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                    < 0 as libc::c_int as libc::c_long
                                {
                                    if (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                        .textintval
                                        .value < 0 as libc::c_int as libc::c_long
                                    {
                                        if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                -(1 as libc::c_int) as intmax_t
                                            }) + (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                        }) - 1 as libc::c_int as libc::c_long)
                                            < 0 as libc::c_int as libc::c_long
                                        {
                                            ((*yyvsp.offset(-(1 as libc::c_int) as isize))
                                                .textintval
                                                .value
                                                < -(1 as libc::c_int) as intmax_t
                                                    / (*yyvsp.offset(0 as libc::c_int as isize)).intval)
                                                as libc::c_int
                                        } else {
                                            ((if (if (if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                            }) - 1 as libc::c_int as libc::c_long)
                                                < 0 as libc::c_int as libc::c_long
                                            {
                                                !(((((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                }) + 1 as libc::c_int as libc::c_long)
                                                    << (::core::mem::size_of::<intmax_t>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    - 1 as libc::c_int as libc::c_long)
                                                    * 2 as libc::c_int as libc::c_long
                                                    + 1 as libc::c_int as libc::c_long)
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                }) + 0 as libc::c_int as libc::c_long
                                            }) < 0 as libc::c_int as libc::c_long
                                            {
                                                ((*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                    < -(if ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                    }) - 1 as libc::c_int as libc::c_long)
                                                        < 0 as libc::c_int as libc::c_long
                                                    {
                                                        ((((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                        }) + 1 as libc::c_int as libc::c_long)
                                                            << (::core::mem::size_of::<intmax_t>() as libc::c_ulong)
                                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                            - 1 as libc::c_int as libc::c_long)
                                                            * 2 as libc::c_int as libc::c_long
                                                            + 1 as libc::c_int as libc::c_long
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                        }) - 1 as libc::c_int as libc::c_long
                                                    })) as libc::c_int
                                            } else {
                                                ((0 as libc::c_int as libc::c_long)
                                                    < (*yyvsp.offset(0 as libc::c_int as isize)).intval)
                                                    as libc::c_int
                                            }) != 0
                                            {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                }) + -(1 as libc::c_int) as intmax_t
                                                    >> (::core::mem::size_of::<intmax_t>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                            } else {
                                                -(1 as libc::c_int) as intmax_t
                                                    / -(*yyvsp.offset(0 as libc::c_int as isize)).intval
                                            })
                                                <= -(1 as libc::c_int) as libc::c_long
                                                    - (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                                        .textintval
                                                        .value) as libc::c_int
                                        }
                                    } else {
                                        if (if (if ((if 1 as libc::c_int != 0 {
                                            0 as libc::c_int as libc::c_long
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                            }) + 0 as libc::c_int as intmax_t
                                        }) - 1 as libc::c_int as libc::c_long)
                                            < 0 as libc::c_int as libc::c_long
                                        {
                                            !(((((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                }) + 0 as libc::c_int as intmax_t
                                            }) + 1 as libc::c_int as libc::c_long)
                                                << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                - 1 as libc::c_int as libc::c_long)
                                                * 2 as libc::c_int as libc::c_long
                                                + 1 as libc::c_int as libc::c_long)
                                        } else {
                                            (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                }) + 0 as libc::c_int as intmax_t
                                            }) + 0 as libc::c_int as libc::c_long
                                        }) < 0 as libc::c_int as libc::c_long
                                        {
                                            (((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                            }) + 0 as libc::c_int as intmax_t)
                                                < -(if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                    }) + 0 as libc::c_int as intmax_t
                                                }) - 1 as libc::c_int as libc::c_long)
                                                    < 0 as libc::c_int as libc::c_long
                                                {
                                                    ((((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                        }) + 0 as libc::c_int as intmax_t
                                                    }) + 1 as libc::c_int as libc::c_long)
                                                        << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                        - 1 as libc::c_int as libc::c_long)
                                                        * 2 as libc::c_int as libc::c_long
                                                        + 1 as libc::c_int as libc::c_long
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                        }) + 0 as libc::c_int as intmax_t
                                                    }) - 1 as libc::c_int as libc::c_long
                                                })) as libc::c_int
                                        } else {
                                            ((0 as libc::c_int as libc::c_long)
                                                < (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                }) + 0 as libc::c_int as intmax_t) as libc::c_int
                                        }) != 0
                                            && (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                == -(1 as libc::c_int) as libc::c_long
                                        {
                                            if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                                    .textintval
                                                    .value
                                            }) - 1 as libc::c_int as libc::c_long)
                                                < 0 as libc::c_int as libc::c_long
                                            {
                                                ((0 as libc::c_int as libc::c_long)
                                                    < (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                                        .textintval
                                                        .value + 0 as libc::c_int as intmax_t) as libc::c_int
                                            } else {
                                                ((0 as libc::c_int as libc::c_long)
                                                    < (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                                        .textintval
                                                        .value
                                                    && (-(1 as libc::c_int) as libc::c_long
                                                        - 0 as libc::c_int as intmax_t)
                                                        < (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                                            .textintval
                                                            .value - 1 as libc::c_int as libc::c_long) as libc::c_int
                                            }
                                        } else {
                                            (0 as libc::c_int as intmax_t
                                                / (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                < (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                                    .textintval
                                                    .value) as libc::c_int
                                        }
                                    }
                                } else {
                                    if (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                        == 0 as libc::c_int as libc::c_long
                                    {
                                        0 as libc::c_int
                                    } else {
                                        if (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                            .textintval
                                            .value < 0 as libc::c_int as libc::c_long
                                        {
                                            if (if (if ((if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_long
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                                        .textintval
                                                        .value
                                                }) + 0 as libc::c_int as intmax_t
                                            }) - 1 as libc::c_int as libc::c_long)
                                                < 0 as libc::c_int as libc::c_long
                                            {
                                                !(((((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                                            .textintval
                                                            .value
                                                    }) + 0 as libc::c_int as intmax_t
                                                }) + 1 as libc::c_int as libc::c_long)
                                                    << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    - 1 as libc::c_int as libc::c_long)
                                                    * 2 as libc::c_int as libc::c_long
                                                    + 1 as libc::c_int as libc::c_long)
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                                            .textintval
                                                            .value
                                                    }) + 0 as libc::c_int as intmax_t
                                                }) + 0 as libc::c_int as libc::c_long
                                            }) < 0 as libc::c_int as libc::c_long
                                            {
                                                (((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                                        .textintval
                                                        .value
                                                }) + 0 as libc::c_int as intmax_t)
                                                    < -(if ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                                                .textintval
                                                                .value
                                                        }) + 0 as libc::c_int as intmax_t
                                                    }) - 1 as libc::c_int as libc::c_long)
                                                        < 0 as libc::c_int as libc::c_long
                                                    {
                                                        ((((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_long
                                                            } else {
                                                                (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                                                    .textintval
                                                                    .value
                                                            }) + 0 as libc::c_int as intmax_t
                                                        }) + 1 as libc::c_int as libc::c_long)
                                                            << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                            - 1 as libc::c_int as libc::c_long)
                                                            * 2 as libc::c_int as libc::c_long
                                                            + 1 as libc::c_int as libc::c_long
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_long
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_long
                                                            } else {
                                                                (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                                                    .textintval
                                                                    .value
                                                            }) + 0 as libc::c_int as intmax_t
                                                        }) - 1 as libc::c_int as libc::c_long
                                                    })) as libc::c_int
                                            } else {
                                                ((0 as libc::c_int as libc::c_long)
                                                    < (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                                            .textintval
                                                            .value
                                                    }) + 0 as libc::c_int as intmax_t) as libc::c_int
                                            }) != 0
                                                && (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                                    .textintval
                                                    .value == -(1 as libc::c_int) as libc::c_long
                                            {
                                                if ((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_long
                                                } else {
                                                    (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                }) - 1 as libc::c_int as libc::c_long)
                                                    < 0 as libc::c_int as libc::c_long
                                                {
                                                    ((0 as libc::c_int as libc::c_long)
                                                        < (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                            + 0 as libc::c_int as intmax_t) as libc::c_int
                                                } else {
                                                    ((-(1 as libc::c_int) as libc::c_long
                                                        - 0 as libc::c_int as intmax_t)
                                                        < (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                            - 1 as libc::c_int as libc::c_long) as libc::c_int
                                                }
                                            } else {
                                                (0 as libc::c_int as intmax_t
                                                    / (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                                        .textintval
                                                        .value < (*yyvsp.offset(0 as libc::c_int as isize)).intval)
                                                    as libc::c_int
                                            }
                                        } else {
                                            (-(1 as libc::c_int) as intmax_t
                                                / (*yyvsp.offset(0 as libc::c_int as isize)).intval
                                                < (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                                    .textintval
                                                    .value) as libc::c_int
                                        }
                                    }
                                }) != 0
                            {
                                let (fresh99, fresh100) = ((*yyvsp
                                    .offset(-(1 as libc::c_int) as isize))
                                    .textintval
                                    .value)
                                    .overflowing_mul(
                                        (*yyvsp.offset(0 as libc::c_int as isize)).intval,
                                    );
                                *(&mut yyval.rel.day as *mut intmax_t) = fresh99;
                                1 as libc::c_int
                            } else {
                                let (fresh101, fresh102) = ((*yyvsp
                                    .offset(-(1 as libc::c_int) as isize))
                                    .textintval
                                    .value)
                                    .overflowing_mul(
                                        (*yyvsp.offset(0 as libc::c_int as isize)).intval,
                                    );
                                *(&mut yyval.rel.day as *mut intmax_t) = fresh101;
                                fresh102 as libc::c_int
                            } != 0
                            {
                                current_block = 16695196076034203957;
                                break;
                            }
                        }
                        79 => {
                            yyval
                                .rel = {
                                let mut init = relative_time {
                                    year: 0 as libc::c_int as intmax_t,
                                    month: 0 as libc::c_int as intmax_t,
                                    day: 0 as libc::c_int as intmax_t,
                                    hour: 0 as libc::c_int as intmax_t,
                                    minutes: 0 as libc::c_int as intmax_t,
                                    seconds: 0 as libc::c_int as intmax_t,
                                    ns: 0 as libc::c_int,
                                };
                                init
                            };
                            yyval
                                .rel
                                .hour = (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                .textintval
                                .value;
                        }
                        80 => {
                            yyval
                                .rel = {
                                let mut init = relative_time {
                                    year: 0 as libc::c_int as intmax_t,
                                    month: 0 as libc::c_int as intmax_t,
                                    day: 0 as libc::c_int as intmax_t,
                                    hour: 0 as libc::c_int as intmax_t,
                                    minutes: 0 as libc::c_int as intmax_t,
                                    seconds: 0 as libc::c_int as intmax_t,
                                    ns: 0 as libc::c_int,
                                };
                                init
                            };
                            yyval
                                .rel
                                .minutes = (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                .textintval
                                .value;
                        }
                        81 => {
                            yyval
                                .rel = {
                                let mut init = relative_time {
                                    year: 0 as libc::c_int as intmax_t,
                                    month: 0 as libc::c_int as intmax_t,
                                    day: 0 as libc::c_int as intmax_t,
                                    hour: 0 as libc::c_int as intmax_t,
                                    minutes: 0 as libc::c_int as intmax_t,
                                    seconds: 0 as libc::c_int as intmax_t,
                                    ns: 0 as libc::c_int,
                                };
                                init
                            };
                            yyval
                                .rel
                                .seconds = (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                .textintval
                                .value;
                        }
                        82 => {
                            yyval
                                .rel = {
                                let mut init = relative_time {
                                    year: 0 as libc::c_int as intmax_t,
                                    month: 0 as libc::c_int as intmax_t,
                                    day: 0 as libc::c_int as intmax_t,
                                    hour: 0 as libc::c_int as intmax_t,
                                    minutes: 0 as libc::c_int as intmax_t,
                                    seconds: 0 as libc::c_int as intmax_t,
                                    ns: 0 as libc::c_int,
                                };
                                init
                            };
                            yyval
                                .rel
                                .day = (*yyvsp.offset(0 as libc::c_int as isize)).intval;
                        }
                        86 => {
                            if time_overflow(
                                (*yyvsp.offset(0 as libc::c_int as isize)).textintval.value,
                            ) {
                                current_block = 16695196076034203957;
                                break;
                            }
                            yyval
                                .timespec = {
                                let mut init = timespec {
                                    tv_sec: (*yyvsp.offset(0 as libc::c_int as isize))
                                        .textintval
                                        .value,
                                    tv_nsec: 0,
                                };
                                init
                            };
                        }
                        88 => {
                            if time_overflow(
                                (*yyvsp.offset(0 as libc::c_int as isize)).textintval.value,
                            ) {
                                current_block = 16695196076034203957;
                                break;
                            }
                            yyval
                                .timespec = {
                                let mut init = timespec {
                                    tv_sec: (*yyvsp.offset(0 as libc::c_int as isize))
                                        .textintval
                                        .value,
                                    tv_nsec: 0,
                                };
                                init
                            };
                        }
                        89 => {
                            digits_to_date_time(
                                pc,
                                (*yyvsp.offset(0 as libc::c_int as isize)).textintval,
                            );
                        }
                        90 => {
                            digits_to_date_time(
                                pc,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).textintval,
                            );
                            if !apply_relative_time(
                                pc,
                                (*yyvsp.offset(0 as libc::c_int as isize)).rel,
                                1 as libc::c_int,
                            ) {
                                current_block = 16695196076034203957;
                                break;
                            }
                        }
                        91 => {
                            yyval.intval = -(1 as libc::c_int) as intmax_t;
                        }
                        92 => {
                            yyval
                                .intval = (*yyvsp.offset(0 as libc::c_int as isize))
                                .textintval
                                .value;
                        }
                        _ => {}
                    }
                    yyvsp = yyvsp.offset(-(yylen as isize));
                    yyssp = yyssp.offset(-(yylen as isize));
                    yylen = 0 as libc::c_int;
                    yyvsp = yyvsp.offset(1);
                    *yyvsp = yyval;
                    let yylhs: libc::c_int = yyr1[yyn as usize] as libc::c_int
                        - 29 as libc::c_int;
                    let yyi: libc::c_int = yypgoto[yylhs as usize] as libc::c_int
                        + *yyssp as libc::c_int;
                    yystate = if 0 as libc::c_int <= yyi && yyi <= 114 as libc::c_int
                        && yycheck[yyi as usize] as libc::c_int == *yyssp as libc::c_int
                    {
                        yytable[yyi as usize] as libc::c_int
                    } else {
                        yydefgoto[yylhs as usize] as libc::c_int
                    };
                }
                16915605972616362265 => {
                    yyerrstatus = 3 as libc::c_int;
                    loop {
                        yyn = yypact[yystate as usize] as libc::c_int;
                        if !(yyn == -(91 as libc::c_int)) {
                            yyn += YYSYMBOL_YYerror as libc::c_int;
                            if 0 as libc::c_int <= yyn && yyn <= 114 as libc::c_int
                                && yycheck[yyn as usize] as libc::c_int
                                    == YYSYMBOL_YYerror as libc::c_int
                            {
                                yyn = yytable[yyn as usize] as libc::c_int;
                                if (0 as libc::c_int) < yyn {
                                    break;
                                }
                            }
                        }
                        if yyssp == yyss {
                            current_block = 16695196076034203957;
                            break 's_54;
                        }
                        yydestruct(
                            b"Error: popping\0" as *const u8 as *const libc::c_char,
                            yystos[yystate as usize] as yysymbol_kind_t,
                            yyvsp,
                            pc,
                        );
                        yyvsp = yyvsp.offset(-(1 as libc::c_int as isize));
                        yyssp = yyssp.offset(-(1 as libc::c_int as isize));
                        yystate = *yyssp as yy_state_fast_t;
                    }
                    yyvsp = yyvsp.offset(1);
                    *yyvsp = yylval;
                    yystate = yyn;
                }
                _ => {}
            }
            yyssp = yyssp.offset(1);
            yyssp;
        }
    }
    match current_block {
        2666335540292013992 => {
            yyerror(pc, b"memory exhausted\0" as *const u8 as *const libc::c_char);
            yyresult = 2 as libc::c_int;
        }
        16695196076034203957 => {
            yyresult = 1 as libc::c_int;
        }
        _ => {}
    }
    if yychar != YYEMPTY as libc::c_int {
        yytoken = (if 0 as libc::c_int <= yychar && yychar <= 277 as libc::c_int {
            yytranslate[yychar as usize] as yysymbol_kind_t as libc::c_int
        } else {
            YYSYMBOL_YYUNDEF as libc::c_int
        }) as yysymbol_kind_t;
        yydestruct(
            b"Cleanup: discarding lookahead\0" as *const u8 as *const libc::c_char,
            yytoken,
            &mut yylval,
            pc,
        );
    }
    yyvsp = yyvsp.offset(-(yylen as isize));
    yyssp = yyssp.offset(-(yylen as isize));
    while yyssp != yyss {
        yydestruct(
            b"Cleanup: popping\0" as *const u8 as *const libc::c_char,
            yystos[*yyssp as libc::c_int as usize] as yysymbol_kind_t,
            yyvsp,
            pc,
        );
        yyvsp = yyvsp.offset(-(1 as libc::c_int as isize));
        yyssp = yyssp.offset(-(1 as libc::c_int as isize));
    }
    if yyss != yyssa.as_mut_ptr() {
        free(yyss as *mut libc::c_void);
    }
    return yyresult;
}
unsafe extern "C" fn debug_strfdatetime(
    mut tm: *const tm,
    mut pc: *const parser_control,
    mut buf: *mut libc::c_char,
    mut n: libc::c_int,
) -> *const libc::c_char {
    let mut m: libc::c_int = nstrftime(
        buf,
        n as size_t,
        b"(Y-M-D) %Y-%m-%d %H:%M:%S\0" as *const u8 as *const libc::c_char,
        tm,
        0 as timezone_t,
        0 as libc::c_int,
    ) as libc::c_int;
    if !pc.is_null() && m < n && (*pc).zones_seen != 0 {
        let mut tz: libc::c_int = (*pc).time_zone;
        if (*pc).local_zones_seen != 0 && (*pc).zones_seen == 0
            && (0 as libc::c_int) < (*pc).local_isdst
        {
            tz += 60 as libc::c_int * 60 as libc::c_int;
        }
        let mut time_zone_buf: [libc::c_char; 27] = [0; 27];
        snprintf(
            &mut *buf.offset(m as isize) as *mut libc::c_char,
            (n - m) as libc::c_ulong,
            b" TZ=%s\0" as *const u8 as *const libc::c_char,
            time_zone_str(tz, time_zone_buf.as_mut_ptr()),
        );
    }
    return buf;
}
unsafe extern "C" fn debug_strfdate(
    mut tm: *const tm,
    mut buf: *mut libc::c_char,
    mut n: libc::c_int,
) -> *const libc::c_char {
    let mut tm_year_buf: [libc::c_char; 13] = [0; 13];
    snprintf(
        buf,
        n as libc::c_ulong,
        b"(Y-M-D) %s-%02d-%02d\0" as *const u8 as *const libc::c_char,
        tm_year_str((*tm).tm_year, tm_year_buf.as_mut_ptr()),
        (*tm).tm_mon + 1 as libc::c_int,
        (*tm).tm_mday,
    );
    return buf;
}
unsafe extern "C" fn debug_strftime(
    mut tm: *const tm,
    mut buf: *mut libc::c_char,
    mut n: libc::c_int,
) -> *const libc::c_char {
    snprintf(
        buf,
        n as libc::c_ulong,
        b"%02d:%02d:%02d\0" as *const u8 as *const libc::c_char,
        (*tm).tm_hour,
        (*tm).tm_min,
        (*tm).tm_sec,
    );
    return buf;
}
unsafe extern "C" fn debug_mktime_not_ok(
    mut tm0: *const tm,
    mut tm1: *const tm,
    mut pc: *const parser_control,
    mut time_zone_seen: bool,
) {
    let mut tmp: [libc::c_char; 100] = [0; 100];
    let mut i: libc::c_int = 0;
    let eq_sec: bool = (*tm0).tm_sec == (*tm1).tm_sec;
    let eq_min: bool = (*tm0).tm_min == (*tm1).tm_min;
    let eq_hour: bool = (*tm0).tm_hour == (*tm1).tm_hour;
    let eq_mday: bool = (*tm0).tm_mday == (*tm1).tm_mday;
    let eq_month: bool = (*tm0).tm_mon == (*tm1).tm_mon;
    let eq_year: bool = (*tm0).tm_year == (*tm1).tm_year;
    let dst_shift: bool = eq_sec as libc::c_int != 0 && eq_min as libc::c_int != 0
        && !eq_hour && eq_mday as libc::c_int != 0 && eq_month as libc::c_int != 0
        && eq_year as libc::c_int != 0;
    if !debugging(pc) {
        return;
    }
    dbg_printf(
        gettext(
            b"error: invalid date/time value:\n\0" as *const u8 as *const libc::c_char,
        ),
    );
    dbg_printf(
        gettext(b"    user provided time: '%s'\n\0" as *const u8 as *const libc::c_char),
        debug_strfdatetime(
            tm0,
            pc,
            tmp.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong as libc::c_int,
        ),
    );
    dbg_printf(
        gettext(b"       normalized time: '%s'\n\0" as *const u8 as *const libc::c_char),
        debug_strfdatetime(
            tm1,
            pc,
            tmp.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong as libc::c_int,
        ),
    );
    i = snprintf(
        tmp.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong,
        b"                                 %4s %2s %2s %2s %2s %2s\0" as *const u8
            as *const libc::c_char,
        if eq_year as libc::c_int != 0 {
            b"\0" as *const u8 as *const libc::c_char
        } else {
            b"----\0" as *const u8 as *const libc::c_char
        },
        if eq_month as libc::c_int != 0 {
            b"\0" as *const u8 as *const libc::c_char
        } else {
            b"--\0" as *const u8 as *const libc::c_char
        },
        if eq_mday as libc::c_int != 0 {
            b"\0" as *const u8 as *const libc::c_char
        } else {
            b"--\0" as *const u8 as *const libc::c_char
        },
        if eq_hour as libc::c_int != 0 {
            b"\0" as *const u8 as *const libc::c_char
        } else {
            b"--\0" as *const u8 as *const libc::c_char
        },
        if eq_min as libc::c_int != 0 {
            b"\0" as *const u8 as *const libc::c_char
        } else {
            b"--\0" as *const u8 as *const libc::c_char
        },
        if eq_sec as libc::c_int != 0 {
            b"\0" as *const u8 as *const libc::c_char
        } else {
            b"--\0" as *const u8 as *const libc::c_char
        },
    );
    if 0 as libc::c_int <= i {
        if (::core::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) < i as libc::c_ulong
        {
            i = (::core::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
        }
        while (0 as libc::c_int) < i
            && tmp[(i - 1 as libc::c_int) as usize] as libc::c_int == ' ' as i32
        {
            i -= 1;
            i;
        }
        tmp[i as usize] = '\0' as i32 as libc::c_char;
    }
    dbg_printf(b"%s\n\0" as *const u8 as *const libc::c_char, tmp.as_mut_ptr());
    dbg_printf(
        gettext(b"     possible reasons:\n\0" as *const u8 as *const libc::c_char),
    );
    if dst_shift {
        dbg_printf(
            gettext(
                b"       nonexistent due to daylight-saving time;\n\0" as *const u8
                    as *const libc::c_char,
            ),
        );
    }
    if !eq_mday && !eq_month {
        dbg_printf(
            gettext(
                b"       invalid day/month combination;\n\0" as *const u8
                    as *const libc::c_char,
            ),
        );
    }
    dbg_printf(
        gettext(
            b"       numeric values overflow;\n\0" as *const u8 as *const libc::c_char,
        ),
    );
    dbg_printf(
        b"       %s\n\0" as *const u8 as *const libc::c_char,
        if time_zone_seen as libc::c_int != 0 {
            gettext(b"incorrect timezone\0" as *const u8 as *const libc::c_char)
        } else {
            gettext(b"missing timezone\0" as *const u8 as *const libc::c_char)
        },
    );
}
unsafe extern "C" fn parse_datetime_body(
    mut result: *mut timespec,
    mut p: *const libc::c_char,
    mut now: *const timespec,
    mut flags: libc::c_uint,
    mut tzdefault: timezone_t,
    mut tzstring: *const libc::c_char,
) -> bool {
    let mut tmp: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const libc::c_char,
    };
    let mut pc: parser_control = parser_control {
        input: 0 as *const libc::c_char,
        day_ordinal: 0,
        day_number: 0,
        local_isdst: 0,
        time_zone: 0,
        meridian: 0,
        year: textint {
            negative: false,
            value: 0,
            digits: 0,
        },
        month: 0,
        day: 0,
        hour: 0,
        minutes: 0,
        seconds: timespec { tv_sec: 0, tv_nsec: 0 },
        rel: relative_time {
            year: 0,
            month: 0,
            day: 0,
            hour: 0,
            minutes: 0,
            seconds: 0,
            ns: 0,
        },
        timespec_seen: false,
        rels_seen: false,
        dates_seen: 0,
        days_seen: 0,
        J_zones_seen: 0,
        local_zones_seen: 0,
        dsts_seen: 0,
        times_seen: 0,
        zones_seen: 0,
        year_seen: false,
        parse_datetime_debug: false,
        debug_dates_seen: false,
        debug_days_seen: false,
        debug_local_zones_seen: false,
        debug_times_seen: false,
        debug_zones_seen: false,
        debug_year_seen: false,
        debug_ordinal_day_seen: false,
        local_time_zone_table: [table {
            name: 0 as *const libc::c_char,
            type_0: 0,
            value: 0,
        }; 3],
    };
    let mut current_block: u64;
    let mut tm: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const libc::c_char,
    };
    let mut tm0: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const libc::c_char,
    };
    let mut time_zone_buf: [libc::c_char; 27] = [0; 27];
    let mut dbg_tm: [libc::c_char; 100] = [0; 100];
    let mut ok: bool = 0 as libc::c_int != 0;
    let mut input_sentinel: *const libc::c_char = p.offset(strlen(p) as isize);
    let mut tz1alloc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tz1buf: [libc::c_char; 100] = [0; 100];
    let mut gettime_buffer: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    if now.is_null() {
        gettime(&mut gettime_buffer);
        now = &mut gettime_buffer;
    }
    let mut Start: time_t = (*now).tv_sec;
    let mut Start_ns: libc::c_int = (*now).tv_nsec as libc::c_int;
    let mut c: libc::c_uchar = 0;
    loop {
        c = *p as libc::c_uchar;
        if !c_isspace(c as libc::c_int) {
            break;
        }
        p = p.offset(1);
        p;
    }
    let mut tz: timezone_t = tzdefault;
    let rel_time_0: relative_time = {
        let mut init = relative_time {
            year: 0 as libc::c_int as intmax_t,
            month: 0 as libc::c_int as intmax_t,
            day: 0 as libc::c_int as intmax_t,
            hour: 0 as libc::c_int as intmax_t,
            minutes: 0 as libc::c_int as intmax_t,
            seconds: 0 as libc::c_int as intmax_t,
            ns: 0 as libc::c_int,
        };
        init
    };
    if strncmp(
        p,
        b"TZ=\"\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        let mut tzbase: *const libc::c_char = p.offset(4 as libc::c_int as isize);
        let mut tzsize: idx_t = 1 as libc::c_int as idx_t;
        let mut s: *const libc::c_char = 0 as *const libc::c_char;
        s = tzbase;
        loop {
            if !(*s != 0) {
                current_block = 1118134448028020070;
                break;
            }
            if *s as libc::c_int == '\\' as i32 {
                s = s.offset(1);
                s;
                if !(*s as libc::c_int == '\\' as i32 || *s as libc::c_int == '"' as i32)
                {
                    current_block = 1118134448028020070;
                    break;
                }
            } else if *s as libc::c_int == '"' as i32 {
                let mut tz1: timezone_t = 0 as *mut tm_zone;
                let mut tz1string: *mut libc::c_char = tz1buf.as_mut_ptr();
                let mut z: *mut libc::c_char = 0 as *mut libc::c_char;
                if (TZBUFSIZE as libc::c_int as libc::c_long) < tzsize {
                    tz1alloc = malloc(tzsize as libc::c_ulong) as *mut libc::c_char;
                    if tz1alloc.is_null() {
                        current_block = 18080936705779730158;
                        break;
                    }
                    tz1string = tz1alloc;
                }
                z = tz1string;
                s = tzbase;
                while *s as libc::c_int != '"' as i32 {
                    s = s
                        .offset(
                            (*s as libc::c_int == '\\' as i32) as libc::c_int as isize,
                        );
                    let fresh103 = z;
                    z = z.offset(1);
                    *fresh103 = *s;
                    s = s.offset(1);
                    s;
                }
                *z = '\0' as i32 as libc::c_char;
                tz1 = tzalloc(tz1string);
                if tz1.is_null() {
                    current_block = 18080936705779730158;
                    break;
                }
                tz = tz1;
                tzstring = tz1string;
                p = s.offset(1 as libc::c_int as isize);
                loop {
                    c = *p as libc::c_uchar;
                    if !c_isspace(c as libc::c_int) {
                        break;
                    }
                    p = p.offset(1);
                    p;
                }
                current_block = 1118134448028020070;
                break;
            }
            s = s.offset(1);
            s;
            tzsize += 1;
            tzsize;
        }
    } else {
        current_block = 1118134448028020070;
    }
    match current_block {
        1118134448028020070 => {
            tmp = tm {
                tm_sec: 0,
                tm_min: 0,
                tm_hour: 0,
                tm_mday: 0,
                tm_mon: 0,
                tm_year: 0,
                tm_wday: 0,
                tm_yday: 0,
                tm_isdst: 0,
                tm_gmtoff: 0,
                tm_zone: 0 as *const libc::c_char,
            };
            if !(localtime_rz(tz, &(*now).tv_sec, &mut tmp)).is_null() {
                if *p as libc::c_int == '\0' as i32 {
                    p = b"0\0" as *const u8 as *const libc::c_char;
                }
                pc = parser_control {
                    input: 0 as *const libc::c_char,
                    day_ordinal: 0,
                    day_number: 0,
                    local_isdst: 0,
                    time_zone: 0,
                    meridian: 0,
                    year: textint {
                        negative: false,
                        value: 0,
                        digits: 0,
                    },
                    month: 0,
                    day: 0,
                    hour: 0,
                    minutes: 0,
                    seconds: timespec { tv_sec: 0, tv_nsec: 0 },
                    rel: relative_time {
                        year: 0,
                        month: 0,
                        day: 0,
                        hour: 0,
                        minutes: 0,
                        seconds: 0,
                        ns: 0,
                    },
                    timespec_seen: false,
                    rels_seen: false,
                    dates_seen: 0,
                    days_seen: 0,
                    J_zones_seen: 0,
                    local_zones_seen: 0,
                    dsts_seen: 0,
                    times_seen: 0,
                    zones_seen: 0,
                    year_seen: false,
                    parse_datetime_debug: false,
                    debug_dates_seen: false,
                    debug_days_seen: false,
                    debug_local_zones_seen: false,
                    debug_times_seen: false,
                    debug_zones_seen: false,
                    debug_year_seen: false,
                    debug_ordinal_day_seen: false,
                    local_time_zone_table: [table {
                        name: 0 as *const libc::c_char,
                        type_0: 0,
                        value: 0,
                    }; 3],
                };
                pc.input = p;
                pc
                    .parse_datetime_debug = flags & 1 as libc::c_int as libc::c_uint
                    != 0 as libc::c_int as libc::c_uint;
                let (fresh104, fresh105) = (tmp.tm_year)
                    .overflowing_add(TM_YEAR_BASE as libc::c_int);
                *(&mut pc.year.value as *mut intmax_t) = fresh104;
                if fresh105 {
                    if debugging(&mut pc) {
                        dbg_printf(
                            gettext(
                                b"error: initial year out of range\n\0" as *const u8
                                    as *const libc::c_char,
                            ),
                        );
                    }
                } else {
                    pc.year.digits = 0 as libc::c_int as idx_t;
                    pc.month = (tmp.tm_mon + 1 as libc::c_int) as intmax_t;
                    pc.day = tmp.tm_mday as intmax_t;
                    pc.hour = tmp.tm_hour as intmax_t;
                    pc.minutes = tmp.tm_min as intmax_t;
                    pc
                        .seconds = {
                        let mut init = timespec {
                            tv_sec: tmp.tm_sec as __time_t,
                            tv_nsec: Start_ns as __syscall_slong_t,
                        };
                        init
                    };
                    tm.tm_isdst = tmp.tm_isdst;
                    pc.meridian = MER24 as libc::c_int;
                    pc.rel = rel_time_0;
                    pc.timespec_seen = 0 as libc::c_int != 0;
                    pc.rels_seen = 0 as libc::c_int != 0;
                    pc.dates_seen = 0 as libc::c_int as idx_t;
                    pc.days_seen = 0 as libc::c_int as idx_t;
                    pc.times_seen = 0 as libc::c_int as idx_t;
                    pc.J_zones_seen = 0 as libc::c_int as idx_t;
                    pc.local_zones_seen = 0 as libc::c_int as idx_t;
                    pc.dsts_seen = 0 as libc::c_int as idx_t;
                    pc.zones_seen = 0 as libc::c_int as idx_t;
                    pc.year_seen = 0 as libc::c_int != 0;
                    pc.debug_dates_seen = 0 as libc::c_int != 0;
                    pc.debug_days_seen = 0 as libc::c_int != 0;
                    pc.debug_times_seen = 0 as libc::c_int != 0;
                    pc.debug_local_zones_seen = 0 as libc::c_int != 0;
                    pc.debug_zones_seen = 0 as libc::c_int != 0;
                    pc.debug_year_seen = 0 as libc::c_int != 0;
                    pc.debug_ordinal_day_seen = 0 as libc::c_int != 0;
                    pc
                        .local_time_zone_table[0 as libc::c_int as usize]
                        .name = tmp.tm_zone;
                    pc
                        .local_time_zone_table[0 as libc::c_int as usize]
                        .type_0 = tLOCAL_ZONE as libc::c_int;
                    pc
                        .local_time_zone_table[0 as libc::c_int as usize]
                        .value = tmp.tm_isdst;
                    pc
                        .local_time_zone_table[1 as libc::c_int as usize]
                        .name = 0 as *const libc::c_char;
                    let mut quarter: libc::c_int = 0;
                    quarter = 1 as libc::c_int;
                    while quarter <= 3 as libc::c_int {
                        let mut probe: time_t = 0;
                        let (fresh106, fresh107) = Start
                            .overflowing_add(
                                (quarter
                                    * (90 as libc::c_int * 24 as libc::c_int * 60 as libc::c_int
                                        * 60 as libc::c_int)).into(),
                            );
                        *(&mut probe as *mut time_t) = fresh106;
                        if fresh107 {
                            break;
                        }
                        let mut probe_tm: tm = tm {
                            tm_sec: 0,
                            tm_min: 0,
                            tm_hour: 0,
                            tm_mday: 0,
                            tm_mon: 0,
                            tm_year: 0,
                            tm_wday: 0,
                            tm_yday: 0,
                            tm_isdst: 0,
                            tm_gmtoff: 0,
                            tm_zone: 0 as *const libc::c_char,
                        };
                        if !(localtime_rz(tz, &mut probe, &mut probe_tm)).is_null()
                            && !(probe_tm.tm_zone).is_null()
                            && probe_tm.tm_isdst
                                != pc.local_time_zone_table[0 as libc::c_int as usize].value
                        {
                            pc
                                .local_time_zone_table[1 as libc::c_int as usize]
                                .name = probe_tm.tm_zone;
                            pc
                                .local_time_zone_table[1 as libc::c_int as usize]
                                .type_0 = tLOCAL_ZONE as libc::c_int;
                            pc
                                .local_time_zone_table[1 as libc::c_int as usize]
                                .value = probe_tm.tm_isdst;
                            pc
                                .local_time_zone_table[2 as libc::c_int as usize]
                                .name = 0 as *const libc::c_char;
                            break;
                        } else {
                            quarter += 1;
                            quarter;
                        }
                    }
                    if !(pc.local_time_zone_table[0 as libc::c_int as usize].name)
                        .is_null()
                        && !(pc.local_time_zone_table[1 as libc::c_int as usize].name)
                            .is_null()
                        && strcmp(
                            pc.local_time_zone_table[0 as libc::c_int as usize].name,
                            pc.local_time_zone_table[1 as libc::c_int as usize].name,
                        ) == 0
                    {
                        pc
                            .local_time_zone_table[0 as libc::c_int as usize]
                            .value = -(1 as libc::c_int);
                        pc
                            .local_time_zone_table[1 as libc::c_int as usize]
                            .name = 0 as *const libc::c_char;
                    }
                    if yyparse(&mut pc) != 0 as libc::c_int {
                        if debugging(&mut pc) {
                            dbg_printf(
                                if input_sentinel <= pc.input {
                                    gettext(
                                        b"error: parsing failed\n\0" as *const u8
                                            as *const libc::c_char,
                                    )
                                } else {
                                    gettext(
                                        b"error: parsing failed, stopped at '%s'\n\0" as *const u8
                                            as *const libc::c_char,
                                    )
                                },
                                pc.input,
                            );
                        }
                    } else {
                        if debugging(&mut pc) {
                            dbg_printf(
                                gettext(
                                    b"input timezone: \0" as *const u8 as *const libc::c_char,
                                ),
                            );
                            if pc.timespec_seen {
                                fprintf(
                                    stderr,
                                    gettext(
                                        b"'@timespec' - always UTC\0" as *const u8
                                            as *const libc::c_char,
                                    ),
                                );
                            } else if pc.zones_seen != 0 {
                                fprintf(
                                    stderr,
                                    gettext(
                                        b"parsed date/time string\0" as *const u8
                                            as *const libc::c_char,
                                    ),
                                );
                            } else if !tzstring.is_null() {
                                if tz != tzdefault {
                                    fprintf(
                                        stderr,
                                        gettext(
                                            b"TZ=\"%s\" in date string\0" as *const u8
                                                as *const libc::c_char,
                                        ),
                                        tzstring,
                                    );
                                } else if strcmp(
                                    tzstring,
                                    b"UTC0\0" as *const u8 as *const libc::c_char,
                                ) == 0 as libc::c_int
                                {
                                    fprintf(
                                        stderr,
                                        gettext(
                                            b"TZ=\"UTC0\" environment value or -u\0" as *const u8
                                                as *const libc::c_char,
                                        ),
                                    );
                                } else {
                                    fprintf(
                                        stderr,
                                        gettext(
                                            b"TZ=\"%s\" environment value\0" as *const u8
                                                as *const libc::c_char,
                                        ),
                                        tzstring,
                                    );
                                }
                            } else {
                                fprintf(
                                    stderr,
                                    gettext(
                                        b"system default\0" as *const u8 as *const libc::c_char,
                                    ),
                                );
                            }
                            if pc.local_zones_seen != 0 && pc.zones_seen == 0
                                && (0 as libc::c_int) < pc.local_isdst
                            {
                                fprintf(
                                    stderr,
                                    b", dst\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            if pc.zones_seen != 0 {
                                fprintf(
                                    stderr,
                                    b" (%s)\0" as *const u8 as *const libc::c_char,
                                    time_zone_str(pc.time_zone, time_zone_buf.as_mut_ptr()),
                                );
                            }
                            fputc('\n' as i32, stderr);
                        }
                        if pc.timespec_seen {
                            *result = pc.seconds;
                            current_block = 7188795011561844502;
                        } else if (1 as libc::c_int as libc::c_long)
                            < pc.times_seen | pc.dates_seen | pc.days_seen | pc.dsts_seen
                                | pc.J_zones_seen + pc.local_zones_seen + pc.zones_seen
                        {
                            if debugging(&mut pc) {
                                if pc.times_seen > 1 as libc::c_int as libc::c_long {
                                    dbg_printf(
                                        b"error: seen multiple time parts\n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                }
                                if pc.dates_seen > 1 as libc::c_int as libc::c_long {
                                    dbg_printf(
                                        b"error: seen multiple date parts\n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                }
                                if pc.days_seen > 1 as libc::c_int as libc::c_long {
                                    dbg_printf(
                                        b"error: seen multiple days parts\n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                }
                                if pc.dsts_seen > 1 as libc::c_int as libc::c_long {
                                    dbg_printf(
                                        b"error: seen multiple daylight-saving parts\n\0"
                                            as *const u8 as *const libc::c_char,
                                    );
                                }
                                if pc.J_zones_seen + pc.local_zones_seen + pc.zones_seen
                                    > 1 as libc::c_int as libc::c_long
                                {
                                    dbg_printf(
                                        b"error: seen multiple time-zone parts\n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                }
                            }
                            current_block = 18080936705779730158;
                        } else if !to_tm_year(
                            pc.year,
                            debugging(&mut pc),
                            &mut tm.tm_year,
                        )
                            || {
                                let (fresh108, fresh109) = (pc.month)
                                    .overflowing_add((-(1 as libc::c_int)).into());
                                *(&mut tm.tm_mon as *mut libc::c_int) = fresh108;
                                fresh109 as libc::c_int != 0
                            }
                            || {
                                let (fresh110, fresh111) = (pc.day)
                                    .overflowing_add((0 as libc::c_int).into());
                                *(&mut tm.tm_mday as *mut libc::c_int) = fresh110;
                                fresh111 as libc::c_int != 0
                            }
                        {
                            if debugging(&mut pc) {
                                dbg_printf(
                                    gettext(
                                        b"error: year, month, or day overflow\n\0" as *const u8
                                            as *const libc::c_char,
                                    ),
                                );
                            }
                            current_block = 18080936705779730158;
                        } else {
                            if pc.times_seen != 0
                                || pc.rels_seen as libc::c_int != 0 && pc.dates_seen == 0
                                    && pc.days_seen == 0
                            {
                                tm.tm_hour = to_hour(pc.hour, pc.meridian);
                                if tm.tm_hour < 0 as libc::c_int {
                                    let mut mrd: *const libc::c_char = if pc.meridian
                                        == MERam as libc::c_int
                                    {
                                        b"am\0" as *const u8 as *const libc::c_char
                                    } else if pc.meridian == MERpm as libc::c_int {
                                        b"pm\0" as *const u8 as *const libc::c_char
                                    } else {
                                        b"\0" as *const u8 as *const libc::c_char
                                    };
                                    if debugging(&mut pc) {
                                        dbg_printf(
                                            gettext(
                                                b"error: invalid hour %ld%s\n\0" as *const u8
                                                    as *const libc::c_char,
                                            ),
                                            pc.hour,
                                            mrd,
                                        );
                                    }
                                    current_block = 18080936705779730158;
                                } else {
                                    tm.tm_min = pc.minutes as libc::c_int;
                                    tm.tm_sec = pc.seconds.tv_sec as libc::c_int;
                                    if debugging(&mut pc) {
                                        dbg_printf(
                                            if pc.times_seen != 0 {
                                                gettext(
                                                    b"using specified time as starting value: '%s'\n\0"
                                                        as *const u8 as *const libc::c_char,
                                                )
                                            } else {
                                                gettext(
                                                    b"using current time as starting value: '%s'\n\0"
                                                        as *const u8 as *const libc::c_char,
                                                )
                                            },
                                            debug_strftime(
                                                &mut tm,
                                                dbg_tm.as_mut_ptr(),
                                                ::core::mem::size_of::<[libc::c_char; 100]>()
                                                    as libc::c_ulong as libc::c_int,
                                            ),
                                        );
                                    }
                                    current_block = 10248984122780841972;
                                }
                            } else {
                                tm.tm_sec = 0 as libc::c_int;
                                tm.tm_min = tm.tm_sec;
                                tm.tm_hour = tm.tm_min;
                                pc.seconds.tv_nsec = 0 as libc::c_int as __syscall_slong_t;
                                if debugging(&mut pc) {
                                    dbg_printf(
                                        b"warning: using midnight as starting time: 00:00:00\n\0"
                                            as *const u8 as *const libc::c_char,
                                    );
                                }
                                current_block = 10248984122780841972;
                            }
                            match current_block {
                                18080936705779730158 => {}
                                _ => {
                                    if pc.dates_seen | pc.days_seen | pc.times_seen != 0 {
                                        tm.tm_isdst = -(1 as libc::c_int);
                                    }
                                    if pc.local_zones_seen != 0 {
                                        tm.tm_isdst = pc.local_isdst;
                                    }
                                    tm0.tm_sec = tm.tm_sec;
                                    tm0.tm_min = tm.tm_min;
                                    tm0.tm_hour = tm.tm_hour;
                                    tm0.tm_mday = tm.tm_mday;
                                    tm0.tm_mon = tm.tm_mon;
                                    tm0.tm_year = tm.tm_year;
                                    tm0.tm_isdst = tm.tm_isdst;
                                    tm.tm_wday = -(1 as libc::c_int);
                                    Start = mktime_z(tz, &mut tm);
                                    if !mktime_ok(&mut tm0, &mut tm) {
                                        let mut repaired: bool = 0 as libc::c_int != 0;
                                        let mut time_zone_seen: bool = pc.zones_seen
                                            != 0 as libc::c_int as libc::c_long;
                                        if time_zone_seen {
                                            let mut tz2buf: [libc::c_char; 30] = [0; 30];
                                            tz2buf[2 as libc::c_int
                                                as usize] = 'X' as i32 as libc::c_char;
                                            tz2buf[1 as libc::c_int
                                                as usize] = tz2buf[2 as libc::c_int as usize];
                                            tz2buf[0 as libc::c_int
                                                as usize] = tz2buf[1 as libc::c_int as usize];
                                            time_zone_str(
                                                pc.time_zone,
                                                &mut *tz2buf.as_mut_ptr().offset(3 as libc::c_int as isize),
                                            );
                                            let mut tz2: timezone_t = tzalloc(tz2buf.as_mut_ptr());
                                            if tz2.is_null() {
                                                if debugging(&mut pc) {
                                                    dbg_printf(
                                                        gettext(
                                                            b"error: tzalloc (\"%s\") failed\n\0" as *const u8
                                                                as *const libc::c_char,
                                                        ),
                                                        tz2buf.as_mut_ptr(),
                                                    );
                                                }
                                                current_block = 18080936705779730158;
                                            } else {
                                                tm.tm_sec = tm0.tm_sec;
                                                tm.tm_min = tm0.tm_min;
                                                tm.tm_hour = tm0.tm_hour;
                                                tm.tm_mday = tm0.tm_mday;
                                                tm.tm_mon = tm0.tm_mon;
                                                tm.tm_year = tm0.tm_year;
                                                tm.tm_isdst = tm0.tm_isdst;
                                                tm.tm_wday = -(1 as libc::c_int);
                                                Start = mktime_z(tz2, &mut tm);
                                                repaired = mktime_ok(&mut tm0, &mut tm);
                                                tzfree(tz2);
                                                current_block = 11227437541145425351;
                                            }
                                        } else {
                                            current_block = 11227437541145425351;
                                        }
                                        match current_block {
                                            18080936705779730158 => {}
                                            _ => {
                                                if !repaired {
                                                    debug_mktime_not_ok(
                                                        &mut tm0,
                                                        &mut tm,
                                                        &mut pc,
                                                        time_zone_seen,
                                                    );
                                                    current_block = 18080936705779730158;
                                                } else {
                                                    current_block = 2884634553824165030;
                                                }
                                            }
                                        }
                                    } else {
                                        current_block = 2884634553824165030;
                                    }
                                    match current_block {
                                        18080936705779730158 => {}
                                        _ => {
                                            let mut dbg_ord: [libc::c_char; 100] = [0; 100];
                                            if pc.days_seen != 0 && pc.dates_seen == 0 {
                                                let mut dayincr: intmax_t = 0;
                                                tm.tm_yday = -(1 as libc::c_int);
                                                let mut day_ordinal: intmax_t = pc.day_ordinal
                                                    - ((0 as libc::c_int as libc::c_long) < pc.day_ordinal
                                                        && tm.tm_wday != pc.day_number) as libc::c_int
                                                        as libc::c_long;
                                                if !((if (0 as libc::c_int as intmax_t)
                                                    < -(1 as libc::c_int) as intmax_t
                                                    && ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_long
                                                    } else {
                                                        day_ordinal
                                                    }) - 1 as libc::c_int as libc::c_long)
                                                        < 0 as libc::c_int as libc::c_long
                                                    && ((if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int
                                                    } else {
                                                        7 as libc::c_int
                                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                                    && (if (7 as libc::c_int) < 0 as libc::c_int {
                                                        if day_ordinal < 0 as libc::c_int as libc::c_long {
                                                            if ((if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_long
                                                            } else {
                                                                (if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int as libc::c_long
                                                                } else {
                                                                    -(1 as libc::c_int) as intmax_t
                                                                }) + 7 as libc::c_int as libc::c_long
                                                            }) - 1 as libc::c_int as libc::c_long)
                                                                < 0 as libc::c_int as libc::c_long
                                                            {
                                                                (day_ordinal
                                                                    < -(1 as libc::c_int) as intmax_t
                                                                        / 7 as libc::c_int as libc::c_long) as libc::c_int
                                                            } else {
                                                                ((if (if (if ((if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int
                                                                } else {
                                                                    7 as libc::c_int
                                                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                                                {
                                                                    !(((((if 1 as libc::c_int != 0 {
                                                                        0 as libc::c_int
                                                                    } else {
                                                                        7 as libc::c_int
                                                                    }) + 1 as libc::c_int)
                                                                        << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                                                } else {
                                                                    (if 1 as libc::c_int != 0 {
                                                                        0 as libc::c_int
                                                                    } else {
                                                                        7 as libc::c_int
                                                                    }) + 0 as libc::c_int
                                                                }) < 0 as libc::c_int
                                                                {
                                                                    ((7 as libc::c_int)
                                                                        < -(if ((if 1 as libc::c_int != 0 {
                                                                            0 as libc::c_int
                                                                        } else {
                                                                            7 as libc::c_int
                                                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                                                        {
                                                                            ((((if 1 as libc::c_int != 0 {
                                                                                0 as libc::c_int
                                                                            } else {
                                                                                7 as libc::c_int
                                                                            }) + 1 as libc::c_int)
                                                                                << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                                                        } else {
                                                                            (if 1 as libc::c_int != 0 {
                                                                                0 as libc::c_int
                                                                            } else {
                                                                                7 as libc::c_int
                                                                            }) - 1 as libc::c_int
                                                                        })) as libc::c_int
                                                                } else {
                                                                    ((0 as libc::c_int) < 7 as libc::c_int) as libc::c_int
                                                                }) != 0
                                                                {
                                                                    (if 1 as libc::c_int != 0 {
                                                                        0 as libc::c_int
                                                                    } else {
                                                                        7 as libc::c_int
                                                                    }) as libc::c_long + -(1 as libc::c_int) as intmax_t
                                                                        >> (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                                } else {
                                                                    -(1 as libc::c_int) as intmax_t
                                                                        / -(7 as libc::c_int) as libc::c_long
                                                                }) <= -(1 as libc::c_int) as libc::c_long - day_ordinal)
                                                                    as libc::c_int
                                                            }
                                                        } else {
                                                            if (if (if ((if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_long
                                                            } else {
                                                                (if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int
                                                                } else {
                                                                    7 as libc::c_int
                                                                }) as libc::c_long + 0 as libc::c_int as intmax_t
                                                            }) - 1 as libc::c_int as libc::c_long)
                                                                < 0 as libc::c_int as libc::c_long
                                                            {
                                                                !(((((if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int as libc::c_long
                                                                } else {
                                                                    (if 1 as libc::c_int != 0 {
                                                                        0 as libc::c_int
                                                                    } else {
                                                                        7 as libc::c_int
                                                                    }) as libc::c_long + 0 as libc::c_int as intmax_t
                                                                }) + 1 as libc::c_int as libc::c_long)
                                                                    << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                                    - 1 as libc::c_int as libc::c_long)
                                                                    * 2 as libc::c_int as libc::c_long
                                                                    + 1 as libc::c_int as libc::c_long)
                                                            } else {
                                                                (if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int as libc::c_long
                                                                } else {
                                                                    (if 1 as libc::c_int != 0 {
                                                                        0 as libc::c_int
                                                                    } else {
                                                                        7 as libc::c_int
                                                                    }) as libc::c_long + 0 as libc::c_int as intmax_t
                                                                }) + 0 as libc::c_int as libc::c_long
                                                            }) < 0 as libc::c_int as libc::c_long
                                                            {
                                                                (((if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int
                                                                } else {
                                                                    7 as libc::c_int
                                                                }) as libc::c_long + 0 as libc::c_int as intmax_t)
                                                                    < -(if ((if 1 as libc::c_int != 0 {
                                                                        0 as libc::c_int as libc::c_long
                                                                    } else {
                                                                        (if 1 as libc::c_int != 0 {
                                                                            0 as libc::c_int
                                                                        } else {
                                                                            7 as libc::c_int
                                                                        }) as libc::c_long + 0 as libc::c_int as intmax_t
                                                                    }) - 1 as libc::c_int as libc::c_long)
                                                                        < 0 as libc::c_int as libc::c_long
                                                                    {
                                                                        ((((if 1 as libc::c_int != 0 {
                                                                            0 as libc::c_int as libc::c_long
                                                                        } else {
                                                                            (if 1 as libc::c_int != 0 {
                                                                                0 as libc::c_int
                                                                            } else {
                                                                                7 as libc::c_int
                                                                            }) as libc::c_long + 0 as libc::c_int as intmax_t
                                                                        }) + 1 as libc::c_int as libc::c_long)
                                                                            << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                                            - 1 as libc::c_int as libc::c_long)
                                                                            * 2 as libc::c_int as libc::c_long
                                                                            + 1 as libc::c_int as libc::c_long
                                                                    } else {
                                                                        (if 1 as libc::c_int != 0 {
                                                                            0 as libc::c_int as libc::c_long
                                                                        } else {
                                                                            (if 1 as libc::c_int != 0 {
                                                                                0 as libc::c_int
                                                                            } else {
                                                                                7 as libc::c_int
                                                                            }) as libc::c_long + 0 as libc::c_int as intmax_t
                                                                        }) - 1 as libc::c_int as libc::c_long
                                                                    })) as libc::c_int
                                                            } else {
                                                                ((0 as libc::c_int as libc::c_long)
                                                                    < (if 1 as libc::c_int != 0 {
                                                                        0 as libc::c_int
                                                                    } else {
                                                                        7 as libc::c_int
                                                                    }) as libc::c_long + 0 as libc::c_int as intmax_t)
                                                                    as libc::c_int
                                                            }) != 0 && 7 as libc::c_int == -(1 as libc::c_int)
                                                            {
                                                                if ((if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int as libc::c_long
                                                                } else {
                                                                    day_ordinal
                                                                }) - 1 as libc::c_int as libc::c_long)
                                                                    < 0 as libc::c_int as libc::c_long
                                                                {
                                                                    ((0 as libc::c_int as libc::c_long)
                                                                        < day_ordinal + 0 as libc::c_int as intmax_t) as libc::c_int
                                                                } else {
                                                                    ((0 as libc::c_int as libc::c_long) < day_ordinal
                                                                        && (-(1 as libc::c_int) as libc::c_long
                                                                            - 0 as libc::c_int as intmax_t)
                                                                            < day_ordinal - 1 as libc::c_int as libc::c_long)
                                                                        as libc::c_int
                                                                }
                                                            } else {
                                                                ((0 as libc::c_int as intmax_t
                                                                    / 7 as libc::c_int as libc::c_long) < day_ordinal)
                                                                    as libc::c_int
                                                            }
                                                        }
                                                    } else {
                                                        if 7 as libc::c_int == 0 as libc::c_int {
                                                            0 as libc::c_int
                                                        } else {
                                                            if day_ordinal < 0 as libc::c_int as libc::c_long {
                                                                if (if (if ((if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int as libc::c_long
                                                                } else {
                                                                    (if 1 as libc::c_int != 0 {
                                                                        0 as libc::c_int as libc::c_long
                                                                    } else {
                                                                        day_ordinal
                                                                    }) + 0 as libc::c_int as intmax_t
                                                                }) - 1 as libc::c_int as libc::c_long)
                                                                    < 0 as libc::c_int as libc::c_long
                                                                {
                                                                    !(((((if 1 as libc::c_int != 0 {
                                                                        0 as libc::c_int as libc::c_long
                                                                    } else {
                                                                        (if 1 as libc::c_int != 0 {
                                                                            0 as libc::c_int as libc::c_long
                                                                        } else {
                                                                            day_ordinal
                                                                        }) + 0 as libc::c_int as intmax_t
                                                                    }) + 1 as libc::c_int as libc::c_long)
                                                                        << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                                        - 1 as libc::c_int as libc::c_long)
                                                                        * 2 as libc::c_int as libc::c_long
                                                                        + 1 as libc::c_int as libc::c_long)
                                                                } else {
                                                                    (if 1 as libc::c_int != 0 {
                                                                        0 as libc::c_int as libc::c_long
                                                                    } else {
                                                                        (if 1 as libc::c_int != 0 {
                                                                            0 as libc::c_int as libc::c_long
                                                                        } else {
                                                                            day_ordinal
                                                                        }) + 0 as libc::c_int as intmax_t
                                                                    }) + 0 as libc::c_int as libc::c_long
                                                                }) < 0 as libc::c_int as libc::c_long
                                                                {
                                                                    (((if 1 as libc::c_int != 0 {
                                                                        0 as libc::c_int as libc::c_long
                                                                    } else {
                                                                        day_ordinal
                                                                    }) + 0 as libc::c_int as intmax_t)
                                                                        < -(if ((if 1 as libc::c_int != 0 {
                                                                            0 as libc::c_int as libc::c_long
                                                                        } else {
                                                                            (if 1 as libc::c_int != 0 {
                                                                                0 as libc::c_int as libc::c_long
                                                                            } else {
                                                                                day_ordinal
                                                                            }) + 0 as libc::c_int as intmax_t
                                                                        }) - 1 as libc::c_int as libc::c_long)
                                                                            < 0 as libc::c_int as libc::c_long
                                                                        {
                                                                            ((((if 1 as libc::c_int != 0 {
                                                                                0 as libc::c_int as libc::c_long
                                                                            } else {
                                                                                (if 1 as libc::c_int != 0 {
                                                                                    0 as libc::c_int as libc::c_long
                                                                                } else {
                                                                                    day_ordinal
                                                                                }) + 0 as libc::c_int as intmax_t
                                                                            }) + 1 as libc::c_int as libc::c_long)
                                                                                << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                                                - 1 as libc::c_int as libc::c_long)
                                                                                * 2 as libc::c_int as libc::c_long
                                                                                + 1 as libc::c_int as libc::c_long
                                                                        } else {
                                                                            (if 1 as libc::c_int != 0 {
                                                                                0 as libc::c_int as libc::c_long
                                                                            } else {
                                                                                (if 1 as libc::c_int != 0 {
                                                                                    0 as libc::c_int as libc::c_long
                                                                                } else {
                                                                                    day_ordinal
                                                                                }) + 0 as libc::c_int as intmax_t
                                                                            }) - 1 as libc::c_int as libc::c_long
                                                                        })) as libc::c_int
                                                                } else {
                                                                    ((0 as libc::c_int as libc::c_long)
                                                                        < (if 1 as libc::c_int != 0 {
                                                                            0 as libc::c_int as libc::c_long
                                                                        } else {
                                                                            day_ordinal
                                                                        }) + 0 as libc::c_int as intmax_t) as libc::c_int
                                                                }) != 0
                                                                    && day_ordinal == -(1 as libc::c_int) as libc::c_long
                                                                {
                                                                    if ((if 1 as libc::c_int != 0 {
                                                                        0 as libc::c_int
                                                                    } else {
                                                                        7 as libc::c_int
                                                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                                                    {
                                                                        ((0 as libc::c_int as libc::c_long)
                                                                            < 7 as libc::c_int as libc::c_long
                                                                                + 0 as libc::c_int as intmax_t) as libc::c_int
                                                                    } else {
                                                                        ((-(1 as libc::c_int) as libc::c_long
                                                                            - 0 as libc::c_int as intmax_t)
                                                                            < (7 as libc::c_int - 1 as libc::c_int) as libc::c_long)
                                                                            as libc::c_int
                                                                    }
                                                                } else {
                                                                    (0 as libc::c_int as intmax_t / day_ordinal
                                                                        < 7 as libc::c_int as libc::c_long) as libc::c_int
                                                                }
                                                            } else {
                                                                ((-(1 as libc::c_int) as intmax_t
                                                                    / 7 as libc::c_int as libc::c_long) < day_ordinal)
                                                                    as libc::c_int
                                                            }
                                                        }
                                                    }) != 0
                                                {
                                                    let (fresh116, fresh117) = day_ordinal
                                                        .overflowing_mul((7 as libc::c_int).into());
                                                    *(&mut dayincr as *mut intmax_t) = fresh116;
                                                    1 as libc::c_int
                                                } else {
                                                    let (fresh118, fresh119) = day_ordinal
                                                        .overflowing_mul((7 as libc::c_int).into());
                                                    *(&mut dayincr as *mut intmax_t) = fresh118;
                                                    fresh119 as libc::c_int
                                                }) != 0
                                                    || {
                                                        let (fresh120, fresh121) = ((pc.day_number - tm.tm_wday
                                                            + 7 as libc::c_int) % 7 as libc::c_int)
                                                            .overflowing_add(dayincr.try_into().unwrap());
                                                        *(&mut dayincr as *mut intmax_t) = fresh120;
                                                        fresh121 as libc::c_int != 0
                                                    }
                                                    || {
                                                        let (fresh122, fresh123) = dayincr
                                                            .overflowing_add(tm.tm_mday.into());
                                                        *(&mut tm.tm_mday as *mut libc::c_int) = fresh122;
                                                        fresh123 as libc::c_int != 0
                                                    })
                                                {
                                                    tm.tm_isdst = -(1 as libc::c_int);
                                                    Start = mktime_z(tz, &mut tm);
                                                }
                                                if tm.tm_yday < 0 as libc::c_int {
                                                    if debugging(&mut pc) {
                                                        dbg_printf(
                                                            gettext(
                                                                b"error: day '%s' (day ordinal=%ld number=%d) resulted in an invalid date: '%s'\n\0"
                                                                    as *const u8 as *const libc::c_char,
                                                            ),
                                                            str_days(
                                                                &mut pc,
                                                                dbg_ord.as_mut_ptr(),
                                                                ::core::mem::size_of::<[libc::c_char; 100]>()
                                                                    as libc::c_ulong as libc::c_int,
                                                            ),
                                                            pc.day_ordinal,
                                                            pc.day_number,
                                                            debug_strfdatetime(
                                                                &mut tm,
                                                                &mut pc,
                                                                dbg_tm.as_mut_ptr(),
                                                                ::core::mem::size_of::<[libc::c_char; 100]>()
                                                                    as libc::c_ulong as libc::c_int,
                                                            ),
                                                        );
                                                    }
                                                    current_block = 18080936705779730158;
                                                } else {
                                                    if debugging(&mut pc) {
                                                        dbg_printf(
                                                            gettext(
                                                                b"new start date: '%s' is '%s'\n\0" as *const u8
                                                                    as *const libc::c_char,
                                                            ),
                                                            str_days(
                                                                &mut pc,
                                                                dbg_ord.as_mut_ptr(),
                                                                ::core::mem::size_of::<[libc::c_char; 100]>()
                                                                    as libc::c_ulong as libc::c_int,
                                                            ),
                                                            debug_strfdatetime(
                                                                &mut tm,
                                                                &mut pc,
                                                                dbg_tm.as_mut_ptr(),
                                                                ::core::mem::size_of::<[libc::c_char; 100]>()
                                                                    as libc::c_ulong as libc::c_int,
                                                            ),
                                                        );
                                                    }
                                                    current_block = 9270770154621591809;
                                                }
                                            } else {
                                                current_block = 9270770154621591809;
                                            }
                                            match current_block {
                                                18080936705779730158 => {}
                                                _ => {
                                                    if debugging(&mut pc) {
                                                        if pc.dates_seen == 0 && pc.days_seen == 0 {
                                                            dbg_printf(
                                                                gettext(
                                                                    b"using current date as starting value: '%s'\n\0"
                                                                        as *const u8 as *const libc::c_char,
                                                                ),
                                                                debug_strfdate(
                                                                    &mut tm,
                                                                    dbg_tm.as_mut_ptr(),
                                                                    ::core::mem::size_of::<[libc::c_char; 100]>()
                                                                        as libc::c_ulong as libc::c_int,
                                                                ),
                                                            );
                                                        }
                                                        if pc.days_seen != 0 && pc.dates_seen != 0 {
                                                            dbg_printf(
                                                                gettext(
                                                                    b"warning: day (%s) ignored when explicit dates are given\n\0"
                                                                        as *const u8 as *const libc::c_char,
                                                                ),
                                                                str_days(
                                                                    &mut pc,
                                                                    dbg_ord.as_mut_ptr(),
                                                                    ::core::mem::size_of::<[libc::c_char; 100]>()
                                                                        as libc::c_ulong as libc::c_int,
                                                                ),
                                                            );
                                                        }
                                                        dbg_printf(
                                                            gettext(
                                                                b"starting date/time: '%s'\n\0" as *const u8
                                                                    as *const libc::c_char,
                                                            ),
                                                            debug_strfdatetime(
                                                                &mut tm,
                                                                &mut pc,
                                                                dbg_tm.as_mut_ptr(),
                                                                ::core::mem::size_of::<[libc::c_char; 100]>()
                                                                    as libc::c_ulong as libc::c_int,
                                                            ),
                                                        );
                                                    }
                                                    if pc.rel.year | pc.rel.month | pc.rel.day != 0 {
                                                        if debugging(&mut pc) {
                                                            if (pc.rel.year != 0 as libc::c_int as libc::c_long
                                                                || pc.rel.month != 0 as libc::c_int as libc::c_long)
                                                                && tm.tm_mday != 15 as libc::c_int
                                                            {
                                                                dbg_printf(
                                                                    gettext(
                                                                        b"warning: when adding relative months/years, it is recommended to specify the 15th of the months\n\0"
                                                                            as *const u8 as *const libc::c_char,
                                                                    ),
                                                                );
                                                            }
                                                            if pc.rel.day != 0 as libc::c_int as libc::c_long
                                                                && tm.tm_hour != 12 as libc::c_int
                                                            {
                                                                dbg_printf(
                                                                    gettext(
                                                                        b"warning: when adding relative days, it is recommended to specify noon\n\0"
                                                                            as *const u8 as *const libc::c_char,
                                                                    ),
                                                                );
                                                            }
                                                        }
                                                        let mut year: libc::c_int = 0;
                                                        let mut month: libc::c_int = 0;
                                                        let mut day: libc::c_int = 0;
                                                        let (fresh124, fresh125) = (tm.tm_year)
                                                            .overflowing_add(pc.rel.year.try_into().unwrap());
                                                        *(&mut year as *mut libc::c_int) = fresh124;
                                                        if fresh125 as libc::c_int != 0
                                                            || {
                                                                let (fresh126, fresh127) = (tm.tm_mon)
                                                                    .overflowing_add(pc.rel.month.try_into().unwrap());
                                                                *(&mut month as *mut libc::c_int) = fresh126;
                                                                fresh127 as libc::c_int != 0
                                                            }
                                                            || {
                                                                let (fresh128, fresh129) = (tm.tm_mday)
                                                                    .overflowing_add(pc.rel.day.try_into().unwrap());
                                                                *(&mut day as *mut libc::c_int) = fresh128;
                                                                fresh129 as libc::c_int != 0
                                                            }
                                                        {
                                                            if debugging(&mut pc) {
                                                                dbg_printf(
                                                                    gettext(
                                                                        b"error: %s:%d\n\0" as *const u8 as *const libc::c_char,
                                                                    ),
                                                                    b"parse-datetime.y\0" as *const u8 as *const libc::c_char,
                                                                    2143 as libc::c_int,
                                                                );
                                                            }
                                                            current_block = 18080936705779730158;
                                                        } else {
                                                            tm.tm_year = year;
                                                            tm.tm_mon = month;
                                                            tm.tm_mday = day;
                                                            tm.tm_hour = tm0.tm_hour;
                                                            tm.tm_min = tm0.tm_min;
                                                            tm.tm_sec = tm0.tm_sec;
                                                            tm.tm_isdst = tm0.tm_isdst;
                                                            tm.tm_wday = -(1 as libc::c_int);
                                                            Start = mktime_z(tz, &mut tm);
                                                            if tm.tm_wday < 0 as libc::c_int {
                                                                if debugging(&mut pc) {
                                                                    dbg_printf(
                                                                        gettext(
                                                                            b"error: adding relative date resulted in an invalid date: '%s'\n\0"
                                                                                as *const u8 as *const libc::c_char,
                                                                        ),
                                                                        debug_strfdatetime(
                                                                            &mut tm,
                                                                            &mut pc,
                                                                            dbg_tm.as_mut_ptr(),
                                                                            ::core::mem::size_of::<[libc::c_char; 100]>()
                                                                                as libc::c_ulong as libc::c_int,
                                                                        ),
                                                                    );
                                                                }
                                                                current_block = 18080936705779730158;
                                                            } else {
                                                                if debugging(&mut pc) {
                                                                    dbg_printf(
                                                                        gettext(
                                                                            b"after date adjustment (%+ld years, %+ld months, %+ld days),\n\0"
                                                                                as *const u8 as *const libc::c_char,
                                                                        ),
                                                                        pc.rel.year,
                                                                        pc.rel.month,
                                                                        pc.rel.day,
                                                                    );
                                                                    dbg_printf(
                                                                        gettext(
                                                                            b"    new date/time = '%s'\n\0" as *const u8
                                                                                as *const libc::c_char,
                                                                        ),
                                                                        debug_strfdatetime(
                                                                            &mut tm,
                                                                            &mut pc,
                                                                            dbg_tm.as_mut_ptr(),
                                                                            ::core::mem::size_of::<[libc::c_char; 100]>()
                                                                                as libc::c_ulong as libc::c_int,
                                                                        ),
                                                                    );
                                                                    if tm0.tm_isdst != -(1 as libc::c_int)
                                                                        && tm.tm_isdst != tm0.tm_isdst
                                                                    {
                                                                        dbg_printf(
                                                                            gettext(
                                                                                b"warning: daylight saving time changed after date adjustment\n\0"
                                                                                    as *const u8 as *const libc::c_char,
                                                                            ),
                                                                        );
                                                                    }
                                                                    if pc.rel.day == 0 as libc::c_int as libc::c_long
                                                                        && (tm.tm_mday != day
                                                                            || pc.rel.month == 0 as libc::c_int as libc::c_long
                                                                                && tm.tm_mon != month)
                                                                    {
                                                                        dbg_printf(
                                                                            gettext(
                                                                                b"warning: month/year adjustment resulted in shifted dates:\n\0"
                                                                                    as *const u8 as *const libc::c_char,
                                                                            ),
                                                                        );
                                                                        let mut tm_year_buf: [libc::c_char; 13] = [0; 13];
                                                                        dbg_printf(
                                                                            gettext(
                                                                                b"     adjusted Y M D: %s %02d %02d\n\0" as *const u8
                                                                                    as *const libc::c_char,
                                                                            ),
                                                                            tm_year_str(year, tm_year_buf.as_mut_ptr()),
                                                                            month + 1 as libc::c_int,
                                                                            day,
                                                                        );
                                                                        dbg_printf(
                                                                            gettext(
                                                                                b"   normalized Y M D: %s %02d %02d\n\0" as *const u8
                                                                                    as *const libc::c_char,
                                                                            ),
                                                                            tm_year_str(tm.tm_year, tm_year_buf.as_mut_ptr()),
                                                                            tm.tm_mon + 1 as libc::c_int,
                                                                            tm.tm_mday,
                                                                        );
                                                                    }
                                                                }
                                                                current_block = 7256998052328658819;
                                                            }
                                                        }
                                                    } else {
                                                        current_block = 7256998052328658819;
                                                    }
                                                    match current_block {
                                                        18080936705779730158 => {}
                                                        _ => {
                                                            if pc.zones_seen != 0 {
                                                                let mut overflow: bool = 0 as libc::c_int != 0;
                                                                let mut utcoff: libc::c_long = tm.tm_gmtoff;
                                                                let mut delta: intmax_t = 0;
                                                                let (fresh130, fresh131) = (pc.time_zone)
                                                                    .overflowing_sub(utcoff.try_into().unwrap());
                                                                *(&mut delta as *mut intmax_t) = fresh130;
                                                                overflow = (overflow as libc::c_int
                                                                    | fresh131 as libc::c_int) != 0;
                                                                let mut t1: time_t = 0;
                                                                let (fresh132, fresh133) = Start.overflowing_sub(delta);
                                                                *(&mut t1 as *mut time_t) = fresh132;
                                                                overflow = (overflow as libc::c_int
                                                                    | fresh133 as libc::c_int) != 0;
                                                                if overflow {
                                                                    if debugging(&mut pc) {
                                                                        dbg_printf(
                                                                            gettext(
                                                                                b"error: timezone %d caused time_t overflow\n\0"
                                                                                    as *const u8 as *const libc::c_char,
                                                                            ),
                                                                            pc.time_zone,
                                                                        );
                                                                    }
                                                                    current_block = 18080936705779730158;
                                                                } else {
                                                                    Start = t1;
                                                                    current_block = 2860109724005416757;
                                                                }
                                                            } else {
                                                                current_block = 2860109724005416757;
                                                            }
                                                            match current_block {
                                                                18080936705779730158 => {}
                                                                _ => {
                                                                    if debugging(&mut pc) {
                                                                        let mut Starti: intmax_t = Start;
                                                                        dbg_printf(
                                                                            gettext(
                                                                                b"'%s' = %ld epoch-seconds\n\0" as *const u8
                                                                                    as *const libc::c_char,
                                                                            ),
                                                                            debug_strfdatetime(
                                                                                &mut tm,
                                                                                &mut pc,
                                                                                dbg_tm.as_mut_ptr(),
                                                                                ::core::mem::size_of::<[libc::c_char; 100]>()
                                                                                    as libc::c_ulong as libc::c_int,
                                                                            ),
                                                                            Starti,
                                                                        );
                                                                    }
                                                                    let mut orig_ns: intmax_t = pc.seconds.tv_nsec;
                                                                    let mut sum_ns: intmax_t = orig_ns
                                                                        + pc.rel.ns as libc::c_long;
                                                                    let mut normalized_ns: libc::c_int = ((sum_ns
                                                                        % BILLION as libc::c_int as libc::c_long
                                                                        + BILLION as libc::c_int as libc::c_long)
                                                                        % BILLION as libc::c_int as libc::c_long) as libc::c_int;
                                                                    let mut d4: libc::c_int = ((sum_ns
                                                                        - normalized_ns as libc::c_long)
                                                                        / BILLION as libc::c_int as libc::c_long) as libc::c_int;
                                                                    let mut d1: intmax_t = 0;
                                                                    let mut t1_0: intmax_t = 0;
                                                                    let mut d2: intmax_t = 0;
                                                                    let mut t2: intmax_t = 0;
                                                                    let mut t3: intmax_t = 0;
                                                                    let mut t4: time_t = 0;
                                                                    if (if (0 as libc::c_int as intmax_t)
                                                                        < -(1 as libc::c_int) as intmax_t
                                                                        && ((if 1 as libc::c_int != 0 {
                                                                            0 as libc::c_int as libc::c_long
                                                                        } else {
                                                                            pc.rel.hour
                                                                        }) - 1 as libc::c_int as libc::c_long)
                                                                            < 0 as libc::c_int as libc::c_long
                                                                        && ((if 1 as libc::c_int != 0 {
                                                                            0 as libc::c_int
                                                                        } else {
                                                                            60 as libc::c_int * 60 as libc::c_int
                                                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                                                        && (if (60 as libc::c_int * 60 as libc::c_int)
                                                                            < 0 as libc::c_int
                                                                        {
                                                                            if pc.rel.hour < 0 as libc::c_int as libc::c_long {
                                                                                if ((if 1 as libc::c_int != 0 {
                                                                                    0 as libc::c_int as libc::c_long
                                                                                } else {
                                                                                    (if 1 as libc::c_int != 0 {
                                                                                        0 as libc::c_int as libc::c_long
                                                                                    } else {
                                                                                        -(1 as libc::c_int) as intmax_t
                                                                                    }) + (60 as libc::c_int * 60 as libc::c_int) as libc::c_long
                                                                                }) - 1 as libc::c_int as libc::c_long)
                                                                                    < 0 as libc::c_int as libc::c_long
                                                                                {
                                                                                    (pc.rel.hour
                                                                                        < -(1 as libc::c_int) as intmax_t
                                                                                            / (60 as libc::c_int * 60 as libc::c_int) as libc::c_long)
                                                                                        as libc::c_int
                                                                                } else {
                                                                                    ((if (if (if ((if 1 as libc::c_int != 0 {
                                                                                        0 as libc::c_int
                                                                                    } else {
                                                                                        60 as libc::c_int * 60 as libc::c_int
                                                                                    }) - 1 as libc::c_int) < 0 as libc::c_int
                                                                                    {
                                                                                        !(((((if 1 as libc::c_int != 0 {
                                                                                            0 as libc::c_int
                                                                                        } else {
                                                                                            60 as libc::c_int * 60 as libc::c_int
                                                                                        }) + 1 as libc::c_int)
                                                                                            << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                                                            - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                                                                    } else {
                                                                                        (if 1 as libc::c_int != 0 {
                                                                                            0 as libc::c_int
                                                                                        } else {
                                                                                            60 as libc::c_int * 60 as libc::c_int
                                                                                        }) + 0 as libc::c_int
                                                                                    }) < 0 as libc::c_int
                                                                                    {
                                                                                        ((60 as libc::c_int * 60 as libc::c_int)
                                                                                            < -(if ((if 1 as libc::c_int != 0 {
                                                                                                0 as libc::c_int
                                                                                            } else {
                                                                                                60 as libc::c_int * 60 as libc::c_int
                                                                                            }) - 1 as libc::c_int) < 0 as libc::c_int
                                                                                            {
                                                                                                ((((if 1 as libc::c_int != 0 {
                                                                                                    0 as libc::c_int
                                                                                                } else {
                                                                                                    60 as libc::c_int * 60 as libc::c_int
                                                                                                }) + 1 as libc::c_int)
                                                                                                    << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                                                                    - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                                                                            } else {
                                                                                                (if 1 as libc::c_int != 0 {
                                                                                                    0 as libc::c_int
                                                                                                } else {
                                                                                                    60 as libc::c_int * 60 as libc::c_int
                                                                                                }) - 1 as libc::c_int
                                                                                            })) as libc::c_int
                                                                                    } else {
                                                                                        ((0 as libc::c_int) < 60 as libc::c_int * 60 as libc::c_int)
                                                                                            as libc::c_int
                                                                                    }) != 0
                                                                                    {
                                                                                        (if 1 as libc::c_int != 0 {
                                                                                            0 as libc::c_int
                                                                                        } else {
                                                                                            60 as libc::c_int * 60 as libc::c_int
                                                                                        }) as libc::c_long + -(1 as libc::c_int) as intmax_t
                                                                                            >> (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                                                    } else {
                                                                                        -(1 as libc::c_int) as intmax_t
                                                                                            / -(60 as libc::c_int * 60 as libc::c_int) as libc::c_long
                                                                                    }) <= -(1 as libc::c_int) as libc::c_long - pc.rel.hour)
                                                                                        as libc::c_int
                                                                                }
                                                                            } else {
                                                                                if (if (if ((if 1 as libc::c_int != 0 {
                                                                                    0 as libc::c_int as libc::c_long
                                                                                } else {
                                                                                    (if 1 as libc::c_int != 0 {
                                                                                        0 as libc::c_int
                                                                                    } else {
                                                                                        60 as libc::c_int * 60 as libc::c_int
                                                                                    }) as libc::c_long + 0 as libc::c_int as intmax_t
                                                                                }) - 1 as libc::c_int as libc::c_long)
                                                                                    < 0 as libc::c_int as libc::c_long
                                                                                {
                                                                                    !(((((if 1 as libc::c_int != 0 {
                                                                                        0 as libc::c_int as libc::c_long
                                                                                    } else {
                                                                                        (if 1 as libc::c_int != 0 {
                                                                                            0 as libc::c_int
                                                                                        } else {
                                                                                            60 as libc::c_int * 60 as libc::c_int
                                                                                        }) as libc::c_long + 0 as libc::c_int as intmax_t
                                                                                    }) + 1 as libc::c_int as libc::c_long)
                                                                                        << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                                                        - 1 as libc::c_int as libc::c_long)
                                                                                        * 2 as libc::c_int as libc::c_long
                                                                                        + 1 as libc::c_int as libc::c_long)
                                                                                } else {
                                                                                    (if 1 as libc::c_int != 0 {
                                                                                        0 as libc::c_int as libc::c_long
                                                                                    } else {
                                                                                        (if 1 as libc::c_int != 0 {
                                                                                            0 as libc::c_int
                                                                                        } else {
                                                                                            60 as libc::c_int * 60 as libc::c_int
                                                                                        }) as libc::c_long + 0 as libc::c_int as intmax_t
                                                                                    }) + 0 as libc::c_int as libc::c_long
                                                                                }) < 0 as libc::c_int as libc::c_long
                                                                                {
                                                                                    (((if 1 as libc::c_int != 0 {
                                                                                        0 as libc::c_int
                                                                                    } else {
                                                                                        60 as libc::c_int * 60 as libc::c_int
                                                                                    }) as libc::c_long + 0 as libc::c_int as intmax_t)
                                                                                        < -(if ((if 1 as libc::c_int != 0 {
                                                                                            0 as libc::c_int as libc::c_long
                                                                                        } else {
                                                                                            (if 1 as libc::c_int != 0 {
                                                                                                0 as libc::c_int
                                                                                            } else {
                                                                                                60 as libc::c_int * 60 as libc::c_int
                                                                                            }) as libc::c_long + 0 as libc::c_int as intmax_t
                                                                                        }) - 1 as libc::c_int as libc::c_long)
                                                                                            < 0 as libc::c_int as libc::c_long
                                                                                        {
                                                                                            ((((if 1 as libc::c_int != 0 {
                                                                                                0 as libc::c_int as libc::c_long
                                                                                            } else {
                                                                                                (if 1 as libc::c_int != 0 {
                                                                                                    0 as libc::c_int
                                                                                                } else {
                                                                                                    60 as libc::c_int * 60 as libc::c_int
                                                                                                }) as libc::c_long + 0 as libc::c_int as intmax_t
                                                                                            }) + 1 as libc::c_int as libc::c_long)
                                                                                                << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                                                                - 1 as libc::c_int as libc::c_long)
                                                                                                * 2 as libc::c_int as libc::c_long
                                                                                                + 1 as libc::c_int as libc::c_long
                                                                                        } else {
                                                                                            (if 1 as libc::c_int != 0 {
                                                                                                0 as libc::c_int as libc::c_long
                                                                                            } else {
                                                                                                (if 1 as libc::c_int != 0 {
                                                                                                    0 as libc::c_int
                                                                                                } else {
                                                                                                    60 as libc::c_int * 60 as libc::c_int
                                                                                                }) as libc::c_long + 0 as libc::c_int as intmax_t
                                                                                            }) - 1 as libc::c_int as libc::c_long
                                                                                        })) as libc::c_int
                                                                                } else {
                                                                                    ((0 as libc::c_int as libc::c_long)
                                                                                        < (if 1 as libc::c_int != 0 {
                                                                                            0 as libc::c_int
                                                                                        } else {
                                                                                            60 as libc::c_int * 60 as libc::c_int
                                                                                        }) as libc::c_long + 0 as libc::c_int as intmax_t)
                                                                                        as libc::c_int
                                                                                }) != 0
                                                                                    && 60 as libc::c_int * 60 as libc::c_int
                                                                                        == -(1 as libc::c_int)
                                                                                {
                                                                                    if ((if 1 as libc::c_int != 0 {
                                                                                        0 as libc::c_int as libc::c_long
                                                                                    } else {
                                                                                        pc.rel.hour
                                                                                    }) - 1 as libc::c_int as libc::c_long)
                                                                                        < 0 as libc::c_int as libc::c_long
                                                                                    {
                                                                                        ((0 as libc::c_int as libc::c_long)
                                                                                            < pc.rel.hour + 0 as libc::c_int as intmax_t) as libc::c_int
                                                                                    } else {
                                                                                        ((0 as libc::c_int as libc::c_long) < pc.rel.hour
                                                                                            && (-(1 as libc::c_int) as libc::c_long
                                                                                                - 0 as libc::c_int as intmax_t)
                                                                                                < pc.rel.hour - 1 as libc::c_int as libc::c_long)
                                                                                            as libc::c_int
                                                                                    }
                                                                                } else {
                                                                                    ((0 as libc::c_int as intmax_t
                                                                                        / (60 as libc::c_int * 60 as libc::c_int) as libc::c_long)
                                                                                        < pc.rel.hour) as libc::c_int
                                                                                }
                                                                            }
                                                                        } else {
                                                                            if 60 as libc::c_int * 60 as libc::c_int
                                                                                == 0 as libc::c_int
                                                                            {
                                                                                0 as libc::c_int
                                                                            } else {
                                                                                if pc.rel.hour < 0 as libc::c_int as libc::c_long {
                                                                                    if (if (if ((if 1 as libc::c_int != 0 {
                                                                                        0 as libc::c_int as libc::c_long
                                                                                    } else {
                                                                                        (if 1 as libc::c_int != 0 {
                                                                                            0 as libc::c_int as libc::c_long
                                                                                        } else {
                                                                                            pc.rel.hour
                                                                                        }) + 0 as libc::c_int as intmax_t
                                                                                    }) - 1 as libc::c_int as libc::c_long)
                                                                                        < 0 as libc::c_int as libc::c_long
                                                                                    {
                                                                                        !(((((if 1 as libc::c_int != 0 {
                                                                                            0 as libc::c_int as libc::c_long
                                                                                        } else {
                                                                                            (if 1 as libc::c_int != 0 {
                                                                                                0 as libc::c_int as libc::c_long
                                                                                            } else {
                                                                                                pc.rel.hour
                                                                                            }) + 0 as libc::c_int as intmax_t
                                                                                        }) + 1 as libc::c_int as libc::c_long)
                                                                                            << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                                                            - 1 as libc::c_int as libc::c_long)
                                                                                            * 2 as libc::c_int as libc::c_long
                                                                                            + 1 as libc::c_int as libc::c_long)
                                                                                    } else {
                                                                                        (if 1 as libc::c_int != 0 {
                                                                                            0 as libc::c_int as libc::c_long
                                                                                        } else {
                                                                                            (if 1 as libc::c_int != 0 {
                                                                                                0 as libc::c_int as libc::c_long
                                                                                            } else {
                                                                                                pc.rel.hour
                                                                                            }) + 0 as libc::c_int as intmax_t
                                                                                        }) + 0 as libc::c_int as libc::c_long
                                                                                    }) < 0 as libc::c_int as libc::c_long
                                                                                    {
                                                                                        (((if 1 as libc::c_int != 0 {
                                                                                            0 as libc::c_int as libc::c_long
                                                                                        } else {
                                                                                            pc.rel.hour
                                                                                        }) + 0 as libc::c_int as intmax_t)
                                                                                            < -(if ((if 1 as libc::c_int != 0 {
                                                                                                0 as libc::c_int as libc::c_long
                                                                                            } else {
                                                                                                (if 1 as libc::c_int != 0 {
                                                                                                    0 as libc::c_int as libc::c_long
                                                                                                } else {
                                                                                                    pc.rel.hour
                                                                                                }) + 0 as libc::c_int as intmax_t
                                                                                            }) - 1 as libc::c_int as libc::c_long)
                                                                                                < 0 as libc::c_int as libc::c_long
                                                                                            {
                                                                                                ((((if 1 as libc::c_int != 0 {
                                                                                                    0 as libc::c_int as libc::c_long
                                                                                                } else {
                                                                                                    (if 1 as libc::c_int != 0 {
                                                                                                        0 as libc::c_int as libc::c_long
                                                                                                    } else {
                                                                                                        pc.rel.hour
                                                                                                    }) + 0 as libc::c_int as intmax_t
                                                                                                }) + 1 as libc::c_int as libc::c_long)
                                                                                                    << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                                                                    - 1 as libc::c_int as libc::c_long)
                                                                                                    * 2 as libc::c_int as libc::c_long
                                                                                                    + 1 as libc::c_int as libc::c_long
                                                                                            } else {
                                                                                                (if 1 as libc::c_int != 0 {
                                                                                                    0 as libc::c_int as libc::c_long
                                                                                                } else {
                                                                                                    (if 1 as libc::c_int != 0 {
                                                                                                        0 as libc::c_int as libc::c_long
                                                                                                    } else {
                                                                                                        pc.rel.hour
                                                                                                    }) + 0 as libc::c_int as intmax_t
                                                                                                }) - 1 as libc::c_int as libc::c_long
                                                                                            })) as libc::c_int
                                                                                    } else {
                                                                                        ((0 as libc::c_int as libc::c_long)
                                                                                            < (if 1 as libc::c_int != 0 {
                                                                                                0 as libc::c_int as libc::c_long
                                                                                            } else {
                                                                                                pc.rel.hour
                                                                                            }) + 0 as libc::c_int as intmax_t) as libc::c_int
                                                                                    }) != 0
                                                                                        && pc.rel.hour == -(1 as libc::c_int) as libc::c_long
                                                                                    {
                                                                                        if ((if 1 as libc::c_int != 0 {
                                                                                            0 as libc::c_int
                                                                                        } else {
                                                                                            60 as libc::c_int * 60 as libc::c_int
                                                                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                                                                        {
                                                                                            ((0 as libc::c_int as libc::c_long)
                                                                                                < (60 as libc::c_int * 60 as libc::c_int) as libc::c_long
                                                                                                    + 0 as libc::c_int as intmax_t) as libc::c_int
                                                                                        } else {
                                                                                            ((-(1 as libc::c_int) as libc::c_long
                                                                                                - 0 as libc::c_int as intmax_t)
                                                                                                < (60 as libc::c_int * 60 as libc::c_int - 1 as libc::c_int)
                                                                                                    as libc::c_long) as libc::c_int
                                                                                        }
                                                                                    } else {
                                                                                        (0 as libc::c_int as intmax_t / pc.rel.hour
                                                                                            < (60 as libc::c_int * 60 as libc::c_int) as libc::c_long)
                                                                                            as libc::c_int
                                                                                    }
                                                                                } else {
                                                                                    ((-(1 as libc::c_int) as intmax_t
                                                                                        / (60 as libc::c_int * 60 as libc::c_int) as libc::c_long)
                                                                                        < pc.rel.hour) as libc::c_int
                                                                                }
                                                                            }
                                                                        }) != 0
                                                                    {
                                                                        let (fresh138, fresh139) = (pc.rel.hour)
                                                                            .overflowing_mul((60 as libc::c_int * 60 as libc::c_int).into());
                                                                        *(&mut d1 as *mut intmax_t) = fresh138;
                                                                        1 as libc::c_int
                                                                    } else {
                                                                        let (fresh140, fresh141) = (pc.rel.hour)
                                                                            .overflowing_mul((60 as libc::c_int * 60 as libc::c_int).into());
                                                                        *(&mut d1 as *mut intmax_t) = fresh140;
                                                                        fresh141 as libc::c_int
                                                                    }) != 0
                                                                        || {
                                                                            let (fresh142, fresh143) = Start.overflowing_add(d1);
                                                                            *(&mut t1_0 as *mut intmax_t) = fresh142;
                                                                            fresh143 as libc::c_int != 0
                                                                        }
                                                                        || (if (0 as libc::c_int as intmax_t)
                                                                            < -(1 as libc::c_int) as intmax_t
                                                                            && ((if 1 as libc::c_int != 0 {
                                                                                0 as libc::c_int as libc::c_long
                                                                            } else {
                                                                                pc.rel.minutes
                                                                            }) - 1 as libc::c_int as libc::c_long)
                                                                                < 0 as libc::c_int as libc::c_long
                                                                            && ((if 1 as libc::c_int != 0 {
                                                                                0 as libc::c_int
                                                                            } else {
                                                                                60 as libc::c_int
                                                                            }) - 1 as libc::c_int) < 0 as libc::c_int
                                                                            && (if (60 as libc::c_int) < 0 as libc::c_int {
                                                                                if pc.rel.minutes < 0 as libc::c_int as libc::c_long {
                                                                                    if ((if 1 as libc::c_int != 0 {
                                                                                        0 as libc::c_int as libc::c_long
                                                                                    } else {
                                                                                        (if 1 as libc::c_int != 0 {
                                                                                            0 as libc::c_int as libc::c_long
                                                                                        } else {
                                                                                            -(1 as libc::c_int) as intmax_t
                                                                                        }) + 60 as libc::c_int as libc::c_long
                                                                                    }) - 1 as libc::c_int as libc::c_long)
                                                                                        < 0 as libc::c_int as libc::c_long
                                                                                    {
                                                                                        (pc.rel.minutes
                                                                                            < -(1 as libc::c_int) as intmax_t
                                                                                                / 60 as libc::c_int as libc::c_long) as libc::c_int
                                                                                    } else {
                                                                                        ((if (if (if ((if 1 as libc::c_int != 0 {
                                                                                            0 as libc::c_int
                                                                                        } else {
                                                                                            60 as libc::c_int
                                                                                        }) - 1 as libc::c_int) < 0 as libc::c_int
                                                                                        {
                                                                                            !(((((if 1 as libc::c_int != 0 {
                                                                                                0 as libc::c_int
                                                                                            } else {
                                                                                                60 as libc::c_int
                                                                                            }) + 1 as libc::c_int)
                                                                                                << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                                                                - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int)
                                                                                        } else {
                                                                                            (if 1 as libc::c_int != 0 {
                                                                                                0 as libc::c_int
                                                                                            } else {
                                                                                                60 as libc::c_int
                                                                                            }) + 0 as libc::c_int
                                                                                        }) < 0 as libc::c_int
                                                                                        {
                                                                                            ((60 as libc::c_int)
                                                                                                < -(if ((if 1 as libc::c_int != 0 {
                                                                                                    0 as libc::c_int
                                                                                                } else {
                                                                                                    60 as libc::c_int
                                                                                                }) - 1 as libc::c_int) < 0 as libc::c_int
                                                                                                {
                                                                                                    ((((if 1 as libc::c_int != 0 {
                                                                                                        0 as libc::c_int
                                                                                                    } else {
                                                                                                        60 as libc::c_int
                                                                                                    }) + 1 as libc::c_int)
                                                                                                        << (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                                                                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                                                                                                } else {
                                                                                                    (if 1 as libc::c_int != 0 {
                                                                                                        0 as libc::c_int
                                                                                                    } else {
                                                                                                        60 as libc::c_int
                                                                                                    }) - 1 as libc::c_int
                                                                                                })) as libc::c_int
                                                                                        } else {
                                                                                            ((0 as libc::c_int) < 60 as libc::c_int) as libc::c_int
                                                                                        }) != 0
                                                                                        {
                                                                                            (if 1 as libc::c_int != 0 {
                                                                                                0 as libc::c_int
                                                                                            } else {
                                                                                                60 as libc::c_int
                                                                                            }) as libc::c_long + -(1 as libc::c_int) as intmax_t
                                                                                                >> (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                                                                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                                                        } else {
                                                                                            -(1 as libc::c_int) as intmax_t
                                                                                                / -(60 as libc::c_int) as libc::c_long
                                                                                        }) <= -(1 as libc::c_int) as libc::c_long - pc.rel.minutes)
                                                                                            as libc::c_int
                                                                                    }
                                                                                } else {
                                                                                    if (if (if ((if 1 as libc::c_int != 0 {
                                                                                        0 as libc::c_int as libc::c_long
                                                                                    } else {
                                                                                        (if 1 as libc::c_int != 0 {
                                                                                            0 as libc::c_int
                                                                                        } else {
                                                                                            60 as libc::c_int
                                                                                        }) as libc::c_long + 0 as libc::c_int as intmax_t
                                                                                    }) - 1 as libc::c_int as libc::c_long)
                                                                                        < 0 as libc::c_int as libc::c_long
                                                                                    {
                                                                                        !(((((if 1 as libc::c_int != 0 {
                                                                                            0 as libc::c_int as libc::c_long
                                                                                        } else {
                                                                                            (if 1 as libc::c_int != 0 {
                                                                                                0 as libc::c_int
                                                                                            } else {
                                                                                                60 as libc::c_int
                                                                                            }) as libc::c_long + 0 as libc::c_int as intmax_t
                                                                                        }) + 1 as libc::c_int as libc::c_long)
                                                                                            << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                                                            - 1 as libc::c_int as libc::c_long)
                                                                                            * 2 as libc::c_int as libc::c_long
                                                                                            + 1 as libc::c_int as libc::c_long)
                                                                                    } else {
                                                                                        (if 1 as libc::c_int != 0 {
                                                                                            0 as libc::c_int as libc::c_long
                                                                                        } else {
                                                                                            (if 1 as libc::c_int != 0 {
                                                                                                0 as libc::c_int
                                                                                            } else {
                                                                                                60 as libc::c_int
                                                                                            }) as libc::c_long + 0 as libc::c_int as intmax_t
                                                                                        }) + 0 as libc::c_int as libc::c_long
                                                                                    }) < 0 as libc::c_int as libc::c_long
                                                                                    {
                                                                                        (((if 1 as libc::c_int != 0 {
                                                                                            0 as libc::c_int
                                                                                        } else {
                                                                                            60 as libc::c_int
                                                                                        }) as libc::c_long + 0 as libc::c_int as intmax_t)
                                                                                            < -(if ((if 1 as libc::c_int != 0 {
                                                                                                0 as libc::c_int as libc::c_long
                                                                                            } else {
                                                                                                (if 1 as libc::c_int != 0 {
                                                                                                    0 as libc::c_int
                                                                                                } else {
                                                                                                    60 as libc::c_int
                                                                                                }) as libc::c_long + 0 as libc::c_int as intmax_t
                                                                                            }) - 1 as libc::c_int as libc::c_long)
                                                                                                < 0 as libc::c_int as libc::c_long
                                                                                            {
                                                                                                ((((if 1 as libc::c_int != 0 {
                                                                                                    0 as libc::c_int as libc::c_long
                                                                                                } else {
                                                                                                    (if 1 as libc::c_int != 0 {
                                                                                                        0 as libc::c_int
                                                                                                    } else {
                                                                                                        60 as libc::c_int
                                                                                                    }) as libc::c_long + 0 as libc::c_int as intmax_t
                                                                                                }) + 1 as libc::c_int as libc::c_long)
                                                                                                    << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                                                                    - 1 as libc::c_int as libc::c_long)
                                                                                                    * 2 as libc::c_int as libc::c_long
                                                                                                    + 1 as libc::c_int as libc::c_long
                                                                                            } else {
                                                                                                (if 1 as libc::c_int != 0 {
                                                                                                    0 as libc::c_int as libc::c_long
                                                                                                } else {
                                                                                                    (if 1 as libc::c_int != 0 {
                                                                                                        0 as libc::c_int
                                                                                                    } else {
                                                                                                        60 as libc::c_int
                                                                                                    }) as libc::c_long + 0 as libc::c_int as intmax_t
                                                                                                }) - 1 as libc::c_int as libc::c_long
                                                                                            })) as libc::c_int
                                                                                    } else {
                                                                                        ((0 as libc::c_int as libc::c_long)
                                                                                            < (if 1 as libc::c_int != 0 {
                                                                                                0 as libc::c_int
                                                                                            } else {
                                                                                                60 as libc::c_int
                                                                                            }) as libc::c_long + 0 as libc::c_int as intmax_t)
                                                                                            as libc::c_int
                                                                                    }) != 0 && 60 as libc::c_int == -(1 as libc::c_int)
                                                                                    {
                                                                                        if ((if 1 as libc::c_int != 0 {
                                                                                            0 as libc::c_int as libc::c_long
                                                                                        } else {
                                                                                            pc.rel.minutes
                                                                                        }) - 1 as libc::c_int as libc::c_long)
                                                                                            < 0 as libc::c_int as libc::c_long
                                                                                        {
                                                                                            ((0 as libc::c_int as libc::c_long)
                                                                                                < pc.rel.minutes + 0 as libc::c_int as intmax_t)
                                                                                                as libc::c_int
                                                                                        } else {
                                                                                            ((0 as libc::c_int as libc::c_long) < pc.rel.minutes
                                                                                                && (-(1 as libc::c_int) as libc::c_long
                                                                                                    - 0 as libc::c_int as intmax_t)
                                                                                                    < pc.rel.minutes - 1 as libc::c_int as libc::c_long)
                                                                                                as libc::c_int
                                                                                        }
                                                                                    } else {
                                                                                        ((0 as libc::c_int as intmax_t
                                                                                            / 60 as libc::c_int as libc::c_long) < pc.rel.minutes)
                                                                                            as libc::c_int
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                if 60 as libc::c_int == 0 as libc::c_int {
                                                                                    0 as libc::c_int
                                                                                } else {
                                                                                    if pc.rel.minutes < 0 as libc::c_int as libc::c_long {
                                                                                        if (if (if ((if 1 as libc::c_int != 0 {
                                                                                            0 as libc::c_int as libc::c_long
                                                                                        } else {
                                                                                            (if 1 as libc::c_int != 0 {
                                                                                                0 as libc::c_int as libc::c_long
                                                                                            } else {
                                                                                                pc.rel.minutes
                                                                                            }) + 0 as libc::c_int as intmax_t
                                                                                        }) - 1 as libc::c_int as libc::c_long)
                                                                                            < 0 as libc::c_int as libc::c_long
                                                                                        {
                                                                                            !(((((if 1 as libc::c_int != 0 {
                                                                                                0 as libc::c_int as libc::c_long
                                                                                            } else {
                                                                                                (if 1 as libc::c_int != 0 {
                                                                                                    0 as libc::c_int as libc::c_long
                                                                                                } else {
                                                                                                    pc.rel.minutes
                                                                                                }) + 0 as libc::c_int as intmax_t
                                                                                            }) + 1 as libc::c_int as libc::c_long)
                                                                                                << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                                                                - 1 as libc::c_int as libc::c_long)
                                                                                                * 2 as libc::c_int as libc::c_long
                                                                                                + 1 as libc::c_int as libc::c_long)
                                                                                        } else {
                                                                                            (if 1 as libc::c_int != 0 {
                                                                                                0 as libc::c_int as libc::c_long
                                                                                            } else {
                                                                                                (if 1 as libc::c_int != 0 {
                                                                                                    0 as libc::c_int as libc::c_long
                                                                                                } else {
                                                                                                    pc.rel.minutes
                                                                                                }) + 0 as libc::c_int as intmax_t
                                                                                            }) + 0 as libc::c_int as libc::c_long
                                                                                        }) < 0 as libc::c_int as libc::c_long
                                                                                        {
                                                                                            (((if 1 as libc::c_int != 0 {
                                                                                                0 as libc::c_int as libc::c_long
                                                                                            } else {
                                                                                                pc.rel.minutes
                                                                                            }) + 0 as libc::c_int as intmax_t)
                                                                                                < -(if ((if 1 as libc::c_int != 0 {
                                                                                                    0 as libc::c_int as libc::c_long
                                                                                                } else {
                                                                                                    (if 1 as libc::c_int != 0 {
                                                                                                        0 as libc::c_int as libc::c_long
                                                                                                    } else {
                                                                                                        pc.rel.minutes
                                                                                                    }) + 0 as libc::c_int as intmax_t
                                                                                                }) - 1 as libc::c_int as libc::c_long)
                                                                                                    < 0 as libc::c_int as libc::c_long
                                                                                                {
                                                                                                    ((((if 1 as libc::c_int != 0 {
                                                                                                        0 as libc::c_int as libc::c_long
                                                                                                    } else {
                                                                                                        (if 1 as libc::c_int != 0 {
                                                                                                            0 as libc::c_int as libc::c_long
                                                                                                        } else {
                                                                                                            pc.rel.minutes
                                                                                                        }) + 0 as libc::c_int as intmax_t
                                                                                                    }) + 1 as libc::c_int as libc::c_long)
                                                                                                        << (::core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                                                                                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                                                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                                                                        - 1 as libc::c_int as libc::c_long)
                                                                                                        * 2 as libc::c_int as libc::c_long
                                                                                                        + 1 as libc::c_int as libc::c_long
                                                                                                } else {
                                                                                                    (if 1 as libc::c_int != 0 {
                                                                                                        0 as libc::c_int as libc::c_long
                                                                                                    } else {
                                                                                                        (if 1 as libc::c_int != 0 {
                                                                                                            0 as libc::c_int as libc::c_long
                                                                                                        } else {
                                                                                                            pc.rel.minutes
                                                                                                        }) + 0 as libc::c_int as intmax_t
                                                                                                    }) - 1 as libc::c_int as libc::c_long
                                                                                                })) as libc::c_int
                                                                                        } else {
                                                                                            ((0 as libc::c_int as libc::c_long)
                                                                                                < (if 1 as libc::c_int != 0 {
                                                                                                    0 as libc::c_int as libc::c_long
                                                                                                } else {
                                                                                                    pc.rel.minutes
                                                                                                }) + 0 as libc::c_int as intmax_t) as libc::c_int
                                                                                        }) != 0
                                                                                            && pc.rel.minutes == -(1 as libc::c_int) as libc::c_long
                                                                                        {
                                                                                            if ((if 1 as libc::c_int != 0 {
                                                                                                0 as libc::c_int
                                                                                            } else {
                                                                                                60 as libc::c_int
                                                                                            }) - 1 as libc::c_int) < 0 as libc::c_int
                                                                                            {
                                                                                                ((0 as libc::c_int as libc::c_long)
                                                                                                    < 60 as libc::c_int as libc::c_long
                                                                                                        + 0 as libc::c_int as intmax_t) as libc::c_int
                                                                                            } else {
                                                                                                ((-(1 as libc::c_int) as libc::c_long
                                                                                                    - 0 as libc::c_int as intmax_t)
                                                                                                    < (60 as libc::c_int - 1 as libc::c_int) as libc::c_long)
                                                                                                    as libc::c_int
                                                                                            }
                                                                                        } else {
                                                                                            (0 as libc::c_int as intmax_t / pc.rel.minutes
                                                                                                < 60 as libc::c_int as libc::c_long) as libc::c_int
                                                                                        }
                                                                                    } else {
                                                                                        ((-(1 as libc::c_int) as intmax_t
                                                                                            / 60 as libc::c_int as libc::c_long) < pc.rel.minutes)
                                                                                            as libc::c_int
                                                                                    }
                                                                                }
                                                                            }) != 0
                                                                        {
                                                                            let (fresh148, fresh149) = (pc.rel.minutes)
                                                                                .overflowing_mul((60 as libc::c_int).into());
                                                                            *(&mut d2 as *mut intmax_t) = fresh148;
                                                                            1 as libc::c_int
                                                                        } else {
                                                                            let (fresh150, fresh151) = (pc.rel.minutes)
                                                                                .overflowing_mul((60 as libc::c_int).into());
                                                                            *(&mut d2 as *mut intmax_t) = fresh150;
                                                                            fresh151 as libc::c_int
                                                                        }) != 0
                                                                        || {
                                                                            let (fresh152, fresh153) = t1_0.overflowing_add(d2);
                                                                            *(&mut t2 as *mut intmax_t) = fresh152;
                                                                            fresh153 as libc::c_int != 0
                                                                        }
                                                                        || {
                                                                            let (fresh154, fresh155) = t2
                                                                                .overflowing_add(pc.rel.seconds);
                                                                            *(&mut t3 as *mut intmax_t) = fresh154;
                                                                            fresh155 as libc::c_int != 0
                                                                        }
                                                                        || {
                                                                            let (fresh156, fresh157) = t3.overflowing_add(d4.into());
                                                                            *(&mut t4 as *mut time_t) = fresh156;
                                                                            fresh157 as libc::c_int != 0
                                                                        }
                                                                    {
                                                                        if debugging(&mut pc) {
                                                                            dbg_printf(
                                                                                gettext(
                                                                                    b"error: adding relative time caused an overflow\n\0"
                                                                                        as *const u8 as *const libc::c_char,
                                                                                ),
                                                                            );
                                                                        }
                                                                        current_block = 18080936705779730158;
                                                                    } else {
                                                                        (*result).tv_sec = t4;
                                                                        (*result).tv_nsec = normalized_ns as __syscall_slong_t;
                                                                        if debugging(&mut pc) as libc::c_int != 0
                                                                            && pc.rel.hour | pc.rel.minutes | pc.rel.seconds
                                                                                | pc.rel.ns as libc::c_long != 0
                                                                        {
                                                                            dbg_printf(
                                                                                gettext(
                                                                                    b"after time adjustment (%+ld hours, %+ld minutes, %+ld seconds, %+d ns),\n\0"
                                                                                        as *const u8 as *const libc::c_char,
                                                                                ),
                                                                                pc.rel.hour,
                                                                                pc.rel.minutes,
                                                                                pc.rel.seconds,
                                                                                pc.rel.ns,
                                                                            );
                                                                            let mut t4i: intmax_t = t4;
                                                                            dbg_printf(
                                                                                gettext(
                                                                                    b"    new time = %ld epoch-seconds\n\0" as *const u8
                                                                                        as *const libc::c_char,
                                                                                ),
                                                                                t4i,
                                                                            );
                                                                            let mut lmt: tm = tm {
                                                                                tm_sec: 0,
                                                                                tm_min: 0,
                                                                                tm_hour: 0,
                                                                                tm_mday: 0,
                                                                                tm_mon: 0,
                                                                                tm_year: 0,
                                                                                tm_wday: 0,
                                                                                tm_yday: 0,
                                                                                tm_isdst: 0,
                                                                                tm_gmtoff: 0,
                                                                                tm_zone: 0 as *const libc::c_char,
                                                                            };
                                                                            if tm.tm_isdst != -(1 as libc::c_int)
                                                                                && !(localtime_rz(tz, &mut (*result).tv_sec, &mut lmt))
                                                                                    .is_null() && tm.tm_isdst != lmt.tm_isdst
                                                                            {
                                                                                dbg_printf(
                                                                                    gettext(
                                                                                        b"warning: daylight saving time changed after time adjustment\n\0"
                                                                                            as *const u8 as *const libc::c_char,
                                                                                    ),
                                                                                );
                                                                            }
                                                                        }
                                                                        current_block = 7188795011561844502;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        match current_block {
                            18080936705779730158 => {}
                            _ => {
                                if debugging(&mut pc) {
                                    if tzstring.is_null() {
                                        dbg_printf(
                                            gettext(
                                                b"timezone: system default\n\0" as *const u8
                                                    as *const libc::c_char,
                                            ),
                                        );
                                    } else if strcmp(
                                        tzstring,
                                        b"UTC0\0" as *const u8 as *const libc::c_char,
                                    ) == 0 as libc::c_int
                                    {
                                        dbg_printf(
                                            gettext(
                                                b"timezone: Universal Time\n\0" as *const u8
                                                    as *const libc::c_char,
                                            ),
                                        );
                                    } else {
                                        dbg_printf(
                                            gettext(
                                                b"timezone: TZ=\"%s\" environment value\n\0" as *const u8
                                                    as *const libc::c_char,
                                            ),
                                            tzstring,
                                        );
                                    }
                                    let mut sec: intmax_t = (*result).tv_sec;
                                    let mut nsec: libc::c_int = (*result).tv_nsec
                                        as libc::c_int;
                                    dbg_printf(
                                        gettext(
                                            b"final: %ld.%09d (epoch-seconds)\n\0" as *const u8
                                                as *const libc::c_char,
                                        ),
                                        sec,
                                        nsec,
                                    );
                                    let mut gmt: tm = tm {
                                        tm_sec: 0,
                                        tm_min: 0,
                                        tm_hour: 0,
                                        tm_mday: 0,
                                        tm_mon: 0,
                                        tm_year: 0,
                                        tm_wday: 0,
                                        tm_yday: 0,
                                        tm_isdst: 0,
                                        tm_gmtoff: 0,
                                        tm_zone: 0 as *const libc::c_char,
                                    };
                                    let mut lmt_0: tm = tm {
                                        tm_sec: 0,
                                        tm_min: 0,
                                        tm_hour: 0,
                                        tm_mday: 0,
                                        tm_mon: 0,
                                        tm_year: 0,
                                        tm_wday: 0,
                                        tm_yday: 0,
                                        tm_isdst: 0,
                                        tm_gmtoff: 0,
                                        tm_zone: 0 as *const libc::c_char,
                                    };
                                    let mut got_utc: bool = !(gmtime_r(
                                        &mut (*result).tv_sec,
                                        &mut gmt,
                                    ))
                                        .is_null();
                                    if got_utc {
                                        dbg_printf(
                                            gettext(
                                                b"final: %s (UTC)\n\0" as *const u8 as *const libc::c_char,
                                            ),
                                            debug_strfdatetime(
                                                &mut gmt,
                                                0 as *const parser_control,
                                                dbg_tm.as_mut_ptr(),
                                                ::core::mem::size_of::<[libc::c_char; 100]>()
                                                    as libc::c_ulong as libc::c_int,
                                            ),
                                        );
                                    }
                                    if !(localtime_rz(tz, &mut (*result).tv_sec, &mut lmt_0))
                                        .is_null()
                                    {
                                        let mut got_utcoff: bool = 1 as libc::c_int != 0;
                                        let mut utcoff_0: libc::c_long = lmt_0.tm_gmtoff;
                                        if got_utcoff {
                                            dbg_printf(
                                                gettext(
                                                    b"final: %s (UTC%s)\n\0" as *const u8 as *const libc::c_char,
                                                ),
                                                debug_strfdatetime(
                                                    &mut lmt_0,
                                                    0 as *const parser_control,
                                                    dbg_tm.as_mut_ptr(),
                                                    ::core::mem::size_of::<[libc::c_char; 100]>()
                                                        as libc::c_ulong as libc::c_int,
                                                ),
                                                time_zone_str(
                                                    utcoff_0 as libc::c_int,
                                                    time_zone_buf.as_mut_ptr(),
                                                ),
                                            );
                                        } else {
                                            dbg_printf(
                                                gettext(
                                                    b"final: %s (unknown time zone offset)\n\0" as *const u8
                                                        as *const libc::c_char,
                                                ),
                                                debug_strfdatetime(
                                                    &mut lmt_0,
                                                    0 as *const parser_control,
                                                    dbg_tm.as_mut_ptr(),
                                                    ::core::mem::size_of::<[libc::c_char; 100]>()
                                                        as libc::c_ulong as libc::c_int,
                                                ),
                                            );
                                        }
                                    }
                                }
                                ok = 1 as libc::c_int != 0;
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    if tz != tzdefault {
        tzfree(tz);
    }
    free(tz1alloc as *mut libc::c_void);
    return ok;
}
#[no_mangle]
pub unsafe extern "C" fn parse_datetime2(
    mut result: *mut timespec,
    mut p: *const libc::c_char,
    mut now: *const timespec,
    mut flags: libc::c_uint,
    mut tzdefault: timezone_t,
    mut tzstring: *const libc::c_char,
) -> bool {
    return parse_datetime_body(result, p, now, flags, tzdefault, tzstring);
}
#[no_mangle]
pub unsafe extern "C" fn parse_datetime(
    mut result: *mut timespec,
    mut p: *const libc::c_char,
    mut now: *const timespec,
) -> bool {
    let mut tzstring: *const libc::c_char = getenv(
        b"TZ\0" as *const u8 as *const libc::c_char,
    );
    let mut tz: timezone_t = tzalloc(tzstring);
    if tz.is_null() {
        return 0 as libc::c_int != 0;
    }
    let mut ok: bool = parse_datetime_body(
        result,
        p,
        now,
        0 as libc::c_int as libc::c_uint,
        tz,
        tzstring,
    );
    tzfree(tz);
    return ok;
}
