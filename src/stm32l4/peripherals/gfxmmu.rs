#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Graphic MMU
//!
//! Used by: stm32l4r5, stm32l4r9

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Graphic MMU configuration register
pub mod CR {

    /// Buffer 0 overflow interrupt enable
    pub mod B0OIE {
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

    /// Buffer 1 overflow interrupt enable
    pub mod B1OIE {
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

    /// Buffer 2 overflow interrupt enable
    pub mod B2OIE {
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

    /// Buffer 3 overflow interrupt enable
    pub mod B3OIE {
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

    /// AHB master error interrupt enable
    pub mod AMEIE {
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

    /// 192 Block mode
    pub mod BM192 {
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
}

/// Graphic MMU status register
pub mod SR {

    /// Buffer 0 overflow flag
    pub mod B0OF {
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

    /// Buffer 1 overflow flag
    pub mod B1OF {
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

    /// Buffer 2 overflow flag
    pub mod B2OF {
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

    /// Buffer 3 overflow flag
    pub mod B3OF {
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

    /// AHB master error flag
    pub mod AMEF {
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

/// Graphic MMU flag clear register
pub mod FCR {

    /// Clear buffer 0 overflow flag
    pub mod CB0OF {
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

    /// Clear buffer 1 overflow flag
    pub mod CB1OF {
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

    /// Clear buffer 2 overflow flag
    pub mod CB2OF {
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

    /// Clear buffer 3 overflow flag
    pub mod CB3OF {
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

    /// Clear AHB master error flag
    pub mod CAMEF {
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

/// Graphic MMU default value register
pub mod DVR {

    /// Default value
    pub mod DV {
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

/// Graphic MMU buffer 0 configuration register
pub mod B0CR {

    /// Physical buffer offset
    pub mod PBO {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (19 bits: 0x7ffff << 4)
        pub const mask: u32 = 0x7ffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Physical buffer base address
    pub mod PBBA {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (9 bits: 0x1ff << 23)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Graphic MMU buffer 1 configuration register
pub mod B1CR {
    pub use super::B0CR::PBBA;
    pub use super::B0CR::PBO;
}

/// Graphic MMU buffer 2 configuration register
pub mod B2CR {
    pub use super::B0CR::PBBA;
    pub use super::B0CR::PBO;
}

/// Graphic MMU buffer 3 configuration register
pub mod B3CR {
    pub use super::B0CR::PBBA;
    pub use super::B0CR::PBO;
}

/// Graphic MMU version register
pub mod VERR {

    /// Minor revision
    pub mod MINREV {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Major revision
    pub mod MAJREV {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Graphic MMU identification register
pub mod IPIDR {

    /// Identification Code
    pub mod ID {
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

/// Graphic MMU size identification register
pub mod SIDR {

    /// Size and ID
    pub mod SID {
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

/// Graphic MMU LUT entry 0 low
pub mod LUT0L {

    /// Enable
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

    /// First Valid Block
    pub mod FVB {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Last Valid Block
    pub mod LVB {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Graphic MMU LUT entry 1 low
pub mod LUT1L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 2 low
pub mod LUT2L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 3 low
pub mod LUT3L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 4 low
pub mod LUT4L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 5 low
pub mod LUT5L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 6 low
pub mod LUT6L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 7 low
pub mod LUT7L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 8 low
pub mod LUT8L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 9 low
pub mod LUT9L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 10 low
pub mod LUT10L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 11 low
pub mod LUT11L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 12 low
pub mod LUT12L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 13 low
pub mod LUT13L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 14 low
pub mod LUT14L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 15 low
pub mod LUT15L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 16 low
pub mod LUT16L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 17 low
pub mod LUT17L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 18 low
pub mod LUT18L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 19 low
pub mod LUT19L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 20 low
pub mod LUT20L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 21 low
pub mod LUT21L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 22 low
pub mod LUT22L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 23 low
pub mod LUT23L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 24 low
pub mod LUT24L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 25 low
pub mod LUT25L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 26 low
pub mod LUT26L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 27 low
pub mod LUT27L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 28 low
pub mod LUT28L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 29 low
pub mod LUT29L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 30 low
pub mod LUT30L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 31 low
pub mod LUT31L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 32 low
pub mod LUT32L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 33 low
pub mod LUT33L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 34 low
pub mod LUT34L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 35 low
pub mod LUT35L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 36 low
pub mod LUT36L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 37 low
pub mod LUT37L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 38 low
pub mod LUT38L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 39 low
pub mod LUT39L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 40 low
pub mod LUT40L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 41 low
pub mod LUT41L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 42 low
pub mod LUT42L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 43 low
pub mod LUT43L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 44 low
pub mod LUT44L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 45 low
pub mod LUT45L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 46 low
pub mod LUT46L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 47 low
pub mod LUT47L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 48 low
pub mod LUT48L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 49 low
pub mod LUT49L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 50 low
pub mod LUT50L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 51 low
pub mod LUT51L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 52 low
pub mod LUT52L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 53 low
pub mod LUT53L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 54 low
pub mod LUT54L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 55 low
pub mod LUT55L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 56 low
pub mod LUT56L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 57 low
pub mod LUT57L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 58 low
pub mod LUT58L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 59 low
pub mod LUT59L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 60 low
pub mod LUT60L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 61 low
pub mod LUT61L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 62 low
pub mod LUT62L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 63 low
pub mod LUT63L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 64 low
pub mod LUT64L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 65 low
pub mod LUT65L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 66 low
pub mod LUT66L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 67 low
pub mod LUT67L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 68 low
pub mod LUT68L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 69 low
pub mod LUT69L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 70 low
pub mod LUT70L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 71 low
pub mod LUT71L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 72 low
pub mod LUT72L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 73 low
pub mod LUT73L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 74 low
pub mod LUT74L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 75 low
pub mod LUT75L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 76 low
pub mod LUT76L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 77 low
pub mod LUT77L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 78 low
pub mod LUT78L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 79 low
pub mod LUT79L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 80 low
pub mod LUT80L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 81 low
pub mod LUT81L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 82 low
pub mod LUT82L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 83 low
pub mod LUT83L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 84 low
pub mod LUT84L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 85 low
pub mod LUT85L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 86 low
pub mod LUT86L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 87 low
pub mod LUT87L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 88 low
pub mod LUT88L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 89 low
pub mod LUT89L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 90 low
pub mod LUT90L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 91 low
pub mod LUT91L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 92 low
pub mod LUT92L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 93 low
pub mod LUT93L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 94 low
pub mod LUT94L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 95 low
pub mod LUT95L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 96 low
pub mod LUT96L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 97 low
pub mod LUT97L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 98 low
pub mod LUT98L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 99 low
pub mod LUT99L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 100 low
pub mod LUT100L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 101 low
pub mod LUT101L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 102 low
pub mod LUT102L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 103 low
pub mod LUT103L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 104 low
pub mod LUT104L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 105 low
pub mod LUT105L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 106 low
pub mod LUT106L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 107 low
pub mod LUT107L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 108 low
pub mod LUT108L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 109 low
pub mod LUT109L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 110 low
pub mod LUT110L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 111 low
pub mod LUT111L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 112 low
pub mod LUT112L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 113 low
pub mod LUT113L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 114 low
pub mod LUT114L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 115 low
pub mod LUT115L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 116 low
pub mod LUT116L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 117 low
pub mod LUT117L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 118 low
pub mod LUT118L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 119 low
pub mod LUT119L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 120 low
pub mod LUT120L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 121 low
pub mod LUT121L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 122 low
pub mod LUT122L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 123 low
pub mod LUT123L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 124 low
pub mod LUT124L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 125 low
pub mod LUT125L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 126 low
pub mod LUT126L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 127 low
pub mod LUT127L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 128 low
pub mod LUT128L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 129 low
pub mod LUT129L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 130 low
pub mod LUT130L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 131 low
pub mod LUT131L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 132 low
pub mod LUT132L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 133 low
pub mod LUT133L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 134 low
pub mod LUT134L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 135 low
pub mod LUT135L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 136 low
pub mod LUT136L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 137 low
pub mod LUT137L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 138 low
pub mod LUT138L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 139 low
pub mod LUT139L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 140 low
pub mod LUT140L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 141 low
pub mod LUT141L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 142 low
pub mod LUT142L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 143 low
pub mod LUT143L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 144 low
pub mod LUT144L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 145 low
pub mod LUT145L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 146 low
pub mod LUT146L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 147 low
pub mod LUT147L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 148 low
pub mod LUT148L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 149 low
pub mod LUT149L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 150 low
pub mod LUT150L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 151 low
pub mod LUT151L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 152 low
pub mod LUT152L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 153 low
pub mod LUT153L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 154 low
pub mod LUT154L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 155 low
pub mod LUT155L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 156 low
pub mod LUT156L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 157 low
pub mod LUT157L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 158 low
pub mod LUT158L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 159 low
pub mod LUT159L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 160 low
pub mod LUT160L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 161 low
pub mod LUT161L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 162 low
pub mod LUT162L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 163 low
pub mod LUT163L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 164 low
pub mod LUT164L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 165 low
pub mod LUT165L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 166 low
pub mod LUT166L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 167 low
pub mod LUT167L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 168 low
pub mod LUT168L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 169 low
pub mod LUT169L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 170 low
pub mod LUT170L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 171 low
pub mod LUT171L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 172 low
pub mod LUT172L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 173 low
pub mod LUT173L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 174 low
pub mod LUT174L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 175 low
pub mod LUT175L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 176 low
pub mod LUT176L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 177 low
pub mod LUT177L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 178 low
pub mod LUT178L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 179 low
pub mod LUT179L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 180 low
pub mod LUT180L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 181 low
pub mod LUT181L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 182 low
pub mod LUT182L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 183 low
pub mod LUT183L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 184 low
pub mod LUT184L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 185 low
pub mod LUT185L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 186 low
pub mod LUT186L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 187 low
pub mod LUT187L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 188 low
pub mod LUT188L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 189 low
pub mod LUT189L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 190 low
pub mod LUT190L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 191 low
pub mod LUT191L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 192 low
pub mod LUT192L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 193 low
pub mod LUT193L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 194 low
pub mod LUT194L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 195 low
pub mod LUT195L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 196 low
pub mod LUT196L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 197 low
pub mod LUT197L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 198 low
pub mod LUT198L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 199 low
pub mod LUT199L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 200 low
pub mod LUT200L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 201 low
pub mod LUT201L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 202 low
pub mod LUT202L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 203 low
pub mod LUT203L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 204 low
pub mod LUT204L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 205 low
pub mod LUT205L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 206 low
pub mod LUT206L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 207 low
pub mod LUT207L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 208 low
pub mod LUT208L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 209 low
pub mod LUT209L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 210 low
pub mod LUT210L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 211 low
pub mod LUT211L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 212 low
pub mod LUT212L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 213 low
pub mod LUT213L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 214 low
pub mod LUT214L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 215 low
pub mod LUT215L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 216 low
pub mod LUT216L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 217 low
pub mod LUT217L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 218 low
pub mod LUT218L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 219 low
pub mod LUT219L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 220 low
pub mod LUT220L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 221 low
pub mod LUT221L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 222 low
pub mod LUT222L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 223 low
pub mod LUT223L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 224 low
pub mod LUT224L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 225 low
pub mod LUT225L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 226 low
pub mod LUT226L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 227 low
pub mod LUT227L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 228 low
pub mod LUT228L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 229 low
pub mod LUT229L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 230 low
pub mod LUT230L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 231 low
pub mod LUT231L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 232 low
pub mod LUT232L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 233 low
pub mod LUT233L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 234 low
pub mod LUT234L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 235 low
pub mod LUT235L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 236 low
pub mod LUT236L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 237 low
pub mod LUT237L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 238 low
pub mod LUT238L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 239 low
pub mod LUT239L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 240 low
pub mod LUT240L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 241 low
pub mod LUT241L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 242 low
pub mod LUT242L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 243 low
pub mod LUT243L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 244 low
pub mod LUT244L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 245 low
pub mod LUT245L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 246 low
pub mod LUT246L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 247 low
pub mod LUT247L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 248 low
pub mod LUT248L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 249 low
pub mod LUT249L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 250 low
pub mod LUT250L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 251 low
pub mod LUT251L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 252 low
pub mod LUT252L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 253 low
pub mod LUT253L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 254 low
pub mod LUT254L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 255 low
pub mod LUT255L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 256 low
pub mod LUT256L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 257 low
pub mod LUT257L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 258 low
pub mod LUT258L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 259 low
pub mod LUT259L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 260 low
pub mod LUT260L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 261 low
pub mod LUT261L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 262 low
pub mod LUT262L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 263 low
pub mod LUT263L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 264 low
pub mod LUT264L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 265 low
pub mod LUT265L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 266 low
pub mod LUT266L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 267 low
pub mod LUT267L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 268 low
pub mod LUT268L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 269 low
pub mod LUT269L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 270 low
pub mod LUT270L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 271 low
pub mod LUT271L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 272 low
pub mod LUT272L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 273 low
pub mod LUT273L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 274 low
pub mod LUT274L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 275 low
pub mod LUT275L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 276 low
pub mod LUT276L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 277 low
pub mod LUT277L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 278 low
pub mod LUT278L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 279 low
pub mod LUT279L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 280 low
pub mod LUT280L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 281 low
pub mod LUT281L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 282 low
pub mod LUT282L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 283 low
pub mod LUT283L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 284 low
pub mod LUT284L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 285 low
pub mod LUT285L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 286 low
pub mod LUT286L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 287 low
pub mod LUT287L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 288 low
pub mod LUT288L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 289 low
pub mod LUT289L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 290 low
pub mod LUT290L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 291 low
pub mod LUT291L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 292 low
pub mod LUT292L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 293 low
pub mod LUT293L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 294 low
pub mod LUT294L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 295 low
pub mod LUT295L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 296 low
pub mod LUT296L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 297 low
pub mod LUT297L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 298 low
pub mod LUT298L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 299 low
pub mod LUT299L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 300 low
pub mod LUT300L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 301 low
pub mod LUT301L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 302 low
pub mod LUT302L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 303 low
pub mod LUT303L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 304 low
pub mod LUT304L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 305 low
pub mod LUT305L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 306 low
pub mod LUT306L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 307 low
pub mod LUT307L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 308 low
pub mod LUT308L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 309 low
pub mod LUT309L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 310 low
pub mod LUT310L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 311 low
pub mod LUT311L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 312 low
pub mod LUT312L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 313 low
pub mod LUT313L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 314 low
pub mod LUT314L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 315 low
pub mod LUT315L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 316 low
pub mod LUT316L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 317 low
pub mod LUT317L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 318 low
pub mod LUT318L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 319 low
pub mod LUT319L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 320 low
pub mod LUT320L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 321 low
pub mod LUT321L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 322 low
pub mod LUT322L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 323 low
pub mod LUT323L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 324 low
pub mod LUT324L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 325 low
pub mod LUT325L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 326 low
pub mod LUT326L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 327 low
pub mod LUT327L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 328 low
pub mod LUT328L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 329 low
pub mod LUT329L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 330 low
pub mod LUT330L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 331 low
pub mod LUT331L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 332 low
pub mod LUT332L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 333 low
pub mod LUT333L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 334 low
pub mod LUT334L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 335 low
pub mod LUT335L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 336 low
pub mod LUT336L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 337 low
pub mod LUT337L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 338 low
pub mod LUT338L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 339 low
pub mod LUT339L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 340 low
pub mod LUT340L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 341 low
pub mod LUT341L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 342 low
pub mod LUT342L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 343 low
pub mod LUT343L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 344 low
pub mod LUT344L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 345 low
pub mod LUT345L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 346 low
pub mod LUT346L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 347 low
pub mod LUT347L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 348 low
pub mod LUT348L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 349 low
pub mod LUT349L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 350 low
pub mod LUT350L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 351 low
pub mod LUT351L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 352 low
pub mod LUT352L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 353 low
pub mod LUT353L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 354 low
pub mod LUT354L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 355 low
pub mod LUT355L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 356 low
pub mod LUT356L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 357 low
pub mod LUT357L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 358 low
pub mod LUT358L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 359 low
pub mod LUT359L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 360 low
pub mod LUT360L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 361 low
pub mod LUT361L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 362 low
pub mod LUT362L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 363 low
pub mod LUT363L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 364 low
pub mod LUT364L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 365 low
pub mod LUT365L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 366 low
pub mod LUT366L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 367 low
pub mod LUT367L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 368 low
pub mod LUT368L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 369 low
pub mod LUT369L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 370 low
pub mod LUT370L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 371 low
pub mod LUT371L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 372 low
pub mod LUT372L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 373 low
pub mod LUT373L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 374 low
pub mod LUT374L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 375 low
pub mod LUT375L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 376 low
pub mod LUT376L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 377 low
pub mod LUT377L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 378 low
pub mod LUT378L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 379 low
pub mod LUT379L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 380 low
pub mod LUT380L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 381 low
pub mod LUT381L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 382 low
pub mod LUT382L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 383 low
pub mod LUT383L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 384 low
pub mod LUT384L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 385 low
pub mod LUT385L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 386 low
pub mod LUT386L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 387 low
pub mod LUT387L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 388 low
pub mod LUT388L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 389 low
pub mod LUT389L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 390 low
pub mod LUT390L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 391 low
pub mod LUT391L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 392 low
pub mod LUT392L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 393 low
pub mod LUT393L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 394 low
pub mod LUT394L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 395 low
pub mod LUT395L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 396 low
pub mod LUT396L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 397 low
pub mod LUT397L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 398 low
pub mod LUT398L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 399 low
pub mod LUT399L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 400 low
pub mod LUT400L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 401 low
pub mod LUT401L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 402 low
pub mod LUT402L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 403 low
pub mod LUT403L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 404 low
pub mod LUT404L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 405 low
pub mod LUT405L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 406 low
pub mod LUT406L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 407 low
pub mod LUT407L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 408 low
pub mod LUT408L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 409 low
pub mod LUT409L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 410 low
pub mod LUT410L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 411 low
pub mod LUT411L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 412 low
pub mod LUT412L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 413 low
pub mod LUT413L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 414 low
pub mod LUT414L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 415 low
pub mod LUT415L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 416 low
pub mod LUT416L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 417 low
pub mod LUT417L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 418 low
pub mod LUT418L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 419 low
pub mod LUT419L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 420 low
pub mod LUT420L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 421 low
pub mod LUT421L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 422 low
pub mod LUT422L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 423 low
pub mod LUT423L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 424 low
pub mod LUT424L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 425 low
pub mod LUT425L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 426 low
pub mod LUT426L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 427 low
pub mod LUT427L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 428 low
pub mod LUT428L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 429 low
pub mod LUT429L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 430 low
pub mod LUT430L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 431 low
pub mod LUT431L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 432 low
pub mod LUT432L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 433 low
pub mod LUT433L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 434 low
pub mod LUT434L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 435 low
pub mod LUT435L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 436 low
pub mod LUT436L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 437 low
pub mod LUT437L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 438 low
pub mod LUT438L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 439 low
pub mod LUT439L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 440 low
pub mod LUT440L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 441 low
pub mod LUT441L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 442 low
pub mod LUT442L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 443 low
pub mod LUT443L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 444 low
pub mod LUT444L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 445 low
pub mod LUT445L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 446 low
pub mod LUT446L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 447 low
pub mod LUT447L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 448 low
pub mod LUT448L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 449 low
pub mod LUT449L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 450 low
pub mod LUT450L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 451 low
pub mod LUT451L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 452 low
pub mod LUT452L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 453 low
pub mod LUT453L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 454 low
pub mod LUT454L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 455 low
pub mod LUT455L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 456 low
pub mod LUT456L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 457 low
pub mod LUT457L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 458 low
pub mod LUT458L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 459 low
pub mod LUT459L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 460 low
pub mod LUT460L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 461 low
pub mod LUT461L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 462 low
pub mod LUT462L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 463 low
pub mod LUT463L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 464 low
pub mod LUT464L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 465 low
pub mod LUT465L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 466 low
pub mod LUT466L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 467 low
pub mod LUT467L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 468 low
pub mod LUT468L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 469 low
pub mod LUT469L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 470 low
pub mod LUT470L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 471 low
pub mod LUT471L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 472 low
pub mod LUT472L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 473 low
pub mod LUT473L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 474 low
pub mod LUT474L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 475 low
pub mod LUT475L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 476 low
pub mod LUT476L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 477 low
pub mod LUT477L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 478 low
pub mod LUT478L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 479 low
pub mod LUT479L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 480 low
pub mod LUT480L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 481 low
pub mod LUT481L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 482 low
pub mod LUT482L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 483 low
pub mod LUT483L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 484 low
pub mod LUT484L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 485 low
pub mod LUT485L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 486 low
pub mod LUT486L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 487 low
pub mod LUT487L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 488 low
pub mod LUT488L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 489 low
pub mod LUT489L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 490 low
pub mod LUT490L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 491 low
pub mod LUT491L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 492 low
pub mod LUT492L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 493 low
pub mod LUT493L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 494 low
pub mod LUT494L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 495 low
pub mod LUT495L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 496 low
pub mod LUT496L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 497 low
pub mod LUT497L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 498 low
pub mod LUT498L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 499 low
pub mod LUT499L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 500 low
pub mod LUT500L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 501 low
pub mod LUT501L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 502 low
pub mod LUT502L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 503 low
pub mod LUT503L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 504 low
pub mod LUT504L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 505 low
pub mod LUT505L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 506 low
pub mod LUT506L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 507 low
pub mod LUT507L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 508 low
pub mod LUT508L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 509 low
pub mod LUT509L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 510 low
pub mod LUT510L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 511 low
pub mod LUT511L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 512 low
pub mod LUT512L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 513 low
pub mod LUT513L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 514 low
pub mod LUT514L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 515 low
pub mod LUT515L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 516 low
pub mod LUT516L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 517 low
pub mod LUT517L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 518 low
pub mod LUT518L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 519 low
pub mod LUT519L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 520 low
pub mod LUT520L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 521 low
pub mod LUT521L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 522 low
pub mod LUT522L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 523 low
pub mod LUT523L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 524 low
pub mod LUT524L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 525 low
pub mod LUT525L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 526 low
pub mod LUT526L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 527 low
pub mod LUT527L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 528 low
pub mod LUT528L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 529 low
pub mod LUT529L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 530 low
pub mod LUT530L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 531 low
pub mod LUT531L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 532 low
pub mod LUT532L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 533 low
pub mod LUT533L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 534 low
pub mod LUT534L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 535 low
pub mod LUT535L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 536 low
pub mod LUT536L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 537 low
pub mod LUT537L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 538 low
pub mod LUT538L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 539 low
pub mod LUT539L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 540 low
pub mod LUT540L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 541 low
pub mod LUT541L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 542 low
pub mod LUT542L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 543 low
pub mod LUT543L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 544 low
pub mod LUT544L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 545 low
pub mod LUT545L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 546 low
pub mod LUT546L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 547 low
pub mod LUT547L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 548 low
pub mod LUT548L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 549 low
pub mod LUT549L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 550 low
pub mod LUT550L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 551 low
pub mod LUT551L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 552 low
pub mod LUT552L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 553 low
pub mod LUT553L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 554 low
pub mod LUT554L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 555 low
pub mod LUT555L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 556 low
pub mod LUT556L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 557 low
pub mod LUT557L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 558 low
pub mod LUT558L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 559 low
pub mod LUT559L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 560 low
pub mod LUT560L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 561 low
pub mod LUT561L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 562 low
pub mod LUT562L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 563 low
pub mod LUT563L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 564 low
pub mod LUT564L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 565 low
pub mod LUT565L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 566 low
pub mod LUT566L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 567 low
pub mod LUT567L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 568 low
pub mod LUT568L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 569 low
pub mod LUT569L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 570 low
pub mod LUT570L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 571 low
pub mod LUT571L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 572 low
pub mod LUT572L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 573 low
pub mod LUT573L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 574 low
pub mod LUT574L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 575 low
pub mod LUT575L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 576 low
pub mod LUT576L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 577 low
pub mod LUT577L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 578 low
pub mod LUT578L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 579 low
pub mod LUT579L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 580 low
pub mod LUT580L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 581 low
pub mod LUT581L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 582 low
pub mod LUT582L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 583 low
pub mod LUT583L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 584 low
pub mod LUT584L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 585 low
pub mod LUT585L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 586 low
pub mod LUT586L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 587 low
pub mod LUT587L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 588 low
pub mod LUT588L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 589 low
pub mod LUT589L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 590 low
pub mod LUT590L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 591 low
pub mod LUT591L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 592 low
pub mod LUT592L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 593 low
pub mod LUT593L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 594 low
pub mod LUT594L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 595 low
pub mod LUT595L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 596 low
pub mod LUT596L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 597 low
pub mod LUT597L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 598 low
pub mod LUT598L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 599 low
pub mod LUT599L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 600 low
pub mod LUT600L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 601 low
pub mod LUT601L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 602 low
pub mod LUT602L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 603 low
pub mod LUT603L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 604 low
pub mod LUT604L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 605 low
pub mod LUT605L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 606 low
pub mod LUT606L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 607 low
pub mod LUT607L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 608 low
pub mod LUT608L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 609 low
pub mod LUT609L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 610 low
pub mod LUT610L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 611 low
pub mod LUT611L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 612 low
pub mod LUT612L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 613 low
pub mod LUT613L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 614 low
pub mod LUT614L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 615 low
pub mod LUT615L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 616 low
pub mod LUT616L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 617 low
pub mod LUT617L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 618 low
pub mod LUT618L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 619 low
pub mod LUT619L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 620 low
pub mod LUT620L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 621 low
pub mod LUT621L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 622 low
pub mod LUT622L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 623 low
pub mod LUT623L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 624 low
pub mod LUT624L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 625 low
pub mod LUT625L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 626 low
pub mod LUT626L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 627 low
pub mod LUT627L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 628 low
pub mod LUT628L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 629 low
pub mod LUT629L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 630 low
pub mod LUT630L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 631 low
pub mod LUT631L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 632 low
pub mod LUT632L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 633 low
pub mod LUT633L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 634 low
pub mod LUT634L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 635 low
pub mod LUT635L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 636 low
pub mod LUT636L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 637 low
pub mod LUT637L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 638 low
pub mod LUT638L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 639 low
pub mod LUT639L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 640 low
pub mod LUT640L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 641 low
pub mod LUT641L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 642 low
pub mod LUT642L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 643 low
pub mod LUT643L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 644 low
pub mod LUT644L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 645 low
pub mod LUT645L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 646 low
pub mod LUT646L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 647 low
pub mod LUT647L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 648 low
pub mod LUT648L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 649 low
pub mod LUT649L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 650 low
pub mod LUT650L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 651 low
pub mod LUT651L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 652 low
pub mod LUT652L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 653 low
pub mod LUT653L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 654 low
pub mod LUT654L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 655 low
pub mod LUT655L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 656 low
pub mod LUT656L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 657 low
pub mod LUT657L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 658 low
pub mod LUT658L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 659 low
pub mod LUT659L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 660 low
pub mod LUT660L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 661 low
pub mod LUT661L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 662 low
pub mod LUT662L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 663 low
pub mod LUT663L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 664 low
pub mod LUT664L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 665 low
pub mod LUT665L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 666 low
pub mod LUT666L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 667 low
pub mod LUT667L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 668 low
pub mod LUT668L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 669 low
pub mod LUT669L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 670 low
pub mod LUT670L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 671 low
pub mod LUT671L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 672 low
pub mod LUT672L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 673 low
pub mod LUT673L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 674 low
pub mod LUT674L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 675 low
pub mod LUT675L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 676 low
pub mod LUT676L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 677 low
pub mod LUT677L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 678 low
pub mod LUT678L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 679 low
pub mod LUT679L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 680 low
pub mod LUT680L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 681 low
pub mod LUT681L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 682 low
pub mod LUT682L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 683 low
pub mod LUT683L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 684 low
pub mod LUT684L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 685 low
pub mod LUT685L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 686 low
pub mod LUT686L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 687 low
pub mod LUT687L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 688 low
pub mod LUT688L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 689 low
pub mod LUT689L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 690 low
pub mod LUT690L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 691 low
pub mod LUT691L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 692 low
pub mod LUT692L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 693 low
pub mod LUT693L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 694 low
pub mod LUT694L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 695 low
pub mod LUT695L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 696 low
pub mod LUT696L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 697 low
pub mod LUT697L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 698 low
pub mod LUT698L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 699 low
pub mod LUT699L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 700 low
pub mod LUT700L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 701 low
pub mod LUT701L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 702 low
pub mod LUT702L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 703 low
pub mod LUT703L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 704 low
pub mod LUT704L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 705 low
pub mod LUT705L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 706 low
pub mod LUT706L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 707 low
pub mod LUT707L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 708 low
pub mod LUT708L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 709 low
pub mod LUT709L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 710 low
pub mod LUT710L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 711 low
pub mod LUT711L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 712 low
pub mod LUT712L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 713 low
pub mod LUT713L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 714 low
pub mod LUT714L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 715 low
pub mod LUT715L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 716 low
pub mod LUT716L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 717 low
pub mod LUT717L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 718 low
pub mod LUT718L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 719 low
pub mod LUT719L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 720 low
pub mod LUT720L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 721 low
pub mod LUT721L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 722 low
pub mod LUT722L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 723 low
pub mod LUT723L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 724 low
pub mod LUT724L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 725 low
pub mod LUT725L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 726 low
pub mod LUT726L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 727 low
pub mod LUT727L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 728 low
pub mod LUT728L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 729 low
pub mod LUT729L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 730 low
pub mod LUT730L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 731 low
pub mod LUT731L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 732 low
pub mod LUT732L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 733 low
pub mod LUT733L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 734 low
pub mod LUT734L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 735 low
pub mod LUT735L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 736 low
pub mod LUT736L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 737 low
pub mod LUT737L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 738 low
pub mod LUT738L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 739 low
pub mod LUT739L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 740 low
pub mod LUT740L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 741 low
pub mod LUT741L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 742 low
pub mod LUT742L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 743 low
pub mod LUT743L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 744 low
pub mod LUT744L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 745 low
pub mod LUT745L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 746 low
pub mod LUT746L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 747 low
pub mod LUT747L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 748 low
pub mod LUT748L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 749 low
pub mod LUT749L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 750 low
pub mod LUT750L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 751 low
pub mod LUT751L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 752 low
pub mod LUT752L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 753 low
pub mod LUT753L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 754 low
pub mod LUT754L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 755 low
pub mod LUT755L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 756 low
pub mod LUT756L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 757 low
pub mod LUT757L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 758 low
pub mod LUT758L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 759 low
pub mod LUT759L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 760 low
pub mod LUT760L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 761 low
pub mod LUT761L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 762 low
pub mod LUT762L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 763 low
pub mod LUT763L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 764 low
pub mod LUT764L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 765 low
pub mod LUT765L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 766 low
pub mod LUT766L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 767 low
pub mod LUT767L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 768 low
pub mod LUT768L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 769 low
pub mod LUT769L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 770 low
pub mod LUT770L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 771 low
pub mod LUT771L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 772 low
pub mod LUT772L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 773 low
pub mod LUT773L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 774 low
pub mod LUT774L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 775 low
pub mod LUT775L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 776 low
pub mod LUT776L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 777 low
pub mod LUT777L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 778 low
pub mod LUT778L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 779 low
pub mod LUT779L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 780 low
pub mod LUT780L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 781 low
pub mod LUT781L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 782 low
pub mod LUT782L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 783 low
pub mod LUT783L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 784 low
pub mod LUT784L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 785 low
pub mod LUT785L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 786 low
pub mod LUT786L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 787 low
pub mod LUT787L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 788 low
pub mod LUT788L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 789 low
pub mod LUT789L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 790 low
pub mod LUT790L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 791 low
pub mod LUT791L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 792 low
pub mod LUT792L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 793 low
pub mod LUT793L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 794 low
pub mod LUT794L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 795 low
pub mod LUT795L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 796 low
pub mod LUT796L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 797 low
pub mod LUT797L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 798 low
pub mod LUT798L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 799 low
pub mod LUT799L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 800 low
pub mod LUT800L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 801 low
pub mod LUT801L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 802 low
pub mod LUT802L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 803 low
pub mod LUT803L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 804 low
pub mod LUT804L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 805 low
pub mod LUT805L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 806 low
pub mod LUT806L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 807 low
pub mod LUT807L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 808 low
pub mod LUT808L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 809 low
pub mod LUT809L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 810 low
pub mod LUT810L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 811 low
pub mod LUT811L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 812 low
pub mod LUT812L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 813 low
pub mod LUT813L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 814 low
pub mod LUT814L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 815 low
pub mod LUT815L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 816 low
pub mod LUT816L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 817 low
pub mod LUT817L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 818 low
pub mod LUT818L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 819 low
pub mod LUT819L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 820 low
pub mod LUT820L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 821 low
pub mod LUT821L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 822 low
pub mod LUT822L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 823 low
pub mod LUT823L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 824 low
pub mod LUT824L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 825 low
pub mod LUT825L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 826 low
pub mod LUT826L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 827 low
pub mod LUT827L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 828 low
pub mod LUT828L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 829 low
pub mod LUT829L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 830 low
pub mod LUT830L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 831 low
pub mod LUT831L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 832 low
pub mod LUT832L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 833 low
pub mod LUT833L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 834 low
pub mod LUT834L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 835 low
pub mod LUT835L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 836 low
pub mod LUT836L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 837 low
pub mod LUT837L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 838 low
pub mod LUT838L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 839 low
pub mod LUT839L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 840 low
pub mod LUT840L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 841 low
pub mod LUT841L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 842 low
pub mod LUT842L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 843 low
pub mod LUT843L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 844 low
pub mod LUT844L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 845 low
pub mod LUT845L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 846 low
pub mod LUT846L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 847 low
pub mod LUT847L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 848 low
pub mod LUT848L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 849 low
pub mod LUT849L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 850 low
pub mod LUT850L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 851 low
pub mod LUT851L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 852 low
pub mod LUT852L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 853 low
pub mod LUT853L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 854 low
pub mod LUT854L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 855 low
pub mod LUT855L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 856 low
pub mod LUT856L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 857 low
pub mod LUT857L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 858 low
pub mod LUT858L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 859 low
pub mod LUT859L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 860 low
pub mod LUT860L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 861 low
pub mod LUT861L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 862 low
pub mod LUT862L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 863 low
pub mod LUT863L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 864 low
pub mod LUT864L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 865 low
pub mod LUT865L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 866 low
pub mod LUT866L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 867 low
pub mod LUT867L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 868 low
pub mod LUT868L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 869 low
pub mod LUT869L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 870 low
pub mod LUT870L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 871 low
pub mod LUT871L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 872 low
pub mod LUT872L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 873 low
pub mod LUT873L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 874 low
pub mod LUT874L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 875 low
pub mod LUT875L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 876 low
pub mod LUT876L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 877 low
pub mod LUT877L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 878 low
pub mod LUT878L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 879 low
pub mod LUT879L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 880 low
pub mod LUT880L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 881 low
pub mod LUT881L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 882 low
pub mod LUT882L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 883 low
pub mod LUT883L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 884 low
pub mod LUT884L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 885 low
pub mod LUT885L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 886 low
pub mod LUT886L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 887 low
pub mod LUT887L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 888 low
pub mod LUT888L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 889 low
pub mod LUT889L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 890 low
pub mod LUT890L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 891 low
pub mod LUT891L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 892 low
pub mod LUT892L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 893 low
pub mod LUT893L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 894 low
pub mod LUT894L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 895 low
pub mod LUT895L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 896 low
pub mod LUT896L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 897 low
pub mod LUT897L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 898 low
pub mod LUT898L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 899 low
pub mod LUT899L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 900 low
pub mod LUT900L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 901 low
pub mod LUT901L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 902 low
pub mod LUT902L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 903 low
pub mod LUT903L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 904 low
pub mod LUT904L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 905 low
pub mod LUT905L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 906 low
pub mod LUT906L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 907 low
pub mod LUT907L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 908 low
pub mod LUT908L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 909 low
pub mod LUT909L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 910 low
pub mod LUT910L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 911 low
pub mod LUT911L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 912 low
pub mod LUT912L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 913 low
pub mod LUT913L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 914 low
pub mod LUT914L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 915 low
pub mod LUT915L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 916 low
pub mod LUT916L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 917 low
pub mod LUT917L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 918 low
pub mod LUT918L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 919 low
pub mod LUT919L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 920 low
pub mod LUT920L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 921 low
pub mod LUT921L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 922 low
pub mod LUT922L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 923 low
pub mod LUT923L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 924 low
pub mod LUT924L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 925 low
pub mod LUT925L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 926 low
pub mod LUT926L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 927 low
pub mod LUT927L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 928 low
pub mod LUT928L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 929 low
pub mod LUT929L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 930 low
pub mod LUT930L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 931 low
pub mod LUT931L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 932 low
pub mod LUT932L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 933 low
pub mod LUT933L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 934 low
pub mod LUT934L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 935 low
pub mod LUT935L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 936 low
pub mod LUT936L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 937 low
pub mod LUT937L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 938 low
pub mod LUT938L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 939 low
pub mod LUT939L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 940 low
pub mod LUT940L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 941 low
pub mod LUT941L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 942 low
pub mod LUT942L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 943 low
pub mod LUT943L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 944 low
pub mod LUT944L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 945 low
pub mod LUT945L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 946 low
pub mod LUT946L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 947 low
pub mod LUT947L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 948 low
pub mod LUT948L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 949 low
pub mod LUT949L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 950 low
pub mod LUT950L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 951 low
pub mod LUT951L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 952 low
pub mod LUT952L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 953 low
pub mod LUT953L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 954 low
pub mod LUT954L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 955 low
pub mod LUT955L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 956 low
pub mod LUT956L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 957 low
pub mod LUT957L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 958 low
pub mod LUT958L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 959 low
pub mod LUT959L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 960 low
pub mod LUT960L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 961 low
pub mod LUT961L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 962 low
pub mod LUT962L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 963 low
pub mod LUT963L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 964 low
pub mod LUT964L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 965 low
pub mod LUT965L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 966 low
pub mod LUT966L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 967 low
pub mod LUT967L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 968 low
pub mod LUT968L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 969 low
pub mod LUT969L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 970 low
pub mod LUT970L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 971 low
pub mod LUT971L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 972 low
pub mod LUT972L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 973 low
pub mod LUT973L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 974 low
pub mod LUT974L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 975 low
pub mod LUT975L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 976 low
pub mod LUT976L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 977 low
pub mod LUT977L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 978 low
pub mod LUT978L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 979 low
pub mod LUT979L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 980 low
pub mod LUT980L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 981 low
pub mod LUT981L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 982 low
pub mod LUT982L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 983 low
pub mod LUT983L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 984 low
pub mod LUT984L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 985 low
pub mod LUT985L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 986 low
pub mod LUT986L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 987 low
pub mod LUT987L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 988 low
pub mod LUT988L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 989 low
pub mod LUT989L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 990 low
pub mod LUT990L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 991 low
pub mod LUT991L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 992 low
pub mod LUT992L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 993 low
pub mod LUT993L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 994 low
pub mod LUT994L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 995 low
pub mod LUT995L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 996 low
pub mod LUT996L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 997 low
pub mod LUT997L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 998 low
pub mod LUT998L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 999 low
pub mod LUT999L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 1000 low
pub mod LUT1000L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 1001 low
pub mod LUT1001L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 1002 low
pub mod LUT1002L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 1003 low
pub mod LUT1003L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 1004 low
pub mod LUT1004L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 1005 low
pub mod LUT1005L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 1006 low
pub mod LUT1006L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 1007 low
pub mod LUT1007L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 1008 low
pub mod LUT1008L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 1009 low
pub mod LUT1009L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 1010 low
pub mod LUT1010L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 1011 low
pub mod LUT1011L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 1012 low
pub mod LUT1012L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 1013 low
pub mod LUT1013L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 1014 low
pub mod LUT1014L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 1015 low
pub mod LUT1015L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 1016 low
pub mod LUT1016L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 1017 low
pub mod LUT1017L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 1018 low
pub mod LUT1018L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 1019 low
pub mod LUT1019L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 1020 low
pub mod LUT1020L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 1021 low
pub mod LUT1021L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 1022 low
pub mod LUT1022L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 1023 low
pub mod LUT1023L {
    pub use super::LUT0L::EN;
    pub use super::LUT0L::FVB;
    pub use super::LUT0L::LVB;
}

/// Graphic MMU LUT entry 0 high
pub mod LUT0H {

    /// Line offset
    pub mod LO {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (18 bits: 0x3ffff << 4)
        pub const mask: u32 = 0x3ffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Graphic MMU LUT entry 1 high
pub mod LUT1H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 2 high
pub mod LUT2H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 3 high
pub mod LUT3H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 4 high
pub mod LUT4H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 5 high
pub mod LUT5H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 6 high
pub mod LUT6H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 7 high
pub mod LUT7H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 8 high
pub mod LUT8H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 9 high
pub mod LUT9H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 10 high
pub mod LUT10H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 11 high
pub mod LUT11H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 12 high
pub mod LUT12H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 13 high
pub mod LUT13H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 14 high
pub mod LUT14H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 15 high
pub mod LUT15H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 16 high
pub mod LUT16H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 17 high
pub mod LUT17H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 18 high
pub mod LUT18H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 19 high
pub mod LUT19H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 20 high
pub mod LUT20H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 21 high
pub mod LUT21H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 22 high
pub mod LUT22H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 23 high
pub mod LUT23H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 24 high
pub mod LUT24H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 25 high
pub mod LUT25H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 26 high
pub mod LUT26H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 27 high
pub mod LUT27H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 28 high
pub mod LUT28H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 29 high
pub mod LUT29H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 30 high
pub mod LUT30H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 31 high
pub mod LUT31H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 32 high
pub mod LUT32H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 33 high
pub mod LUT33H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 34 high
pub mod LUT34H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 35 high
pub mod LUT35H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 36 high
pub mod LUT36H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 37 high
pub mod LUT37H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 38 high
pub mod LUT38H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 39 high
pub mod LUT39H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 40 high
pub mod LUT40H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 41 high
pub mod LUT41H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 42 high
pub mod LUT42H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 43 high
pub mod LUT43H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 44 high
pub mod LUT44H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 45 high
pub mod LUT45H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 46 high
pub mod LUT46H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 47 high
pub mod LUT47H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 48 high
pub mod LUT48H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 49 high
pub mod LUT49H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 50 high
pub mod LUT50H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 51 high
pub mod LUT51H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 52 high
pub mod LUT52H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 53 high
pub mod LUT53H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 54 high
pub mod LUT54H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 55 high
pub mod LUT55H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 56 high
pub mod LUT56H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 57 high
pub mod LUT57H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 58 high
pub mod LUT58H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 59 high
pub mod LUT59H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 60 high
pub mod LUT60H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 61 high
pub mod LUT61H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 62 high
pub mod LUT62H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 63 high
pub mod LUT63H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 64 high
pub mod LUT64H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 65 high
pub mod LUT65H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 66 high
pub mod LUT66H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 67 high
pub mod LUT67H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 68 high
pub mod LUT68H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 69 high
pub mod LUT69H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 70 high
pub mod LUT70H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 71 high
pub mod LUT71H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 72 high
pub mod LUT72H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 73 high
pub mod LUT73H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 74 high
pub mod LUT74H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 75 high
pub mod LUT75H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 76 high
pub mod LUT76H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 77 high
pub mod LUT77H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 78 high
pub mod LUT78H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 79 high
pub mod LUT79H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 80 high
pub mod LUT80H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 81 high
pub mod LUT81H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 82 high
pub mod LUT82H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 83 high
pub mod LUT83H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 84 high
pub mod LUT84H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 85 high
pub mod LUT85H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 86 high
pub mod LUT86H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 87 high
pub mod LUT87H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 88 high
pub mod LUT88H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 89 high
pub mod LUT89H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 90 high
pub mod LUT90H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 91 high
pub mod LUT91H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 92 high
pub mod LUT92H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 93 high
pub mod LUT93H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 94 high
pub mod LUT94H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 95 high
pub mod LUT95H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 96 high
pub mod LUT96H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 97 high
pub mod LUT97H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 98 high
pub mod LUT98H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 99 high
pub mod LUT99H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 100 high
pub mod LUT100H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 101 high
pub mod LUT101H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 102 high
pub mod LUT102H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 103 high
pub mod LUT103H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 104 high
pub mod LUT104H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 105 high
pub mod LUT105H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 106 high
pub mod LUT106H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 107 high
pub mod LUT107H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 108 high
pub mod LUT108H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 109 high
pub mod LUT109H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 110 high
pub mod LUT110H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 111 high
pub mod LUT111H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 112 high
pub mod LUT112H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 113 high
pub mod LUT113H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 114 high
pub mod LUT114H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 115 high
pub mod LUT115H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 116 high
pub mod LUT116H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 117 high
pub mod LUT117H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 118 high
pub mod LUT118H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 119 high
pub mod LUT119H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 120 high
pub mod LUT120H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 121 high
pub mod LUT121H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 122 high
pub mod LUT122H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 123 high
pub mod LUT123H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 124 high
pub mod LUT124H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 125 high
pub mod LUT125H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 126 high
pub mod LUT126H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 127 high
pub mod LUT127H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 128 high
pub mod LUT128H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 129 high
pub mod LUT129H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 130 high
pub mod LUT130H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 131 high
pub mod LUT131H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 132 high
pub mod LUT132H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 133 high
pub mod LUT133H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 134 high
pub mod LUT134H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 135 high
pub mod LUT135H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 136 high
pub mod LUT136H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 137 high
pub mod LUT137H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 138 high
pub mod LUT138H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 139 high
pub mod LUT139H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 140 high
pub mod LUT140H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 141 high
pub mod LUT141H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 142 high
pub mod LUT142H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 143 high
pub mod LUT143H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 144 high
pub mod LUT144H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 145 high
pub mod LUT145H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 146 high
pub mod LUT146H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 147 high
pub mod LUT147H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 148 high
pub mod LUT148H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 149 high
pub mod LUT149H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 150 high
pub mod LUT150H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 151 high
pub mod LUT151H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 152 high
pub mod LUT152H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 153 high
pub mod LUT153H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 154 high
pub mod LUT154H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 155 high
pub mod LUT155H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 156 high
pub mod LUT156H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 157 high
pub mod LUT157H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 158 high
pub mod LUT158H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 159 high
pub mod LUT159H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 160 high
pub mod LUT160H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 161 high
pub mod LUT161H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 162 high
pub mod LUT162H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 163 high
pub mod LUT163H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 164 high
pub mod LUT164H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 165 high
pub mod LUT165H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 166 high
pub mod LUT166H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 167 high
pub mod LUT167H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 168 high
pub mod LUT168H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 169 high
pub mod LUT169H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 170 high
pub mod LUT170H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 171 high
pub mod LUT171H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 172 high
pub mod LUT172H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 173 high
pub mod LUT173H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 174 high
pub mod LUT174H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 175 high
pub mod LUT175H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 176 high
pub mod LUT176H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 177 high
pub mod LUT177H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 178 high
pub mod LUT178H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 179 high
pub mod LUT179H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 180 high
pub mod LUT180H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 181 high
pub mod LUT181H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 182 high
pub mod LUT182H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 183 high
pub mod LUT183H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 184 high
pub mod LUT184H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 185 high
pub mod LUT185H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 186 high
pub mod LUT186H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 187 high
pub mod LUT187H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 188 high
pub mod LUT188H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 189 high
pub mod LUT189H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 190 high
pub mod LUT190H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 191 high
pub mod LUT191H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 192 high
pub mod LUT192H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 193 high
pub mod LUT193H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 194 high
pub mod LUT194H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 195 high
pub mod LUT195H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 196 high
pub mod LUT196H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 197 high
pub mod LUT197H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 198 high
pub mod LUT198H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 199 high
pub mod LUT199H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 200 high
pub mod LUT200H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 201 high
pub mod LUT201H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 202 high
pub mod LUT202H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 203 high
pub mod LUT203H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 204 high
pub mod LUT204H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 205 high
pub mod LUT205H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 206 high
pub mod LUT206H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 207 high
pub mod LUT207H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 208 high
pub mod LUT208H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 209 high
pub mod LUT209H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 210 high
pub mod LUT210H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 211 high
pub mod LUT211H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 212 high
pub mod LUT212H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 213 high
pub mod LUT213H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 214 high
pub mod LUT214H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 215 high
pub mod LUT215H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 216 high
pub mod LUT216H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 217 high
pub mod LUT217H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 218 high
pub mod LUT218H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 219 high
pub mod LUT219H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 220 high
pub mod LUT220H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 221 high
pub mod LUT221H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 222 high
pub mod LUT222H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 223 high
pub mod LUT223H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 224 high
pub mod LUT224H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 225 high
pub mod LUT225H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 226 high
pub mod LUT226H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 227 high
pub mod LUT227H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 228 high
pub mod LUT228H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 229 high
pub mod LUT229H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 230 high
pub mod LUT230H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 231 high
pub mod LUT231H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 232 high
pub mod LUT232H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 233 high
pub mod LUT233H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 234 high
pub mod LUT234H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 235 high
pub mod LUT235H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 236 high
pub mod LUT236H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 237 high
pub mod LUT237H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 238 high
pub mod LUT238H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 239 high
pub mod LUT239H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 240 high
pub mod LUT240H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 241 high
pub mod LUT241H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 242 high
pub mod LUT242H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 243 high
pub mod LUT243H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 244 high
pub mod LUT244H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 245 high
pub mod LUT245H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 246 high
pub mod LUT246H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 247 high
pub mod LUT247H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 248 high
pub mod LUT248H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 249 high
pub mod LUT249H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 250 high
pub mod LUT250H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 251 high
pub mod LUT251H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 252 high
pub mod LUT252H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 253 high
pub mod LUT253H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 254 high
pub mod LUT254H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 255 high
pub mod LUT255H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 256 high
pub mod LUT256H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 257 high
pub mod LUT257H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 258 high
pub mod LUT258H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 259 high
pub mod LUT259H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 260 high
pub mod LUT260H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 261 high
pub mod LUT261H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 262 high
pub mod LUT262H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 263 high
pub mod LUT263H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 264 high
pub mod LUT264H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 265 high
pub mod LUT265H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 266 high
pub mod LUT266H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 267 high
pub mod LUT267H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 268 high
pub mod LUT268H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 269 high
pub mod LUT269H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 270 high
pub mod LUT270H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 271 high
pub mod LUT271H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 272 high
pub mod LUT272H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 273 high
pub mod LUT273H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 274 high
pub mod LUT274H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 275 high
pub mod LUT275H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 276 high
pub mod LUT276H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 277 high
pub mod LUT277H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 278 high
pub mod LUT278H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 279 high
pub mod LUT279H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 280 high
pub mod LUT280H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 281 high
pub mod LUT281H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 282 high
pub mod LUT282H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 283 high
pub mod LUT283H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 284 high
pub mod LUT284H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 285 high
pub mod LUT285H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 286 high
pub mod LUT286H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 287 high
pub mod LUT287H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 288 high
pub mod LUT288H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 289 high
pub mod LUT289H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 290 high
pub mod LUT290H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 291 high
pub mod LUT291H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 292 high
pub mod LUT292H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 293 high
pub mod LUT293H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 294 high
pub mod LUT294H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 295 high
pub mod LUT295H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 296 high
pub mod LUT296H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 297 high
pub mod LUT297H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 298 high
pub mod LUT298H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 299 high
pub mod LUT299H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 300 high
pub mod LUT300H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 301 high
pub mod LUT301H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 302 high
pub mod LUT302H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 303 high
pub mod LUT303H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 304 high
pub mod LUT304H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 305 high
pub mod LUT305H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 306 high
pub mod LUT306H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 307 high
pub mod LUT307H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 308 high
pub mod LUT308H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 309 high
pub mod LUT309H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 310 high
pub mod LUT310H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 311 high
pub mod LUT311H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 312 high
pub mod LUT312H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 313 high
pub mod LUT313H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 314 high
pub mod LUT314H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 315 high
pub mod LUT315H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 316 high
pub mod LUT316H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 317 high
pub mod LUT317H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 318 high
pub mod LUT318H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 319 high
pub mod LUT319H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 320 high
pub mod LUT320H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 321 high
pub mod LUT321H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 322 high
pub mod LUT322H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 323 high
pub mod LUT323H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 324 high
pub mod LUT324H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 325 high
pub mod LUT325H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 326 high
pub mod LUT326H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 327 high
pub mod LUT327H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 328 high
pub mod LUT328H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 329 high
pub mod LUT329H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 330 high
pub mod LUT330H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 331 high
pub mod LUT331H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 332 high
pub mod LUT332H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 333 high
pub mod LUT333H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 334 high
pub mod LUT334H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 335 high
pub mod LUT335H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 336 high
pub mod LUT336H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 337 high
pub mod LUT337H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 338 high
pub mod LUT338H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 339 high
pub mod LUT339H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 340 high
pub mod LUT340H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 341 high
pub mod LUT341H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 342 high
pub mod LUT342H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 343 high
pub mod LUT343H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 344 high
pub mod LUT344H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 345 high
pub mod LUT345H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 346 high
pub mod LUT346H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 347 high
pub mod LUT347H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 348 high
pub mod LUT348H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 349 high
pub mod LUT349H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 350 high
pub mod LUT350H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 351 high
pub mod LUT351H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 352 high
pub mod LUT352H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 353 high
pub mod LUT353H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 354 high
pub mod LUT354H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 355 high
pub mod LUT355H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 356 high
pub mod LUT356H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 357 high
pub mod LUT357H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 358 high
pub mod LUT358H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 359 high
pub mod LUT359H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 360 high
pub mod LUT360H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 361 high
pub mod LUT361H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 362 high
pub mod LUT362H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 363 high
pub mod LUT363H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 364 high
pub mod LUT364H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 365 high
pub mod LUT365H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 366 high
pub mod LUT366H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 367 high
pub mod LUT367H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 368 high
pub mod LUT368H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 369 high
pub mod LUT369H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 370 high
pub mod LUT370H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 371 high
pub mod LUT371H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 372 high
pub mod LUT372H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 373 high
pub mod LUT373H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 374 high
pub mod LUT374H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 375 high
pub mod LUT375H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 376 high
pub mod LUT376H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 377 high
pub mod LUT377H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 378 high
pub mod LUT378H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 379 high
pub mod LUT379H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 380 high
pub mod LUT380H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 381 high
pub mod LUT381H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 382 high
pub mod LUT382H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 383 high
pub mod LUT383H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 384 high
pub mod LUT384H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 385 high
pub mod LUT385H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 386 high
pub mod LUT386H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 387 high
pub mod LUT387H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 388 high
pub mod LUT388H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 389 high
pub mod LUT389H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 390 high
pub mod LUT390H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 391 high
pub mod LUT391H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 392 high
pub mod LUT392H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 393 high
pub mod LUT393H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 394 high
pub mod LUT394H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 395 high
pub mod LUT395H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 396 high
pub mod LUT396H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 397 high
pub mod LUT397H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 398 high
pub mod LUT398H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 399 high
pub mod LUT399H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 400 high
pub mod LUT400H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 401 high
pub mod LUT401H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 402 high
pub mod LUT402H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 403 high
pub mod LUT403H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 404 high
pub mod LUT404H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 405 high
pub mod LUT405H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 406 high
pub mod LUT406H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 407 high
pub mod LUT407H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 408 high
pub mod LUT408H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 409 high
pub mod LUT409H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 410 high
pub mod LUT410H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 411 high
pub mod LUT411H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 412 high
pub mod LUT412H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 413 high
pub mod LUT413H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 414 high
pub mod LUT414H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 415 high
pub mod LUT415H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 416 high
pub mod LUT416H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 417 high
pub mod LUT417H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 418 high
pub mod LUT418H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 419 high
pub mod LUT419H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 420 high
pub mod LUT420H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 421 high
pub mod LUT421H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 422 high
pub mod LUT422H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 423 high
pub mod LUT423H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 424 high
pub mod LUT424H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 425 high
pub mod LUT425H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 426 high
pub mod LUT426H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 427 high
pub mod LUT427H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 428 high
pub mod LUT428H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 429 high
pub mod LUT429H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 430 high
pub mod LUT430H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 431 high
pub mod LUT431H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 432 high
pub mod LUT432H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 433 high
pub mod LUT433H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 434 high
pub mod LUT434H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 435 high
pub mod LUT435H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 436 high
pub mod LUT436H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 437 high
pub mod LUT437H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 438 high
pub mod LUT438H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 439 high
pub mod LUT439H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 440 high
pub mod LUT440H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 441 high
pub mod LUT441H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 442 high
pub mod LUT442H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 443 high
pub mod LUT443H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 444 high
pub mod LUT444H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 445 high
pub mod LUT445H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 446 high
pub mod LUT446H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 447 high
pub mod LUT447H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 448 high
pub mod LUT448H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 449 high
pub mod LUT449H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 450 high
pub mod LUT450H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 451 high
pub mod LUT451H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 452 high
pub mod LUT452H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 453 high
pub mod LUT453H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 454 high
pub mod LUT454H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 455 high
pub mod LUT455H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 456 high
pub mod LUT456H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 457 high
pub mod LUT457H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 458 high
pub mod LUT458H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 459 high
pub mod LUT459H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 460 high
pub mod LUT460H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 461 high
pub mod LUT461H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 462 high
pub mod LUT462H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 463 high
pub mod LUT463H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 464 high
pub mod LUT464H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 465 high
pub mod LUT465H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 466 high
pub mod LUT466H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 467 high
pub mod LUT467H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 468 high
pub mod LUT468H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 469 high
pub mod LUT469H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 470 high
pub mod LUT470H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 471 high
pub mod LUT471H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 472 high
pub mod LUT472H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 473 high
pub mod LUT473H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 474 high
pub mod LUT474H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 475 high
pub mod LUT475H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 476 high
pub mod LUT476H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 477 high
pub mod LUT477H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 478 high
pub mod LUT478H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 479 high
pub mod LUT479H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 480 high
pub mod LUT480H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 481 high
pub mod LUT481H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 482 high
pub mod LUT482H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 483 high
pub mod LUT483H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 484 high
pub mod LUT484H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 485 high
pub mod LUT485H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 486 high
pub mod LUT486H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 487 high
pub mod LUT487H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 488 high
pub mod LUT488H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 489 high
pub mod LUT489H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 490 high
pub mod LUT490H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 491 high
pub mod LUT491H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 492 high
pub mod LUT492H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 493 high
pub mod LUT493H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 494 high
pub mod LUT494H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 495 high
pub mod LUT495H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 496 high
pub mod LUT496H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 497 high
pub mod LUT497H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 498 high
pub mod LUT498H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 499 high
pub mod LUT499H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 500 high
pub mod LUT500H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 501 high
pub mod LUT501H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 502 high
pub mod LUT502H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 503 high
pub mod LUT503H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 504 high
pub mod LUT504H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 505 high
pub mod LUT505H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 506 high
pub mod LUT506H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 507 high
pub mod LUT507H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 508 high
pub mod LUT508H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 509 high
pub mod LUT509H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 510 high
pub mod LUT510H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 511 high
pub mod LUT511H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 512 high
pub mod LUT512H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 513 high
pub mod LUT513H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 514 high
pub mod LUT514H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 515 high
pub mod LUT515H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 516 high
pub mod LUT516H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 517 high
pub mod LUT517H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 518 high
pub mod LUT518H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 519 high
pub mod LUT519H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 520 high
pub mod LUT520H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 521 high
pub mod LUT521H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 522 high
pub mod LUT522H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 523 high
pub mod LUT523H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 524 high
pub mod LUT524H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 525 high
pub mod LUT525H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 526 high
pub mod LUT526H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 527 high
pub mod LUT527H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 528 high
pub mod LUT528H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 529 high
pub mod LUT529H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 530 high
pub mod LUT530H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 531 high
pub mod LUT531H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 532 high
pub mod LUT532H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 533 high
pub mod LUT533H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 534 high
pub mod LUT534H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 535 high
pub mod LUT535H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 536 high
pub mod LUT536H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 537 high
pub mod LUT537H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 538 high
pub mod LUT538H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 539 high
pub mod LUT539H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 540 high
pub mod LUT540H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 541 high
pub mod LUT541H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 542 high
pub mod LUT542H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 543 high
pub mod LUT543H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 544 high
pub mod LUT544H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 545 high
pub mod LUT545H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 546 high
pub mod LUT546H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 547 high
pub mod LUT547H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 548 high
pub mod LUT548H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 549 high
pub mod LUT549H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 550 high
pub mod LUT550H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 551 high
pub mod LUT551H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 552 high
pub mod LUT552H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 553 high
pub mod LUT553H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 554 high
pub mod LUT554H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 555 high
pub mod LUT555H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 556 high
pub mod LUT556H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 557 high
pub mod LUT557H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 558 high
pub mod LUT558H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 559 high
pub mod LUT559H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 560 high
pub mod LUT560H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 561 high
pub mod LUT561H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 562 high
pub mod LUT562H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 563 high
pub mod LUT563H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 564 high
pub mod LUT564H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 565 high
pub mod LUT565H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 566 high
pub mod LUT566H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 567 high
pub mod LUT567H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 568 high
pub mod LUT568H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 569 high
pub mod LUT569H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 570 high
pub mod LUT570H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 571 high
pub mod LUT571H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 572 high
pub mod LUT572H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 573 high
pub mod LUT573H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 574 high
pub mod LUT574H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 575 high
pub mod LUT575H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 576 high
pub mod LUT576H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 577 high
pub mod LUT577H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 578 high
pub mod LUT578H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 579 high
pub mod LUT579H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 580 high
pub mod LUT580H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 581 high
pub mod LUT581H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 582 high
pub mod LUT582H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 583 high
pub mod LUT583H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 584 high
pub mod LUT584H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 585 high
pub mod LUT585H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 586 high
pub mod LUT586H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 587 high
pub mod LUT587H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 588 high
pub mod LUT588H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 589 high
pub mod LUT589H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 590 high
pub mod LUT590H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 591 high
pub mod LUT591H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 592 high
pub mod LUT592H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 593 high
pub mod LUT593H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 594 high
pub mod LUT594H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 595 high
pub mod LUT595H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 596 high
pub mod LUT596H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 597 high
pub mod LUT597H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 598 high
pub mod LUT598H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 599 high
pub mod LUT599H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 600 high
pub mod LUT600H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 601 high
pub mod LUT601H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 602 high
pub mod LUT602H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 603 high
pub mod LUT603H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 604 high
pub mod LUT604H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 605 high
pub mod LUT605H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 606 high
pub mod LUT606H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 607 high
pub mod LUT607H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 608 high
pub mod LUT608H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 609 high
pub mod LUT609H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 610 high
pub mod LUT610H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 611 high
pub mod LUT611H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 612 high
pub mod LUT612H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 613 high
pub mod LUT613H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 614 high
pub mod LUT614H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 615 high
pub mod LUT615H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 616 high
pub mod LUT616H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 617 high
pub mod LUT617H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 618 high
pub mod LUT618H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 619 high
pub mod LUT619H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 620 high
pub mod LUT620H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 621 high
pub mod LUT621H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 622 high
pub mod LUT622H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 623 high
pub mod LUT623H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 624 high
pub mod LUT624H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 625 high
pub mod LUT625H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 626 high
pub mod LUT626H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 627 high
pub mod LUT627H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 628 high
pub mod LUT628H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 629 high
pub mod LUT629H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 630 high
pub mod LUT630H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 631 high
pub mod LUT631H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 632 high
pub mod LUT632H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 633 high
pub mod LUT633H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 634 high
pub mod LUT634H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 635 high
pub mod LUT635H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 636 high
pub mod LUT636H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 637 high
pub mod LUT637H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 638 high
pub mod LUT638H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 639 high
pub mod LUT639H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 640 high
pub mod LUT640H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 641 high
pub mod LUT641H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 642 high
pub mod LUT642H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 643 high
pub mod LUT643H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 644 high
pub mod LUT644H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 645 high
pub mod LUT645H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 646 high
pub mod LUT646H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 647 high
pub mod LUT647H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 648 high
pub mod LUT648H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 649 high
pub mod LUT649H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 650 high
pub mod LUT650H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 651 high
pub mod LUT651H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 652 high
pub mod LUT652H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 653 high
pub mod LUT653H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 654 high
pub mod LUT654H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 655 high
pub mod LUT655H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 656 high
pub mod LUT656H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 657 high
pub mod LUT657H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 658 high
pub mod LUT658H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 659 high
pub mod LUT659H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 660 high
pub mod LUT660H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 661 high
pub mod LUT661H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 662 high
pub mod LUT662H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 663 high
pub mod LUT663H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 664 high
pub mod LUT664H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 665 high
pub mod LUT665H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 666 high
pub mod LUT666H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 667 high
pub mod LUT667H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 668 high
pub mod LUT668H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 669 high
pub mod LUT669H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 670 high
pub mod LUT670H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 671 high
pub mod LUT671H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 672 high
pub mod LUT672H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 673 high
pub mod LUT673H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 674 high
pub mod LUT674H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 675 high
pub mod LUT675H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 676 high
pub mod LUT676H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 677 high
pub mod LUT677H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 678 high
pub mod LUT678H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 679 high
pub mod LUT679H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 680 high
pub mod LUT680H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 681 high
pub mod LUT681H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 682 high
pub mod LUT682H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 683 high
pub mod LUT683H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 684 high
pub mod LUT684H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 685 high
pub mod LUT685H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 686 high
pub mod LUT686H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 687 high
pub mod LUT687H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 688 high
pub mod LUT688H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 689 high
pub mod LUT689H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 690 high
pub mod LUT690H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 691 high
pub mod LUT691H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 692 high
pub mod LUT692H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 693 high
pub mod LUT693H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 694 high
pub mod LUT694H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 695 high
pub mod LUT695H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 696 high
pub mod LUT696H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 697 high
pub mod LUT697H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 698 high
pub mod LUT698H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 699 high
pub mod LUT699H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 700 high
pub mod LUT700H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 701 high
pub mod LUT701H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 702 high
pub mod LUT702H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 703 high
pub mod LUT703H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 704 high
pub mod LUT704H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 705 high
pub mod LUT705H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 706 high
pub mod LUT706H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 707 high
pub mod LUT707H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 708 high
pub mod LUT708H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 709 high
pub mod LUT709H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 710 high
pub mod LUT710H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 711 high
pub mod LUT711H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 712 high
pub mod LUT712H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 713 high
pub mod LUT713H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 714 high
pub mod LUT714H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 715 high
pub mod LUT715H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 716 high
pub mod LUT716H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 717 high
pub mod LUT717H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 718 high
pub mod LUT718H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 719 high
pub mod LUT719H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 720 high
pub mod LUT720H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 721 high
pub mod LUT721H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 722 high
pub mod LUT722H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 723 high
pub mod LUT723H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 724 high
pub mod LUT724H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 725 high
pub mod LUT725H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 726 high
pub mod LUT726H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 727 high
pub mod LUT727H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 728 high
pub mod LUT728H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 729 high
pub mod LUT729H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 730 high
pub mod LUT730H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 731 high
pub mod LUT731H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 732 high
pub mod LUT732H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 733 high
pub mod LUT733H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 734 high
pub mod LUT734H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 735 high
pub mod LUT735H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 736 high
pub mod LUT736H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 737 high
pub mod LUT737H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 738 high
pub mod LUT738H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 739 high
pub mod LUT739H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 740 high
pub mod LUT740H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 741 high
pub mod LUT741H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 742 high
pub mod LUT742H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 743 high
pub mod LUT743H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 744 high
pub mod LUT744H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 745 high
pub mod LUT745H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 746 high
pub mod LUT746H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 747 high
pub mod LUT747H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 748 high
pub mod LUT748H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 749 high
pub mod LUT749H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 750 high
pub mod LUT750H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 751 high
pub mod LUT751H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 752 high
pub mod LUT752H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 753 high
pub mod LUT753H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 754 high
pub mod LUT754H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 755 high
pub mod LUT755H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 756 high
pub mod LUT756H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 757 high
pub mod LUT757H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 758 high
pub mod LUT758H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 759 high
pub mod LUT759H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 760 high
pub mod LUT760H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 761 high
pub mod LUT761H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 762 high
pub mod LUT762H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 763 high
pub mod LUT763H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 764 high
pub mod LUT764H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 765 high
pub mod LUT765H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 766 high
pub mod LUT766H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 767 high
pub mod LUT767H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 768 high
pub mod LUT768H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 769 high
pub mod LUT769H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 770 high
pub mod LUT770H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 771 high
pub mod LUT771H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 772 high
pub mod LUT772H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 773 high
pub mod LUT773H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 774 high
pub mod LUT774H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 775 high
pub mod LUT775H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 776 high
pub mod LUT776H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 777 high
pub mod LUT777H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 778 high
pub mod LUT778H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 779 high
pub mod LUT779H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 780 high
pub mod LUT780H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 781 high
pub mod LUT781H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 782 high
pub mod LUT782H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 783 high
pub mod LUT783H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 784 high
pub mod LUT784H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 785 high
pub mod LUT785H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 786 high
pub mod LUT786H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 787 high
pub mod LUT787H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 788 high
pub mod LUT788H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 789 high
pub mod LUT789H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 790 high
pub mod LUT790H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 791 high
pub mod LUT791H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 792 high
pub mod LUT792H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 793 high
pub mod LUT793H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 794 high
pub mod LUT794H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 795 high
pub mod LUT795H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 796 high
pub mod LUT796H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 797 high
pub mod LUT797H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 798 high
pub mod LUT798H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 799 high
pub mod LUT799H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 800 high
pub mod LUT800H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 801 high
pub mod LUT801H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 802 high
pub mod LUT802H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 803 high
pub mod LUT803H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 804 high
pub mod LUT804H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 805 high
pub mod LUT805H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 806 high
pub mod LUT806H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 807 high
pub mod LUT807H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 808 high
pub mod LUT808H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 809 high
pub mod LUT809H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 810 high
pub mod LUT810H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 811 high
pub mod LUT811H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 812 high
pub mod LUT812H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 813 high
pub mod LUT813H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 814 high
pub mod LUT814H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 815 high
pub mod LUT815H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 816 high
pub mod LUT816H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 817 high
pub mod LUT817H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 818 high
pub mod LUT818H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 819 high
pub mod LUT819H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 820 high
pub mod LUT820H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 821 high
pub mod LUT821H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 822 high
pub mod LUT822H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 823 high
pub mod LUT823H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 824 high
pub mod LUT824H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 825 high
pub mod LUT825H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 826 high
pub mod LUT826H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 827 high
pub mod LUT827H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 828 high
pub mod LUT828H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 829 high
pub mod LUT829H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 830 high
pub mod LUT830H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 831 high
pub mod LUT831H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 832 high
pub mod LUT832H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 833 high
pub mod LUT833H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 834 high
pub mod LUT834H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 835 high
pub mod LUT835H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 836 high
pub mod LUT836H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 837 high
pub mod LUT837H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 838 high
pub mod LUT838H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 839 high
pub mod LUT839H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 840 high
pub mod LUT840H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 841 high
pub mod LUT841H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 842 high
pub mod LUT842H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 843 high
pub mod LUT843H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 844 high
pub mod LUT844H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 845 high
pub mod LUT845H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 846 high
pub mod LUT846H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 847 high
pub mod LUT847H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 848 high
pub mod LUT848H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 849 high
pub mod LUT849H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 850 high
pub mod LUT850H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 851 high
pub mod LUT851H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 852 high
pub mod LUT852H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 853 high
pub mod LUT853H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 854 high
pub mod LUT854H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 855 high
pub mod LUT855H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 856 high
pub mod LUT856H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 857 high
pub mod LUT857H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 858 high
pub mod LUT858H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 859 high
pub mod LUT859H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 860 high
pub mod LUT860H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 861 high
pub mod LUT861H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 862 high
pub mod LUT862H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 863 high
pub mod LUT863H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 864 high
pub mod LUT864H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 865 high
pub mod LUT865H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 866 high
pub mod LUT866H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 867 high
pub mod LUT867H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 868 high
pub mod LUT868H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 869 high
pub mod LUT869H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 870 high
pub mod LUT870H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 871 high
pub mod LUT871H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 872 high
pub mod LUT872H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 873 high
pub mod LUT873H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 874 high
pub mod LUT874H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 875 high
pub mod LUT875H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 876 high
pub mod LUT876H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 877 high
pub mod LUT877H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 878 high
pub mod LUT878H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 879 high
pub mod LUT879H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 880 high
pub mod LUT880H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 881 high
pub mod LUT881H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 882 high
pub mod LUT882H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 883 high
pub mod LUT883H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 884 high
pub mod LUT884H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 885 high
pub mod LUT885H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 886 high
pub mod LUT886H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 887 high
pub mod LUT887H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 888 high
pub mod LUT888H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 889 high
pub mod LUT889H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 890 high
pub mod LUT890H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 891 high
pub mod LUT891H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 892 high
pub mod LUT892H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 893 high
pub mod LUT893H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 894 high
pub mod LUT894H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 895 high
pub mod LUT895H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 896 high
pub mod LUT896H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 897 high
pub mod LUT897H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 898 high
pub mod LUT898H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 899 high
pub mod LUT899H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 900 high
pub mod LUT900H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 901 high
pub mod LUT901H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 902 high
pub mod LUT902H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 903 high
pub mod LUT903H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 904 high
pub mod LUT904H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 905 high
pub mod LUT905H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 906 high
pub mod LUT906H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 907 high
pub mod LUT907H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 908 high
pub mod LUT908H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 909 high
pub mod LUT909H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 910 high
pub mod LUT910H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 911 high
pub mod LUT911H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 912 high
pub mod LUT912H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 913 high
pub mod LUT913H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 914 high
pub mod LUT914H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 915 high
pub mod LUT915H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 916 high
pub mod LUT916H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 917 high
pub mod LUT917H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 918 high
pub mod LUT918H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 919 high
pub mod LUT919H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 920 high
pub mod LUT920H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 921 high
pub mod LUT921H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 922 high
pub mod LUT922H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 923 high
pub mod LUT923H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 924 high
pub mod LUT924H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 925 high
pub mod LUT925H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 926 high
pub mod LUT926H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 927 high
pub mod LUT927H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 928 high
pub mod LUT928H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 929 high
pub mod LUT929H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 930 high
pub mod LUT930H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 931 high
pub mod LUT931H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 932 high
pub mod LUT932H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 933 high
pub mod LUT933H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 934 high
pub mod LUT934H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 935 high
pub mod LUT935H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 936 high
pub mod LUT936H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 937 high
pub mod LUT937H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 938 high
pub mod LUT938H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 939 high
pub mod LUT939H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 940 high
pub mod LUT940H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 941 high
pub mod LUT941H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 942 high
pub mod LUT942H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 943 high
pub mod LUT943H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 944 high
pub mod LUT944H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 945 high
pub mod LUT945H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 946 high
pub mod LUT946H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 947 high
pub mod LUT947H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 948 high
pub mod LUT948H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 949 high
pub mod LUT949H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 950 high
pub mod LUT950H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 951 high
pub mod LUT951H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 952 high
pub mod LUT952H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 953 high
pub mod LUT953H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 954 high
pub mod LUT954H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 955 high
pub mod LUT955H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 956 high
pub mod LUT956H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 957 high
pub mod LUT957H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 958 high
pub mod LUT958H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 959 high
pub mod LUT959H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 960 high
pub mod LUT960H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 961 high
pub mod LUT961H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 962 high
pub mod LUT962H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 963 high
pub mod LUT963H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 964 high
pub mod LUT964H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 965 high
pub mod LUT965H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 966 high
pub mod LUT966H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 967 high
pub mod LUT967H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 968 high
pub mod LUT968H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 969 high
pub mod LUT969H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 970 high
pub mod LUT970H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 971 high
pub mod LUT971H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 972 high
pub mod LUT972H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 973 high
pub mod LUT973H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 974 high
pub mod LUT974H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 975 high
pub mod LUT975H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 976 high
pub mod LUT976H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 977 high
pub mod LUT977H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 978 high
pub mod LUT978H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 979 high
pub mod LUT979H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 980 high
pub mod LUT980H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 981 high
pub mod LUT981H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 982 high
pub mod LUT982H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 983 high
pub mod LUT983H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 984 high
pub mod LUT984H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 985 high
pub mod LUT985H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 986 high
pub mod LUT986H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 987 high
pub mod LUT987H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 988 high
pub mod LUT988H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 989 high
pub mod LUT989H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 990 high
pub mod LUT990H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 991 high
pub mod LUT991H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 992 high
pub mod LUT992H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 993 high
pub mod LUT993H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 994 high
pub mod LUT994H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 995 high
pub mod LUT995H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 996 high
pub mod LUT996H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 997 high
pub mod LUT997H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 998 high
pub mod LUT998H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 999 high
pub mod LUT999H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 1000 high
pub mod LUT1000H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 1001 high
pub mod LUT1001H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 1002 high
pub mod LUT1002H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 1003 high
pub mod LUT1003H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 1004 high
pub mod LUT1004H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 1005 high
pub mod LUT1005H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 1006 high
pub mod LUT1006H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 1007 high
pub mod LUT1007H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 1008 high
pub mod LUT1008H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 1009 high
pub mod LUT1009H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 1010 high
pub mod LUT1010H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 1011 high
pub mod LUT1011H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 1012 high
pub mod LUT1012H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 1013 high
pub mod LUT1013H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 1014 high
pub mod LUT1014H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 1015 high
pub mod LUT1015H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 1016 high
pub mod LUT1016H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 1017 high
pub mod LUT1017H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 1018 high
pub mod LUT1018H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 1019 high
pub mod LUT1019H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 1020 high
pub mod LUT1020H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 1021 high
pub mod LUT1021H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 1022 high
pub mod LUT1022H {
    pub use super::LUT0H::LO;
}

/// Graphic MMU LUT entry 1023 high
pub mod LUT1023H {
    pub use super::LUT0H::LO;
}
#[repr(C)]
pub struct RegisterBlock {
    /// Graphic MMU configuration register
    pub CR: RWRegister<u32>,

    /// Graphic MMU status register
    pub SR: RORegister<u32>,

    /// Graphic MMU flag clear register
    pub FCR: WORegister<u32>,

    _reserved1: [u8; 4],

    /// Graphic MMU default value register
    pub DVR: RWRegister<u32>,

    _reserved2: [u8; 12],

    /// Graphic MMU buffer 0 configuration register
    pub B0CR: RWRegister<u32>,

    /// Graphic MMU buffer 1 configuration register
    pub B1CR: RWRegister<u32>,

    /// Graphic MMU buffer 2 configuration register
    pub B2CR: RWRegister<u32>,

    /// Graphic MMU buffer 3 configuration register
    pub B3CR: RWRegister<u32>,

    _reserved3: [u8; 4036],

    /// Graphic MMU version register
    pub VERR: RORegister<u32>,

    /// Graphic MMU identification register
    pub IPIDR: RORegister<u32>,

    /// Graphic MMU size identification register
    pub SIDR: RORegister<u32>,

    /// Graphic MMU LUT entry 0 low
    pub LUT0L: RWRegister<u32>,

    /// Graphic MMU LUT entry 0 high
    pub LUT0H: RWRegister<u32>,

    /// Graphic MMU LUT entry 1 low
    pub LUT1L: RWRegister<u32>,

    /// Graphic MMU LUT entry 1 high
    pub LUT1H: RWRegister<u32>,

    /// Graphic MMU LUT entry 2 low
    pub LUT2L: RWRegister<u32>,

    /// Graphic MMU LUT entry 2 high
    pub LUT2H: RWRegister<u32>,

    /// Graphic MMU LUT entry 3 low
    pub LUT3L: RWRegister<u32>,

    /// Graphic MMU LUT entry 3 high
    pub LUT3H: RWRegister<u32>,

    /// Graphic MMU LUT entry 4 low
    pub LUT4L: RWRegister<u32>,

    /// Graphic MMU LUT entry 4 high
    pub LUT4H: RWRegister<u32>,

    /// Graphic MMU LUT entry 5 low
    pub LUT5L: RWRegister<u32>,

    /// Graphic MMU LUT entry 5 high
    pub LUT5H: RWRegister<u32>,

    /// Graphic MMU LUT entry 6 low
    pub LUT6L: RWRegister<u32>,

    /// Graphic MMU LUT entry 6 high
    pub LUT6H: RWRegister<u32>,

    /// Graphic MMU LUT entry 7 low
    pub LUT7L: RWRegister<u32>,

    /// Graphic MMU LUT entry 7 high
    pub LUT7H: RWRegister<u32>,

    /// Graphic MMU LUT entry 8 low
    pub LUT8L: RWRegister<u32>,

    /// Graphic MMU LUT entry 8 high
    pub LUT8H: RWRegister<u32>,

    /// Graphic MMU LUT entry 9 low
    pub LUT9L: RWRegister<u32>,

    /// Graphic MMU LUT entry 9 high
    pub LUT9H: RWRegister<u32>,

    /// Graphic MMU LUT entry 10 low
    pub LUT10L: RWRegister<u32>,

    /// Graphic MMU LUT entry 10 high
    pub LUT10H: RWRegister<u32>,

    /// Graphic MMU LUT entry 11 low
    pub LUT11L: RWRegister<u32>,

    /// Graphic MMU LUT entry 11 high
    pub LUT11H: RWRegister<u32>,

    /// Graphic MMU LUT entry 12 low
    pub LUT12L: RWRegister<u32>,

    /// Graphic MMU LUT entry 12 high
    pub LUT12H: RWRegister<u32>,

    /// Graphic MMU LUT entry 13 low
    pub LUT13L: RWRegister<u32>,

    /// Graphic MMU LUT entry 13 high
    pub LUT13H: RWRegister<u32>,

    /// Graphic MMU LUT entry 14 low
    pub LUT14L: RWRegister<u32>,

    /// Graphic MMU LUT entry 14 high
    pub LUT14H: RWRegister<u32>,

    /// Graphic MMU LUT entry 15 low
    pub LUT15L: RWRegister<u32>,

    /// Graphic MMU LUT entry 15 high
    pub LUT15H: RWRegister<u32>,

    /// Graphic MMU LUT entry 16 low
    pub LUT16L: RWRegister<u32>,

    /// Graphic MMU LUT entry 16 high
    pub LUT16H: RWRegister<u32>,

    /// Graphic MMU LUT entry 17 low
    pub LUT17L: RWRegister<u32>,

    /// Graphic MMU LUT entry 17 high
    pub LUT17H: RWRegister<u32>,

    /// Graphic MMU LUT entry 18 low
    pub LUT18L: RWRegister<u32>,

    /// Graphic MMU LUT entry 18 high
    pub LUT18H: RWRegister<u32>,

    /// Graphic MMU LUT entry 19 low
    pub LUT19L: RWRegister<u32>,

    /// Graphic MMU LUT entry 19 high
    pub LUT19H: RWRegister<u32>,

    /// Graphic MMU LUT entry 20 low
    pub LUT20L: RWRegister<u32>,

    /// Graphic MMU LUT entry 20 high
    pub LUT20H: RWRegister<u32>,

    /// Graphic MMU LUT entry 21 low
    pub LUT21L: RWRegister<u32>,

    /// Graphic MMU LUT entry 21 high
    pub LUT21H: RWRegister<u32>,

    /// Graphic MMU LUT entry 22 low
    pub LUT22L: RWRegister<u32>,

    /// Graphic MMU LUT entry 22 high
    pub LUT22H: RWRegister<u32>,

    /// Graphic MMU LUT entry 23 low
    pub LUT23L: RWRegister<u32>,

    /// Graphic MMU LUT entry 23 high
    pub LUT23H: RWRegister<u32>,

    /// Graphic MMU LUT entry 24 low
    pub LUT24L: RWRegister<u32>,

    /// Graphic MMU LUT entry 24 high
    pub LUT24H: RWRegister<u32>,

    /// Graphic MMU LUT entry 25 low
    pub LUT25L: RWRegister<u32>,

    /// Graphic MMU LUT entry 25 high
    pub LUT25H: RWRegister<u32>,

    /// Graphic MMU LUT entry 26 low
    pub LUT26L: RWRegister<u32>,

    /// Graphic MMU LUT entry 26 high
    pub LUT26H: RWRegister<u32>,

    /// Graphic MMU LUT entry 27 low
    pub LUT27L: RWRegister<u32>,

    /// Graphic MMU LUT entry 27 high
    pub LUT27H: RWRegister<u32>,

    /// Graphic MMU LUT entry 28 low
    pub LUT28L: RWRegister<u32>,

    /// Graphic MMU LUT entry 28 high
    pub LUT28H: RWRegister<u32>,

    /// Graphic MMU LUT entry 29 low
    pub LUT29L: RWRegister<u32>,

    /// Graphic MMU LUT entry 29 high
    pub LUT29H: RWRegister<u32>,

    /// Graphic MMU LUT entry 30 low
    pub LUT30L: RWRegister<u32>,

    /// Graphic MMU LUT entry 30 high
    pub LUT30H: RWRegister<u32>,

    /// Graphic MMU LUT entry 31 low
    pub LUT31L: RWRegister<u32>,

    /// Graphic MMU LUT entry 31 high
    pub LUT31H: RWRegister<u32>,

    /// Graphic MMU LUT entry 32 low
    pub LUT32L: RWRegister<u32>,

    /// Graphic MMU LUT entry 32 high
    pub LUT32H: RWRegister<u32>,

    /// Graphic MMU LUT entry 33 low
    pub LUT33L: RWRegister<u32>,

    /// Graphic MMU LUT entry 33 high
    pub LUT33H: RWRegister<u32>,

    /// Graphic MMU LUT entry 34 low
    pub LUT34L: RWRegister<u32>,

    /// Graphic MMU LUT entry 34 high
    pub LUT34H: RWRegister<u32>,

    /// Graphic MMU LUT entry 35 low
    pub LUT35L: RWRegister<u32>,

    /// Graphic MMU LUT entry 35 high
    pub LUT35H: RWRegister<u32>,

    /// Graphic MMU LUT entry 36 low
    pub LUT36L: RWRegister<u32>,

    /// Graphic MMU LUT entry 36 high
    pub LUT36H: RWRegister<u32>,

    /// Graphic MMU LUT entry 37 low
    pub LUT37L: RWRegister<u32>,

    /// Graphic MMU LUT entry 37 high
    pub LUT37H: RWRegister<u32>,

    /// Graphic MMU LUT entry 38 low
    pub LUT38L: RWRegister<u32>,

    /// Graphic MMU LUT entry 38 high
    pub LUT38H: RWRegister<u32>,

    /// Graphic MMU LUT entry 39 low
    pub LUT39L: RWRegister<u32>,

    /// Graphic MMU LUT entry 39 high
    pub LUT39H: RWRegister<u32>,

    /// Graphic MMU LUT entry 40 low
    pub LUT40L: RWRegister<u32>,

    /// Graphic MMU LUT entry 40 high
    pub LUT40H: RWRegister<u32>,

    /// Graphic MMU LUT entry 41 low
    pub LUT41L: RWRegister<u32>,

    /// Graphic MMU LUT entry 41 high
    pub LUT41H: RWRegister<u32>,

    /// Graphic MMU LUT entry 42 low
    pub LUT42L: RWRegister<u32>,

    /// Graphic MMU LUT entry 42 high
    pub LUT42H: RWRegister<u32>,

    /// Graphic MMU LUT entry 43 low
    pub LUT43L: RWRegister<u32>,

    /// Graphic MMU LUT entry 43 high
    pub LUT43H: RWRegister<u32>,

    /// Graphic MMU LUT entry 44 low
    pub LUT44L: RWRegister<u32>,

    /// Graphic MMU LUT entry 44 high
    pub LUT44H: RWRegister<u32>,

    /// Graphic MMU LUT entry 45 low
    pub LUT45L: RWRegister<u32>,

    /// Graphic MMU LUT entry 45 high
    pub LUT45H: RWRegister<u32>,

    /// Graphic MMU LUT entry 46 low
    pub LUT46L: RWRegister<u32>,

    /// Graphic MMU LUT entry 46 high
    pub LUT46H: RWRegister<u32>,

    /// Graphic MMU LUT entry 47 low
    pub LUT47L: RWRegister<u32>,

    /// Graphic MMU LUT entry 47 high
    pub LUT47H: RWRegister<u32>,

    /// Graphic MMU LUT entry 48 low
    pub LUT48L: RWRegister<u32>,

    /// Graphic MMU LUT entry 48 high
    pub LUT48H: RWRegister<u32>,

    /// Graphic MMU LUT entry 49 low
    pub LUT49L: RWRegister<u32>,

    /// Graphic MMU LUT entry 49 high
    pub LUT49H: RWRegister<u32>,

    /// Graphic MMU LUT entry 50 low
    pub LUT50L: RWRegister<u32>,

    /// Graphic MMU LUT entry 50 high
    pub LUT50H: RWRegister<u32>,

    /// Graphic MMU LUT entry 51 low
    pub LUT51L: RWRegister<u32>,

    /// Graphic MMU LUT entry 51 high
    pub LUT51H: RWRegister<u32>,

    /// Graphic MMU LUT entry 52 low
    pub LUT52L: RWRegister<u32>,

    /// Graphic MMU LUT entry 52 high
    pub LUT52H: RWRegister<u32>,

    /// Graphic MMU LUT entry 53 low
    pub LUT53L: RWRegister<u32>,

    /// Graphic MMU LUT entry 53 high
    pub LUT53H: RWRegister<u32>,

    /// Graphic MMU LUT entry 54 low
    pub LUT54L: RWRegister<u32>,

    /// Graphic MMU LUT entry 54 high
    pub LUT54H: RWRegister<u32>,

    /// Graphic MMU LUT entry 55 low
    pub LUT55L: RWRegister<u32>,

    /// Graphic MMU LUT entry 55 high
    pub LUT55H: RWRegister<u32>,

    /// Graphic MMU LUT entry 56 low
    pub LUT56L: RWRegister<u32>,

    /// Graphic MMU LUT entry 56 high
    pub LUT56H: RWRegister<u32>,

    /// Graphic MMU LUT entry 57 low
    pub LUT57L: RWRegister<u32>,

    /// Graphic MMU LUT entry 57 high
    pub LUT57H: RWRegister<u32>,

    /// Graphic MMU LUT entry 58 low
    pub LUT58L: RWRegister<u32>,

    /// Graphic MMU LUT entry 58 high
    pub LUT58H: RWRegister<u32>,

    /// Graphic MMU LUT entry 59 low
    pub LUT59L: RWRegister<u32>,

    /// Graphic MMU LUT entry 59 high
    pub LUT59H: RWRegister<u32>,

    /// Graphic MMU LUT entry 60 low
    pub LUT60L: RWRegister<u32>,

    /// Graphic MMU LUT entry 60 high
    pub LUT60H: RWRegister<u32>,

    /// Graphic MMU LUT entry 61 low
    pub LUT61L: RWRegister<u32>,

    /// Graphic MMU LUT entry 61 high
    pub LUT61H: RWRegister<u32>,

    /// Graphic MMU LUT entry 62 low
    pub LUT62L: RWRegister<u32>,

    /// Graphic MMU LUT entry 62 high
    pub LUT62H: RWRegister<u32>,

    /// Graphic MMU LUT entry 63 low
    pub LUT63L: RWRegister<u32>,

    /// Graphic MMU LUT entry 63 high
    pub LUT63H: RWRegister<u32>,

    /// Graphic MMU LUT entry 64 low
    pub LUT64L: RWRegister<u32>,

    /// Graphic MMU LUT entry 64 high
    pub LUT64H: RWRegister<u32>,

    /// Graphic MMU LUT entry 65 low
    pub LUT65L: RWRegister<u32>,

    /// Graphic MMU LUT entry 65 high
    pub LUT65H: RWRegister<u32>,

    /// Graphic MMU LUT entry 66 low
    pub LUT66L: RWRegister<u32>,

    /// Graphic MMU LUT entry 66 high
    pub LUT66H: RWRegister<u32>,

    /// Graphic MMU LUT entry 67 low
    pub LUT67L: RWRegister<u32>,

    /// Graphic MMU LUT entry 67 high
    pub LUT67H: RWRegister<u32>,

    /// Graphic MMU LUT entry 68 low
    pub LUT68L: RWRegister<u32>,

    /// Graphic MMU LUT entry 68 high
    pub LUT68H: RWRegister<u32>,

    /// Graphic MMU LUT entry 69 low
    pub LUT69L: RWRegister<u32>,

    /// Graphic MMU LUT entry 69 high
    pub LUT69H: RWRegister<u32>,

    /// Graphic MMU LUT entry 70 low
    pub LUT70L: RWRegister<u32>,

    /// Graphic MMU LUT entry 70 high
    pub LUT70H: RWRegister<u32>,

    /// Graphic MMU LUT entry 71 low
    pub LUT71L: RWRegister<u32>,

    /// Graphic MMU LUT entry 71 high
    pub LUT71H: RWRegister<u32>,

    /// Graphic MMU LUT entry 72 low
    pub LUT72L: RWRegister<u32>,

    /// Graphic MMU LUT entry 72 high
    pub LUT72H: RWRegister<u32>,

    /// Graphic MMU LUT entry 73 low
    pub LUT73L: RWRegister<u32>,

    /// Graphic MMU LUT entry 73 high
    pub LUT73H: RWRegister<u32>,

    /// Graphic MMU LUT entry 74 low
    pub LUT74L: RWRegister<u32>,

    /// Graphic MMU LUT entry 74 high
    pub LUT74H: RWRegister<u32>,

    /// Graphic MMU LUT entry 75 low
    pub LUT75L: RWRegister<u32>,

    /// Graphic MMU LUT entry 75 high
    pub LUT75H: RWRegister<u32>,

    /// Graphic MMU LUT entry 76 low
    pub LUT76L: RWRegister<u32>,

    /// Graphic MMU LUT entry 76 high
    pub LUT76H: RWRegister<u32>,

    /// Graphic MMU LUT entry 77 low
    pub LUT77L: RWRegister<u32>,

    /// Graphic MMU LUT entry 77 high
    pub LUT77H: RWRegister<u32>,

    /// Graphic MMU LUT entry 78 low
    pub LUT78L: RWRegister<u32>,

    /// Graphic MMU LUT entry 78 high
    pub LUT78H: RWRegister<u32>,

    /// Graphic MMU LUT entry 79 low
    pub LUT79L: RWRegister<u32>,

    /// Graphic MMU LUT entry 79 high
    pub LUT79H: RWRegister<u32>,

    /// Graphic MMU LUT entry 80 low
    pub LUT80L: RWRegister<u32>,

    /// Graphic MMU LUT entry 80 high
    pub LUT80H: RWRegister<u32>,

    /// Graphic MMU LUT entry 81 low
    pub LUT81L: RWRegister<u32>,

    /// Graphic MMU LUT entry 81 high
    pub LUT81H: RWRegister<u32>,

    /// Graphic MMU LUT entry 82 low
    pub LUT82L: RWRegister<u32>,

    /// Graphic MMU LUT entry 82 high
    pub LUT82H: RWRegister<u32>,

    /// Graphic MMU LUT entry 83 low
    pub LUT83L: RWRegister<u32>,

    /// Graphic MMU LUT entry 83 high
    pub LUT83H: RWRegister<u32>,

    /// Graphic MMU LUT entry 84 low
    pub LUT84L: RWRegister<u32>,

    /// Graphic MMU LUT entry 84 high
    pub LUT84H: RWRegister<u32>,

    /// Graphic MMU LUT entry 85 low
    pub LUT85L: RWRegister<u32>,

    /// Graphic MMU LUT entry 85 high
    pub LUT85H: RWRegister<u32>,

    /// Graphic MMU LUT entry 86 low
    pub LUT86L: RWRegister<u32>,

    /// Graphic MMU LUT entry 86 high
    pub LUT86H: RWRegister<u32>,

    /// Graphic MMU LUT entry 87 low
    pub LUT87L: RWRegister<u32>,

    /// Graphic MMU LUT entry 87 high
    pub LUT87H: RWRegister<u32>,

    /// Graphic MMU LUT entry 88 low
    pub LUT88L: RWRegister<u32>,

    /// Graphic MMU LUT entry 88 high
    pub LUT88H: RWRegister<u32>,

    /// Graphic MMU LUT entry 89 low
    pub LUT89L: RWRegister<u32>,

    /// Graphic MMU LUT entry 89 high
    pub LUT89H: RWRegister<u32>,

    /// Graphic MMU LUT entry 90 low
    pub LUT90L: RWRegister<u32>,

    /// Graphic MMU LUT entry 90 high
    pub LUT90H: RWRegister<u32>,

    /// Graphic MMU LUT entry 91 low
    pub LUT91L: RWRegister<u32>,

    /// Graphic MMU LUT entry 91 high
    pub LUT91H: RWRegister<u32>,

    /// Graphic MMU LUT entry 92 low
    pub LUT92L: RWRegister<u32>,

    /// Graphic MMU LUT entry 92 high
    pub LUT92H: RWRegister<u32>,

    /// Graphic MMU LUT entry 93 low
    pub LUT93L: RWRegister<u32>,

    /// Graphic MMU LUT entry 93 high
    pub LUT93H: RWRegister<u32>,

    /// Graphic MMU LUT entry 94 low
    pub LUT94L: RWRegister<u32>,

    /// Graphic MMU LUT entry 94 high
    pub LUT94H: RWRegister<u32>,

    /// Graphic MMU LUT entry 95 low
    pub LUT95L: RWRegister<u32>,

    /// Graphic MMU LUT entry 95 high
    pub LUT95H: RWRegister<u32>,

    /// Graphic MMU LUT entry 96 low
    pub LUT96L: RWRegister<u32>,

    /// Graphic MMU LUT entry 96 high
    pub LUT96H: RWRegister<u32>,

    /// Graphic MMU LUT entry 97 low
    pub LUT97L: RWRegister<u32>,

    /// Graphic MMU LUT entry 97 high
    pub LUT97H: RWRegister<u32>,

    /// Graphic MMU LUT entry 98 low
    pub LUT98L: RWRegister<u32>,

    /// Graphic MMU LUT entry 98 high
    pub LUT98H: RWRegister<u32>,

    /// Graphic MMU LUT entry 99 low
    pub LUT99L: RWRegister<u32>,

    /// Graphic MMU LUT entry 99 high
    pub LUT99H: RWRegister<u32>,

    /// Graphic MMU LUT entry 100 low
    pub LUT100L: RWRegister<u32>,

    /// Graphic MMU LUT entry 100 high
    pub LUT100H: RWRegister<u32>,

    /// Graphic MMU LUT entry 101 low
    pub LUT101L: RWRegister<u32>,

    /// Graphic MMU LUT entry 101 high
    pub LUT101H: RWRegister<u32>,

    /// Graphic MMU LUT entry 102 low
    pub LUT102L: RWRegister<u32>,

    /// Graphic MMU LUT entry 102 high
    pub LUT102H: RWRegister<u32>,

    /// Graphic MMU LUT entry 103 low
    pub LUT103L: RWRegister<u32>,

    /// Graphic MMU LUT entry 103 high
    pub LUT103H: RWRegister<u32>,

    /// Graphic MMU LUT entry 104 low
    pub LUT104L: RWRegister<u32>,

    /// Graphic MMU LUT entry 104 high
    pub LUT104H: RWRegister<u32>,

    /// Graphic MMU LUT entry 105 low
    pub LUT105L: RWRegister<u32>,

    /// Graphic MMU LUT entry 105 high
    pub LUT105H: RWRegister<u32>,

    /// Graphic MMU LUT entry 106 low
    pub LUT106L: RWRegister<u32>,

    /// Graphic MMU LUT entry 106 high
    pub LUT106H: RWRegister<u32>,

    /// Graphic MMU LUT entry 107 low
    pub LUT107L: RWRegister<u32>,

    /// Graphic MMU LUT entry 107 high
    pub LUT107H: RWRegister<u32>,

    /// Graphic MMU LUT entry 108 low
    pub LUT108L: RWRegister<u32>,

    /// Graphic MMU LUT entry 108 high
    pub LUT108H: RWRegister<u32>,

    /// Graphic MMU LUT entry 109 low
    pub LUT109L: RWRegister<u32>,

    /// Graphic MMU LUT entry 109 high
    pub LUT109H: RWRegister<u32>,

    /// Graphic MMU LUT entry 110 low
    pub LUT110L: RWRegister<u32>,

    /// Graphic MMU LUT entry 110 high
    pub LUT110H: RWRegister<u32>,

    /// Graphic MMU LUT entry 111 low
    pub LUT111L: RWRegister<u32>,

    /// Graphic MMU LUT entry 111 high
    pub LUT111H: RWRegister<u32>,

    /// Graphic MMU LUT entry 112 low
    pub LUT112L: RWRegister<u32>,

    /// Graphic MMU LUT entry 112 high
    pub LUT112H: RWRegister<u32>,

    /// Graphic MMU LUT entry 113 low
    pub LUT113L: RWRegister<u32>,

    /// Graphic MMU LUT entry 113 high
    pub LUT113H: RWRegister<u32>,

    /// Graphic MMU LUT entry 114 low
    pub LUT114L: RWRegister<u32>,

    /// Graphic MMU LUT entry 114 high
    pub LUT114H: RWRegister<u32>,

    /// Graphic MMU LUT entry 115 low
    pub LUT115L: RWRegister<u32>,

    /// Graphic MMU LUT entry 115 high
    pub LUT115H: RWRegister<u32>,

    /// Graphic MMU LUT entry 116 low
    pub LUT116L: RWRegister<u32>,

    /// Graphic MMU LUT entry 116 high
    pub LUT116H: RWRegister<u32>,

    /// Graphic MMU LUT entry 117 low
    pub LUT117L: RWRegister<u32>,

    /// Graphic MMU LUT entry 117 high
    pub LUT117H: RWRegister<u32>,

    /// Graphic MMU LUT entry 118 low
    pub LUT118L: RWRegister<u32>,

    /// Graphic MMU LUT entry 118 high
    pub LUT118H: RWRegister<u32>,

    /// Graphic MMU LUT entry 119 low
    pub LUT119L: RWRegister<u32>,

    /// Graphic MMU LUT entry 119 high
    pub LUT119H: RWRegister<u32>,

    /// Graphic MMU LUT entry 120 low
    pub LUT120L: RWRegister<u32>,

    /// Graphic MMU LUT entry 120 high
    pub LUT120H: RWRegister<u32>,

    /// Graphic MMU LUT entry 121 low
    pub LUT121L: RWRegister<u32>,

    /// Graphic MMU LUT entry 121 high
    pub LUT121H: RWRegister<u32>,

    /// Graphic MMU LUT entry 122 low
    pub LUT122L: RWRegister<u32>,

    /// Graphic MMU LUT entry 122 high
    pub LUT122H: RWRegister<u32>,

    /// Graphic MMU LUT entry 123 low
    pub LUT123L: RWRegister<u32>,

    /// Graphic MMU LUT entry 123 high
    pub LUT123H: RWRegister<u32>,

    /// Graphic MMU LUT entry 124 low
    pub LUT124L: RWRegister<u32>,

    /// Graphic MMU LUT entry 124 high
    pub LUT124H: RWRegister<u32>,

    /// Graphic MMU LUT entry 125 low
    pub LUT125L: RWRegister<u32>,

    /// Graphic MMU LUT entry 125 high
    pub LUT125H: RWRegister<u32>,

    /// Graphic MMU LUT entry 126 low
    pub LUT126L: RWRegister<u32>,

    /// Graphic MMU LUT entry 126 high
    pub LUT126H: RWRegister<u32>,

    /// Graphic MMU LUT entry 127 low
    pub LUT127L: RWRegister<u32>,

    /// Graphic MMU LUT entry 127 high
    pub LUT127H: RWRegister<u32>,

    /// Graphic MMU LUT entry 128 low
    pub LUT128L: RWRegister<u32>,

    /// Graphic MMU LUT entry 128 high
    pub LUT128H: RWRegister<u32>,

    /// Graphic MMU LUT entry 129 low
    pub LUT129L: RWRegister<u32>,

    /// Graphic MMU LUT entry 129 high
    pub LUT129H: RWRegister<u32>,

    /// Graphic MMU LUT entry 130 low
    pub LUT130L: RWRegister<u32>,

    /// Graphic MMU LUT entry 130 high
    pub LUT130H: RWRegister<u32>,

    /// Graphic MMU LUT entry 131 low
    pub LUT131L: RWRegister<u32>,

    /// Graphic MMU LUT entry 131 high
    pub LUT131H: RWRegister<u32>,

    /// Graphic MMU LUT entry 132 low
    pub LUT132L: RWRegister<u32>,

    /// Graphic MMU LUT entry 132 high
    pub LUT132H: RWRegister<u32>,

    /// Graphic MMU LUT entry 133 low
    pub LUT133L: RWRegister<u32>,

    /// Graphic MMU LUT entry 133 high
    pub LUT133H: RWRegister<u32>,

    /// Graphic MMU LUT entry 134 low
    pub LUT134L: RWRegister<u32>,

    /// Graphic MMU LUT entry 134 high
    pub LUT134H: RWRegister<u32>,

    /// Graphic MMU LUT entry 135 low
    pub LUT135L: RWRegister<u32>,

    /// Graphic MMU LUT entry 135 high
    pub LUT135H: RWRegister<u32>,

    /// Graphic MMU LUT entry 136 low
    pub LUT136L: RWRegister<u32>,

    /// Graphic MMU LUT entry 136 high
    pub LUT136H: RWRegister<u32>,

    /// Graphic MMU LUT entry 137 low
    pub LUT137L: RWRegister<u32>,

    /// Graphic MMU LUT entry 137 high
    pub LUT137H: RWRegister<u32>,

    /// Graphic MMU LUT entry 138 low
    pub LUT138L: RWRegister<u32>,

    /// Graphic MMU LUT entry 138 high
    pub LUT138H: RWRegister<u32>,

    /// Graphic MMU LUT entry 139 low
    pub LUT139L: RWRegister<u32>,

    /// Graphic MMU LUT entry 139 high
    pub LUT139H: RWRegister<u32>,

    /// Graphic MMU LUT entry 140 low
    pub LUT140L: RWRegister<u32>,

    /// Graphic MMU LUT entry 140 high
    pub LUT140H: RWRegister<u32>,

    /// Graphic MMU LUT entry 141 low
    pub LUT141L: RWRegister<u32>,

    /// Graphic MMU LUT entry 141 high
    pub LUT141H: RWRegister<u32>,

    /// Graphic MMU LUT entry 142 low
    pub LUT142L: RWRegister<u32>,

    /// Graphic MMU LUT entry 142 high
    pub LUT142H: RWRegister<u32>,

    /// Graphic MMU LUT entry 143 low
    pub LUT143L: RWRegister<u32>,

    /// Graphic MMU LUT entry 143 high
    pub LUT143H: RWRegister<u32>,

    /// Graphic MMU LUT entry 144 low
    pub LUT144L: RWRegister<u32>,

    /// Graphic MMU LUT entry 144 high
    pub LUT144H: RWRegister<u32>,

    /// Graphic MMU LUT entry 145 low
    pub LUT145L: RWRegister<u32>,

    /// Graphic MMU LUT entry 145 high
    pub LUT145H: RWRegister<u32>,

    /// Graphic MMU LUT entry 146 low
    pub LUT146L: RWRegister<u32>,

    /// Graphic MMU LUT entry 146 high
    pub LUT146H: RWRegister<u32>,

    /// Graphic MMU LUT entry 147 low
    pub LUT147L: RWRegister<u32>,

    /// Graphic MMU LUT entry 147 high
    pub LUT147H: RWRegister<u32>,

    /// Graphic MMU LUT entry 148 low
    pub LUT148L: RWRegister<u32>,

    /// Graphic MMU LUT entry 148 high
    pub LUT148H: RWRegister<u32>,

    /// Graphic MMU LUT entry 149 low
    pub LUT149L: RWRegister<u32>,

    /// Graphic MMU LUT entry 149 high
    pub LUT149H: RWRegister<u32>,

    /// Graphic MMU LUT entry 150 low
    pub LUT150L: RWRegister<u32>,

    /// Graphic MMU LUT entry 150 high
    pub LUT150H: RWRegister<u32>,

    /// Graphic MMU LUT entry 151 low
    pub LUT151L: RWRegister<u32>,

    /// Graphic MMU LUT entry 151 high
    pub LUT151H: RWRegister<u32>,

    /// Graphic MMU LUT entry 152 low
    pub LUT152L: RWRegister<u32>,

    /// Graphic MMU LUT entry 152 high
    pub LUT152H: RWRegister<u32>,

    /// Graphic MMU LUT entry 153 low
    pub LUT153L: RWRegister<u32>,

    /// Graphic MMU LUT entry 153 high
    pub LUT153H: RWRegister<u32>,

    /// Graphic MMU LUT entry 154 low
    pub LUT154L: RWRegister<u32>,

    /// Graphic MMU LUT entry 154 high
    pub LUT154H: RWRegister<u32>,

    /// Graphic MMU LUT entry 155 low
    pub LUT155L: RWRegister<u32>,

    /// Graphic MMU LUT entry 155 high
    pub LUT155H: RWRegister<u32>,

    /// Graphic MMU LUT entry 156 low
    pub LUT156L: RWRegister<u32>,

    /// Graphic MMU LUT entry 156 high
    pub LUT156H: RWRegister<u32>,

    /// Graphic MMU LUT entry 157 low
    pub LUT157L: RWRegister<u32>,

    /// Graphic MMU LUT entry 157 high
    pub LUT157H: RWRegister<u32>,

    /// Graphic MMU LUT entry 158 low
    pub LUT158L: RWRegister<u32>,

    /// Graphic MMU LUT entry 158 high
    pub LUT158H: RWRegister<u32>,

    /// Graphic MMU LUT entry 159 low
    pub LUT159L: RWRegister<u32>,

    /// Graphic MMU LUT entry 159 high
    pub LUT159H: RWRegister<u32>,

    /// Graphic MMU LUT entry 160 low
    pub LUT160L: RWRegister<u32>,

    /// Graphic MMU LUT entry 160 high
    pub LUT160H: RWRegister<u32>,

    /// Graphic MMU LUT entry 161 low
    pub LUT161L: RWRegister<u32>,

    /// Graphic MMU LUT entry 161 high
    pub LUT161H: RWRegister<u32>,

    /// Graphic MMU LUT entry 162 low
    pub LUT162L: RWRegister<u32>,

    /// Graphic MMU LUT entry 162 high
    pub LUT162H: RWRegister<u32>,

    /// Graphic MMU LUT entry 163 low
    pub LUT163L: RWRegister<u32>,

    /// Graphic MMU LUT entry 163 high
    pub LUT163H: RWRegister<u32>,

    /// Graphic MMU LUT entry 164 low
    pub LUT164L: RWRegister<u32>,

    /// Graphic MMU LUT entry 164 high
    pub LUT164H: RWRegister<u32>,

    /// Graphic MMU LUT entry 165 low
    pub LUT165L: RWRegister<u32>,

    /// Graphic MMU LUT entry 165 high
    pub LUT165H: RWRegister<u32>,

    /// Graphic MMU LUT entry 166 low
    pub LUT166L: RWRegister<u32>,

    /// Graphic MMU LUT entry 166 high
    pub LUT166H: RWRegister<u32>,

    /// Graphic MMU LUT entry 167 low
    pub LUT167L: RWRegister<u32>,

    /// Graphic MMU LUT entry 167 high
    pub LUT167H: RWRegister<u32>,

    /// Graphic MMU LUT entry 168 low
    pub LUT168L: RWRegister<u32>,

    /// Graphic MMU LUT entry 168 high
    pub LUT168H: RWRegister<u32>,

    /// Graphic MMU LUT entry 169 low
    pub LUT169L: RWRegister<u32>,

    /// Graphic MMU LUT entry 169 high
    pub LUT169H: RWRegister<u32>,

    /// Graphic MMU LUT entry 170 low
    pub LUT170L: RWRegister<u32>,

    /// Graphic MMU LUT entry 170 high
    pub LUT170H: RWRegister<u32>,

    /// Graphic MMU LUT entry 171 low
    pub LUT171L: RWRegister<u32>,

    /// Graphic MMU LUT entry 171 high
    pub LUT171H: RWRegister<u32>,

    /// Graphic MMU LUT entry 172 low
    pub LUT172L: RWRegister<u32>,

    /// Graphic MMU LUT entry 172 high
    pub LUT172H: RWRegister<u32>,

    /// Graphic MMU LUT entry 173 low
    pub LUT173L: RWRegister<u32>,

    /// Graphic MMU LUT entry 173 high
    pub LUT173H: RWRegister<u32>,

    /// Graphic MMU LUT entry 174 low
    pub LUT174L: RWRegister<u32>,

    /// Graphic MMU LUT entry 174 high
    pub LUT174H: RWRegister<u32>,

    /// Graphic MMU LUT entry 175 low
    pub LUT175L: RWRegister<u32>,

    /// Graphic MMU LUT entry 175 high
    pub LUT175H: RWRegister<u32>,

    /// Graphic MMU LUT entry 176 low
    pub LUT176L: RWRegister<u32>,

    /// Graphic MMU LUT entry 176 high
    pub LUT176H: RWRegister<u32>,

    /// Graphic MMU LUT entry 177 low
    pub LUT177L: RWRegister<u32>,

    /// Graphic MMU LUT entry 177 high
    pub LUT177H: RWRegister<u32>,

    /// Graphic MMU LUT entry 178 low
    pub LUT178L: RWRegister<u32>,

    /// Graphic MMU LUT entry 178 high
    pub LUT178H: RWRegister<u32>,

    /// Graphic MMU LUT entry 179 low
    pub LUT179L: RWRegister<u32>,

    /// Graphic MMU LUT entry 179 high
    pub LUT179H: RWRegister<u32>,

    /// Graphic MMU LUT entry 180 low
    pub LUT180L: RWRegister<u32>,

    /// Graphic MMU LUT entry 180 high
    pub LUT180H: RWRegister<u32>,

    /// Graphic MMU LUT entry 181 low
    pub LUT181L: RWRegister<u32>,

    /// Graphic MMU LUT entry 181 high
    pub LUT181H: RWRegister<u32>,

    /// Graphic MMU LUT entry 182 low
    pub LUT182L: RWRegister<u32>,

    /// Graphic MMU LUT entry 182 high
    pub LUT182H: RWRegister<u32>,

    /// Graphic MMU LUT entry 183 low
    pub LUT183L: RWRegister<u32>,

    /// Graphic MMU LUT entry 183 high
    pub LUT183H: RWRegister<u32>,

    /// Graphic MMU LUT entry 184 low
    pub LUT184L: RWRegister<u32>,

    /// Graphic MMU LUT entry 184 high
    pub LUT184H: RWRegister<u32>,

    /// Graphic MMU LUT entry 185 low
    pub LUT185L: RWRegister<u32>,

    /// Graphic MMU LUT entry 185 high
    pub LUT185H: RWRegister<u32>,

    /// Graphic MMU LUT entry 186 low
    pub LUT186L: RWRegister<u32>,

    /// Graphic MMU LUT entry 186 high
    pub LUT186H: RWRegister<u32>,

    /// Graphic MMU LUT entry 187 low
    pub LUT187L: RWRegister<u32>,

    /// Graphic MMU LUT entry 187 high
    pub LUT187H: RWRegister<u32>,

    /// Graphic MMU LUT entry 188 low
    pub LUT188L: RWRegister<u32>,

    /// Graphic MMU LUT entry 188 high
    pub LUT188H: RWRegister<u32>,

    /// Graphic MMU LUT entry 189 low
    pub LUT189L: RWRegister<u32>,

    /// Graphic MMU LUT entry 189 high
    pub LUT189H: RWRegister<u32>,

    /// Graphic MMU LUT entry 190 low
    pub LUT190L: RWRegister<u32>,

    /// Graphic MMU LUT entry 190 high
    pub LUT190H: RWRegister<u32>,

    /// Graphic MMU LUT entry 191 low
    pub LUT191L: RWRegister<u32>,

    /// Graphic MMU LUT entry 191 high
    pub LUT191H: RWRegister<u32>,

    /// Graphic MMU LUT entry 192 low
    pub LUT192L: RWRegister<u32>,

    /// Graphic MMU LUT entry 192 high
    pub LUT192H: RWRegister<u32>,

    /// Graphic MMU LUT entry 193 low
    pub LUT193L: RWRegister<u32>,

    /// Graphic MMU LUT entry 193 high
    pub LUT193H: RWRegister<u32>,

    /// Graphic MMU LUT entry 194 low
    pub LUT194L: RWRegister<u32>,

    /// Graphic MMU LUT entry 194 high
    pub LUT194H: RWRegister<u32>,

    /// Graphic MMU LUT entry 195 low
    pub LUT195L: RWRegister<u32>,

    /// Graphic MMU LUT entry 195 high
    pub LUT195H: RWRegister<u32>,

    /// Graphic MMU LUT entry 196 low
    pub LUT196L: RWRegister<u32>,

    /// Graphic MMU LUT entry 196 high
    pub LUT196H: RWRegister<u32>,

    /// Graphic MMU LUT entry 197 low
    pub LUT197L: RWRegister<u32>,

    /// Graphic MMU LUT entry 197 high
    pub LUT197H: RWRegister<u32>,

    /// Graphic MMU LUT entry 198 low
    pub LUT198L: RWRegister<u32>,

    /// Graphic MMU LUT entry 198 high
    pub LUT198H: RWRegister<u32>,

    /// Graphic MMU LUT entry 199 low
    pub LUT199L: RWRegister<u32>,

    /// Graphic MMU LUT entry 199 high
    pub LUT199H: RWRegister<u32>,

    /// Graphic MMU LUT entry 200 low
    pub LUT200L: RWRegister<u32>,

    /// Graphic MMU LUT entry 200 high
    pub LUT200H: RWRegister<u32>,

    /// Graphic MMU LUT entry 201 low
    pub LUT201L: RWRegister<u32>,

    /// Graphic MMU LUT entry 201 high
    pub LUT201H: RWRegister<u32>,

    /// Graphic MMU LUT entry 202 low
    pub LUT202L: RWRegister<u32>,

    /// Graphic MMU LUT entry 202 high
    pub LUT202H: RWRegister<u32>,

    /// Graphic MMU LUT entry 203 low
    pub LUT203L: RWRegister<u32>,

    /// Graphic MMU LUT entry 203 high
    pub LUT203H: RWRegister<u32>,

    /// Graphic MMU LUT entry 204 low
    pub LUT204L: RWRegister<u32>,

    /// Graphic MMU LUT entry 204 high
    pub LUT204H: RWRegister<u32>,

    /// Graphic MMU LUT entry 205 low
    pub LUT205L: RWRegister<u32>,

    /// Graphic MMU LUT entry 205 high
    pub LUT205H: RWRegister<u32>,

    /// Graphic MMU LUT entry 206 low
    pub LUT206L: RWRegister<u32>,

    /// Graphic MMU LUT entry 206 high
    pub LUT206H: RWRegister<u32>,

    /// Graphic MMU LUT entry 207 low
    pub LUT207L: RWRegister<u32>,

    /// Graphic MMU LUT entry 207 high
    pub LUT207H: RWRegister<u32>,

    /// Graphic MMU LUT entry 208 low
    pub LUT208L: RWRegister<u32>,

    /// Graphic MMU LUT entry 208 high
    pub LUT208H: RWRegister<u32>,

    /// Graphic MMU LUT entry 209 low
    pub LUT209L: RWRegister<u32>,

    /// Graphic MMU LUT entry 209 high
    pub LUT209H: RWRegister<u32>,

    /// Graphic MMU LUT entry 210 low
    pub LUT210L: RWRegister<u32>,

    /// Graphic MMU LUT entry 210 high
    pub LUT210H: RWRegister<u32>,

    /// Graphic MMU LUT entry 211 low
    pub LUT211L: RWRegister<u32>,

    /// Graphic MMU LUT entry 211 high
    pub LUT211H: RWRegister<u32>,

    /// Graphic MMU LUT entry 212 low
    pub LUT212L: RWRegister<u32>,

    /// Graphic MMU LUT entry 212 high
    pub LUT212H: RWRegister<u32>,

    /// Graphic MMU LUT entry 213 low
    pub LUT213L: RWRegister<u32>,

    /// Graphic MMU LUT entry 213 high
    pub LUT213H: RWRegister<u32>,

    /// Graphic MMU LUT entry 214 low
    pub LUT214L: RWRegister<u32>,

    /// Graphic MMU LUT entry 214 high
    pub LUT214H: RWRegister<u32>,

    /// Graphic MMU LUT entry 215 low
    pub LUT215L: RWRegister<u32>,

    /// Graphic MMU LUT entry 215 high
    pub LUT215H: RWRegister<u32>,

    /// Graphic MMU LUT entry 216 low
    pub LUT216L: RWRegister<u32>,

    /// Graphic MMU LUT entry 216 high
    pub LUT216H: RWRegister<u32>,

    /// Graphic MMU LUT entry 217 low
    pub LUT217L: RWRegister<u32>,

    /// Graphic MMU LUT entry 217 high
    pub LUT217H: RWRegister<u32>,

    /// Graphic MMU LUT entry 218 low
    pub LUT218L: RWRegister<u32>,

    /// Graphic MMU LUT entry 218 high
    pub LUT218H: RWRegister<u32>,

    /// Graphic MMU LUT entry 219 low
    pub LUT219L: RWRegister<u32>,

    /// Graphic MMU LUT entry 219 high
    pub LUT219H: RWRegister<u32>,

    /// Graphic MMU LUT entry 220 low
    pub LUT220L: RWRegister<u32>,

    /// Graphic MMU LUT entry 220 high
    pub LUT220H: RWRegister<u32>,

    /// Graphic MMU LUT entry 221 low
    pub LUT221L: RWRegister<u32>,

    /// Graphic MMU LUT entry 221 high
    pub LUT221H: RWRegister<u32>,

    /// Graphic MMU LUT entry 222 low
    pub LUT222L: RWRegister<u32>,

    /// Graphic MMU LUT entry 222 high
    pub LUT222H: RWRegister<u32>,

    /// Graphic MMU LUT entry 223 low
    pub LUT223L: RWRegister<u32>,

    /// Graphic MMU LUT entry 223 high
    pub LUT223H: RWRegister<u32>,

    /// Graphic MMU LUT entry 224 low
    pub LUT224L: RWRegister<u32>,

    /// Graphic MMU LUT entry 224 high
    pub LUT224H: RWRegister<u32>,

    /// Graphic MMU LUT entry 225 low
    pub LUT225L: RWRegister<u32>,

    /// Graphic MMU LUT entry 225 high
    pub LUT225H: RWRegister<u32>,

    /// Graphic MMU LUT entry 226 low
    pub LUT226L: RWRegister<u32>,

    /// Graphic MMU LUT entry 226 high
    pub LUT226H: RWRegister<u32>,

    /// Graphic MMU LUT entry 227 low
    pub LUT227L: RWRegister<u32>,

    /// Graphic MMU LUT entry 227 high
    pub LUT227H: RWRegister<u32>,

    /// Graphic MMU LUT entry 228 low
    pub LUT228L: RWRegister<u32>,

    /// Graphic MMU LUT entry 228 high
    pub LUT228H: RWRegister<u32>,

    /// Graphic MMU LUT entry 229 low
    pub LUT229L: RWRegister<u32>,

    /// Graphic MMU LUT entry 229 high
    pub LUT229H: RWRegister<u32>,

    /// Graphic MMU LUT entry 230 low
    pub LUT230L: RWRegister<u32>,

    /// Graphic MMU LUT entry 230 high
    pub LUT230H: RWRegister<u32>,

    /// Graphic MMU LUT entry 231 low
    pub LUT231L: RWRegister<u32>,

    /// Graphic MMU LUT entry 231 high
    pub LUT231H: RWRegister<u32>,

    /// Graphic MMU LUT entry 232 low
    pub LUT232L: RWRegister<u32>,

    /// Graphic MMU LUT entry 232 high
    pub LUT232H: RWRegister<u32>,

    /// Graphic MMU LUT entry 233 low
    pub LUT233L: RWRegister<u32>,

    /// Graphic MMU LUT entry 233 high
    pub LUT233H: RWRegister<u32>,

    /// Graphic MMU LUT entry 234 low
    pub LUT234L: RWRegister<u32>,

    /// Graphic MMU LUT entry 234 high
    pub LUT234H: RWRegister<u32>,

    /// Graphic MMU LUT entry 235 low
    pub LUT235L: RWRegister<u32>,

    /// Graphic MMU LUT entry 235 high
    pub LUT235H: RWRegister<u32>,

    /// Graphic MMU LUT entry 236 low
    pub LUT236L: RWRegister<u32>,

    /// Graphic MMU LUT entry 236 high
    pub LUT236H: RWRegister<u32>,

    /// Graphic MMU LUT entry 237 low
    pub LUT237L: RWRegister<u32>,

    /// Graphic MMU LUT entry 237 high
    pub LUT237H: RWRegister<u32>,

    /// Graphic MMU LUT entry 238 low
    pub LUT238L: RWRegister<u32>,

    /// Graphic MMU LUT entry 238 high
    pub LUT238H: RWRegister<u32>,

    /// Graphic MMU LUT entry 239 low
    pub LUT239L: RWRegister<u32>,

    /// Graphic MMU LUT entry 239 high
    pub LUT239H: RWRegister<u32>,

    /// Graphic MMU LUT entry 240 low
    pub LUT240L: RWRegister<u32>,

    /// Graphic MMU LUT entry 240 high
    pub LUT240H: RWRegister<u32>,

    /// Graphic MMU LUT entry 241 low
    pub LUT241L: RWRegister<u32>,

    /// Graphic MMU LUT entry 241 high
    pub LUT241H: RWRegister<u32>,

    /// Graphic MMU LUT entry 242 low
    pub LUT242L: RWRegister<u32>,

    /// Graphic MMU LUT entry 242 high
    pub LUT242H: RWRegister<u32>,

    /// Graphic MMU LUT entry 243 low
    pub LUT243L: RWRegister<u32>,

    /// Graphic MMU LUT entry 243 high
    pub LUT243H: RWRegister<u32>,

    /// Graphic MMU LUT entry 244 low
    pub LUT244L: RWRegister<u32>,

    /// Graphic MMU LUT entry 244 high
    pub LUT244H: RWRegister<u32>,

    /// Graphic MMU LUT entry 245 low
    pub LUT245L: RWRegister<u32>,

    /// Graphic MMU LUT entry 245 high
    pub LUT245H: RWRegister<u32>,

    /// Graphic MMU LUT entry 246 low
    pub LUT246L: RWRegister<u32>,

    /// Graphic MMU LUT entry 246 high
    pub LUT246H: RWRegister<u32>,

    /// Graphic MMU LUT entry 247 low
    pub LUT247L: RWRegister<u32>,

    /// Graphic MMU LUT entry 247 high
    pub LUT247H: RWRegister<u32>,

    /// Graphic MMU LUT entry 248 low
    pub LUT248L: RWRegister<u32>,

    /// Graphic MMU LUT entry 248 high
    pub LUT248H: RWRegister<u32>,

    /// Graphic MMU LUT entry 249 low
    pub LUT249L: RWRegister<u32>,

    /// Graphic MMU LUT entry 249 high
    pub LUT249H: RWRegister<u32>,

    /// Graphic MMU LUT entry 250 low
    pub LUT250L: RWRegister<u32>,

    /// Graphic MMU LUT entry 250 high
    pub LUT250H: RWRegister<u32>,

    /// Graphic MMU LUT entry 251 low
    pub LUT251L: RWRegister<u32>,

    /// Graphic MMU LUT entry 251 high
    pub LUT251H: RWRegister<u32>,

    /// Graphic MMU LUT entry 252 low
    pub LUT252L: RWRegister<u32>,

    /// Graphic MMU LUT entry 252 high
    pub LUT252H: RWRegister<u32>,

    /// Graphic MMU LUT entry 253 low
    pub LUT253L: RWRegister<u32>,

    /// Graphic MMU LUT entry 253 high
    pub LUT253H: RWRegister<u32>,

    /// Graphic MMU LUT entry 254 low
    pub LUT254L: RWRegister<u32>,

    /// Graphic MMU LUT entry 254 high
    pub LUT254H: RWRegister<u32>,

    /// Graphic MMU LUT entry 255 low
    pub LUT255L: RWRegister<u32>,

    /// Graphic MMU LUT entry 255 high
    pub LUT255H: RWRegister<u32>,

    /// Graphic MMU LUT entry 256 low
    pub LUT256L: RWRegister<u32>,

    /// Graphic MMU LUT entry 256 high
    pub LUT256H: RWRegister<u32>,

    /// Graphic MMU LUT entry 257 low
    pub LUT257L: RWRegister<u32>,

    /// Graphic MMU LUT entry 257 high
    pub LUT257H: RWRegister<u32>,

    /// Graphic MMU LUT entry 258 low
    pub LUT258L: RWRegister<u32>,

    /// Graphic MMU LUT entry 258 high
    pub LUT258H: RWRegister<u32>,

    /// Graphic MMU LUT entry 259 low
    pub LUT259L: RWRegister<u32>,

    /// Graphic MMU LUT entry 259 high
    pub LUT259H: RWRegister<u32>,

    /// Graphic MMU LUT entry 260 low
    pub LUT260L: RWRegister<u32>,

    /// Graphic MMU LUT entry 260 high
    pub LUT260H: RWRegister<u32>,

    /// Graphic MMU LUT entry 261 low
    pub LUT261L: RWRegister<u32>,

    /// Graphic MMU LUT entry 261 high
    pub LUT261H: RWRegister<u32>,

    /// Graphic MMU LUT entry 262 low
    pub LUT262L: RWRegister<u32>,

    /// Graphic MMU LUT entry 262 high
    pub LUT262H: RWRegister<u32>,

    /// Graphic MMU LUT entry 263 low
    pub LUT263L: RWRegister<u32>,

    /// Graphic MMU LUT entry 263 high
    pub LUT263H: RWRegister<u32>,

    /// Graphic MMU LUT entry 264 low
    pub LUT264L: RWRegister<u32>,

    /// Graphic MMU LUT entry 264 high
    pub LUT264H: RWRegister<u32>,

    /// Graphic MMU LUT entry 265 low
    pub LUT265L: RWRegister<u32>,

    /// Graphic MMU LUT entry 265 high
    pub LUT265H: RWRegister<u32>,

    /// Graphic MMU LUT entry 266 low
    pub LUT266L: RWRegister<u32>,

    /// Graphic MMU LUT entry 266 high
    pub LUT266H: RWRegister<u32>,

    /// Graphic MMU LUT entry 267 low
    pub LUT267L: RWRegister<u32>,

    /// Graphic MMU LUT entry 267 high
    pub LUT267H: RWRegister<u32>,

    /// Graphic MMU LUT entry 268 low
    pub LUT268L: RWRegister<u32>,

    /// Graphic MMU LUT entry 268 high
    pub LUT268H: RWRegister<u32>,

    /// Graphic MMU LUT entry 269 low
    pub LUT269L: RWRegister<u32>,

    /// Graphic MMU LUT entry 269 high
    pub LUT269H: RWRegister<u32>,

    /// Graphic MMU LUT entry 270 low
    pub LUT270L: RWRegister<u32>,

    /// Graphic MMU LUT entry 270 high
    pub LUT270H: RWRegister<u32>,

    /// Graphic MMU LUT entry 271 low
    pub LUT271L: RWRegister<u32>,

    /// Graphic MMU LUT entry 271 high
    pub LUT271H: RWRegister<u32>,

    /// Graphic MMU LUT entry 272 low
    pub LUT272L: RWRegister<u32>,

    /// Graphic MMU LUT entry 272 high
    pub LUT272H: RWRegister<u32>,

    /// Graphic MMU LUT entry 273 low
    pub LUT273L: RWRegister<u32>,

    /// Graphic MMU LUT entry 273 high
    pub LUT273H: RWRegister<u32>,

    /// Graphic MMU LUT entry 274 low
    pub LUT274L: RWRegister<u32>,

    /// Graphic MMU LUT entry 274 high
    pub LUT274H: RWRegister<u32>,

    /// Graphic MMU LUT entry 275 low
    pub LUT275L: RWRegister<u32>,

    /// Graphic MMU LUT entry 275 high
    pub LUT275H: RWRegister<u32>,

    /// Graphic MMU LUT entry 276 low
    pub LUT276L: RWRegister<u32>,

    /// Graphic MMU LUT entry 276 high
    pub LUT276H: RWRegister<u32>,

    /// Graphic MMU LUT entry 277 low
    pub LUT277L: RWRegister<u32>,

    /// Graphic MMU LUT entry 277 high
    pub LUT277H: RWRegister<u32>,

    /// Graphic MMU LUT entry 278 low
    pub LUT278L: RWRegister<u32>,

    /// Graphic MMU LUT entry 278 high
    pub LUT278H: RWRegister<u32>,

    /// Graphic MMU LUT entry 279 low
    pub LUT279L: RWRegister<u32>,

    /// Graphic MMU LUT entry 279 high
    pub LUT279H: RWRegister<u32>,

    /// Graphic MMU LUT entry 280 low
    pub LUT280L: RWRegister<u32>,

    /// Graphic MMU LUT entry 280 high
    pub LUT280H: RWRegister<u32>,

    /// Graphic MMU LUT entry 281 low
    pub LUT281L: RWRegister<u32>,

    /// Graphic MMU LUT entry 281 high
    pub LUT281H: RWRegister<u32>,

    /// Graphic MMU LUT entry 282 low
    pub LUT282L: RWRegister<u32>,

    /// Graphic MMU LUT entry 282 high
    pub LUT282H: RWRegister<u32>,

    /// Graphic MMU LUT entry 283 low
    pub LUT283L: RWRegister<u32>,

    /// Graphic MMU LUT entry 283 high
    pub LUT283H: RWRegister<u32>,

    /// Graphic MMU LUT entry 284 low
    pub LUT284L: RWRegister<u32>,

    /// Graphic MMU LUT entry 284 high
    pub LUT284H: RWRegister<u32>,

    /// Graphic MMU LUT entry 285 low
    pub LUT285L: RWRegister<u32>,

    /// Graphic MMU LUT entry 285 high
    pub LUT285H: RWRegister<u32>,

    /// Graphic MMU LUT entry 286 low
    pub LUT286L: RWRegister<u32>,

    /// Graphic MMU LUT entry 286 high
    pub LUT286H: RWRegister<u32>,

    /// Graphic MMU LUT entry 287 low
    pub LUT287L: RWRegister<u32>,

    /// Graphic MMU LUT entry 287 high
    pub LUT287H: RWRegister<u32>,

    /// Graphic MMU LUT entry 288 low
    pub LUT288L: RWRegister<u32>,

    /// Graphic MMU LUT entry 288 high
    pub LUT288H: RWRegister<u32>,

    /// Graphic MMU LUT entry 289 low
    pub LUT289L: RWRegister<u32>,

    /// Graphic MMU LUT entry 289 high
    pub LUT289H: RWRegister<u32>,

    /// Graphic MMU LUT entry 290 low
    pub LUT290L: RWRegister<u32>,

    /// Graphic MMU LUT entry 290 high
    pub LUT290H: RWRegister<u32>,

    /// Graphic MMU LUT entry 291 low
    pub LUT291L: RWRegister<u32>,

    /// Graphic MMU LUT entry 291 high
    pub LUT291H: RWRegister<u32>,

    /// Graphic MMU LUT entry 292 low
    pub LUT292L: RWRegister<u32>,

    /// Graphic MMU LUT entry 292 high
    pub LUT292H: RWRegister<u32>,

    /// Graphic MMU LUT entry 293 low
    pub LUT293L: RWRegister<u32>,

    /// Graphic MMU LUT entry 293 high
    pub LUT293H: RWRegister<u32>,

    /// Graphic MMU LUT entry 294 low
    pub LUT294L: RWRegister<u32>,

    /// Graphic MMU LUT entry 294 high
    pub LUT294H: RWRegister<u32>,

    /// Graphic MMU LUT entry 295 low
    pub LUT295L: RWRegister<u32>,

    /// Graphic MMU LUT entry 295 high
    pub LUT295H: RWRegister<u32>,

    /// Graphic MMU LUT entry 296 low
    pub LUT296L: RWRegister<u32>,

    /// Graphic MMU LUT entry 296 high
    pub LUT296H: RWRegister<u32>,

    /// Graphic MMU LUT entry 297 low
    pub LUT297L: RWRegister<u32>,

    /// Graphic MMU LUT entry 297 high
    pub LUT297H: RWRegister<u32>,

    /// Graphic MMU LUT entry 298 low
    pub LUT298L: RWRegister<u32>,

    /// Graphic MMU LUT entry 298 high
    pub LUT298H: RWRegister<u32>,

    /// Graphic MMU LUT entry 299 low
    pub LUT299L: RWRegister<u32>,

    /// Graphic MMU LUT entry 299 high
    pub LUT299H: RWRegister<u32>,

    /// Graphic MMU LUT entry 300 low
    pub LUT300L: RWRegister<u32>,

    /// Graphic MMU LUT entry 300 high
    pub LUT300H: RWRegister<u32>,

    /// Graphic MMU LUT entry 301 low
    pub LUT301L: RWRegister<u32>,

    /// Graphic MMU LUT entry 301 high
    pub LUT301H: RWRegister<u32>,

    /// Graphic MMU LUT entry 302 low
    pub LUT302L: RWRegister<u32>,

    /// Graphic MMU LUT entry 302 high
    pub LUT302H: RWRegister<u32>,

    /// Graphic MMU LUT entry 303 low
    pub LUT303L: RWRegister<u32>,

    /// Graphic MMU LUT entry 303 high
    pub LUT303H: RWRegister<u32>,

    /// Graphic MMU LUT entry 304 low
    pub LUT304L: RWRegister<u32>,

    /// Graphic MMU LUT entry 304 high
    pub LUT304H: RWRegister<u32>,

    /// Graphic MMU LUT entry 305 low
    pub LUT305L: RWRegister<u32>,

    /// Graphic MMU LUT entry 305 high
    pub LUT305H: RWRegister<u32>,

    /// Graphic MMU LUT entry 306 low
    pub LUT306L: RWRegister<u32>,

    /// Graphic MMU LUT entry 306 high
    pub LUT306H: RWRegister<u32>,

    /// Graphic MMU LUT entry 307 low
    pub LUT307L: RWRegister<u32>,

    /// Graphic MMU LUT entry 307 high
    pub LUT307H: RWRegister<u32>,

    /// Graphic MMU LUT entry 308 low
    pub LUT308L: RWRegister<u32>,

    /// Graphic MMU LUT entry 308 high
    pub LUT308H: RWRegister<u32>,

    /// Graphic MMU LUT entry 309 low
    pub LUT309L: RWRegister<u32>,

    /// Graphic MMU LUT entry 309 high
    pub LUT309H: RWRegister<u32>,

    /// Graphic MMU LUT entry 310 low
    pub LUT310L: RWRegister<u32>,

    /// Graphic MMU LUT entry 310 high
    pub LUT310H: RWRegister<u32>,

    /// Graphic MMU LUT entry 311 low
    pub LUT311L: RWRegister<u32>,

    /// Graphic MMU LUT entry 311 high
    pub LUT311H: RWRegister<u32>,

    /// Graphic MMU LUT entry 312 low
    pub LUT312L: RWRegister<u32>,

    /// Graphic MMU LUT entry 312 high
    pub LUT312H: RWRegister<u32>,

    /// Graphic MMU LUT entry 313 low
    pub LUT313L: RWRegister<u32>,

    /// Graphic MMU LUT entry 313 high
    pub LUT313H: RWRegister<u32>,

    /// Graphic MMU LUT entry 314 low
    pub LUT314L: RWRegister<u32>,

    /// Graphic MMU LUT entry 314 high
    pub LUT314H: RWRegister<u32>,

    /// Graphic MMU LUT entry 315 low
    pub LUT315L: RWRegister<u32>,

    /// Graphic MMU LUT entry 315 high
    pub LUT315H: RWRegister<u32>,

    /// Graphic MMU LUT entry 316 low
    pub LUT316L: RWRegister<u32>,

    /// Graphic MMU LUT entry 316 high
    pub LUT316H: RWRegister<u32>,

    /// Graphic MMU LUT entry 317 low
    pub LUT317L: RWRegister<u32>,

    /// Graphic MMU LUT entry 317 high
    pub LUT317H: RWRegister<u32>,

    /// Graphic MMU LUT entry 318 low
    pub LUT318L: RWRegister<u32>,

    /// Graphic MMU LUT entry 318 high
    pub LUT318H: RWRegister<u32>,

    /// Graphic MMU LUT entry 319 low
    pub LUT319L: RWRegister<u32>,

    /// Graphic MMU LUT entry 319 high
    pub LUT319H: RWRegister<u32>,

    /// Graphic MMU LUT entry 320 low
    pub LUT320L: RWRegister<u32>,

    /// Graphic MMU LUT entry 320 high
    pub LUT320H: RWRegister<u32>,

    /// Graphic MMU LUT entry 321 low
    pub LUT321L: RWRegister<u32>,

    /// Graphic MMU LUT entry 321 high
    pub LUT321H: RWRegister<u32>,

    /// Graphic MMU LUT entry 322 low
    pub LUT322L: RWRegister<u32>,

    /// Graphic MMU LUT entry 322 high
    pub LUT322H: RWRegister<u32>,

    /// Graphic MMU LUT entry 323 low
    pub LUT323L: RWRegister<u32>,

    /// Graphic MMU LUT entry 323 high
    pub LUT323H: RWRegister<u32>,

    /// Graphic MMU LUT entry 324 low
    pub LUT324L: RWRegister<u32>,

    /// Graphic MMU LUT entry 324 high
    pub LUT324H: RWRegister<u32>,

    /// Graphic MMU LUT entry 325 low
    pub LUT325L: RWRegister<u32>,

    /// Graphic MMU LUT entry 325 high
    pub LUT325H: RWRegister<u32>,

    /// Graphic MMU LUT entry 326 low
    pub LUT326L: RWRegister<u32>,

    /// Graphic MMU LUT entry 326 high
    pub LUT326H: RWRegister<u32>,

    /// Graphic MMU LUT entry 327 low
    pub LUT327L: RWRegister<u32>,

    /// Graphic MMU LUT entry 327 high
    pub LUT327H: RWRegister<u32>,

    /// Graphic MMU LUT entry 328 low
    pub LUT328L: RWRegister<u32>,

    /// Graphic MMU LUT entry 328 high
    pub LUT328H: RWRegister<u32>,

    /// Graphic MMU LUT entry 329 low
    pub LUT329L: RWRegister<u32>,

    /// Graphic MMU LUT entry 329 high
    pub LUT329H: RWRegister<u32>,

    /// Graphic MMU LUT entry 330 low
    pub LUT330L: RWRegister<u32>,

    /// Graphic MMU LUT entry 330 high
    pub LUT330H: RWRegister<u32>,

    /// Graphic MMU LUT entry 331 low
    pub LUT331L: RWRegister<u32>,

    /// Graphic MMU LUT entry 331 high
    pub LUT331H: RWRegister<u32>,

    /// Graphic MMU LUT entry 332 low
    pub LUT332L: RWRegister<u32>,

    /// Graphic MMU LUT entry 332 high
    pub LUT332H: RWRegister<u32>,

    /// Graphic MMU LUT entry 333 low
    pub LUT333L: RWRegister<u32>,

    /// Graphic MMU LUT entry 333 high
    pub LUT333H: RWRegister<u32>,

    /// Graphic MMU LUT entry 334 low
    pub LUT334L: RWRegister<u32>,

    /// Graphic MMU LUT entry 334 high
    pub LUT334H: RWRegister<u32>,

    /// Graphic MMU LUT entry 335 low
    pub LUT335L: RWRegister<u32>,

    /// Graphic MMU LUT entry 335 high
    pub LUT335H: RWRegister<u32>,

    /// Graphic MMU LUT entry 336 low
    pub LUT336L: RWRegister<u32>,

    /// Graphic MMU LUT entry 336 high
    pub LUT336H: RWRegister<u32>,

    /// Graphic MMU LUT entry 337 low
    pub LUT337L: RWRegister<u32>,

    /// Graphic MMU LUT entry 337 high
    pub LUT337H: RWRegister<u32>,

    /// Graphic MMU LUT entry 338 low
    pub LUT338L: RWRegister<u32>,

    /// Graphic MMU LUT entry 338 high
    pub LUT338H: RWRegister<u32>,

    /// Graphic MMU LUT entry 339 low
    pub LUT339L: RWRegister<u32>,

    /// Graphic MMU LUT entry 339 high
    pub LUT339H: RWRegister<u32>,

    /// Graphic MMU LUT entry 340 low
    pub LUT340L: RWRegister<u32>,

    /// Graphic MMU LUT entry 340 high
    pub LUT340H: RWRegister<u32>,

    /// Graphic MMU LUT entry 341 low
    pub LUT341L: RWRegister<u32>,

    /// Graphic MMU LUT entry 341 high
    pub LUT341H: RWRegister<u32>,

    /// Graphic MMU LUT entry 342 low
    pub LUT342L: RWRegister<u32>,

    /// Graphic MMU LUT entry 342 high
    pub LUT342H: RWRegister<u32>,

    /// Graphic MMU LUT entry 343 low
    pub LUT343L: RWRegister<u32>,

    /// Graphic MMU LUT entry 343 high
    pub LUT343H: RWRegister<u32>,

    /// Graphic MMU LUT entry 344 low
    pub LUT344L: RWRegister<u32>,

    /// Graphic MMU LUT entry 344 high
    pub LUT344H: RWRegister<u32>,

    /// Graphic MMU LUT entry 345 low
    pub LUT345L: RWRegister<u32>,

    /// Graphic MMU LUT entry 345 high
    pub LUT345H: RWRegister<u32>,

    /// Graphic MMU LUT entry 346 low
    pub LUT346L: RWRegister<u32>,

    /// Graphic MMU LUT entry 346 high
    pub LUT346H: RWRegister<u32>,

    /// Graphic MMU LUT entry 347 low
    pub LUT347L: RWRegister<u32>,

    /// Graphic MMU LUT entry 347 high
    pub LUT347H: RWRegister<u32>,

    /// Graphic MMU LUT entry 348 low
    pub LUT348L: RWRegister<u32>,

    /// Graphic MMU LUT entry 348 high
    pub LUT348H: RWRegister<u32>,

    /// Graphic MMU LUT entry 349 low
    pub LUT349L: RWRegister<u32>,

    /// Graphic MMU LUT entry 349 high
    pub LUT349H: RWRegister<u32>,

    /// Graphic MMU LUT entry 350 low
    pub LUT350L: RWRegister<u32>,

    /// Graphic MMU LUT entry 350 high
    pub LUT350H: RWRegister<u32>,

    /// Graphic MMU LUT entry 351 low
    pub LUT351L: RWRegister<u32>,

    /// Graphic MMU LUT entry 351 high
    pub LUT351H: RWRegister<u32>,

    /// Graphic MMU LUT entry 352 low
    pub LUT352L: RWRegister<u32>,

    /// Graphic MMU LUT entry 352 high
    pub LUT352H: RWRegister<u32>,

    /// Graphic MMU LUT entry 353 low
    pub LUT353L: RWRegister<u32>,

    /// Graphic MMU LUT entry 353 high
    pub LUT353H: RWRegister<u32>,

    /// Graphic MMU LUT entry 354 low
    pub LUT354L: RWRegister<u32>,

    /// Graphic MMU LUT entry 354 high
    pub LUT354H: RWRegister<u32>,

    /// Graphic MMU LUT entry 355 low
    pub LUT355L: RWRegister<u32>,

    /// Graphic MMU LUT entry 355 high
    pub LUT355H: RWRegister<u32>,

    /// Graphic MMU LUT entry 356 low
    pub LUT356L: RWRegister<u32>,

    /// Graphic MMU LUT entry 356 high
    pub LUT356H: RWRegister<u32>,

    /// Graphic MMU LUT entry 357 low
    pub LUT357L: RWRegister<u32>,

    /// Graphic MMU LUT entry 357 high
    pub LUT357H: RWRegister<u32>,

    /// Graphic MMU LUT entry 358 low
    pub LUT358L: RWRegister<u32>,

    /// Graphic MMU LUT entry 358 high
    pub LUT358H: RWRegister<u32>,

    /// Graphic MMU LUT entry 359 low
    pub LUT359L: RWRegister<u32>,

    /// Graphic MMU LUT entry 359 high
    pub LUT359H: RWRegister<u32>,

    /// Graphic MMU LUT entry 360 low
    pub LUT360L: RWRegister<u32>,

    /// Graphic MMU LUT entry 360 high
    pub LUT360H: RWRegister<u32>,

    /// Graphic MMU LUT entry 361 low
    pub LUT361L: RWRegister<u32>,

    /// Graphic MMU LUT entry 361 high
    pub LUT361H: RWRegister<u32>,

    /// Graphic MMU LUT entry 362 low
    pub LUT362L: RWRegister<u32>,

    /// Graphic MMU LUT entry 362 high
    pub LUT362H: RWRegister<u32>,

    /// Graphic MMU LUT entry 363 low
    pub LUT363L: RWRegister<u32>,

    /// Graphic MMU LUT entry 363 high
    pub LUT363H: RWRegister<u32>,

    /// Graphic MMU LUT entry 364 low
    pub LUT364L: RWRegister<u32>,

    /// Graphic MMU LUT entry 364 high
    pub LUT364H: RWRegister<u32>,

    /// Graphic MMU LUT entry 365 low
    pub LUT365L: RWRegister<u32>,

    /// Graphic MMU LUT entry 365 high
    pub LUT365H: RWRegister<u32>,

    /// Graphic MMU LUT entry 366 low
    pub LUT366L: RWRegister<u32>,

    /// Graphic MMU LUT entry 366 high
    pub LUT366H: RWRegister<u32>,

    /// Graphic MMU LUT entry 367 low
    pub LUT367L: RWRegister<u32>,

    /// Graphic MMU LUT entry 367 high
    pub LUT367H: RWRegister<u32>,

    /// Graphic MMU LUT entry 368 low
    pub LUT368L: RWRegister<u32>,

    /// Graphic MMU LUT entry 368 high
    pub LUT368H: RWRegister<u32>,

    /// Graphic MMU LUT entry 369 low
    pub LUT369L: RWRegister<u32>,

    /// Graphic MMU LUT entry 369 high
    pub LUT369H: RWRegister<u32>,

    /// Graphic MMU LUT entry 370 low
    pub LUT370L: RWRegister<u32>,

    /// Graphic MMU LUT entry 370 high
    pub LUT370H: RWRegister<u32>,

    /// Graphic MMU LUT entry 371 low
    pub LUT371L: RWRegister<u32>,

    /// Graphic MMU LUT entry 371 high
    pub LUT371H: RWRegister<u32>,

    /// Graphic MMU LUT entry 372 low
    pub LUT372L: RWRegister<u32>,

    /// Graphic MMU LUT entry 372 high
    pub LUT372H: RWRegister<u32>,

    /// Graphic MMU LUT entry 373 low
    pub LUT373L: RWRegister<u32>,

    /// Graphic MMU LUT entry 373 high
    pub LUT373H: RWRegister<u32>,

    /// Graphic MMU LUT entry 374 low
    pub LUT374L: RWRegister<u32>,

    /// Graphic MMU LUT entry 374 high
    pub LUT374H: RWRegister<u32>,

    /// Graphic MMU LUT entry 375 low
    pub LUT375L: RWRegister<u32>,

    /// Graphic MMU LUT entry 375 high
    pub LUT375H: RWRegister<u32>,

    /// Graphic MMU LUT entry 376 low
    pub LUT376L: RWRegister<u32>,

    /// Graphic MMU LUT entry 376 high
    pub LUT376H: RWRegister<u32>,

    /// Graphic MMU LUT entry 377 low
    pub LUT377L: RWRegister<u32>,

    /// Graphic MMU LUT entry 377 high
    pub LUT377H: RWRegister<u32>,

    /// Graphic MMU LUT entry 378 low
    pub LUT378L: RWRegister<u32>,

    /// Graphic MMU LUT entry 378 high
    pub LUT378H: RWRegister<u32>,

    /// Graphic MMU LUT entry 379 low
    pub LUT379L: RWRegister<u32>,

    /// Graphic MMU LUT entry 379 high
    pub LUT379H: RWRegister<u32>,

    /// Graphic MMU LUT entry 380 low
    pub LUT380L: RWRegister<u32>,

    /// Graphic MMU LUT entry 380 high
    pub LUT380H: RWRegister<u32>,

    /// Graphic MMU LUT entry 381 low
    pub LUT381L: RWRegister<u32>,

    /// Graphic MMU LUT entry 381 high
    pub LUT381H: RWRegister<u32>,

    /// Graphic MMU LUT entry 382 low
    pub LUT382L: RWRegister<u32>,

    /// Graphic MMU LUT entry 382 high
    pub LUT382H: RWRegister<u32>,

    /// Graphic MMU LUT entry 383 low
    pub LUT383L: RWRegister<u32>,

    /// Graphic MMU LUT entry 383 high
    pub LUT383H: RWRegister<u32>,

    /// Graphic MMU LUT entry 384 low
    pub LUT384L: RWRegister<u32>,

    /// Graphic MMU LUT entry 384 high
    pub LUT384H: RWRegister<u32>,

    /// Graphic MMU LUT entry 385 low
    pub LUT385L: RWRegister<u32>,

    /// Graphic MMU LUT entry 385 high
    pub LUT385H: RWRegister<u32>,

    /// Graphic MMU LUT entry 386 low
    pub LUT386L: RWRegister<u32>,

    /// Graphic MMU LUT entry 386 high
    pub LUT386H: RWRegister<u32>,

    /// Graphic MMU LUT entry 387 low
    pub LUT387L: RWRegister<u32>,

    /// Graphic MMU LUT entry 387 high
    pub LUT387H: RWRegister<u32>,

    /// Graphic MMU LUT entry 388 low
    pub LUT388L: RWRegister<u32>,

    /// Graphic MMU LUT entry 388 high
    pub LUT388H: RWRegister<u32>,

    /// Graphic MMU LUT entry 389 low
    pub LUT389L: RWRegister<u32>,

    /// Graphic MMU LUT entry 389 high
    pub LUT389H: RWRegister<u32>,

    /// Graphic MMU LUT entry 390 low
    pub LUT390L: RWRegister<u32>,

    /// Graphic MMU LUT entry 390 high
    pub LUT390H: RWRegister<u32>,

    /// Graphic MMU LUT entry 391 low
    pub LUT391L: RWRegister<u32>,

    /// Graphic MMU LUT entry 391 high
    pub LUT391H: RWRegister<u32>,

    /// Graphic MMU LUT entry 392 low
    pub LUT392L: RWRegister<u32>,

    /// Graphic MMU LUT entry 392 high
    pub LUT392H: RWRegister<u32>,

    /// Graphic MMU LUT entry 393 low
    pub LUT393L: RWRegister<u32>,

    /// Graphic MMU LUT entry 393 high
    pub LUT393H: RWRegister<u32>,

    /// Graphic MMU LUT entry 394 low
    pub LUT394L: RWRegister<u32>,

    /// Graphic MMU LUT entry 394 high
    pub LUT394H: RWRegister<u32>,

    /// Graphic MMU LUT entry 395 low
    pub LUT395L: RWRegister<u32>,

    /// Graphic MMU LUT entry 395 high
    pub LUT395H: RWRegister<u32>,

    /// Graphic MMU LUT entry 396 low
    pub LUT396L: RWRegister<u32>,

    /// Graphic MMU LUT entry 396 high
    pub LUT396H: RWRegister<u32>,

    /// Graphic MMU LUT entry 397 low
    pub LUT397L: RWRegister<u32>,

    /// Graphic MMU LUT entry 397 high
    pub LUT397H: RWRegister<u32>,

    /// Graphic MMU LUT entry 398 low
    pub LUT398L: RWRegister<u32>,

    /// Graphic MMU LUT entry 398 high
    pub LUT398H: RWRegister<u32>,

    /// Graphic MMU LUT entry 399 low
    pub LUT399L: RWRegister<u32>,

    /// Graphic MMU LUT entry 399 high
    pub LUT399H: RWRegister<u32>,

    /// Graphic MMU LUT entry 400 low
    pub LUT400L: RWRegister<u32>,

    /// Graphic MMU LUT entry 400 high
    pub LUT400H: RWRegister<u32>,

    /// Graphic MMU LUT entry 401 low
    pub LUT401L: RWRegister<u32>,

    /// Graphic MMU LUT entry 401 high
    pub LUT401H: RWRegister<u32>,

    /// Graphic MMU LUT entry 402 low
    pub LUT402L: RWRegister<u32>,

    /// Graphic MMU LUT entry 402 high
    pub LUT402H: RWRegister<u32>,

    /// Graphic MMU LUT entry 403 low
    pub LUT403L: RWRegister<u32>,

    /// Graphic MMU LUT entry 403 high
    pub LUT403H: RWRegister<u32>,

    /// Graphic MMU LUT entry 404 low
    pub LUT404L: RWRegister<u32>,

    /// Graphic MMU LUT entry 404 high
    pub LUT404H: RWRegister<u32>,

    /// Graphic MMU LUT entry 405 low
    pub LUT405L: RWRegister<u32>,

    /// Graphic MMU LUT entry 405 high
    pub LUT405H: RWRegister<u32>,

    /// Graphic MMU LUT entry 406 low
    pub LUT406L: RWRegister<u32>,

    /// Graphic MMU LUT entry 406 high
    pub LUT406H: RWRegister<u32>,

    /// Graphic MMU LUT entry 407 low
    pub LUT407L: RWRegister<u32>,

    /// Graphic MMU LUT entry 407 high
    pub LUT407H: RWRegister<u32>,

    /// Graphic MMU LUT entry 408 low
    pub LUT408L: RWRegister<u32>,

    /// Graphic MMU LUT entry 408 high
    pub LUT408H: RWRegister<u32>,

    /// Graphic MMU LUT entry 409 low
    pub LUT409L: RWRegister<u32>,

    /// Graphic MMU LUT entry 409 high
    pub LUT409H: RWRegister<u32>,

    /// Graphic MMU LUT entry 410 low
    pub LUT410L: RWRegister<u32>,

    /// Graphic MMU LUT entry 410 high
    pub LUT410H: RWRegister<u32>,

    /// Graphic MMU LUT entry 411 low
    pub LUT411L: RWRegister<u32>,

    /// Graphic MMU LUT entry 411 high
    pub LUT411H: RWRegister<u32>,

    /// Graphic MMU LUT entry 412 low
    pub LUT412L: RWRegister<u32>,

    /// Graphic MMU LUT entry 412 high
    pub LUT412H: RWRegister<u32>,

    /// Graphic MMU LUT entry 413 low
    pub LUT413L: RWRegister<u32>,

    /// Graphic MMU LUT entry 413 high
    pub LUT413H: RWRegister<u32>,

    /// Graphic MMU LUT entry 414 low
    pub LUT414L: RWRegister<u32>,

    /// Graphic MMU LUT entry 414 high
    pub LUT414H: RWRegister<u32>,

    /// Graphic MMU LUT entry 415 low
    pub LUT415L: RWRegister<u32>,

    /// Graphic MMU LUT entry 415 high
    pub LUT415H: RWRegister<u32>,

    /// Graphic MMU LUT entry 416 low
    pub LUT416L: RWRegister<u32>,

    /// Graphic MMU LUT entry 416 high
    pub LUT416H: RWRegister<u32>,

    /// Graphic MMU LUT entry 417 low
    pub LUT417L: RWRegister<u32>,

    /// Graphic MMU LUT entry 417 high
    pub LUT417H: RWRegister<u32>,

    /// Graphic MMU LUT entry 418 low
    pub LUT418L: RWRegister<u32>,

    /// Graphic MMU LUT entry 418 high
    pub LUT418H: RWRegister<u32>,

    /// Graphic MMU LUT entry 419 low
    pub LUT419L: RWRegister<u32>,

    /// Graphic MMU LUT entry 419 high
    pub LUT419H: RWRegister<u32>,

    /// Graphic MMU LUT entry 420 low
    pub LUT420L: RWRegister<u32>,

    /// Graphic MMU LUT entry 420 high
    pub LUT420H: RWRegister<u32>,

    /// Graphic MMU LUT entry 421 low
    pub LUT421L: RWRegister<u32>,

    /// Graphic MMU LUT entry 421 high
    pub LUT421H: RWRegister<u32>,

    /// Graphic MMU LUT entry 422 low
    pub LUT422L: RWRegister<u32>,

    /// Graphic MMU LUT entry 422 high
    pub LUT422H: RWRegister<u32>,

    /// Graphic MMU LUT entry 423 low
    pub LUT423L: RWRegister<u32>,

    /// Graphic MMU LUT entry 423 high
    pub LUT423H: RWRegister<u32>,

    /// Graphic MMU LUT entry 424 low
    pub LUT424L: RWRegister<u32>,

    /// Graphic MMU LUT entry 424 high
    pub LUT424H: RWRegister<u32>,

    /// Graphic MMU LUT entry 425 low
    pub LUT425L: RWRegister<u32>,

    /// Graphic MMU LUT entry 425 high
    pub LUT425H: RWRegister<u32>,

    /// Graphic MMU LUT entry 426 low
    pub LUT426L: RWRegister<u32>,

    /// Graphic MMU LUT entry 426 high
    pub LUT426H: RWRegister<u32>,

    /// Graphic MMU LUT entry 427 low
    pub LUT427L: RWRegister<u32>,

    /// Graphic MMU LUT entry 427 high
    pub LUT427H: RWRegister<u32>,

    /// Graphic MMU LUT entry 428 low
    pub LUT428L: RWRegister<u32>,

    /// Graphic MMU LUT entry 428 high
    pub LUT428H: RWRegister<u32>,

    /// Graphic MMU LUT entry 429 low
    pub LUT429L: RWRegister<u32>,

    /// Graphic MMU LUT entry 429 high
    pub LUT429H: RWRegister<u32>,

    /// Graphic MMU LUT entry 430 low
    pub LUT430L: RWRegister<u32>,

    /// Graphic MMU LUT entry 430 high
    pub LUT430H: RWRegister<u32>,

    /// Graphic MMU LUT entry 431 low
    pub LUT431L: RWRegister<u32>,

    /// Graphic MMU LUT entry 431 high
    pub LUT431H: RWRegister<u32>,

    /// Graphic MMU LUT entry 432 low
    pub LUT432L: RWRegister<u32>,

    /// Graphic MMU LUT entry 432 high
    pub LUT432H: RWRegister<u32>,

    /// Graphic MMU LUT entry 433 low
    pub LUT433L: RWRegister<u32>,

    /// Graphic MMU LUT entry 433 high
    pub LUT433H: RWRegister<u32>,

    /// Graphic MMU LUT entry 434 low
    pub LUT434L: RWRegister<u32>,

    /// Graphic MMU LUT entry 434 high
    pub LUT434H: RWRegister<u32>,

    /// Graphic MMU LUT entry 435 low
    pub LUT435L: RWRegister<u32>,

    /// Graphic MMU LUT entry 435 high
    pub LUT435H: RWRegister<u32>,

    /// Graphic MMU LUT entry 436 low
    pub LUT436L: RWRegister<u32>,

    /// Graphic MMU LUT entry 436 high
    pub LUT436H: RWRegister<u32>,

    /// Graphic MMU LUT entry 437 low
    pub LUT437L: RWRegister<u32>,

    /// Graphic MMU LUT entry 437 high
    pub LUT437H: RWRegister<u32>,

    /// Graphic MMU LUT entry 438 low
    pub LUT438L: RWRegister<u32>,

    /// Graphic MMU LUT entry 438 high
    pub LUT438H: RWRegister<u32>,

    /// Graphic MMU LUT entry 439 low
    pub LUT439L: RWRegister<u32>,

    /// Graphic MMU LUT entry 439 high
    pub LUT439H: RWRegister<u32>,

    /// Graphic MMU LUT entry 440 low
    pub LUT440L: RWRegister<u32>,

    /// Graphic MMU LUT entry 440 high
    pub LUT440H: RWRegister<u32>,

    /// Graphic MMU LUT entry 441 low
    pub LUT441L: RWRegister<u32>,

    /// Graphic MMU LUT entry 441 high
    pub LUT441H: RWRegister<u32>,

    /// Graphic MMU LUT entry 442 low
    pub LUT442L: RWRegister<u32>,

    /// Graphic MMU LUT entry 442 high
    pub LUT442H: RWRegister<u32>,

    /// Graphic MMU LUT entry 443 low
    pub LUT443L: RWRegister<u32>,

    /// Graphic MMU LUT entry 443 high
    pub LUT443H: RWRegister<u32>,

    /// Graphic MMU LUT entry 444 low
    pub LUT444L: RWRegister<u32>,

    /// Graphic MMU LUT entry 444 high
    pub LUT444H: RWRegister<u32>,

    /// Graphic MMU LUT entry 445 low
    pub LUT445L: RWRegister<u32>,

    /// Graphic MMU LUT entry 445 high
    pub LUT445H: RWRegister<u32>,

    /// Graphic MMU LUT entry 446 low
    pub LUT446L: RWRegister<u32>,

    /// Graphic MMU LUT entry 446 high
    pub LUT446H: RWRegister<u32>,

    /// Graphic MMU LUT entry 447 low
    pub LUT447L: RWRegister<u32>,

    /// Graphic MMU LUT entry 447 high
    pub LUT447H: RWRegister<u32>,

    /// Graphic MMU LUT entry 448 low
    pub LUT448L: RWRegister<u32>,

    /// Graphic MMU LUT entry 448 high
    pub LUT448H: RWRegister<u32>,

    /// Graphic MMU LUT entry 449 low
    pub LUT449L: RWRegister<u32>,

    /// Graphic MMU LUT entry 449 high
    pub LUT449H: RWRegister<u32>,

    /// Graphic MMU LUT entry 450 low
    pub LUT450L: RWRegister<u32>,

    /// Graphic MMU LUT entry 450 high
    pub LUT450H: RWRegister<u32>,

    /// Graphic MMU LUT entry 451 low
    pub LUT451L: RWRegister<u32>,

    /// Graphic MMU LUT entry 451 high
    pub LUT451H: RWRegister<u32>,

    /// Graphic MMU LUT entry 452 low
    pub LUT452L: RWRegister<u32>,

    /// Graphic MMU LUT entry 452 high
    pub LUT452H: RWRegister<u32>,

    /// Graphic MMU LUT entry 453 low
    pub LUT453L: RWRegister<u32>,

    /// Graphic MMU LUT entry 453 high
    pub LUT453H: RWRegister<u32>,

    /// Graphic MMU LUT entry 454 low
    pub LUT454L: RWRegister<u32>,

    /// Graphic MMU LUT entry 454 high
    pub LUT454H: RWRegister<u32>,

    /// Graphic MMU LUT entry 455 low
    pub LUT455L: RWRegister<u32>,

    /// Graphic MMU LUT entry 455 high
    pub LUT455H: RWRegister<u32>,

    /// Graphic MMU LUT entry 456 low
    pub LUT456L: RWRegister<u32>,

    /// Graphic MMU LUT entry 456 high
    pub LUT456H: RWRegister<u32>,

    /// Graphic MMU LUT entry 457 low
    pub LUT457L: RWRegister<u32>,

    /// Graphic MMU LUT entry 457 high
    pub LUT457H: RWRegister<u32>,

    /// Graphic MMU LUT entry 458 low
    pub LUT458L: RWRegister<u32>,

    /// Graphic MMU LUT entry 458 high
    pub LUT458H: RWRegister<u32>,

    /// Graphic MMU LUT entry 459 low
    pub LUT459L: RWRegister<u32>,

    /// Graphic MMU LUT entry 459 high
    pub LUT459H: RWRegister<u32>,

    /// Graphic MMU LUT entry 460 low
    pub LUT460L: RWRegister<u32>,

    /// Graphic MMU LUT entry 460 high
    pub LUT460H: RWRegister<u32>,

    /// Graphic MMU LUT entry 461 low
    pub LUT461L: RWRegister<u32>,

    /// Graphic MMU LUT entry 461 high
    pub LUT461H: RWRegister<u32>,

    /// Graphic MMU LUT entry 462 low
    pub LUT462L: RWRegister<u32>,

    /// Graphic MMU LUT entry 462 high
    pub LUT462H: RWRegister<u32>,

    /// Graphic MMU LUT entry 463 low
    pub LUT463L: RWRegister<u32>,

    /// Graphic MMU LUT entry 463 high
    pub LUT463H: RWRegister<u32>,

    /// Graphic MMU LUT entry 464 low
    pub LUT464L: RWRegister<u32>,

    /// Graphic MMU LUT entry 464 high
    pub LUT464H: RWRegister<u32>,

    /// Graphic MMU LUT entry 465 low
    pub LUT465L: RWRegister<u32>,

    /// Graphic MMU LUT entry 465 high
    pub LUT465H: RWRegister<u32>,

    /// Graphic MMU LUT entry 466 low
    pub LUT466L: RWRegister<u32>,

    /// Graphic MMU LUT entry 466 high
    pub LUT466H: RWRegister<u32>,

    /// Graphic MMU LUT entry 467 low
    pub LUT467L: RWRegister<u32>,

    /// Graphic MMU LUT entry 467 high
    pub LUT467H: RWRegister<u32>,

    /// Graphic MMU LUT entry 468 low
    pub LUT468L: RWRegister<u32>,

    /// Graphic MMU LUT entry 468 high
    pub LUT468H: RWRegister<u32>,

    /// Graphic MMU LUT entry 469 low
    pub LUT469L: RWRegister<u32>,

    /// Graphic MMU LUT entry 469 high
    pub LUT469H: RWRegister<u32>,

    /// Graphic MMU LUT entry 470 low
    pub LUT470L: RWRegister<u32>,

    /// Graphic MMU LUT entry 470 high
    pub LUT470H: RWRegister<u32>,

    /// Graphic MMU LUT entry 471 low
    pub LUT471L: RWRegister<u32>,

    /// Graphic MMU LUT entry 471 high
    pub LUT471H: RWRegister<u32>,

    /// Graphic MMU LUT entry 472 low
    pub LUT472L: RWRegister<u32>,

    /// Graphic MMU LUT entry 472 high
    pub LUT472H: RWRegister<u32>,

    /// Graphic MMU LUT entry 473 low
    pub LUT473L: RWRegister<u32>,

    /// Graphic MMU LUT entry 473 high
    pub LUT473H: RWRegister<u32>,

    /// Graphic MMU LUT entry 474 low
    pub LUT474L: RWRegister<u32>,

    /// Graphic MMU LUT entry 474 high
    pub LUT474H: RWRegister<u32>,

    /// Graphic MMU LUT entry 475 low
    pub LUT475L: RWRegister<u32>,

    /// Graphic MMU LUT entry 475 high
    pub LUT475H: RWRegister<u32>,

    /// Graphic MMU LUT entry 476 low
    pub LUT476L: RWRegister<u32>,

    /// Graphic MMU LUT entry 476 high
    pub LUT476H: RWRegister<u32>,

    /// Graphic MMU LUT entry 477 low
    pub LUT477L: RWRegister<u32>,

    /// Graphic MMU LUT entry 477 high
    pub LUT477H: RWRegister<u32>,

    /// Graphic MMU LUT entry 478 low
    pub LUT478L: RWRegister<u32>,

    /// Graphic MMU LUT entry 478 high
    pub LUT478H: RWRegister<u32>,

    /// Graphic MMU LUT entry 479 low
    pub LUT479L: RWRegister<u32>,

    /// Graphic MMU LUT entry 479 high
    pub LUT479H: RWRegister<u32>,

    /// Graphic MMU LUT entry 480 low
    pub LUT480L: RWRegister<u32>,

    /// Graphic MMU LUT entry 480 high
    pub LUT480H: RWRegister<u32>,

    /// Graphic MMU LUT entry 481 low
    pub LUT481L: RWRegister<u32>,

    /// Graphic MMU LUT entry 481 high
    pub LUT481H: RWRegister<u32>,

    /// Graphic MMU LUT entry 482 low
    pub LUT482L: RWRegister<u32>,

    /// Graphic MMU LUT entry 482 high
    pub LUT482H: RWRegister<u32>,

    /// Graphic MMU LUT entry 483 low
    pub LUT483L: RWRegister<u32>,

    /// Graphic MMU LUT entry 483 high
    pub LUT483H: RWRegister<u32>,

    /// Graphic MMU LUT entry 484 low
    pub LUT484L: RWRegister<u32>,

    /// Graphic MMU LUT entry 484 high
    pub LUT484H: RWRegister<u32>,

    /// Graphic MMU LUT entry 485 low
    pub LUT485L: RWRegister<u32>,

    /// Graphic MMU LUT entry 485 high
    pub LUT485H: RWRegister<u32>,

    /// Graphic MMU LUT entry 486 low
    pub LUT486L: RWRegister<u32>,

    /// Graphic MMU LUT entry 486 high
    pub LUT486H: RWRegister<u32>,

    /// Graphic MMU LUT entry 487 low
    pub LUT487L: RWRegister<u32>,

    /// Graphic MMU LUT entry 487 high
    pub LUT487H: RWRegister<u32>,

    /// Graphic MMU LUT entry 488 low
    pub LUT488L: RWRegister<u32>,

    /// Graphic MMU LUT entry 488 high
    pub LUT488H: RWRegister<u32>,

    /// Graphic MMU LUT entry 489 low
    pub LUT489L: RWRegister<u32>,

    /// Graphic MMU LUT entry 489 high
    pub LUT489H: RWRegister<u32>,

    /// Graphic MMU LUT entry 490 low
    pub LUT490L: RWRegister<u32>,

    /// Graphic MMU LUT entry 490 high
    pub LUT490H: RWRegister<u32>,

    /// Graphic MMU LUT entry 491 low
    pub LUT491L: RWRegister<u32>,

    /// Graphic MMU LUT entry 491 high
    pub LUT491H: RWRegister<u32>,

    /// Graphic MMU LUT entry 492 low
    pub LUT492L: RWRegister<u32>,

    /// Graphic MMU LUT entry 492 high
    pub LUT492H: RWRegister<u32>,

    /// Graphic MMU LUT entry 493 low
    pub LUT493L: RWRegister<u32>,

    /// Graphic MMU LUT entry 493 high
    pub LUT493H: RWRegister<u32>,

    /// Graphic MMU LUT entry 494 low
    pub LUT494L: RWRegister<u32>,

    /// Graphic MMU LUT entry 494 high
    pub LUT494H: RWRegister<u32>,

    /// Graphic MMU LUT entry 495 low
    pub LUT495L: RWRegister<u32>,

    /// Graphic MMU LUT entry 495 high
    pub LUT495H: RWRegister<u32>,

    /// Graphic MMU LUT entry 496 low
    pub LUT496L: RWRegister<u32>,

    /// Graphic MMU LUT entry 496 high
    pub LUT496H: RWRegister<u32>,

    /// Graphic MMU LUT entry 497 low
    pub LUT497L: RWRegister<u32>,

    /// Graphic MMU LUT entry 497 high
    pub LUT497H: RWRegister<u32>,

    /// Graphic MMU LUT entry 498 low
    pub LUT498L: RWRegister<u32>,

    /// Graphic MMU LUT entry 498 high
    pub LUT498H: RWRegister<u32>,

    /// Graphic MMU LUT entry 499 low
    pub LUT499L: RWRegister<u32>,

    /// Graphic MMU LUT entry 499 high
    pub LUT499H: RWRegister<u32>,

    /// Graphic MMU LUT entry 500 low
    pub LUT500L: RWRegister<u32>,

    /// Graphic MMU LUT entry 500 high
    pub LUT500H: RWRegister<u32>,

    /// Graphic MMU LUT entry 501 low
    pub LUT501L: RWRegister<u32>,

    /// Graphic MMU LUT entry 501 high
    pub LUT501H: RWRegister<u32>,

    /// Graphic MMU LUT entry 502 low
    pub LUT502L: RWRegister<u32>,

    /// Graphic MMU LUT entry 502 high
    pub LUT502H: RWRegister<u32>,

    /// Graphic MMU LUT entry 503 low
    pub LUT503L: RWRegister<u32>,

    /// Graphic MMU LUT entry 503 high
    pub LUT503H: RWRegister<u32>,

    /// Graphic MMU LUT entry 504 low
    pub LUT504L: RWRegister<u32>,

    /// Graphic MMU LUT entry 504 high
    pub LUT504H: RWRegister<u32>,

    /// Graphic MMU LUT entry 505 low
    pub LUT505L: RWRegister<u32>,

    /// Graphic MMU LUT entry 505 high
    pub LUT505H: RWRegister<u32>,

    /// Graphic MMU LUT entry 506 low
    pub LUT506L: RWRegister<u32>,

    /// Graphic MMU LUT entry 506 high
    pub LUT506H: RWRegister<u32>,

    /// Graphic MMU LUT entry 507 low
    pub LUT507L: RWRegister<u32>,

    /// Graphic MMU LUT entry 507 high
    pub LUT507H: RWRegister<u32>,

    /// Graphic MMU LUT entry 508 low
    pub LUT508L: RWRegister<u32>,

    /// Graphic MMU LUT entry 508 high
    pub LUT508H: RWRegister<u32>,

    /// Graphic MMU LUT entry 509 low
    pub LUT509L: RWRegister<u32>,

    /// Graphic MMU LUT entry 509 high
    pub LUT509H: RWRegister<u32>,

    /// Graphic MMU LUT entry 510 low
    pub LUT510L: RWRegister<u32>,

    /// Graphic MMU LUT entry 510 high
    pub LUT510H: RWRegister<u32>,

    /// Graphic MMU LUT entry 511 low
    pub LUT511L: RWRegister<u32>,

    /// Graphic MMU LUT entry 511 high
    pub LUT511H: RWRegister<u32>,

    /// Graphic MMU LUT entry 512 low
    pub LUT512L: RWRegister<u32>,

    /// Graphic MMU LUT entry 512 high
    pub LUT512H: RWRegister<u32>,

    /// Graphic MMU LUT entry 513 low
    pub LUT513L: RWRegister<u32>,

    /// Graphic MMU LUT entry 513 high
    pub LUT513H: RWRegister<u32>,

    /// Graphic MMU LUT entry 514 low
    pub LUT514L: RWRegister<u32>,

    /// Graphic MMU LUT entry 514 high
    pub LUT514H: RWRegister<u32>,

    /// Graphic MMU LUT entry 515 low
    pub LUT515L: RWRegister<u32>,

    /// Graphic MMU LUT entry 515 high
    pub LUT515H: RWRegister<u32>,

    /// Graphic MMU LUT entry 516 low
    pub LUT516L: RWRegister<u32>,

    /// Graphic MMU LUT entry 516 high
    pub LUT516H: RWRegister<u32>,

    /// Graphic MMU LUT entry 517 low
    pub LUT517L: RWRegister<u32>,

    /// Graphic MMU LUT entry 517 high
    pub LUT517H: RWRegister<u32>,

    /// Graphic MMU LUT entry 518 low
    pub LUT518L: RWRegister<u32>,

    /// Graphic MMU LUT entry 518 high
    pub LUT518H: RWRegister<u32>,

    /// Graphic MMU LUT entry 519 low
    pub LUT519L: RWRegister<u32>,

    /// Graphic MMU LUT entry 519 high
    pub LUT519H: RWRegister<u32>,

    /// Graphic MMU LUT entry 520 low
    pub LUT520L: RWRegister<u32>,

    /// Graphic MMU LUT entry 520 high
    pub LUT520H: RWRegister<u32>,

    /// Graphic MMU LUT entry 521 low
    pub LUT521L: RWRegister<u32>,

    /// Graphic MMU LUT entry 521 high
    pub LUT521H: RWRegister<u32>,

    /// Graphic MMU LUT entry 522 low
    pub LUT522L: RWRegister<u32>,

    /// Graphic MMU LUT entry 522 high
    pub LUT522H: RWRegister<u32>,

    /// Graphic MMU LUT entry 523 low
    pub LUT523L: RWRegister<u32>,

    /// Graphic MMU LUT entry 523 high
    pub LUT523H: RWRegister<u32>,

    /// Graphic MMU LUT entry 524 low
    pub LUT524L: RWRegister<u32>,

    /// Graphic MMU LUT entry 524 high
    pub LUT524H: RWRegister<u32>,

    /// Graphic MMU LUT entry 525 low
    pub LUT525L: RWRegister<u32>,

    /// Graphic MMU LUT entry 525 high
    pub LUT525H: RWRegister<u32>,

    /// Graphic MMU LUT entry 526 low
    pub LUT526L: RWRegister<u32>,

    /// Graphic MMU LUT entry 526 high
    pub LUT526H: RWRegister<u32>,

    /// Graphic MMU LUT entry 527 low
    pub LUT527L: RWRegister<u32>,

    /// Graphic MMU LUT entry 527 high
    pub LUT527H: RWRegister<u32>,

    /// Graphic MMU LUT entry 528 low
    pub LUT528L: RWRegister<u32>,

    /// Graphic MMU LUT entry 528 high
    pub LUT528H: RWRegister<u32>,

    /// Graphic MMU LUT entry 529 low
    pub LUT529L: RWRegister<u32>,

    /// Graphic MMU LUT entry 529 high
    pub LUT529H: RWRegister<u32>,

    /// Graphic MMU LUT entry 530 low
    pub LUT530L: RWRegister<u32>,

    /// Graphic MMU LUT entry 530 high
    pub LUT530H: RWRegister<u32>,

    /// Graphic MMU LUT entry 531 low
    pub LUT531L: RWRegister<u32>,

    /// Graphic MMU LUT entry 531 high
    pub LUT531H: RWRegister<u32>,

    /// Graphic MMU LUT entry 532 low
    pub LUT532L: RWRegister<u32>,

    /// Graphic MMU LUT entry 532 high
    pub LUT532H: RWRegister<u32>,

    /// Graphic MMU LUT entry 533 low
    pub LUT533L: RWRegister<u32>,

    /// Graphic MMU LUT entry 533 high
    pub LUT533H: RWRegister<u32>,

    /// Graphic MMU LUT entry 534 low
    pub LUT534L: RWRegister<u32>,

    /// Graphic MMU LUT entry 534 high
    pub LUT534H: RWRegister<u32>,

    /// Graphic MMU LUT entry 535 low
    pub LUT535L: RWRegister<u32>,

    /// Graphic MMU LUT entry 535 high
    pub LUT535H: RWRegister<u32>,

    /// Graphic MMU LUT entry 536 low
    pub LUT536L: RWRegister<u32>,

    /// Graphic MMU LUT entry 536 high
    pub LUT536H: RWRegister<u32>,

    /// Graphic MMU LUT entry 537 low
    pub LUT537L: RWRegister<u32>,

    /// Graphic MMU LUT entry 537 high
    pub LUT537H: RWRegister<u32>,

    /// Graphic MMU LUT entry 538 low
    pub LUT538L: RWRegister<u32>,

    /// Graphic MMU LUT entry 538 high
    pub LUT538H: RWRegister<u32>,

    /// Graphic MMU LUT entry 539 low
    pub LUT539L: RWRegister<u32>,

    /// Graphic MMU LUT entry 539 high
    pub LUT539H: RWRegister<u32>,

    /// Graphic MMU LUT entry 540 low
    pub LUT540L: RWRegister<u32>,

    /// Graphic MMU LUT entry 540 high
    pub LUT540H: RWRegister<u32>,

    /// Graphic MMU LUT entry 541 low
    pub LUT541L: RWRegister<u32>,

    /// Graphic MMU LUT entry 541 high
    pub LUT541H: RWRegister<u32>,

    /// Graphic MMU LUT entry 542 low
    pub LUT542L: RWRegister<u32>,

    /// Graphic MMU LUT entry 542 high
    pub LUT542H: RWRegister<u32>,

    /// Graphic MMU LUT entry 543 low
    pub LUT543L: RWRegister<u32>,

    /// Graphic MMU LUT entry 543 high
    pub LUT543H: RWRegister<u32>,

    /// Graphic MMU LUT entry 544 low
    pub LUT544L: RWRegister<u32>,

    /// Graphic MMU LUT entry 544 high
    pub LUT544H: RWRegister<u32>,

    /// Graphic MMU LUT entry 545 low
    pub LUT545L: RWRegister<u32>,

    /// Graphic MMU LUT entry 545 high
    pub LUT545H: RWRegister<u32>,

    /// Graphic MMU LUT entry 546 low
    pub LUT546L: RWRegister<u32>,

    /// Graphic MMU LUT entry 546 high
    pub LUT546H: RWRegister<u32>,

    /// Graphic MMU LUT entry 547 low
    pub LUT547L: RWRegister<u32>,

    /// Graphic MMU LUT entry 547 high
    pub LUT547H: RWRegister<u32>,

    /// Graphic MMU LUT entry 548 low
    pub LUT548L: RWRegister<u32>,

    /// Graphic MMU LUT entry 548 high
    pub LUT548H: RWRegister<u32>,

    /// Graphic MMU LUT entry 549 low
    pub LUT549L: RWRegister<u32>,

    /// Graphic MMU LUT entry 549 high
    pub LUT549H: RWRegister<u32>,

    /// Graphic MMU LUT entry 550 low
    pub LUT550L: RWRegister<u32>,

    /// Graphic MMU LUT entry 550 high
    pub LUT550H: RWRegister<u32>,

    /// Graphic MMU LUT entry 551 low
    pub LUT551L: RWRegister<u32>,

    /// Graphic MMU LUT entry 551 high
    pub LUT551H: RWRegister<u32>,

    /// Graphic MMU LUT entry 552 low
    pub LUT552L: RWRegister<u32>,

    /// Graphic MMU LUT entry 552 high
    pub LUT552H: RWRegister<u32>,

    /// Graphic MMU LUT entry 553 low
    pub LUT553L: RWRegister<u32>,

    /// Graphic MMU LUT entry 553 high
    pub LUT553H: RWRegister<u32>,

    /// Graphic MMU LUT entry 554 low
    pub LUT554L: RWRegister<u32>,

    /// Graphic MMU LUT entry 554 high
    pub LUT554H: RWRegister<u32>,

    /// Graphic MMU LUT entry 555 low
    pub LUT555L: RWRegister<u32>,

    /// Graphic MMU LUT entry 555 high
    pub LUT555H: RWRegister<u32>,

    /// Graphic MMU LUT entry 556 low
    pub LUT556L: RWRegister<u32>,

    /// Graphic MMU LUT entry 556 high
    pub LUT556H: RWRegister<u32>,

    /// Graphic MMU LUT entry 557 low
    pub LUT557L: RWRegister<u32>,

    /// Graphic MMU LUT entry 557 high
    pub LUT557H: RWRegister<u32>,

    /// Graphic MMU LUT entry 558 low
    pub LUT558L: RWRegister<u32>,

    /// Graphic MMU LUT entry 558 high
    pub LUT558H: RWRegister<u32>,

    /// Graphic MMU LUT entry 559 low
    pub LUT559L: RWRegister<u32>,

    /// Graphic MMU LUT entry 559 high
    pub LUT559H: RWRegister<u32>,

    /// Graphic MMU LUT entry 560 low
    pub LUT560L: RWRegister<u32>,

    /// Graphic MMU LUT entry 560 high
    pub LUT560H: RWRegister<u32>,

    /// Graphic MMU LUT entry 561 low
    pub LUT561L: RWRegister<u32>,

    /// Graphic MMU LUT entry 561 high
    pub LUT561H: RWRegister<u32>,

    /// Graphic MMU LUT entry 562 low
    pub LUT562L: RWRegister<u32>,

    /// Graphic MMU LUT entry 562 high
    pub LUT562H: RWRegister<u32>,

    /// Graphic MMU LUT entry 563 low
    pub LUT563L: RWRegister<u32>,

    /// Graphic MMU LUT entry 563 high
    pub LUT563H: RWRegister<u32>,

    /// Graphic MMU LUT entry 564 low
    pub LUT564L: RWRegister<u32>,

    /// Graphic MMU LUT entry 564 high
    pub LUT564H: RWRegister<u32>,

    /// Graphic MMU LUT entry 565 low
    pub LUT565L: RWRegister<u32>,

    /// Graphic MMU LUT entry 565 high
    pub LUT565H: RWRegister<u32>,

    /// Graphic MMU LUT entry 566 low
    pub LUT566L: RWRegister<u32>,

    /// Graphic MMU LUT entry 566 high
    pub LUT566H: RWRegister<u32>,

    /// Graphic MMU LUT entry 567 low
    pub LUT567L: RWRegister<u32>,

    /// Graphic MMU LUT entry 567 high
    pub LUT567H: RWRegister<u32>,

    /// Graphic MMU LUT entry 568 low
    pub LUT568L: RWRegister<u32>,

    /// Graphic MMU LUT entry 568 high
    pub LUT568H: RWRegister<u32>,

    /// Graphic MMU LUT entry 569 low
    pub LUT569L: RWRegister<u32>,

    /// Graphic MMU LUT entry 569 high
    pub LUT569H: RWRegister<u32>,

    /// Graphic MMU LUT entry 570 low
    pub LUT570L: RWRegister<u32>,

    /// Graphic MMU LUT entry 570 high
    pub LUT570H: RWRegister<u32>,

    /// Graphic MMU LUT entry 571 low
    pub LUT571L: RWRegister<u32>,

    /// Graphic MMU LUT entry 571 high
    pub LUT571H: RWRegister<u32>,

    /// Graphic MMU LUT entry 572 low
    pub LUT572L: RWRegister<u32>,

    /// Graphic MMU LUT entry 572 high
    pub LUT572H: RWRegister<u32>,

    /// Graphic MMU LUT entry 573 low
    pub LUT573L: RWRegister<u32>,

    /// Graphic MMU LUT entry 573 high
    pub LUT573H: RWRegister<u32>,

    /// Graphic MMU LUT entry 574 low
    pub LUT574L: RWRegister<u32>,

    /// Graphic MMU LUT entry 574 high
    pub LUT574H: RWRegister<u32>,

    /// Graphic MMU LUT entry 575 low
    pub LUT575L: RWRegister<u32>,

    /// Graphic MMU LUT entry 575 high
    pub LUT575H: RWRegister<u32>,

    /// Graphic MMU LUT entry 576 low
    pub LUT576L: RWRegister<u32>,

    /// Graphic MMU LUT entry 576 high
    pub LUT576H: RWRegister<u32>,

    /// Graphic MMU LUT entry 577 low
    pub LUT577L: RWRegister<u32>,

    /// Graphic MMU LUT entry 577 high
    pub LUT577H: RWRegister<u32>,

    /// Graphic MMU LUT entry 578 low
    pub LUT578L: RWRegister<u32>,

    /// Graphic MMU LUT entry 578 high
    pub LUT578H: RWRegister<u32>,

    /// Graphic MMU LUT entry 579 low
    pub LUT579L: RWRegister<u32>,

    /// Graphic MMU LUT entry 579 high
    pub LUT579H: RWRegister<u32>,

    /// Graphic MMU LUT entry 580 low
    pub LUT580L: RWRegister<u32>,

    /// Graphic MMU LUT entry 580 high
    pub LUT580H: RWRegister<u32>,

    /// Graphic MMU LUT entry 581 low
    pub LUT581L: RWRegister<u32>,

    /// Graphic MMU LUT entry 581 high
    pub LUT581H: RWRegister<u32>,

    /// Graphic MMU LUT entry 582 low
    pub LUT582L: RWRegister<u32>,

    /// Graphic MMU LUT entry 582 high
    pub LUT582H: RWRegister<u32>,

    /// Graphic MMU LUT entry 583 low
    pub LUT583L: RWRegister<u32>,

    /// Graphic MMU LUT entry 583 high
    pub LUT583H: RWRegister<u32>,

    /// Graphic MMU LUT entry 584 low
    pub LUT584L: RWRegister<u32>,

    /// Graphic MMU LUT entry 584 high
    pub LUT584H: RWRegister<u32>,

    /// Graphic MMU LUT entry 585 low
    pub LUT585L: RWRegister<u32>,

    /// Graphic MMU LUT entry 585 high
    pub LUT585H: RWRegister<u32>,

    /// Graphic MMU LUT entry 586 low
    pub LUT586L: RWRegister<u32>,

    /// Graphic MMU LUT entry 586 high
    pub LUT586H: RWRegister<u32>,

    /// Graphic MMU LUT entry 587 low
    pub LUT587L: RWRegister<u32>,

    /// Graphic MMU LUT entry 587 high
    pub LUT587H: RWRegister<u32>,

    /// Graphic MMU LUT entry 588 low
    pub LUT588L: RWRegister<u32>,

    /// Graphic MMU LUT entry 588 high
    pub LUT588H: RWRegister<u32>,

    /// Graphic MMU LUT entry 589 low
    pub LUT589L: RWRegister<u32>,

    /// Graphic MMU LUT entry 589 high
    pub LUT589H: RWRegister<u32>,

    /// Graphic MMU LUT entry 590 low
    pub LUT590L: RWRegister<u32>,

    /// Graphic MMU LUT entry 590 high
    pub LUT590H: RWRegister<u32>,

    /// Graphic MMU LUT entry 591 low
    pub LUT591L: RWRegister<u32>,

    /// Graphic MMU LUT entry 591 high
    pub LUT591H: RWRegister<u32>,

    /// Graphic MMU LUT entry 592 low
    pub LUT592L: RWRegister<u32>,

    /// Graphic MMU LUT entry 592 high
    pub LUT592H: RWRegister<u32>,

    /// Graphic MMU LUT entry 593 low
    pub LUT593L: RWRegister<u32>,

    /// Graphic MMU LUT entry 593 high
    pub LUT593H: RWRegister<u32>,

    /// Graphic MMU LUT entry 594 low
    pub LUT594L: RWRegister<u32>,

    /// Graphic MMU LUT entry 594 high
    pub LUT594H: RWRegister<u32>,

    /// Graphic MMU LUT entry 595 low
    pub LUT595L: RWRegister<u32>,

    /// Graphic MMU LUT entry 595 high
    pub LUT595H: RWRegister<u32>,

    /// Graphic MMU LUT entry 596 low
    pub LUT596L: RWRegister<u32>,

    /// Graphic MMU LUT entry 596 high
    pub LUT596H: RWRegister<u32>,

    /// Graphic MMU LUT entry 597 low
    pub LUT597L: RWRegister<u32>,

    /// Graphic MMU LUT entry 597 high
    pub LUT597H: RWRegister<u32>,

    /// Graphic MMU LUT entry 598 low
    pub LUT598L: RWRegister<u32>,

    /// Graphic MMU LUT entry 598 high
    pub LUT598H: RWRegister<u32>,

    /// Graphic MMU LUT entry 599 low
    pub LUT599L: RWRegister<u32>,

    /// Graphic MMU LUT entry 599 high
    pub LUT599H: RWRegister<u32>,

    /// Graphic MMU LUT entry 600 low
    pub LUT600L: RWRegister<u32>,

    /// Graphic MMU LUT entry 600 high
    pub LUT600H: RWRegister<u32>,

    /// Graphic MMU LUT entry 601 low
    pub LUT601L: RWRegister<u32>,

    /// Graphic MMU LUT entry 601 high
    pub LUT601H: RWRegister<u32>,

    /// Graphic MMU LUT entry 602 low
    pub LUT602L: RWRegister<u32>,

    /// Graphic MMU LUT entry 602 high
    pub LUT602H: RWRegister<u32>,

    /// Graphic MMU LUT entry 603 low
    pub LUT603L: RWRegister<u32>,

    /// Graphic MMU LUT entry 603 high
    pub LUT603H: RWRegister<u32>,

    /// Graphic MMU LUT entry 604 low
    pub LUT604L: RWRegister<u32>,

    /// Graphic MMU LUT entry 604 high
    pub LUT604H: RWRegister<u32>,

    /// Graphic MMU LUT entry 605 low
    pub LUT605L: RWRegister<u32>,

    /// Graphic MMU LUT entry 605 high
    pub LUT605H: RWRegister<u32>,

    /// Graphic MMU LUT entry 606 low
    pub LUT606L: RWRegister<u32>,

    /// Graphic MMU LUT entry 606 high
    pub LUT606H: RWRegister<u32>,

    /// Graphic MMU LUT entry 607 low
    pub LUT607L: RWRegister<u32>,

    /// Graphic MMU LUT entry 607 high
    pub LUT607H: RWRegister<u32>,

    /// Graphic MMU LUT entry 608 low
    pub LUT608L: RWRegister<u32>,

    /// Graphic MMU LUT entry 608 high
    pub LUT608H: RWRegister<u32>,

    /// Graphic MMU LUT entry 609 low
    pub LUT609L: RWRegister<u32>,

    /// Graphic MMU LUT entry 609 high
    pub LUT609H: RWRegister<u32>,

    /// Graphic MMU LUT entry 610 low
    pub LUT610L: RWRegister<u32>,

    /// Graphic MMU LUT entry 610 high
    pub LUT610H: RWRegister<u32>,

    /// Graphic MMU LUT entry 611 low
    pub LUT611L: RWRegister<u32>,

    /// Graphic MMU LUT entry 611 high
    pub LUT611H: RWRegister<u32>,

    /// Graphic MMU LUT entry 612 low
    pub LUT612L: RWRegister<u32>,

    /// Graphic MMU LUT entry 612 high
    pub LUT612H: RWRegister<u32>,

    /// Graphic MMU LUT entry 613 low
    pub LUT613L: RWRegister<u32>,

    /// Graphic MMU LUT entry 613 high
    pub LUT613H: RWRegister<u32>,

    /// Graphic MMU LUT entry 614 low
    pub LUT614L: RWRegister<u32>,

    /// Graphic MMU LUT entry 614 high
    pub LUT614H: RWRegister<u32>,

    /// Graphic MMU LUT entry 615 low
    pub LUT615L: RWRegister<u32>,

    /// Graphic MMU LUT entry 615 high
    pub LUT615H: RWRegister<u32>,

    /// Graphic MMU LUT entry 616 low
    pub LUT616L: RWRegister<u32>,

    /// Graphic MMU LUT entry 616 high
    pub LUT616H: RWRegister<u32>,

    /// Graphic MMU LUT entry 617 low
    pub LUT617L: RWRegister<u32>,

    /// Graphic MMU LUT entry 617 high
    pub LUT617H: RWRegister<u32>,

    /// Graphic MMU LUT entry 618 low
    pub LUT618L: RWRegister<u32>,

    /// Graphic MMU LUT entry 618 high
    pub LUT618H: RWRegister<u32>,

    /// Graphic MMU LUT entry 619 low
    pub LUT619L: RWRegister<u32>,

    /// Graphic MMU LUT entry 619 high
    pub LUT619H: RWRegister<u32>,

    /// Graphic MMU LUT entry 620 low
    pub LUT620L: RWRegister<u32>,

    /// Graphic MMU LUT entry 620 high
    pub LUT620H: RWRegister<u32>,

    /// Graphic MMU LUT entry 621 low
    pub LUT621L: RWRegister<u32>,

    /// Graphic MMU LUT entry 621 high
    pub LUT621H: RWRegister<u32>,

    /// Graphic MMU LUT entry 622 low
    pub LUT622L: RWRegister<u32>,

    /// Graphic MMU LUT entry 622 high
    pub LUT622H: RWRegister<u32>,

    /// Graphic MMU LUT entry 623 low
    pub LUT623L: RWRegister<u32>,

    /// Graphic MMU LUT entry 623 high
    pub LUT623H: RWRegister<u32>,

    /// Graphic MMU LUT entry 624 low
    pub LUT624L: RWRegister<u32>,

    /// Graphic MMU LUT entry 624 high
    pub LUT624H: RWRegister<u32>,

    /// Graphic MMU LUT entry 625 low
    pub LUT625L: RWRegister<u32>,

    /// Graphic MMU LUT entry 625 high
    pub LUT625H: RWRegister<u32>,

    /// Graphic MMU LUT entry 626 low
    pub LUT626L: RWRegister<u32>,

    /// Graphic MMU LUT entry 626 high
    pub LUT626H: RWRegister<u32>,

    /// Graphic MMU LUT entry 627 low
    pub LUT627L: RWRegister<u32>,

    /// Graphic MMU LUT entry 627 high
    pub LUT627H: RWRegister<u32>,

    /// Graphic MMU LUT entry 628 low
    pub LUT628L: RWRegister<u32>,

    /// Graphic MMU LUT entry 628 high
    pub LUT628H: RWRegister<u32>,

    /// Graphic MMU LUT entry 629 low
    pub LUT629L: RWRegister<u32>,

    /// Graphic MMU LUT entry 629 high
    pub LUT629H: RWRegister<u32>,

    /// Graphic MMU LUT entry 630 low
    pub LUT630L: RWRegister<u32>,

    /// Graphic MMU LUT entry 630 high
    pub LUT630H: RWRegister<u32>,

    /// Graphic MMU LUT entry 631 low
    pub LUT631L: RWRegister<u32>,

    /// Graphic MMU LUT entry 631 high
    pub LUT631H: RWRegister<u32>,

    /// Graphic MMU LUT entry 632 low
    pub LUT632L: RWRegister<u32>,

    /// Graphic MMU LUT entry 632 high
    pub LUT632H: RWRegister<u32>,

    /// Graphic MMU LUT entry 633 low
    pub LUT633L: RWRegister<u32>,

    /// Graphic MMU LUT entry 633 high
    pub LUT633H: RWRegister<u32>,

    /// Graphic MMU LUT entry 634 low
    pub LUT634L: RWRegister<u32>,

    /// Graphic MMU LUT entry 634 high
    pub LUT634H: RWRegister<u32>,

    /// Graphic MMU LUT entry 635 low
    pub LUT635L: RWRegister<u32>,

    /// Graphic MMU LUT entry 635 high
    pub LUT635H: RWRegister<u32>,

    /// Graphic MMU LUT entry 636 low
    pub LUT636L: RWRegister<u32>,

    /// Graphic MMU LUT entry 636 high
    pub LUT636H: RWRegister<u32>,

    /// Graphic MMU LUT entry 637 low
    pub LUT637L: RWRegister<u32>,

    /// Graphic MMU LUT entry 637 high
    pub LUT637H: RWRegister<u32>,

    /// Graphic MMU LUT entry 638 low
    pub LUT638L: RWRegister<u32>,

    /// Graphic MMU LUT entry 638 high
    pub LUT638H: RWRegister<u32>,

    /// Graphic MMU LUT entry 639 low
    pub LUT639L: RWRegister<u32>,

    /// Graphic MMU LUT entry 639 high
    pub LUT639H: RWRegister<u32>,

    /// Graphic MMU LUT entry 640 low
    pub LUT640L: RWRegister<u32>,

    /// Graphic MMU LUT entry 640 high
    pub LUT640H: RWRegister<u32>,

    /// Graphic MMU LUT entry 641 low
    pub LUT641L: RWRegister<u32>,

    /// Graphic MMU LUT entry 641 high
    pub LUT641H: RWRegister<u32>,

    /// Graphic MMU LUT entry 642 low
    pub LUT642L: RWRegister<u32>,

    /// Graphic MMU LUT entry 642 high
    pub LUT642H: RWRegister<u32>,

    /// Graphic MMU LUT entry 643 low
    pub LUT643L: RWRegister<u32>,

    /// Graphic MMU LUT entry 643 high
    pub LUT643H: RWRegister<u32>,

    /// Graphic MMU LUT entry 644 low
    pub LUT644L: RWRegister<u32>,

    /// Graphic MMU LUT entry 644 high
    pub LUT644H: RWRegister<u32>,

    /// Graphic MMU LUT entry 645 low
    pub LUT645L: RWRegister<u32>,

    /// Graphic MMU LUT entry 645 high
    pub LUT645H: RWRegister<u32>,

    /// Graphic MMU LUT entry 646 low
    pub LUT646L: RWRegister<u32>,

    /// Graphic MMU LUT entry 646 high
    pub LUT646H: RWRegister<u32>,

    /// Graphic MMU LUT entry 647 low
    pub LUT647L: RWRegister<u32>,

    /// Graphic MMU LUT entry 647 high
    pub LUT647H: RWRegister<u32>,

    /// Graphic MMU LUT entry 648 low
    pub LUT648L: RWRegister<u32>,

    /// Graphic MMU LUT entry 648 high
    pub LUT648H: RWRegister<u32>,

    /// Graphic MMU LUT entry 649 low
    pub LUT649L: RWRegister<u32>,

    /// Graphic MMU LUT entry 649 high
    pub LUT649H: RWRegister<u32>,

    /// Graphic MMU LUT entry 650 low
    pub LUT650L: RWRegister<u32>,

    /// Graphic MMU LUT entry 650 high
    pub LUT650H: RWRegister<u32>,

    /// Graphic MMU LUT entry 651 low
    pub LUT651L: RWRegister<u32>,

    /// Graphic MMU LUT entry 651 high
    pub LUT651H: RWRegister<u32>,

    /// Graphic MMU LUT entry 652 low
    pub LUT652L: RWRegister<u32>,

    /// Graphic MMU LUT entry 652 high
    pub LUT652H: RWRegister<u32>,

    /// Graphic MMU LUT entry 653 low
    pub LUT653L: RWRegister<u32>,

    /// Graphic MMU LUT entry 653 high
    pub LUT653H: RWRegister<u32>,

    /// Graphic MMU LUT entry 654 low
    pub LUT654L: RWRegister<u32>,

    /// Graphic MMU LUT entry 654 high
    pub LUT654H: RWRegister<u32>,

    /// Graphic MMU LUT entry 655 low
    pub LUT655L: RWRegister<u32>,

    /// Graphic MMU LUT entry 655 high
    pub LUT655H: RWRegister<u32>,

    /// Graphic MMU LUT entry 656 low
    pub LUT656L: RWRegister<u32>,

    /// Graphic MMU LUT entry 656 high
    pub LUT656H: RWRegister<u32>,

    /// Graphic MMU LUT entry 657 low
    pub LUT657L: RWRegister<u32>,

    /// Graphic MMU LUT entry 657 high
    pub LUT657H: RWRegister<u32>,

    /// Graphic MMU LUT entry 658 low
    pub LUT658L: RWRegister<u32>,

    /// Graphic MMU LUT entry 658 high
    pub LUT658H: RWRegister<u32>,

    /// Graphic MMU LUT entry 659 low
    pub LUT659L: RWRegister<u32>,

    /// Graphic MMU LUT entry 659 high
    pub LUT659H: RWRegister<u32>,

    /// Graphic MMU LUT entry 660 low
    pub LUT660L: RWRegister<u32>,

    /// Graphic MMU LUT entry 660 high
    pub LUT660H: RWRegister<u32>,

    /// Graphic MMU LUT entry 661 low
    pub LUT661L: RWRegister<u32>,

    /// Graphic MMU LUT entry 661 high
    pub LUT661H: RWRegister<u32>,

    /// Graphic MMU LUT entry 662 low
    pub LUT662L: RWRegister<u32>,

    /// Graphic MMU LUT entry 662 high
    pub LUT662H: RWRegister<u32>,

    /// Graphic MMU LUT entry 663 low
    pub LUT663L: RWRegister<u32>,

    /// Graphic MMU LUT entry 663 high
    pub LUT663H: RWRegister<u32>,

    /// Graphic MMU LUT entry 664 low
    pub LUT664L: RWRegister<u32>,

    /// Graphic MMU LUT entry 664 high
    pub LUT664H: RWRegister<u32>,

    /// Graphic MMU LUT entry 665 low
    pub LUT665L: RWRegister<u32>,

    /// Graphic MMU LUT entry 665 high
    pub LUT665H: RWRegister<u32>,

    /// Graphic MMU LUT entry 666 low
    pub LUT666L: RWRegister<u32>,

    /// Graphic MMU LUT entry 666 high
    pub LUT666H: RWRegister<u32>,

    /// Graphic MMU LUT entry 667 low
    pub LUT667L: RWRegister<u32>,

    /// Graphic MMU LUT entry 667 high
    pub LUT667H: RWRegister<u32>,

    /// Graphic MMU LUT entry 668 low
    pub LUT668L: RWRegister<u32>,

    /// Graphic MMU LUT entry 668 high
    pub LUT668H: RWRegister<u32>,

    /// Graphic MMU LUT entry 669 low
    pub LUT669L: RWRegister<u32>,

    /// Graphic MMU LUT entry 669 high
    pub LUT669H: RWRegister<u32>,

    /// Graphic MMU LUT entry 670 low
    pub LUT670L: RWRegister<u32>,

    /// Graphic MMU LUT entry 670 high
    pub LUT670H: RWRegister<u32>,

    /// Graphic MMU LUT entry 671 low
    pub LUT671L: RWRegister<u32>,

    /// Graphic MMU LUT entry 671 high
    pub LUT671H: RWRegister<u32>,

    /// Graphic MMU LUT entry 672 low
    pub LUT672L: RWRegister<u32>,

    /// Graphic MMU LUT entry 672 high
    pub LUT672H: RWRegister<u32>,

    /// Graphic MMU LUT entry 673 low
    pub LUT673L: RWRegister<u32>,

    /// Graphic MMU LUT entry 673 high
    pub LUT673H: RWRegister<u32>,

    /// Graphic MMU LUT entry 674 low
    pub LUT674L: RWRegister<u32>,

    /// Graphic MMU LUT entry 674 high
    pub LUT674H: RWRegister<u32>,

    /// Graphic MMU LUT entry 675 low
    pub LUT675L: RWRegister<u32>,

    /// Graphic MMU LUT entry 675 high
    pub LUT675H: RWRegister<u32>,

    /// Graphic MMU LUT entry 676 low
    pub LUT676L: RWRegister<u32>,

    /// Graphic MMU LUT entry 676 high
    pub LUT676H: RWRegister<u32>,

    /// Graphic MMU LUT entry 677 low
    pub LUT677L: RWRegister<u32>,

    /// Graphic MMU LUT entry 677 high
    pub LUT677H: RWRegister<u32>,

    /// Graphic MMU LUT entry 678 low
    pub LUT678L: RWRegister<u32>,

    /// Graphic MMU LUT entry 678 high
    pub LUT678H: RWRegister<u32>,

    /// Graphic MMU LUT entry 679 low
    pub LUT679L: RWRegister<u32>,

    /// Graphic MMU LUT entry 679 high
    pub LUT679H: RWRegister<u32>,

    /// Graphic MMU LUT entry 680 low
    pub LUT680L: RWRegister<u32>,

    /// Graphic MMU LUT entry 680 high
    pub LUT680H: RWRegister<u32>,

    /// Graphic MMU LUT entry 681 low
    pub LUT681L: RWRegister<u32>,

    /// Graphic MMU LUT entry 681 high
    pub LUT681H: RWRegister<u32>,

    /// Graphic MMU LUT entry 682 low
    pub LUT682L: RWRegister<u32>,

    /// Graphic MMU LUT entry 682 high
    pub LUT682H: RWRegister<u32>,

    /// Graphic MMU LUT entry 683 low
    pub LUT683L: RWRegister<u32>,

    /// Graphic MMU LUT entry 683 high
    pub LUT683H: RWRegister<u32>,

    /// Graphic MMU LUT entry 684 low
    pub LUT684L: RWRegister<u32>,

    /// Graphic MMU LUT entry 684 high
    pub LUT684H: RWRegister<u32>,

    /// Graphic MMU LUT entry 685 low
    pub LUT685L: RWRegister<u32>,

    /// Graphic MMU LUT entry 685 high
    pub LUT685H: RWRegister<u32>,

    /// Graphic MMU LUT entry 686 low
    pub LUT686L: RWRegister<u32>,

    /// Graphic MMU LUT entry 686 high
    pub LUT686H: RWRegister<u32>,

    /// Graphic MMU LUT entry 687 low
    pub LUT687L: RWRegister<u32>,

    /// Graphic MMU LUT entry 687 high
    pub LUT687H: RWRegister<u32>,

    /// Graphic MMU LUT entry 688 low
    pub LUT688L: RWRegister<u32>,

    /// Graphic MMU LUT entry 688 high
    pub LUT688H: RWRegister<u32>,

    /// Graphic MMU LUT entry 689 low
    pub LUT689L: RWRegister<u32>,

    /// Graphic MMU LUT entry 689 high
    pub LUT689H: RWRegister<u32>,

    /// Graphic MMU LUT entry 690 low
    pub LUT690L: RWRegister<u32>,

    /// Graphic MMU LUT entry 690 high
    pub LUT690H: RWRegister<u32>,

    /// Graphic MMU LUT entry 691 low
    pub LUT691L: RWRegister<u32>,

    /// Graphic MMU LUT entry 691 high
    pub LUT691H: RWRegister<u32>,

    /// Graphic MMU LUT entry 692 low
    pub LUT692L: RWRegister<u32>,

    /// Graphic MMU LUT entry 692 high
    pub LUT692H: RWRegister<u32>,

    /// Graphic MMU LUT entry 693 low
    pub LUT693L: RWRegister<u32>,

    /// Graphic MMU LUT entry 693 high
    pub LUT693H: RWRegister<u32>,

    /// Graphic MMU LUT entry 694 low
    pub LUT694L: RWRegister<u32>,

    /// Graphic MMU LUT entry 694 high
    pub LUT694H: RWRegister<u32>,

    /// Graphic MMU LUT entry 695 low
    pub LUT695L: RWRegister<u32>,

    /// Graphic MMU LUT entry 695 high
    pub LUT695H: RWRegister<u32>,

    /// Graphic MMU LUT entry 696 low
    pub LUT696L: RWRegister<u32>,

    /// Graphic MMU LUT entry 696 high
    pub LUT696H: RWRegister<u32>,

    /// Graphic MMU LUT entry 697 low
    pub LUT697L: RWRegister<u32>,

    /// Graphic MMU LUT entry 697 high
    pub LUT697H: RWRegister<u32>,

    /// Graphic MMU LUT entry 698 low
    pub LUT698L: RWRegister<u32>,

    /// Graphic MMU LUT entry 698 high
    pub LUT698H: RWRegister<u32>,

    /// Graphic MMU LUT entry 699 low
    pub LUT699L: RWRegister<u32>,

    /// Graphic MMU LUT entry 699 high
    pub LUT699H: RWRegister<u32>,

    /// Graphic MMU LUT entry 700 low
    pub LUT700L: RWRegister<u32>,

    /// Graphic MMU LUT entry 700 high
    pub LUT700H: RWRegister<u32>,

    /// Graphic MMU LUT entry 701 low
    pub LUT701L: RWRegister<u32>,

    /// Graphic MMU LUT entry 701 high
    pub LUT701H: RWRegister<u32>,

    /// Graphic MMU LUT entry 702 low
    pub LUT702L: RWRegister<u32>,

    /// Graphic MMU LUT entry 702 high
    pub LUT702H: RWRegister<u32>,

    /// Graphic MMU LUT entry 703 low
    pub LUT703L: RWRegister<u32>,

    /// Graphic MMU LUT entry 703 high
    pub LUT703H: RWRegister<u32>,

    /// Graphic MMU LUT entry 704 low
    pub LUT704L: RWRegister<u32>,

    /// Graphic MMU LUT entry 704 high
    pub LUT704H: RWRegister<u32>,

    /// Graphic MMU LUT entry 705 low
    pub LUT705L: RWRegister<u32>,

    /// Graphic MMU LUT entry 705 high
    pub LUT705H: RWRegister<u32>,

    /// Graphic MMU LUT entry 706 low
    pub LUT706L: RWRegister<u32>,

    /// Graphic MMU LUT entry 706 high
    pub LUT706H: RWRegister<u32>,

    /// Graphic MMU LUT entry 707 low
    pub LUT707L: RWRegister<u32>,

    /// Graphic MMU LUT entry 707 high
    pub LUT707H: RWRegister<u32>,

    /// Graphic MMU LUT entry 708 low
    pub LUT708L: RWRegister<u32>,

    /// Graphic MMU LUT entry 708 high
    pub LUT708H: RWRegister<u32>,

    /// Graphic MMU LUT entry 709 low
    pub LUT709L: RWRegister<u32>,

    /// Graphic MMU LUT entry 709 high
    pub LUT709H: RWRegister<u32>,

    /// Graphic MMU LUT entry 710 low
    pub LUT710L: RWRegister<u32>,

    /// Graphic MMU LUT entry 710 high
    pub LUT710H: RWRegister<u32>,

    /// Graphic MMU LUT entry 711 low
    pub LUT711L: RWRegister<u32>,

    /// Graphic MMU LUT entry 711 high
    pub LUT711H: RWRegister<u32>,

    /// Graphic MMU LUT entry 712 low
    pub LUT712L: RWRegister<u32>,

    /// Graphic MMU LUT entry 712 high
    pub LUT712H: RWRegister<u32>,

    /// Graphic MMU LUT entry 713 low
    pub LUT713L: RWRegister<u32>,

    /// Graphic MMU LUT entry 713 high
    pub LUT713H: RWRegister<u32>,

    /// Graphic MMU LUT entry 714 low
    pub LUT714L: RWRegister<u32>,

    /// Graphic MMU LUT entry 714 high
    pub LUT714H: RWRegister<u32>,

    /// Graphic MMU LUT entry 715 low
    pub LUT715L: RWRegister<u32>,

    /// Graphic MMU LUT entry 715 high
    pub LUT715H: RWRegister<u32>,

    /// Graphic MMU LUT entry 716 low
    pub LUT716L: RWRegister<u32>,

    /// Graphic MMU LUT entry 716 high
    pub LUT716H: RWRegister<u32>,

    /// Graphic MMU LUT entry 717 low
    pub LUT717L: RWRegister<u32>,

    /// Graphic MMU LUT entry 717 high
    pub LUT717H: RWRegister<u32>,

    /// Graphic MMU LUT entry 718 low
    pub LUT718L: RWRegister<u32>,

    /// Graphic MMU LUT entry 718 high
    pub LUT718H: RWRegister<u32>,

    /// Graphic MMU LUT entry 719 low
    pub LUT719L: RWRegister<u32>,

    /// Graphic MMU LUT entry 719 high
    pub LUT719H: RWRegister<u32>,

    /// Graphic MMU LUT entry 720 low
    pub LUT720L: RWRegister<u32>,

    /// Graphic MMU LUT entry 720 high
    pub LUT720H: RWRegister<u32>,

    /// Graphic MMU LUT entry 721 low
    pub LUT721L: RWRegister<u32>,

    /// Graphic MMU LUT entry 721 high
    pub LUT721H: RWRegister<u32>,

    /// Graphic MMU LUT entry 722 low
    pub LUT722L: RWRegister<u32>,

    /// Graphic MMU LUT entry 722 high
    pub LUT722H: RWRegister<u32>,

    /// Graphic MMU LUT entry 723 low
    pub LUT723L: RWRegister<u32>,

    /// Graphic MMU LUT entry 723 high
    pub LUT723H: RWRegister<u32>,

    /// Graphic MMU LUT entry 724 low
    pub LUT724L: RWRegister<u32>,

    /// Graphic MMU LUT entry 724 high
    pub LUT724H: RWRegister<u32>,

    /// Graphic MMU LUT entry 725 low
    pub LUT725L: RWRegister<u32>,

    /// Graphic MMU LUT entry 725 high
    pub LUT725H: RWRegister<u32>,

    /// Graphic MMU LUT entry 726 low
    pub LUT726L: RWRegister<u32>,

    /// Graphic MMU LUT entry 726 high
    pub LUT726H: RWRegister<u32>,

    /// Graphic MMU LUT entry 727 low
    pub LUT727L: RWRegister<u32>,

    /// Graphic MMU LUT entry 727 high
    pub LUT727H: RWRegister<u32>,

    /// Graphic MMU LUT entry 728 low
    pub LUT728L: RWRegister<u32>,

    /// Graphic MMU LUT entry 728 high
    pub LUT728H: RWRegister<u32>,

    /// Graphic MMU LUT entry 729 low
    pub LUT729L: RWRegister<u32>,

    /// Graphic MMU LUT entry 729 high
    pub LUT729H: RWRegister<u32>,

    /// Graphic MMU LUT entry 730 low
    pub LUT730L: RWRegister<u32>,

    /// Graphic MMU LUT entry 730 high
    pub LUT730H: RWRegister<u32>,

    /// Graphic MMU LUT entry 731 low
    pub LUT731L: RWRegister<u32>,

    /// Graphic MMU LUT entry 731 high
    pub LUT731H: RWRegister<u32>,

    /// Graphic MMU LUT entry 732 low
    pub LUT732L: RWRegister<u32>,

    /// Graphic MMU LUT entry 732 high
    pub LUT732H: RWRegister<u32>,

    /// Graphic MMU LUT entry 733 low
    pub LUT733L: RWRegister<u32>,

    /// Graphic MMU LUT entry 733 high
    pub LUT733H: RWRegister<u32>,

    /// Graphic MMU LUT entry 734 low
    pub LUT734L: RWRegister<u32>,

    /// Graphic MMU LUT entry 734 high
    pub LUT734H: RWRegister<u32>,

    /// Graphic MMU LUT entry 735 low
    pub LUT735L: RWRegister<u32>,

    /// Graphic MMU LUT entry 735 high
    pub LUT735H: RWRegister<u32>,

    /// Graphic MMU LUT entry 736 low
    pub LUT736L: RWRegister<u32>,

    /// Graphic MMU LUT entry 736 high
    pub LUT736H: RWRegister<u32>,

    /// Graphic MMU LUT entry 737 low
    pub LUT737L: RWRegister<u32>,

    /// Graphic MMU LUT entry 737 high
    pub LUT737H: RWRegister<u32>,

    /// Graphic MMU LUT entry 738 low
    pub LUT738L: RWRegister<u32>,

    /// Graphic MMU LUT entry 738 high
    pub LUT738H: RWRegister<u32>,

    /// Graphic MMU LUT entry 739 low
    pub LUT739L: RWRegister<u32>,

    /// Graphic MMU LUT entry 739 high
    pub LUT739H: RWRegister<u32>,

    /// Graphic MMU LUT entry 740 low
    pub LUT740L: RWRegister<u32>,

    /// Graphic MMU LUT entry 740 high
    pub LUT740H: RWRegister<u32>,

    /// Graphic MMU LUT entry 741 low
    pub LUT741L: RWRegister<u32>,

    /// Graphic MMU LUT entry 741 high
    pub LUT741H: RWRegister<u32>,

    /// Graphic MMU LUT entry 742 low
    pub LUT742L: RWRegister<u32>,

    /// Graphic MMU LUT entry 742 high
    pub LUT742H: RWRegister<u32>,

    /// Graphic MMU LUT entry 743 low
    pub LUT743L: RWRegister<u32>,

    /// Graphic MMU LUT entry 743 high
    pub LUT743H: RWRegister<u32>,

    /// Graphic MMU LUT entry 744 low
    pub LUT744L: RWRegister<u32>,

    /// Graphic MMU LUT entry 744 high
    pub LUT744H: RWRegister<u32>,

    /// Graphic MMU LUT entry 745 low
    pub LUT745L: RWRegister<u32>,

    /// Graphic MMU LUT entry 745 high
    pub LUT745H: RWRegister<u32>,

    /// Graphic MMU LUT entry 746 low
    pub LUT746L: RWRegister<u32>,

    /// Graphic MMU LUT entry 746 high
    pub LUT746H: RWRegister<u32>,

    /// Graphic MMU LUT entry 747 low
    pub LUT747L: RWRegister<u32>,

    /// Graphic MMU LUT entry 747 high
    pub LUT747H: RWRegister<u32>,

    /// Graphic MMU LUT entry 748 low
    pub LUT748L: RWRegister<u32>,

    /// Graphic MMU LUT entry 748 high
    pub LUT748H: RWRegister<u32>,

    /// Graphic MMU LUT entry 749 low
    pub LUT749L: RWRegister<u32>,

    /// Graphic MMU LUT entry 749 high
    pub LUT749H: RWRegister<u32>,

    /// Graphic MMU LUT entry 750 low
    pub LUT750L: RWRegister<u32>,

    /// Graphic MMU LUT entry 750 high
    pub LUT750H: RWRegister<u32>,

    /// Graphic MMU LUT entry 751 low
    pub LUT751L: RWRegister<u32>,

    /// Graphic MMU LUT entry 751 high
    pub LUT751H: RWRegister<u32>,

    /// Graphic MMU LUT entry 752 low
    pub LUT752L: RWRegister<u32>,

    /// Graphic MMU LUT entry 752 high
    pub LUT752H: RWRegister<u32>,

    /// Graphic MMU LUT entry 753 low
    pub LUT753L: RWRegister<u32>,

    /// Graphic MMU LUT entry 753 high
    pub LUT753H: RWRegister<u32>,

    /// Graphic MMU LUT entry 754 low
    pub LUT754L: RWRegister<u32>,

    /// Graphic MMU LUT entry 754 high
    pub LUT754H: RWRegister<u32>,

    /// Graphic MMU LUT entry 755 low
    pub LUT755L: RWRegister<u32>,

    /// Graphic MMU LUT entry 755 high
    pub LUT755H: RWRegister<u32>,

    /// Graphic MMU LUT entry 756 low
    pub LUT756L: RWRegister<u32>,

    /// Graphic MMU LUT entry 756 high
    pub LUT756H: RWRegister<u32>,

    /// Graphic MMU LUT entry 757 low
    pub LUT757L: RWRegister<u32>,

    /// Graphic MMU LUT entry 757 high
    pub LUT757H: RWRegister<u32>,

    /// Graphic MMU LUT entry 758 low
    pub LUT758L: RWRegister<u32>,

    /// Graphic MMU LUT entry 758 high
    pub LUT758H: RWRegister<u32>,

    /// Graphic MMU LUT entry 759 low
    pub LUT759L: RWRegister<u32>,

    /// Graphic MMU LUT entry 759 high
    pub LUT759H: RWRegister<u32>,

    /// Graphic MMU LUT entry 760 low
    pub LUT760L: RWRegister<u32>,

    /// Graphic MMU LUT entry 760 high
    pub LUT760H: RWRegister<u32>,

    /// Graphic MMU LUT entry 761 low
    pub LUT761L: RWRegister<u32>,

    /// Graphic MMU LUT entry 761 high
    pub LUT761H: RWRegister<u32>,

    /// Graphic MMU LUT entry 762 low
    pub LUT762L: RWRegister<u32>,

    /// Graphic MMU LUT entry 762 high
    pub LUT762H: RWRegister<u32>,

    /// Graphic MMU LUT entry 763 low
    pub LUT763L: RWRegister<u32>,

    /// Graphic MMU LUT entry 763 high
    pub LUT763H: RWRegister<u32>,

    /// Graphic MMU LUT entry 764 low
    pub LUT764L: RWRegister<u32>,

    /// Graphic MMU LUT entry 764 high
    pub LUT764H: RWRegister<u32>,

    /// Graphic MMU LUT entry 765 low
    pub LUT765L: RWRegister<u32>,

    /// Graphic MMU LUT entry 765 high
    pub LUT765H: RWRegister<u32>,

    /// Graphic MMU LUT entry 766 low
    pub LUT766L: RWRegister<u32>,

    /// Graphic MMU LUT entry 766 high
    pub LUT766H: RWRegister<u32>,

    /// Graphic MMU LUT entry 767 low
    pub LUT767L: RWRegister<u32>,

    /// Graphic MMU LUT entry 767 high
    pub LUT767H: RWRegister<u32>,

    /// Graphic MMU LUT entry 768 low
    pub LUT768L: RWRegister<u32>,

    /// Graphic MMU LUT entry 768 high
    pub LUT768H: RWRegister<u32>,

    /// Graphic MMU LUT entry 769 low
    pub LUT769L: RWRegister<u32>,

    /// Graphic MMU LUT entry 769 high
    pub LUT769H: RWRegister<u32>,

    /// Graphic MMU LUT entry 770 low
    pub LUT770L: RWRegister<u32>,

    /// Graphic MMU LUT entry 770 high
    pub LUT770H: RWRegister<u32>,

    /// Graphic MMU LUT entry 771 low
    pub LUT771L: RWRegister<u32>,

    /// Graphic MMU LUT entry 771 high
    pub LUT771H: RWRegister<u32>,

    /// Graphic MMU LUT entry 772 low
    pub LUT772L: RWRegister<u32>,

    /// Graphic MMU LUT entry 772 high
    pub LUT772H: RWRegister<u32>,

    /// Graphic MMU LUT entry 773 low
    pub LUT773L: RWRegister<u32>,

    /// Graphic MMU LUT entry 773 high
    pub LUT773H: RWRegister<u32>,

    /// Graphic MMU LUT entry 774 low
    pub LUT774L: RWRegister<u32>,

    /// Graphic MMU LUT entry 774 high
    pub LUT774H: RWRegister<u32>,

    /// Graphic MMU LUT entry 775 low
    pub LUT775L: RWRegister<u32>,

    /// Graphic MMU LUT entry 775 high
    pub LUT775H: RWRegister<u32>,

    /// Graphic MMU LUT entry 776 low
    pub LUT776L: RWRegister<u32>,

    /// Graphic MMU LUT entry 776 high
    pub LUT776H: RWRegister<u32>,

    /// Graphic MMU LUT entry 777 low
    pub LUT777L: RWRegister<u32>,

    /// Graphic MMU LUT entry 777 high
    pub LUT777H: RWRegister<u32>,

    /// Graphic MMU LUT entry 778 low
    pub LUT778L: RWRegister<u32>,

    /// Graphic MMU LUT entry 778 high
    pub LUT778H: RWRegister<u32>,

    /// Graphic MMU LUT entry 779 low
    pub LUT779L: RWRegister<u32>,

    /// Graphic MMU LUT entry 779 high
    pub LUT779H: RWRegister<u32>,

    /// Graphic MMU LUT entry 780 low
    pub LUT780L: RWRegister<u32>,

    /// Graphic MMU LUT entry 780 high
    pub LUT780H: RWRegister<u32>,

    /// Graphic MMU LUT entry 781 low
    pub LUT781L: RWRegister<u32>,

    /// Graphic MMU LUT entry 781 high
    pub LUT781H: RWRegister<u32>,

    /// Graphic MMU LUT entry 782 low
    pub LUT782L: RWRegister<u32>,

    /// Graphic MMU LUT entry 782 high
    pub LUT782H: RWRegister<u32>,

    /// Graphic MMU LUT entry 783 low
    pub LUT783L: RWRegister<u32>,

    /// Graphic MMU LUT entry 783 high
    pub LUT783H: RWRegister<u32>,

    /// Graphic MMU LUT entry 784 low
    pub LUT784L: RWRegister<u32>,

    /// Graphic MMU LUT entry 784 high
    pub LUT784H: RWRegister<u32>,

    /// Graphic MMU LUT entry 785 low
    pub LUT785L: RWRegister<u32>,

    /// Graphic MMU LUT entry 785 high
    pub LUT785H: RWRegister<u32>,

    /// Graphic MMU LUT entry 786 low
    pub LUT786L: RWRegister<u32>,

    /// Graphic MMU LUT entry 786 high
    pub LUT786H: RWRegister<u32>,

    /// Graphic MMU LUT entry 787 low
    pub LUT787L: RWRegister<u32>,

    /// Graphic MMU LUT entry 787 high
    pub LUT787H: RWRegister<u32>,

    /// Graphic MMU LUT entry 788 low
    pub LUT788L: RWRegister<u32>,

    /// Graphic MMU LUT entry 788 high
    pub LUT788H: RWRegister<u32>,

    /// Graphic MMU LUT entry 789 low
    pub LUT789L: RWRegister<u32>,

    /// Graphic MMU LUT entry 789 high
    pub LUT789H: RWRegister<u32>,

    /// Graphic MMU LUT entry 790 low
    pub LUT790L: RWRegister<u32>,

    /// Graphic MMU LUT entry 790 high
    pub LUT790H: RWRegister<u32>,

    /// Graphic MMU LUT entry 791 low
    pub LUT791L: RWRegister<u32>,

    /// Graphic MMU LUT entry 791 high
    pub LUT791H: RWRegister<u32>,

    /// Graphic MMU LUT entry 792 low
    pub LUT792L: RWRegister<u32>,

    /// Graphic MMU LUT entry 792 high
    pub LUT792H: RWRegister<u32>,

    /// Graphic MMU LUT entry 793 low
    pub LUT793L: RWRegister<u32>,

    /// Graphic MMU LUT entry 793 high
    pub LUT793H: RWRegister<u32>,

    /// Graphic MMU LUT entry 794 low
    pub LUT794L: RWRegister<u32>,

    /// Graphic MMU LUT entry 794 high
    pub LUT794H: RWRegister<u32>,

    /// Graphic MMU LUT entry 795 low
    pub LUT795L: RWRegister<u32>,

    /// Graphic MMU LUT entry 795 high
    pub LUT795H: RWRegister<u32>,

    /// Graphic MMU LUT entry 796 low
    pub LUT796L: RWRegister<u32>,

    /// Graphic MMU LUT entry 796 high
    pub LUT796H: RWRegister<u32>,

    /// Graphic MMU LUT entry 797 low
    pub LUT797L: RWRegister<u32>,

    /// Graphic MMU LUT entry 797 high
    pub LUT797H: RWRegister<u32>,

    /// Graphic MMU LUT entry 798 low
    pub LUT798L: RWRegister<u32>,

    /// Graphic MMU LUT entry 798 high
    pub LUT798H: RWRegister<u32>,

    /// Graphic MMU LUT entry 799 low
    pub LUT799L: RWRegister<u32>,

    /// Graphic MMU LUT entry 799 high
    pub LUT799H: RWRegister<u32>,

    /// Graphic MMU LUT entry 800 low
    pub LUT800L: RWRegister<u32>,

    /// Graphic MMU LUT entry 800 high
    pub LUT800H: RWRegister<u32>,

    /// Graphic MMU LUT entry 801 low
    pub LUT801L: RWRegister<u32>,

    /// Graphic MMU LUT entry 801 high
    pub LUT801H: RWRegister<u32>,

    /// Graphic MMU LUT entry 802 low
    pub LUT802L: RWRegister<u32>,

    /// Graphic MMU LUT entry 802 high
    pub LUT802H: RWRegister<u32>,

    /// Graphic MMU LUT entry 803 low
    pub LUT803L: RWRegister<u32>,

    /// Graphic MMU LUT entry 803 high
    pub LUT803H: RWRegister<u32>,

    /// Graphic MMU LUT entry 804 low
    pub LUT804L: RWRegister<u32>,

    /// Graphic MMU LUT entry 804 high
    pub LUT804H: RWRegister<u32>,

    /// Graphic MMU LUT entry 805 low
    pub LUT805L: RWRegister<u32>,

    /// Graphic MMU LUT entry 805 high
    pub LUT805H: RWRegister<u32>,

    /// Graphic MMU LUT entry 806 low
    pub LUT806L: RWRegister<u32>,

    /// Graphic MMU LUT entry 806 high
    pub LUT806H: RWRegister<u32>,

    /// Graphic MMU LUT entry 807 low
    pub LUT807L: RWRegister<u32>,

    /// Graphic MMU LUT entry 807 high
    pub LUT807H: RWRegister<u32>,

    /// Graphic MMU LUT entry 808 low
    pub LUT808L: RWRegister<u32>,

    /// Graphic MMU LUT entry 808 high
    pub LUT808H: RWRegister<u32>,

    /// Graphic MMU LUT entry 809 low
    pub LUT809L: RWRegister<u32>,

    /// Graphic MMU LUT entry 809 high
    pub LUT809H: RWRegister<u32>,

    /// Graphic MMU LUT entry 810 low
    pub LUT810L: RWRegister<u32>,

    /// Graphic MMU LUT entry 810 high
    pub LUT810H: RWRegister<u32>,

    /// Graphic MMU LUT entry 811 low
    pub LUT811L: RWRegister<u32>,

    /// Graphic MMU LUT entry 811 high
    pub LUT811H: RWRegister<u32>,

    /// Graphic MMU LUT entry 812 low
    pub LUT812L: RWRegister<u32>,

    /// Graphic MMU LUT entry 812 high
    pub LUT812H: RWRegister<u32>,

    /// Graphic MMU LUT entry 813 low
    pub LUT813L: RWRegister<u32>,

    /// Graphic MMU LUT entry 813 high
    pub LUT813H: RWRegister<u32>,

    /// Graphic MMU LUT entry 814 low
    pub LUT814L: RWRegister<u32>,

    /// Graphic MMU LUT entry 814 high
    pub LUT814H: RWRegister<u32>,

    /// Graphic MMU LUT entry 815 low
    pub LUT815L: RWRegister<u32>,

    /// Graphic MMU LUT entry 815 high
    pub LUT815H: RWRegister<u32>,

    /// Graphic MMU LUT entry 816 low
    pub LUT816L: RWRegister<u32>,

    /// Graphic MMU LUT entry 816 high
    pub LUT816H: RWRegister<u32>,

    /// Graphic MMU LUT entry 817 low
    pub LUT817L: RWRegister<u32>,

    /// Graphic MMU LUT entry 817 high
    pub LUT817H: RWRegister<u32>,

    /// Graphic MMU LUT entry 818 low
    pub LUT818L: RWRegister<u32>,

    /// Graphic MMU LUT entry 818 high
    pub LUT818H: RWRegister<u32>,

    /// Graphic MMU LUT entry 819 low
    pub LUT819L: RWRegister<u32>,

    /// Graphic MMU LUT entry 819 high
    pub LUT819H: RWRegister<u32>,

    /// Graphic MMU LUT entry 820 low
    pub LUT820L: RWRegister<u32>,

    /// Graphic MMU LUT entry 820 high
    pub LUT820H: RWRegister<u32>,

    /// Graphic MMU LUT entry 821 low
    pub LUT821L: RWRegister<u32>,

    /// Graphic MMU LUT entry 821 high
    pub LUT821H: RWRegister<u32>,

    /// Graphic MMU LUT entry 822 low
    pub LUT822L: RWRegister<u32>,

    /// Graphic MMU LUT entry 822 high
    pub LUT822H: RWRegister<u32>,

    /// Graphic MMU LUT entry 823 low
    pub LUT823L: RWRegister<u32>,

    /// Graphic MMU LUT entry 823 high
    pub LUT823H: RWRegister<u32>,

    /// Graphic MMU LUT entry 824 low
    pub LUT824L: RWRegister<u32>,

    /// Graphic MMU LUT entry 824 high
    pub LUT824H: RWRegister<u32>,

    /// Graphic MMU LUT entry 825 low
    pub LUT825L: RWRegister<u32>,

    /// Graphic MMU LUT entry 825 high
    pub LUT825H: RWRegister<u32>,

    /// Graphic MMU LUT entry 826 low
    pub LUT826L: RWRegister<u32>,

    /// Graphic MMU LUT entry 826 high
    pub LUT826H: RWRegister<u32>,

    /// Graphic MMU LUT entry 827 low
    pub LUT827L: RWRegister<u32>,

    /// Graphic MMU LUT entry 827 high
    pub LUT827H: RWRegister<u32>,

    /// Graphic MMU LUT entry 828 low
    pub LUT828L: RWRegister<u32>,

    /// Graphic MMU LUT entry 828 high
    pub LUT828H: RWRegister<u32>,

    /// Graphic MMU LUT entry 829 low
    pub LUT829L: RWRegister<u32>,

    /// Graphic MMU LUT entry 829 high
    pub LUT829H: RWRegister<u32>,

    /// Graphic MMU LUT entry 830 low
    pub LUT830L: RWRegister<u32>,

    /// Graphic MMU LUT entry 830 high
    pub LUT830H: RWRegister<u32>,

    /// Graphic MMU LUT entry 831 low
    pub LUT831L: RWRegister<u32>,

    /// Graphic MMU LUT entry 831 high
    pub LUT831H: RWRegister<u32>,

    /// Graphic MMU LUT entry 832 low
    pub LUT832L: RWRegister<u32>,

    /// Graphic MMU LUT entry 832 high
    pub LUT832H: RWRegister<u32>,

    /// Graphic MMU LUT entry 833 low
    pub LUT833L: RWRegister<u32>,

    /// Graphic MMU LUT entry 833 high
    pub LUT833H: RWRegister<u32>,

    /// Graphic MMU LUT entry 834 low
    pub LUT834L: RWRegister<u32>,

    /// Graphic MMU LUT entry 834 high
    pub LUT834H: RWRegister<u32>,

    /// Graphic MMU LUT entry 835 low
    pub LUT835L: RWRegister<u32>,

    /// Graphic MMU LUT entry 835 high
    pub LUT835H: RWRegister<u32>,

    /// Graphic MMU LUT entry 836 low
    pub LUT836L: RWRegister<u32>,

    /// Graphic MMU LUT entry 836 high
    pub LUT836H: RWRegister<u32>,

    /// Graphic MMU LUT entry 837 low
    pub LUT837L: RWRegister<u32>,

    /// Graphic MMU LUT entry 837 high
    pub LUT837H: RWRegister<u32>,

    /// Graphic MMU LUT entry 838 low
    pub LUT838L: RWRegister<u32>,

    /// Graphic MMU LUT entry 838 high
    pub LUT838H: RWRegister<u32>,

    /// Graphic MMU LUT entry 839 low
    pub LUT839L: RWRegister<u32>,

    /// Graphic MMU LUT entry 839 high
    pub LUT839H: RWRegister<u32>,

    /// Graphic MMU LUT entry 840 low
    pub LUT840L: RWRegister<u32>,

    /// Graphic MMU LUT entry 840 high
    pub LUT840H: RWRegister<u32>,

    /// Graphic MMU LUT entry 841 low
    pub LUT841L: RWRegister<u32>,

    /// Graphic MMU LUT entry 841 high
    pub LUT841H: RWRegister<u32>,

    /// Graphic MMU LUT entry 842 low
    pub LUT842L: RWRegister<u32>,

    /// Graphic MMU LUT entry 842 high
    pub LUT842H: RWRegister<u32>,

    /// Graphic MMU LUT entry 843 low
    pub LUT843L: RWRegister<u32>,

    /// Graphic MMU LUT entry 843 high
    pub LUT843H: RWRegister<u32>,

    /// Graphic MMU LUT entry 844 low
    pub LUT844L: RWRegister<u32>,

    /// Graphic MMU LUT entry 844 high
    pub LUT844H: RWRegister<u32>,

    /// Graphic MMU LUT entry 845 low
    pub LUT845L: RWRegister<u32>,

    /// Graphic MMU LUT entry 845 high
    pub LUT845H: RWRegister<u32>,

    /// Graphic MMU LUT entry 846 low
    pub LUT846L: RWRegister<u32>,

    /// Graphic MMU LUT entry 846 high
    pub LUT846H: RWRegister<u32>,

    /// Graphic MMU LUT entry 847 low
    pub LUT847L: RWRegister<u32>,

    /// Graphic MMU LUT entry 847 high
    pub LUT847H: RWRegister<u32>,

    /// Graphic MMU LUT entry 848 low
    pub LUT848L: RWRegister<u32>,

    /// Graphic MMU LUT entry 848 high
    pub LUT848H: RWRegister<u32>,

    /// Graphic MMU LUT entry 849 low
    pub LUT849L: RWRegister<u32>,

    /// Graphic MMU LUT entry 849 high
    pub LUT849H: RWRegister<u32>,

    /// Graphic MMU LUT entry 850 low
    pub LUT850L: RWRegister<u32>,

    /// Graphic MMU LUT entry 850 high
    pub LUT850H: RWRegister<u32>,

    /// Graphic MMU LUT entry 851 low
    pub LUT851L: RWRegister<u32>,

    /// Graphic MMU LUT entry 851 high
    pub LUT851H: RWRegister<u32>,

    /// Graphic MMU LUT entry 852 low
    pub LUT852L: RWRegister<u32>,

    /// Graphic MMU LUT entry 852 high
    pub LUT852H: RWRegister<u32>,

    /// Graphic MMU LUT entry 853 low
    pub LUT853L: RWRegister<u32>,

    /// Graphic MMU LUT entry 853 high
    pub LUT853H: RWRegister<u32>,

    /// Graphic MMU LUT entry 854 low
    pub LUT854L: RWRegister<u32>,

    /// Graphic MMU LUT entry 854 high
    pub LUT854H: RWRegister<u32>,

    /// Graphic MMU LUT entry 855 low
    pub LUT855L: RWRegister<u32>,

    /// Graphic MMU LUT entry 855 high
    pub LUT855H: RWRegister<u32>,

    /// Graphic MMU LUT entry 856 low
    pub LUT856L: RWRegister<u32>,

    /// Graphic MMU LUT entry 856 high
    pub LUT856H: RWRegister<u32>,

    /// Graphic MMU LUT entry 857 low
    pub LUT857L: RWRegister<u32>,

    /// Graphic MMU LUT entry 857 high
    pub LUT857H: RWRegister<u32>,

    /// Graphic MMU LUT entry 858 low
    pub LUT858L: RWRegister<u32>,

    /// Graphic MMU LUT entry 858 high
    pub LUT858H: RWRegister<u32>,

    /// Graphic MMU LUT entry 859 low
    pub LUT859L: RWRegister<u32>,

    /// Graphic MMU LUT entry 859 high
    pub LUT859H: RWRegister<u32>,

    /// Graphic MMU LUT entry 860 low
    pub LUT860L: RWRegister<u32>,

    /// Graphic MMU LUT entry 860 high
    pub LUT860H: RWRegister<u32>,

    /// Graphic MMU LUT entry 861 low
    pub LUT861L: RWRegister<u32>,

    /// Graphic MMU LUT entry 861 high
    pub LUT861H: RWRegister<u32>,

    /// Graphic MMU LUT entry 862 low
    pub LUT862L: RWRegister<u32>,

    /// Graphic MMU LUT entry 862 high
    pub LUT862H: RWRegister<u32>,

    /// Graphic MMU LUT entry 863 low
    pub LUT863L: RWRegister<u32>,

    /// Graphic MMU LUT entry 863 high
    pub LUT863H: RWRegister<u32>,

    /// Graphic MMU LUT entry 864 low
    pub LUT864L: RWRegister<u32>,

    /// Graphic MMU LUT entry 864 high
    pub LUT864H: RWRegister<u32>,

    /// Graphic MMU LUT entry 865 low
    pub LUT865L: RWRegister<u32>,

    /// Graphic MMU LUT entry 865 high
    pub LUT865H: RWRegister<u32>,

    /// Graphic MMU LUT entry 866 low
    pub LUT866L: RWRegister<u32>,

    /// Graphic MMU LUT entry 866 high
    pub LUT866H: RWRegister<u32>,

    /// Graphic MMU LUT entry 867 low
    pub LUT867L: RWRegister<u32>,

    /// Graphic MMU LUT entry 867 high
    pub LUT867H: RWRegister<u32>,

    /// Graphic MMU LUT entry 868 low
    pub LUT868L: RWRegister<u32>,

    /// Graphic MMU LUT entry 868 high
    pub LUT868H: RWRegister<u32>,

    /// Graphic MMU LUT entry 869 low
    pub LUT869L: RWRegister<u32>,

    /// Graphic MMU LUT entry 869 high
    pub LUT869H: RWRegister<u32>,

    /// Graphic MMU LUT entry 870 low
    pub LUT870L: RWRegister<u32>,

    /// Graphic MMU LUT entry 870 high
    pub LUT870H: RWRegister<u32>,

    /// Graphic MMU LUT entry 871 low
    pub LUT871L: RWRegister<u32>,

    /// Graphic MMU LUT entry 871 high
    pub LUT871H: RWRegister<u32>,

    /// Graphic MMU LUT entry 872 low
    pub LUT872L: RWRegister<u32>,

    /// Graphic MMU LUT entry 872 high
    pub LUT872H: RWRegister<u32>,

    /// Graphic MMU LUT entry 873 low
    pub LUT873L: RWRegister<u32>,

    /// Graphic MMU LUT entry 873 high
    pub LUT873H: RWRegister<u32>,

    /// Graphic MMU LUT entry 874 low
    pub LUT874L: RWRegister<u32>,

    /// Graphic MMU LUT entry 874 high
    pub LUT874H: RWRegister<u32>,

    /// Graphic MMU LUT entry 875 low
    pub LUT875L: RWRegister<u32>,

    /// Graphic MMU LUT entry 875 high
    pub LUT875H: RWRegister<u32>,

    /// Graphic MMU LUT entry 876 low
    pub LUT876L: RWRegister<u32>,

    /// Graphic MMU LUT entry 876 high
    pub LUT876H: RWRegister<u32>,

    /// Graphic MMU LUT entry 877 low
    pub LUT877L: RWRegister<u32>,

    /// Graphic MMU LUT entry 877 high
    pub LUT877H: RWRegister<u32>,

    /// Graphic MMU LUT entry 878 low
    pub LUT878L: RWRegister<u32>,

    /// Graphic MMU LUT entry 878 high
    pub LUT878H: RWRegister<u32>,

    /// Graphic MMU LUT entry 879 low
    pub LUT879L: RWRegister<u32>,

    /// Graphic MMU LUT entry 879 high
    pub LUT879H: RWRegister<u32>,

    /// Graphic MMU LUT entry 880 low
    pub LUT880L: RWRegister<u32>,

    /// Graphic MMU LUT entry 880 high
    pub LUT880H: RWRegister<u32>,

    /// Graphic MMU LUT entry 881 low
    pub LUT881L: RWRegister<u32>,

    /// Graphic MMU LUT entry 881 high
    pub LUT881H: RWRegister<u32>,

    /// Graphic MMU LUT entry 882 low
    pub LUT882L: RWRegister<u32>,

    /// Graphic MMU LUT entry 882 high
    pub LUT882H: RWRegister<u32>,

    /// Graphic MMU LUT entry 883 low
    pub LUT883L: RWRegister<u32>,

    /// Graphic MMU LUT entry 883 high
    pub LUT883H: RWRegister<u32>,

    /// Graphic MMU LUT entry 884 low
    pub LUT884L: RWRegister<u32>,

    /// Graphic MMU LUT entry 884 high
    pub LUT884H: RWRegister<u32>,

    /// Graphic MMU LUT entry 885 low
    pub LUT885L: RWRegister<u32>,

    /// Graphic MMU LUT entry 885 high
    pub LUT885H: RWRegister<u32>,

    /// Graphic MMU LUT entry 886 low
    pub LUT886L: RWRegister<u32>,

    /// Graphic MMU LUT entry 886 high
    pub LUT886H: RWRegister<u32>,

    /// Graphic MMU LUT entry 887 low
    pub LUT887L: RWRegister<u32>,

    /// Graphic MMU LUT entry 887 high
    pub LUT887H: RWRegister<u32>,

    /// Graphic MMU LUT entry 888 low
    pub LUT888L: RWRegister<u32>,

    /// Graphic MMU LUT entry 888 high
    pub LUT888H: RWRegister<u32>,

    /// Graphic MMU LUT entry 889 low
    pub LUT889L: RWRegister<u32>,

    /// Graphic MMU LUT entry 889 high
    pub LUT889H: RWRegister<u32>,

    /// Graphic MMU LUT entry 890 low
    pub LUT890L: RWRegister<u32>,

    /// Graphic MMU LUT entry 890 high
    pub LUT890H: RWRegister<u32>,

    /// Graphic MMU LUT entry 891 low
    pub LUT891L: RWRegister<u32>,

    /// Graphic MMU LUT entry 891 high
    pub LUT891H: RWRegister<u32>,

    /// Graphic MMU LUT entry 892 low
    pub LUT892L: RWRegister<u32>,

    /// Graphic MMU LUT entry 892 high
    pub LUT892H: RWRegister<u32>,

    /// Graphic MMU LUT entry 893 low
    pub LUT893L: RWRegister<u32>,

    /// Graphic MMU LUT entry 893 high
    pub LUT893H: RWRegister<u32>,

    /// Graphic MMU LUT entry 894 low
    pub LUT894L: RWRegister<u32>,

    /// Graphic MMU LUT entry 894 high
    pub LUT894H: RWRegister<u32>,

    /// Graphic MMU LUT entry 895 low
    pub LUT895L: RWRegister<u32>,

    /// Graphic MMU LUT entry 895 high
    pub LUT895H: RWRegister<u32>,

    /// Graphic MMU LUT entry 896 low
    pub LUT896L: RWRegister<u32>,

    /// Graphic MMU LUT entry 896 high
    pub LUT896H: RWRegister<u32>,

    /// Graphic MMU LUT entry 897 low
    pub LUT897L: RWRegister<u32>,

    /// Graphic MMU LUT entry 897 high
    pub LUT897H: RWRegister<u32>,

    /// Graphic MMU LUT entry 898 low
    pub LUT898L: RWRegister<u32>,

    /// Graphic MMU LUT entry 898 high
    pub LUT898H: RWRegister<u32>,

    /// Graphic MMU LUT entry 899 low
    pub LUT899L: RWRegister<u32>,

    /// Graphic MMU LUT entry 899 high
    pub LUT899H: RWRegister<u32>,

    /// Graphic MMU LUT entry 900 low
    pub LUT900L: RWRegister<u32>,

    /// Graphic MMU LUT entry 900 high
    pub LUT900H: RWRegister<u32>,

    /// Graphic MMU LUT entry 901 low
    pub LUT901L: RWRegister<u32>,

    /// Graphic MMU LUT entry 901 high
    pub LUT901H: RWRegister<u32>,

    /// Graphic MMU LUT entry 902 low
    pub LUT902L: RWRegister<u32>,

    /// Graphic MMU LUT entry 902 high
    pub LUT902H: RWRegister<u32>,

    /// Graphic MMU LUT entry 903 low
    pub LUT903L: RWRegister<u32>,

    /// Graphic MMU LUT entry 903 high
    pub LUT903H: RWRegister<u32>,

    /// Graphic MMU LUT entry 904 low
    pub LUT904L: RWRegister<u32>,

    /// Graphic MMU LUT entry 904 high
    pub LUT904H: RWRegister<u32>,

    /// Graphic MMU LUT entry 905 low
    pub LUT905L: RWRegister<u32>,

    /// Graphic MMU LUT entry 905 high
    pub LUT905H: RWRegister<u32>,

    /// Graphic MMU LUT entry 906 low
    pub LUT906L: RWRegister<u32>,

    /// Graphic MMU LUT entry 906 high
    pub LUT906H: RWRegister<u32>,

    /// Graphic MMU LUT entry 907 low
    pub LUT907L: RWRegister<u32>,

    /// Graphic MMU LUT entry 907 high
    pub LUT907H: RWRegister<u32>,

    /// Graphic MMU LUT entry 908 low
    pub LUT908L: RWRegister<u32>,

    /// Graphic MMU LUT entry 908 high
    pub LUT908H: RWRegister<u32>,

    /// Graphic MMU LUT entry 909 low
    pub LUT909L: RWRegister<u32>,

    /// Graphic MMU LUT entry 909 high
    pub LUT909H: RWRegister<u32>,

    /// Graphic MMU LUT entry 910 low
    pub LUT910L: RWRegister<u32>,

    /// Graphic MMU LUT entry 910 high
    pub LUT910H: RWRegister<u32>,

    /// Graphic MMU LUT entry 911 low
    pub LUT911L: RWRegister<u32>,

    /// Graphic MMU LUT entry 911 high
    pub LUT911H: RWRegister<u32>,

    /// Graphic MMU LUT entry 912 low
    pub LUT912L: RWRegister<u32>,

    /// Graphic MMU LUT entry 912 high
    pub LUT912H: RWRegister<u32>,

    /// Graphic MMU LUT entry 913 low
    pub LUT913L: RWRegister<u32>,

    /// Graphic MMU LUT entry 913 high
    pub LUT913H: RWRegister<u32>,

    /// Graphic MMU LUT entry 914 low
    pub LUT914L: RWRegister<u32>,

    /// Graphic MMU LUT entry 914 high
    pub LUT914H: RWRegister<u32>,

    /// Graphic MMU LUT entry 915 low
    pub LUT915L: RWRegister<u32>,

    /// Graphic MMU LUT entry 915 high
    pub LUT915H: RWRegister<u32>,

    /// Graphic MMU LUT entry 916 low
    pub LUT916L: RWRegister<u32>,

    /// Graphic MMU LUT entry 916 high
    pub LUT916H: RWRegister<u32>,

    /// Graphic MMU LUT entry 917 low
    pub LUT917L: RWRegister<u32>,

    /// Graphic MMU LUT entry 917 high
    pub LUT917H: RWRegister<u32>,

    /// Graphic MMU LUT entry 918 low
    pub LUT918L: RWRegister<u32>,

    /// Graphic MMU LUT entry 918 high
    pub LUT918H: RWRegister<u32>,

    /// Graphic MMU LUT entry 919 low
    pub LUT919L: RWRegister<u32>,

    /// Graphic MMU LUT entry 919 high
    pub LUT919H: RWRegister<u32>,

    /// Graphic MMU LUT entry 920 low
    pub LUT920L: RWRegister<u32>,

    /// Graphic MMU LUT entry 920 high
    pub LUT920H: RWRegister<u32>,

    /// Graphic MMU LUT entry 921 low
    pub LUT921L: RWRegister<u32>,

    /// Graphic MMU LUT entry 921 high
    pub LUT921H: RWRegister<u32>,

    /// Graphic MMU LUT entry 922 low
    pub LUT922L: RWRegister<u32>,

    /// Graphic MMU LUT entry 922 high
    pub LUT922H: RWRegister<u32>,

    /// Graphic MMU LUT entry 923 low
    pub LUT923L: RWRegister<u32>,

    /// Graphic MMU LUT entry 923 high
    pub LUT923H: RWRegister<u32>,

    /// Graphic MMU LUT entry 924 low
    pub LUT924L: RWRegister<u32>,

    /// Graphic MMU LUT entry 924 high
    pub LUT924H: RWRegister<u32>,

    /// Graphic MMU LUT entry 925 low
    pub LUT925L: RWRegister<u32>,

    /// Graphic MMU LUT entry 925 high
    pub LUT925H: RWRegister<u32>,

    /// Graphic MMU LUT entry 926 low
    pub LUT926L: RWRegister<u32>,

    /// Graphic MMU LUT entry 926 high
    pub LUT926H: RWRegister<u32>,

    /// Graphic MMU LUT entry 927 low
    pub LUT927L: RWRegister<u32>,

    /// Graphic MMU LUT entry 927 high
    pub LUT927H: RWRegister<u32>,

    /// Graphic MMU LUT entry 928 low
    pub LUT928L: RWRegister<u32>,

    /// Graphic MMU LUT entry 928 high
    pub LUT928H: RWRegister<u32>,

    /// Graphic MMU LUT entry 929 low
    pub LUT929L: RWRegister<u32>,

    /// Graphic MMU LUT entry 929 high
    pub LUT929H: RWRegister<u32>,

    /// Graphic MMU LUT entry 930 low
    pub LUT930L: RWRegister<u32>,

    /// Graphic MMU LUT entry 930 high
    pub LUT930H: RWRegister<u32>,

    /// Graphic MMU LUT entry 931 low
    pub LUT931L: RWRegister<u32>,

    /// Graphic MMU LUT entry 931 high
    pub LUT931H: RWRegister<u32>,

    /// Graphic MMU LUT entry 932 low
    pub LUT932L: RWRegister<u32>,

    /// Graphic MMU LUT entry 932 high
    pub LUT932H: RWRegister<u32>,

    /// Graphic MMU LUT entry 933 low
    pub LUT933L: RWRegister<u32>,

    /// Graphic MMU LUT entry 933 high
    pub LUT933H: RWRegister<u32>,

    /// Graphic MMU LUT entry 934 low
    pub LUT934L: RWRegister<u32>,

    /// Graphic MMU LUT entry 934 high
    pub LUT934H: RWRegister<u32>,

    /// Graphic MMU LUT entry 935 low
    pub LUT935L: RWRegister<u32>,

    /// Graphic MMU LUT entry 935 high
    pub LUT935H: RWRegister<u32>,

    /// Graphic MMU LUT entry 936 low
    pub LUT936L: RWRegister<u32>,

    /// Graphic MMU LUT entry 936 high
    pub LUT936H: RWRegister<u32>,

    /// Graphic MMU LUT entry 937 low
    pub LUT937L: RWRegister<u32>,

    /// Graphic MMU LUT entry 937 high
    pub LUT937H: RWRegister<u32>,

    /// Graphic MMU LUT entry 938 low
    pub LUT938L: RWRegister<u32>,

    /// Graphic MMU LUT entry 938 high
    pub LUT938H: RWRegister<u32>,

    /// Graphic MMU LUT entry 939 low
    pub LUT939L: RWRegister<u32>,

    /// Graphic MMU LUT entry 939 high
    pub LUT939H: RWRegister<u32>,

    /// Graphic MMU LUT entry 940 low
    pub LUT940L: RWRegister<u32>,

    /// Graphic MMU LUT entry 940 high
    pub LUT940H: RWRegister<u32>,

    /// Graphic MMU LUT entry 941 low
    pub LUT941L: RWRegister<u32>,

    /// Graphic MMU LUT entry 941 high
    pub LUT941H: RWRegister<u32>,

    /// Graphic MMU LUT entry 942 low
    pub LUT942L: RWRegister<u32>,

    /// Graphic MMU LUT entry 942 high
    pub LUT942H: RWRegister<u32>,

    /// Graphic MMU LUT entry 943 low
    pub LUT943L: RWRegister<u32>,

    /// Graphic MMU LUT entry 943 high
    pub LUT943H: RWRegister<u32>,

    /// Graphic MMU LUT entry 944 low
    pub LUT944L: RWRegister<u32>,

    /// Graphic MMU LUT entry 944 high
    pub LUT944H: RWRegister<u32>,

    /// Graphic MMU LUT entry 945 low
    pub LUT945L: RWRegister<u32>,

    /// Graphic MMU LUT entry 945 high
    pub LUT945H: RWRegister<u32>,

    /// Graphic MMU LUT entry 946 low
    pub LUT946L: RWRegister<u32>,

    /// Graphic MMU LUT entry 946 high
    pub LUT946H: RWRegister<u32>,

    /// Graphic MMU LUT entry 947 low
    pub LUT947L: RWRegister<u32>,

    /// Graphic MMU LUT entry 947 high
    pub LUT947H: RWRegister<u32>,

    /// Graphic MMU LUT entry 948 low
    pub LUT948L: RWRegister<u32>,

    /// Graphic MMU LUT entry 948 high
    pub LUT948H: RWRegister<u32>,

    /// Graphic MMU LUT entry 949 low
    pub LUT949L: RWRegister<u32>,

    /// Graphic MMU LUT entry 949 high
    pub LUT949H: RWRegister<u32>,

    /// Graphic MMU LUT entry 950 low
    pub LUT950L: RWRegister<u32>,

    /// Graphic MMU LUT entry 950 high
    pub LUT950H: RWRegister<u32>,

    /// Graphic MMU LUT entry 951 low
    pub LUT951L: RWRegister<u32>,

    /// Graphic MMU LUT entry 951 high
    pub LUT951H: RWRegister<u32>,

    /// Graphic MMU LUT entry 952 low
    pub LUT952L: RWRegister<u32>,

    /// Graphic MMU LUT entry 952 high
    pub LUT952H: RWRegister<u32>,

    /// Graphic MMU LUT entry 953 low
    pub LUT953L: RWRegister<u32>,

    /// Graphic MMU LUT entry 953 high
    pub LUT953H: RWRegister<u32>,

    /// Graphic MMU LUT entry 954 low
    pub LUT954L: RWRegister<u32>,

    /// Graphic MMU LUT entry 954 high
    pub LUT954H: RWRegister<u32>,

    /// Graphic MMU LUT entry 955 low
    pub LUT955L: RWRegister<u32>,

    /// Graphic MMU LUT entry 955 high
    pub LUT955H: RWRegister<u32>,

    /// Graphic MMU LUT entry 956 low
    pub LUT956L: RWRegister<u32>,

    /// Graphic MMU LUT entry 956 high
    pub LUT956H: RWRegister<u32>,

    /// Graphic MMU LUT entry 957 low
    pub LUT957L: RWRegister<u32>,

    /// Graphic MMU LUT entry 957 high
    pub LUT957H: RWRegister<u32>,

    /// Graphic MMU LUT entry 958 low
    pub LUT958L: RWRegister<u32>,

    /// Graphic MMU LUT entry 958 high
    pub LUT958H: RWRegister<u32>,

    /// Graphic MMU LUT entry 959 low
    pub LUT959L: RWRegister<u32>,

    /// Graphic MMU LUT entry 959 high
    pub LUT959H: RWRegister<u32>,

    /// Graphic MMU LUT entry 960 low
    pub LUT960L: RWRegister<u32>,

    /// Graphic MMU LUT entry 960 high
    pub LUT960H: RWRegister<u32>,

    /// Graphic MMU LUT entry 961 low
    pub LUT961L: RWRegister<u32>,

    /// Graphic MMU LUT entry 961 high
    pub LUT961H: RWRegister<u32>,

    /// Graphic MMU LUT entry 962 low
    pub LUT962L: RWRegister<u32>,

    /// Graphic MMU LUT entry 962 high
    pub LUT962H: RWRegister<u32>,

    /// Graphic MMU LUT entry 963 low
    pub LUT963L: RWRegister<u32>,

    /// Graphic MMU LUT entry 963 high
    pub LUT963H: RWRegister<u32>,

    /// Graphic MMU LUT entry 964 low
    pub LUT964L: RWRegister<u32>,

    /// Graphic MMU LUT entry 964 high
    pub LUT964H: RWRegister<u32>,

    /// Graphic MMU LUT entry 965 low
    pub LUT965L: RWRegister<u32>,

    /// Graphic MMU LUT entry 965 high
    pub LUT965H: RWRegister<u32>,

    /// Graphic MMU LUT entry 966 low
    pub LUT966L: RWRegister<u32>,

    /// Graphic MMU LUT entry 966 high
    pub LUT966H: RWRegister<u32>,

    /// Graphic MMU LUT entry 967 low
    pub LUT967L: RWRegister<u32>,

    /// Graphic MMU LUT entry 967 high
    pub LUT967H: RWRegister<u32>,

    /// Graphic MMU LUT entry 968 low
    pub LUT968L: RWRegister<u32>,

    /// Graphic MMU LUT entry 968 high
    pub LUT968H: RWRegister<u32>,

    /// Graphic MMU LUT entry 969 low
    pub LUT969L: RWRegister<u32>,

    /// Graphic MMU LUT entry 969 high
    pub LUT969H: RWRegister<u32>,

    /// Graphic MMU LUT entry 970 low
    pub LUT970L: RWRegister<u32>,

    /// Graphic MMU LUT entry 970 high
    pub LUT970H: RWRegister<u32>,

    /// Graphic MMU LUT entry 971 low
    pub LUT971L: RWRegister<u32>,

    /// Graphic MMU LUT entry 971 high
    pub LUT971H: RWRegister<u32>,

    /// Graphic MMU LUT entry 972 low
    pub LUT972L: RWRegister<u32>,

    /// Graphic MMU LUT entry 972 high
    pub LUT972H: RWRegister<u32>,

    /// Graphic MMU LUT entry 973 low
    pub LUT973L: RWRegister<u32>,

    /// Graphic MMU LUT entry 973 high
    pub LUT973H: RWRegister<u32>,

    /// Graphic MMU LUT entry 974 low
    pub LUT974L: RWRegister<u32>,

    /// Graphic MMU LUT entry 974 high
    pub LUT974H: RWRegister<u32>,

    /// Graphic MMU LUT entry 975 low
    pub LUT975L: RWRegister<u32>,

    /// Graphic MMU LUT entry 975 high
    pub LUT975H: RWRegister<u32>,

    /// Graphic MMU LUT entry 976 low
    pub LUT976L: RWRegister<u32>,

    /// Graphic MMU LUT entry 976 high
    pub LUT976H: RWRegister<u32>,

    /// Graphic MMU LUT entry 977 low
    pub LUT977L: RWRegister<u32>,

    /// Graphic MMU LUT entry 977 high
    pub LUT977H: RWRegister<u32>,

    /// Graphic MMU LUT entry 978 low
    pub LUT978L: RWRegister<u32>,

    /// Graphic MMU LUT entry 978 high
    pub LUT978H: RWRegister<u32>,

    /// Graphic MMU LUT entry 979 low
    pub LUT979L: RWRegister<u32>,

    /// Graphic MMU LUT entry 979 high
    pub LUT979H: RWRegister<u32>,

    /// Graphic MMU LUT entry 980 low
    pub LUT980L: RWRegister<u32>,

    /// Graphic MMU LUT entry 980 high
    pub LUT980H: RWRegister<u32>,

    /// Graphic MMU LUT entry 981 low
    pub LUT981L: RWRegister<u32>,

    /// Graphic MMU LUT entry 981 high
    pub LUT981H: RWRegister<u32>,

    /// Graphic MMU LUT entry 982 low
    pub LUT982L: RWRegister<u32>,

    /// Graphic MMU LUT entry 982 high
    pub LUT982H: RWRegister<u32>,

    /// Graphic MMU LUT entry 983 low
    pub LUT983L: RWRegister<u32>,

    /// Graphic MMU LUT entry 983 high
    pub LUT983H: RWRegister<u32>,

    /// Graphic MMU LUT entry 984 low
    pub LUT984L: RWRegister<u32>,

    /// Graphic MMU LUT entry 984 high
    pub LUT984H: RWRegister<u32>,

    /// Graphic MMU LUT entry 985 low
    pub LUT985L: RWRegister<u32>,

    /// Graphic MMU LUT entry 985 high
    pub LUT985H: RWRegister<u32>,

    /// Graphic MMU LUT entry 986 low
    pub LUT986L: RWRegister<u32>,

    /// Graphic MMU LUT entry 986 high
    pub LUT986H: RWRegister<u32>,

    /// Graphic MMU LUT entry 987 low
    pub LUT987L: RWRegister<u32>,

    /// Graphic MMU LUT entry 987 high
    pub LUT987H: RWRegister<u32>,

    /// Graphic MMU LUT entry 988 low
    pub LUT988L: RWRegister<u32>,

    /// Graphic MMU LUT entry 988 high
    pub LUT988H: RWRegister<u32>,

    /// Graphic MMU LUT entry 989 low
    pub LUT989L: RWRegister<u32>,

    /// Graphic MMU LUT entry 989 high
    pub LUT989H: RWRegister<u32>,

    /// Graphic MMU LUT entry 990 low
    pub LUT990L: RWRegister<u32>,

    /// Graphic MMU LUT entry 990 high
    pub LUT990H: RWRegister<u32>,

    /// Graphic MMU LUT entry 991 low
    pub LUT991L: RWRegister<u32>,

    /// Graphic MMU LUT entry 991 high
    pub LUT991H: RWRegister<u32>,

    /// Graphic MMU LUT entry 992 low
    pub LUT992L: RWRegister<u32>,

    /// Graphic MMU LUT entry 992 high
    pub LUT992H: RWRegister<u32>,

    /// Graphic MMU LUT entry 993 low
    pub LUT993L: RWRegister<u32>,

    /// Graphic MMU LUT entry 993 high
    pub LUT993H: RWRegister<u32>,

    /// Graphic MMU LUT entry 994 low
    pub LUT994L: RWRegister<u32>,

    /// Graphic MMU LUT entry 994 high
    pub LUT994H: RWRegister<u32>,

    /// Graphic MMU LUT entry 995 low
    pub LUT995L: RWRegister<u32>,

    /// Graphic MMU LUT entry 995 high
    pub LUT995H: RWRegister<u32>,

    /// Graphic MMU LUT entry 996 low
    pub LUT996L: RWRegister<u32>,

    /// Graphic MMU LUT entry 996 high
    pub LUT996H: RWRegister<u32>,

    /// Graphic MMU LUT entry 997 low
    pub LUT997L: RWRegister<u32>,

    /// Graphic MMU LUT entry 997 high
    pub LUT997H: RWRegister<u32>,

    /// Graphic MMU LUT entry 998 low
    pub LUT998L: RWRegister<u32>,

    /// Graphic MMU LUT entry 998 high
    pub LUT998H: RWRegister<u32>,

    /// Graphic MMU LUT entry 999 low
    pub LUT999L: RWRegister<u32>,

    /// Graphic MMU LUT entry 999 high
    pub LUT999H: RWRegister<u32>,

    /// Graphic MMU LUT entry 1000 low
    pub LUT1000L: RWRegister<u32>,

    /// Graphic MMU LUT entry 1000 high
    pub LUT1000H: RWRegister<u32>,

    /// Graphic MMU LUT entry 1001 low
    pub LUT1001L: RWRegister<u32>,

    /// Graphic MMU LUT entry 1001 high
    pub LUT1001H: RWRegister<u32>,

    /// Graphic MMU LUT entry 1002 low
    pub LUT1002L: RWRegister<u32>,

    /// Graphic MMU LUT entry 1002 high
    pub LUT1002H: RWRegister<u32>,

    /// Graphic MMU LUT entry 1003 low
    pub LUT1003L: RWRegister<u32>,

    /// Graphic MMU LUT entry 1003 high
    pub LUT1003H: RWRegister<u32>,

    /// Graphic MMU LUT entry 1004 low
    pub LUT1004L: RWRegister<u32>,

    /// Graphic MMU LUT entry 1004 high
    pub LUT1004H: RWRegister<u32>,

    /// Graphic MMU LUT entry 1005 low
    pub LUT1005L: RWRegister<u32>,

    /// Graphic MMU LUT entry 1005 high
    pub LUT1005H: RWRegister<u32>,

    /// Graphic MMU LUT entry 1006 low
    pub LUT1006L: RWRegister<u32>,

    /// Graphic MMU LUT entry 1006 high
    pub LUT1006H: RWRegister<u32>,

    /// Graphic MMU LUT entry 1007 low
    pub LUT1007L: RWRegister<u32>,

    /// Graphic MMU LUT entry 1007 high
    pub LUT1007H: RWRegister<u32>,

    /// Graphic MMU LUT entry 1008 low
    pub LUT1008L: RWRegister<u32>,

    /// Graphic MMU LUT entry 1008 high
    pub LUT1008H: RWRegister<u32>,

    /// Graphic MMU LUT entry 1009 low
    pub LUT1009L: RWRegister<u32>,

    /// Graphic MMU LUT entry 1009 high
    pub LUT1009H: RWRegister<u32>,

    /// Graphic MMU LUT entry 1010 low
    pub LUT1010L: RWRegister<u32>,

    /// Graphic MMU LUT entry 1010 high
    pub LUT1010H: RWRegister<u32>,

    /// Graphic MMU LUT entry 1011 low
    pub LUT1011L: RWRegister<u32>,

    /// Graphic MMU LUT entry 1011 high
    pub LUT1011H: RWRegister<u32>,

    /// Graphic MMU LUT entry 1012 low
    pub LUT1012L: RWRegister<u32>,

    /// Graphic MMU LUT entry 1012 high
    pub LUT1012H: RWRegister<u32>,

    /// Graphic MMU LUT entry 1013 low
    pub LUT1013L: RWRegister<u32>,

    /// Graphic MMU LUT entry 1013 high
    pub LUT1013H: RWRegister<u32>,

    /// Graphic MMU LUT entry 1014 low
    pub LUT1014L: RWRegister<u32>,

    /// Graphic MMU LUT entry 1014 high
    pub LUT1014H: RWRegister<u32>,

    /// Graphic MMU LUT entry 1015 low
    pub LUT1015L: RWRegister<u32>,

    /// Graphic MMU LUT entry 1015 high
    pub LUT1015H: RWRegister<u32>,

    /// Graphic MMU LUT entry 1016 low
    pub LUT1016L: RWRegister<u32>,

    /// Graphic MMU LUT entry 1016 high
    pub LUT1016H: RWRegister<u32>,

    /// Graphic MMU LUT entry 1017 low
    pub LUT1017L: RWRegister<u32>,

    /// Graphic MMU LUT entry 1017 high
    pub LUT1017H: RWRegister<u32>,

    /// Graphic MMU LUT entry 1018 low
    pub LUT1018L: RWRegister<u32>,

    /// Graphic MMU LUT entry 1018 high
    pub LUT1018H: RWRegister<u32>,

    /// Graphic MMU LUT entry 1019 low
    pub LUT1019L: RWRegister<u32>,

    /// Graphic MMU LUT entry 1019 high
    pub LUT1019H: RWRegister<u32>,

    /// Graphic MMU LUT entry 1020 low
    pub LUT1020L: RWRegister<u32>,

    /// Graphic MMU LUT entry 1020 high
    pub LUT1020H: RWRegister<u32>,

    /// Graphic MMU LUT entry 1021 low
    pub LUT1021L: RWRegister<u32>,

    /// Graphic MMU LUT entry 1021 high
    pub LUT1021H: RWRegister<u32>,

    /// Graphic MMU LUT entry 1022 low
    pub LUT1022L: RWRegister<u32>,

    /// Graphic MMU LUT entry 1022 high
    pub LUT1022H: RWRegister<u32>,

    /// Graphic MMU LUT entry 1023 low
    pub LUT1023L: RWRegister<u32>,

    /// Graphic MMU LUT entry 1023 high
    pub LUT1023H: RWRegister<u32>,
}
pub struct ResetValues {
    pub CR: u32,
    pub SR: u32,
    pub FCR: u32,
    pub DVR: u32,
    pub B0CR: u32,
    pub B1CR: u32,
    pub B2CR: u32,
    pub B3CR: u32,
    pub VERR: u32,
    pub IPIDR: u32,
    pub SIDR: u32,
    pub LUT0L: u32,
    pub LUT0H: u32,
    pub LUT1L: u32,
    pub LUT1H: u32,
    pub LUT2L: u32,
    pub LUT2H: u32,
    pub LUT3L: u32,
    pub LUT3H: u32,
    pub LUT4L: u32,
    pub LUT4H: u32,
    pub LUT5L: u32,
    pub LUT5H: u32,
    pub LUT6L: u32,
    pub LUT6H: u32,
    pub LUT7L: u32,
    pub LUT7H: u32,
    pub LUT8L: u32,
    pub LUT8H: u32,
    pub LUT9L: u32,
    pub LUT9H: u32,
    pub LUT10L: u32,
    pub LUT10H: u32,
    pub LUT11L: u32,
    pub LUT11H: u32,
    pub LUT12L: u32,
    pub LUT12H: u32,
    pub LUT13L: u32,
    pub LUT13H: u32,
    pub LUT14L: u32,
    pub LUT14H: u32,
    pub LUT15L: u32,
    pub LUT15H: u32,
    pub LUT16L: u32,
    pub LUT16H: u32,
    pub LUT17L: u32,
    pub LUT17H: u32,
    pub LUT18L: u32,
    pub LUT18H: u32,
    pub LUT19L: u32,
    pub LUT19H: u32,
    pub LUT20L: u32,
    pub LUT20H: u32,
    pub LUT21L: u32,
    pub LUT21H: u32,
    pub LUT22L: u32,
    pub LUT22H: u32,
    pub LUT23L: u32,
    pub LUT23H: u32,
    pub LUT24L: u32,
    pub LUT24H: u32,
    pub LUT25L: u32,
    pub LUT25H: u32,
    pub LUT26L: u32,
    pub LUT26H: u32,
    pub LUT27L: u32,
    pub LUT27H: u32,
    pub LUT28L: u32,
    pub LUT28H: u32,
    pub LUT29L: u32,
    pub LUT29H: u32,
    pub LUT30L: u32,
    pub LUT30H: u32,
    pub LUT31L: u32,
    pub LUT31H: u32,
    pub LUT32L: u32,
    pub LUT32H: u32,
    pub LUT33L: u32,
    pub LUT33H: u32,
    pub LUT34L: u32,
    pub LUT34H: u32,
    pub LUT35L: u32,
    pub LUT35H: u32,
    pub LUT36L: u32,
    pub LUT36H: u32,
    pub LUT37L: u32,
    pub LUT37H: u32,
    pub LUT38L: u32,
    pub LUT38H: u32,
    pub LUT39L: u32,
    pub LUT39H: u32,
    pub LUT40L: u32,
    pub LUT40H: u32,
    pub LUT41L: u32,
    pub LUT41H: u32,
    pub LUT42L: u32,
    pub LUT42H: u32,
    pub LUT43L: u32,
    pub LUT43H: u32,
    pub LUT44L: u32,
    pub LUT44H: u32,
    pub LUT45L: u32,
    pub LUT45H: u32,
    pub LUT46L: u32,
    pub LUT46H: u32,
    pub LUT47L: u32,
    pub LUT47H: u32,
    pub LUT48L: u32,
    pub LUT48H: u32,
    pub LUT49L: u32,
    pub LUT49H: u32,
    pub LUT50L: u32,
    pub LUT50H: u32,
    pub LUT51L: u32,
    pub LUT51H: u32,
    pub LUT52L: u32,
    pub LUT52H: u32,
    pub LUT53L: u32,
    pub LUT53H: u32,
    pub LUT54L: u32,
    pub LUT54H: u32,
    pub LUT55L: u32,
    pub LUT55H: u32,
    pub LUT56L: u32,
    pub LUT56H: u32,
    pub LUT57L: u32,
    pub LUT57H: u32,
    pub LUT58L: u32,
    pub LUT58H: u32,
    pub LUT59L: u32,
    pub LUT59H: u32,
    pub LUT60L: u32,
    pub LUT60H: u32,
    pub LUT61L: u32,
    pub LUT61H: u32,
    pub LUT62L: u32,
    pub LUT62H: u32,
    pub LUT63L: u32,
    pub LUT63H: u32,
    pub LUT64L: u32,
    pub LUT64H: u32,
    pub LUT65L: u32,
    pub LUT65H: u32,
    pub LUT66L: u32,
    pub LUT66H: u32,
    pub LUT67L: u32,
    pub LUT67H: u32,
    pub LUT68L: u32,
    pub LUT68H: u32,
    pub LUT69L: u32,
    pub LUT69H: u32,
    pub LUT70L: u32,
    pub LUT70H: u32,
    pub LUT71L: u32,
    pub LUT71H: u32,
    pub LUT72L: u32,
    pub LUT72H: u32,
    pub LUT73L: u32,
    pub LUT73H: u32,
    pub LUT74L: u32,
    pub LUT74H: u32,
    pub LUT75L: u32,
    pub LUT75H: u32,
    pub LUT76L: u32,
    pub LUT76H: u32,
    pub LUT77L: u32,
    pub LUT77H: u32,
    pub LUT78L: u32,
    pub LUT78H: u32,
    pub LUT79L: u32,
    pub LUT79H: u32,
    pub LUT80L: u32,
    pub LUT80H: u32,
    pub LUT81L: u32,
    pub LUT81H: u32,
    pub LUT82L: u32,
    pub LUT82H: u32,
    pub LUT83L: u32,
    pub LUT83H: u32,
    pub LUT84L: u32,
    pub LUT84H: u32,
    pub LUT85L: u32,
    pub LUT85H: u32,
    pub LUT86L: u32,
    pub LUT86H: u32,
    pub LUT87L: u32,
    pub LUT87H: u32,
    pub LUT88L: u32,
    pub LUT88H: u32,
    pub LUT89L: u32,
    pub LUT89H: u32,
    pub LUT90L: u32,
    pub LUT90H: u32,
    pub LUT91L: u32,
    pub LUT91H: u32,
    pub LUT92L: u32,
    pub LUT92H: u32,
    pub LUT93L: u32,
    pub LUT93H: u32,
    pub LUT94L: u32,
    pub LUT94H: u32,
    pub LUT95L: u32,
    pub LUT95H: u32,
    pub LUT96L: u32,
    pub LUT96H: u32,
    pub LUT97L: u32,
    pub LUT97H: u32,
    pub LUT98L: u32,
    pub LUT98H: u32,
    pub LUT99L: u32,
    pub LUT99H: u32,
    pub LUT100L: u32,
    pub LUT100H: u32,
    pub LUT101L: u32,
    pub LUT101H: u32,
    pub LUT102L: u32,
    pub LUT102H: u32,
    pub LUT103L: u32,
    pub LUT103H: u32,
    pub LUT104L: u32,
    pub LUT104H: u32,
    pub LUT105L: u32,
    pub LUT105H: u32,
    pub LUT106L: u32,
    pub LUT106H: u32,
    pub LUT107L: u32,
    pub LUT107H: u32,
    pub LUT108L: u32,
    pub LUT108H: u32,
    pub LUT109L: u32,
    pub LUT109H: u32,
    pub LUT110L: u32,
    pub LUT110H: u32,
    pub LUT111L: u32,
    pub LUT111H: u32,
    pub LUT112L: u32,
    pub LUT112H: u32,
    pub LUT113L: u32,
    pub LUT113H: u32,
    pub LUT114L: u32,
    pub LUT114H: u32,
    pub LUT115L: u32,
    pub LUT115H: u32,
    pub LUT116L: u32,
    pub LUT116H: u32,
    pub LUT117L: u32,
    pub LUT117H: u32,
    pub LUT118L: u32,
    pub LUT118H: u32,
    pub LUT119L: u32,
    pub LUT119H: u32,
    pub LUT120L: u32,
    pub LUT120H: u32,
    pub LUT121L: u32,
    pub LUT121H: u32,
    pub LUT122L: u32,
    pub LUT122H: u32,
    pub LUT123L: u32,
    pub LUT123H: u32,
    pub LUT124L: u32,
    pub LUT124H: u32,
    pub LUT125L: u32,
    pub LUT125H: u32,
    pub LUT126L: u32,
    pub LUT126H: u32,
    pub LUT127L: u32,
    pub LUT127H: u32,
    pub LUT128L: u32,
    pub LUT128H: u32,
    pub LUT129L: u32,
    pub LUT129H: u32,
    pub LUT130L: u32,
    pub LUT130H: u32,
    pub LUT131L: u32,
    pub LUT131H: u32,
    pub LUT132L: u32,
    pub LUT132H: u32,
    pub LUT133L: u32,
    pub LUT133H: u32,
    pub LUT134L: u32,
    pub LUT134H: u32,
    pub LUT135L: u32,
    pub LUT135H: u32,
    pub LUT136L: u32,
    pub LUT136H: u32,
    pub LUT137L: u32,
    pub LUT137H: u32,
    pub LUT138L: u32,
    pub LUT138H: u32,
    pub LUT139L: u32,
    pub LUT139H: u32,
    pub LUT140L: u32,
    pub LUT140H: u32,
    pub LUT141L: u32,
    pub LUT141H: u32,
    pub LUT142L: u32,
    pub LUT142H: u32,
    pub LUT143L: u32,
    pub LUT143H: u32,
    pub LUT144L: u32,
    pub LUT144H: u32,
    pub LUT145L: u32,
    pub LUT145H: u32,
    pub LUT146L: u32,
    pub LUT146H: u32,
    pub LUT147L: u32,
    pub LUT147H: u32,
    pub LUT148L: u32,
    pub LUT148H: u32,
    pub LUT149L: u32,
    pub LUT149H: u32,
    pub LUT150L: u32,
    pub LUT150H: u32,
    pub LUT151L: u32,
    pub LUT151H: u32,
    pub LUT152L: u32,
    pub LUT152H: u32,
    pub LUT153L: u32,
    pub LUT153H: u32,
    pub LUT154L: u32,
    pub LUT154H: u32,
    pub LUT155L: u32,
    pub LUT155H: u32,
    pub LUT156L: u32,
    pub LUT156H: u32,
    pub LUT157L: u32,
    pub LUT157H: u32,
    pub LUT158L: u32,
    pub LUT158H: u32,
    pub LUT159L: u32,
    pub LUT159H: u32,
    pub LUT160L: u32,
    pub LUT160H: u32,
    pub LUT161L: u32,
    pub LUT161H: u32,
    pub LUT162L: u32,
    pub LUT162H: u32,
    pub LUT163L: u32,
    pub LUT163H: u32,
    pub LUT164L: u32,
    pub LUT164H: u32,
    pub LUT165L: u32,
    pub LUT165H: u32,
    pub LUT166L: u32,
    pub LUT166H: u32,
    pub LUT167L: u32,
    pub LUT167H: u32,
    pub LUT168L: u32,
    pub LUT168H: u32,
    pub LUT169L: u32,
    pub LUT169H: u32,
    pub LUT170L: u32,
    pub LUT170H: u32,
    pub LUT171L: u32,
    pub LUT171H: u32,
    pub LUT172L: u32,
    pub LUT172H: u32,
    pub LUT173L: u32,
    pub LUT173H: u32,
    pub LUT174L: u32,
    pub LUT174H: u32,
    pub LUT175L: u32,
    pub LUT175H: u32,
    pub LUT176L: u32,
    pub LUT176H: u32,
    pub LUT177L: u32,
    pub LUT177H: u32,
    pub LUT178L: u32,
    pub LUT178H: u32,
    pub LUT179L: u32,
    pub LUT179H: u32,
    pub LUT180L: u32,
    pub LUT180H: u32,
    pub LUT181L: u32,
    pub LUT181H: u32,
    pub LUT182L: u32,
    pub LUT182H: u32,
    pub LUT183L: u32,
    pub LUT183H: u32,
    pub LUT184L: u32,
    pub LUT184H: u32,
    pub LUT185L: u32,
    pub LUT185H: u32,
    pub LUT186L: u32,
    pub LUT186H: u32,
    pub LUT187L: u32,
    pub LUT187H: u32,
    pub LUT188L: u32,
    pub LUT188H: u32,
    pub LUT189L: u32,
    pub LUT189H: u32,
    pub LUT190L: u32,
    pub LUT190H: u32,
    pub LUT191L: u32,
    pub LUT191H: u32,
    pub LUT192L: u32,
    pub LUT192H: u32,
    pub LUT193L: u32,
    pub LUT193H: u32,
    pub LUT194L: u32,
    pub LUT194H: u32,
    pub LUT195L: u32,
    pub LUT195H: u32,
    pub LUT196L: u32,
    pub LUT196H: u32,
    pub LUT197L: u32,
    pub LUT197H: u32,
    pub LUT198L: u32,
    pub LUT198H: u32,
    pub LUT199L: u32,
    pub LUT199H: u32,
    pub LUT200L: u32,
    pub LUT200H: u32,
    pub LUT201L: u32,
    pub LUT201H: u32,
    pub LUT202L: u32,
    pub LUT202H: u32,
    pub LUT203L: u32,
    pub LUT203H: u32,
    pub LUT204L: u32,
    pub LUT204H: u32,
    pub LUT205L: u32,
    pub LUT205H: u32,
    pub LUT206L: u32,
    pub LUT206H: u32,
    pub LUT207L: u32,
    pub LUT207H: u32,
    pub LUT208L: u32,
    pub LUT208H: u32,
    pub LUT209L: u32,
    pub LUT209H: u32,
    pub LUT210L: u32,
    pub LUT210H: u32,
    pub LUT211L: u32,
    pub LUT211H: u32,
    pub LUT212L: u32,
    pub LUT212H: u32,
    pub LUT213L: u32,
    pub LUT213H: u32,
    pub LUT214L: u32,
    pub LUT214H: u32,
    pub LUT215L: u32,
    pub LUT215H: u32,
    pub LUT216L: u32,
    pub LUT216H: u32,
    pub LUT217L: u32,
    pub LUT217H: u32,
    pub LUT218L: u32,
    pub LUT218H: u32,
    pub LUT219L: u32,
    pub LUT219H: u32,
    pub LUT220L: u32,
    pub LUT220H: u32,
    pub LUT221L: u32,
    pub LUT221H: u32,
    pub LUT222L: u32,
    pub LUT222H: u32,
    pub LUT223L: u32,
    pub LUT223H: u32,
    pub LUT224L: u32,
    pub LUT224H: u32,
    pub LUT225L: u32,
    pub LUT225H: u32,
    pub LUT226L: u32,
    pub LUT226H: u32,
    pub LUT227L: u32,
    pub LUT227H: u32,
    pub LUT228L: u32,
    pub LUT228H: u32,
    pub LUT229L: u32,
    pub LUT229H: u32,
    pub LUT230L: u32,
    pub LUT230H: u32,
    pub LUT231L: u32,
    pub LUT231H: u32,
    pub LUT232L: u32,
    pub LUT232H: u32,
    pub LUT233L: u32,
    pub LUT233H: u32,
    pub LUT234L: u32,
    pub LUT234H: u32,
    pub LUT235L: u32,
    pub LUT235H: u32,
    pub LUT236L: u32,
    pub LUT236H: u32,
    pub LUT237L: u32,
    pub LUT237H: u32,
    pub LUT238L: u32,
    pub LUT238H: u32,
    pub LUT239L: u32,
    pub LUT239H: u32,
    pub LUT240L: u32,
    pub LUT240H: u32,
    pub LUT241L: u32,
    pub LUT241H: u32,
    pub LUT242L: u32,
    pub LUT242H: u32,
    pub LUT243L: u32,
    pub LUT243H: u32,
    pub LUT244L: u32,
    pub LUT244H: u32,
    pub LUT245L: u32,
    pub LUT245H: u32,
    pub LUT246L: u32,
    pub LUT246H: u32,
    pub LUT247L: u32,
    pub LUT247H: u32,
    pub LUT248L: u32,
    pub LUT248H: u32,
    pub LUT249L: u32,
    pub LUT249H: u32,
    pub LUT250L: u32,
    pub LUT250H: u32,
    pub LUT251L: u32,
    pub LUT251H: u32,
    pub LUT252L: u32,
    pub LUT252H: u32,
    pub LUT253L: u32,
    pub LUT253H: u32,
    pub LUT254L: u32,
    pub LUT254H: u32,
    pub LUT255L: u32,
    pub LUT255H: u32,
    pub LUT256L: u32,
    pub LUT256H: u32,
    pub LUT257L: u32,
    pub LUT257H: u32,
    pub LUT258L: u32,
    pub LUT258H: u32,
    pub LUT259L: u32,
    pub LUT259H: u32,
    pub LUT260L: u32,
    pub LUT260H: u32,
    pub LUT261L: u32,
    pub LUT261H: u32,
    pub LUT262L: u32,
    pub LUT262H: u32,
    pub LUT263L: u32,
    pub LUT263H: u32,
    pub LUT264L: u32,
    pub LUT264H: u32,
    pub LUT265L: u32,
    pub LUT265H: u32,
    pub LUT266L: u32,
    pub LUT266H: u32,
    pub LUT267L: u32,
    pub LUT267H: u32,
    pub LUT268L: u32,
    pub LUT268H: u32,
    pub LUT269L: u32,
    pub LUT269H: u32,
    pub LUT270L: u32,
    pub LUT270H: u32,
    pub LUT271L: u32,
    pub LUT271H: u32,
    pub LUT272L: u32,
    pub LUT272H: u32,
    pub LUT273L: u32,
    pub LUT273H: u32,
    pub LUT274L: u32,
    pub LUT274H: u32,
    pub LUT275L: u32,
    pub LUT275H: u32,
    pub LUT276L: u32,
    pub LUT276H: u32,
    pub LUT277L: u32,
    pub LUT277H: u32,
    pub LUT278L: u32,
    pub LUT278H: u32,
    pub LUT279L: u32,
    pub LUT279H: u32,
    pub LUT280L: u32,
    pub LUT280H: u32,
    pub LUT281L: u32,
    pub LUT281H: u32,
    pub LUT282L: u32,
    pub LUT282H: u32,
    pub LUT283L: u32,
    pub LUT283H: u32,
    pub LUT284L: u32,
    pub LUT284H: u32,
    pub LUT285L: u32,
    pub LUT285H: u32,
    pub LUT286L: u32,
    pub LUT286H: u32,
    pub LUT287L: u32,
    pub LUT287H: u32,
    pub LUT288L: u32,
    pub LUT288H: u32,
    pub LUT289L: u32,
    pub LUT289H: u32,
    pub LUT290L: u32,
    pub LUT290H: u32,
    pub LUT291L: u32,
    pub LUT291H: u32,
    pub LUT292L: u32,
    pub LUT292H: u32,
    pub LUT293L: u32,
    pub LUT293H: u32,
    pub LUT294L: u32,
    pub LUT294H: u32,
    pub LUT295L: u32,
    pub LUT295H: u32,
    pub LUT296L: u32,
    pub LUT296H: u32,
    pub LUT297L: u32,
    pub LUT297H: u32,
    pub LUT298L: u32,
    pub LUT298H: u32,
    pub LUT299L: u32,
    pub LUT299H: u32,
    pub LUT300L: u32,
    pub LUT300H: u32,
    pub LUT301L: u32,
    pub LUT301H: u32,
    pub LUT302L: u32,
    pub LUT302H: u32,
    pub LUT303L: u32,
    pub LUT303H: u32,
    pub LUT304L: u32,
    pub LUT304H: u32,
    pub LUT305L: u32,
    pub LUT305H: u32,
    pub LUT306L: u32,
    pub LUT306H: u32,
    pub LUT307L: u32,
    pub LUT307H: u32,
    pub LUT308L: u32,
    pub LUT308H: u32,
    pub LUT309L: u32,
    pub LUT309H: u32,
    pub LUT310L: u32,
    pub LUT310H: u32,
    pub LUT311L: u32,
    pub LUT311H: u32,
    pub LUT312L: u32,
    pub LUT312H: u32,
    pub LUT313L: u32,
    pub LUT313H: u32,
    pub LUT314L: u32,
    pub LUT314H: u32,
    pub LUT315L: u32,
    pub LUT315H: u32,
    pub LUT316L: u32,
    pub LUT316H: u32,
    pub LUT317L: u32,
    pub LUT317H: u32,
    pub LUT318L: u32,
    pub LUT318H: u32,
    pub LUT319L: u32,
    pub LUT319H: u32,
    pub LUT320L: u32,
    pub LUT320H: u32,
    pub LUT321L: u32,
    pub LUT321H: u32,
    pub LUT322L: u32,
    pub LUT322H: u32,
    pub LUT323L: u32,
    pub LUT323H: u32,
    pub LUT324L: u32,
    pub LUT324H: u32,
    pub LUT325L: u32,
    pub LUT325H: u32,
    pub LUT326L: u32,
    pub LUT326H: u32,
    pub LUT327L: u32,
    pub LUT327H: u32,
    pub LUT328L: u32,
    pub LUT328H: u32,
    pub LUT329L: u32,
    pub LUT329H: u32,
    pub LUT330L: u32,
    pub LUT330H: u32,
    pub LUT331L: u32,
    pub LUT331H: u32,
    pub LUT332L: u32,
    pub LUT332H: u32,
    pub LUT333L: u32,
    pub LUT333H: u32,
    pub LUT334L: u32,
    pub LUT334H: u32,
    pub LUT335L: u32,
    pub LUT335H: u32,
    pub LUT336L: u32,
    pub LUT336H: u32,
    pub LUT337L: u32,
    pub LUT337H: u32,
    pub LUT338L: u32,
    pub LUT338H: u32,
    pub LUT339L: u32,
    pub LUT339H: u32,
    pub LUT340L: u32,
    pub LUT340H: u32,
    pub LUT341L: u32,
    pub LUT341H: u32,
    pub LUT342L: u32,
    pub LUT342H: u32,
    pub LUT343L: u32,
    pub LUT343H: u32,
    pub LUT344L: u32,
    pub LUT344H: u32,
    pub LUT345L: u32,
    pub LUT345H: u32,
    pub LUT346L: u32,
    pub LUT346H: u32,
    pub LUT347L: u32,
    pub LUT347H: u32,
    pub LUT348L: u32,
    pub LUT348H: u32,
    pub LUT349L: u32,
    pub LUT349H: u32,
    pub LUT350L: u32,
    pub LUT350H: u32,
    pub LUT351L: u32,
    pub LUT351H: u32,
    pub LUT352L: u32,
    pub LUT352H: u32,
    pub LUT353L: u32,
    pub LUT353H: u32,
    pub LUT354L: u32,
    pub LUT354H: u32,
    pub LUT355L: u32,
    pub LUT355H: u32,
    pub LUT356L: u32,
    pub LUT356H: u32,
    pub LUT357L: u32,
    pub LUT357H: u32,
    pub LUT358L: u32,
    pub LUT358H: u32,
    pub LUT359L: u32,
    pub LUT359H: u32,
    pub LUT360L: u32,
    pub LUT360H: u32,
    pub LUT361L: u32,
    pub LUT361H: u32,
    pub LUT362L: u32,
    pub LUT362H: u32,
    pub LUT363L: u32,
    pub LUT363H: u32,
    pub LUT364L: u32,
    pub LUT364H: u32,
    pub LUT365L: u32,
    pub LUT365H: u32,
    pub LUT366L: u32,
    pub LUT366H: u32,
    pub LUT367L: u32,
    pub LUT367H: u32,
    pub LUT368L: u32,
    pub LUT368H: u32,
    pub LUT369L: u32,
    pub LUT369H: u32,
    pub LUT370L: u32,
    pub LUT370H: u32,
    pub LUT371L: u32,
    pub LUT371H: u32,
    pub LUT372L: u32,
    pub LUT372H: u32,
    pub LUT373L: u32,
    pub LUT373H: u32,
    pub LUT374L: u32,
    pub LUT374H: u32,
    pub LUT375L: u32,
    pub LUT375H: u32,
    pub LUT376L: u32,
    pub LUT376H: u32,
    pub LUT377L: u32,
    pub LUT377H: u32,
    pub LUT378L: u32,
    pub LUT378H: u32,
    pub LUT379L: u32,
    pub LUT379H: u32,
    pub LUT380L: u32,
    pub LUT380H: u32,
    pub LUT381L: u32,
    pub LUT381H: u32,
    pub LUT382L: u32,
    pub LUT382H: u32,
    pub LUT383L: u32,
    pub LUT383H: u32,
    pub LUT384L: u32,
    pub LUT384H: u32,
    pub LUT385L: u32,
    pub LUT385H: u32,
    pub LUT386L: u32,
    pub LUT386H: u32,
    pub LUT387L: u32,
    pub LUT387H: u32,
    pub LUT388L: u32,
    pub LUT388H: u32,
    pub LUT389L: u32,
    pub LUT389H: u32,
    pub LUT390L: u32,
    pub LUT390H: u32,
    pub LUT391L: u32,
    pub LUT391H: u32,
    pub LUT392L: u32,
    pub LUT392H: u32,
    pub LUT393L: u32,
    pub LUT393H: u32,
    pub LUT394L: u32,
    pub LUT394H: u32,
    pub LUT395L: u32,
    pub LUT395H: u32,
    pub LUT396L: u32,
    pub LUT396H: u32,
    pub LUT397L: u32,
    pub LUT397H: u32,
    pub LUT398L: u32,
    pub LUT398H: u32,
    pub LUT399L: u32,
    pub LUT399H: u32,
    pub LUT400L: u32,
    pub LUT400H: u32,
    pub LUT401L: u32,
    pub LUT401H: u32,
    pub LUT402L: u32,
    pub LUT402H: u32,
    pub LUT403L: u32,
    pub LUT403H: u32,
    pub LUT404L: u32,
    pub LUT404H: u32,
    pub LUT405L: u32,
    pub LUT405H: u32,
    pub LUT406L: u32,
    pub LUT406H: u32,
    pub LUT407L: u32,
    pub LUT407H: u32,
    pub LUT408L: u32,
    pub LUT408H: u32,
    pub LUT409L: u32,
    pub LUT409H: u32,
    pub LUT410L: u32,
    pub LUT410H: u32,
    pub LUT411L: u32,
    pub LUT411H: u32,
    pub LUT412L: u32,
    pub LUT412H: u32,
    pub LUT413L: u32,
    pub LUT413H: u32,
    pub LUT414L: u32,
    pub LUT414H: u32,
    pub LUT415L: u32,
    pub LUT415H: u32,
    pub LUT416L: u32,
    pub LUT416H: u32,
    pub LUT417L: u32,
    pub LUT417H: u32,
    pub LUT418L: u32,
    pub LUT418H: u32,
    pub LUT419L: u32,
    pub LUT419H: u32,
    pub LUT420L: u32,
    pub LUT420H: u32,
    pub LUT421L: u32,
    pub LUT421H: u32,
    pub LUT422L: u32,
    pub LUT422H: u32,
    pub LUT423L: u32,
    pub LUT423H: u32,
    pub LUT424L: u32,
    pub LUT424H: u32,
    pub LUT425L: u32,
    pub LUT425H: u32,
    pub LUT426L: u32,
    pub LUT426H: u32,
    pub LUT427L: u32,
    pub LUT427H: u32,
    pub LUT428L: u32,
    pub LUT428H: u32,
    pub LUT429L: u32,
    pub LUT429H: u32,
    pub LUT430L: u32,
    pub LUT430H: u32,
    pub LUT431L: u32,
    pub LUT431H: u32,
    pub LUT432L: u32,
    pub LUT432H: u32,
    pub LUT433L: u32,
    pub LUT433H: u32,
    pub LUT434L: u32,
    pub LUT434H: u32,
    pub LUT435L: u32,
    pub LUT435H: u32,
    pub LUT436L: u32,
    pub LUT436H: u32,
    pub LUT437L: u32,
    pub LUT437H: u32,
    pub LUT438L: u32,
    pub LUT438H: u32,
    pub LUT439L: u32,
    pub LUT439H: u32,
    pub LUT440L: u32,
    pub LUT440H: u32,
    pub LUT441L: u32,
    pub LUT441H: u32,
    pub LUT442L: u32,
    pub LUT442H: u32,
    pub LUT443L: u32,
    pub LUT443H: u32,
    pub LUT444L: u32,
    pub LUT444H: u32,
    pub LUT445L: u32,
    pub LUT445H: u32,
    pub LUT446L: u32,
    pub LUT446H: u32,
    pub LUT447L: u32,
    pub LUT447H: u32,
    pub LUT448L: u32,
    pub LUT448H: u32,
    pub LUT449L: u32,
    pub LUT449H: u32,
    pub LUT450L: u32,
    pub LUT450H: u32,
    pub LUT451L: u32,
    pub LUT451H: u32,
    pub LUT452L: u32,
    pub LUT452H: u32,
    pub LUT453L: u32,
    pub LUT453H: u32,
    pub LUT454L: u32,
    pub LUT454H: u32,
    pub LUT455L: u32,
    pub LUT455H: u32,
    pub LUT456L: u32,
    pub LUT456H: u32,
    pub LUT457L: u32,
    pub LUT457H: u32,
    pub LUT458L: u32,
    pub LUT458H: u32,
    pub LUT459L: u32,
    pub LUT459H: u32,
    pub LUT460L: u32,
    pub LUT460H: u32,
    pub LUT461L: u32,
    pub LUT461H: u32,
    pub LUT462L: u32,
    pub LUT462H: u32,
    pub LUT463L: u32,
    pub LUT463H: u32,
    pub LUT464L: u32,
    pub LUT464H: u32,
    pub LUT465L: u32,
    pub LUT465H: u32,
    pub LUT466L: u32,
    pub LUT466H: u32,
    pub LUT467L: u32,
    pub LUT467H: u32,
    pub LUT468L: u32,
    pub LUT468H: u32,
    pub LUT469L: u32,
    pub LUT469H: u32,
    pub LUT470L: u32,
    pub LUT470H: u32,
    pub LUT471L: u32,
    pub LUT471H: u32,
    pub LUT472L: u32,
    pub LUT472H: u32,
    pub LUT473L: u32,
    pub LUT473H: u32,
    pub LUT474L: u32,
    pub LUT474H: u32,
    pub LUT475L: u32,
    pub LUT475H: u32,
    pub LUT476L: u32,
    pub LUT476H: u32,
    pub LUT477L: u32,
    pub LUT477H: u32,
    pub LUT478L: u32,
    pub LUT478H: u32,
    pub LUT479L: u32,
    pub LUT479H: u32,
    pub LUT480L: u32,
    pub LUT480H: u32,
    pub LUT481L: u32,
    pub LUT481H: u32,
    pub LUT482L: u32,
    pub LUT482H: u32,
    pub LUT483L: u32,
    pub LUT483H: u32,
    pub LUT484L: u32,
    pub LUT484H: u32,
    pub LUT485L: u32,
    pub LUT485H: u32,
    pub LUT486L: u32,
    pub LUT486H: u32,
    pub LUT487L: u32,
    pub LUT487H: u32,
    pub LUT488L: u32,
    pub LUT488H: u32,
    pub LUT489L: u32,
    pub LUT489H: u32,
    pub LUT490L: u32,
    pub LUT490H: u32,
    pub LUT491L: u32,
    pub LUT491H: u32,
    pub LUT492L: u32,
    pub LUT492H: u32,
    pub LUT493L: u32,
    pub LUT493H: u32,
    pub LUT494L: u32,
    pub LUT494H: u32,
    pub LUT495L: u32,
    pub LUT495H: u32,
    pub LUT496L: u32,
    pub LUT496H: u32,
    pub LUT497L: u32,
    pub LUT497H: u32,
    pub LUT498L: u32,
    pub LUT498H: u32,
    pub LUT499L: u32,
    pub LUT499H: u32,
    pub LUT500L: u32,
    pub LUT500H: u32,
    pub LUT501L: u32,
    pub LUT501H: u32,
    pub LUT502L: u32,
    pub LUT502H: u32,
    pub LUT503L: u32,
    pub LUT503H: u32,
    pub LUT504L: u32,
    pub LUT504H: u32,
    pub LUT505L: u32,
    pub LUT505H: u32,
    pub LUT506L: u32,
    pub LUT506H: u32,
    pub LUT507L: u32,
    pub LUT507H: u32,
    pub LUT508L: u32,
    pub LUT508H: u32,
    pub LUT509L: u32,
    pub LUT509H: u32,
    pub LUT510L: u32,
    pub LUT510H: u32,
    pub LUT511L: u32,
    pub LUT511H: u32,
    pub LUT512L: u32,
    pub LUT512H: u32,
    pub LUT513L: u32,
    pub LUT513H: u32,
    pub LUT514L: u32,
    pub LUT514H: u32,
    pub LUT515L: u32,
    pub LUT515H: u32,
    pub LUT516L: u32,
    pub LUT516H: u32,
    pub LUT517L: u32,
    pub LUT517H: u32,
    pub LUT518L: u32,
    pub LUT518H: u32,
    pub LUT519L: u32,
    pub LUT519H: u32,
    pub LUT520L: u32,
    pub LUT520H: u32,
    pub LUT521L: u32,
    pub LUT521H: u32,
    pub LUT522L: u32,
    pub LUT522H: u32,
    pub LUT523L: u32,
    pub LUT523H: u32,
    pub LUT524L: u32,
    pub LUT524H: u32,
    pub LUT525L: u32,
    pub LUT525H: u32,
    pub LUT526L: u32,
    pub LUT526H: u32,
    pub LUT527L: u32,
    pub LUT527H: u32,
    pub LUT528L: u32,
    pub LUT528H: u32,
    pub LUT529L: u32,
    pub LUT529H: u32,
    pub LUT530L: u32,
    pub LUT530H: u32,
    pub LUT531L: u32,
    pub LUT531H: u32,
    pub LUT532L: u32,
    pub LUT532H: u32,
    pub LUT533L: u32,
    pub LUT533H: u32,
    pub LUT534L: u32,
    pub LUT534H: u32,
    pub LUT535L: u32,
    pub LUT535H: u32,
    pub LUT536L: u32,
    pub LUT536H: u32,
    pub LUT537L: u32,
    pub LUT537H: u32,
    pub LUT538L: u32,
    pub LUT538H: u32,
    pub LUT539L: u32,
    pub LUT539H: u32,
    pub LUT540L: u32,
    pub LUT540H: u32,
    pub LUT541L: u32,
    pub LUT541H: u32,
    pub LUT542L: u32,
    pub LUT542H: u32,
    pub LUT543L: u32,
    pub LUT543H: u32,
    pub LUT544L: u32,
    pub LUT544H: u32,
    pub LUT545L: u32,
    pub LUT545H: u32,
    pub LUT546L: u32,
    pub LUT546H: u32,
    pub LUT547L: u32,
    pub LUT547H: u32,
    pub LUT548L: u32,
    pub LUT548H: u32,
    pub LUT549L: u32,
    pub LUT549H: u32,
    pub LUT550L: u32,
    pub LUT550H: u32,
    pub LUT551L: u32,
    pub LUT551H: u32,
    pub LUT552L: u32,
    pub LUT552H: u32,
    pub LUT553L: u32,
    pub LUT553H: u32,
    pub LUT554L: u32,
    pub LUT554H: u32,
    pub LUT555L: u32,
    pub LUT555H: u32,
    pub LUT556L: u32,
    pub LUT556H: u32,
    pub LUT557L: u32,
    pub LUT557H: u32,
    pub LUT558L: u32,
    pub LUT558H: u32,
    pub LUT559L: u32,
    pub LUT559H: u32,
    pub LUT560L: u32,
    pub LUT560H: u32,
    pub LUT561L: u32,
    pub LUT561H: u32,
    pub LUT562L: u32,
    pub LUT562H: u32,
    pub LUT563L: u32,
    pub LUT563H: u32,
    pub LUT564L: u32,
    pub LUT564H: u32,
    pub LUT565L: u32,
    pub LUT565H: u32,
    pub LUT566L: u32,
    pub LUT566H: u32,
    pub LUT567L: u32,
    pub LUT567H: u32,
    pub LUT568L: u32,
    pub LUT568H: u32,
    pub LUT569L: u32,
    pub LUT569H: u32,
    pub LUT570L: u32,
    pub LUT570H: u32,
    pub LUT571L: u32,
    pub LUT571H: u32,
    pub LUT572L: u32,
    pub LUT572H: u32,
    pub LUT573L: u32,
    pub LUT573H: u32,
    pub LUT574L: u32,
    pub LUT574H: u32,
    pub LUT575L: u32,
    pub LUT575H: u32,
    pub LUT576L: u32,
    pub LUT576H: u32,
    pub LUT577L: u32,
    pub LUT577H: u32,
    pub LUT578L: u32,
    pub LUT578H: u32,
    pub LUT579L: u32,
    pub LUT579H: u32,
    pub LUT580L: u32,
    pub LUT580H: u32,
    pub LUT581L: u32,
    pub LUT581H: u32,
    pub LUT582L: u32,
    pub LUT582H: u32,
    pub LUT583L: u32,
    pub LUT583H: u32,
    pub LUT584L: u32,
    pub LUT584H: u32,
    pub LUT585L: u32,
    pub LUT585H: u32,
    pub LUT586L: u32,
    pub LUT586H: u32,
    pub LUT587L: u32,
    pub LUT587H: u32,
    pub LUT588L: u32,
    pub LUT588H: u32,
    pub LUT589L: u32,
    pub LUT589H: u32,
    pub LUT590L: u32,
    pub LUT590H: u32,
    pub LUT591L: u32,
    pub LUT591H: u32,
    pub LUT592L: u32,
    pub LUT592H: u32,
    pub LUT593L: u32,
    pub LUT593H: u32,
    pub LUT594L: u32,
    pub LUT594H: u32,
    pub LUT595L: u32,
    pub LUT595H: u32,
    pub LUT596L: u32,
    pub LUT596H: u32,
    pub LUT597L: u32,
    pub LUT597H: u32,
    pub LUT598L: u32,
    pub LUT598H: u32,
    pub LUT599L: u32,
    pub LUT599H: u32,
    pub LUT600L: u32,
    pub LUT600H: u32,
    pub LUT601L: u32,
    pub LUT601H: u32,
    pub LUT602L: u32,
    pub LUT602H: u32,
    pub LUT603L: u32,
    pub LUT603H: u32,
    pub LUT604L: u32,
    pub LUT604H: u32,
    pub LUT605L: u32,
    pub LUT605H: u32,
    pub LUT606L: u32,
    pub LUT606H: u32,
    pub LUT607L: u32,
    pub LUT607H: u32,
    pub LUT608L: u32,
    pub LUT608H: u32,
    pub LUT609L: u32,
    pub LUT609H: u32,
    pub LUT610L: u32,
    pub LUT610H: u32,
    pub LUT611L: u32,
    pub LUT611H: u32,
    pub LUT612L: u32,
    pub LUT612H: u32,
    pub LUT613L: u32,
    pub LUT613H: u32,
    pub LUT614L: u32,
    pub LUT614H: u32,
    pub LUT615L: u32,
    pub LUT615H: u32,
    pub LUT616L: u32,
    pub LUT616H: u32,
    pub LUT617L: u32,
    pub LUT617H: u32,
    pub LUT618L: u32,
    pub LUT618H: u32,
    pub LUT619L: u32,
    pub LUT619H: u32,
    pub LUT620L: u32,
    pub LUT620H: u32,
    pub LUT621L: u32,
    pub LUT621H: u32,
    pub LUT622L: u32,
    pub LUT622H: u32,
    pub LUT623L: u32,
    pub LUT623H: u32,
    pub LUT624L: u32,
    pub LUT624H: u32,
    pub LUT625L: u32,
    pub LUT625H: u32,
    pub LUT626L: u32,
    pub LUT626H: u32,
    pub LUT627L: u32,
    pub LUT627H: u32,
    pub LUT628L: u32,
    pub LUT628H: u32,
    pub LUT629L: u32,
    pub LUT629H: u32,
    pub LUT630L: u32,
    pub LUT630H: u32,
    pub LUT631L: u32,
    pub LUT631H: u32,
    pub LUT632L: u32,
    pub LUT632H: u32,
    pub LUT633L: u32,
    pub LUT633H: u32,
    pub LUT634L: u32,
    pub LUT634H: u32,
    pub LUT635L: u32,
    pub LUT635H: u32,
    pub LUT636L: u32,
    pub LUT636H: u32,
    pub LUT637L: u32,
    pub LUT637H: u32,
    pub LUT638L: u32,
    pub LUT638H: u32,
    pub LUT639L: u32,
    pub LUT639H: u32,
    pub LUT640L: u32,
    pub LUT640H: u32,
    pub LUT641L: u32,
    pub LUT641H: u32,
    pub LUT642L: u32,
    pub LUT642H: u32,
    pub LUT643L: u32,
    pub LUT643H: u32,
    pub LUT644L: u32,
    pub LUT644H: u32,
    pub LUT645L: u32,
    pub LUT645H: u32,
    pub LUT646L: u32,
    pub LUT646H: u32,
    pub LUT647L: u32,
    pub LUT647H: u32,
    pub LUT648L: u32,
    pub LUT648H: u32,
    pub LUT649L: u32,
    pub LUT649H: u32,
    pub LUT650L: u32,
    pub LUT650H: u32,
    pub LUT651L: u32,
    pub LUT651H: u32,
    pub LUT652L: u32,
    pub LUT652H: u32,
    pub LUT653L: u32,
    pub LUT653H: u32,
    pub LUT654L: u32,
    pub LUT654H: u32,
    pub LUT655L: u32,
    pub LUT655H: u32,
    pub LUT656L: u32,
    pub LUT656H: u32,
    pub LUT657L: u32,
    pub LUT657H: u32,
    pub LUT658L: u32,
    pub LUT658H: u32,
    pub LUT659L: u32,
    pub LUT659H: u32,
    pub LUT660L: u32,
    pub LUT660H: u32,
    pub LUT661L: u32,
    pub LUT661H: u32,
    pub LUT662L: u32,
    pub LUT662H: u32,
    pub LUT663L: u32,
    pub LUT663H: u32,
    pub LUT664L: u32,
    pub LUT664H: u32,
    pub LUT665L: u32,
    pub LUT665H: u32,
    pub LUT666L: u32,
    pub LUT666H: u32,
    pub LUT667L: u32,
    pub LUT667H: u32,
    pub LUT668L: u32,
    pub LUT668H: u32,
    pub LUT669L: u32,
    pub LUT669H: u32,
    pub LUT670L: u32,
    pub LUT670H: u32,
    pub LUT671L: u32,
    pub LUT671H: u32,
    pub LUT672L: u32,
    pub LUT672H: u32,
    pub LUT673L: u32,
    pub LUT673H: u32,
    pub LUT674L: u32,
    pub LUT674H: u32,
    pub LUT675L: u32,
    pub LUT675H: u32,
    pub LUT676L: u32,
    pub LUT676H: u32,
    pub LUT677L: u32,
    pub LUT677H: u32,
    pub LUT678L: u32,
    pub LUT678H: u32,
    pub LUT679L: u32,
    pub LUT679H: u32,
    pub LUT680L: u32,
    pub LUT680H: u32,
    pub LUT681L: u32,
    pub LUT681H: u32,
    pub LUT682L: u32,
    pub LUT682H: u32,
    pub LUT683L: u32,
    pub LUT683H: u32,
    pub LUT684L: u32,
    pub LUT684H: u32,
    pub LUT685L: u32,
    pub LUT685H: u32,
    pub LUT686L: u32,
    pub LUT686H: u32,
    pub LUT687L: u32,
    pub LUT687H: u32,
    pub LUT688L: u32,
    pub LUT688H: u32,
    pub LUT689L: u32,
    pub LUT689H: u32,
    pub LUT690L: u32,
    pub LUT690H: u32,
    pub LUT691L: u32,
    pub LUT691H: u32,
    pub LUT692L: u32,
    pub LUT692H: u32,
    pub LUT693L: u32,
    pub LUT693H: u32,
    pub LUT694L: u32,
    pub LUT694H: u32,
    pub LUT695L: u32,
    pub LUT695H: u32,
    pub LUT696L: u32,
    pub LUT696H: u32,
    pub LUT697L: u32,
    pub LUT697H: u32,
    pub LUT698L: u32,
    pub LUT698H: u32,
    pub LUT699L: u32,
    pub LUT699H: u32,
    pub LUT700L: u32,
    pub LUT700H: u32,
    pub LUT701L: u32,
    pub LUT701H: u32,
    pub LUT702L: u32,
    pub LUT702H: u32,
    pub LUT703L: u32,
    pub LUT703H: u32,
    pub LUT704L: u32,
    pub LUT704H: u32,
    pub LUT705L: u32,
    pub LUT705H: u32,
    pub LUT706L: u32,
    pub LUT706H: u32,
    pub LUT707L: u32,
    pub LUT707H: u32,
    pub LUT708L: u32,
    pub LUT708H: u32,
    pub LUT709L: u32,
    pub LUT709H: u32,
    pub LUT710L: u32,
    pub LUT710H: u32,
    pub LUT711L: u32,
    pub LUT711H: u32,
    pub LUT712L: u32,
    pub LUT712H: u32,
    pub LUT713L: u32,
    pub LUT713H: u32,
    pub LUT714L: u32,
    pub LUT714H: u32,
    pub LUT715L: u32,
    pub LUT715H: u32,
    pub LUT716L: u32,
    pub LUT716H: u32,
    pub LUT717L: u32,
    pub LUT717H: u32,
    pub LUT718L: u32,
    pub LUT718H: u32,
    pub LUT719L: u32,
    pub LUT719H: u32,
    pub LUT720L: u32,
    pub LUT720H: u32,
    pub LUT721L: u32,
    pub LUT721H: u32,
    pub LUT722L: u32,
    pub LUT722H: u32,
    pub LUT723L: u32,
    pub LUT723H: u32,
    pub LUT724L: u32,
    pub LUT724H: u32,
    pub LUT725L: u32,
    pub LUT725H: u32,
    pub LUT726L: u32,
    pub LUT726H: u32,
    pub LUT727L: u32,
    pub LUT727H: u32,
    pub LUT728L: u32,
    pub LUT728H: u32,
    pub LUT729L: u32,
    pub LUT729H: u32,
    pub LUT730L: u32,
    pub LUT730H: u32,
    pub LUT731L: u32,
    pub LUT731H: u32,
    pub LUT732L: u32,
    pub LUT732H: u32,
    pub LUT733L: u32,
    pub LUT733H: u32,
    pub LUT734L: u32,
    pub LUT734H: u32,
    pub LUT735L: u32,
    pub LUT735H: u32,
    pub LUT736L: u32,
    pub LUT736H: u32,
    pub LUT737L: u32,
    pub LUT737H: u32,
    pub LUT738L: u32,
    pub LUT738H: u32,
    pub LUT739L: u32,
    pub LUT739H: u32,
    pub LUT740L: u32,
    pub LUT740H: u32,
    pub LUT741L: u32,
    pub LUT741H: u32,
    pub LUT742L: u32,
    pub LUT742H: u32,
    pub LUT743L: u32,
    pub LUT743H: u32,
    pub LUT744L: u32,
    pub LUT744H: u32,
    pub LUT745L: u32,
    pub LUT745H: u32,
    pub LUT746L: u32,
    pub LUT746H: u32,
    pub LUT747L: u32,
    pub LUT747H: u32,
    pub LUT748L: u32,
    pub LUT748H: u32,
    pub LUT749L: u32,
    pub LUT749H: u32,
    pub LUT750L: u32,
    pub LUT750H: u32,
    pub LUT751L: u32,
    pub LUT751H: u32,
    pub LUT752L: u32,
    pub LUT752H: u32,
    pub LUT753L: u32,
    pub LUT753H: u32,
    pub LUT754L: u32,
    pub LUT754H: u32,
    pub LUT755L: u32,
    pub LUT755H: u32,
    pub LUT756L: u32,
    pub LUT756H: u32,
    pub LUT757L: u32,
    pub LUT757H: u32,
    pub LUT758L: u32,
    pub LUT758H: u32,
    pub LUT759L: u32,
    pub LUT759H: u32,
    pub LUT760L: u32,
    pub LUT760H: u32,
    pub LUT761L: u32,
    pub LUT761H: u32,
    pub LUT762L: u32,
    pub LUT762H: u32,
    pub LUT763L: u32,
    pub LUT763H: u32,
    pub LUT764L: u32,
    pub LUT764H: u32,
    pub LUT765L: u32,
    pub LUT765H: u32,
    pub LUT766L: u32,
    pub LUT766H: u32,
    pub LUT767L: u32,
    pub LUT767H: u32,
    pub LUT768L: u32,
    pub LUT768H: u32,
    pub LUT769L: u32,
    pub LUT769H: u32,
    pub LUT770L: u32,
    pub LUT770H: u32,
    pub LUT771L: u32,
    pub LUT771H: u32,
    pub LUT772L: u32,
    pub LUT772H: u32,
    pub LUT773L: u32,
    pub LUT773H: u32,
    pub LUT774L: u32,
    pub LUT774H: u32,
    pub LUT775L: u32,
    pub LUT775H: u32,
    pub LUT776L: u32,
    pub LUT776H: u32,
    pub LUT777L: u32,
    pub LUT777H: u32,
    pub LUT778L: u32,
    pub LUT778H: u32,
    pub LUT779L: u32,
    pub LUT779H: u32,
    pub LUT780L: u32,
    pub LUT780H: u32,
    pub LUT781L: u32,
    pub LUT781H: u32,
    pub LUT782L: u32,
    pub LUT782H: u32,
    pub LUT783L: u32,
    pub LUT783H: u32,
    pub LUT784L: u32,
    pub LUT784H: u32,
    pub LUT785L: u32,
    pub LUT785H: u32,
    pub LUT786L: u32,
    pub LUT786H: u32,
    pub LUT787L: u32,
    pub LUT787H: u32,
    pub LUT788L: u32,
    pub LUT788H: u32,
    pub LUT789L: u32,
    pub LUT789H: u32,
    pub LUT790L: u32,
    pub LUT790H: u32,
    pub LUT791L: u32,
    pub LUT791H: u32,
    pub LUT792L: u32,
    pub LUT792H: u32,
    pub LUT793L: u32,
    pub LUT793H: u32,
    pub LUT794L: u32,
    pub LUT794H: u32,
    pub LUT795L: u32,
    pub LUT795H: u32,
    pub LUT796L: u32,
    pub LUT796H: u32,
    pub LUT797L: u32,
    pub LUT797H: u32,
    pub LUT798L: u32,
    pub LUT798H: u32,
    pub LUT799L: u32,
    pub LUT799H: u32,
    pub LUT800L: u32,
    pub LUT800H: u32,
    pub LUT801L: u32,
    pub LUT801H: u32,
    pub LUT802L: u32,
    pub LUT802H: u32,
    pub LUT803L: u32,
    pub LUT803H: u32,
    pub LUT804L: u32,
    pub LUT804H: u32,
    pub LUT805L: u32,
    pub LUT805H: u32,
    pub LUT806L: u32,
    pub LUT806H: u32,
    pub LUT807L: u32,
    pub LUT807H: u32,
    pub LUT808L: u32,
    pub LUT808H: u32,
    pub LUT809L: u32,
    pub LUT809H: u32,
    pub LUT810L: u32,
    pub LUT810H: u32,
    pub LUT811L: u32,
    pub LUT811H: u32,
    pub LUT812L: u32,
    pub LUT812H: u32,
    pub LUT813L: u32,
    pub LUT813H: u32,
    pub LUT814L: u32,
    pub LUT814H: u32,
    pub LUT815L: u32,
    pub LUT815H: u32,
    pub LUT816L: u32,
    pub LUT816H: u32,
    pub LUT817L: u32,
    pub LUT817H: u32,
    pub LUT818L: u32,
    pub LUT818H: u32,
    pub LUT819L: u32,
    pub LUT819H: u32,
    pub LUT820L: u32,
    pub LUT820H: u32,
    pub LUT821L: u32,
    pub LUT821H: u32,
    pub LUT822L: u32,
    pub LUT822H: u32,
    pub LUT823L: u32,
    pub LUT823H: u32,
    pub LUT824L: u32,
    pub LUT824H: u32,
    pub LUT825L: u32,
    pub LUT825H: u32,
    pub LUT826L: u32,
    pub LUT826H: u32,
    pub LUT827L: u32,
    pub LUT827H: u32,
    pub LUT828L: u32,
    pub LUT828H: u32,
    pub LUT829L: u32,
    pub LUT829H: u32,
    pub LUT830L: u32,
    pub LUT830H: u32,
    pub LUT831L: u32,
    pub LUT831H: u32,
    pub LUT832L: u32,
    pub LUT832H: u32,
    pub LUT833L: u32,
    pub LUT833H: u32,
    pub LUT834L: u32,
    pub LUT834H: u32,
    pub LUT835L: u32,
    pub LUT835H: u32,
    pub LUT836L: u32,
    pub LUT836H: u32,
    pub LUT837L: u32,
    pub LUT837H: u32,
    pub LUT838L: u32,
    pub LUT838H: u32,
    pub LUT839L: u32,
    pub LUT839H: u32,
    pub LUT840L: u32,
    pub LUT840H: u32,
    pub LUT841L: u32,
    pub LUT841H: u32,
    pub LUT842L: u32,
    pub LUT842H: u32,
    pub LUT843L: u32,
    pub LUT843H: u32,
    pub LUT844L: u32,
    pub LUT844H: u32,
    pub LUT845L: u32,
    pub LUT845H: u32,
    pub LUT846L: u32,
    pub LUT846H: u32,
    pub LUT847L: u32,
    pub LUT847H: u32,
    pub LUT848L: u32,
    pub LUT848H: u32,
    pub LUT849L: u32,
    pub LUT849H: u32,
    pub LUT850L: u32,
    pub LUT850H: u32,
    pub LUT851L: u32,
    pub LUT851H: u32,
    pub LUT852L: u32,
    pub LUT852H: u32,
    pub LUT853L: u32,
    pub LUT853H: u32,
    pub LUT854L: u32,
    pub LUT854H: u32,
    pub LUT855L: u32,
    pub LUT855H: u32,
    pub LUT856L: u32,
    pub LUT856H: u32,
    pub LUT857L: u32,
    pub LUT857H: u32,
    pub LUT858L: u32,
    pub LUT858H: u32,
    pub LUT859L: u32,
    pub LUT859H: u32,
    pub LUT860L: u32,
    pub LUT860H: u32,
    pub LUT861L: u32,
    pub LUT861H: u32,
    pub LUT862L: u32,
    pub LUT862H: u32,
    pub LUT863L: u32,
    pub LUT863H: u32,
    pub LUT864L: u32,
    pub LUT864H: u32,
    pub LUT865L: u32,
    pub LUT865H: u32,
    pub LUT866L: u32,
    pub LUT866H: u32,
    pub LUT867L: u32,
    pub LUT867H: u32,
    pub LUT868L: u32,
    pub LUT868H: u32,
    pub LUT869L: u32,
    pub LUT869H: u32,
    pub LUT870L: u32,
    pub LUT870H: u32,
    pub LUT871L: u32,
    pub LUT871H: u32,
    pub LUT872L: u32,
    pub LUT872H: u32,
    pub LUT873L: u32,
    pub LUT873H: u32,
    pub LUT874L: u32,
    pub LUT874H: u32,
    pub LUT875L: u32,
    pub LUT875H: u32,
    pub LUT876L: u32,
    pub LUT876H: u32,
    pub LUT877L: u32,
    pub LUT877H: u32,
    pub LUT878L: u32,
    pub LUT878H: u32,
    pub LUT879L: u32,
    pub LUT879H: u32,
    pub LUT880L: u32,
    pub LUT880H: u32,
    pub LUT881L: u32,
    pub LUT881H: u32,
    pub LUT882L: u32,
    pub LUT882H: u32,
    pub LUT883L: u32,
    pub LUT883H: u32,
    pub LUT884L: u32,
    pub LUT884H: u32,
    pub LUT885L: u32,
    pub LUT885H: u32,
    pub LUT886L: u32,
    pub LUT886H: u32,
    pub LUT887L: u32,
    pub LUT887H: u32,
    pub LUT888L: u32,
    pub LUT888H: u32,
    pub LUT889L: u32,
    pub LUT889H: u32,
    pub LUT890L: u32,
    pub LUT890H: u32,
    pub LUT891L: u32,
    pub LUT891H: u32,
    pub LUT892L: u32,
    pub LUT892H: u32,
    pub LUT893L: u32,
    pub LUT893H: u32,
    pub LUT894L: u32,
    pub LUT894H: u32,
    pub LUT895L: u32,
    pub LUT895H: u32,
    pub LUT896L: u32,
    pub LUT896H: u32,
    pub LUT897L: u32,
    pub LUT897H: u32,
    pub LUT898L: u32,
    pub LUT898H: u32,
    pub LUT899L: u32,
    pub LUT899H: u32,
    pub LUT900L: u32,
    pub LUT900H: u32,
    pub LUT901L: u32,
    pub LUT901H: u32,
    pub LUT902L: u32,
    pub LUT902H: u32,
    pub LUT903L: u32,
    pub LUT903H: u32,
    pub LUT904L: u32,
    pub LUT904H: u32,
    pub LUT905L: u32,
    pub LUT905H: u32,
    pub LUT906L: u32,
    pub LUT906H: u32,
    pub LUT907L: u32,
    pub LUT907H: u32,
    pub LUT908L: u32,
    pub LUT908H: u32,
    pub LUT909L: u32,
    pub LUT909H: u32,
    pub LUT910L: u32,
    pub LUT910H: u32,
    pub LUT911L: u32,
    pub LUT911H: u32,
    pub LUT912L: u32,
    pub LUT912H: u32,
    pub LUT913L: u32,
    pub LUT913H: u32,
    pub LUT914L: u32,
    pub LUT914H: u32,
    pub LUT915L: u32,
    pub LUT915H: u32,
    pub LUT916L: u32,
    pub LUT916H: u32,
    pub LUT917L: u32,
    pub LUT917H: u32,
    pub LUT918L: u32,
    pub LUT918H: u32,
    pub LUT919L: u32,
    pub LUT919H: u32,
    pub LUT920L: u32,
    pub LUT920H: u32,
    pub LUT921L: u32,
    pub LUT921H: u32,
    pub LUT922L: u32,
    pub LUT922H: u32,
    pub LUT923L: u32,
    pub LUT923H: u32,
    pub LUT924L: u32,
    pub LUT924H: u32,
    pub LUT925L: u32,
    pub LUT925H: u32,
    pub LUT926L: u32,
    pub LUT926H: u32,
    pub LUT927L: u32,
    pub LUT927H: u32,
    pub LUT928L: u32,
    pub LUT928H: u32,
    pub LUT929L: u32,
    pub LUT929H: u32,
    pub LUT930L: u32,
    pub LUT930H: u32,
    pub LUT931L: u32,
    pub LUT931H: u32,
    pub LUT932L: u32,
    pub LUT932H: u32,
    pub LUT933L: u32,
    pub LUT933H: u32,
    pub LUT934L: u32,
    pub LUT934H: u32,
    pub LUT935L: u32,
    pub LUT935H: u32,
    pub LUT936L: u32,
    pub LUT936H: u32,
    pub LUT937L: u32,
    pub LUT937H: u32,
    pub LUT938L: u32,
    pub LUT938H: u32,
    pub LUT939L: u32,
    pub LUT939H: u32,
    pub LUT940L: u32,
    pub LUT940H: u32,
    pub LUT941L: u32,
    pub LUT941H: u32,
    pub LUT942L: u32,
    pub LUT942H: u32,
    pub LUT943L: u32,
    pub LUT943H: u32,
    pub LUT944L: u32,
    pub LUT944H: u32,
    pub LUT945L: u32,
    pub LUT945H: u32,
    pub LUT946L: u32,
    pub LUT946H: u32,
    pub LUT947L: u32,
    pub LUT947H: u32,
    pub LUT948L: u32,
    pub LUT948H: u32,
    pub LUT949L: u32,
    pub LUT949H: u32,
    pub LUT950L: u32,
    pub LUT950H: u32,
    pub LUT951L: u32,
    pub LUT951H: u32,
    pub LUT952L: u32,
    pub LUT952H: u32,
    pub LUT953L: u32,
    pub LUT953H: u32,
    pub LUT954L: u32,
    pub LUT954H: u32,
    pub LUT955L: u32,
    pub LUT955H: u32,
    pub LUT956L: u32,
    pub LUT956H: u32,
    pub LUT957L: u32,
    pub LUT957H: u32,
    pub LUT958L: u32,
    pub LUT958H: u32,
    pub LUT959L: u32,
    pub LUT959H: u32,
    pub LUT960L: u32,
    pub LUT960H: u32,
    pub LUT961L: u32,
    pub LUT961H: u32,
    pub LUT962L: u32,
    pub LUT962H: u32,
    pub LUT963L: u32,
    pub LUT963H: u32,
    pub LUT964L: u32,
    pub LUT964H: u32,
    pub LUT965L: u32,
    pub LUT965H: u32,
    pub LUT966L: u32,
    pub LUT966H: u32,
    pub LUT967L: u32,
    pub LUT967H: u32,
    pub LUT968L: u32,
    pub LUT968H: u32,
    pub LUT969L: u32,
    pub LUT969H: u32,
    pub LUT970L: u32,
    pub LUT970H: u32,
    pub LUT971L: u32,
    pub LUT971H: u32,
    pub LUT972L: u32,
    pub LUT972H: u32,
    pub LUT973L: u32,
    pub LUT973H: u32,
    pub LUT974L: u32,
    pub LUT974H: u32,
    pub LUT975L: u32,
    pub LUT975H: u32,
    pub LUT976L: u32,
    pub LUT976H: u32,
    pub LUT977L: u32,
    pub LUT977H: u32,
    pub LUT978L: u32,
    pub LUT978H: u32,
    pub LUT979L: u32,
    pub LUT979H: u32,
    pub LUT980L: u32,
    pub LUT980H: u32,
    pub LUT981L: u32,
    pub LUT981H: u32,
    pub LUT982L: u32,
    pub LUT982H: u32,
    pub LUT983L: u32,
    pub LUT983H: u32,
    pub LUT984L: u32,
    pub LUT984H: u32,
    pub LUT985L: u32,
    pub LUT985H: u32,
    pub LUT986L: u32,
    pub LUT986H: u32,
    pub LUT987L: u32,
    pub LUT987H: u32,
    pub LUT988L: u32,
    pub LUT988H: u32,
    pub LUT989L: u32,
    pub LUT989H: u32,
    pub LUT990L: u32,
    pub LUT990H: u32,
    pub LUT991L: u32,
    pub LUT991H: u32,
    pub LUT992L: u32,
    pub LUT992H: u32,
    pub LUT993L: u32,
    pub LUT993H: u32,
    pub LUT994L: u32,
    pub LUT994H: u32,
    pub LUT995L: u32,
    pub LUT995H: u32,
    pub LUT996L: u32,
    pub LUT996H: u32,
    pub LUT997L: u32,
    pub LUT997H: u32,
    pub LUT998L: u32,
    pub LUT998H: u32,
    pub LUT999L: u32,
    pub LUT999H: u32,
    pub LUT1000L: u32,
    pub LUT1000H: u32,
    pub LUT1001L: u32,
    pub LUT1001H: u32,
    pub LUT1002L: u32,
    pub LUT1002H: u32,
    pub LUT1003L: u32,
    pub LUT1003H: u32,
    pub LUT1004L: u32,
    pub LUT1004H: u32,
    pub LUT1005L: u32,
    pub LUT1005H: u32,
    pub LUT1006L: u32,
    pub LUT1006H: u32,
    pub LUT1007L: u32,
    pub LUT1007H: u32,
    pub LUT1008L: u32,
    pub LUT1008H: u32,
    pub LUT1009L: u32,
    pub LUT1009H: u32,
    pub LUT1010L: u32,
    pub LUT1010H: u32,
    pub LUT1011L: u32,
    pub LUT1011H: u32,
    pub LUT1012L: u32,
    pub LUT1012H: u32,
    pub LUT1013L: u32,
    pub LUT1013H: u32,
    pub LUT1014L: u32,
    pub LUT1014H: u32,
    pub LUT1015L: u32,
    pub LUT1015H: u32,
    pub LUT1016L: u32,
    pub LUT1016H: u32,
    pub LUT1017L: u32,
    pub LUT1017H: u32,
    pub LUT1018L: u32,
    pub LUT1018H: u32,
    pub LUT1019L: u32,
    pub LUT1019H: u32,
    pub LUT1020L: u32,
    pub LUT1020H: u32,
    pub LUT1021L: u32,
    pub LUT1021H: u32,
    pub LUT1022L: u32,
    pub LUT1022H: u32,
    pub LUT1023L: u32,
    pub LUT1023H: u32,
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
