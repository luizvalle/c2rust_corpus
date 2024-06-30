#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(linkage)]


extern crate libc;
pub mod src {
pub mod c32isblank;
pub mod c32isprint;
pub mod c_ctype;
pub mod c_strcasecmp;
pub mod close_stream;
pub mod closeout;
pub mod dup_safer;
pub mod exitfail;
pub mod fadvise;
pub mod fclose;
pub mod fcntl;
pub mod fflush;
pub mod fopen;
pub mod fopen_safer;
pub mod fpurge;
pub mod fseeko;
pub mod hard_locale;
pub mod ialloc;
pub mod linebuffer;
pub mod localcharset;
pub mod mbrtoc32;
pub mod mbszero;
pub mod mcel;
pub mod memcasecmp;
pub mod memcoll;
pub mod progname;
pub mod propername_lite;
pub mod quotearg;
pub mod setlocale_null;
pub mod setlocale_null_unlocked;
pub mod skipchars;
pub mod version;
pub mod version_etc;
pub mod version_etc_fsf;
pub mod xalloc_die;
pub mod xmalloc;
pub mod xmemcoll;
pub mod xstrtoimax;
pub mod xstrtol;
} // mod src
