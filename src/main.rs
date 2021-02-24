// SPDX-License-Identifier: MIT
//
// Copyright (c) 2021 Garen Tyler <garentyler@gmail.com>

// Rust embedded logo for `make doc`
#![doc(html_logo_url = "https://git.io/JeGIp")]

#![feature(asm)]
#![feature(global_asm)]
#![no_main]
#![no_std]
#![allow(non_snake_case)]

mod bsp;
mod cpu;
mod panic_wait;
