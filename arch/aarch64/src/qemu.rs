///! QEMU actions.

use qemu_exit::{
    QEMUExit, AArch64,
};

/// Make the host QEMU binary execute `exit(1)`.
pub fn qemu_exit_failure() -> ! {
    AArch64::new().exit_failure()
}

/// Make the host QEMU binary execute `exit(0)`.
pub fn qemu_exit_success() -> ! {
    AArch64::new().exit_success()
}
