#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! GPIO
//!
//! Used by: stm32h743, stm32h743v, stm32h747cm4, stm32h747cm7, stm32h753, stm32h753v

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// GPIO port mode register
pub mod MODER {

    /// \[1:0\]: Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O mode.
    pub mod MODER0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Input mode (reset state)
            pub const Input: u32 = 0b00;

            /// 0b01: General purpose output mode
            pub const Output: u32 = 0b01;

            /// 0b10: Alternate function mode
            pub const Alternate: u32 = 0b10;

            /// 0b11: Analog mode
            pub const Analog: u32 = 0b11;
        }
    }

    /// \[1:0\]: Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O mode.
    pub mod MODER1 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MODER0::RW;
    }

    /// \[1:0\]: Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O mode.
    pub mod MODER2 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MODER0::RW;
    }

    /// \[1:0\]: Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O mode.
    pub mod MODER3 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (2 bits: 0b11 << 6)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MODER0::RW;
    }

    /// \[1:0\]: Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O mode.
    pub mod MODER4 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MODER0::RW;
    }

    /// \[1:0\]: Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O mode.
    pub mod MODER5 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (2 bits: 0b11 << 10)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MODER0::RW;
    }

    /// \[1:0\]: Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O mode.
    pub mod MODER6 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (2 bits: 0b11 << 12)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MODER0::RW;
    }

    /// \[1:0\]: Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O mode.
    pub mod MODER7 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (2 bits: 0b11 << 14)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MODER0::RW;
    }

    /// \[1:0\]: Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O mode.
    pub mod MODER8 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (2 bits: 0b11 << 16)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MODER0::RW;
    }

    /// \[1:0\]: Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O mode.
    pub mod MODER9 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (2 bits: 0b11 << 18)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MODER0::RW;
    }

    /// \[1:0\]: Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O mode.
    pub mod MODER10 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (2 bits: 0b11 << 20)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MODER0::RW;
    }

    /// \[1:0\]: Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O mode.
    pub mod MODER11 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (2 bits: 0b11 << 22)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MODER0::RW;
    }

    /// \[1:0\]: Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O mode.
    pub mod MODER12 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (2 bits: 0b11 << 24)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MODER0::RW;
    }

    /// \[1:0\]: Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O mode.
    pub mod MODER13 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (2 bits: 0b11 << 26)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MODER0::RW;
    }

    /// \[1:0\]: Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O mode.
    pub mod MODER14 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (2 bits: 0b11 << 28)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MODER0::RW;
    }

    /// \[1:0\]: Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O mode.
    pub mod MODER15 {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MODER0::RW;
    }
}

/// GPIO port output type register
pub mod OTYPER {

    /// Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O output type.
    pub mod OT0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Output push-pull (reset state)
            pub const PushPull: u32 = 0b0;

            /// 0b1: Output open-drain
            pub const OpenDrain: u32 = 0b1;
        }
    }

    /// Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O output type.
    pub mod OT1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OT0::RW;
    }

    /// Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O output type.
    pub mod OT2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OT0::RW;
    }

    /// Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O output type.
    pub mod OT3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OT0::RW;
    }

    /// Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O output type.
    pub mod OT4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OT0::RW;
    }

    /// Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O output type.
    pub mod OT5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OT0::RW;
    }

    /// Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O output type.
    pub mod OT6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OT0::RW;
    }

    /// Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O output type.
    pub mod OT7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OT0::RW;
    }

    /// Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O output type.
    pub mod OT8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OT0::RW;
    }

    /// Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O output type.
    pub mod OT9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OT0::RW;
    }

    /// Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O output type.
    pub mod OT10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OT0::RW;
    }

    /// Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O output type.
    pub mod OT11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OT0::RW;
    }

    /// Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O output type.
    pub mod OT12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OT0::RW;
    }

    /// Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O output type.
    pub mod OT13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OT0::RW;
    }

    /// Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O output type.
    pub mod OT14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OT0::RW;
    }

    /// Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O output type.
    pub mod OT15 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OT0::RW;
    }
}

/// GPIO port output speed register
pub mod OSPEEDR {

    /// \[1:0\]: Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.
    pub mod OSPEEDR0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Low speed
            pub const LowSpeed: u32 = 0b00;

            /// 0b01: Medium speed
            pub const MediumSpeed: u32 = 0b01;

            /// 0b10: High speed
            pub const HighSpeed: u32 = 0b10;

            /// 0b11: Very high speed
            pub const VeryHighSpeed: u32 = 0b11;
        }
    }

    /// \[1:0\]: Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.
    pub mod OSPEEDR1 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OSPEEDR0::RW;
    }

    /// \[1:0\]: Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.
    pub mod OSPEEDR2 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OSPEEDR0::RW;
    }

    /// \[1:0\]: Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.
    pub mod OSPEEDR3 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (2 bits: 0b11 << 6)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OSPEEDR0::RW;
    }

    /// \[1:0\]: Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.
    pub mod OSPEEDR4 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OSPEEDR0::RW;
    }

    /// \[1:0\]: Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.
    pub mod OSPEEDR5 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (2 bits: 0b11 << 10)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OSPEEDR0::RW;
    }

    /// \[1:0\]: Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.
    pub mod OSPEEDR6 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (2 bits: 0b11 << 12)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OSPEEDR0::RW;
    }

    /// \[1:0\]: Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.
    pub mod OSPEEDR7 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (2 bits: 0b11 << 14)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OSPEEDR0::RW;
    }

    /// \[1:0\]: Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.
    pub mod OSPEEDR8 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (2 bits: 0b11 << 16)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OSPEEDR0::RW;
    }

    /// \[1:0\]: Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.
    pub mod OSPEEDR9 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (2 bits: 0b11 << 18)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OSPEEDR0::RW;
    }

    /// \[1:0\]: Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.
    pub mod OSPEEDR10 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (2 bits: 0b11 << 20)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OSPEEDR0::RW;
    }

    /// \[1:0\]: Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.
    pub mod OSPEEDR11 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (2 bits: 0b11 << 22)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OSPEEDR0::RW;
    }

    /// \[1:0\]: Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.
    pub mod OSPEEDR12 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (2 bits: 0b11 << 24)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OSPEEDR0::RW;
    }

    /// \[1:0\]: Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.
    pub mod OSPEEDR13 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (2 bits: 0b11 << 26)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OSPEEDR0::RW;
    }

    /// \[1:0\]: Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.
    pub mod OSPEEDR14 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (2 bits: 0b11 << 28)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OSPEEDR0::RW;
    }

    /// \[1:0\]: Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O output speed. Note: Refer to the device datasheet for the frequency specifications and the power supply and load conditions for each speed.
    pub mod OSPEEDR15 {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OSPEEDR0::RW;
    }
}

/// GPIO port pull-up/pull-down register
pub mod PUPDR {

    /// \[1:0\]: Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O pull-up or pull-down
    pub mod PUPDR0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: No pull-up, pull-down
            pub const Floating: u32 = 0b00;

            /// 0b01: Pull-up
            pub const PullUp: u32 = 0b01;

            /// 0b10: Pull-down
            pub const PullDown: u32 = 0b10;
        }
    }

    /// \[1:0\]: Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O pull-up or pull-down
    pub mod PUPDR1 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PUPDR0::RW;
    }

    /// \[1:0\]: Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O pull-up or pull-down
    pub mod PUPDR2 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PUPDR0::RW;
    }

    /// \[1:0\]: Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O pull-up or pull-down
    pub mod PUPDR3 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (2 bits: 0b11 << 6)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PUPDR0::RW;
    }

    /// \[1:0\]: Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O pull-up or pull-down
    pub mod PUPDR4 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PUPDR0::RW;
    }

    /// \[1:0\]: Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O pull-up or pull-down
    pub mod PUPDR5 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (2 bits: 0b11 << 10)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PUPDR0::RW;
    }

    /// \[1:0\]: Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O pull-up or pull-down
    pub mod PUPDR6 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (2 bits: 0b11 << 12)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PUPDR0::RW;
    }

    /// \[1:0\]: Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O pull-up or pull-down
    pub mod PUPDR7 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (2 bits: 0b11 << 14)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PUPDR0::RW;
    }

    /// \[1:0\]: Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O pull-up or pull-down
    pub mod PUPDR8 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (2 bits: 0b11 << 16)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PUPDR0::RW;
    }

    /// \[1:0\]: Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O pull-up or pull-down
    pub mod PUPDR9 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (2 bits: 0b11 << 18)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PUPDR0::RW;
    }

    /// \[1:0\]: Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O pull-up or pull-down
    pub mod PUPDR10 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (2 bits: 0b11 << 20)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PUPDR0::RW;
    }

    /// \[1:0\]: Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O pull-up or pull-down
    pub mod PUPDR11 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (2 bits: 0b11 << 22)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PUPDR0::RW;
    }

    /// \[1:0\]: Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O pull-up or pull-down
    pub mod PUPDR12 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (2 bits: 0b11 << 24)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PUPDR0::RW;
    }

    /// \[1:0\]: Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O pull-up or pull-down
    pub mod PUPDR13 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (2 bits: 0b11 << 26)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PUPDR0::RW;
    }

    /// \[1:0\]: Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O pull-up or pull-down
    pub mod PUPDR14 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (2 bits: 0b11 << 28)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PUPDR0::RW;
    }

    /// \[1:0\]: Port x configuration bits (y = 0..15) These bits are written by software to configure the I/O pull-up or pull-down
    pub mod PUPDR15 {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PUPDR0::RW;
    }
}

/// GPIO port input data register
pub mod IDR {

    /// Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port.
    pub mod IDR0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b1: Input is logic high
            pub const High: u32 = 0b1;

            /// 0b0: Input is logic low
            pub const Low: u32 = 0b0;
        }
    }

    /// Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port.
    pub mod IDR1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IDR0::RW;
    }

    /// Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port.
    pub mod IDR2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IDR0::RW;
    }

    /// Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port.
    pub mod IDR3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IDR0::RW;
    }

    /// Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port.
    pub mod IDR4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IDR0::RW;
    }

    /// Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port.
    pub mod IDR5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IDR0::RW;
    }

    /// Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port.
    pub mod IDR6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IDR0::RW;
    }

    /// Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port.
    pub mod IDR7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IDR0::RW;
    }

    /// Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port.
    pub mod IDR8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IDR0::RW;
    }

    /// Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port.
    pub mod IDR9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IDR0::RW;
    }

    /// Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port.
    pub mod IDR10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IDR0::RW;
    }

    /// Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port.
    pub mod IDR11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IDR0::RW;
    }

    /// Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port.
    pub mod IDR12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IDR0::RW;
    }

    /// Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port.
    pub mod IDR13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IDR0::RW;
    }

    /// Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port.
    pub mod IDR14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IDR0::RW;
    }

    /// Port input data bit (y = 0..15) These bits are read-only. They contain the input value of the corresponding I/O port.
    pub mod IDR15 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::IDR0::RW;
    }
}

/// GPIO port output data register
pub mod ODR {

    /// Port output data bit These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A..F).
    pub mod ODR0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b1: Set output to logic high
            pub const High: u32 = 0b1;

            /// 0b0: Set output to logic low
            pub const Low: u32 = 0b0;
        }
    }

    /// Port output data bit These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A..F).
    pub mod ODR1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ODR0::RW;
    }

    /// Port output data bit These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A..F).
    pub mod ODR2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ODR0::RW;
    }

    /// Port output data bit These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A..F).
    pub mod ODR3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ODR0::RW;
    }

    /// Port output data bit These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A..F).
    pub mod ODR4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ODR0::RW;
    }

    /// Port output data bit These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A..F).
    pub mod ODR5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ODR0::RW;
    }

    /// Port output data bit These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A..F).
    pub mod ODR6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ODR0::RW;
    }

    /// Port output data bit These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A..F).
    pub mod ODR7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ODR0::RW;
    }

    /// Port output data bit These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A..F).
    pub mod ODR8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ODR0::RW;
    }

    /// Port output data bit These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A..F).
    pub mod ODR9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ODR0::RW;
    }

    /// Port output data bit These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A..F).
    pub mod ODR10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ODR0::RW;
    }

    /// Port output data bit These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A..F).
    pub mod ODR11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ODR0::RW;
    }

    /// Port output data bit These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A..F).
    pub mod ODR12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ODR0::RW;
    }

    /// Port output data bit These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A..F).
    pub mod ODR13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ODR0::RW;
    }

    /// Port output data bit These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A..F).
    pub mod ODR14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ODR0::RW;
    }

    /// Port output data bit These bits can be read and written by software. Note: For atomic bit set/reset, the OD bits can be individually set and/or reset by writing to the GPIOx_BSRR or GPIOx_BRR registers (x = A..F).
    pub mod ODR15 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::ODR0::RW;
    }
}

/// GPIO port bit set/reset register
pub mod BSRR {

    /// Port x set bit y (y= 0..15) These bits are write-only. A read to these bits returns the value 0x0000.
    pub mod BS0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Sets the corresponding ODRx bit
            pub const Set: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port x set bit y (y= 0..15) These bits are write-only. A read to these bits returns the value 0x0000.
    pub mod BS1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::BS0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port x set bit y (y= 0..15) These bits are write-only. A read to these bits returns the value 0x0000.
    pub mod BS2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::BS0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port x set bit y (y= 0..15) These bits are write-only. A read to these bits returns the value 0x0000.
    pub mod BS3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::BS0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port x set bit y (y= 0..15) These bits are write-only. A read to these bits returns the value 0x0000.
    pub mod BS4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::BS0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port x set bit y (y= 0..15) These bits are write-only. A read to these bits returns the value 0x0000.
    pub mod BS5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::BS0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port x set bit y (y= 0..15) These bits are write-only. A read to these bits returns the value 0x0000.
    pub mod BS6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::BS0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port x set bit y (y= 0..15) These bits are write-only. A read to these bits returns the value 0x0000.
    pub mod BS7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::BS0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port x set bit y (y= 0..15) These bits are write-only. A read to these bits returns the value 0x0000.
    pub mod BS8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::BS0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port x set bit y (y= 0..15) These bits are write-only. A read to these bits returns the value 0x0000.
    pub mod BS9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::BS0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port x set bit y (y= 0..15) These bits are write-only. A read to these bits returns the value 0x0000.
    pub mod BS10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::BS0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port x set bit y (y= 0..15) These bits are write-only. A read to these bits returns the value 0x0000.
    pub mod BS11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::BS0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port x set bit y (y= 0..15) These bits are write-only. A read to these bits returns the value 0x0000.
    pub mod BS12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::BS0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port x set bit y (y= 0..15) These bits are write-only. A read to these bits returns the value 0x0000.
    pub mod BS13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::BS0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port x set bit y (y= 0..15) These bits are write-only. A read to these bits returns the value 0x0000.
    pub mod BS14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::BS0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port x set bit y (y= 0..15) These bits are write-only. A read to these bits returns the value 0x0000.
    pub mod BS15 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::BS0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port x reset bit y (y = 0..15) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority.
    pub mod BR0 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Resets the corresponding ODRx bit
            pub const Reset: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port x reset bit y (y = 0..15) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority.
    pub mod BR1 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::BR0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port x reset bit y (y = 0..15) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority.
    pub mod BR2 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::BR0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port x reset bit y (y = 0..15) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority.
    pub mod BR3 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::BR0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port x reset bit y (y = 0..15) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority.
    pub mod BR4 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::BR0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port x reset bit y (y = 0..15) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority.
    pub mod BR5 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::BR0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port x reset bit y (y = 0..15) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority.
    pub mod BR6 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::BR0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port x reset bit y (y = 0..15) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority.
    pub mod BR7 {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::BR0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port x reset bit y (y = 0..15) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority.
    pub mod BR8 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::BR0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port x reset bit y (y = 0..15) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority.
    pub mod BR9 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::BR0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port x reset bit y (y = 0..15) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority.
    pub mod BR10 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::BR0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port x reset bit y (y = 0..15) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority.
    pub mod BR11 {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::BR0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port x reset bit y (y = 0..15) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority.
    pub mod BR12 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::BR0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port x reset bit y (y = 0..15) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority.
    pub mod BR13 {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::BR0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port x reset bit y (y = 0..15) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority.
    pub mod BR14 {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::BR0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port x reset bit y (y = 0..15) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSx and BRx are set, BSx has priority.
    pub mod BR15 {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::BR0::W;
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \[15:0\] is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\[15:0\] must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset.A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence.Each lock bit freezes a specific configuration register (control and alternate function registers).
pub mod LCKR {

    /// Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
    pub mod LCK0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Port configuration not locked
            pub const Unlocked: u32 = 0b0;

            /// 0b1: Port configuration locked
            pub const Locked: u32 = 0b1;
        }
    }

    /// Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
    pub mod LCK1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LCK0::RW;
    }

    /// Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
    pub mod LCK2 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LCK0::RW;
    }

    /// Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
    pub mod LCK3 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LCK0::RW;
    }

    /// Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
    pub mod LCK4 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LCK0::RW;
    }

    /// Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
    pub mod LCK5 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LCK0::RW;
    }

    /// Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
    pub mod LCK6 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LCK0::RW;
    }

    /// Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
    pub mod LCK7 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LCK0::RW;
    }

    /// Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
    pub mod LCK8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LCK0::RW;
    }

    /// Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
    pub mod LCK9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LCK0::RW;
    }

    /// Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
    pub mod LCK10 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LCK0::RW;
    }

    /// Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
    pub mod LCK11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LCK0::RW;
    }

    /// Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
    pub mod LCK12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LCK0::RW;
    }

    /// Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
    pub mod LCK13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LCK0::RW;
    }

    /// Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
    pub mod LCK14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LCK0::RW;
    }

    /// Port x lock bit y (y= 0..15) These bits are read/write but can only be written when the LCKK bit is 0.
    pub mod LCK15 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::LCK0::RW;
    }

    /// Lock key This bit can be read any time. It can only be modified using the lock key write sequence. LOCK key write sequence: WR LCKR\[16\] = 1 + LCKR\[15:0\] WR LCKR\[16\] = 0 + LCKR\[15:0\] WR LCKR\[16\] = 1 + LCKR\[15:0\] RD LCKR RD LCKR\[16\] = 1 (this read operation is optional but it confirms that the lock is active) Note: During the LOCK key write sequence, the value of LCK\[15:0\] must not change. Any error in the lock sequence aborts the lock. After the first lock sequence on any bit of the port, any read access on the LCKK bit will return 1 until the next MCU reset or peripheral reset.
    pub mod LCKK {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Port configuration lock key not active
            pub const NotActive: u32 = 0b0;

            /// 0b1: Port configuration lock key active
            pub const Active: u32 = 0b1;
        }
    }
}

/// GPIO alternate function low register
pub mod AFRL {

    /// \[3:0\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:
    pub mod AFR0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: AF0
            pub const AF0: u32 = 0b0000;

            /// 0b0001: AF1
            pub const AF1: u32 = 0b0001;

            /// 0b0010: AF2
            pub const AF2: u32 = 0b0010;

            /// 0b0011: AF3
            pub const AF3: u32 = 0b0011;

            /// 0b0100: AF4
            pub const AF4: u32 = 0b0100;

            /// 0b0101: AF5
            pub const AF5: u32 = 0b0101;

            /// 0b0110: AF6
            pub const AF6: u32 = 0b0110;

            /// 0b0111: AF7
            pub const AF7: u32 = 0b0111;

            /// 0b1000: AF8
            pub const AF8: u32 = 0b1000;

            /// 0b1001: AF9
            pub const AF9: u32 = 0b1001;

            /// 0b1010: AF10
            pub const AF10: u32 = 0b1010;

            /// 0b1011: AF11
            pub const AF11: u32 = 0b1011;

            /// 0b1100: AF12
            pub const AF12: u32 = 0b1100;

            /// 0b1101: AF13
            pub const AF13: u32 = 0b1101;

            /// 0b1110: AF14
            pub const AF14: u32 = 0b1110;

            /// 0b1111: AF15
            pub const AF15: u32 = 0b1111;
        }
    }

    /// \[3:0\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:
    pub mod AFR1 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AFR0::RW;
    }

    /// \[3:0\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:
    pub mod AFR2 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AFR0::RW;
    }

    /// \[3:0\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:
    pub mod AFR3 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AFR0::RW;
    }

    /// \[3:0\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:
    pub mod AFR4 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AFR0::RW;
    }

    /// \[3:0\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:
    pub mod AFR5 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AFR0::RW;
    }

    /// \[3:0\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:
    pub mod AFR6 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AFR0::RW;
    }

    /// \[3:0\]: Alternate function selection for port x pin y (y = 0..7) These bits are written by software to configure alternate function I/Os AFSELy selection:
    pub mod AFR7 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (4 bits: 0b1111 << 28)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AFR0::RW;
    }
}

/// GPIO alternate function high register
pub mod AFRH {

    /// \[3:0\]: Alternate function selection for port x pin y (y = 8..15) These bits are written by software to configure alternate function I/Os
    pub mod AFR8 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: AF0
            pub const AF0: u32 = 0b0000;

            /// 0b0001: AF1
            pub const AF1: u32 = 0b0001;

            /// 0b0010: AF2
            pub const AF2: u32 = 0b0010;

            /// 0b0011: AF3
            pub const AF3: u32 = 0b0011;

            /// 0b0100: AF4
            pub const AF4: u32 = 0b0100;

            /// 0b0101: AF5
            pub const AF5: u32 = 0b0101;

            /// 0b0110: AF6
            pub const AF6: u32 = 0b0110;

            /// 0b0111: AF7
            pub const AF7: u32 = 0b0111;

            /// 0b1000: AF8
            pub const AF8: u32 = 0b1000;

            /// 0b1001: AF9
            pub const AF9: u32 = 0b1001;

            /// 0b1010: AF10
            pub const AF10: u32 = 0b1010;

            /// 0b1011: AF11
            pub const AF11: u32 = 0b1011;

            /// 0b1100: AF12
            pub const AF12: u32 = 0b1100;

            /// 0b1101: AF13
            pub const AF13: u32 = 0b1101;

            /// 0b1110: AF14
            pub const AF14: u32 = 0b1110;

            /// 0b1111: AF15
            pub const AF15: u32 = 0b1111;
        }
    }

    /// \[3:0\]: Alternate function selection for port x pin y (y = 8..15) These bits are written by software to configure alternate function I/Os
    pub mod AFR9 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AFR8::RW;
    }

    /// \[3:0\]: Alternate function selection for port x pin y (y = 8..15) These bits are written by software to configure alternate function I/Os
    pub mod AFR10 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AFR8::RW;
    }

    /// \[3:0\]: Alternate function selection for port x pin y (y = 8..15) These bits are written by software to configure alternate function I/Os
    pub mod AFR11 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AFR8::RW;
    }

    /// \[3:0\]: Alternate function selection for port x pin y (y = 8..15) These bits are written by software to configure alternate function I/Os
    pub mod AFR12 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AFR8::RW;
    }

    /// \[3:0\]: Alternate function selection for port x pin y (y = 8..15) These bits are written by software to configure alternate function I/Os
    pub mod AFR13 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AFR8::RW;
    }

    /// \[3:0\]: Alternate function selection for port x pin y (y = 8..15) These bits are written by software to configure alternate function I/Os
    pub mod AFR14 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AFR8::RW;
    }

    /// \[3:0\]: Alternate function selection for port x pin y (y = 8..15) These bits are written by software to configure alternate function I/Os
    pub mod AFR15 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (4 bits: 0b1111 << 28)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::AFR8::RW;
    }
}
pub struct RegisterBlock {
    /// GPIO port mode register
    pub MODER: RWRegister<u32>,

    /// GPIO port output type register
    pub OTYPER: RWRegister<u32>,

    /// GPIO port output speed register
    pub OSPEEDR: RWRegister<u32>,

    /// GPIO port pull-up/pull-down register
    pub PUPDR: RWRegister<u32>,

    /// GPIO port input data register
    pub IDR: RORegister<u32>,

    /// GPIO port output data register
    pub ODR: RWRegister<u32>,

    /// GPIO port bit set/reset register
    pub BSRR: WORegister<u32>,

    /// This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \[15:0\] is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\[15:0\] must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset.A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence.Each lock bit freezes a specific configuration register (control and alternate function registers).
    pub LCKR: RWRegister<u32>,

    /// GPIO alternate function low register
    pub AFRL: RWRegister<u32>,

    /// GPIO alternate function high register
    pub AFRH: RWRegister<u32>,
}
pub struct ResetValues {
    pub MODER: u32,
    pub OTYPER: u32,
    pub OSPEEDR: u32,
    pub PUPDR: u32,
    pub IDR: u32,
    pub ODR: u32,
    pub BSRR: u32,
    pub LCKR: u32,
    pub AFRL: u32,
    pub AFRH: u32,
}
#[cfg(not(feature = "nosync"))]
pub struct Instance {
    pub(crate) addr: u32,
    pub(crate) _marker: PhantomData<*const RegisterBlock>,
}
#[cfg(not(feature = "nosync"))]
impl ::core::ops::Deref for Instance {
    type Target = RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &RegisterBlock {
        unsafe { &*(self.addr as *const _) }
    }
}
#[cfg(feature = "rtfm")]
unsafe impl Send for Instance {}
