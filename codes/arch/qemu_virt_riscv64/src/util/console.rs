use core::fmt::{self, Write};
use super::sbi::console_putchar;

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}

pub fn print_raw_chars(chars: &[u8]) {
    for c in chars {
        console_putchar(c.clone() as usize);
    }
}
pub fn print(args: fmt::Arguments) {
    //here accessible
    Stdout.write_fmt(args).unwrap();
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


