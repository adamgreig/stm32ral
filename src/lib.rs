// Copyright 2018 Adam Greig
// See LICENSE-APACHE and LICENSE-MIT for license details.

//! This project provides a register access layer (RAL) for all
//! STM32 microcontrollers.
//!
//! When built, you must specify a device feature, such as `stm32f405`.
//! This will cause all modules in that device's module to be re-exported
//! from the top level, so that for example `stm32ral::gpio` will resolve to
//! `stm32ral::stm32f4::stm32f405::gpio`.
//!
//! In the generated documentation, all devices are visible inside their family
//! modules, but when built for a specific device, only that devices' constants
//! will be available.
//!
//! See the
//! [README](https://github.com/adamgreig/stm32ral/blob/master/README.md)
//! for example usage.

#![no_std]

#[cfg(feature="rt")]
extern crate cortex_m_rt;

mod register;

#[cfg(feature="rt")]
pub use cortex_m_rt::interrupt;

pub use crate::register::{RORegister, UnsafeRORegister};
pub use crate::register::{WORegister, UnsafeWORegister};
pub use crate::register::{RWRegister, UnsafeRWRegister};
#[cfg(any(feature="doc", feature="armv6m", feature="armv7em", feature="armv7m"))]
pub mod cortex_m;

#[cfg(feature="armv6m")]
pub use cortex_m::armv6m::*;

#[cfg(feature="armv7em")]
pub use cortex_m::armv7em::*;

#[cfg(feature="armv7m")]
pub use cortex_m::armv7m::*;

#[cfg(any(feature="doc", feature="stm32f0x0", feature="stm32f0x1", feature="stm32f0x2", feature="stm32f0x8"))]
pub mod stm32f0;

#[cfg(feature="stm32f0x0")]
pub use stm32f0::stm32f0x0::*;

#[cfg(feature="stm32f0x1")]
pub use stm32f0::stm32f0x1::*;

#[cfg(feature="stm32f0x2")]
pub use stm32f0::stm32f0x2::*;

#[cfg(feature="stm32f0x8")]
pub use stm32f0::stm32f0x8::*;

#[cfg(any(feature="doc", feature="stm32f100", feature="stm32f101", feature="stm32f102", feature="stm32f103", feature="stm32f107"))]
pub mod stm32f1;

#[cfg(feature="stm32f100")]
pub use stm32f1::stm32f100::*;

#[cfg(feature="stm32f101")]
pub use stm32f1::stm32f101::*;

#[cfg(feature="stm32f102")]
pub use stm32f1::stm32f102::*;

#[cfg(feature="stm32f103")]
pub use stm32f1::stm32f103::*;

#[cfg(feature="stm32f107")]
pub use stm32f1::stm32f107::*;

#[cfg(any(feature="doc", feature="stm32f215", feature="stm32f217"))]
pub mod stm32f2;

#[cfg(feature="stm32f215")]
pub use stm32f2::stm32f215::*;

#[cfg(feature="stm32f217")]
pub use stm32f2::stm32f217::*;

#[cfg(any(feature="doc", feature="stm32f301", feature="stm32f302", feature="stm32f303", feature="stm32f373", feature="stm32f3x4", feature="stm32f3x8"))]
pub mod stm32f3;

#[cfg(feature="stm32f301")]
pub use stm32f3::stm32f301::*;

#[cfg(feature="stm32f302")]
pub use stm32f3::stm32f302::*;

#[cfg(feature="stm32f303")]
pub use stm32f3::stm32f303::*;

#[cfg(feature="stm32f373")]
pub use stm32f3::stm32f373::*;

#[cfg(feature="stm32f3x4")]
pub use stm32f3::stm32f3x4::*;

#[cfg(feature="stm32f3x8")]
pub use stm32f3::stm32f3x8::*;

#[cfg(any(feature="doc", feature="stm32f401", feature="stm32f405", feature="stm32f407", feature="stm32f410", feature="stm32f411", feature="stm32f412", feature="stm32f413", feature="stm32f427", feature="stm32f429", feature="stm32f446", feature="stm32f469"))]
pub mod stm32f4;

#[cfg(feature="stm32f401")]
pub use stm32f4::stm32f401::*;

#[cfg(feature="stm32f405")]
pub use stm32f4::stm32f405::*;

#[cfg(feature="stm32f407")]
pub use stm32f4::stm32f407::*;

#[cfg(feature="stm32f410")]
pub use stm32f4::stm32f410::*;

#[cfg(feature="stm32f411")]
pub use stm32f4::stm32f411::*;

#[cfg(feature="stm32f412")]
pub use stm32f4::stm32f412::*;

#[cfg(feature="stm32f413")]
pub use stm32f4::stm32f413::*;

#[cfg(feature="stm32f427")]
pub use stm32f4::stm32f427::*;

#[cfg(feature="stm32f429")]
pub use stm32f4::stm32f429::*;

#[cfg(feature="stm32f446")]
pub use stm32f4::stm32f446::*;

#[cfg(feature="stm32f469")]
pub use stm32f4::stm32f469::*;

#[cfg(any(feature="doc", feature="stm32f730", feature="stm32f745", feature="stm32f765", feature="stm32f7x2", feature="stm32f7x3", feature="stm32f7x6", feature="stm32f7x7", feature="stm32f7x9"))]
pub mod stm32f7;

#[cfg(feature="stm32f730")]
pub use stm32f7::stm32f730::*;

#[cfg(feature="stm32f745")]
pub use stm32f7::stm32f745::*;

#[cfg(feature="stm32f765")]
pub use stm32f7::stm32f765::*;

#[cfg(feature="stm32f7x2")]
pub use stm32f7::stm32f7x2::*;

#[cfg(feature="stm32f7x3")]
pub use stm32f7::stm32f7x3::*;

#[cfg(feature="stm32f7x6")]
pub use stm32f7::stm32f7x6::*;

#[cfg(feature="stm32f7x7")]
pub use stm32f7::stm32f7x7::*;

#[cfg(feature="stm32f7x9")]
pub use stm32f7::stm32f7x9::*;

#[cfg(any(feature="doc", feature="stm32g030", feature="stm32g031", feature="stm32g041", feature="stm32g07x", feature="stm32g081"))]
pub mod stm32g0;

#[cfg(feature="stm32g030")]
pub use stm32g0::stm32g030::*;

#[cfg(feature="stm32g031")]
pub use stm32g0::stm32g031::*;

#[cfg(feature="stm32g041")]
pub use stm32g0::stm32g041::*;

#[cfg(feature="stm32g07x")]
pub use stm32g0::stm32g07x::*;

#[cfg(feature="stm32g081")]
pub use stm32g0::stm32g081::*;

#[cfg(any(feature="doc", feature="stm32g431", feature="stm32g441", feature="stm32g471", feature="stm32g473", feature="stm32g474", feature="stm32g483", feature="stm32g484"))]
pub mod stm32g4;

#[cfg(feature="stm32g431")]
pub use stm32g4::stm32g431::*;

#[cfg(feature="stm32g441")]
pub use stm32g4::stm32g441::*;

#[cfg(feature="stm32g471")]
pub use stm32g4::stm32g471::*;

#[cfg(feature="stm32g473")]
pub use stm32g4::stm32g473::*;

#[cfg(feature="stm32g474")]
pub use stm32g4::stm32g474::*;

#[cfg(feature="stm32g483")]
pub use stm32g4::stm32g483::*;

#[cfg(feature="stm32g484")]
pub use stm32g4::stm32g484::*;

#[cfg(any(feature="doc", feature="stm32h743", feature="stm32h743v", feature="stm32h747cm4", feature="stm32h747cm7", feature="stm32h753", feature="stm32h753v"))]
pub mod stm32h7;

#[cfg(feature="stm32h743")]
pub use stm32h7::stm32h743::*;

#[cfg(feature="stm32h743v")]
pub use stm32h7::stm32h743v::*;

#[cfg(feature="stm32h747cm4")]
pub use stm32h7::stm32h747cm4::*;

#[cfg(feature="stm32h747cm7")]
pub use stm32h7::stm32h747cm7::*;

#[cfg(feature="stm32h753")]
pub use stm32h7::stm32h753::*;

#[cfg(feature="stm32h753v")]
pub use stm32h7::stm32h753v::*;

#[cfg(any(feature="doc", feature="stm32l0x1", feature="stm32l0x2", feature="stm32l0x3"))]
pub mod stm32l0;

#[cfg(feature="stm32l0x1")]
pub use stm32l0::stm32l0x1::*;

#[cfg(feature="stm32l0x2")]
pub use stm32l0::stm32l0x2::*;

#[cfg(feature="stm32l0x3")]
pub use stm32l0::stm32l0x3::*;

#[cfg(any(feature="doc", feature="stm32l100", feature="stm32l151", feature="stm32l162"))]
pub mod stm32l1;

#[cfg(feature="stm32l100")]
pub use stm32l1::stm32l100::*;

#[cfg(feature="stm32l151")]
pub use stm32l1::stm32l151::*;

#[cfg(feature="stm32l162")]
pub use stm32l1::stm32l162::*;

#[cfg(any(feature="doc", feature="stm32l4x1", feature="stm32l4x2", feature="stm32l4x3", feature="stm32l4x5", feature="stm32l4x6"))]
pub mod stm32l4;

#[cfg(feature="stm32l4x1")]
pub use stm32l4::stm32l4x1::*;

#[cfg(feature="stm32l4x2")]
pub use stm32l4::stm32l4x2::*;

#[cfg(feature="stm32l4x3")]
pub use stm32l4::stm32l4x3::*;

#[cfg(feature="stm32l4x5")]
pub use stm32l4::stm32l4x5::*;

#[cfg(feature="stm32l4x6")]
pub use stm32l4::stm32l4x6::*;

