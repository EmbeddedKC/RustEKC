use core::fmt::{self, Write};
use super::console_putchar;
use spin::Mutex;

use crate::alloc::string::String;

struct Stdout;

static PRINT_LOCK: Mutex<()> = Mutex::new(());


impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}

//memcpy aligned .... 

pub fn print(args: fmt::Arguments) {
    let _locked = PRINT_LOCK.lock();
    Stdout.write_fmt(args).unwrap();

    // let mut output: String = String::new();
    // fmt::write(&mut output, args).expect("Error formatting the string");
    
    // for c in output.chars() {
    //     console_putchar(c as usize);
    // }
}


pub fn print_raw(s: &str) {
    for c in s.chars() {
        console_putchar(c as usize);
    }
}


#[macro_export]
macro_rules! arch_print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::arch_print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! arch_debug_info {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::arch_print(format_args!("\x1b[{}m[arch_info] \x1b[{}m", 32, 37));
        $crate::arch_print(format_args!(concat!($fmt, "\x1b[0m\n") $(, $($arg)+)?));
     }
}

#[macro_export]
macro_rules! arch_debug_warn {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::arch_print(format_args!("\x1b[{}m[arch_warn] \x1b[{}m", 33, 37));
        $crate::arch_print(format_args!(concat!($fmt, "\x1b[0m\n") $(, $($arg)+)?));
        //$crate::fs::_print(format_args!(core::concat!($fmt, "\n") $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! arch_debug_error {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::arch_print(format_args!("\x1b[{}m[arch_error] \x1b[{}m", 31, 37));
        $crate::arch_print(format_args!(concat!($fmt, "\x1b[0m\n") $(, $($arg)+)?));
        //$crate::fs::_print(format_args!(core::concat!($fmt, "\n") $(, $($arg)+)?));
    }
}


