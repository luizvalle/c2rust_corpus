#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(extern_types)]
#![feature(linkage)]


extern crate libc;
pub mod src {
pub mod argmatch;
pub mod c32isprint;
pub mod c_ctype;
pub mod c_strcasecmp;
pub mod close_stream;
pub mod closeout;
pub mod exitfail;
pub mod fclose;
pub mod fflush;
pub mod fopen;
pub mod fprintftime;
pub mod fseeko;
pub mod fseterr;
pub mod gettime;
pub mod gettime_res;
pub mod hard_locale;
pub mod ialloc;
pub mod imaxtostr;
pub mod localcharset;
pub mod mbrtoc32;
pub mod mbszero;
pub mod mktime;
pub mod nstrftime;
pub mod parse_datetime;
pub mod posixtm;
pub mod printf_args;
pub mod printf_parse;
pub mod progname;
pub mod propername_lite;
pub mod quotearg;
pub mod setlocale_null;
pub mod setlocale_null_unlocked;
pub mod settime;
pub mod show_date;
pub mod stat_time;
pub mod time;
pub mod time_rz;
pub mod timegm;
pub mod vasnprintf;
pub mod version;
pub mod version_etc;
pub mod version_etc_fsf;
pub mod vfprintf;
pub mod xalloc_die;
pub mod xmalloc;
pub mod xsize;
} // mod src
