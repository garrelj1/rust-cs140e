#![no_std]
#![no_builtins]
#![feature(compiler_builtins, llvm_asm, lang_items, pointer_methods)]
// Updated asm to llvm_asm (though this needs to be updated since it's deprecated)
// Update compiler_builtins_lib to compiler_builtins
// removed compiler_builtins extern crate

pub mod lang_items;

const GPIO_BASE: usize = 0x3F000000 + 0x200000;

const GPIO_FSEL1: *mut u32 = (GPIO_BASE + 0x04) as *mut u32;
const GPIO_SET0: *mut u32 = (GPIO_BASE + 0x1C) as *mut u32;
const GPIO_CLR0: *mut u32 = (GPIO_BASE + 0x28) as *mut u32;

#[inline(never)]
fn spin_sleep_ms(ms: usize) {
    for _ in 0..(ms * 6000) {
        unsafe { llvm_asm!("nop" :::: "volatile"); }
    }
}

#[no_mangle]
pub unsafe extern "C" fn kmain() {

    set_fsel_output();

    let mut on = true;
    loop {
        if on {
            set_led_on();
        }
        else {
            set_led_off();
        }
        on = !on;
        spin_sleep_ms(100);
    }
}

fn set_fsel_output() {
    unsafe {
        let write_bits = GPIO_FSEL1.read_volatile() | 0x40000;
        GPIO_FSEL1.write_volatile(write_bits);
    }
}

fn set_led_on() {
    unsafe {
        let write_bits = GPIO_SET0.read_volatile() | 0x10000;
        GPIO_SET0.write_volatile(write_bits);
    }
}

fn set_led_off() {
    unsafe {
        let write_bits = GPIO_CLR0.read_volatile() | 0x10000;
        GPIO_CLR0.write_volatile(write_bits);
    }
}
