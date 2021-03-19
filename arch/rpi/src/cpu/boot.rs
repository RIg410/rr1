use crate::cpu::core_id;

/// Os entry point.
#[no_mangle]
pub unsafe fn _start() -> ! {
    extern "Rust" {
        pub fn runtime_init() -> !;
    }

    if BOOT_CORE_ID == core_id() {
        //SP.set(bsp::memory::boot_core_stack_end() as u64);
        runtime_init()
    } else {
        // If not core0, infinitely wait for events.
        panic!("core0 not found")
    }
}

/// Boot core id.
pub const BOOT_CORE_ID: usize = 0;
