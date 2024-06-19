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

#[macro_use]
extern crate c2rust_bitfields;
extern crate libc;
pub mod src {
pub mod algo_splaytree;
pub mod array;
pub mod base64;
pub mod buffer;
pub mod burl;
pub mod chunk;
pub mod ck;
pub mod configfile;
pub mod configfile_glue;
pub mod configparser;
pub mod connections;
pub mod data_config;
pub mod fdevent;
pub mod fdevent_fdnode;
pub mod fdevent_impl;
pub mod fdlog;
pub mod fdlog_maint;
pub mod h1;
pub mod http_cgi;
pub mod http_chunk;
pub mod http_date;
pub mod http_etag;
pub mod http_header;
pub mod http_header_glue;
pub mod http_kv;
pub mod http_range;
pub mod keyvalue;
pub mod log;
pub mod network;
pub mod network_write;
pub mod plugin;
pub mod rand;
pub mod reqpool;
pub mod request;
pub mod response;
pub mod sock_addr;
pub mod sock_addr_cache;
pub mod stat_cache;
pub mod sys_setjmp;
} // mod src
