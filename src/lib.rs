#![feature(libc)]

extern crate libc;

mod bash;

extern fn builtin_lulz(_: *mut bash::word_list) -> libc::c_int {
    println!("lulz");
    0
}

#[no_mangle]
pub static mut lulz_struct: bash::builtin = bash::builtin {
    name: b"lulz\0" as *const u8 as *const libc::c_char,
    function: builtin_lulz,
    flags: bash::BUILTIN_ENABLED,
    long_doc: 0 as *const *const libc::c_char,
    short_doc: b"lolwut\0" as *const u8 as *const libc::c_char,
    handle: 0 as *const i8,
};
