extern crate libc;
mod bash;

extern fn builtin_lulz(args: *mut bash::word_list) -> libc::c_int {
    println!("lulz");
    0
}

static NAME: [i8; 5] = ['l' as i8, 'u' as i8, 'l' as i8, 'z' as i8, 0 as i8];

#[no_mangle]
pub static mut lulz_struct: bash::builtin = bash::builtin {
    name: &NAME as *const i8,
    function: builtin_lulz,
    flags: bash::BUILTIN_ENABLED,
    long_doc: 0 as *const *const i8,
    short_doc: &NAME as *const i8,
    handle: 0 as *const i8,
};
