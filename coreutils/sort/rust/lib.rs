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

#[macro_use]
extern crate c2rust_bitfields;
extern crate libc;
pub mod src {
pub mod argmatch;
pub mod bitrotate;
pub mod c32iscntrl;
pub mod c32isprint;
pub mod c32width;
pub mod c_ctype;
pub mod c_strcasecmp;
pub mod close_stream;
pub mod closeout;
pub mod dtotimespec;
pub mod dup_safer;
pub mod dup_safer_flag;
pub mod exitfail;
pub mod fadvise;
pub mod fclose;
pub mod fcntl;
pub mod fd_safer;
pub mod fd_safer_flag;
pub mod fflush;
pub mod filevercmp;
pub mod fopen;
pub mod fopen_safer;
pub mod fseeko;
pub mod full_read;
pub mod hard_locale;
pub mod hash;
pub mod heap;
pub mod ialloc;
pub mod inttostr;
pub mod localcharset;
pub mod mbrtoc32;
pub mod mbswidth;
pub mod mbszero;
pub mod md5_stream;
pub mod memcoll;
pub mod mkstemp_safer;
pub mod nanosleep;
pub mod nproc;
pub mod obstack;
pub mod physmem;
pub mod pipe2;
pub mod posixver;
pub mod progname;
pub mod propername_lite;
pub mod quotearg;
pub mod rand_isaac;
pub mod randread;
pub mod readtokens0;
pub mod safe_read;
pub mod same_inode;
pub mod setlocale_null;
pub mod setlocale_null_unlocked;
pub mod strnumcmp;
pub mod timespec;
pub mod uinttostr;
pub mod umaxtostr;
pub mod version;
pub mod version_etc;
pub mod version_etc_fsf;
pub mod xalloc_die;
pub mod xmalloc;
pub mod xmemcoll;
pub mod xnanosleep;
pub mod xstrtol_error;
pub mod xstrtoumax;
} // mod src
