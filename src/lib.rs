#![no_std]
#![feature(ascii_char)]
#![feature(c_variadic)]

use core::arch::asm;
use core::ffi::{c_long};

pub mod common;

#[repr(C)]
pub struct SBIRet {
    pub error: c_long,
    pub value: c_long,
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
