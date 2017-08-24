//! Firmware part of project Enlightenment.
//!
//! Target MCU is STM32F103C8.
#![feature(const_fn)]
#![feature(naked_functions)]
#![feature(compiler_builtins_lib)]
#![no_std]
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![cfg_attr(feature = "clippy", allow(precedence, doc_markdown))]

extern crate compiler_builtins;
extern crate drone;
#[cfg(test)]
#[macro_use]
extern crate test;

pub use vector_table::VectorTable;

pub mod consts;
pub mod exception;
pub mod vector_table;
