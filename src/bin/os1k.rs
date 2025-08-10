#![no_std]
#![no_main]

use core::arch::{naked_asm};
use core::ffi::{c_void, c_char};
use core::panic::PanicInfo;

unsafe extern "C" {
    static __bss: usize;
    static __bss_end: usize;
    static __stack_top: usize;
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Since we're not using `main` as our entrypoint, this function is marked as unreference and
// removed (as an optimization) unless we add no_mangle.
#[no_mangle]
// Don't generate extra code before or after the fn.
#[unsafe(naked)]
// Put the boot fn at the start address, which is where .text.boot is. See kernel.ld.
#[link_section = ".text.boot"]
pub unsafe extern "C" fn boot() {
    // Note: The example code at https://operating-system-in-1000-lines.vercel.app/en/04-boot#minimal-kernel
    // uses "r" (see https://gcc.gnu.org/onlinedocs/gcc/Extended-Asm.html):
    // [stack_top] "r" (__stack_top)
    // which loads the variable into a register, so it can then do
    // mv sp, %[stack_top]
    // However, Rust's naked_asm doesn't generate any additional assembly (hence the `naked`),
    // so we must load it into sp (`la`), as opposed to moving it (`mv`).
    // See https://github.com/riscv-non-isa/riscv-asm-manual/blob/b99bfb68e0f3d05c1f58ba3ae76b48a68d533e8d/src/asm-manual.adoc?plain=1#L684-L701.
    naked_asm!(
        "la sp, {}",        // Load address of stack_top into sp
        "j kernel_main",    // Jump to the kernel_main function
        sym __stack_top,
    );
}

// Needed so rustc doesn't optimize away this function, since it's only referenced in boot via
// naked_asm.
#[no_mangle]
pub unsafe fn kernel_main() {
    // Zero-out the bss (block storage segment), which starts at __bss and ends at __bss_end.
    memset(__bss as *mut c_void, 0,  __bss_end - __bss);

    loop {}
}

pub unsafe fn memset(buf: *mut c_void, c: c_char, n: usize) -> *mut c_void {
    let mut p = buf as *mut u8;
    let mut count = n;
    while count > 0 {
        *p = c as u8;
        p = p.add(1);
        count -= 1;
    }
    buf
}
