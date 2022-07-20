#![no_main]
#![no_std]

#[macro_use]
mod console;
mod lang_items;
mod sbi;

use crate::sbi::shutdown;

core::arch::global_asm!(include_str!("entry.asm"));

fn clear_bss() {
        extern "C" {
            fn sbss();
            fn ebss();
        }
        (sbss as usize..ebss as usize).for_each(|a| {
            unsafe { (a as *mut u8).write_volatile(0) }
        });
   }

#[no_mangle]
pub fn rust_main() -> ! {
    println!("Hello, world!");
    clear_bss();
    shutdown();
}