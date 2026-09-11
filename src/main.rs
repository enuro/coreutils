use core::arch::asm;
use std::io;

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

fn read(mut buffer: String) -> String {
    unsafe {
        syscall3(0, 0, buffer.as_mut_ptr() as usize, 2048);
    }
    buffer
}

fn write(buffer: &str) {
    unsafe {
        syscall3(1, 1, buffer.as_ptr() as usize, buffer.len());
    }
}

fn openat(dirfd: usize, pathname: &str, flags: usize, mode: usize) -> isize {
    unsafe { syscall3(257, dirfd, pathname.as_ptr() as usize, flags | (mode << 12)) }
}

fn close(fd: usize) -> isize {
    unsafe { syscall3(3, fd, 0, 0) }
}

fn exit_group(status: usize) -> ! {
    unsafe {
        syscall3(60, status, 0, 0);
    }
    loop {}
}

fn bit(buffer: String) -> usize {
    let bite = buffer.len();
    bite
}

fn main() {
    let mut path: String = String::new();
    io::stdin().read_line(&mut path);
    let guess = read(path);
    let bite = bit(guess.clone());
    println!("Guess: {}, Bit: {}", guess, bite);
}
