// SPDX-License-Identifier: GPL-2.0

//! Rust printing macros sample.

use kernel::pr_cont;
use kernel::prelude::*;

module! {
    type: RustPrint,
    name: "rust_print",
    author: "Rust for Linux Contributors",
    description: "Rust printing macros sample",
    license: "GPL",
}

struct RustPrint;

impl kernel::Module for RustPrint {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("Hello World from Rust!\n");

        Ok(RustPrint)
    }
}

impl Drop for RustPrint {
    fn drop(&mut self) {
        pr_info!("Rust printing macros sample (exit)\n");
    }
}
