#![no_std]
#![no_main]
#![feature(ascii_char)]

use core::arch::{asm, naked_asm};
use core::ffi::{c_char, c_long, c_void};
use core::panic::PanicInfo;

unsafe extern "C" {
    static __bss: usize;
    static __bss_end: usize;
    static __stack_top: usize;
}

#[repr(C)]
pub struct SBIRet {
    pub error: c_long,
    pub value: c_long,
}

// Calls sbi_console_putchar.
// https://github.com/riscv-non-isa/riscv-sbi-doc/blob/7120a83/src/ext-legacy.adoc#extension-console-putchar-eid-0x01
pub unsafe fn putchar(ch: c_char) {
    sbi_call(ch as c_long, 0, 0, 0, 0, 0, 0, 1 /* 0x01: Console Putchar */);
}

// sbi_call calls SBI functions through `ecall`.
pub unsafe extern "C" fn sbi_call(mut arg0: c_long, mut arg1: c_long, arg2: c_long, arg3: c_long, arg4: c_long, arg5: c_long, fid: c_long, eid: c_long) -> SBIRet {
    // sbi functions are called through the ecall instruction. Registers a0 through a5 hold the
    // arguments for the function. a7 must hold the SBI Function ID (FID), and a6 the Extension ID
    // (EID).
    // After the call, a0 & a1 will hold an error code (0 for success) and a return value, respectively.
    // https://github.com/riscv-non-isa/riscv-sbi-doc/blob/2b09fad/src/binary-encoding.adoc.
    asm!(
        "ecall",
        inout("a0") arg0,
        inout("a1") arg1,
        in("a2") arg2,
        in("a3") arg3,
        in("a4") arg4,
        in("a5") arg5,
        in("a6") fid,
        in("a7") eid,
    );
    return SBIRet { error: arg0, value: arg1 }
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
    
    // Print Hello World! after boot.
    let s: &str = "\n\nHello World!\n";
    for c in s.as_ascii_unchecked().iter() {
        putchar(*c as c_char);
    }

    loop {
        asm!("wfi")
    }
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
