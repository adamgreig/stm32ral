#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! MDMA1
//!
//! Used by: stm32mp153, stm32mp157

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// MDMA global interrupt/status register
pub mod MDMA_GISR0 {

    /// GIF0
    pub mod GIF0 {
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

    /// GIF1
    pub mod GIF1 {
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

    /// GIF2
    pub mod GIF2 {
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

    /// GIF3
    pub mod GIF3 {
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

    /// GIF4
    pub mod GIF4 {
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

    /// GIF5
    pub mod GIF5 {
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

    /// GIF6
    pub mod GIF6 {
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

    /// GIF7
    pub mod GIF7 {
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

    /// GIF8
    pub mod GIF8 {
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

    /// GIF9
    pub mod GIF9 {
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

    /// GIF10
    pub mod GIF10 {
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

    /// GIF11
    pub mod GIF11 {
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

    /// GIF12
    pub mod GIF12 {
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

    /// GIF13
    pub mod GIF13 {
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

    /// GIF14
    pub mod GIF14 {
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

    /// GIF15
    pub mod GIF15 {
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

    /// GIF16
    pub mod GIF16 {
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

    /// GIF17
    pub mod GIF17 {
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

    /// GIF18
    pub mod GIF18 {
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

    /// GIF19
    pub mod GIF19 {
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

    /// GIF20
    pub mod GIF20 {
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

    /// GIF21
    pub mod GIF21 {
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

    /// GIF22
    pub mod GIF22 {
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

    /// GIF23
    pub mod GIF23 {
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

    /// GIF24
    pub mod GIF24 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// GIF25
    pub mod GIF25 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// GIF26
    pub mod GIF26 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// GIF27
    pub mod GIF27 {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// GIF28
    pub mod GIF28 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// GIF29
    pub mod GIF29 {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// GIF30
    pub mod GIF30 {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// GIF31
    pub mod GIF31 {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// MDMA secure global interrupt/status register
pub mod MDMA_SGISR0 {
    pub use super::MDMA_GISR0::GIF0;
    pub use super::MDMA_GISR0::GIF1;
    pub use super::MDMA_GISR0::GIF10;
    pub use super::MDMA_GISR0::GIF11;
    pub use super::MDMA_GISR0::GIF12;
    pub use super::MDMA_GISR0::GIF13;
    pub use super::MDMA_GISR0::GIF14;
    pub use super::MDMA_GISR0::GIF15;
    pub use super::MDMA_GISR0::GIF16;
    pub use super::MDMA_GISR0::GIF17;
    pub use super::MDMA_GISR0::GIF18;
    pub use super::MDMA_GISR0::GIF19;
    pub use super::MDMA_GISR0::GIF2;
    pub use super::MDMA_GISR0::GIF20;
    pub use super::MDMA_GISR0::GIF21;
    pub use super::MDMA_GISR0::GIF22;
    pub use super::MDMA_GISR0::GIF23;
    pub use super::MDMA_GISR0::GIF24;
    pub use super::MDMA_GISR0::GIF25;
    pub use super::MDMA_GISR0::GIF26;
    pub use super::MDMA_GISR0::GIF27;
    pub use super::MDMA_GISR0::GIF28;
    pub use super::MDMA_GISR0::GIF29;
    pub use super::MDMA_GISR0::GIF3;
    pub use super::MDMA_GISR0::GIF30;
    pub use super::MDMA_GISR0::GIF31;
    pub use super::MDMA_GISR0::GIF4;
    pub use super::MDMA_GISR0::GIF5;
    pub use super::MDMA_GISR0::GIF6;
    pub use super::MDMA_GISR0::GIF7;
    pub use super::MDMA_GISR0::GIF8;
    pub use super::MDMA_GISR0::GIF9;
}

/// MDMA channel 0 interrupt/status register
pub mod MDMA_C0ISR {

    /// TEIF
    pub mod TEIF {
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

    /// CTCIF
    pub mod CTCIF {
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

    /// BRTIF
    pub mod BRTIF {
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

    /// BTIF
    pub mod BTIF {
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

    /// TCIF
    pub mod TCIF {
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

    /// CRQA
    pub mod CRQA {
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
}

/// MDMA channel 0 interrupt flag clear register
pub mod MDMA_C0IFCR {

    /// CTEIF
    pub mod CTEIF {
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

    /// CCTCIF
    pub mod CCTCIF {
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

    /// CBRTIF
    pub mod CBRTIF {
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

    /// CBTIF
    pub mod CBTIF {
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

    /// CLTCIF
    pub mod CLTCIF {
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
}

/// MDMA channel 0 error status register
pub mod MDMA_C0ESR {

    /// TEA
    pub mod TEA {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TED
    pub mod TED {
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

    /// TELD
    pub mod TELD {
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

    /// TEMD
    pub mod TEMD {
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

    /// ASE
    pub mod ASE {
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

    /// BSE
    pub mod BSE {
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
}

/// This register is used to control the concerned channel.
pub mod MDMA_C0CR {

    /// EN
    pub mod EN {
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

    /// TEIE
    pub mod TEIE {
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

    /// CTCIE
    pub mod CTCIE {
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

    /// BRTIE
    pub mod BRTIE {
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

    /// BTIE
    pub mod BTIE {
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

    /// TCIE
    pub mod TCIE {
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

    /// PL
    pub mod PL {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (2 bits: 0b11 << 6)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// BEX
    pub mod BEX {
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

    /// HEX
    pub mod HEX {
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

    /// WEX
    pub mod WEX {
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

    /// SWRQ
    pub mod SWRQ {
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
}

/// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod MDMA_C0TCR {

    /// SINC
    pub mod SINC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DINC
    pub mod DINC {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SSIZE
    pub mod SSIZE {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DSIZE
    pub mod DSIZE {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (2 bits: 0b11 << 6)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SINCOS
    pub mod SINCOS {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DINCOS
    pub mod DINCOS {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (2 bits: 0b11 << 10)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SBURST
    pub mod SBURST {
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

    /// DBURST
    pub mod DBURST {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (3 bits: 0b111 << 15)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TLEN
    pub mod TLEN {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (7 bits: 0x7f << 18)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PKE
    pub mod PKE {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PAM
    pub mod PAM {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (2 bits: 0b11 << 26)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TRGM
    pub mod TRGM {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (2 bits: 0b11 << 28)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SWRM
    pub mod SWRM {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// BWM
    pub mod BWM {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod MDMA_C0BNDTR {

    /// BNDT
    pub mod BNDT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (17 bits: 0x1ffff << 0)
        pub const mask: u32 = 0x1ffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// BRSUM
    pub mod BRSUM {
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

    /// BRDUM
    pub mod BRDUM {
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

    /// BRC
    pub mod BRC {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (12 bits: 0xfff << 20)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod MDMA_C0SAR {

    /// SAR
    pub mod SAR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod MDMA_C0DAR {

    /// DAR
    pub mod DAR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod MDMA_C0BRUR {

    /// SUV
    pub mod SUV {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DUV
    pub mod DUV {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (16 bits: 0xffff << 16)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod MDMA_C0LAR {

    /// LAR
    pub mod LAR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod MDMA_C0TBR {

    /// TSEL
    pub mod TSEL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SBUS
    pub mod SBUS {
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

    /// DBUS
    pub mod DBUS {
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
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod MDMA_C0MAR {

    /// MAR
    pub mod MAR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod MDMA_C0MDR {

    /// MDR
    pub mod MDR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// MDMA channel 1 interrupt/status register
pub mod MDMA_C1ISR {
    pub use super::MDMA_C0ISR::BRTIF;
    pub use super::MDMA_C0ISR::BTIF;
    pub use super::MDMA_C0ISR::CRQA;
    pub use super::MDMA_C0ISR::CTCIF;
    pub use super::MDMA_C0ISR::TCIF;
    pub use super::MDMA_C0ISR::TEIF;
}

/// MDMA channel 1 interrupt flag clear register
pub mod MDMA_C1IFCR {
    pub use super::MDMA_C0IFCR::CBRTIF;
    pub use super::MDMA_C0IFCR::CBTIF;
    pub use super::MDMA_C0IFCR::CCTCIF;
    pub use super::MDMA_C0IFCR::CLTCIF;
    pub use super::MDMA_C0IFCR::CTEIF;
}

/// MDMA channel 1 error status register
pub mod MDMA_C1ESR {
    pub use super::MDMA_C0ESR::ASE;
    pub use super::MDMA_C0ESR::BSE;
    pub use super::MDMA_C0ESR::TEA;
    pub use super::MDMA_C0ESR::TED;
    pub use super::MDMA_C0ESR::TELD;
    pub use super::MDMA_C0ESR::TEMD;
}

/// This register is used to control the concerned channel.
pub mod MDMA_C1CR {
    pub use super::MDMA_C0CR::BEX;
    pub use super::MDMA_C0CR::BRTIE;
    pub use super::MDMA_C0CR::BTIE;
    pub use super::MDMA_C0CR::CTCIE;
    pub use super::MDMA_C0CR::EN;
    pub use super::MDMA_C0CR::HEX;
    pub use super::MDMA_C0CR::PL;
    pub use super::MDMA_C0CR::SWRQ;
    pub use super::MDMA_C0CR::TCIE;
    pub use super::MDMA_C0CR::TEIE;
    pub use super::MDMA_C0CR::WEX;
}

/// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod MDMA_C1TCR {
    pub use super::MDMA_C0TCR::BWM;
    pub use super::MDMA_C0TCR::DBURST;
    pub use super::MDMA_C0TCR::DINC;
    pub use super::MDMA_C0TCR::DINCOS;
    pub use super::MDMA_C0TCR::DSIZE;
    pub use super::MDMA_C0TCR::PAM;
    pub use super::MDMA_C0TCR::PKE;
    pub use super::MDMA_C0TCR::SBURST;
    pub use super::MDMA_C0TCR::SINC;
    pub use super::MDMA_C0TCR::SINCOS;
    pub use super::MDMA_C0TCR::SSIZE;
    pub use super::MDMA_C0TCR::SWRM;
    pub use super::MDMA_C0TCR::TLEN;
    pub use super::MDMA_C0TCR::TRGM;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod MDMA_C1BNDTR {
    pub use super::MDMA_C0BNDTR::BNDT;
    pub use super::MDMA_C0BNDTR::BRC;
    pub use super::MDMA_C0BNDTR::BRDUM;
    pub use super::MDMA_C0BNDTR::BRSUM;
}

/// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod MDMA_C1SAR {
    pub use super::MDMA_C0SAR::SAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod MDMA_C1DAR {
    pub use super::MDMA_C0DAR::DAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod MDMA_C1BRUR {
    pub use super::MDMA_C0BRUR::DUV;
    pub use super::MDMA_C0BRUR::SUV;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod MDMA_C1LAR {
    pub use super::MDMA_C0LAR::LAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod MDMA_C1TBR {
    pub use super::MDMA_C0TBR::DBUS;
    pub use super::MDMA_C0TBR::SBUS;
    pub use super::MDMA_C0TBR::TSEL;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod MDMA_C1MAR {
    pub use super::MDMA_C0MAR::MAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod MDMA_C1MDR {
    pub use super::MDMA_C0MDR::MDR;
}

/// MDMA channel 2 interrupt/status register
pub mod MDMA_C2ISR {
    pub use super::MDMA_C0ISR::BRTIF;
    pub use super::MDMA_C0ISR::BTIF;
    pub use super::MDMA_C0ISR::CRQA;
    pub use super::MDMA_C0ISR::CTCIF;
    pub use super::MDMA_C0ISR::TCIF;
    pub use super::MDMA_C0ISR::TEIF;
}

/// MDMA channel 2 interrupt flag clear register
pub mod MDMA_C2IFCR {
    pub use super::MDMA_C0IFCR::CBRTIF;
    pub use super::MDMA_C0IFCR::CBTIF;
    pub use super::MDMA_C0IFCR::CCTCIF;
    pub use super::MDMA_C0IFCR::CLTCIF;
    pub use super::MDMA_C0IFCR::CTEIF;
}

/// MDMA channel 2 error status register
pub mod MDMA_C2ESR {
    pub use super::MDMA_C0ESR::ASE;
    pub use super::MDMA_C0ESR::BSE;
    pub use super::MDMA_C0ESR::TEA;
    pub use super::MDMA_C0ESR::TED;
    pub use super::MDMA_C0ESR::TELD;
    pub use super::MDMA_C0ESR::TEMD;
}

/// This register is used to control the concerned channel.
pub mod MDMA_C2CR {
    pub use super::MDMA_C0CR::BEX;
    pub use super::MDMA_C0CR::BRTIE;
    pub use super::MDMA_C0CR::BTIE;
    pub use super::MDMA_C0CR::CTCIE;
    pub use super::MDMA_C0CR::EN;
    pub use super::MDMA_C0CR::HEX;
    pub use super::MDMA_C0CR::PL;
    pub use super::MDMA_C0CR::SWRQ;
    pub use super::MDMA_C0CR::TCIE;
    pub use super::MDMA_C0CR::TEIE;
    pub use super::MDMA_C0CR::WEX;
}

/// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod MDMA_C2TCR {
    pub use super::MDMA_C0TCR::BWM;
    pub use super::MDMA_C0TCR::DBURST;
    pub use super::MDMA_C0TCR::DINC;
    pub use super::MDMA_C0TCR::DINCOS;
    pub use super::MDMA_C0TCR::DSIZE;
    pub use super::MDMA_C0TCR::PAM;
    pub use super::MDMA_C0TCR::PKE;
    pub use super::MDMA_C0TCR::SBURST;
    pub use super::MDMA_C0TCR::SINC;
    pub use super::MDMA_C0TCR::SINCOS;
    pub use super::MDMA_C0TCR::SSIZE;
    pub use super::MDMA_C0TCR::SWRM;
    pub use super::MDMA_C0TCR::TLEN;
    pub use super::MDMA_C0TCR::TRGM;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod MDMA_C2BNDTR {
    pub use super::MDMA_C0BNDTR::BNDT;
    pub use super::MDMA_C0BNDTR::BRC;
    pub use super::MDMA_C0BNDTR::BRDUM;
    pub use super::MDMA_C0BNDTR::BRSUM;
}

/// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod MDMA_C2SAR {
    pub use super::MDMA_C0SAR::SAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod MDMA_C2DAR {
    pub use super::MDMA_C0DAR::DAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod MDMA_C2BRUR {
    pub use super::MDMA_C0BRUR::DUV;
    pub use super::MDMA_C0BRUR::SUV;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod MDMA_C2LAR {
    pub use super::MDMA_C0LAR::LAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod MDMA_C2TBR {
    pub use super::MDMA_C0TBR::DBUS;
    pub use super::MDMA_C0TBR::SBUS;
    pub use super::MDMA_C0TBR::TSEL;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod MDMA_C2MAR {
    pub use super::MDMA_C0MAR::MAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod MDMA_C2MDR {
    pub use super::MDMA_C0MDR::MDR;
}

/// MDMA channel 3 interrupt/status register
pub mod MDMA_C3ISR {
    pub use super::MDMA_C0ISR::BRTIF;
    pub use super::MDMA_C0ISR::BTIF;
    pub use super::MDMA_C0ISR::CRQA;
    pub use super::MDMA_C0ISR::CTCIF;
    pub use super::MDMA_C0ISR::TCIF;
    pub use super::MDMA_C0ISR::TEIF;
}

/// MDMA channel 3 interrupt flag clear register
pub mod MDMA_C3IFCR {
    pub use super::MDMA_C0IFCR::CBRTIF;
    pub use super::MDMA_C0IFCR::CBTIF;
    pub use super::MDMA_C0IFCR::CCTCIF;
    pub use super::MDMA_C0IFCR::CLTCIF;
    pub use super::MDMA_C0IFCR::CTEIF;
}

/// MDMA channel 3 error status register
pub mod MDMA_C3ESR {
    pub use super::MDMA_C0ESR::ASE;
    pub use super::MDMA_C0ESR::BSE;
    pub use super::MDMA_C0ESR::TEA;
    pub use super::MDMA_C0ESR::TED;
    pub use super::MDMA_C0ESR::TELD;
    pub use super::MDMA_C0ESR::TEMD;
}

/// This register is used to control the concerned channel.
pub mod MDMA_C3CR {
    pub use super::MDMA_C0CR::BEX;
    pub use super::MDMA_C0CR::BRTIE;
    pub use super::MDMA_C0CR::BTIE;
    pub use super::MDMA_C0CR::CTCIE;
    pub use super::MDMA_C0CR::EN;
    pub use super::MDMA_C0CR::HEX;
    pub use super::MDMA_C0CR::PL;
    pub use super::MDMA_C0CR::SWRQ;
    pub use super::MDMA_C0CR::TCIE;
    pub use super::MDMA_C0CR::TEIE;
    pub use super::MDMA_C0CR::WEX;
}

/// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod MDMA_C3TCR {
    pub use super::MDMA_C0TCR::BWM;
    pub use super::MDMA_C0TCR::DBURST;
    pub use super::MDMA_C0TCR::DINC;
    pub use super::MDMA_C0TCR::DINCOS;
    pub use super::MDMA_C0TCR::DSIZE;
    pub use super::MDMA_C0TCR::PAM;
    pub use super::MDMA_C0TCR::PKE;
    pub use super::MDMA_C0TCR::SBURST;
    pub use super::MDMA_C0TCR::SINC;
    pub use super::MDMA_C0TCR::SINCOS;
    pub use super::MDMA_C0TCR::SSIZE;
    pub use super::MDMA_C0TCR::SWRM;
    pub use super::MDMA_C0TCR::TLEN;
    pub use super::MDMA_C0TCR::TRGM;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod MDMA_C3BNDTR {
    pub use super::MDMA_C0BNDTR::BNDT;
    pub use super::MDMA_C0BNDTR::BRC;
    pub use super::MDMA_C0BNDTR::BRDUM;
    pub use super::MDMA_C0BNDTR::BRSUM;
}

/// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod MDMA_C3SAR {
    pub use super::MDMA_C0SAR::SAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod MDMA_C3DAR {
    pub use super::MDMA_C0DAR::DAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod MDMA_C3BRUR {
    pub use super::MDMA_C0BRUR::DUV;
    pub use super::MDMA_C0BRUR::SUV;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod MDMA_C3LAR {
    pub use super::MDMA_C0LAR::LAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod MDMA_C3TBR {
    pub use super::MDMA_C0TBR::DBUS;
    pub use super::MDMA_C0TBR::SBUS;
    pub use super::MDMA_C0TBR::TSEL;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod MDMA_C3MAR {
    pub use super::MDMA_C0MAR::MAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod MDMA_C3MDR {
    pub use super::MDMA_C0MDR::MDR;
}

/// MDMA channel 4 interrupt/status register
pub mod MDMA_C4ISR {
    pub use super::MDMA_C0ISR::BRTIF;
    pub use super::MDMA_C0ISR::BTIF;
    pub use super::MDMA_C0ISR::CRQA;
    pub use super::MDMA_C0ISR::CTCIF;
    pub use super::MDMA_C0ISR::TCIF;
    pub use super::MDMA_C0ISR::TEIF;
}

/// MDMA channel 4 interrupt flag clear register
pub mod MDMA_C4IFCR {
    pub use super::MDMA_C0IFCR::CBRTIF;
    pub use super::MDMA_C0IFCR::CBTIF;
    pub use super::MDMA_C0IFCR::CCTCIF;
    pub use super::MDMA_C0IFCR::CLTCIF;
    pub use super::MDMA_C0IFCR::CTEIF;
}

/// MDMA channel 4 error status register
pub mod MDMA_C4ESR {
    pub use super::MDMA_C0ESR::ASE;
    pub use super::MDMA_C0ESR::BSE;
    pub use super::MDMA_C0ESR::TEA;
    pub use super::MDMA_C0ESR::TED;
    pub use super::MDMA_C0ESR::TELD;
    pub use super::MDMA_C0ESR::TEMD;
}

/// This register is used to control the concerned channel.
pub mod MDMA_C4CR {
    pub use super::MDMA_C0CR::BEX;
    pub use super::MDMA_C0CR::BRTIE;
    pub use super::MDMA_C0CR::BTIE;
    pub use super::MDMA_C0CR::CTCIE;
    pub use super::MDMA_C0CR::EN;
    pub use super::MDMA_C0CR::HEX;
    pub use super::MDMA_C0CR::PL;
    pub use super::MDMA_C0CR::SWRQ;
    pub use super::MDMA_C0CR::TCIE;
    pub use super::MDMA_C0CR::TEIE;
    pub use super::MDMA_C0CR::WEX;
}

/// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod MDMA_C4TCR {
    pub use super::MDMA_C0TCR::BWM;
    pub use super::MDMA_C0TCR::DBURST;
    pub use super::MDMA_C0TCR::DINC;
    pub use super::MDMA_C0TCR::DINCOS;
    pub use super::MDMA_C0TCR::DSIZE;
    pub use super::MDMA_C0TCR::PAM;
    pub use super::MDMA_C0TCR::PKE;
    pub use super::MDMA_C0TCR::SBURST;
    pub use super::MDMA_C0TCR::SINC;
    pub use super::MDMA_C0TCR::SINCOS;
    pub use super::MDMA_C0TCR::SSIZE;
    pub use super::MDMA_C0TCR::SWRM;
    pub use super::MDMA_C0TCR::TLEN;
    pub use super::MDMA_C0TCR::TRGM;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod MDMA_C4BNDTR {
    pub use super::MDMA_C0BNDTR::BNDT;
    pub use super::MDMA_C0BNDTR::BRC;
    pub use super::MDMA_C0BNDTR::BRDUM;
    pub use super::MDMA_C0BNDTR::BRSUM;
}

/// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod MDMA_C4SAR {
    pub use super::MDMA_C0SAR::SAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod MDMA_C4DAR {
    pub use super::MDMA_C0DAR::DAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod MDMA_C4BRUR {
    pub use super::MDMA_C0BRUR::DUV;
    pub use super::MDMA_C0BRUR::SUV;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod MDMA_C4LAR {
    pub use super::MDMA_C0LAR::LAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod MDMA_C4TBR {
    pub use super::MDMA_C0TBR::DBUS;
    pub use super::MDMA_C0TBR::SBUS;
    pub use super::MDMA_C0TBR::TSEL;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod MDMA_C4MAR {
    pub use super::MDMA_C0MAR::MAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod MDMA_C4MDR {
    pub use super::MDMA_C0MDR::MDR;
}

/// MDMA channel 5 interrupt/status register
pub mod MDMA_C5ISR {
    pub use super::MDMA_C0ISR::BRTIF;
    pub use super::MDMA_C0ISR::BTIF;
    pub use super::MDMA_C0ISR::CRQA;
    pub use super::MDMA_C0ISR::CTCIF;
    pub use super::MDMA_C0ISR::TCIF;
    pub use super::MDMA_C0ISR::TEIF;
}

/// MDMA channel 5 interrupt flag clear register
pub mod MDMA_C5IFCR {
    pub use super::MDMA_C0IFCR::CBRTIF;
    pub use super::MDMA_C0IFCR::CBTIF;
    pub use super::MDMA_C0IFCR::CCTCIF;
    pub use super::MDMA_C0IFCR::CLTCIF;
    pub use super::MDMA_C0IFCR::CTEIF;
}

/// MDMA channel 5 error status register
pub mod MDMA_C5ESR {
    pub use super::MDMA_C0ESR::ASE;
    pub use super::MDMA_C0ESR::BSE;
    pub use super::MDMA_C0ESR::TEA;
    pub use super::MDMA_C0ESR::TED;
    pub use super::MDMA_C0ESR::TELD;
    pub use super::MDMA_C0ESR::TEMD;
}

/// This register is used to control the concerned channel.
pub mod MDMA_C5CR {
    pub use super::MDMA_C0CR::BEX;
    pub use super::MDMA_C0CR::BRTIE;
    pub use super::MDMA_C0CR::BTIE;
    pub use super::MDMA_C0CR::CTCIE;
    pub use super::MDMA_C0CR::EN;
    pub use super::MDMA_C0CR::HEX;
    pub use super::MDMA_C0CR::PL;
    pub use super::MDMA_C0CR::SWRQ;
    pub use super::MDMA_C0CR::TCIE;
    pub use super::MDMA_C0CR::TEIE;
    pub use super::MDMA_C0CR::WEX;
}

/// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod MDMA_C5TCR {
    pub use super::MDMA_C0TCR::BWM;
    pub use super::MDMA_C0TCR::DBURST;
    pub use super::MDMA_C0TCR::DINC;
    pub use super::MDMA_C0TCR::DINCOS;
    pub use super::MDMA_C0TCR::DSIZE;
    pub use super::MDMA_C0TCR::PAM;
    pub use super::MDMA_C0TCR::PKE;
    pub use super::MDMA_C0TCR::SBURST;
    pub use super::MDMA_C0TCR::SINC;
    pub use super::MDMA_C0TCR::SINCOS;
    pub use super::MDMA_C0TCR::SSIZE;
    pub use super::MDMA_C0TCR::SWRM;
    pub use super::MDMA_C0TCR::TLEN;
    pub use super::MDMA_C0TCR::TRGM;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod MDMA_C5BNDTR {
    pub use super::MDMA_C0BNDTR::BNDT;
    pub use super::MDMA_C0BNDTR::BRC;
    pub use super::MDMA_C0BNDTR::BRDUM;
    pub use super::MDMA_C0BNDTR::BRSUM;
}

/// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod MDMA_C5SAR {
    pub use super::MDMA_C0SAR::SAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod MDMA_C5DAR {
    pub use super::MDMA_C0DAR::DAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod MDMA_C5BRUR {
    pub use super::MDMA_C0BRUR::DUV;
    pub use super::MDMA_C0BRUR::SUV;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod MDMA_C5LAR {
    pub use super::MDMA_C0LAR::LAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod MDMA_C5TBR {
    pub use super::MDMA_C0TBR::DBUS;
    pub use super::MDMA_C0TBR::SBUS;
    pub use super::MDMA_C0TBR::TSEL;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod MDMA_C5MAR {
    pub use super::MDMA_C0MAR::MAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod MDMA_C5MDR {
    pub use super::MDMA_C0MDR::MDR;
}

/// MDMA channel 6 interrupt/status register
pub mod MDMA_C6ISR {
    pub use super::MDMA_C0ISR::BRTIF;
    pub use super::MDMA_C0ISR::BTIF;
    pub use super::MDMA_C0ISR::CRQA;
    pub use super::MDMA_C0ISR::CTCIF;
    pub use super::MDMA_C0ISR::TCIF;
    pub use super::MDMA_C0ISR::TEIF;
}

/// MDMA channel 6 interrupt flag clear register
pub mod MDMA_C6IFCR {
    pub use super::MDMA_C0IFCR::CBRTIF;
    pub use super::MDMA_C0IFCR::CBTIF;
    pub use super::MDMA_C0IFCR::CCTCIF;
    pub use super::MDMA_C0IFCR::CLTCIF;
    pub use super::MDMA_C0IFCR::CTEIF;
}

/// MDMA channel 6 error status register
pub mod MDMA_C6ESR {
    pub use super::MDMA_C0ESR::ASE;
    pub use super::MDMA_C0ESR::BSE;
    pub use super::MDMA_C0ESR::TEA;
    pub use super::MDMA_C0ESR::TED;
    pub use super::MDMA_C0ESR::TELD;
    pub use super::MDMA_C0ESR::TEMD;
}

/// This register is used to control the concerned channel.
pub mod MDMA_C6CR {
    pub use super::MDMA_C0CR::BEX;
    pub use super::MDMA_C0CR::BRTIE;
    pub use super::MDMA_C0CR::BTIE;
    pub use super::MDMA_C0CR::CTCIE;
    pub use super::MDMA_C0CR::EN;
    pub use super::MDMA_C0CR::HEX;
    pub use super::MDMA_C0CR::PL;
    pub use super::MDMA_C0CR::SWRQ;
    pub use super::MDMA_C0CR::TCIE;
    pub use super::MDMA_C0CR::TEIE;
    pub use super::MDMA_C0CR::WEX;
}

/// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod MDMA_C6TCR {
    pub use super::MDMA_C0TCR::BWM;
    pub use super::MDMA_C0TCR::DBURST;
    pub use super::MDMA_C0TCR::DINC;
    pub use super::MDMA_C0TCR::DINCOS;
    pub use super::MDMA_C0TCR::DSIZE;
    pub use super::MDMA_C0TCR::PAM;
    pub use super::MDMA_C0TCR::PKE;
    pub use super::MDMA_C0TCR::SBURST;
    pub use super::MDMA_C0TCR::SINC;
    pub use super::MDMA_C0TCR::SINCOS;
    pub use super::MDMA_C0TCR::SSIZE;
    pub use super::MDMA_C0TCR::SWRM;
    pub use super::MDMA_C0TCR::TLEN;
    pub use super::MDMA_C0TCR::TRGM;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod MDMA_C6BNDTR {
    pub use super::MDMA_C0BNDTR::BNDT;
    pub use super::MDMA_C0BNDTR::BRC;
    pub use super::MDMA_C0BNDTR::BRDUM;
    pub use super::MDMA_C0BNDTR::BRSUM;
}

/// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod MDMA_C6SAR {
    pub use super::MDMA_C0SAR::SAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod MDMA_C6DAR {
    pub use super::MDMA_C0DAR::DAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod MDMA_C6BRUR {
    pub use super::MDMA_C0BRUR::DUV;
    pub use super::MDMA_C0BRUR::SUV;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod MDMA_C6LAR {
    pub use super::MDMA_C0LAR::LAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod MDMA_C6TBR {
    pub use super::MDMA_C0TBR::DBUS;
    pub use super::MDMA_C0TBR::SBUS;
    pub use super::MDMA_C0TBR::TSEL;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod MDMA_C6MAR {
    pub use super::MDMA_C0MAR::MAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod MDMA_C6MDR {
    pub use super::MDMA_C0MDR::MDR;
}

/// MDMA channel 7 interrupt/status register
pub mod MDMA_C7ISR {
    pub use super::MDMA_C0ISR::BRTIF;
    pub use super::MDMA_C0ISR::BTIF;
    pub use super::MDMA_C0ISR::CRQA;
    pub use super::MDMA_C0ISR::CTCIF;
    pub use super::MDMA_C0ISR::TCIF;
    pub use super::MDMA_C0ISR::TEIF;
}

/// MDMA channel 7 interrupt flag clear register
pub mod MDMA_C7IFCR {
    pub use super::MDMA_C0IFCR::CBRTIF;
    pub use super::MDMA_C0IFCR::CBTIF;
    pub use super::MDMA_C0IFCR::CCTCIF;
    pub use super::MDMA_C0IFCR::CLTCIF;
    pub use super::MDMA_C0IFCR::CTEIF;
}

/// MDMA channel 7 error status register
pub mod MDMA_C7ESR {
    pub use super::MDMA_C0ESR::ASE;
    pub use super::MDMA_C0ESR::BSE;
    pub use super::MDMA_C0ESR::TEA;
    pub use super::MDMA_C0ESR::TED;
    pub use super::MDMA_C0ESR::TELD;
    pub use super::MDMA_C0ESR::TEMD;
}

/// This register is used to control the concerned channel.
pub mod MDMA_C7CR {
    pub use super::MDMA_C0CR::BEX;
    pub use super::MDMA_C0CR::BRTIE;
    pub use super::MDMA_C0CR::BTIE;
    pub use super::MDMA_C0CR::CTCIE;
    pub use super::MDMA_C0CR::EN;
    pub use super::MDMA_C0CR::HEX;
    pub use super::MDMA_C0CR::PL;
    pub use super::MDMA_C0CR::SWRQ;
    pub use super::MDMA_C0CR::TCIE;
    pub use super::MDMA_C0CR::TEIE;
    pub use super::MDMA_C0CR::WEX;
}

/// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod MDMA_C7TCR {
    pub use super::MDMA_C0TCR::BWM;
    pub use super::MDMA_C0TCR::DBURST;
    pub use super::MDMA_C0TCR::DINC;
    pub use super::MDMA_C0TCR::DINCOS;
    pub use super::MDMA_C0TCR::DSIZE;
    pub use super::MDMA_C0TCR::PAM;
    pub use super::MDMA_C0TCR::PKE;
    pub use super::MDMA_C0TCR::SBURST;
    pub use super::MDMA_C0TCR::SINC;
    pub use super::MDMA_C0TCR::SINCOS;
    pub use super::MDMA_C0TCR::SSIZE;
    pub use super::MDMA_C0TCR::SWRM;
    pub use super::MDMA_C0TCR::TLEN;
    pub use super::MDMA_C0TCR::TRGM;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod MDMA_C7BNDTR {
    pub use super::MDMA_C0BNDTR::BNDT;
    pub use super::MDMA_C0BNDTR::BRC;
    pub use super::MDMA_C0BNDTR::BRDUM;
    pub use super::MDMA_C0BNDTR::BRSUM;
}

/// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod MDMA_C7SAR {
    pub use super::MDMA_C0SAR::SAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod MDMA_C7DAR {
    pub use super::MDMA_C0DAR::DAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod MDMA_C7BRUR {
    pub use super::MDMA_C0BRUR::DUV;
    pub use super::MDMA_C0BRUR::SUV;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod MDMA_C7LAR {
    pub use super::MDMA_C0LAR::LAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod MDMA_C7TBR {
    pub use super::MDMA_C0TBR::DBUS;
    pub use super::MDMA_C0TBR::SBUS;
    pub use super::MDMA_C0TBR::TSEL;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod MDMA_C7MAR {
    pub use super::MDMA_C0MAR::MAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod MDMA_C7MDR {
    pub use super::MDMA_C0MDR::MDR;
}

/// MDMA channel 8 interrupt/status register
pub mod MDMA_C8ISR {
    pub use super::MDMA_C0ISR::BRTIF;
    pub use super::MDMA_C0ISR::BTIF;
    pub use super::MDMA_C0ISR::CRQA;
    pub use super::MDMA_C0ISR::CTCIF;
    pub use super::MDMA_C0ISR::TCIF;
    pub use super::MDMA_C0ISR::TEIF;
}

/// MDMA channel 8 interrupt flag clear register
pub mod MDMA_C8IFCR {
    pub use super::MDMA_C0IFCR::CBRTIF;
    pub use super::MDMA_C0IFCR::CBTIF;
    pub use super::MDMA_C0IFCR::CCTCIF;
    pub use super::MDMA_C0IFCR::CLTCIF;
    pub use super::MDMA_C0IFCR::CTEIF;
}

/// MDMA channel 8 error status register
pub mod MDMA_C8ESR {
    pub use super::MDMA_C0ESR::ASE;
    pub use super::MDMA_C0ESR::BSE;
    pub use super::MDMA_C0ESR::TEA;
    pub use super::MDMA_C0ESR::TED;
    pub use super::MDMA_C0ESR::TELD;
    pub use super::MDMA_C0ESR::TEMD;
}

/// This register is used to control the concerned channel.
pub mod MDMA_C8CR {
    pub use super::MDMA_C0CR::BEX;
    pub use super::MDMA_C0CR::BRTIE;
    pub use super::MDMA_C0CR::BTIE;
    pub use super::MDMA_C0CR::CTCIE;
    pub use super::MDMA_C0CR::EN;
    pub use super::MDMA_C0CR::HEX;
    pub use super::MDMA_C0CR::PL;
    pub use super::MDMA_C0CR::SWRQ;
    pub use super::MDMA_C0CR::TCIE;
    pub use super::MDMA_C0CR::TEIE;
    pub use super::MDMA_C0CR::WEX;
}

/// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod MDMA_C8TCR {
    pub use super::MDMA_C0TCR::BWM;
    pub use super::MDMA_C0TCR::DBURST;
    pub use super::MDMA_C0TCR::DINC;
    pub use super::MDMA_C0TCR::DINCOS;
    pub use super::MDMA_C0TCR::DSIZE;
    pub use super::MDMA_C0TCR::PAM;
    pub use super::MDMA_C0TCR::PKE;
    pub use super::MDMA_C0TCR::SBURST;
    pub use super::MDMA_C0TCR::SINC;
    pub use super::MDMA_C0TCR::SINCOS;
    pub use super::MDMA_C0TCR::SSIZE;
    pub use super::MDMA_C0TCR::SWRM;
    pub use super::MDMA_C0TCR::TLEN;
    pub use super::MDMA_C0TCR::TRGM;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod MDMA_C8BNDTR {
    pub use super::MDMA_C0BNDTR::BNDT;
    pub use super::MDMA_C0BNDTR::BRC;
    pub use super::MDMA_C0BNDTR::BRDUM;
    pub use super::MDMA_C0BNDTR::BRSUM;
}

/// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod MDMA_C8SAR {
    pub use super::MDMA_C0SAR::SAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod MDMA_C8DAR {
    pub use super::MDMA_C0DAR::DAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod MDMA_C8BRUR {
    pub use super::MDMA_C0BRUR::DUV;
    pub use super::MDMA_C0BRUR::SUV;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod MDMA_C8LAR {
    pub use super::MDMA_C0LAR::LAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod MDMA_C8TBR {
    pub use super::MDMA_C0TBR::DBUS;
    pub use super::MDMA_C0TBR::SBUS;
    pub use super::MDMA_C0TBR::TSEL;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod MDMA_C8MAR {
    pub use super::MDMA_C0MAR::MAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod MDMA_C8MDR {
    pub use super::MDMA_C0MDR::MDR;
}

/// MDMA channel 9 interrupt/status register
pub mod MDMA_C9ISR {
    pub use super::MDMA_C0ISR::BRTIF;
    pub use super::MDMA_C0ISR::BTIF;
    pub use super::MDMA_C0ISR::CRQA;
    pub use super::MDMA_C0ISR::CTCIF;
    pub use super::MDMA_C0ISR::TCIF;
    pub use super::MDMA_C0ISR::TEIF;
}

/// MDMA channel 9 interrupt flag clear register
pub mod MDMA_C9IFCR {
    pub use super::MDMA_C0IFCR::CBRTIF;
    pub use super::MDMA_C0IFCR::CBTIF;
    pub use super::MDMA_C0IFCR::CCTCIF;
    pub use super::MDMA_C0IFCR::CLTCIF;
    pub use super::MDMA_C0IFCR::CTEIF;
}

/// MDMA channel 9 error status register
pub mod MDMA_C9ESR {
    pub use super::MDMA_C0ESR::ASE;
    pub use super::MDMA_C0ESR::BSE;
    pub use super::MDMA_C0ESR::TEA;
    pub use super::MDMA_C0ESR::TED;
    pub use super::MDMA_C0ESR::TELD;
    pub use super::MDMA_C0ESR::TEMD;
}

/// This register is used to control the concerned channel.
pub mod MDMA_C9CR {
    pub use super::MDMA_C0CR::BEX;
    pub use super::MDMA_C0CR::BRTIE;
    pub use super::MDMA_C0CR::BTIE;
    pub use super::MDMA_C0CR::CTCIE;
    pub use super::MDMA_C0CR::EN;
    pub use super::MDMA_C0CR::HEX;
    pub use super::MDMA_C0CR::PL;
    pub use super::MDMA_C0CR::SWRQ;
    pub use super::MDMA_C0CR::TCIE;
    pub use super::MDMA_C0CR::TEIE;
    pub use super::MDMA_C0CR::WEX;
}

/// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod MDMA_C9TCR {
    pub use super::MDMA_C0TCR::BWM;
    pub use super::MDMA_C0TCR::DBURST;
    pub use super::MDMA_C0TCR::DINC;
    pub use super::MDMA_C0TCR::DINCOS;
    pub use super::MDMA_C0TCR::DSIZE;
    pub use super::MDMA_C0TCR::PAM;
    pub use super::MDMA_C0TCR::PKE;
    pub use super::MDMA_C0TCR::SBURST;
    pub use super::MDMA_C0TCR::SINC;
    pub use super::MDMA_C0TCR::SINCOS;
    pub use super::MDMA_C0TCR::SSIZE;
    pub use super::MDMA_C0TCR::SWRM;
    pub use super::MDMA_C0TCR::TLEN;
    pub use super::MDMA_C0TCR::TRGM;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod MDMA_C9BNDTR {
    pub use super::MDMA_C0BNDTR::BNDT;
    pub use super::MDMA_C0BNDTR::BRC;
    pub use super::MDMA_C0BNDTR::BRDUM;
    pub use super::MDMA_C0BNDTR::BRSUM;
}

/// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod MDMA_C9SAR {
    pub use super::MDMA_C0SAR::SAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod MDMA_C9DAR {
    pub use super::MDMA_C0DAR::DAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod MDMA_C9BRUR {
    pub use super::MDMA_C0BRUR::DUV;
    pub use super::MDMA_C0BRUR::SUV;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod MDMA_C9LAR {
    pub use super::MDMA_C0LAR::LAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod MDMA_C9TBR {
    pub use super::MDMA_C0TBR::DBUS;
    pub use super::MDMA_C0TBR::SBUS;
    pub use super::MDMA_C0TBR::TSEL;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod MDMA_C9MAR {
    pub use super::MDMA_C0MAR::MAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod MDMA_C9MDR {
    pub use super::MDMA_C0MDR::MDR;
}

/// MDMA channel 10 interrupt/status register
pub mod MDMA_C10ISR {
    pub use super::MDMA_C0ISR::BRTIF;
    pub use super::MDMA_C0ISR::BTIF;
    pub use super::MDMA_C0ISR::CRQA;
    pub use super::MDMA_C0ISR::CTCIF;
    pub use super::MDMA_C0ISR::TCIF;
    pub use super::MDMA_C0ISR::TEIF;
}

/// MDMA channel 10 interrupt flag clear register
pub mod MDMA_C10IFCR {
    pub use super::MDMA_C0IFCR::CBRTIF;
    pub use super::MDMA_C0IFCR::CBTIF;
    pub use super::MDMA_C0IFCR::CCTCIF;
    pub use super::MDMA_C0IFCR::CLTCIF;
    pub use super::MDMA_C0IFCR::CTEIF;
}

/// MDMA channel 10 error status register
pub mod MDMA_C10ESR {
    pub use super::MDMA_C0ESR::ASE;
    pub use super::MDMA_C0ESR::BSE;
    pub use super::MDMA_C0ESR::TEA;
    pub use super::MDMA_C0ESR::TED;
    pub use super::MDMA_C0ESR::TELD;
    pub use super::MDMA_C0ESR::TEMD;
}

/// This register is used to control the concerned channel.
pub mod MDMA_C10CR {
    pub use super::MDMA_C0CR::BEX;
    pub use super::MDMA_C0CR::BRTIE;
    pub use super::MDMA_C0CR::BTIE;
    pub use super::MDMA_C0CR::CTCIE;
    pub use super::MDMA_C0CR::EN;
    pub use super::MDMA_C0CR::HEX;
    pub use super::MDMA_C0CR::PL;
    pub use super::MDMA_C0CR::SWRQ;
    pub use super::MDMA_C0CR::TCIE;
    pub use super::MDMA_C0CR::TEIE;
    pub use super::MDMA_C0CR::WEX;
}

/// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod MDMA_C10TCR {
    pub use super::MDMA_C0TCR::BWM;
    pub use super::MDMA_C0TCR::DBURST;
    pub use super::MDMA_C0TCR::DINC;
    pub use super::MDMA_C0TCR::DINCOS;
    pub use super::MDMA_C0TCR::DSIZE;
    pub use super::MDMA_C0TCR::PAM;
    pub use super::MDMA_C0TCR::PKE;
    pub use super::MDMA_C0TCR::SBURST;
    pub use super::MDMA_C0TCR::SINC;
    pub use super::MDMA_C0TCR::SINCOS;
    pub use super::MDMA_C0TCR::SSIZE;
    pub use super::MDMA_C0TCR::SWRM;
    pub use super::MDMA_C0TCR::TLEN;
    pub use super::MDMA_C0TCR::TRGM;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod MDMA_C10BNDTR {
    pub use super::MDMA_C0BNDTR::BNDT;
    pub use super::MDMA_C0BNDTR::BRC;
    pub use super::MDMA_C0BNDTR::BRDUM;
    pub use super::MDMA_C0BNDTR::BRSUM;
}

/// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod MDMA_C10SAR {
    pub use super::MDMA_C0SAR::SAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod MDMA_C10DAR {
    pub use super::MDMA_C0DAR::DAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod MDMA_C10BRUR {
    pub use super::MDMA_C0BRUR::DUV;
    pub use super::MDMA_C0BRUR::SUV;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod MDMA_C10LAR {
    pub use super::MDMA_C0LAR::LAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod MDMA_C10TBR {
    pub use super::MDMA_C0TBR::DBUS;
    pub use super::MDMA_C0TBR::SBUS;
    pub use super::MDMA_C0TBR::TSEL;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod MDMA_C10MAR {
    pub use super::MDMA_C0MAR::MAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod MDMA_C10MDR {
    pub use super::MDMA_C0MDR::MDR;
}

/// MDMA channel 11 interrupt/status register
pub mod MDMA_C11ISR {
    pub use super::MDMA_C0ISR::BRTIF;
    pub use super::MDMA_C0ISR::BTIF;
    pub use super::MDMA_C0ISR::CRQA;
    pub use super::MDMA_C0ISR::CTCIF;
    pub use super::MDMA_C0ISR::TCIF;
    pub use super::MDMA_C0ISR::TEIF;
}

/// MDMA channel 11 interrupt flag clear register
pub mod MDMA_C11IFCR {
    pub use super::MDMA_C0IFCR::CBRTIF;
    pub use super::MDMA_C0IFCR::CBTIF;
    pub use super::MDMA_C0IFCR::CCTCIF;
    pub use super::MDMA_C0IFCR::CLTCIF;
    pub use super::MDMA_C0IFCR::CTEIF;
}

/// MDMA channel 11 error status register
pub mod MDMA_C11ESR {
    pub use super::MDMA_C0ESR::ASE;
    pub use super::MDMA_C0ESR::BSE;
    pub use super::MDMA_C0ESR::TEA;
    pub use super::MDMA_C0ESR::TED;
    pub use super::MDMA_C0ESR::TELD;
    pub use super::MDMA_C0ESR::TEMD;
}

/// This register is used to control the concerned channel.
pub mod MDMA_C11CR {
    pub use super::MDMA_C0CR::BEX;
    pub use super::MDMA_C0CR::BRTIE;
    pub use super::MDMA_C0CR::BTIE;
    pub use super::MDMA_C0CR::CTCIE;
    pub use super::MDMA_C0CR::EN;
    pub use super::MDMA_C0CR::HEX;
    pub use super::MDMA_C0CR::PL;
    pub use super::MDMA_C0CR::SWRQ;
    pub use super::MDMA_C0CR::TCIE;
    pub use super::MDMA_C0CR::TEIE;
    pub use super::MDMA_C0CR::WEX;
}

/// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod MDMA_C11TCR {
    pub use super::MDMA_C0TCR::BWM;
    pub use super::MDMA_C0TCR::DBURST;
    pub use super::MDMA_C0TCR::DINC;
    pub use super::MDMA_C0TCR::DINCOS;
    pub use super::MDMA_C0TCR::DSIZE;
    pub use super::MDMA_C0TCR::PAM;
    pub use super::MDMA_C0TCR::PKE;
    pub use super::MDMA_C0TCR::SBURST;
    pub use super::MDMA_C0TCR::SINC;
    pub use super::MDMA_C0TCR::SINCOS;
    pub use super::MDMA_C0TCR::SSIZE;
    pub use super::MDMA_C0TCR::SWRM;
    pub use super::MDMA_C0TCR::TLEN;
    pub use super::MDMA_C0TCR::TRGM;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod MDMA_C11BNDTR {
    pub use super::MDMA_C0BNDTR::BNDT;
    pub use super::MDMA_C0BNDTR::BRC;
    pub use super::MDMA_C0BNDTR::BRDUM;
    pub use super::MDMA_C0BNDTR::BRSUM;
}

/// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod MDMA_C11SAR {
    pub use super::MDMA_C0SAR::SAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod MDMA_C11DAR {
    pub use super::MDMA_C0DAR::DAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod MDMA_C11BRUR {
    pub use super::MDMA_C0BRUR::DUV;
    pub use super::MDMA_C0BRUR::SUV;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod MDMA_C11LAR {
    pub use super::MDMA_C0LAR::LAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod MDMA_C11TBR {
    pub use super::MDMA_C0TBR::DBUS;
    pub use super::MDMA_C0TBR::SBUS;
    pub use super::MDMA_C0TBR::TSEL;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod MDMA_C11MAR {
    pub use super::MDMA_C0MAR::MAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod MDMA_C11MDR {
    pub use super::MDMA_C0MDR::MDR;
}

/// MDMA channel 12 interrupt/status register
pub mod MDMA_C12ISR {
    pub use super::MDMA_C0ISR::BRTIF;
    pub use super::MDMA_C0ISR::BTIF;
    pub use super::MDMA_C0ISR::CRQA;
    pub use super::MDMA_C0ISR::CTCIF;
    pub use super::MDMA_C0ISR::TCIF;
    pub use super::MDMA_C0ISR::TEIF;
}

/// MDMA channel 12 interrupt flag clear register
pub mod MDMA_C12IFCR {
    pub use super::MDMA_C0IFCR::CBRTIF;
    pub use super::MDMA_C0IFCR::CBTIF;
    pub use super::MDMA_C0IFCR::CCTCIF;
    pub use super::MDMA_C0IFCR::CLTCIF;
    pub use super::MDMA_C0IFCR::CTEIF;
}

/// MDMA channel 12 error status register
pub mod MDMA_C12ESR {
    pub use super::MDMA_C0ESR::ASE;
    pub use super::MDMA_C0ESR::BSE;
    pub use super::MDMA_C0ESR::TEA;
    pub use super::MDMA_C0ESR::TED;
    pub use super::MDMA_C0ESR::TELD;
    pub use super::MDMA_C0ESR::TEMD;
}

/// This register is used to control the concerned channel.
pub mod MDMA_C12CR {
    pub use super::MDMA_C0CR::BEX;
    pub use super::MDMA_C0CR::BRTIE;
    pub use super::MDMA_C0CR::BTIE;
    pub use super::MDMA_C0CR::CTCIE;
    pub use super::MDMA_C0CR::EN;
    pub use super::MDMA_C0CR::HEX;
    pub use super::MDMA_C0CR::PL;
    pub use super::MDMA_C0CR::SWRQ;
    pub use super::MDMA_C0CR::TCIE;
    pub use super::MDMA_C0CR::TEIE;
    pub use super::MDMA_C0CR::WEX;
}

/// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod MDMA_C12TCR {
    pub use super::MDMA_C0TCR::BWM;
    pub use super::MDMA_C0TCR::DBURST;
    pub use super::MDMA_C0TCR::DINC;
    pub use super::MDMA_C0TCR::DINCOS;
    pub use super::MDMA_C0TCR::DSIZE;
    pub use super::MDMA_C0TCR::PAM;
    pub use super::MDMA_C0TCR::PKE;
    pub use super::MDMA_C0TCR::SBURST;
    pub use super::MDMA_C0TCR::SINC;
    pub use super::MDMA_C0TCR::SINCOS;
    pub use super::MDMA_C0TCR::SSIZE;
    pub use super::MDMA_C0TCR::SWRM;
    pub use super::MDMA_C0TCR::TLEN;
    pub use super::MDMA_C0TCR::TRGM;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod MDMA_C12BNDTR {
    pub use super::MDMA_C0BNDTR::BNDT;
    pub use super::MDMA_C0BNDTR::BRC;
    pub use super::MDMA_C0BNDTR::BRDUM;
    pub use super::MDMA_C0BNDTR::BRSUM;
}

/// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod MDMA_C12SAR {
    pub use super::MDMA_C0SAR::SAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod MDMA_C12DAR {
    pub use super::MDMA_C0DAR::DAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod MDMA_C12BRUR {
    pub use super::MDMA_C0BRUR::DUV;
    pub use super::MDMA_C0BRUR::SUV;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod MDMA_C12LAR {
    pub use super::MDMA_C0LAR::LAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod MDMA_C12TBR {
    pub use super::MDMA_C0TBR::DBUS;
    pub use super::MDMA_C0TBR::SBUS;
    pub use super::MDMA_C0TBR::TSEL;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod MDMA_C12MAR {
    pub use super::MDMA_C0MAR::MAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod MDMA_C12MDR {
    pub use super::MDMA_C0MDR::MDR;
}

/// MDMA channel 13 interrupt/status register
pub mod MDMA_C13ISR {
    pub use super::MDMA_C0ISR::BRTIF;
    pub use super::MDMA_C0ISR::BTIF;
    pub use super::MDMA_C0ISR::CRQA;
    pub use super::MDMA_C0ISR::CTCIF;
    pub use super::MDMA_C0ISR::TCIF;
    pub use super::MDMA_C0ISR::TEIF;
}

/// MDMA channel 13 interrupt flag clear register
pub mod MDMA_C13IFCR {
    pub use super::MDMA_C0IFCR::CBRTIF;
    pub use super::MDMA_C0IFCR::CBTIF;
    pub use super::MDMA_C0IFCR::CCTCIF;
    pub use super::MDMA_C0IFCR::CLTCIF;
    pub use super::MDMA_C0IFCR::CTEIF;
}

/// MDMA channel 13 error status register
pub mod MDMA_C13ESR {
    pub use super::MDMA_C0ESR::ASE;
    pub use super::MDMA_C0ESR::BSE;
    pub use super::MDMA_C0ESR::TEA;
    pub use super::MDMA_C0ESR::TED;
    pub use super::MDMA_C0ESR::TELD;
    pub use super::MDMA_C0ESR::TEMD;
}

/// This register is used to control the concerned channel.
pub mod MDMA_C13CR {
    pub use super::MDMA_C0CR::BEX;
    pub use super::MDMA_C0CR::BRTIE;
    pub use super::MDMA_C0CR::BTIE;
    pub use super::MDMA_C0CR::CTCIE;
    pub use super::MDMA_C0CR::EN;
    pub use super::MDMA_C0CR::HEX;
    pub use super::MDMA_C0CR::PL;
    pub use super::MDMA_C0CR::SWRQ;
    pub use super::MDMA_C0CR::TCIE;
    pub use super::MDMA_C0CR::TEIE;
    pub use super::MDMA_C0CR::WEX;
}

/// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod MDMA_C13TCR {
    pub use super::MDMA_C0TCR::BWM;
    pub use super::MDMA_C0TCR::DBURST;
    pub use super::MDMA_C0TCR::DINC;
    pub use super::MDMA_C0TCR::DINCOS;
    pub use super::MDMA_C0TCR::DSIZE;
    pub use super::MDMA_C0TCR::PAM;
    pub use super::MDMA_C0TCR::PKE;
    pub use super::MDMA_C0TCR::SBURST;
    pub use super::MDMA_C0TCR::SINC;
    pub use super::MDMA_C0TCR::SINCOS;
    pub use super::MDMA_C0TCR::SSIZE;
    pub use super::MDMA_C0TCR::SWRM;
    pub use super::MDMA_C0TCR::TLEN;
    pub use super::MDMA_C0TCR::TRGM;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod MDMA_C13BNDTR {
    pub use super::MDMA_C0BNDTR::BNDT;
    pub use super::MDMA_C0BNDTR::BRC;
    pub use super::MDMA_C0BNDTR::BRDUM;
    pub use super::MDMA_C0BNDTR::BRSUM;
}

/// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod MDMA_C13SAR {
    pub use super::MDMA_C0SAR::SAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod MDMA_C13DAR {
    pub use super::MDMA_C0DAR::DAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod MDMA_C13BRUR {
    pub use super::MDMA_C0BRUR::DUV;
    pub use super::MDMA_C0BRUR::SUV;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod MDMA_C13LAR {
    pub use super::MDMA_C0LAR::LAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod MDMA_C13TBR {
    pub use super::MDMA_C0TBR::DBUS;
    pub use super::MDMA_C0TBR::SBUS;
    pub use super::MDMA_C0TBR::TSEL;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod MDMA_C13MAR {
    pub use super::MDMA_C0MAR::MAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod MDMA_C13MDR {
    pub use super::MDMA_C0MDR::MDR;
}

/// MDMA channel 14 interrupt/status register
pub mod MDMA_C14ISR {
    pub use super::MDMA_C0ISR::BRTIF;
    pub use super::MDMA_C0ISR::BTIF;
    pub use super::MDMA_C0ISR::CRQA;
    pub use super::MDMA_C0ISR::CTCIF;
    pub use super::MDMA_C0ISR::TCIF;
    pub use super::MDMA_C0ISR::TEIF;
}

/// MDMA channel 14 interrupt flag clear register
pub mod MDMA_C14IFCR {
    pub use super::MDMA_C0IFCR::CBRTIF;
    pub use super::MDMA_C0IFCR::CBTIF;
    pub use super::MDMA_C0IFCR::CCTCIF;
    pub use super::MDMA_C0IFCR::CLTCIF;
    pub use super::MDMA_C0IFCR::CTEIF;
}

/// MDMA channel 14 error status register
pub mod MDMA_C14ESR {
    pub use super::MDMA_C0ESR::ASE;
    pub use super::MDMA_C0ESR::BSE;
    pub use super::MDMA_C0ESR::TEA;
    pub use super::MDMA_C0ESR::TED;
    pub use super::MDMA_C0ESR::TELD;
    pub use super::MDMA_C0ESR::TEMD;
}

/// This register is used to control the concerned channel.
pub mod MDMA_C14CR {
    pub use super::MDMA_C0CR::BEX;
    pub use super::MDMA_C0CR::BRTIE;
    pub use super::MDMA_C0CR::BTIE;
    pub use super::MDMA_C0CR::CTCIE;
    pub use super::MDMA_C0CR::EN;
    pub use super::MDMA_C0CR::HEX;
    pub use super::MDMA_C0CR::PL;
    pub use super::MDMA_C0CR::SWRQ;
    pub use super::MDMA_C0CR::TCIE;
    pub use super::MDMA_C0CR::TEIE;
    pub use super::MDMA_C0CR::WEX;
}

/// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod MDMA_C14TCR {
    pub use super::MDMA_C0TCR::BWM;
    pub use super::MDMA_C0TCR::DBURST;
    pub use super::MDMA_C0TCR::DINC;
    pub use super::MDMA_C0TCR::DINCOS;
    pub use super::MDMA_C0TCR::DSIZE;
    pub use super::MDMA_C0TCR::PAM;
    pub use super::MDMA_C0TCR::PKE;
    pub use super::MDMA_C0TCR::SBURST;
    pub use super::MDMA_C0TCR::SINC;
    pub use super::MDMA_C0TCR::SINCOS;
    pub use super::MDMA_C0TCR::SSIZE;
    pub use super::MDMA_C0TCR::SWRM;
    pub use super::MDMA_C0TCR::TLEN;
    pub use super::MDMA_C0TCR::TRGM;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod MDMA_C14BNDTR {
    pub use super::MDMA_C0BNDTR::BNDT;
    pub use super::MDMA_C0BNDTR::BRC;
    pub use super::MDMA_C0BNDTR::BRDUM;
    pub use super::MDMA_C0BNDTR::BRSUM;
}

/// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod MDMA_C14SAR {
    pub use super::MDMA_C0SAR::SAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod MDMA_C14DAR {
    pub use super::MDMA_C0DAR::DAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod MDMA_C14BRUR {
    pub use super::MDMA_C0BRUR::DUV;
    pub use super::MDMA_C0BRUR::SUV;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod MDMA_C14LAR {
    pub use super::MDMA_C0LAR::LAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod MDMA_C14TBR {
    pub use super::MDMA_C0TBR::DBUS;
    pub use super::MDMA_C0TBR::SBUS;
    pub use super::MDMA_C0TBR::TSEL;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod MDMA_C14MAR {
    pub use super::MDMA_C0MAR::MAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod MDMA_C14MDR {
    pub use super::MDMA_C0MDR::MDR;
}

/// MDMA channel 15 interrupt/status register
pub mod MDMA_C15ISR {
    pub use super::MDMA_C0ISR::BRTIF;
    pub use super::MDMA_C0ISR::BTIF;
    pub use super::MDMA_C0ISR::CRQA;
    pub use super::MDMA_C0ISR::CTCIF;
    pub use super::MDMA_C0ISR::TCIF;
    pub use super::MDMA_C0ISR::TEIF;
}

/// MDMA channel 15 interrupt flag clear register
pub mod MDMA_C15IFCR {
    pub use super::MDMA_C0IFCR::CBRTIF;
    pub use super::MDMA_C0IFCR::CBTIF;
    pub use super::MDMA_C0IFCR::CCTCIF;
    pub use super::MDMA_C0IFCR::CLTCIF;
    pub use super::MDMA_C0IFCR::CTEIF;
}

/// MDMA channel 15 error status register
pub mod MDMA_C15ESR {
    pub use super::MDMA_C0ESR::ASE;
    pub use super::MDMA_C0ESR::BSE;
    pub use super::MDMA_C0ESR::TEA;
    pub use super::MDMA_C0ESR::TED;
    pub use super::MDMA_C0ESR::TELD;
    pub use super::MDMA_C0ESR::TEMD;
}

/// This register is used to control the concerned channel.
pub mod MDMA_C15CR {
    pub use super::MDMA_C0CR::BEX;
    pub use super::MDMA_C0CR::BRTIE;
    pub use super::MDMA_C0CR::BTIE;
    pub use super::MDMA_C0CR::CTCIE;
    pub use super::MDMA_C0CR::EN;
    pub use super::MDMA_C0CR::HEX;
    pub use super::MDMA_C0CR::PL;
    pub use super::MDMA_C0CR::SWRQ;
    pub use super::MDMA_C0CR::TCIE;
    pub use super::MDMA_C0CR::TEIE;
    pub use super::MDMA_C0CR::WEX;
}

/// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod MDMA_C15TCR {
    pub use super::MDMA_C0TCR::BWM;
    pub use super::MDMA_C0TCR::DBURST;
    pub use super::MDMA_C0TCR::DINC;
    pub use super::MDMA_C0TCR::DINCOS;
    pub use super::MDMA_C0TCR::DSIZE;
    pub use super::MDMA_C0TCR::PAM;
    pub use super::MDMA_C0TCR::PKE;
    pub use super::MDMA_C0TCR::SBURST;
    pub use super::MDMA_C0TCR::SINC;
    pub use super::MDMA_C0TCR::SINCOS;
    pub use super::MDMA_C0TCR::SSIZE;
    pub use super::MDMA_C0TCR::SWRM;
    pub use super::MDMA_C0TCR::TLEN;
    pub use super::MDMA_C0TCR::TRGM;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod MDMA_C15BNDTR {
    pub use super::MDMA_C0BNDTR::BNDT;
    pub use super::MDMA_C0BNDTR::BRC;
    pub use super::MDMA_C0BNDTR::BRDUM;
    pub use super::MDMA_C0BNDTR::BRSUM;
}

/// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod MDMA_C15SAR {
    pub use super::MDMA_C0SAR::SAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod MDMA_C15DAR {
    pub use super::MDMA_C0DAR::DAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod MDMA_C15BRUR {
    pub use super::MDMA_C0BRUR::DUV;
    pub use super::MDMA_C0BRUR::SUV;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod MDMA_C15LAR {
    pub use super::MDMA_C0LAR::LAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod MDMA_C15TBR {
    pub use super::MDMA_C0TBR::DBUS;
    pub use super::MDMA_C0TBR::SBUS;
    pub use super::MDMA_C0TBR::TSEL;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod MDMA_C15MAR {
    pub use super::MDMA_C0MAR::MAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod MDMA_C15MDR {
    pub use super::MDMA_C0MDR::MDR;
}

/// MDMA channel 16 interrupt/status register
pub mod MDMA_C16ISR {
    pub use super::MDMA_C0ISR::BRTIF;
    pub use super::MDMA_C0ISR::BTIF;
    pub use super::MDMA_C0ISR::CRQA;
    pub use super::MDMA_C0ISR::CTCIF;
    pub use super::MDMA_C0ISR::TCIF;
    pub use super::MDMA_C0ISR::TEIF;
}

/// MDMA channel 16 interrupt flag clear register
pub mod MDMA_C16IFCR {
    pub use super::MDMA_C0IFCR::CBRTIF;
    pub use super::MDMA_C0IFCR::CBTIF;
    pub use super::MDMA_C0IFCR::CCTCIF;
    pub use super::MDMA_C0IFCR::CLTCIF;
    pub use super::MDMA_C0IFCR::CTEIF;
}

/// MDMA channel 16 error status register
pub mod MDMA_C16ESR {
    pub use super::MDMA_C0ESR::ASE;
    pub use super::MDMA_C0ESR::BSE;
    pub use super::MDMA_C0ESR::TEA;
    pub use super::MDMA_C0ESR::TED;
    pub use super::MDMA_C0ESR::TELD;
    pub use super::MDMA_C0ESR::TEMD;
}

/// This register is used to control the concerned channel.
pub mod MDMA_C16CR {
    pub use super::MDMA_C0CR::BEX;
    pub use super::MDMA_C0CR::BRTIE;
    pub use super::MDMA_C0CR::BTIE;
    pub use super::MDMA_C0CR::CTCIE;
    pub use super::MDMA_C0CR::EN;
    pub use super::MDMA_C0CR::HEX;
    pub use super::MDMA_C0CR::PL;
    pub use super::MDMA_C0CR::SWRQ;
    pub use super::MDMA_C0CR::TCIE;
    pub use super::MDMA_C0CR::TEIE;
    pub use super::MDMA_C0CR::WEX;
}

/// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod MDMA_C16TCR {
    pub use super::MDMA_C0TCR::BWM;
    pub use super::MDMA_C0TCR::DBURST;
    pub use super::MDMA_C0TCR::DINC;
    pub use super::MDMA_C0TCR::DINCOS;
    pub use super::MDMA_C0TCR::DSIZE;
    pub use super::MDMA_C0TCR::PAM;
    pub use super::MDMA_C0TCR::PKE;
    pub use super::MDMA_C0TCR::SBURST;
    pub use super::MDMA_C0TCR::SINC;
    pub use super::MDMA_C0TCR::SINCOS;
    pub use super::MDMA_C0TCR::SSIZE;
    pub use super::MDMA_C0TCR::SWRM;
    pub use super::MDMA_C0TCR::TLEN;
    pub use super::MDMA_C0TCR::TRGM;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod MDMA_C16BNDTR {
    pub use super::MDMA_C0BNDTR::BNDT;
    pub use super::MDMA_C0BNDTR::BRC;
    pub use super::MDMA_C0BNDTR::BRDUM;
    pub use super::MDMA_C0BNDTR::BRSUM;
}

/// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod MDMA_C16SAR {
    pub use super::MDMA_C0SAR::SAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod MDMA_C16DAR {
    pub use super::MDMA_C0DAR::DAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod MDMA_C16BRUR {
    pub use super::MDMA_C0BRUR::DUV;
    pub use super::MDMA_C0BRUR::SUV;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod MDMA_C16LAR {
    pub use super::MDMA_C0LAR::LAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod MDMA_C16TBR {
    pub use super::MDMA_C0TBR::DBUS;
    pub use super::MDMA_C0TBR::SBUS;
    pub use super::MDMA_C0TBR::TSEL;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod MDMA_C16MAR {
    pub use super::MDMA_C0MAR::MAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod MDMA_C16MDR {
    pub use super::MDMA_C0MDR::MDR;
}

/// MDMA channel 17 interrupt/status register
pub mod MDMA_C17ISR {
    pub use super::MDMA_C0ISR::BRTIF;
    pub use super::MDMA_C0ISR::BTIF;
    pub use super::MDMA_C0ISR::CRQA;
    pub use super::MDMA_C0ISR::CTCIF;
    pub use super::MDMA_C0ISR::TCIF;
    pub use super::MDMA_C0ISR::TEIF;
}

/// MDMA channel 17 interrupt flag clear register
pub mod MDMA_C17IFCR {
    pub use super::MDMA_C0IFCR::CBRTIF;
    pub use super::MDMA_C0IFCR::CBTIF;
    pub use super::MDMA_C0IFCR::CCTCIF;
    pub use super::MDMA_C0IFCR::CLTCIF;
    pub use super::MDMA_C0IFCR::CTEIF;
}

/// MDMA channel 17 error status register
pub mod MDMA_C17ESR {
    pub use super::MDMA_C0ESR::ASE;
    pub use super::MDMA_C0ESR::BSE;
    pub use super::MDMA_C0ESR::TEA;
    pub use super::MDMA_C0ESR::TED;
    pub use super::MDMA_C0ESR::TELD;
    pub use super::MDMA_C0ESR::TEMD;
}

/// This register is used to control the concerned channel.
pub mod MDMA_C17CR {
    pub use super::MDMA_C0CR::BEX;
    pub use super::MDMA_C0CR::BRTIE;
    pub use super::MDMA_C0CR::BTIE;
    pub use super::MDMA_C0CR::CTCIE;
    pub use super::MDMA_C0CR::EN;
    pub use super::MDMA_C0CR::HEX;
    pub use super::MDMA_C0CR::PL;
    pub use super::MDMA_C0CR::SWRQ;
    pub use super::MDMA_C0CR::TCIE;
    pub use super::MDMA_C0CR::TEIE;
    pub use super::MDMA_C0CR::WEX;
}

/// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod MDMA_C17TCR {
    pub use super::MDMA_C0TCR::BWM;
    pub use super::MDMA_C0TCR::DBURST;
    pub use super::MDMA_C0TCR::DINC;
    pub use super::MDMA_C0TCR::DINCOS;
    pub use super::MDMA_C0TCR::DSIZE;
    pub use super::MDMA_C0TCR::PAM;
    pub use super::MDMA_C0TCR::PKE;
    pub use super::MDMA_C0TCR::SBURST;
    pub use super::MDMA_C0TCR::SINC;
    pub use super::MDMA_C0TCR::SINCOS;
    pub use super::MDMA_C0TCR::SSIZE;
    pub use super::MDMA_C0TCR::SWRM;
    pub use super::MDMA_C0TCR::TLEN;
    pub use super::MDMA_C0TCR::TRGM;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod MDMA_C17BNDTR {
    pub use super::MDMA_C0BNDTR::BNDT;
    pub use super::MDMA_C0BNDTR::BRC;
    pub use super::MDMA_C0BNDTR::BRDUM;
    pub use super::MDMA_C0BNDTR::BRSUM;
}

/// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod MDMA_C17SAR {
    pub use super::MDMA_C0SAR::SAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod MDMA_C17DAR {
    pub use super::MDMA_C0DAR::DAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod MDMA_C17BRUR {
    pub use super::MDMA_C0BRUR::DUV;
    pub use super::MDMA_C0BRUR::SUV;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod MDMA_C17LAR {
    pub use super::MDMA_C0LAR::LAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod MDMA_C17TBR {
    pub use super::MDMA_C0TBR::DBUS;
    pub use super::MDMA_C0TBR::SBUS;
    pub use super::MDMA_C0TBR::TSEL;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod MDMA_C17MAR {
    pub use super::MDMA_C0MAR::MAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod MDMA_C17MDR {
    pub use super::MDMA_C0MDR::MDR;
}

/// MDMA channel 18 interrupt/status register
pub mod MDMA_C18ISR {
    pub use super::MDMA_C0ISR::BRTIF;
    pub use super::MDMA_C0ISR::BTIF;
    pub use super::MDMA_C0ISR::CRQA;
    pub use super::MDMA_C0ISR::CTCIF;
    pub use super::MDMA_C0ISR::TCIF;
    pub use super::MDMA_C0ISR::TEIF;
}

/// MDMA channel 18 interrupt flag clear register
pub mod MDMA_C18IFCR {
    pub use super::MDMA_C0IFCR::CBRTIF;
    pub use super::MDMA_C0IFCR::CBTIF;
    pub use super::MDMA_C0IFCR::CCTCIF;
    pub use super::MDMA_C0IFCR::CLTCIF;
    pub use super::MDMA_C0IFCR::CTEIF;
}

/// MDMA channel 18 error status register
pub mod MDMA_C18ESR {
    pub use super::MDMA_C0ESR::ASE;
    pub use super::MDMA_C0ESR::BSE;
    pub use super::MDMA_C0ESR::TEA;
    pub use super::MDMA_C0ESR::TED;
    pub use super::MDMA_C0ESR::TELD;
    pub use super::MDMA_C0ESR::TEMD;
}

/// This register is used to control the concerned channel.
pub mod MDMA_C18CR {
    pub use super::MDMA_C0CR::BEX;
    pub use super::MDMA_C0CR::BRTIE;
    pub use super::MDMA_C0CR::BTIE;
    pub use super::MDMA_C0CR::CTCIE;
    pub use super::MDMA_C0CR::EN;
    pub use super::MDMA_C0CR::HEX;
    pub use super::MDMA_C0CR::PL;
    pub use super::MDMA_C0CR::SWRQ;
    pub use super::MDMA_C0CR::TCIE;
    pub use super::MDMA_C0CR::TEIE;
    pub use super::MDMA_C0CR::WEX;
}

/// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod MDMA_C18TCR {
    pub use super::MDMA_C0TCR::BWM;
    pub use super::MDMA_C0TCR::DBURST;
    pub use super::MDMA_C0TCR::DINC;
    pub use super::MDMA_C0TCR::DINCOS;
    pub use super::MDMA_C0TCR::DSIZE;
    pub use super::MDMA_C0TCR::PAM;
    pub use super::MDMA_C0TCR::PKE;
    pub use super::MDMA_C0TCR::SBURST;
    pub use super::MDMA_C0TCR::SINC;
    pub use super::MDMA_C0TCR::SINCOS;
    pub use super::MDMA_C0TCR::SSIZE;
    pub use super::MDMA_C0TCR::SWRM;
    pub use super::MDMA_C0TCR::TLEN;
    pub use super::MDMA_C0TCR::TRGM;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod MDMA_C18BNDTR {
    pub use super::MDMA_C0BNDTR::BNDT;
    pub use super::MDMA_C0BNDTR::BRC;
    pub use super::MDMA_C0BNDTR::BRDUM;
    pub use super::MDMA_C0BNDTR::BRSUM;
}

/// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod MDMA_C18SAR {
    pub use super::MDMA_C0SAR::SAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod MDMA_C18DAR {
    pub use super::MDMA_C0DAR::DAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod MDMA_C18BRUR {
    pub use super::MDMA_C0BRUR::DUV;
    pub use super::MDMA_C0BRUR::SUV;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod MDMA_C18LAR {
    pub use super::MDMA_C0LAR::LAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod MDMA_C18TBR {
    pub use super::MDMA_C0TBR::DBUS;
    pub use super::MDMA_C0TBR::SBUS;
    pub use super::MDMA_C0TBR::TSEL;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod MDMA_C18MAR {
    pub use super::MDMA_C0MAR::MAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod MDMA_C18MDR {
    pub use super::MDMA_C0MDR::MDR;
}

/// MDMA channel 19 interrupt/status register
pub mod MDMA_C19ISR {
    pub use super::MDMA_C0ISR::BRTIF;
    pub use super::MDMA_C0ISR::BTIF;
    pub use super::MDMA_C0ISR::CRQA;
    pub use super::MDMA_C0ISR::CTCIF;
    pub use super::MDMA_C0ISR::TCIF;
    pub use super::MDMA_C0ISR::TEIF;
}

/// MDMA channel 19 interrupt flag clear register
pub mod MDMA_C19IFCR {
    pub use super::MDMA_C0IFCR::CBRTIF;
    pub use super::MDMA_C0IFCR::CBTIF;
    pub use super::MDMA_C0IFCR::CCTCIF;
    pub use super::MDMA_C0IFCR::CLTCIF;
    pub use super::MDMA_C0IFCR::CTEIF;
}

/// MDMA channel 19 error status register
pub mod MDMA_C19ESR {
    pub use super::MDMA_C0ESR::ASE;
    pub use super::MDMA_C0ESR::BSE;
    pub use super::MDMA_C0ESR::TEA;
    pub use super::MDMA_C0ESR::TED;
    pub use super::MDMA_C0ESR::TELD;
    pub use super::MDMA_C0ESR::TEMD;
}

/// This register is used to control the concerned channel.
pub mod MDMA_C19CR {
    pub use super::MDMA_C0CR::BEX;
    pub use super::MDMA_C0CR::BRTIE;
    pub use super::MDMA_C0CR::BTIE;
    pub use super::MDMA_C0CR::CTCIE;
    pub use super::MDMA_C0CR::EN;
    pub use super::MDMA_C0CR::HEX;
    pub use super::MDMA_C0CR::PL;
    pub use super::MDMA_C0CR::SWRQ;
    pub use super::MDMA_C0CR::TCIE;
    pub use super::MDMA_C0CR::TEIE;
    pub use super::MDMA_C0CR::WEX;
}

/// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod MDMA_C19TCR {
    pub use super::MDMA_C0TCR::BWM;
    pub use super::MDMA_C0TCR::DBURST;
    pub use super::MDMA_C0TCR::DINC;
    pub use super::MDMA_C0TCR::DINCOS;
    pub use super::MDMA_C0TCR::DSIZE;
    pub use super::MDMA_C0TCR::PAM;
    pub use super::MDMA_C0TCR::PKE;
    pub use super::MDMA_C0TCR::SBURST;
    pub use super::MDMA_C0TCR::SINC;
    pub use super::MDMA_C0TCR::SINCOS;
    pub use super::MDMA_C0TCR::SSIZE;
    pub use super::MDMA_C0TCR::SWRM;
    pub use super::MDMA_C0TCR::TLEN;
    pub use super::MDMA_C0TCR::TRGM;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod MDMA_C19BNDTR {
    pub use super::MDMA_C0BNDTR::BNDT;
    pub use super::MDMA_C0BNDTR::BRC;
    pub use super::MDMA_C0BNDTR::BRDUM;
    pub use super::MDMA_C0BNDTR::BRSUM;
}

/// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod MDMA_C19SAR {
    pub use super::MDMA_C0SAR::SAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod MDMA_C19DAR {
    pub use super::MDMA_C0DAR::DAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod MDMA_C19BRUR {
    pub use super::MDMA_C0BRUR::DUV;
    pub use super::MDMA_C0BRUR::SUV;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod MDMA_C19LAR {
    pub use super::MDMA_C0LAR::LAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod MDMA_C19TBR {
    pub use super::MDMA_C0TBR::DBUS;
    pub use super::MDMA_C0TBR::SBUS;
    pub use super::MDMA_C0TBR::TSEL;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod MDMA_C19MAR {
    pub use super::MDMA_C0MAR::MAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod MDMA_C19MDR {
    pub use super::MDMA_C0MDR::MDR;
}

/// MDMA channel 20 interrupt/status register
pub mod MDMA_C20ISR {
    pub use super::MDMA_C0ISR::BRTIF;
    pub use super::MDMA_C0ISR::BTIF;
    pub use super::MDMA_C0ISR::CRQA;
    pub use super::MDMA_C0ISR::CTCIF;
    pub use super::MDMA_C0ISR::TCIF;
    pub use super::MDMA_C0ISR::TEIF;
}

/// MDMA channel 20 interrupt flag clear register
pub mod MDMA_C20IFCR {
    pub use super::MDMA_C0IFCR::CBRTIF;
    pub use super::MDMA_C0IFCR::CBTIF;
    pub use super::MDMA_C0IFCR::CCTCIF;
    pub use super::MDMA_C0IFCR::CLTCIF;
    pub use super::MDMA_C0IFCR::CTEIF;
}

/// MDMA channel 20 error status register
pub mod MDMA_C20ESR {
    pub use super::MDMA_C0ESR::ASE;
    pub use super::MDMA_C0ESR::BSE;
    pub use super::MDMA_C0ESR::TEA;
    pub use super::MDMA_C0ESR::TED;
    pub use super::MDMA_C0ESR::TELD;
    pub use super::MDMA_C0ESR::TEMD;
}

/// This register is used to control the concerned channel.
pub mod MDMA_C20CR {
    pub use super::MDMA_C0CR::BEX;
    pub use super::MDMA_C0CR::BRTIE;
    pub use super::MDMA_C0CR::BTIE;
    pub use super::MDMA_C0CR::CTCIE;
    pub use super::MDMA_C0CR::EN;
    pub use super::MDMA_C0CR::HEX;
    pub use super::MDMA_C0CR::PL;
    pub use super::MDMA_C0CR::SWRQ;
    pub use super::MDMA_C0CR::TCIE;
    pub use super::MDMA_C0CR::TEIE;
    pub use super::MDMA_C0CR::WEX;
}

/// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod MDMA_C20TCR {
    pub use super::MDMA_C0TCR::BWM;
    pub use super::MDMA_C0TCR::DBURST;
    pub use super::MDMA_C0TCR::DINC;
    pub use super::MDMA_C0TCR::DINCOS;
    pub use super::MDMA_C0TCR::DSIZE;
    pub use super::MDMA_C0TCR::PAM;
    pub use super::MDMA_C0TCR::PKE;
    pub use super::MDMA_C0TCR::SBURST;
    pub use super::MDMA_C0TCR::SINC;
    pub use super::MDMA_C0TCR::SINCOS;
    pub use super::MDMA_C0TCR::SSIZE;
    pub use super::MDMA_C0TCR::SWRM;
    pub use super::MDMA_C0TCR::TLEN;
    pub use super::MDMA_C0TCR::TRGM;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod MDMA_C20BNDTR {
    pub use super::MDMA_C0BNDTR::BNDT;
    pub use super::MDMA_C0BNDTR::BRC;
    pub use super::MDMA_C0BNDTR::BRDUM;
    pub use super::MDMA_C0BNDTR::BRSUM;
}

/// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod MDMA_C20SAR {
    pub use super::MDMA_C0SAR::SAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod MDMA_C20DAR {
    pub use super::MDMA_C0DAR::DAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod MDMA_C20BRUR {
    pub use super::MDMA_C0BRUR::DUV;
    pub use super::MDMA_C0BRUR::SUV;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod MDMA_C20LAR {
    pub use super::MDMA_C0LAR::LAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod MDMA_C20TBR {
    pub use super::MDMA_C0TBR::DBUS;
    pub use super::MDMA_C0TBR::SBUS;
    pub use super::MDMA_C0TBR::TSEL;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod MDMA_C20MAR {
    pub use super::MDMA_C0MAR::MAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod MDMA_C20MDR {
    pub use super::MDMA_C0MDR::MDR;
}

/// MDMA channel 21 interrupt/status register
pub mod MDMA_C21ISR {
    pub use super::MDMA_C0ISR::BRTIF;
    pub use super::MDMA_C0ISR::BTIF;
    pub use super::MDMA_C0ISR::CRQA;
    pub use super::MDMA_C0ISR::CTCIF;
    pub use super::MDMA_C0ISR::TCIF;
    pub use super::MDMA_C0ISR::TEIF;
}

/// MDMA channel 21 interrupt flag clear register
pub mod MDMA_C21IFCR {
    pub use super::MDMA_C0IFCR::CBRTIF;
    pub use super::MDMA_C0IFCR::CBTIF;
    pub use super::MDMA_C0IFCR::CCTCIF;
    pub use super::MDMA_C0IFCR::CLTCIF;
    pub use super::MDMA_C0IFCR::CTEIF;
}

/// MDMA channel 21 error status register
pub mod MDMA_C21ESR {
    pub use super::MDMA_C0ESR::ASE;
    pub use super::MDMA_C0ESR::BSE;
    pub use super::MDMA_C0ESR::TEA;
    pub use super::MDMA_C0ESR::TED;
    pub use super::MDMA_C0ESR::TELD;
    pub use super::MDMA_C0ESR::TEMD;
}

/// This register is used to control the concerned channel.
pub mod MDMA_C21CR {
    pub use super::MDMA_C0CR::BEX;
    pub use super::MDMA_C0CR::BRTIE;
    pub use super::MDMA_C0CR::BTIE;
    pub use super::MDMA_C0CR::CTCIE;
    pub use super::MDMA_C0CR::EN;
    pub use super::MDMA_C0CR::HEX;
    pub use super::MDMA_C0CR::PL;
    pub use super::MDMA_C0CR::SWRQ;
    pub use super::MDMA_C0CR::TCIE;
    pub use super::MDMA_C0CR::TEIE;
    pub use super::MDMA_C0CR::WEX;
}

/// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod MDMA_C21TCR {
    pub use super::MDMA_C0TCR::BWM;
    pub use super::MDMA_C0TCR::DBURST;
    pub use super::MDMA_C0TCR::DINC;
    pub use super::MDMA_C0TCR::DINCOS;
    pub use super::MDMA_C0TCR::DSIZE;
    pub use super::MDMA_C0TCR::PAM;
    pub use super::MDMA_C0TCR::PKE;
    pub use super::MDMA_C0TCR::SBURST;
    pub use super::MDMA_C0TCR::SINC;
    pub use super::MDMA_C0TCR::SINCOS;
    pub use super::MDMA_C0TCR::SSIZE;
    pub use super::MDMA_C0TCR::SWRM;
    pub use super::MDMA_C0TCR::TLEN;
    pub use super::MDMA_C0TCR::TRGM;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod MDMA_C21BNDTR {
    pub use super::MDMA_C0BNDTR::BNDT;
    pub use super::MDMA_C0BNDTR::BRC;
    pub use super::MDMA_C0BNDTR::BRDUM;
    pub use super::MDMA_C0BNDTR::BRSUM;
}

/// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod MDMA_C21SAR {
    pub use super::MDMA_C0SAR::SAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod MDMA_C21DAR {
    pub use super::MDMA_C0DAR::DAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod MDMA_C21BRUR {
    pub use super::MDMA_C0BRUR::DUV;
    pub use super::MDMA_C0BRUR::SUV;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod MDMA_C21LAR {
    pub use super::MDMA_C0LAR::LAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod MDMA_C21TBR {
    pub use super::MDMA_C0TBR::DBUS;
    pub use super::MDMA_C0TBR::SBUS;
    pub use super::MDMA_C0TBR::TSEL;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod MDMA_C21MAR {
    pub use super::MDMA_C0MAR::MAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod MDMA_C21MDR {
    pub use super::MDMA_C0MDR::MDR;
}

/// MDMA channel 22 interrupt/status register
pub mod MDMA_C22ISR {
    pub use super::MDMA_C0ISR::BRTIF;
    pub use super::MDMA_C0ISR::BTIF;
    pub use super::MDMA_C0ISR::CRQA;
    pub use super::MDMA_C0ISR::CTCIF;
    pub use super::MDMA_C0ISR::TCIF;
    pub use super::MDMA_C0ISR::TEIF;
}

/// MDMA channel 22 interrupt flag clear register
pub mod MDMA_C22IFCR {
    pub use super::MDMA_C0IFCR::CBRTIF;
    pub use super::MDMA_C0IFCR::CBTIF;
    pub use super::MDMA_C0IFCR::CCTCIF;
    pub use super::MDMA_C0IFCR::CLTCIF;
    pub use super::MDMA_C0IFCR::CTEIF;
}

/// MDMA channel 22 error status register
pub mod MDMA_C22ESR {
    pub use super::MDMA_C0ESR::ASE;
    pub use super::MDMA_C0ESR::BSE;
    pub use super::MDMA_C0ESR::TEA;
    pub use super::MDMA_C0ESR::TED;
    pub use super::MDMA_C0ESR::TELD;
    pub use super::MDMA_C0ESR::TEMD;
}

/// This register is used to control the concerned channel.
pub mod MDMA_C22CR {
    pub use super::MDMA_C0CR::BEX;
    pub use super::MDMA_C0CR::BRTIE;
    pub use super::MDMA_C0CR::BTIE;
    pub use super::MDMA_C0CR::CTCIE;
    pub use super::MDMA_C0CR::EN;
    pub use super::MDMA_C0CR::HEX;
    pub use super::MDMA_C0CR::PL;
    pub use super::MDMA_C0CR::SWRQ;
    pub use super::MDMA_C0CR::TCIE;
    pub use super::MDMA_C0CR::TEIE;
    pub use super::MDMA_C0CR::WEX;
}

/// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod MDMA_C22TCR {
    pub use super::MDMA_C0TCR::BWM;
    pub use super::MDMA_C0TCR::DBURST;
    pub use super::MDMA_C0TCR::DINC;
    pub use super::MDMA_C0TCR::DINCOS;
    pub use super::MDMA_C0TCR::DSIZE;
    pub use super::MDMA_C0TCR::PAM;
    pub use super::MDMA_C0TCR::PKE;
    pub use super::MDMA_C0TCR::SBURST;
    pub use super::MDMA_C0TCR::SINC;
    pub use super::MDMA_C0TCR::SINCOS;
    pub use super::MDMA_C0TCR::SSIZE;
    pub use super::MDMA_C0TCR::SWRM;
    pub use super::MDMA_C0TCR::TLEN;
    pub use super::MDMA_C0TCR::TRGM;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod MDMA_C22BNDTR {
    pub use super::MDMA_C0BNDTR::BNDT;
    pub use super::MDMA_C0BNDTR::BRC;
    pub use super::MDMA_C0BNDTR::BRDUM;
    pub use super::MDMA_C0BNDTR::BRSUM;
}

/// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod MDMA_C22SAR {
    pub use super::MDMA_C0SAR::SAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod MDMA_C22DAR {
    pub use super::MDMA_C0DAR::DAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod MDMA_C22BRUR {
    pub use super::MDMA_C0BRUR::DUV;
    pub use super::MDMA_C0BRUR::SUV;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod MDMA_C22LAR {
    pub use super::MDMA_C0LAR::LAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod MDMA_C22TBR {
    pub use super::MDMA_C0TBR::DBUS;
    pub use super::MDMA_C0TBR::SBUS;
    pub use super::MDMA_C0TBR::TSEL;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod MDMA_C22MAR {
    pub use super::MDMA_C0MAR::MAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod MDMA_C22MDR {
    pub use super::MDMA_C0MDR::MDR;
}

/// MDMA channel 23 interrupt/status register
pub mod MDMA_C23ISR {
    pub use super::MDMA_C0ISR::BRTIF;
    pub use super::MDMA_C0ISR::BTIF;
    pub use super::MDMA_C0ISR::CRQA;
    pub use super::MDMA_C0ISR::CTCIF;
    pub use super::MDMA_C0ISR::TCIF;
    pub use super::MDMA_C0ISR::TEIF;
}

/// MDMA channel 23 interrupt flag clear register
pub mod MDMA_C23IFCR {
    pub use super::MDMA_C0IFCR::CBRTIF;
    pub use super::MDMA_C0IFCR::CBTIF;
    pub use super::MDMA_C0IFCR::CCTCIF;
    pub use super::MDMA_C0IFCR::CLTCIF;
    pub use super::MDMA_C0IFCR::CTEIF;
}

/// MDMA channel 23 error status register
pub mod MDMA_C23ESR {
    pub use super::MDMA_C0ESR::ASE;
    pub use super::MDMA_C0ESR::BSE;
    pub use super::MDMA_C0ESR::TEA;
    pub use super::MDMA_C0ESR::TED;
    pub use super::MDMA_C0ESR::TELD;
    pub use super::MDMA_C0ESR::TEMD;
}

/// This register is used to control the concerned channel.
pub mod MDMA_C23CR {
    pub use super::MDMA_C0CR::BEX;
    pub use super::MDMA_C0CR::BRTIE;
    pub use super::MDMA_C0CR::BTIE;
    pub use super::MDMA_C0CR::CTCIE;
    pub use super::MDMA_C0CR::EN;
    pub use super::MDMA_C0CR::HEX;
    pub use super::MDMA_C0CR::PL;
    pub use super::MDMA_C0CR::SWRQ;
    pub use super::MDMA_C0CR::TCIE;
    pub use super::MDMA_C0CR::TEIE;
    pub use super::MDMA_C0CR::WEX;
}

/// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod MDMA_C23TCR {
    pub use super::MDMA_C0TCR::BWM;
    pub use super::MDMA_C0TCR::DBURST;
    pub use super::MDMA_C0TCR::DINC;
    pub use super::MDMA_C0TCR::DINCOS;
    pub use super::MDMA_C0TCR::DSIZE;
    pub use super::MDMA_C0TCR::PAM;
    pub use super::MDMA_C0TCR::PKE;
    pub use super::MDMA_C0TCR::SBURST;
    pub use super::MDMA_C0TCR::SINC;
    pub use super::MDMA_C0TCR::SINCOS;
    pub use super::MDMA_C0TCR::SSIZE;
    pub use super::MDMA_C0TCR::SWRM;
    pub use super::MDMA_C0TCR::TLEN;
    pub use super::MDMA_C0TCR::TRGM;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod MDMA_C23BNDTR {
    pub use super::MDMA_C0BNDTR::BNDT;
    pub use super::MDMA_C0BNDTR::BRC;
    pub use super::MDMA_C0BNDTR::BRDUM;
    pub use super::MDMA_C0BNDTR::BRSUM;
}

/// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod MDMA_C23SAR {
    pub use super::MDMA_C0SAR::SAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod MDMA_C23DAR {
    pub use super::MDMA_C0DAR::DAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod MDMA_C23BRUR {
    pub use super::MDMA_C0BRUR::DUV;
    pub use super::MDMA_C0BRUR::SUV;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod MDMA_C23LAR {
    pub use super::MDMA_C0LAR::LAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod MDMA_C23TBR {
    pub use super::MDMA_C0TBR::DBUS;
    pub use super::MDMA_C0TBR::SBUS;
    pub use super::MDMA_C0TBR::TSEL;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod MDMA_C23MAR {
    pub use super::MDMA_C0MAR::MAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod MDMA_C23MDR {
    pub use super::MDMA_C0MDR::MDR;
}

/// MDMA channel 24 interrupt/status register
pub mod MDMA_C24ISR {
    pub use super::MDMA_C0ISR::BRTIF;
    pub use super::MDMA_C0ISR::BTIF;
    pub use super::MDMA_C0ISR::CRQA;
    pub use super::MDMA_C0ISR::CTCIF;
    pub use super::MDMA_C0ISR::TCIF;
    pub use super::MDMA_C0ISR::TEIF;
}

/// MDMA channel 24 interrupt flag clear register
pub mod MDMA_C24IFCR {
    pub use super::MDMA_C0IFCR::CBRTIF;
    pub use super::MDMA_C0IFCR::CBTIF;
    pub use super::MDMA_C0IFCR::CCTCIF;
    pub use super::MDMA_C0IFCR::CLTCIF;
    pub use super::MDMA_C0IFCR::CTEIF;
}

/// MDMA channel 24 error status register
pub mod MDMA_C24ESR {
    pub use super::MDMA_C0ESR::ASE;
    pub use super::MDMA_C0ESR::BSE;
    pub use super::MDMA_C0ESR::TEA;
    pub use super::MDMA_C0ESR::TED;
    pub use super::MDMA_C0ESR::TELD;
    pub use super::MDMA_C0ESR::TEMD;
}

/// This register is used to control the concerned channel.
pub mod MDMA_C24CR {
    pub use super::MDMA_C0CR::BEX;
    pub use super::MDMA_C0CR::BRTIE;
    pub use super::MDMA_C0CR::BTIE;
    pub use super::MDMA_C0CR::CTCIE;
    pub use super::MDMA_C0CR::EN;
    pub use super::MDMA_C0CR::HEX;
    pub use super::MDMA_C0CR::PL;
    pub use super::MDMA_C0CR::SWRQ;
    pub use super::MDMA_C0CR::TCIE;
    pub use super::MDMA_C0CR::TEIE;
    pub use super::MDMA_C0CR::WEX;
}

/// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod MDMA_C24TCR {
    pub use super::MDMA_C0TCR::BWM;
    pub use super::MDMA_C0TCR::DBURST;
    pub use super::MDMA_C0TCR::DINC;
    pub use super::MDMA_C0TCR::DINCOS;
    pub use super::MDMA_C0TCR::DSIZE;
    pub use super::MDMA_C0TCR::PAM;
    pub use super::MDMA_C0TCR::PKE;
    pub use super::MDMA_C0TCR::SBURST;
    pub use super::MDMA_C0TCR::SINC;
    pub use super::MDMA_C0TCR::SINCOS;
    pub use super::MDMA_C0TCR::SSIZE;
    pub use super::MDMA_C0TCR::SWRM;
    pub use super::MDMA_C0TCR::TLEN;
    pub use super::MDMA_C0TCR::TRGM;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod MDMA_C24BNDTR {
    pub use super::MDMA_C0BNDTR::BNDT;
    pub use super::MDMA_C0BNDTR::BRC;
    pub use super::MDMA_C0BNDTR::BRDUM;
    pub use super::MDMA_C0BNDTR::BRSUM;
}

/// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod MDMA_C24SAR {
    pub use super::MDMA_C0SAR::SAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod MDMA_C24DAR {
    pub use super::MDMA_C0DAR::DAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod MDMA_C24BRUR {
    pub use super::MDMA_C0BRUR::DUV;
    pub use super::MDMA_C0BRUR::SUV;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod MDMA_C24LAR {
    pub use super::MDMA_C0LAR::LAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod MDMA_C24TBR {
    pub use super::MDMA_C0TBR::DBUS;
    pub use super::MDMA_C0TBR::SBUS;
    pub use super::MDMA_C0TBR::TSEL;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod MDMA_C24MAR {
    pub use super::MDMA_C0MAR::MAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod MDMA_C24MDR {
    pub use super::MDMA_C0MDR::MDR;
}

/// MDMA channel 25 interrupt/status register
pub mod MDMA_C25ISR {
    pub use super::MDMA_C0ISR::BRTIF;
    pub use super::MDMA_C0ISR::BTIF;
    pub use super::MDMA_C0ISR::CRQA;
    pub use super::MDMA_C0ISR::CTCIF;
    pub use super::MDMA_C0ISR::TCIF;
    pub use super::MDMA_C0ISR::TEIF;
}

/// MDMA channel 25 interrupt flag clear register
pub mod MDMA_C25IFCR {
    pub use super::MDMA_C0IFCR::CBRTIF;
    pub use super::MDMA_C0IFCR::CBTIF;
    pub use super::MDMA_C0IFCR::CCTCIF;
    pub use super::MDMA_C0IFCR::CLTCIF;
    pub use super::MDMA_C0IFCR::CTEIF;
}

/// MDMA channel 25 error status register
pub mod MDMA_C25ESR {
    pub use super::MDMA_C0ESR::ASE;
    pub use super::MDMA_C0ESR::BSE;
    pub use super::MDMA_C0ESR::TEA;
    pub use super::MDMA_C0ESR::TED;
    pub use super::MDMA_C0ESR::TELD;
    pub use super::MDMA_C0ESR::TEMD;
}

/// This register is used to control the concerned channel.
pub mod MDMA_C25CR {
    pub use super::MDMA_C0CR::BEX;
    pub use super::MDMA_C0CR::BRTIE;
    pub use super::MDMA_C0CR::BTIE;
    pub use super::MDMA_C0CR::CTCIE;
    pub use super::MDMA_C0CR::EN;
    pub use super::MDMA_C0CR::HEX;
    pub use super::MDMA_C0CR::PL;
    pub use super::MDMA_C0CR::SWRQ;
    pub use super::MDMA_C0CR::TCIE;
    pub use super::MDMA_C0CR::TEIE;
    pub use super::MDMA_C0CR::WEX;
}

/// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod MDMA_C25TCR {
    pub use super::MDMA_C0TCR::BWM;
    pub use super::MDMA_C0TCR::DBURST;
    pub use super::MDMA_C0TCR::DINC;
    pub use super::MDMA_C0TCR::DINCOS;
    pub use super::MDMA_C0TCR::DSIZE;
    pub use super::MDMA_C0TCR::PAM;
    pub use super::MDMA_C0TCR::PKE;
    pub use super::MDMA_C0TCR::SBURST;
    pub use super::MDMA_C0TCR::SINC;
    pub use super::MDMA_C0TCR::SINCOS;
    pub use super::MDMA_C0TCR::SSIZE;
    pub use super::MDMA_C0TCR::SWRM;
    pub use super::MDMA_C0TCR::TLEN;
    pub use super::MDMA_C0TCR::TRGM;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod MDMA_C25BNDTR {
    pub use super::MDMA_C0BNDTR::BNDT;
    pub use super::MDMA_C0BNDTR::BRC;
    pub use super::MDMA_C0BNDTR::BRDUM;
    pub use super::MDMA_C0BNDTR::BRSUM;
}

/// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod MDMA_C25SAR {
    pub use super::MDMA_C0SAR::SAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod MDMA_C25DAR {
    pub use super::MDMA_C0DAR::DAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod MDMA_C25BRUR {
    pub use super::MDMA_C0BRUR::DUV;
    pub use super::MDMA_C0BRUR::SUV;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod MDMA_C25LAR {
    pub use super::MDMA_C0LAR::LAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod MDMA_C25TBR {
    pub use super::MDMA_C0TBR::DBUS;
    pub use super::MDMA_C0TBR::SBUS;
    pub use super::MDMA_C0TBR::TSEL;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod MDMA_C25MAR {
    pub use super::MDMA_C0MAR::MAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod MDMA_C25MDR {
    pub use super::MDMA_C0MDR::MDR;
}

/// MDMA channel 26 interrupt/status register
pub mod MDMA_C26ISR {
    pub use super::MDMA_C0ISR::BRTIF;
    pub use super::MDMA_C0ISR::BTIF;
    pub use super::MDMA_C0ISR::CRQA;
    pub use super::MDMA_C0ISR::CTCIF;
    pub use super::MDMA_C0ISR::TCIF;
    pub use super::MDMA_C0ISR::TEIF;
}

/// MDMA channel 26 interrupt flag clear register
pub mod MDMA_C26IFCR {
    pub use super::MDMA_C0IFCR::CBRTIF;
    pub use super::MDMA_C0IFCR::CBTIF;
    pub use super::MDMA_C0IFCR::CCTCIF;
    pub use super::MDMA_C0IFCR::CLTCIF;
    pub use super::MDMA_C0IFCR::CTEIF;
}

/// MDMA channel 26 error status register
pub mod MDMA_C26ESR {
    pub use super::MDMA_C0ESR::ASE;
    pub use super::MDMA_C0ESR::BSE;
    pub use super::MDMA_C0ESR::TEA;
    pub use super::MDMA_C0ESR::TED;
    pub use super::MDMA_C0ESR::TELD;
    pub use super::MDMA_C0ESR::TEMD;
}

/// This register is used to control the concerned channel.
pub mod MDMA_C26CR {
    pub use super::MDMA_C0CR::BEX;
    pub use super::MDMA_C0CR::BRTIE;
    pub use super::MDMA_C0CR::BTIE;
    pub use super::MDMA_C0CR::CTCIE;
    pub use super::MDMA_C0CR::EN;
    pub use super::MDMA_C0CR::HEX;
    pub use super::MDMA_C0CR::PL;
    pub use super::MDMA_C0CR::SWRQ;
    pub use super::MDMA_C0CR::TCIE;
    pub use super::MDMA_C0CR::TEIE;
    pub use super::MDMA_C0CR::WEX;
}

/// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod MDMA_C26TCR {
    pub use super::MDMA_C0TCR::BWM;
    pub use super::MDMA_C0TCR::DBURST;
    pub use super::MDMA_C0TCR::DINC;
    pub use super::MDMA_C0TCR::DINCOS;
    pub use super::MDMA_C0TCR::DSIZE;
    pub use super::MDMA_C0TCR::PAM;
    pub use super::MDMA_C0TCR::PKE;
    pub use super::MDMA_C0TCR::SBURST;
    pub use super::MDMA_C0TCR::SINC;
    pub use super::MDMA_C0TCR::SINCOS;
    pub use super::MDMA_C0TCR::SSIZE;
    pub use super::MDMA_C0TCR::SWRM;
    pub use super::MDMA_C0TCR::TLEN;
    pub use super::MDMA_C0TCR::TRGM;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod MDMA_C26BNDTR {
    pub use super::MDMA_C0BNDTR::BNDT;
    pub use super::MDMA_C0BNDTR::BRC;
    pub use super::MDMA_C0BNDTR::BRDUM;
    pub use super::MDMA_C0BNDTR::BRSUM;
}

/// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod MDMA_C26SAR {
    pub use super::MDMA_C0SAR::SAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod MDMA_C26DAR {
    pub use super::MDMA_C0DAR::DAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod MDMA_C26BRUR {
    pub use super::MDMA_C0BRUR::DUV;
    pub use super::MDMA_C0BRUR::SUV;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod MDMA_C26LAR {
    pub use super::MDMA_C0LAR::LAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod MDMA_C26TBR {
    pub use super::MDMA_C0TBR::DBUS;
    pub use super::MDMA_C0TBR::SBUS;
    pub use super::MDMA_C0TBR::TSEL;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod MDMA_C26MAR {
    pub use super::MDMA_C0MAR::MAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod MDMA_C26MDR {
    pub use super::MDMA_C0MDR::MDR;
}

/// MDMA channel 27 interrupt/status register
pub mod MDMA_C27ISR {
    pub use super::MDMA_C0ISR::BRTIF;
    pub use super::MDMA_C0ISR::BTIF;
    pub use super::MDMA_C0ISR::CRQA;
    pub use super::MDMA_C0ISR::CTCIF;
    pub use super::MDMA_C0ISR::TCIF;
    pub use super::MDMA_C0ISR::TEIF;
}

/// MDMA channel 27 interrupt flag clear register
pub mod MDMA_C27IFCR {
    pub use super::MDMA_C0IFCR::CBRTIF;
    pub use super::MDMA_C0IFCR::CBTIF;
    pub use super::MDMA_C0IFCR::CCTCIF;
    pub use super::MDMA_C0IFCR::CLTCIF;
    pub use super::MDMA_C0IFCR::CTEIF;
}

/// MDMA channel 27 error status register
pub mod MDMA_C27ESR {
    pub use super::MDMA_C0ESR::ASE;
    pub use super::MDMA_C0ESR::BSE;
    pub use super::MDMA_C0ESR::TEA;
    pub use super::MDMA_C0ESR::TED;
    pub use super::MDMA_C0ESR::TELD;
    pub use super::MDMA_C0ESR::TEMD;
}

/// This register is used to control the concerned channel.
pub mod MDMA_C27CR {
    pub use super::MDMA_C0CR::BEX;
    pub use super::MDMA_C0CR::BRTIE;
    pub use super::MDMA_C0CR::BTIE;
    pub use super::MDMA_C0CR::CTCIE;
    pub use super::MDMA_C0CR::EN;
    pub use super::MDMA_C0CR::HEX;
    pub use super::MDMA_C0CR::PL;
    pub use super::MDMA_C0CR::SWRQ;
    pub use super::MDMA_C0CR::TCIE;
    pub use super::MDMA_C0CR::TEIE;
    pub use super::MDMA_C0CR::WEX;
}

/// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod MDMA_C27TCR {
    pub use super::MDMA_C0TCR::BWM;
    pub use super::MDMA_C0TCR::DBURST;
    pub use super::MDMA_C0TCR::DINC;
    pub use super::MDMA_C0TCR::DINCOS;
    pub use super::MDMA_C0TCR::DSIZE;
    pub use super::MDMA_C0TCR::PAM;
    pub use super::MDMA_C0TCR::PKE;
    pub use super::MDMA_C0TCR::SBURST;
    pub use super::MDMA_C0TCR::SINC;
    pub use super::MDMA_C0TCR::SINCOS;
    pub use super::MDMA_C0TCR::SSIZE;
    pub use super::MDMA_C0TCR::SWRM;
    pub use super::MDMA_C0TCR::TLEN;
    pub use super::MDMA_C0TCR::TRGM;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod MDMA_C27BNDTR {
    pub use super::MDMA_C0BNDTR::BNDT;
    pub use super::MDMA_C0BNDTR::BRC;
    pub use super::MDMA_C0BNDTR::BRDUM;
    pub use super::MDMA_C0BNDTR::BRSUM;
}

/// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod MDMA_C27SAR {
    pub use super::MDMA_C0SAR::SAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod MDMA_C27DAR {
    pub use super::MDMA_C0DAR::DAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod MDMA_C27BRUR {
    pub use super::MDMA_C0BRUR::DUV;
    pub use super::MDMA_C0BRUR::SUV;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod MDMA_C27LAR {
    pub use super::MDMA_C0LAR::LAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod MDMA_C27TBR {
    pub use super::MDMA_C0TBR::DBUS;
    pub use super::MDMA_C0TBR::SBUS;
    pub use super::MDMA_C0TBR::TSEL;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod MDMA_C27MAR {
    pub use super::MDMA_C0MAR::MAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod MDMA_C27MDR {
    pub use super::MDMA_C0MDR::MDR;
}

/// MDMA channel 28 interrupt/status register
pub mod MDMA_C28ISR {
    pub use super::MDMA_C0ISR::BRTIF;
    pub use super::MDMA_C0ISR::BTIF;
    pub use super::MDMA_C0ISR::CRQA;
    pub use super::MDMA_C0ISR::CTCIF;
    pub use super::MDMA_C0ISR::TCIF;
    pub use super::MDMA_C0ISR::TEIF;
}

/// MDMA channel 28 interrupt flag clear register
pub mod MDMA_C28IFCR {
    pub use super::MDMA_C0IFCR::CBRTIF;
    pub use super::MDMA_C0IFCR::CBTIF;
    pub use super::MDMA_C0IFCR::CCTCIF;
    pub use super::MDMA_C0IFCR::CLTCIF;
    pub use super::MDMA_C0IFCR::CTEIF;
}

/// MDMA channel 28 error status register
pub mod MDMA_C28ESR {
    pub use super::MDMA_C0ESR::ASE;
    pub use super::MDMA_C0ESR::BSE;
    pub use super::MDMA_C0ESR::TEA;
    pub use super::MDMA_C0ESR::TED;
    pub use super::MDMA_C0ESR::TELD;
    pub use super::MDMA_C0ESR::TEMD;
}

/// This register is used to control the concerned channel.
pub mod MDMA_C28CR {
    pub use super::MDMA_C0CR::BEX;
    pub use super::MDMA_C0CR::BRTIE;
    pub use super::MDMA_C0CR::BTIE;
    pub use super::MDMA_C0CR::CTCIE;
    pub use super::MDMA_C0CR::EN;
    pub use super::MDMA_C0CR::HEX;
    pub use super::MDMA_C0CR::PL;
    pub use super::MDMA_C0CR::SWRQ;
    pub use super::MDMA_C0CR::TCIE;
    pub use super::MDMA_C0CR::TEIE;
    pub use super::MDMA_C0CR::WEX;
}

/// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod MDMA_C28TCR {
    pub use super::MDMA_C0TCR::BWM;
    pub use super::MDMA_C0TCR::DBURST;
    pub use super::MDMA_C0TCR::DINC;
    pub use super::MDMA_C0TCR::DINCOS;
    pub use super::MDMA_C0TCR::DSIZE;
    pub use super::MDMA_C0TCR::PAM;
    pub use super::MDMA_C0TCR::PKE;
    pub use super::MDMA_C0TCR::SBURST;
    pub use super::MDMA_C0TCR::SINC;
    pub use super::MDMA_C0TCR::SINCOS;
    pub use super::MDMA_C0TCR::SSIZE;
    pub use super::MDMA_C0TCR::SWRM;
    pub use super::MDMA_C0TCR::TLEN;
    pub use super::MDMA_C0TCR::TRGM;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod MDMA_C28BNDTR {
    pub use super::MDMA_C0BNDTR::BNDT;
    pub use super::MDMA_C0BNDTR::BRC;
    pub use super::MDMA_C0BNDTR::BRDUM;
    pub use super::MDMA_C0BNDTR::BRSUM;
}

/// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod MDMA_C28SAR {
    pub use super::MDMA_C0SAR::SAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod MDMA_C28DAR {
    pub use super::MDMA_C0DAR::DAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod MDMA_C28BRUR {
    pub use super::MDMA_C0BRUR::DUV;
    pub use super::MDMA_C0BRUR::SUV;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod MDMA_C28LAR {
    pub use super::MDMA_C0LAR::LAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod MDMA_C28TBR {
    pub use super::MDMA_C0TBR::DBUS;
    pub use super::MDMA_C0TBR::SBUS;
    pub use super::MDMA_C0TBR::TSEL;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod MDMA_C28MAR {
    pub use super::MDMA_C0MAR::MAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod MDMA_C28MDR {
    pub use super::MDMA_C0MDR::MDR;
}

/// MDMA channel 29 interrupt/status register
pub mod MDMA_C29ISR {
    pub use super::MDMA_C0ISR::BRTIF;
    pub use super::MDMA_C0ISR::BTIF;
    pub use super::MDMA_C0ISR::CRQA;
    pub use super::MDMA_C0ISR::CTCIF;
    pub use super::MDMA_C0ISR::TCIF;
    pub use super::MDMA_C0ISR::TEIF;
}

/// MDMA channel 29 interrupt flag clear register
pub mod MDMA_C29IFCR {
    pub use super::MDMA_C0IFCR::CBRTIF;
    pub use super::MDMA_C0IFCR::CBTIF;
    pub use super::MDMA_C0IFCR::CCTCIF;
    pub use super::MDMA_C0IFCR::CLTCIF;
    pub use super::MDMA_C0IFCR::CTEIF;
}

/// MDMA channel 29 error status register
pub mod MDMA_C29ESR {
    pub use super::MDMA_C0ESR::ASE;
    pub use super::MDMA_C0ESR::BSE;
    pub use super::MDMA_C0ESR::TEA;
    pub use super::MDMA_C0ESR::TED;
    pub use super::MDMA_C0ESR::TELD;
    pub use super::MDMA_C0ESR::TEMD;
}

/// This register is used to control the concerned channel.
pub mod MDMA_C29CR {
    pub use super::MDMA_C0CR::BEX;
    pub use super::MDMA_C0CR::BRTIE;
    pub use super::MDMA_C0CR::BTIE;
    pub use super::MDMA_C0CR::CTCIE;
    pub use super::MDMA_C0CR::EN;
    pub use super::MDMA_C0CR::HEX;
    pub use super::MDMA_C0CR::PL;
    pub use super::MDMA_C0CR::SWRQ;
    pub use super::MDMA_C0CR::TCIE;
    pub use super::MDMA_C0CR::TEIE;
    pub use super::MDMA_C0CR::WEX;
}

/// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod MDMA_C29TCR {
    pub use super::MDMA_C0TCR::BWM;
    pub use super::MDMA_C0TCR::DBURST;
    pub use super::MDMA_C0TCR::DINC;
    pub use super::MDMA_C0TCR::DINCOS;
    pub use super::MDMA_C0TCR::DSIZE;
    pub use super::MDMA_C0TCR::PAM;
    pub use super::MDMA_C0TCR::PKE;
    pub use super::MDMA_C0TCR::SBURST;
    pub use super::MDMA_C0TCR::SINC;
    pub use super::MDMA_C0TCR::SINCOS;
    pub use super::MDMA_C0TCR::SSIZE;
    pub use super::MDMA_C0TCR::SWRM;
    pub use super::MDMA_C0TCR::TLEN;
    pub use super::MDMA_C0TCR::TRGM;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod MDMA_C29BNDTR {
    pub use super::MDMA_C0BNDTR::BNDT;
    pub use super::MDMA_C0BNDTR::BRC;
    pub use super::MDMA_C0BNDTR::BRDUM;
    pub use super::MDMA_C0BNDTR::BRSUM;
}

/// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod MDMA_C29SAR {
    pub use super::MDMA_C0SAR::SAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod MDMA_C29DAR {
    pub use super::MDMA_C0DAR::DAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod MDMA_C29BRUR {
    pub use super::MDMA_C0BRUR::DUV;
    pub use super::MDMA_C0BRUR::SUV;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod MDMA_C29LAR {
    pub use super::MDMA_C0LAR::LAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod MDMA_C29TBR {
    pub use super::MDMA_C0TBR::DBUS;
    pub use super::MDMA_C0TBR::SBUS;
    pub use super::MDMA_C0TBR::TSEL;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod MDMA_C29MAR {
    pub use super::MDMA_C0MAR::MAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod MDMA_C29MDR {
    pub use super::MDMA_C0MDR::MDR;
}

/// MDMA channel 30 interrupt/status register
pub mod MDMA_C30ISR {
    pub use super::MDMA_C0ISR::BRTIF;
    pub use super::MDMA_C0ISR::BTIF;
    pub use super::MDMA_C0ISR::CRQA;
    pub use super::MDMA_C0ISR::CTCIF;
    pub use super::MDMA_C0ISR::TCIF;
    pub use super::MDMA_C0ISR::TEIF;
}

/// MDMA channel 30 interrupt flag clear register
pub mod MDMA_C30IFCR {
    pub use super::MDMA_C0IFCR::CBRTIF;
    pub use super::MDMA_C0IFCR::CBTIF;
    pub use super::MDMA_C0IFCR::CCTCIF;
    pub use super::MDMA_C0IFCR::CLTCIF;
    pub use super::MDMA_C0IFCR::CTEIF;
}

/// MDMA channel 30 error status register
pub mod MDMA_C30ESR {
    pub use super::MDMA_C0ESR::ASE;
    pub use super::MDMA_C0ESR::BSE;
    pub use super::MDMA_C0ESR::TEA;
    pub use super::MDMA_C0ESR::TED;
    pub use super::MDMA_C0ESR::TELD;
    pub use super::MDMA_C0ESR::TEMD;
}

/// This register is used to control the concerned channel.
pub mod MDMA_C30CR {
    pub use super::MDMA_C0CR::BEX;
    pub use super::MDMA_C0CR::BRTIE;
    pub use super::MDMA_C0CR::BTIE;
    pub use super::MDMA_C0CR::CTCIE;
    pub use super::MDMA_C0CR::EN;
    pub use super::MDMA_C0CR::HEX;
    pub use super::MDMA_C0CR::PL;
    pub use super::MDMA_C0CR::SWRQ;
    pub use super::MDMA_C0CR::TCIE;
    pub use super::MDMA_C0CR::TEIE;
    pub use super::MDMA_C0CR::WEX;
}

/// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod MDMA_C30TCR {
    pub use super::MDMA_C0TCR::BWM;
    pub use super::MDMA_C0TCR::DBURST;
    pub use super::MDMA_C0TCR::DINC;
    pub use super::MDMA_C0TCR::DINCOS;
    pub use super::MDMA_C0TCR::DSIZE;
    pub use super::MDMA_C0TCR::PAM;
    pub use super::MDMA_C0TCR::PKE;
    pub use super::MDMA_C0TCR::SBURST;
    pub use super::MDMA_C0TCR::SINC;
    pub use super::MDMA_C0TCR::SINCOS;
    pub use super::MDMA_C0TCR::SSIZE;
    pub use super::MDMA_C0TCR::SWRM;
    pub use super::MDMA_C0TCR::TLEN;
    pub use super::MDMA_C0TCR::TRGM;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod MDMA_C30BNDTR {
    pub use super::MDMA_C0BNDTR::BNDT;
    pub use super::MDMA_C0BNDTR::BRC;
    pub use super::MDMA_C0BNDTR::BRDUM;
    pub use super::MDMA_C0BNDTR::BRSUM;
}

/// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod MDMA_C30SAR {
    pub use super::MDMA_C0SAR::SAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod MDMA_C30DAR {
    pub use super::MDMA_C0DAR::DAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod MDMA_C30BRUR {
    pub use super::MDMA_C0BRUR::DUV;
    pub use super::MDMA_C0BRUR::SUV;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod MDMA_C30LAR {
    pub use super::MDMA_C0LAR::LAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod MDMA_C30TBR {
    pub use super::MDMA_C0TBR::DBUS;
    pub use super::MDMA_C0TBR::SBUS;
    pub use super::MDMA_C0TBR::TSEL;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod MDMA_C30MAR {
    pub use super::MDMA_C0MAR::MAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod MDMA_C30MDR {
    pub use super::MDMA_C0MDR::MDR;
}

/// MDMA channel 31 interrupt/status register
pub mod MDMA_C31ISR {
    pub use super::MDMA_C0ISR::BRTIF;
    pub use super::MDMA_C0ISR::BTIF;
    pub use super::MDMA_C0ISR::CRQA;
    pub use super::MDMA_C0ISR::CTCIF;
    pub use super::MDMA_C0ISR::TCIF;
    pub use super::MDMA_C0ISR::TEIF;
}

/// MDMA channel 31 interrupt flag clear register
pub mod MDMA_C31IFCR {
    pub use super::MDMA_C0IFCR::CBRTIF;
    pub use super::MDMA_C0IFCR::CBTIF;
    pub use super::MDMA_C0IFCR::CCTCIF;
    pub use super::MDMA_C0IFCR::CLTCIF;
    pub use super::MDMA_C0IFCR::CTEIF;
}

/// MDMA channel 31 error status register
pub mod MDMA_C31ESR {
    pub use super::MDMA_C0ESR::ASE;
    pub use super::MDMA_C0ESR::BSE;
    pub use super::MDMA_C0ESR::TEA;
    pub use super::MDMA_C0ESR::TED;
    pub use super::MDMA_C0ESR::TELD;
    pub use super::MDMA_C0ESR::TEMD;
}

/// This register is used to control the concerned channel.
pub mod MDMA_C31CR {
    pub use super::MDMA_C0CR::BEX;
    pub use super::MDMA_C0CR::BRTIE;
    pub use super::MDMA_C0CR::BTIE;
    pub use super::MDMA_C0CR::CTCIE;
    pub use super::MDMA_C0CR::EN;
    pub use super::MDMA_C0CR::HEX;
    pub use super::MDMA_C0CR::PL;
    pub use super::MDMA_C0CR::SWRQ;
    pub use super::MDMA_C0CR::TCIE;
    pub use super::MDMA_C0CR::TEIE;
    pub use super::MDMA_C0CR::WEX;
}

/// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod MDMA_C31TCR {
    pub use super::MDMA_C0TCR::BWM;
    pub use super::MDMA_C0TCR::DBURST;
    pub use super::MDMA_C0TCR::DINC;
    pub use super::MDMA_C0TCR::DINCOS;
    pub use super::MDMA_C0TCR::DSIZE;
    pub use super::MDMA_C0TCR::PAM;
    pub use super::MDMA_C0TCR::PKE;
    pub use super::MDMA_C0TCR::SBURST;
    pub use super::MDMA_C0TCR::SINC;
    pub use super::MDMA_C0TCR::SINCOS;
    pub use super::MDMA_C0TCR::SSIZE;
    pub use super::MDMA_C0TCR::SWRM;
    pub use super::MDMA_C0TCR::TLEN;
    pub use super::MDMA_C0TCR::TRGM;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod MDMA_C31BNDTR {
    pub use super::MDMA_C0BNDTR::BNDT;
    pub use super::MDMA_C0BNDTR::BRC;
    pub use super::MDMA_C0BNDTR::BRDUM;
    pub use super::MDMA_C0BNDTR::BRSUM;
}

/// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod MDMA_C31SAR {
    pub use super::MDMA_C0SAR::SAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod MDMA_C31DAR {
    pub use super::MDMA_C0DAR::DAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod MDMA_C31BRUR {
    pub use super::MDMA_C0BRUR::DUV;
    pub use super::MDMA_C0BRUR::SUV;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod MDMA_C31LAR {
    pub use super::MDMA_C0LAR::LAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod MDMA_C31TBR {
    pub use super::MDMA_C0TBR::DBUS;
    pub use super::MDMA_C0TBR::SBUS;
    pub use super::MDMA_C0TBR::TSEL;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod MDMA_C31MAR {
    pub use super::MDMA_C0MAR::MAR;
}

/// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod MDMA_C31MDR {
    pub use super::MDMA_C0MDR::MDR;
}
#[repr(C)]
pub struct RegisterBlock {
    /// MDMA global interrupt/status register
    pub MDMA_GISR0: RORegister<u32>,

    _reserved1: [u8; 4],

    /// MDMA secure global interrupt/status register
    pub MDMA_SGISR0: RORegister<u32>,

    _reserved2: [u8; 52],

    /// MDMA channel 0 interrupt/status register
    pub MDMA_C0ISR: RORegister<u32>,

    /// MDMA channel 0 interrupt flag clear register
    pub MDMA_C0IFCR: WORegister<u32>,

    /// MDMA channel 0 error status register
    pub MDMA_C0ESR: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub MDMA_C0CR: RWRegister<u32>,

    /// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    pub MDMA_C0TCR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    pub MDMA_C0BNDTR: RWRegister<u32>,

    /// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    pub MDMA_C0SAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    pub MDMA_C0DAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    pub MDMA_C0BRUR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    pub MDMA_C0LAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    pub MDMA_C0TBR: RWRegister<u32>,

    _reserved3: [u8; 4],

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    pub MDMA_C0MAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    pub MDMA_C0MDR: RWRegister<u32>,

    _reserved4: [u8; 8],

    /// MDMA channel 1 interrupt/status register
    pub MDMA_C1ISR: RORegister<u32>,

    /// MDMA channel 1 interrupt flag clear register
    pub MDMA_C1IFCR: WORegister<u32>,

    /// MDMA channel 1 error status register
    pub MDMA_C1ESR: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub MDMA_C1CR: RWRegister<u32>,

    /// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    pub MDMA_C1TCR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    pub MDMA_C1BNDTR: RWRegister<u32>,

    /// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    pub MDMA_C1SAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    pub MDMA_C1DAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    pub MDMA_C1BRUR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    pub MDMA_C1LAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    pub MDMA_C1TBR: RWRegister<u32>,

    _reserved5: [u8; 4],

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    pub MDMA_C1MAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    pub MDMA_C1MDR: RWRegister<u32>,

    _reserved6: [u8; 8],

    /// MDMA channel 2 interrupt/status register
    pub MDMA_C2ISR: RORegister<u32>,

    /// MDMA channel 2 interrupt flag clear register
    pub MDMA_C2IFCR: WORegister<u32>,

    /// MDMA channel 2 error status register
    pub MDMA_C2ESR: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub MDMA_C2CR: RWRegister<u32>,

    /// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    pub MDMA_C2TCR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    pub MDMA_C2BNDTR: RWRegister<u32>,

    /// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    pub MDMA_C2SAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    pub MDMA_C2DAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    pub MDMA_C2BRUR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    pub MDMA_C2LAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    pub MDMA_C2TBR: RWRegister<u32>,

    _reserved7: [u8; 4],

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    pub MDMA_C2MAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    pub MDMA_C2MDR: RWRegister<u32>,

    _reserved8: [u8; 8],

    /// MDMA channel 3 interrupt/status register
    pub MDMA_C3ISR: RORegister<u32>,

    /// MDMA channel 3 interrupt flag clear register
    pub MDMA_C3IFCR: WORegister<u32>,

    /// MDMA channel 3 error status register
    pub MDMA_C3ESR: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub MDMA_C3CR: RWRegister<u32>,

    /// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    pub MDMA_C3TCR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    pub MDMA_C3BNDTR: RWRegister<u32>,

    /// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    pub MDMA_C3SAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    pub MDMA_C3DAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    pub MDMA_C3BRUR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    pub MDMA_C3LAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    pub MDMA_C3TBR: RWRegister<u32>,

    _reserved9: [u8; 4],

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    pub MDMA_C3MAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    pub MDMA_C3MDR: RWRegister<u32>,

    _reserved10: [u8; 8],

    /// MDMA channel 4 interrupt/status register
    pub MDMA_C4ISR: RORegister<u32>,

    /// MDMA channel 4 interrupt flag clear register
    pub MDMA_C4IFCR: WORegister<u32>,

    /// MDMA channel 4 error status register
    pub MDMA_C4ESR: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub MDMA_C4CR: RWRegister<u32>,

    /// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    pub MDMA_C4TCR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    pub MDMA_C4BNDTR: RWRegister<u32>,

    /// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    pub MDMA_C4SAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    pub MDMA_C4DAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    pub MDMA_C4BRUR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    pub MDMA_C4LAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    pub MDMA_C4TBR: RWRegister<u32>,

    _reserved11: [u8; 4],

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    pub MDMA_C4MAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    pub MDMA_C4MDR: RWRegister<u32>,

    _reserved12: [u8; 8],

    /// MDMA channel 5 interrupt/status register
    pub MDMA_C5ISR: RORegister<u32>,

    /// MDMA channel 5 interrupt flag clear register
    pub MDMA_C5IFCR: WORegister<u32>,

    /// MDMA channel 5 error status register
    pub MDMA_C5ESR: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub MDMA_C5CR: RWRegister<u32>,

    /// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    pub MDMA_C5TCR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    pub MDMA_C5BNDTR: RWRegister<u32>,

    /// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    pub MDMA_C5SAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    pub MDMA_C5DAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    pub MDMA_C5BRUR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    pub MDMA_C5LAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    pub MDMA_C5TBR: RWRegister<u32>,

    _reserved13: [u8; 4],

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    pub MDMA_C5MAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    pub MDMA_C5MDR: RWRegister<u32>,

    _reserved14: [u8; 8],

    /// MDMA channel 6 interrupt/status register
    pub MDMA_C6ISR: RORegister<u32>,

    /// MDMA channel 6 interrupt flag clear register
    pub MDMA_C6IFCR: WORegister<u32>,

    /// MDMA channel 6 error status register
    pub MDMA_C6ESR: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub MDMA_C6CR: RWRegister<u32>,

    /// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    pub MDMA_C6TCR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    pub MDMA_C6BNDTR: RWRegister<u32>,

    /// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    pub MDMA_C6SAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    pub MDMA_C6DAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    pub MDMA_C6BRUR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    pub MDMA_C6LAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    pub MDMA_C6TBR: RWRegister<u32>,

    _reserved15: [u8; 4],

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    pub MDMA_C6MAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    pub MDMA_C6MDR: RWRegister<u32>,

    _reserved16: [u8; 8],

    /// MDMA channel 7 interrupt/status register
    pub MDMA_C7ISR: RORegister<u32>,

    /// MDMA channel 7 interrupt flag clear register
    pub MDMA_C7IFCR: WORegister<u32>,

    /// MDMA channel 7 error status register
    pub MDMA_C7ESR: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub MDMA_C7CR: RWRegister<u32>,

    /// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    pub MDMA_C7TCR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    pub MDMA_C7BNDTR: RWRegister<u32>,

    /// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    pub MDMA_C7SAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    pub MDMA_C7DAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    pub MDMA_C7BRUR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    pub MDMA_C7LAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    pub MDMA_C7TBR: RWRegister<u32>,

    _reserved17: [u8; 4],

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    pub MDMA_C7MAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    pub MDMA_C7MDR: RWRegister<u32>,

    _reserved18: [u8; 8],

    /// MDMA channel 8 interrupt/status register
    pub MDMA_C8ISR: RORegister<u32>,

    /// MDMA channel 8 interrupt flag clear register
    pub MDMA_C8IFCR: WORegister<u32>,

    /// MDMA channel 8 error status register
    pub MDMA_C8ESR: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub MDMA_C8CR: RWRegister<u32>,

    /// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    pub MDMA_C8TCR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    pub MDMA_C8BNDTR: RWRegister<u32>,

    /// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    pub MDMA_C8SAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    pub MDMA_C8DAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    pub MDMA_C8BRUR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    pub MDMA_C8LAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    pub MDMA_C8TBR: RWRegister<u32>,

    _reserved19: [u8; 4],

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    pub MDMA_C8MAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    pub MDMA_C8MDR: RWRegister<u32>,

    _reserved20: [u8; 8],

    /// MDMA channel 9 interrupt/status register
    pub MDMA_C9ISR: RORegister<u32>,

    /// MDMA channel 9 interrupt flag clear register
    pub MDMA_C9IFCR: WORegister<u32>,

    /// MDMA channel 9 error status register
    pub MDMA_C9ESR: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub MDMA_C9CR: RWRegister<u32>,

    /// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    pub MDMA_C9TCR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    pub MDMA_C9BNDTR: RWRegister<u32>,

    /// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    pub MDMA_C9SAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    pub MDMA_C9DAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    pub MDMA_C9BRUR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    pub MDMA_C9LAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    pub MDMA_C9TBR: RWRegister<u32>,

    _reserved21: [u8; 4],

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    pub MDMA_C9MAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    pub MDMA_C9MDR: RWRegister<u32>,

    _reserved22: [u8; 8],

    /// MDMA channel 10 interrupt/status register
    pub MDMA_C10ISR: RORegister<u32>,

    /// MDMA channel 10 interrupt flag clear register
    pub MDMA_C10IFCR: WORegister<u32>,

    /// MDMA channel 10 error status register
    pub MDMA_C10ESR: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub MDMA_C10CR: RWRegister<u32>,

    /// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    pub MDMA_C10TCR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    pub MDMA_C10BNDTR: RWRegister<u32>,

    /// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    pub MDMA_C10SAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    pub MDMA_C10DAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    pub MDMA_C10BRUR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    pub MDMA_C10LAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    pub MDMA_C10TBR: RWRegister<u32>,

    _reserved23: [u8; 4],

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    pub MDMA_C10MAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    pub MDMA_C10MDR: RWRegister<u32>,

    _reserved24: [u8; 8],

    /// MDMA channel 11 interrupt/status register
    pub MDMA_C11ISR: RORegister<u32>,

    /// MDMA channel 11 interrupt flag clear register
    pub MDMA_C11IFCR: WORegister<u32>,

    /// MDMA channel 11 error status register
    pub MDMA_C11ESR: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub MDMA_C11CR: RWRegister<u32>,

    /// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    pub MDMA_C11TCR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    pub MDMA_C11BNDTR: RWRegister<u32>,

    /// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    pub MDMA_C11SAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    pub MDMA_C11DAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    pub MDMA_C11BRUR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    pub MDMA_C11LAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    pub MDMA_C11TBR: RWRegister<u32>,

    _reserved25: [u8; 4],

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    pub MDMA_C11MAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    pub MDMA_C11MDR: RWRegister<u32>,

    _reserved26: [u8; 8],

    /// MDMA channel 12 interrupt/status register
    pub MDMA_C12ISR: RORegister<u32>,

    /// MDMA channel 12 interrupt flag clear register
    pub MDMA_C12IFCR: WORegister<u32>,

    /// MDMA channel 12 error status register
    pub MDMA_C12ESR: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub MDMA_C12CR: RWRegister<u32>,

    /// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    pub MDMA_C12TCR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    pub MDMA_C12BNDTR: RWRegister<u32>,

    /// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    pub MDMA_C12SAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    pub MDMA_C12DAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    pub MDMA_C12BRUR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    pub MDMA_C12LAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    pub MDMA_C12TBR: RWRegister<u32>,

    _reserved27: [u8; 4],

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    pub MDMA_C12MAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    pub MDMA_C12MDR: RWRegister<u32>,

    _reserved28: [u8; 8],

    /// MDMA channel 13 interrupt/status register
    pub MDMA_C13ISR: RORegister<u32>,

    /// MDMA channel 13 interrupt flag clear register
    pub MDMA_C13IFCR: WORegister<u32>,

    /// MDMA channel 13 error status register
    pub MDMA_C13ESR: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub MDMA_C13CR: RWRegister<u32>,

    /// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    pub MDMA_C13TCR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    pub MDMA_C13BNDTR: RWRegister<u32>,

    /// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    pub MDMA_C13SAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    pub MDMA_C13DAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    pub MDMA_C13BRUR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    pub MDMA_C13LAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    pub MDMA_C13TBR: RWRegister<u32>,

    _reserved29: [u8; 4],

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    pub MDMA_C13MAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    pub MDMA_C13MDR: RWRegister<u32>,

    _reserved30: [u8; 8],

    /// MDMA channel 14 interrupt/status register
    pub MDMA_C14ISR: RORegister<u32>,

    /// MDMA channel 14 interrupt flag clear register
    pub MDMA_C14IFCR: WORegister<u32>,

    /// MDMA channel 14 error status register
    pub MDMA_C14ESR: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub MDMA_C14CR: RWRegister<u32>,

    /// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    pub MDMA_C14TCR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    pub MDMA_C14BNDTR: RWRegister<u32>,

    /// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    pub MDMA_C14SAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    pub MDMA_C14DAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    pub MDMA_C14BRUR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    pub MDMA_C14LAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    pub MDMA_C14TBR: RWRegister<u32>,

    _reserved31: [u8; 4],

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    pub MDMA_C14MAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    pub MDMA_C14MDR: RWRegister<u32>,

    _reserved32: [u8; 8],

    /// MDMA channel 15 interrupt/status register
    pub MDMA_C15ISR: RORegister<u32>,

    /// MDMA channel 15 interrupt flag clear register
    pub MDMA_C15IFCR: WORegister<u32>,

    /// MDMA channel 15 error status register
    pub MDMA_C15ESR: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub MDMA_C15CR: RWRegister<u32>,

    /// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    pub MDMA_C15TCR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    pub MDMA_C15BNDTR: RWRegister<u32>,

    /// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    pub MDMA_C15SAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    pub MDMA_C15DAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    pub MDMA_C15BRUR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    pub MDMA_C15LAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    pub MDMA_C15TBR: RWRegister<u32>,

    _reserved33: [u8; 4],

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    pub MDMA_C15MAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    pub MDMA_C15MDR: RWRegister<u32>,

    _reserved34: [u8; 8],

    /// MDMA channel 16 interrupt/status register
    pub MDMA_C16ISR: RORegister<u32>,

    /// MDMA channel 16 interrupt flag clear register
    pub MDMA_C16IFCR: WORegister<u32>,

    /// MDMA channel 16 error status register
    pub MDMA_C16ESR: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub MDMA_C16CR: RWRegister<u32>,

    /// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    pub MDMA_C16TCR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    pub MDMA_C16BNDTR: RWRegister<u32>,

    /// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    pub MDMA_C16SAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    pub MDMA_C16DAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    pub MDMA_C16BRUR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    pub MDMA_C16LAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    pub MDMA_C16TBR: RWRegister<u32>,

    _reserved35: [u8; 4],

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    pub MDMA_C16MAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    pub MDMA_C16MDR: RWRegister<u32>,

    _reserved36: [u8; 8],

    /// MDMA channel 17 interrupt/status register
    pub MDMA_C17ISR: RORegister<u32>,

    /// MDMA channel 17 interrupt flag clear register
    pub MDMA_C17IFCR: WORegister<u32>,

    /// MDMA channel 17 error status register
    pub MDMA_C17ESR: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub MDMA_C17CR: RWRegister<u32>,

    /// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    pub MDMA_C17TCR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    pub MDMA_C17BNDTR: RWRegister<u32>,

    /// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    pub MDMA_C17SAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    pub MDMA_C17DAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    pub MDMA_C17BRUR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    pub MDMA_C17LAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    pub MDMA_C17TBR: RWRegister<u32>,

    _reserved37: [u8; 4],

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    pub MDMA_C17MAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    pub MDMA_C17MDR: RWRegister<u32>,

    _reserved38: [u8; 8],

    /// MDMA channel 18 interrupt/status register
    pub MDMA_C18ISR: RORegister<u32>,

    /// MDMA channel 18 interrupt flag clear register
    pub MDMA_C18IFCR: WORegister<u32>,

    /// MDMA channel 18 error status register
    pub MDMA_C18ESR: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub MDMA_C18CR: RWRegister<u32>,

    /// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    pub MDMA_C18TCR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    pub MDMA_C18BNDTR: RWRegister<u32>,

    /// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    pub MDMA_C18SAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    pub MDMA_C18DAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    pub MDMA_C18BRUR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    pub MDMA_C18LAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    pub MDMA_C18TBR: RWRegister<u32>,

    _reserved39: [u8; 4],

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    pub MDMA_C18MAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    pub MDMA_C18MDR: RWRegister<u32>,

    _reserved40: [u8; 8],

    /// MDMA channel 19 interrupt/status register
    pub MDMA_C19ISR: RORegister<u32>,

    /// MDMA channel 19 interrupt flag clear register
    pub MDMA_C19IFCR: WORegister<u32>,

    /// MDMA channel 19 error status register
    pub MDMA_C19ESR: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub MDMA_C19CR: RWRegister<u32>,

    /// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    pub MDMA_C19TCR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    pub MDMA_C19BNDTR: RWRegister<u32>,

    /// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    pub MDMA_C19SAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    pub MDMA_C19DAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    pub MDMA_C19BRUR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    pub MDMA_C19LAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    pub MDMA_C19TBR: RWRegister<u32>,

    _reserved41: [u8; 4],

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    pub MDMA_C19MAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    pub MDMA_C19MDR: RWRegister<u32>,

    _reserved42: [u8; 8],

    /// MDMA channel 20 interrupt/status register
    pub MDMA_C20ISR: RORegister<u32>,

    /// MDMA channel 20 interrupt flag clear register
    pub MDMA_C20IFCR: WORegister<u32>,

    /// MDMA channel 20 error status register
    pub MDMA_C20ESR: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub MDMA_C20CR: RWRegister<u32>,

    /// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    pub MDMA_C20TCR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    pub MDMA_C20BNDTR: RWRegister<u32>,

    /// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    pub MDMA_C20SAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    pub MDMA_C20DAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    pub MDMA_C20BRUR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    pub MDMA_C20LAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    pub MDMA_C20TBR: RWRegister<u32>,

    _reserved43: [u8; 4],

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    pub MDMA_C20MAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    pub MDMA_C20MDR: RWRegister<u32>,

    _reserved44: [u8; 8],

    /// MDMA channel 21 interrupt/status register
    pub MDMA_C21ISR: RORegister<u32>,

    /// MDMA channel 21 interrupt flag clear register
    pub MDMA_C21IFCR: WORegister<u32>,

    /// MDMA channel 21 error status register
    pub MDMA_C21ESR: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub MDMA_C21CR: RWRegister<u32>,

    /// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    pub MDMA_C21TCR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    pub MDMA_C21BNDTR: RWRegister<u32>,

    /// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    pub MDMA_C21SAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    pub MDMA_C21DAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    pub MDMA_C21BRUR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    pub MDMA_C21LAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    pub MDMA_C21TBR: RWRegister<u32>,

    _reserved45: [u8; 4],

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    pub MDMA_C21MAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    pub MDMA_C21MDR: RWRegister<u32>,

    _reserved46: [u8; 8],

    /// MDMA channel 22 interrupt/status register
    pub MDMA_C22ISR: RORegister<u32>,

    /// MDMA channel 22 interrupt flag clear register
    pub MDMA_C22IFCR: WORegister<u32>,

    /// MDMA channel 22 error status register
    pub MDMA_C22ESR: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub MDMA_C22CR: RWRegister<u32>,

    /// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    pub MDMA_C22TCR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    pub MDMA_C22BNDTR: RWRegister<u32>,

    /// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    pub MDMA_C22SAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    pub MDMA_C22DAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    pub MDMA_C22BRUR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    pub MDMA_C22LAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    pub MDMA_C22TBR: RWRegister<u32>,

    _reserved47: [u8; 4],

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    pub MDMA_C22MAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    pub MDMA_C22MDR: RWRegister<u32>,

    _reserved48: [u8; 8],

    /// MDMA channel 23 interrupt/status register
    pub MDMA_C23ISR: RORegister<u32>,

    /// MDMA channel 23 interrupt flag clear register
    pub MDMA_C23IFCR: WORegister<u32>,

    /// MDMA channel 23 error status register
    pub MDMA_C23ESR: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub MDMA_C23CR: RWRegister<u32>,

    /// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    pub MDMA_C23TCR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    pub MDMA_C23BNDTR: RWRegister<u32>,

    /// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    pub MDMA_C23SAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    pub MDMA_C23DAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    pub MDMA_C23BRUR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    pub MDMA_C23LAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    pub MDMA_C23TBR: RWRegister<u32>,

    _reserved49: [u8; 4],

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    pub MDMA_C23MAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    pub MDMA_C23MDR: RWRegister<u32>,

    _reserved50: [u8; 8],

    /// MDMA channel 24 interrupt/status register
    pub MDMA_C24ISR: RORegister<u32>,

    /// MDMA channel 24 interrupt flag clear register
    pub MDMA_C24IFCR: WORegister<u32>,

    /// MDMA channel 24 error status register
    pub MDMA_C24ESR: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub MDMA_C24CR: RWRegister<u32>,

    /// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    pub MDMA_C24TCR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    pub MDMA_C24BNDTR: RWRegister<u32>,

    /// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    pub MDMA_C24SAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    pub MDMA_C24DAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    pub MDMA_C24BRUR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    pub MDMA_C24LAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    pub MDMA_C24TBR: RWRegister<u32>,

    _reserved51: [u8; 4],

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    pub MDMA_C24MAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    pub MDMA_C24MDR: RWRegister<u32>,

    _reserved52: [u8; 8],

    /// MDMA channel 25 interrupt/status register
    pub MDMA_C25ISR: RORegister<u32>,

    /// MDMA channel 25 interrupt flag clear register
    pub MDMA_C25IFCR: WORegister<u32>,

    /// MDMA channel 25 error status register
    pub MDMA_C25ESR: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub MDMA_C25CR: RWRegister<u32>,

    /// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    pub MDMA_C25TCR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    pub MDMA_C25BNDTR: RWRegister<u32>,

    /// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    pub MDMA_C25SAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    pub MDMA_C25DAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    pub MDMA_C25BRUR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    pub MDMA_C25LAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    pub MDMA_C25TBR: RWRegister<u32>,

    _reserved53: [u8; 4],

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    pub MDMA_C25MAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    pub MDMA_C25MDR: RWRegister<u32>,

    _reserved54: [u8; 8],

    /// MDMA channel 26 interrupt/status register
    pub MDMA_C26ISR: RORegister<u32>,

    /// MDMA channel 26 interrupt flag clear register
    pub MDMA_C26IFCR: WORegister<u32>,

    /// MDMA channel 26 error status register
    pub MDMA_C26ESR: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub MDMA_C26CR: RWRegister<u32>,

    /// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    pub MDMA_C26TCR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    pub MDMA_C26BNDTR: RWRegister<u32>,

    /// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    pub MDMA_C26SAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    pub MDMA_C26DAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    pub MDMA_C26BRUR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    pub MDMA_C26LAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    pub MDMA_C26TBR: RWRegister<u32>,

    _reserved55: [u8; 4],

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    pub MDMA_C26MAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    pub MDMA_C26MDR: RWRegister<u32>,

    _reserved56: [u8; 8],

    /// MDMA channel 27 interrupt/status register
    pub MDMA_C27ISR: RORegister<u32>,

    /// MDMA channel 27 interrupt flag clear register
    pub MDMA_C27IFCR: WORegister<u32>,

    /// MDMA channel 27 error status register
    pub MDMA_C27ESR: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub MDMA_C27CR: RWRegister<u32>,

    /// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    pub MDMA_C27TCR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    pub MDMA_C27BNDTR: RWRegister<u32>,

    /// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    pub MDMA_C27SAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    pub MDMA_C27DAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    pub MDMA_C27BRUR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    pub MDMA_C27LAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    pub MDMA_C27TBR: RWRegister<u32>,

    _reserved57: [u8; 4],

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    pub MDMA_C27MAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    pub MDMA_C27MDR: RWRegister<u32>,

    _reserved58: [u8; 8],

    /// MDMA channel 28 interrupt/status register
    pub MDMA_C28ISR: RORegister<u32>,

    /// MDMA channel 28 interrupt flag clear register
    pub MDMA_C28IFCR: WORegister<u32>,

    /// MDMA channel 28 error status register
    pub MDMA_C28ESR: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub MDMA_C28CR: RWRegister<u32>,

    /// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    pub MDMA_C28TCR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    pub MDMA_C28BNDTR: RWRegister<u32>,

    /// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    pub MDMA_C28SAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    pub MDMA_C28DAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    pub MDMA_C28BRUR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    pub MDMA_C28LAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    pub MDMA_C28TBR: RWRegister<u32>,

    _reserved59: [u8; 4],

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    pub MDMA_C28MAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    pub MDMA_C28MDR: RWRegister<u32>,

    _reserved60: [u8; 8],

    /// MDMA channel 29 interrupt/status register
    pub MDMA_C29ISR: RORegister<u32>,

    /// MDMA channel 29 interrupt flag clear register
    pub MDMA_C29IFCR: WORegister<u32>,

    /// MDMA channel 29 error status register
    pub MDMA_C29ESR: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub MDMA_C29CR: RWRegister<u32>,

    /// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    pub MDMA_C29TCR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    pub MDMA_C29BNDTR: RWRegister<u32>,

    /// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    pub MDMA_C29SAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    pub MDMA_C29DAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    pub MDMA_C29BRUR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    pub MDMA_C29LAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    pub MDMA_C29TBR: RWRegister<u32>,

    _reserved61: [u8; 4],

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    pub MDMA_C29MAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    pub MDMA_C29MDR: RWRegister<u32>,

    _reserved62: [u8; 8],

    /// MDMA channel 30 interrupt/status register
    pub MDMA_C30ISR: RORegister<u32>,

    /// MDMA channel 30 interrupt flag clear register
    pub MDMA_C30IFCR: WORegister<u32>,

    /// MDMA channel 30 error status register
    pub MDMA_C30ESR: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub MDMA_C30CR: RWRegister<u32>,

    /// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    pub MDMA_C30TCR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    pub MDMA_C30BNDTR: RWRegister<u32>,

    /// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    pub MDMA_C30SAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    pub MDMA_C30DAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    pub MDMA_C30BRUR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    pub MDMA_C30LAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    pub MDMA_C30TBR: RWRegister<u32>,

    _reserved63: [u8; 4],

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    pub MDMA_C30MAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    pub MDMA_C30MDR: RWRegister<u32>,

    _reserved64: [u8; 8],

    /// MDMA channel 31 interrupt/status register
    pub MDMA_C31ISR: RORegister<u32>,

    /// MDMA channel 31 interrupt flag clear register
    pub MDMA_C31IFCR: WORegister<u32>,

    /// MDMA channel 31 error status register
    pub MDMA_C31ESR: RORegister<u32>,

    /// This register is used to control the concerned channel.
    pub MDMA_C31CR: RWRegister<u32>,

    /// This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    pub MDMA_C31TCR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    pub MDMA_C31BNDTR: RWRegister<u32>,

    /// In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    pub MDMA_C31SAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    pub MDMA_C31DAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    pub MDMA_C31BRUR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    pub MDMA_C31LAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    pub MDMA_C31TBR: RWRegister<u32>,

    _reserved65: [u8; 4],

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    pub MDMA_C31MAR: RWRegister<u32>,

    /// In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    pub MDMA_C31MDR: RWRegister<u32>,
}
pub struct ResetValues {
    pub MDMA_GISR0: u32,
    pub MDMA_SGISR0: u32,
    pub MDMA_C0ISR: u32,
    pub MDMA_C0IFCR: u32,
    pub MDMA_C0ESR: u32,
    pub MDMA_C0CR: u32,
    pub MDMA_C0TCR: u32,
    pub MDMA_C0BNDTR: u32,
    pub MDMA_C0SAR: u32,
    pub MDMA_C0DAR: u32,
    pub MDMA_C0BRUR: u32,
    pub MDMA_C0LAR: u32,
    pub MDMA_C0TBR: u32,
    pub MDMA_C0MAR: u32,
    pub MDMA_C0MDR: u32,
    pub MDMA_C1ISR: u32,
    pub MDMA_C1IFCR: u32,
    pub MDMA_C1ESR: u32,
    pub MDMA_C1CR: u32,
    pub MDMA_C1TCR: u32,
    pub MDMA_C1BNDTR: u32,
    pub MDMA_C1SAR: u32,
    pub MDMA_C1DAR: u32,
    pub MDMA_C1BRUR: u32,
    pub MDMA_C1LAR: u32,
    pub MDMA_C1TBR: u32,
    pub MDMA_C1MAR: u32,
    pub MDMA_C1MDR: u32,
    pub MDMA_C2ISR: u32,
    pub MDMA_C2IFCR: u32,
    pub MDMA_C2ESR: u32,
    pub MDMA_C2CR: u32,
    pub MDMA_C2TCR: u32,
    pub MDMA_C2BNDTR: u32,
    pub MDMA_C2SAR: u32,
    pub MDMA_C2DAR: u32,
    pub MDMA_C2BRUR: u32,
    pub MDMA_C2LAR: u32,
    pub MDMA_C2TBR: u32,
    pub MDMA_C2MAR: u32,
    pub MDMA_C2MDR: u32,
    pub MDMA_C3ISR: u32,
    pub MDMA_C3IFCR: u32,
    pub MDMA_C3ESR: u32,
    pub MDMA_C3CR: u32,
    pub MDMA_C3TCR: u32,
    pub MDMA_C3BNDTR: u32,
    pub MDMA_C3SAR: u32,
    pub MDMA_C3DAR: u32,
    pub MDMA_C3BRUR: u32,
    pub MDMA_C3LAR: u32,
    pub MDMA_C3TBR: u32,
    pub MDMA_C3MAR: u32,
    pub MDMA_C3MDR: u32,
    pub MDMA_C4ISR: u32,
    pub MDMA_C4IFCR: u32,
    pub MDMA_C4ESR: u32,
    pub MDMA_C4CR: u32,
    pub MDMA_C4TCR: u32,
    pub MDMA_C4BNDTR: u32,
    pub MDMA_C4SAR: u32,
    pub MDMA_C4DAR: u32,
    pub MDMA_C4BRUR: u32,
    pub MDMA_C4LAR: u32,
    pub MDMA_C4TBR: u32,
    pub MDMA_C4MAR: u32,
    pub MDMA_C4MDR: u32,
    pub MDMA_C5ISR: u32,
    pub MDMA_C5IFCR: u32,
    pub MDMA_C5ESR: u32,
    pub MDMA_C5CR: u32,
    pub MDMA_C5TCR: u32,
    pub MDMA_C5BNDTR: u32,
    pub MDMA_C5SAR: u32,
    pub MDMA_C5DAR: u32,
    pub MDMA_C5BRUR: u32,
    pub MDMA_C5LAR: u32,
    pub MDMA_C5TBR: u32,
    pub MDMA_C5MAR: u32,
    pub MDMA_C5MDR: u32,
    pub MDMA_C6ISR: u32,
    pub MDMA_C6IFCR: u32,
    pub MDMA_C6ESR: u32,
    pub MDMA_C6CR: u32,
    pub MDMA_C6TCR: u32,
    pub MDMA_C6BNDTR: u32,
    pub MDMA_C6SAR: u32,
    pub MDMA_C6DAR: u32,
    pub MDMA_C6BRUR: u32,
    pub MDMA_C6LAR: u32,
    pub MDMA_C6TBR: u32,
    pub MDMA_C6MAR: u32,
    pub MDMA_C6MDR: u32,
    pub MDMA_C7ISR: u32,
    pub MDMA_C7IFCR: u32,
    pub MDMA_C7ESR: u32,
    pub MDMA_C7CR: u32,
    pub MDMA_C7TCR: u32,
    pub MDMA_C7BNDTR: u32,
    pub MDMA_C7SAR: u32,
    pub MDMA_C7DAR: u32,
    pub MDMA_C7BRUR: u32,
    pub MDMA_C7LAR: u32,
    pub MDMA_C7TBR: u32,
    pub MDMA_C7MAR: u32,
    pub MDMA_C7MDR: u32,
    pub MDMA_C8ISR: u32,
    pub MDMA_C8IFCR: u32,
    pub MDMA_C8ESR: u32,
    pub MDMA_C8CR: u32,
    pub MDMA_C8TCR: u32,
    pub MDMA_C8BNDTR: u32,
    pub MDMA_C8SAR: u32,
    pub MDMA_C8DAR: u32,
    pub MDMA_C8BRUR: u32,
    pub MDMA_C8LAR: u32,
    pub MDMA_C8TBR: u32,
    pub MDMA_C8MAR: u32,
    pub MDMA_C8MDR: u32,
    pub MDMA_C9ISR: u32,
    pub MDMA_C9IFCR: u32,
    pub MDMA_C9ESR: u32,
    pub MDMA_C9CR: u32,
    pub MDMA_C9TCR: u32,
    pub MDMA_C9BNDTR: u32,
    pub MDMA_C9SAR: u32,
    pub MDMA_C9DAR: u32,
    pub MDMA_C9BRUR: u32,
    pub MDMA_C9LAR: u32,
    pub MDMA_C9TBR: u32,
    pub MDMA_C9MAR: u32,
    pub MDMA_C9MDR: u32,
    pub MDMA_C10ISR: u32,
    pub MDMA_C10IFCR: u32,
    pub MDMA_C10ESR: u32,
    pub MDMA_C10CR: u32,
    pub MDMA_C10TCR: u32,
    pub MDMA_C10BNDTR: u32,
    pub MDMA_C10SAR: u32,
    pub MDMA_C10DAR: u32,
    pub MDMA_C10BRUR: u32,
    pub MDMA_C10LAR: u32,
    pub MDMA_C10TBR: u32,
    pub MDMA_C10MAR: u32,
    pub MDMA_C10MDR: u32,
    pub MDMA_C11ISR: u32,
    pub MDMA_C11IFCR: u32,
    pub MDMA_C11ESR: u32,
    pub MDMA_C11CR: u32,
    pub MDMA_C11TCR: u32,
    pub MDMA_C11BNDTR: u32,
    pub MDMA_C11SAR: u32,
    pub MDMA_C11DAR: u32,
    pub MDMA_C11BRUR: u32,
    pub MDMA_C11LAR: u32,
    pub MDMA_C11TBR: u32,
    pub MDMA_C11MAR: u32,
    pub MDMA_C11MDR: u32,
    pub MDMA_C12ISR: u32,
    pub MDMA_C12IFCR: u32,
    pub MDMA_C12ESR: u32,
    pub MDMA_C12CR: u32,
    pub MDMA_C12TCR: u32,
    pub MDMA_C12BNDTR: u32,
    pub MDMA_C12SAR: u32,
    pub MDMA_C12DAR: u32,
    pub MDMA_C12BRUR: u32,
    pub MDMA_C12LAR: u32,
    pub MDMA_C12TBR: u32,
    pub MDMA_C12MAR: u32,
    pub MDMA_C12MDR: u32,
    pub MDMA_C13ISR: u32,
    pub MDMA_C13IFCR: u32,
    pub MDMA_C13ESR: u32,
    pub MDMA_C13CR: u32,
    pub MDMA_C13TCR: u32,
    pub MDMA_C13BNDTR: u32,
    pub MDMA_C13SAR: u32,
    pub MDMA_C13DAR: u32,
    pub MDMA_C13BRUR: u32,
    pub MDMA_C13LAR: u32,
    pub MDMA_C13TBR: u32,
    pub MDMA_C13MAR: u32,
    pub MDMA_C13MDR: u32,
    pub MDMA_C14ISR: u32,
    pub MDMA_C14IFCR: u32,
    pub MDMA_C14ESR: u32,
    pub MDMA_C14CR: u32,
    pub MDMA_C14TCR: u32,
    pub MDMA_C14BNDTR: u32,
    pub MDMA_C14SAR: u32,
    pub MDMA_C14DAR: u32,
    pub MDMA_C14BRUR: u32,
    pub MDMA_C14LAR: u32,
    pub MDMA_C14TBR: u32,
    pub MDMA_C14MAR: u32,
    pub MDMA_C14MDR: u32,
    pub MDMA_C15ISR: u32,
    pub MDMA_C15IFCR: u32,
    pub MDMA_C15ESR: u32,
    pub MDMA_C15CR: u32,
    pub MDMA_C15TCR: u32,
    pub MDMA_C15BNDTR: u32,
    pub MDMA_C15SAR: u32,
    pub MDMA_C15DAR: u32,
    pub MDMA_C15BRUR: u32,
    pub MDMA_C15LAR: u32,
    pub MDMA_C15TBR: u32,
    pub MDMA_C15MAR: u32,
    pub MDMA_C15MDR: u32,
    pub MDMA_C16ISR: u32,
    pub MDMA_C16IFCR: u32,
    pub MDMA_C16ESR: u32,
    pub MDMA_C16CR: u32,
    pub MDMA_C16TCR: u32,
    pub MDMA_C16BNDTR: u32,
    pub MDMA_C16SAR: u32,
    pub MDMA_C16DAR: u32,
    pub MDMA_C16BRUR: u32,
    pub MDMA_C16LAR: u32,
    pub MDMA_C16TBR: u32,
    pub MDMA_C16MAR: u32,
    pub MDMA_C16MDR: u32,
    pub MDMA_C17ISR: u32,
    pub MDMA_C17IFCR: u32,
    pub MDMA_C17ESR: u32,
    pub MDMA_C17CR: u32,
    pub MDMA_C17TCR: u32,
    pub MDMA_C17BNDTR: u32,
    pub MDMA_C17SAR: u32,
    pub MDMA_C17DAR: u32,
    pub MDMA_C17BRUR: u32,
    pub MDMA_C17LAR: u32,
    pub MDMA_C17TBR: u32,
    pub MDMA_C17MAR: u32,
    pub MDMA_C17MDR: u32,
    pub MDMA_C18ISR: u32,
    pub MDMA_C18IFCR: u32,
    pub MDMA_C18ESR: u32,
    pub MDMA_C18CR: u32,
    pub MDMA_C18TCR: u32,
    pub MDMA_C18BNDTR: u32,
    pub MDMA_C18SAR: u32,
    pub MDMA_C18DAR: u32,
    pub MDMA_C18BRUR: u32,
    pub MDMA_C18LAR: u32,
    pub MDMA_C18TBR: u32,
    pub MDMA_C18MAR: u32,
    pub MDMA_C18MDR: u32,
    pub MDMA_C19ISR: u32,
    pub MDMA_C19IFCR: u32,
    pub MDMA_C19ESR: u32,
    pub MDMA_C19CR: u32,
    pub MDMA_C19TCR: u32,
    pub MDMA_C19BNDTR: u32,
    pub MDMA_C19SAR: u32,
    pub MDMA_C19DAR: u32,
    pub MDMA_C19BRUR: u32,
    pub MDMA_C19LAR: u32,
    pub MDMA_C19TBR: u32,
    pub MDMA_C19MAR: u32,
    pub MDMA_C19MDR: u32,
    pub MDMA_C20ISR: u32,
    pub MDMA_C20IFCR: u32,
    pub MDMA_C20ESR: u32,
    pub MDMA_C20CR: u32,
    pub MDMA_C20TCR: u32,
    pub MDMA_C20BNDTR: u32,
    pub MDMA_C20SAR: u32,
    pub MDMA_C20DAR: u32,
    pub MDMA_C20BRUR: u32,
    pub MDMA_C20LAR: u32,
    pub MDMA_C20TBR: u32,
    pub MDMA_C20MAR: u32,
    pub MDMA_C20MDR: u32,
    pub MDMA_C21ISR: u32,
    pub MDMA_C21IFCR: u32,
    pub MDMA_C21ESR: u32,
    pub MDMA_C21CR: u32,
    pub MDMA_C21TCR: u32,
    pub MDMA_C21BNDTR: u32,
    pub MDMA_C21SAR: u32,
    pub MDMA_C21DAR: u32,
    pub MDMA_C21BRUR: u32,
    pub MDMA_C21LAR: u32,
    pub MDMA_C21TBR: u32,
    pub MDMA_C21MAR: u32,
    pub MDMA_C21MDR: u32,
    pub MDMA_C22ISR: u32,
    pub MDMA_C22IFCR: u32,
    pub MDMA_C22ESR: u32,
    pub MDMA_C22CR: u32,
    pub MDMA_C22TCR: u32,
    pub MDMA_C22BNDTR: u32,
    pub MDMA_C22SAR: u32,
    pub MDMA_C22DAR: u32,
    pub MDMA_C22BRUR: u32,
    pub MDMA_C22LAR: u32,
    pub MDMA_C22TBR: u32,
    pub MDMA_C22MAR: u32,
    pub MDMA_C22MDR: u32,
    pub MDMA_C23ISR: u32,
    pub MDMA_C23IFCR: u32,
    pub MDMA_C23ESR: u32,
    pub MDMA_C23CR: u32,
    pub MDMA_C23TCR: u32,
    pub MDMA_C23BNDTR: u32,
    pub MDMA_C23SAR: u32,
    pub MDMA_C23DAR: u32,
    pub MDMA_C23BRUR: u32,
    pub MDMA_C23LAR: u32,
    pub MDMA_C23TBR: u32,
    pub MDMA_C23MAR: u32,
    pub MDMA_C23MDR: u32,
    pub MDMA_C24ISR: u32,
    pub MDMA_C24IFCR: u32,
    pub MDMA_C24ESR: u32,
    pub MDMA_C24CR: u32,
    pub MDMA_C24TCR: u32,
    pub MDMA_C24BNDTR: u32,
    pub MDMA_C24SAR: u32,
    pub MDMA_C24DAR: u32,
    pub MDMA_C24BRUR: u32,
    pub MDMA_C24LAR: u32,
    pub MDMA_C24TBR: u32,
    pub MDMA_C24MAR: u32,
    pub MDMA_C24MDR: u32,
    pub MDMA_C25ISR: u32,
    pub MDMA_C25IFCR: u32,
    pub MDMA_C25ESR: u32,
    pub MDMA_C25CR: u32,
    pub MDMA_C25TCR: u32,
    pub MDMA_C25BNDTR: u32,
    pub MDMA_C25SAR: u32,
    pub MDMA_C25DAR: u32,
    pub MDMA_C25BRUR: u32,
    pub MDMA_C25LAR: u32,
    pub MDMA_C25TBR: u32,
    pub MDMA_C25MAR: u32,
    pub MDMA_C25MDR: u32,
    pub MDMA_C26ISR: u32,
    pub MDMA_C26IFCR: u32,
    pub MDMA_C26ESR: u32,
    pub MDMA_C26CR: u32,
    pub MDMA_C26TCR: u32,
    pub MDMA_C26BNDTR: u32,
    pub MDMA_C26SAR: u32,
    pub MDMA_C26DAR: u32,
    pub MDMA_C26BRUR: u32,
    pub MDMA_C26LAR: u32,
    pub MDMA_C26TBR: u32,
    pub MDMA_C26MAR: u32,
    pub MDMA_C26MDR: u32,
    pub MDMA_C27ISR: u32,
    pub MDMA_C27IFCR: u32,
    pub MDMA_C27ESR: u32,
    pub MDMA_C27CR: u32,
    pub MDMA_C27TCR: u32,
    pub MDMA_C27BNDTR: u32,
    pub MDMA_C27SAR: u32,
    pub MDMA_C27DAR: u32,
    pub MDMA_C27BRUR: u32,
    pub MDMA_C27LAR: u32,
    pub MDMA_C27TBR: u32,
    pub MDMA_C27MAR: u32,
    pub MDMA_C27MDR: u32,
    pub MDMA_C28ISR: u32,
    pub MDMA_C28IFCR: u32,
    pub MDMA_C28ESR: u32,
    pub MDMA_C28CR: u32,
    pub MDMA_C28TCR: u32,
    pub MDMA_C28BNDTR: u32,
    pub MDMA_C28SAR: u32,
    pub MDMA_C28DAR: u32,
    pub MDMA_C28BRUR: u32,
    pub MDMA_C28LAR: u32,
    pub MDMA_C28TBR: u32,
    pub MDMA_C28MAR: u32,
    pub MDMA_C28MDR: u32,
    pub MDMA_C29ISR: u32,
    pub MDMA_C29IFCR: u32,
    pub MDMA_C29ESR: u32,
    pub MDMA_C29CR: u32,
    pub MDMA_C29TCR: u32,
    pub MDMA_C29BNDTR: u32,
    pub MDMA_C29SAR: u32,
    pub MDMA_C29DAR: u32,
    pub MDMA_C29BRUR: u32,
    pub MDMA_C29LAR: u32,
    pub MDMA_C29TBR: u32,
    pub MDMA_C29MAR: u32,
    pub MDMA_C29MDR: u32,
    pub MDMA_C30ISR: u32,
    pub MDMA_C30IFCR: u32,
    pub MDMA_C30ESR: u32,
    pub MDMA_C30CR: u32,
    pub MDMA_C30TCR: u32,
    pub MDMA_C30BNDTR: u32,
    pub MDMA_C30SAR: u32,
    pub MDMA_C30DAR: u32,
    pub MDMA_C30BRUR: u32,
    pub MDMA_C30LAR: u32,
    pub MDMA_C30TBR: u32,
    pub MDMA_C30MAR: u32,
    pub MDMA_C30MDR: u32,
    pub MDMA_C31ISR: u32,
    pub MDMA_C31IFCR: u32,
    pub MDMA_C31ESR: u32,
    pub MDMA_C31CR: u32,
    pub MDMA_C31TCR: u32,
    pub MDMA_C31BNDTR: u32,
    pub MDMA_C31SAR: u32,
    pub MDMA_C31DAR: u32,
    pub MDMA_C31BRUR: u32,
    pub MDMA_C31LAR: u32,
    pub MDMA_C31TBR: u32,
    pub MDMA_C31MAR: u32,
    pub MDMA_C31MDR: u32,
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
#[cfg(feature = "rtic")]
unsafe impl Send for Instance {}
