//! Init kernel.

#![feature(asm)]
#![feature(format_args_nl)]
#![feature(global_asm)]
#![feature(panic_info_message)]
#![no_main]
#![no_std]

#[cfg(any(feature = "bsp_rpi3", feature = "bsp_rpi4"))]
extern crate arch;
#[cfg(not(any(feature = "bsp_rpi3", feature = "bsp_rpi4")))]
compile_error!("Unsupported arch");

mod memory;
mod panic_wait;
mod print;
mod runtime_init;

/// Early init code.
///
/// # Safety
///
/// - Only a single core must be active and running this function.
unsafe fn kernel_init() -> ! {
    println!("Hello!");

    panic!("Stopping here.")
}
