use std::arch::asm;

fn main() {
    let mut a: i32;
    unsafe {
        asm!(
            "mov {0:e}, 10",
            "2:",
            "sub {0:e}, 1",
            "cmp {0:e}, 3",
            "jle 2f",
            "jmp 2b",
            "2:",
            "add {0:e}, 2",
            out(reg) a
        );
    }
    assert_eq!(a, 5);
}
