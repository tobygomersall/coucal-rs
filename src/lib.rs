//! Low level access to PicoRV32 RISC-V processor
//!
//! # Minimum Supported Rust Version (MSRV)
//!
//! This crate is guaranteed to compile on stable Rust 1.32 and up. It *might*
//! compile with older versions but that may change in any new patch release.
//!
//! # Features
//!
//! This crate provides:
//!
//! - PicoRV32's interrupt manipulation mechanisms.
//! - Wrappers around assembly instructions such as `waitirq`.

#![no_std]
#![deny(warnings)]

extern crate bare_metal;

pub mod asm;
pub mod interrupt;
