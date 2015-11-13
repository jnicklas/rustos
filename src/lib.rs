#![feature(no_std, lang_items, const_fn, unique, core_str_ext)]
#![no_std]

mod vga_buffer;

use vga_buffer::{Writer, Color};

use core::fmt::Write;

extern crate rlibc;

#[no_mangle]
pub extern fn rust_main() {
    let mut writer = Writer::new();
    writer.write_byte(b'H');
    writer.set_color(Color::LightBlue, Color::Black);
    writer.write_str("ello! ");
    write!(writer, "The numbers are {} and {}", 42, 1.0/3.0).unwrap();

    loop{}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! {loop{}}
