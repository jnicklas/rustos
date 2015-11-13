#![feature(no_std, lang_items, const_fn, unique, core_str_ext)]
#![no_std]

#[macro_use]
mod vga_buffer;

use vga_buffer::{set_color, clear_screen, Color};

use core::fmt::Write;

extern crate rlibc;
extern crate spin;

#[no_mangle]
pub extern fn rust_main() {
    clear_screen();
    println!("The numbers are {} and {}", 42, 1.0/3.0);
    set_color(Color::Yellow, Color::Brown);
    println!("This is ugly!");

    loop{}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! {loop{}}
