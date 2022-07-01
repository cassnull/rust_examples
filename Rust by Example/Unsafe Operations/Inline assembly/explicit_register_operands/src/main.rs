#![allow(unused)]
use std::arch::asm;

fn main() {
    let cmd = 0xd1;
    unsafe {
        asm!("out 0x64, eax", in("eax") cmd);
    }
}

fn mul(a: u64, b: u64) -> u128 {
    let lo: u64;
    let hi: u64;

    unsafe {
        asm!(
            // The x86 mul instruction takes rax as an implicit input and writes
            // the 128-bit result of the multiplication to rax:rdx.
            "mul {}",
            in(reg) a,
            inlateout("rax") b => lo,
            lateout("rdx") hi
        );
    }

    ((hi as u128) << 64) + lo as u128
}
