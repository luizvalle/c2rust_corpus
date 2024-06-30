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
pub mod asprintf;
pub mod c32isprint;
pub mod c_ctype;
pub mod c_strcasecmp;
pub mod canon_host;
pub mod close_stream;
pub mod closeout;
pub mod exitfail;
pub mod fclose;
pub mod fflush;
pub mod fopen;
pub mod fseeko;
pub mod hard_locale;
pub mod ialloc;
pub mod imaxtostr;
pub mod localcharset;
pub mod mbrtoc32;
pub mod mbszero;
pub mod printf_args;
pub mod printf_parse;
pub mod progname;
pub mod propername_lite;
pub mod quotearg;
pub mod readutmp;
pub mod setlocale_null;
pub mod setlocale_null_unlocked;
pub mod stat_time;
pub mod time;
pub mod vasnprintf;
pub mod vasprintf;
pub mod version;
pub mod version_etc;
pub mod version_etc_fsf;
pub mod xalloc_die;
pub mod xmalloc;
pub mod xsize;
} // mod src
