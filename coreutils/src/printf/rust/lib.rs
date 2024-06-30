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


extern crate f128;
extern crate libc;
pub mod src {
pub mod c32isprint;
pub mod c_ctype;
pub mod c_strcasecmp;
pub mod c_strtold;
pub mod cl_strtold;
pub mod close_stream;
pub mod closeout;
pub mod exitfail;
pub mod fclose;
pub mod fflush;
pub mod fseeko;
pub mod fseterr;
pub mod hard_locale;
pub mod ialloc;
pub mod localcharset;
pub mod mbrtoc32;
pub mod mbrtowc;
pub mod mbszero;
pub mod printf_args;
pub mod printf_parse;
pub mod progname;
pub mod propername_lite;
pub mod quotearg;
pub mod setlocale_null;
pub mod setlocale_null_unlocked;
pub mod u8_uctomb_aux;
pub mod unicodeio;
pub mod vasnprintf;
pub mod version;
pub mod version_etc;
pub mod version_etc_fsf;
pub mod vfprintf;
pub mod vprintf;
pub mod xalloc_die;
pub mod xmalloc;
pub mod xprintf;
pub mod xsize;
} // mod src
