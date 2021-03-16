//! common arc.

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

#[cfg(target_arch = "aarch64")]
extern crate aarch64;

#[cfg(target_arch = "aarch64")]
pub use aarch64::*;
