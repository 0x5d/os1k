use core::ffi::{c_char, c_long};
use crate::sbi_call;

const HEX_CHARSET: [c_char; 16] = *b"0123456789abcdef";

// Calls sbi_console_putchar.
// https://github.com/riscv-non-isa/riscv-sbi-doc/blob/7120a83/src/ext-legacy.adoc#extension-console-putchar-eid-0x01
pub unsafe fn putchar(ch: c_char) {
    sbi_call(ch as c_long, 0, 0, 0, 0, 0, 0, 1 /* 0x01: Console Putchar */);
}

pub unsafe extern "C" fn printf(mut fmt: *const c_char, mut args: ...) {
    let mut args = args.as_va_list();

    while *fmt != 0 {
        // If it's not a fmt directive (%), just print it.
        if *fmt != b'%' {
            putchar(*fmt);
            fmt = fmt.add(1);
            continue;
        }
        // Skip the %.
        fmt = fmt.add(1);
        // Format the arg accordingly.
        match *fmt {
            // If it's a null char, we've reached the end.
            b'\0' => break,
            // If it's a '%', print it.
            b'%' => putchar(b'%'),
            // Get a string and print it.
            // TODO: This will explode if the arg isn't a string (*const c_char).
            b's' => print_str(args.arg::<*const c_char>()),
            // Get a signed 32-bit int and print it.
            b'd' => print_i32(args.arg::<i32>()),
            // Print an unsigned hex.
            b'x' => print_hex(args.arg::<u32>()),
            _ => putchar(*fmt),
        }
        fmt = fmt.add(1)
    }
}

unsafe fn print_str(mut s: *const c_char) {
    while *s != 0 {
        putchar(*s);
        s = s.add(1);
    }
}

unsafe fn print_i32(mut val: i32) {
    if val == 0 {
        putchar(b'0');
        return;
    }
    if val < 0 {
        putchar(b'-');
        val = -val;
    }
    let mut divisor = 1;
    while val / divisor > 9 {
        divisor *= 10;
    }
    while divisor > 0 {
        let d = b'0' + (val / divisor) as c_char;
        putchar(d);
        val = val % divisor;
        divisor /= 10;
    }
}

unsafe fn print_hex(val: u32) {
    let val = val as usize;
    if val == 0 {
        putchar(b'0');
        return;
    }
    let mut started = false;
    for i in (0..7).rev() {
        let nibble = (val >> (i * 4)) & 0xf;
        if nibble != 0 || started {
            putchar(HEX_CHARSET[nibble]);
            started = true;
        }
    }
}
