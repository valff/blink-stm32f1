#![feature(const_fn)]
#![no_main]
#![no_std]
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate blink_stm32f1;

use blink_stm32f1::VectorTable;

#[no_mangle]
pub static VECTOR_TABLE: VectorTable = VectorTable::new();
