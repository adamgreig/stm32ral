#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! ETZPC
//!
//! Used by: stm32mp153, stm32mp157

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// ETZPC ROM secure size definition
pub mod ETZPC_TZMA0_SIZE {

    /// R0SIZE
    pub mod R0SIZE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (10 bits: 0x3ff << 0)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LOCK
    pub mod LOCK {
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

/// ETZPC RAM secure size definition
pub mod ETZPC_TZMA1_SIZE {
    pub use super::ETZPC_TZMA0_SIZE::LOCK;
    pub use super::ETZPC_TZMA0_SIZE::R0SIZE;
}

/// Register reset values
pub mod ETZPC_DECPROT0 {

    /// DECPROT0
    pub mod DECPROT0 {
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

    /// DECPROT1
    pub mod DECPROT1 {
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

    /// DECPROT2
    pub mod DECPROT2 {
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

    /// DECPROT3
    pub mod DECPROT3 {
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

    /// DECPROT4
    pub mod DECPROT4 {
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

    /// DECPROT5
    pub mod DECPROT5 {
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

    /// DECPROT6
    pub mod DECPROT6 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (2 bits: 0b11 << 12)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DECPROT7
    pub mod DECPROT7 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (2 bits: 0b11 << 14)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DECPROT8
    pub mod DECPROT8 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (2 bits: 0b11 << 16)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DECPROT9
    pub mod DECPROT9 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (2 bits: 0b11 << 18)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DECPROT10
    pub mod DECPROT10 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (2 bits: 0b11 << 20)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DECPROT11
    pub mod DECPROT11 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (2 bits: 0b11 << 22)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DECPROT12
    pub mod DECPROT12 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (2 bits: 0b11 << 24)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DECPROT13
    pub mod DECPROT13 {
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

    /// DECPROT14
    pub mod DECPROT14 {
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

    /// DECPROT15
    pub mod DECPROT15 {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (2 bits: 0b11 << 30)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Register reset values
pub mod ETZPC_DECPROT1 {
    pub use super::ETZPC_DECPROT0::DECPROT0;
    pub use super::ETZPC_DECPROT0::DECPROT1;
    pub use super::ETZPC_DECPROT0::DECPROT10;
    pub use super::ETZPC_DECPROT0::DECPROT11;
    pub use super::ETZPC_DECPROT0::DECPROT12;
    pub use super::ETZPC_DECPROT0::DECPROT13;
    pub use super::ETZPC_DECPROT0::DECPROT14;
    pub use super::ETZPC_DECPROT0::DECPROT15;
    pub use super::ETZPC_DECPROT0::DECPROT2;
    pub use super::ETZPC_DECPROT0::DECPROT3;
    pub use super::ETZPC_DECPROT0::DECPROT4;
    pub use super::ETZPC_DECPROT0::DECPROT5;
    pub use super::ETZPC_DECPROT0::DECPROT6;
    pub use super::ETZPC_DECPROT0::DECPROT7;
    pub use super::ETZPC_DECPROT0::DECPROT8;
    pub use super::ETZPC_DECPROT0::DECPROT9;
}

/// Register reset values
pub mod ETZPC_DECPROT2 {
    pub use super::ETZPC_DECPROT0::DECPROT0;
    pub use super::ETZPC_DECPROT0::DECPROT1;
    pub use super::ETZPC_DECPROT0::DECPROT10;
    pub use super::ETZPC_DECPROT0::DECPROT11;
    pub use super::ETZPC_DECPROT0::DECPROT12;
    pub use super::ETZPC_DECPROT0::DECPROT13;
    pub use super::ETZPC_DECPROT0::DECPROT14;
    pub use super::ETZPC_DECPROT0::DECPROT15;
    pub use super::ETZPC_DECPROT0::DECPROT2;
    pub use super::ETZPC_DECPROT0::DECPROT3;
    pub use super::ETZPC_DECPROT0::DECPROT4;
    pub use super::ETZPC_DECPROT0::DECPROT5;
    pub use super::ETZPC_DECPROT0::DECPROT6;
    pub use super::ETZPC_DECPROT0::DECPROT7;
    pub use super::ETZPC_DECPROT0::DECPROT8;
    pub use super::ETZPC_DECPROT0::DECPROT9;
}

/// Register reset values
pub mod ETZPC_DECPROT3 {
    pub use super::ETZPC_DECPROT0::DECPROT0;
    pub use super::ETZPC_DECPROT0::DECPROT1;
    pub use super::ETZPC_DECPROT0::DECPROT10;
    pub use super::ETZPC_DECPROT0::DECPROT11;
    pub use super::ETZPC_DECPROT0::DECPROT12;
    pub use super::ETZPC_DECPROT0::DECPROT13;
    pub use super::ETZPC_DECPROT0::DECPROT14;
    pub use super::ETZPC_DECPROT0::DECPROT15;
    pub use super::ETZPC_DECPROT0::DECPROT2;
    pub use super::ETZPC_DECPROT0::DECPROT3;
    pub use super::ETZPC_DECPROT0::DECPROT4;
    pub use super::ETZPC_DECPROT0::DECPROT5;
    pub use super::ETZPC_DECPROT0::DECPROT6;
    pub use super::ETZPC_DECPROT0::DECPROT7;
    pub use super::ETZPC_DECPROT0::DECPROT8;
    pub use super::ETZPC_DECPROT0::DECPROT9;
}

/// Register reset values
pub mod ETZPC_DECPROT4 {
    pub use super::ETZPC_DECPROT0::DECPROT0;
    pub use super::ETZPC_DECPROT0::DECPROT1;
    pub use super::ETZPC_DECPROT0::DECPROT10;
    pub use super::ETZPC_DECPROT0::DECPROT11;
    pub use super::ETZPC_DECPROT0::DECPROT12;
    pub use super::ETZPC_DECPROT0::DECPROT13;
    pub use super::ETZPC_DECPROT0::DECPROT14;
    pub use super::ETZPC_DECPROT0::DECPROT15;
    pub use super::ETZPC_DECPROT0::DECPROT2;
    pub use super::ETZPC_DECPROT0::DECPROT3;
    pub use super::ETZPC_DECPROT0::DECPROT4;
    pub use super::ETZPC_DECPROT0::DECPROT5;
    pub use super::ETZPC_DECPROT0::DECPROT6;
    pub use super::ETZPC_DECPROT0::DECPROT7;
    pub use super::ETZPC_DECPROT0::DECPROT8;
    pub use super::ETZPC_DECPROT0::DECPROT9;
}

/// Register reset values
pub mod ETZPC_DECPROT5 {
    pub use super::ETZPC_DECPROT0::DECPROT0;
    pub use super::ETZPC_DECPROT0::DECPROT1;
    pub use super::ETZPC_DECPROT0::DECPROT10;
    pub use super::ETZPC_DECPROT0::DECPROT11;
    pub use super::ETZPC_DECPROT0::DECPROT12;
    pub use super::ETZPC_DECPROT0::DECPROT13;
    pub use super::ETZPC_DECPROT0::DECPROT14;
    pub use super::ETZPC_DECPROT0::DECPROT15;
    pub use super::ETZPC_DECPROT0::DECPROT2;
    pub use super::ETZPC_DECPROT0::DECPROT3;
    pub use super::ETZPC_DECPROT0::DECPROT4;
    pub use super::ETZPC_DECPROT0::DECPROT5;
    pub use super::ETZPC_DECPROT0::DECPROT6;
    pub use super::ETZPC_DECPROT0::DECPROT7;
    pub use super::ETZPC_DECPROT0::DECPROT8;
    pub use super::ETZPC_DECPROT0::DECPROT9;
}

/// ETZPC decprot lock 0 register
pub mod ETZPC_DECPROT_LOCK0 {

    /// LOCK0
    pub mod LOCK0 {
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

    /// LOCK1
    pub mod LOCK1 {
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

    /// LOCK2
    pub mod LOCK2 {
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

    /// LOCK3
    pub mod LOCK3 {
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

    /// LOCK4
    pub mod LOCK4 {
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

    /// LOCK5
    pub mod LOCK5 {
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

    /// LOCK6
    pub mod LOCK6 {
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

    /// LOCK7
    pub mod LOCK7 {
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

    /// LOCK8
    pub mod LOCK8 {
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

    /// LOCK9
    pub mod LOCK9 {
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

    /// LOCK10
    pub mod LOCK10 {
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

    /// LOCK11
    pub mod LOCK11 {
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

    /// LOCK12
    pub mod LOCK12 {
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

    /// LOCK13
    pub mod LOCK13 {
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

    /// LOCK14
    pub mod LOCK14 {
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

    /// LOCK15
    pub mod LOCK15 {
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

    /// LOCK16
    pub mod LOCK16 {
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

    /// LOCK17
    pub mod LOCK17 {
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

    /// LOCK18
    pub mod LOCK18 {
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

    /// LOCK19
    pub mod LOCK19 {
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

    /// LOCK20
    pub mod LOCK20 {
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

    /// LOCK21
    pub mod LOCK21 {
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

    /// LOCK22
    pub mod LOCK22 {
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

    /// LOCK23
    pub mod LOCK23 {
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

    /// LOCK24
    pub mod LOCK24 {
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

    /// LOCK25
    pub mod LOCK25 {
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

    /// LOCK26
    pub mod LOCK26 {
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

    /// LOCK27
    pub mod LOCK27 {
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

    /// LOCK28
    pub mod LOCK28 {
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

    /// LOCK29
    pub mod LOCK29 {
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

    /// LOCK30
    pub mod LOCK30 {
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

    /// LOCK31
    pub mod LOCK31 {
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

/// ETZPC decprot lock 1 register
pub mod ETZPC_DECPROT_LOCK1 {
    pub use super::ETZPC_DECPROT_LOCK0::LOCK0;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK1;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK10;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK11;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK12;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK13;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK14;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK15;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK16;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK17;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK18;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK19;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK2;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK20;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK21;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK22;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK23;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK24;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK25;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK26;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK27;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK28;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK29;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK3;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK30;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK31;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK4;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK5;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK6;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK7;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK8;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK9;
}

/// ETZPC decprot lock 2 register
pub mod ETZPC_DECPROT_LOCK2 {
    pub use super::ETZPC_DECPROT_LOCK0::LOCK0;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK1;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK10;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK11;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK12;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK13;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK14;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK15;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK16;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK17;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK18;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK19;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK2;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK20;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK21;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK22;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK23;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK24;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK25;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK26;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK27;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK28;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK29;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK3;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK30;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK31;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK4;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK5;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK6;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK7;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK8;
    pub use super::ETZPC_DECPROT_LOCK0::LOCK9;
}

/// ETZPC IP HW configuration register
pub mod ETZPC_HWCFGR {

    /// NUM_TZMA
    pub mod NUM_TZMA {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// NUM_PER_SEC
    pub mod NUM_PER_SEC {
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

    /// NUM_AHB_SEC
    pub mod NUM_AHB_SEC {
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

    /// CHUNKS1N4
    pub mod CHUNKS1N4 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (8 bits: 0xff << 24)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// ETZPC IP version register
pub mod ETZPC_VERR {

    /// MINREV
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

    /// MAJREV
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

/// ETZPC IP version register
pub mod ETZPC_IDR {

    /// ID
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

/// ETZPC IP version register
pub mod ETZPC_SIDR {

    /// SID
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
#[repr(C)]
pub struct RegisterBlock {
    /// ETZPC ROM secure size definition
    pub ETZPC_TZMA0_SIZE: RWRegister<u32>,

    /// ETZPC RAM secure size definition
    pub ETZPC_TZMA1_SIZE: RWRegister<u32>,

    _reserved1: [u8; 8],

    /// Register reset values
    pub ETZPC_DECPROT0: RWRegister<u32>,

    /// Register reset values
    pub ETZPC_DECPROT1: RWRegister<u32>,

    /// Register reset values
    pub ETZPC_DECPROT2: RWRegister<u32>,

    /// Register reset values
    pub ETZPC_DECPROT3: RWRegister<u32>,

    /// Register reset values
    pub ETZPC_DECPROT4: RWRegister<u32>,

    /// Register reset values
    pub ETZPC_DECPROT5: RWRegister<u32>,

    _reserved2: [u8; 8],

    /// ETZPC decprot lock 0 register
    pub ETZPC_DECPROT_LOCK0: RWRegister<u32>,

    /// ETZPC decprot lock 1 register
    pub ETZPC_DECPROT_LOCK1: RWRegister<u32>,

    /// ETZPC decprot lock 2 register
    pub ETZPC_DECPROT_LOCK2: RWRegister<u32>,

    _reserved3: [u8; 948],

    /// ETZPC IP HW configuration register
    pub ETZPC_HWCFGR: RORegister<u32>,

    /// ETZPC IP version register
    pub ETZPC_VERR: RORegister<u32>,

    /// ETZPC IP version register
    pub ETZPC_IDR: RORegister<u32>,

    /// ETZPC IP version register
    pub ETZPC_SIDR: RORegister<u32>,
}
pub struct ResetValues {
    pub ETZPC_TZMA0_SIZE: u32,
    pub ETZPC_TZMA1_SIZE: u32,
    pub ETZPC_DECPROT0: u32,
    pub ETZPC_DECPROT1: u32,
    pub ETZPC_DECPROT2: u32,
    pub ETZPC_DECPROT3: u32,
    pub ETZPC_DECPROT4: u32,
    pub ETZPC_DECPROT5: u32,
    pub ETZPC_DECPROT_LOCK0: u32,
    pub ETZPC_DECPROT_LOCK1: u32,
    pub ETZPC_DECPROT_LOCK2: u32,
    pub ETZPC_HWCFGR: u32,
    pub ETZPC_VERR: u32,
    pub ETZPC_IDR: u32,
    pub ETZPC_SIDR: u32,
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
