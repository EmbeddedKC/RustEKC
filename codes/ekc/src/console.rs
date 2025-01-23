use core::fmt::{self, Write};

//#[cfg(feature = "board_qemu")]
#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::arch_print(format_args!($fmt $(, $($arg)+)?));
        //$crate::fs::_print(format_args!($fmt $(, $($arg)+)?));
    }
}


#[macro_export]
macro_rules! debug_info {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::arch_print(format_args!("\x1b[{}m[info] \x1b[{}m", 32, 37));
        $crate::arch_print(format_args!(concat!($fmt, "\x1b[0m\n") $(, $($arg)+)?));
        //$crate::fs::_print(format_args!(core::concat!($fmt, "\n") $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! _debug_info_level {
    () => {
        10
    }
}

#[macro_export]
macro_rules! debug_info_level {
    ($level: literal, $fmt: literal $(, $($arg: tt)+)?) => {
        if $level >= crate::_debug_info_level!() {
            $crate::arch_print(format_args!("\x1b[{}m[info] \x1b[{}m", 32, 37));
            $crate::arch_print(format_args!(concat!($fmt, "\x1b[0m\n") $(, $($arg)+)?));
            //$crate::fs::_print(format_args!(core::concat!($fmt, "\n") $(, $($arg)+)?));    
        }
     }
}

#[macro_export]
macro_rules! debug_warn {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::arch_print(format_args!("\x1b[{}m[warn] \x1b[{}m", 33, 37));
        $crate::arch_print(format_args!(concat!($fmt, "\x1b[0m\n") $(, $($arg)+)?));
        //$crate::fs::_print(format_args!(core::concat!($fmt, "\n") $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! debug_error {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::arch_print(format_args!("\x1b[{}m[error] \x1b[{}m", 31, 37));
        $crate::arch_print(format_args!(concat!($fmt, "\x1b[0m\n") $(, $($arg)+)?));
        //$crate::fs::_print(format_args!(core::concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
