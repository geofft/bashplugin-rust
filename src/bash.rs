#![allow(non_camel_case_types, dead_code)]

extern crate libc;

// command.h

#[repr(C)]
pub struct word_desc {
    word: *mut libc::c_char,
    flags: libc::c_int,
}

pub struct word_list {
    next: *mut word_list,
    word: *mut word_desc,
}

// general.h

pub type sh_builtin_func_t = extern fn (*mut word_list) -> libc::c_int;

// builtins.h

pub const BUILTIN_ENABLED: libc::c_int = 0x01;
pub const BUILTIN_DELETED: libc::c_int = 0x02;
pub const STATIC_BUILTIN: libc::c_int = 0x04;
pub const SPECIAL_BUILTIN: libc::c_int = 0x08;
pub const ASSIGNMENT_BUILTIN: libc::c_int = 0x10;
pub const POSIX_BUILTIN: libc::c_int = 0x20;

#[repr(C)]
pub struct builtin {
    pub name: *const libc::c_char,
    pub function: sh_builtin_func_t,
    pub flags: libc::c_int,
    pub long_doc: *const *const libc::c_char,
    pub short_doc: *const libc::c_char,
    pub handle: *const libc::c_char,
}


