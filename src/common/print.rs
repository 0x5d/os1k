use core::ffi::{c_char, c_long};
use crate::sbi_call;

// Calls sbi_console_putchar.
// https://github.com/riscv-non-isa/riscv-sbi-doc/blob/7120a83/src/ext-legacy.adoc#extension-console-putchar-eid-0x01
pub unsafe fn putchar(ch: c_char) {
    sbi_call(ch as c_long, 0, 0, 0, 0, 0, 0, 1 /* 0x01: Console Putchar */);
}

pub unsafe extern "C" fn printf(mut s: *const c_char, mut args: ...) {
    let mut args = args.as_va_list();

    while *s != 0 {
        // If it's not a fmt directive (%), just print it.
        if *s != b'%' {
            putchar(*s);
            s = s.add(1);
            continue;
        }
        // Skip the %.
        s = s.add(1);
        // Format the arg accordingly.
        match *s {
            b's' => {
                // TODO: This will explode if the arg isn't a string (*const c_char).
                let mut sa: *const c_char = args.arg();
                while *sa != 0 {
                    putchar(*sa);
                    sa = sa.add(1);
                }
            },
            _ => {
                putchar(*s);
            }
        }
        s = s.add(1)
    }
}
