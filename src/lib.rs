// Copyright 2018 Adam Greig
// See LICENSE-APACHE and LICENSE-MIT for license details.

//! This project provides a register abstraction layer (RAL) for all STM32 microcontrollers.

#![no_std]

#[macro_use]
mod register;

pub use register::{RORegister, RWRegister, SafeRORegister, SafeRWRegister};

#[cfg(any(feature = "stm32f405", feature = "doc"))]
pub mod stm32f4;
#[cfg(feature = "stm32f405")]
pub use stm32f4::stm32f405::*;
