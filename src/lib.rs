// Copyright 2018 Adam Greig
// See LICENSE-APACHE and LICENSE-MIT for license details.

//! This project provides a register access layer (RAL) for all
//! STM32 microcontrollers.

#![no_std]

#[macro_use]
mod register;

pub use register::{RORegister, WORegister, RWRegister};
pub use register::{UnsafeRORegister, UnsafeRWRegister, UnsafeWORegister};

pub mod peripherals;

#[cfg(feature="doc")]
pub mod stm32f0;

#[cfg(feature="stm32f0x0")]
pub mod stm32f0;
#[cfg(feature="stm32f0x0")]
pub use stm32f0::stm32f0x0::*;

#[cfg(feature="stm32f0x1")]
pub mod stm32f0;
#[cfg(feature="stm32f0x1")]
pub use stm32f0::stm32f0x1::*;

#[cfg(feature="stm32f0x2")]
pub mod stm32f0;
#[cfg(feature="stm32f0x2")]
pub use stm32f0::stm32f0x2::*;

#[cfg(feature="stm32f0x8")]
pub mod stm32f0;
#[cfg(feature="stm32f0x8")]
pub use stm32f0::stm32f0x8::*;

#[cfg(feature="doc")]
pub mod stm32f1;

#[cfg(feature="stm32f100")]
pub mod stm32f1;
#[cfg(feature="stm32f100")]
pub use stm32f1::stm32f100::*;

#[cfg(feature="stm32f101")]
pub mod stm32f1;
#[cfg(feature="stm32f101")]
pub use stm32f1::stm32f101::*;

#[cfg(feature="stm32f102")]
pub mod stm32f1;
#[cfg(feature="stm32f102")]
pub use stm32f1::stm32f102::*;

#[cfg(feature="stm32f103")]
pub mod stm32f1;
#[cfg(feature="stm32f103")]
pub use stm32f1::stm32f103::*;

#[cfg(feature="stm32f107")]
pub mod stm32f1;
#[cfg(feature="stm32f107")]
pub use stm32f1::stm32f107::*;

#[cfg(feature="doc")]
pub mod stm32f2;

#[cfg(feature="stm32f215")]
pub mod stm32f2;
#[cfg(feature="stm32f215")]
pub use stm32f2::stm32f215::*;

#[cfg(feature="stm32f217")]
pub mod stm32f2;
#[cfg(feature="stm32f217")]
pub use stm32f2::stm32f217::*;

#[cfg(feature="doc")]
pub mod stm32f3;

#[cfg(feature="stm32f301")]
pub mod stm32f3;
#[cfg(feature="stm32f301")]
pub use stm32f3::stm32f301::*;

#[cfg(feature="stm32f302")]
pub mod stm32f3;
#[cfg(feature="stm32f302")]
pub use stm32f3::stm32f302::*;

#[cfg(feature="stm32f303")]
pub mod stm32f3;
#[cfg(feature="stm32f303")]
pub use stm32f3::stm32f303::*;

#[cfg(feature="stm32f373")]
pub mod stm32f3;
#[cfg(feature="stm32f373")]
pub use stm32f3::stm32f373::*;

#[cfg(feature="stm32f3x4")]
pub mod stm32f3;
#[cfg(feature="stm32f3x4")]
pub use stm32f3::stm32f3x4::*;

#[cfg(feature="stm32f3x8")]
pub mod stm32f3;
#[cfg(feature="stm32f3x8")]
pub use stm32f3::stm32f3x8::*;

#[cfg(feature="doc")]
pub mod stm32f4;

#[cfg(feature="stm32f401")]
pub mod stm32f4;
#[cfg(feature="stm32f401")]
pub use stm32f4::stm32f401::*;

#[cfg(feature="stm32f405")]
pub mod stm32f4;
#[cfg(feature="stm32f405")]
pub use stm32f4::stm32f405::*;

#[cfg(feature="stm32f407")]
pub mod stm32f4;
#[cfg(feature="stm32f407")]
pub use stm32f4::stm32f407::*;

#[cfg(feature="stm32f410")]
pub mod stm32f4;
#[cfg(feature="stm32f410")]
pub use stm32f4::stm32f410::*;

#[cfg(feature="stm32f411")]
pub mod stm32f4;
#[cfg(feature="stm32f411")]
pub use stm32f4::stm32f411::*;

#[cfg(feature="stm32f412")]
pub mod stm32f4;
#[cfg(feature="stm32f412")]
pub use stm32f4::stm32f412::*;

#[cfg(feature="stm32f413")]
pub mod stm32f4;
#[cfg(feature="stm32f413")]
pub use stm32f4::stm32f413::*;

#[cfg(feature="stm32f427")]
pub mod stm32f4;
#[cfg(feature="stm32f427")]
pub use stm32f4::stm32f427::*;

#[cfg(feature="stm32f429")]
pub mod stm32f4;
#[cfg(feature="stm32f429")]
pub use stm32f4::stm32f429::*;

#[cfg(feature="stm32f446")]
pub mod stm32f4;
#[cfg(feature="stm32f446")]
pub use stm32f4::stm32f446::*;

#[cfg(feature="stm32f469")]
pub mod stm32f4;
#[cfg(feature="stm32f469")]
pub use stm32f4::stm32f469::*;

#[cfg(feature="doc")]
pub mod stm32f7;

#[cfg(feature="stm32f7x2")]
pub mod stm32f7;
#[cfg(feature="stm32f7x2")]
pub use stm32f7::stm32f7x2::*;

#[cfg(feature="stm32f7x3")]
pub mod stm32f7;
#[cfg(feature="stm32f7x3")]
pub use stm32f7::stm32f7x3::*;

#[cfg(feature="stm32f7x5")]
pub mod stm32f7;
#[cfg(feature="stm32f7x5")]
pub use stm32f7::stm32f7x5::*;

#[cfg(feature="stm32f7x6")]
pub mod stm32f7;
#[cfg(feature="stm32f7x6")]
pub use stm32f7::stm32f7x6::*;

#[cfg(feature="stm32f7x7")]
pub mod stm32f7;
#[cfg(feature="stm32f7x7")]
pub use stm32f7::stm32f7x7::*;

#[cfg(feature="stm32f7x9")]
pub mod stm32f7;
#[cfg(feature="stm32f7x9")]
pub use stm32f7::stm32f7x9::*;

#[cfg(feature="doc")]
pub mod stm32h7;

#[cfg(feature="stm32h7x3")]
pub mod stm32h7;
#[cfg(feature="stm32h7x3")]
pub use stm32h7::stm32h7x3::*;

#[cfg(feature="doc")]
pub mod stm32l0;

#[cfg(feature="stm32l0x1")]
pub mod stm32l0;
#[cfg(feature="stm32l0x1")]
pub use stm32l0::stm32l0x1::*;

#[cfg(feature="stm32l0x2")]
pub mod stm32l0;
#[cfg(feature="stm32l0x2")]
pub use stm32l0::stm32l0x2::*;

#[cfg(feature="stm32l0x3")]
pub mod stm32l0;
#[cfg(feature="stm32l0x3")]
pub use stm32l0::stm32l0x3::*;

#[cfg(feature="doc")]
pub mod stm32l1;

#[cfg(feature="stm32l100")]
pub mod stm32l1;
#[cfg(feature="stm32l100")]
pub use stm32l1::stm32l100::*;

#[cfg(feature="stm32l151")]
pub mod stm32l1;
#[cfg(feature="stm32l151")]
pub use stm32l1::stm32l151::*;

#[cfg(feature="stm32l162")]
pub mod stm32l1;
#[cfg(feature="stm32l162")]
pub use stm32l1::stm32l162::*;

#[cfg(feature="doc")]
pub mod stm32l4;

#[cfg(feature="stm32l4x1")]
pub mod stm32l4;
#[cfg(feature="stm32l4x1")]
pub use stm32l4::stm32l4x1::*;

#[cfg(feature="stm32l4x2")]
pub mod stm32l4;
#[cfg(feature="stm32l4x2")]
pub use stm32l4::stm32l4x2::*;

#[cfg(feature="stm32l4x3")]
pub mod stm32l4;
#[cfg(feature="stm32l4x3")]
pub use stm32l4::stm32l4x3::*;

#[cfg(feature="stm32l4x5")]
pub mod stm32l4;
#[cfg(feature="stm32l4x5")]
pub use stm32l4::stm32l4x5::*;

#[cfg(feature="stm32l4x6")]
pub mod stm32l4;
#[cfg(feature="stm32l4x6")]
pub use stm32l4::stm32l4x6::*;

