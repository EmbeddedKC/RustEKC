#![no_std]

#[macro_use]
extern crate bitflags;

pub mod address;
pub mod flags;
#[macro_use]
//pub mod nkapi;
//pub mod config;
pub mod context;

pub use address::*;
//pub use config::*;
pub use flags::*;
//pub use nkapi::*;
pub use context::*;

//global_asm!(include_str!("nk_gate.S"));
