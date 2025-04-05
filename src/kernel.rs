#![no_std]
#![no_main]

use core::{arch::asm, panic::PanicInfo};

unsafe extern "C" {
    unsafe static __bss: u8;
    unsafe static __bss_end: u8;
    unsafe static __stack_top: u32;
    unsafe static mut __free_ram: u8;
    unsafe static __free_ram_end: u8;
    unsafe static __kernel_base: u8;
}

pub fn memset(dest: *mut u8, val: u8, count: usize) {
    for i in 0..count {
        unsafe {
            *dest.add(i) = val;
        }
    }
}

#[repr(C)]
pub struct SbiRet {
    pub error: isize,
    pub value: isize,
}

pub fn sbi_call(
    arg0: isize,
    arg1: isize,
    arg2: isize,
    arg3: isize,
    arg4: isize,
    arg5: isize,
    fid: isize,
    eid: isize,
) -> SbiRet {
    let mut error: isize;
    let mut value: isize;

    unsafe {
        asm!(
            "ecall",
            inout("a0") arg0 => error,
            inout("a1") arg1 => value,
            in("a2") arg2,
            in("a3") arg3,
            in("a4") arg4,
            in("a5") arg5,
            in("a6") fid,
            in("a7") eid,
            options(nostack)
        );
    }

    SbiRet { error, value }
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Add this function to print a single character to console
pub fn putchar(ch: char) {
    sbi_call(ch as isize, 0, 0, 0, 0, 0, 0, 1 /* Console Putchar */);
}

#[unsafe(no_mangle)]
fn kernel_main() {
    unsafe {
        memset(
            &__bss as *const u8 as *mut u8,
            0,
            (__bss_end - __bss) as usize,
        );
        // Print "Hello World!"
        let s = "\n\nHello World!\n";
        for ch in s.chars() {
            putchar(ch);
        }

        // Enter an infinite loop with wfi instruction
        loop {
            asm!("wfi");
        }
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.boot")]
pub unsafe extern "C" fn boot() -> ! {
    unsafe {
        asm!(
            "mv sp, {stack_top}
            j {kernel_main}",
            stack_top = in(reg) &__stack_top,
            kernel_main = sym kernel_main,
        )
    };
    loop {}
}
