use std::io;
use core::arch::asm;

#[inline(always)]
unsafe fn syscall3(n: usize, a1: usize, a2: usize, a3: usize) -> isize {
    let ret: isize;
    unsafe {
        asm!(
            "syscall",
            inlateout("rax") n as isize => ret,
            in("rdi") a1,
            in("rsi") a2,
            in("rdx") a3,
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack, preserves_flags)
        );
    }
    ret
}

fn read() -> String {
    let mut buffer = String::new();
    unsafe {
        syscall3(0, 0, buffer.as_mut_ptr() as usize, 2048);
    }
    buffer

}

fn bit(buffer: String) -> usize {
    let mut bite: usize = 0;
    bite = buffer.trim().len();
    bite
}

fn main() {
    let guess = read().trim().to_string();
    let bite = bit(guess.clone());
    println!("Guess: {}, Bit: {}", guess, bite);
}
