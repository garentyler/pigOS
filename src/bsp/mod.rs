// SPDX-License-Identifier: MIT
//
// Copyright (c) 2021 Garen Tyler <garentyler@gmail.com>

#[cfg(any(feature = "bsp_rpi3", feature = "bsp_rpi4"))]
mod raspberrypi;

#[cfg(any(feature = "bsp_rpi3", feature = "bsp_rpi4"))]
pub use raspberrypi::*;
