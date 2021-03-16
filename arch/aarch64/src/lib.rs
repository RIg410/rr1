//! aarch64

#![feature(asm)]
#![feature(const_fn)]
#![feature(const_fn_fn_ptr_basics)]
#![feature(const_panic)]
#![feature(core_intrinsics)]
#![feature(format_args_nl)]
#![feature(global_asm)]
#![feature(linkage)]
#![feature(panic_info_message)]
#![feature(trait_alias)]
#![no_std]

#[cfg(feature = "test_build")]
extern crate qemu_exit;

/// aarch64 cpu abstraction.
pub mod cpu;

/// QEMU operations. Used for test.
#[cfg(feature = "test_build")]
pub mod qemu;