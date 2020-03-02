#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Touch sensing controller
//!
//! Used by: stm32f0x1, stm32f0x2, stm32f0x8

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// control register
pub mod CR {

    /// Charge transfer pulse high
    pub mod CTPH {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (4 bits: 0b1111 << 28)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Charge transfer pulse low
    pub mod CTPL {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Spread spectrum deviation
    pub mod SSD {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (7 bits: 0x7f << 17)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Spread spectrum enable
    pub mod SSE {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Spread spectrum prescaler
    pub mod SSPSC {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// pulse generator prescaler
    pub mod PGPSC {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (3 bits: 0b111 << 12)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Max count value
    pub mod MCV {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (3 bits: 0b111 << 5)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// I/O Default mode
    pub mod IODEF {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Synchronization pin polarity
    pub mod SYNCPOL {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Acquisition mode
    pub mod AM {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Start a new acquisition
    pub mod START {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Touch sensing controller enable
    pub mod TSCE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// interrupt enable register
pub mod IER {

    /// Max count error interrupt enable
    pub mod MCEIE {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// End of acquisition interrupt enable
    pub mod EOAIE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// interrupt clear register
pub mod ICR {

    /// Max count error interrupt clear
    pub mod MCEIC {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// End of acquisition interrupt clear
    pub mod EOAIC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// interrupt status register
pub mod ISR {

    /// Max count error flag
    pub mod MCEF {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// End of acquisition flag
    pub mod EOAF {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// I/O hysteresis control register
pub mod IOHCR {

    /// G6_IO4 Schmitt trigger hysteresis mode
    pub mod G6_IO4 {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// G6_IO3 Schmitt trigger hysteresis mode
    pub mod G6_IO3 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// G6_IO2 Schmitt trigger hysteresis mode
    pub mod G6_IO2 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// G6_IO1 Schmitt trigger hysteresis mode
    pub mod G6_IO1 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// G5_IO4 Schmitt trigger hysteresis mode
    pub mod G5_IO4 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// G5_IO3 Schmitt trigger hysteresis mode
    pub mod G5_IO3 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// G5_IO2 Schmitt trigger hysteresis mode
    pub mod G5_IO2 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// G5_IO1 Schmitt trigger hysteresis mode
    pub mod G5_IO1 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// G4_IO4 Schmitt trigger hysteresis mode
    pub mod G4_IO4 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// G4_IO3 Schmitt trigger hysteresis mode
    pub mod G4_IO3 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// G4_IO2 Schmitt trigger hysteresis mode
    pub mod G4_IO2 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// G4_IO1 Schmitt trigger hysteresis mode
    pub mod G4_IO1 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// G3_IO4 Schmitt trigger hysteresis mode
    pub mod G3_IO4 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// G3_IO3 Schmitt trigger hysteresis mode
    pub mod G3_IO3 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// G3_IO2 Schmitt trigger hysteresis mode
    pub mod G3_IO2 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// G3_IO1 Schmitt trigger hysteresis mode
    pub mod G3_IO1 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// G2_IO4 Schmitt trigger hysteresis mode
    pub mod G2_IO4 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// G2_IO3 Schmitt trigger hysteresis mode
    pub mod G2_IO3 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// G2_IO2 Schmitt trigger hysteresis mode
    pub mod G2_IO2 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// G2_IO1 Schmitt trigger hysteresis mode
    pub mod G2_IO1 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// G1_IO4 Schmitt trigger hysteresis mode
    pub mod G1_IO4 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// G1_IO3 Schmitt trigger hysteresis mode
    pub mod G1_IO3 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// G1_IO2 Schmitt trigger hysteresis mode
    pub mod G1_IO2 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// G1_IO1 Schmitt trigger hysteresis mode
    pub mod G1_IO1 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// I/O analog switch control register
pub mod IOASCR {
    pub use super::IOHCR::G1_IO1;
    pub use super::IOHCR::G1_IO2;
    pub use super::IOHCR::G1_IO3;
    pub use super::IOHCR::G1_IO4;
    pub use super::IOHCR::G2_IO1;
    pub use super::IOHCR::G2_IO2;
    pub use super::IOHCR::G2_IO3;
    pub use super::IOHCR::G2_IO4;
    pub use super::IOHCR::G3_IO1;
    pub use super::IOHCR::G3_IO2;
    pub use super::IOHCR::G3_IO3;
    pub use super::IOHCR::G3_IO4;
    pub use super::IOHCR::G4_IO1;
    pub use super::IOHCR::G4_IO2;
    pub use super::IOHCR::G4_IO3;
    pub use super::IOHCR::G4_IO4;
    pub use super::IOHCR::G5_IO1;
    pub use super::IOHCR::G5_IO2;
    pub use super::IOHCR::G5_IO3;
    pub use super::IOHCR::G5_IO4;
    pub use super::IOHCR::G6_IO1;
    pub use super::IOHCR::G6_IO2;
    pub use super::IOHCR::G6_IO3;
    pub use super::IOHCR::G6_IO4;
}

/// I/O sampling control register
pub mod IOSCR {
    pub use super::IOHCR::G1_IO1;
    pub use super::IOHCR::G1_IO2;
    pub use super::IOHCR::G1_IO3;
    pub use super::IOHCR::G1_IO4;
    pub use super::IOHCR::G2_IO1;
    pub use super::IOHCR::G2_IO2;
    pub use super::IOHCR::G2_IO3;
    pub use super::IOHCR::G2_IO4;
    pub use super::IOHCR::G3_IO1;
    pub use super::IOHCR::G3_IO2;
    pub use super::IOHCR::G3_IO3;
    pub use super::IOHCR::G3_IO4;
    pub use super::IOHCR::G4_IO1;
    pub use super::IOHCR::G4_IO2;
    pub use super::IOHCR::G4_IO3;
    pub use super::IOHCR::G4_IO4;
    pub use super::IOHCR::G5_IO1;
    pub use super::IOHCR::G5_IO2;
    pub use super::IOHCR::G5_IO3;
    pub use super::IOHCR::G5_IO4;
    pub use super::IOHCR::G6_IO1;
    pub use super::IOHCR::G6_IO2;
    pub use super::IOHCR::G6_IO3;
    pub use super::IOHCR::G6_IO4;
}

/// I/O channel control register
pub mod IOCCR {
    pub use super::IOHCR::G1_IO1;
    pub use super::IOHCR::G1_IO2;
    pub use super::IOHCR::G1_IO3;
    pub use super::IOHCR::G1_IO4;
    pub use super::IOHCR::G2_IO1;
    pub use super::IOHCR::G2_IO2;
    pub use super::IOHCR::G2_IO3;
    pub use super::IOHCR::G2_IO4;
    pub use super::IOHCR::G3_IO1;
    pub use super::IOHCR::G3_IO2;
    pub use super::IOHCR::G3_IO3;
    pub use super::IOHCR::G3_IO4;
    pub use super::IOHCR::G4_IO1;
    pub use super::IOHCR::G4_IO2;
    pub use super::IOHCR::G4_IO3;
    pub use super::IOHCR::G4_IO4;
    pub use super::IOHCR::G5_IO1;
    pub use super::IOHCR::G5_IO2;
    pub use super::IOHCR::G5_IO3;
    pub use super::IOHCR::G5_IO4;
    pub use super::IOHCR::G6_IO1;
    pub use super::IOHCR::G6_IO2;
    pub use super::IOHCR::G6_IO3;
    pub use super::IOHCR::G6_IO4;
}

/// I/O group control status register
pub mod IOGCSR {

    /// Analog I/O group x status
    pub mod G8S {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Analog I/O group x status
    pub mod G7S {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Analog I/O group x status
    pub mod G6S {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Analog I/O group x status
    pub mod G5S {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Analog I/O group x status
    pub mod G4S {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Analog I/O group x status
    pub mod G3S {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Analog I/O group x status
    pub mod G2S {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Analog I/O group x status
    pub mod G1S {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Analog I/O group x enable
    pub mod G8E {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Analog I/O group x enable
    pub mod G7E {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Analog I/O group x enable
    pub mod G6E {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Analog I/O group x enable
    pub mod G5E {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Analog I/O group x enable
    pub mod G4E {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Analog I/O group x enable
    pub mod G3E {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Analog I/O group x enable
    pub mod G2E {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Analog I/O group x enable
    pub mod G1E {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// I/O group x counter register
pub mod IOG1CR {

    /// Counter value
    pub mod CNT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (14 bits: 0x3fff << 0)
        pub const mask: u32 = 0x3fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// I/O group x counter register
pub mod IOG2CR {
    pub use super::IOG1CR::CNT;
}

/// I/O group x counter register
pub mod IOG3CR {
    pub use super::IOG1CR::CNT;
}

/// I/O group x counter register
pub mod IOG4CR {
    pub use super::IOG1CR::CNT;
}

/// I/O group x counter register
pub mod IOG5CR {
    pub use super::IOG1CR::CNT;
}

/// I/O group x counter register
pub mod IOG6CR {
    pub use super::IOG1CR::CNT;
}
#[repr(C)]
pub struct RegisterBlock {
    /// control register
    pub CR: RWRegister<u32>,

    /// interrupt enable register
    pub IER: RWRegister<u32>,

    /// interrupt clear register
    pub ICR: RWRegister<u32>,

    /// interrupt status register
    pub ISR: RWRegister<u32>,

    /// I/O hysteresis control register
    pub IOHCR: RWRegister<u32>,

    _reserved1: [u32; 1],

    /// I/O analog switch control register
    pub IOASCR: RWRegister<u32>,

    _reserved2: [u32; 1],

    /// I/O sampling control register
    pub IOSCR: RWRegister<u32>,

    _reserved3: [u32; 1],

    /// I/O channel control register
    pub IOCCR: RWRegister<u32>,

    _reserved4: [u32; 1],

    /// I/O group control status register
    pub IOGCSR: RWRegister<u32>,

    /// I/O group x counter register
    pub IOG1CR: RORegister<u32>,

    /// I/O group x counter register
    pub IOG2CR: RORegister<u32>,

    /// I/O group x counter register
    pub IOG3CR: RORegister<u32>,

    /// I/O group x counter register
    pub IOG4CR: RORegister<u32>,

    /// I/O group x counter register
    pub IOG5CR: RORegister<u32>,

    /// I/O group x counter register
    pub IOG6CR: RORegister<u32>,
}
pub struct ResetValues {
    pub CR: u32,
    pub IER: u32,
    pub ICR: u32,
    pub ISR: u32,
    pub IOHCR: u32,
    pub IOASCR: u32,
    pub IOSCR: u32,
    pub IOCCR: u32,
    pub IOGCSR: u32,
    pub IOG1CR: u32,
    pub IOG2CR: u32,
    pub IOG3CR: u32,
    pub IOG4CR: u32,
    pub IOG5CR: u32,
    pub IOG6CR: u32,
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
