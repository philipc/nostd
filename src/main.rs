#![feature(lang_items, core_intrinsics)]
#![feature(start)]
#![feature(compiler_builtins_lib)]
#![no_std]
#![no_main]
use core::intrinsics;

// Pull in the system libc library for what crt0.o likely requires.
extern crate libc;
//extern crate compiler_builtins;

extern crate object;
//extern crate addr2line;



// Entry point for this program.
#[no_mangle] // ensure that this symbol is called `main` in the output
pub extern fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    let data = [0; 32];
    let object = object::File::parse(&data[..]);
    if object.is_ok() {
        //let context = addr2line::Context::new(&object);
        0
    } else {
        1
    }
}

// These functions are used by the compiler, but not
// for a bare-bones hello world. These are normally
// provided by libstd.
#[lang = "eh_personality"]
#[no_mangle]
pub extern fn rust_eh_personality() {
}

// This function may be needed based on the compilation target.
#[lang = "eh_unwind_resume"]
#[no_mangle]
pub extern fn rust_eh_unwind_resume() {
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(_msg: core::fmt::Arguments,
                               _file: &'static str,
                               _line: u32,
                               _column: u32) -> ! {
    unsafe { intrinsics::abort() }
}

#[lang = "oom"]
#[no_mangle]
pub fn oom() -> ! {
    unsafe { intrinsics::abort() }
}
