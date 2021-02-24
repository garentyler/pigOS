// SPDX-License-Identifier: MIT
//
// Copyright (c) 2021 Garen Tyler <garentyler@gmail.com>

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unimplemented!()
}
