#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! BSEC2
//!
//! Used by: stm32mp153, stm32mp157

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// BSEC OTP configuration register
pub mod BSEC_OTP_CONFIG {

    /// PWRUP
    pub mod PWRUP {
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

    /// FRC
    pub mod FRC {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (2 bits: 0b11 << 1)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PRGWIDTH
    pub mod PRGWIDTH {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (4 bits: 0b1111 << 3)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TREAD
    pub mod TREAD {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (2 bits: 0b11 << 7)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// BSEC OTP control register
pub mod BSEC_OTP_CONTROL {

    /// ADDR
    pub mod ADDR {
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

    /// PROG
    pub mod PROG {
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

    /// LOCK
    pub mod LOCK {
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
}

/// BSEC OTP write data register
pub mod BSEC_OTP_WRDATA {

    /// WRDATA
    pub mod WRDATA {
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

/// BSEC OTP status register
pub mod BSEC_OTP_STATUS {

    /// SECURE
    pub mod SECURE {
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

    /// FULLDBG
    pub mod FULLDBG {
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

    /// INVALID
    pub mod INVALID {
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

    /// BUSY
    pub mod BUSY {
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

    /// PROGFAIL
    pub mod PROGFAIL {
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

    /// PWRON
    pub mod PWRON {
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

    /// BIST1LOCK
    pub mod BIST1LOCK {
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

    /// BIST2LOCK
    pub mod BIST2LOCK {
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
}

/// BSEC OTP lock configuration register
pub mod BSEC_OTP_LOCK {

    /// OTP
    pub mod OTP {
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

    /// ROMLOCK
    pub mod ROMLOCK {
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

    /// DENREG
    pub mod DENREG {
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

    /// GPLOCK
    pub mod GPLOCK {
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

/// reset value depends on OTP secure mode according toTable18: BSEC_DENABLE default values after reset on page181.
pub mod BSEC_DENABLE {

    /// DFTEN
    pub mod DFTEN {
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

    /// DBGEN
    pub mod DBGEN {
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

    /// NIDEN
    pub mod NIDEN {
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

    /// DEVICEEN
    pub mod DEVICEEN {
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

    /// HDPEN
    pub mod HDPEN {
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

    /// SPIDEN
    pub mod SPIDEN {
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

    /// SPNIDEN
    pub mod SPNIDEN {
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

    /// CP15SDISABLE
    pub mod CP15SDISABLE {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (2 bits: 0b11 << 7)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CFGSDISABLE
    pub mod CFGSDISABLE {
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

    /// DBGSWENABLE
    pub mod DBGSWENABLE {
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
}

/// BSEC_OTP_DISTURBED0 is used to report disturbed state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP). BSEC_OTP_DISTURBED1 is used to report disturbed state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_DISTURBED2 is used to report disturbed state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95.
pub mod BSEC_OTP_DISTURBED0 {

    /// DIS
    pub mod DIS {
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

/// BSEC_OTP_DISTURBED0 is used to report disturbed state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP). BSEC_OTP_DISTURBED1 is used to report disturbed state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_DISTURBED2 is used to report disturbed state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95.
pub mod BSEC_OTP_DISTURBED1 {
    pub use super::BSEC_OTP_DISTURBED0::DIS;
}

/// BSEC_OTP_DISTURBED0 is used to report disturbed state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP). BSEC_OTP_DISTURBED1 is used to report disturbed state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_DISTURBED2 is used to report disturbed state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95.
pub mod BSEC_OTP_DISTURBED2 {
    pub use super::BSEC_OTP_DISTURBED0::DIS;
}

/// BSEC_OTP_ERROR0 is used to report error state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP which are protected by 2:1 redundancy). BSEC_OTP_ERROR1 is used to report error state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 which are protected by 6-bit ECC. BSEC_OTP_ERROR2 is used to report error state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 which are protected by 6-bit ECC.
pub mod BSEC_OTP_ERROR0 {

    /// ERR
    pub mod ERR {
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

/// BSEC_OTP_ERROR0 is used to report error state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP which are protected by 2:1 redundancy). BSEC_OTP_ERROR1 is used to report error state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 which are protected by 6-bit ECC. BSEC_OTP_ERROR2 is used to report error state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 which are protected by 6-bit ECC.
pub mod BSEC_OTP_ERROR1 {
    pub use super::BSEC_OTP_ERROR0::ERR;
}

/// BSEC_OTP_ERROR0 is used to report error state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP which are protected by 2:1 redundancy). BSEC_OTP_ERROR1 is used to report error state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 which are protected by 6-bit ECC. BSEC_OTP_ERROR2 is used to report error state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 which are protected by 6-bit ECC.
pub mod BSEC_OTP_ERROR2 {
    pub use super::BSEC_OTP_ERROR0::ERR;
}

/// BSEC_OTP_WLOCK0 is used to report permanent write lock of BSEC_OTP_DATA0 to BSEC_OTP_DATA31. BSEC_OTP_WLOCK1 is used to report permanent write lock of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_WLOCK2 is used to report permanent write lock of BSEC_OTP_DATA64 to BSEC_OTP_DATA95. Permanent write lock requires a programming sequence to lock a word (see section:Section3.3.6: OTP operations on page178).
pub mod BSEC_OTP_WRLOCK0 {

    /// WRLOCK
    pub mod WRLOCK {
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

/// BSEC_OTP_WLOCK0 is used to report permanent write lock of BSEC_OTP_DATA0 to BSEC_OTP_DATA31. BSEC_OTP_WLOCK1 is used to report permanent write lock of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_WLOCK2 is used to report permanent write lock of BSEC_OTP_DATA64 to BSEC_OTP_DATA95. Permanent write lock requires a programming sequence to lock a word (see section:Section3.3.6: OTP operations on page178).
pub mod BSEC_OTP_WRLOCK1 {
    pub use super::BSEC_OTP_WRLOCK0::WRLOCK;
}

/// BSEC_OTP_WLOCK0 is used to report permanent write lock of BSEC_OTP_DATA0 to BSEC_OTP_DATA31. BSEC_OTP_WLOCK1 is used to report permanent write lock of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_WLOCK2 is used to report permanent write lock of BSEC_OTP_DATA64 to BSEC_OTP_DATA95. Permanent write lock requires a programming sequence to lock a word (see section:Section3.3.6: OTP operations on page178).
pub mod BSEC_OTP_WRLOCK2 {
    pub use super::BSEC_OTP_WRLOCK0::WRLOCK;
}

/// BSEC_OTP_SPLOCK0 is used to lock the programming of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset BSEC_OTP_SPLOCK1 is used to lock the programming of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset BSEC_OTP_SPLOCK2 is used to lock the programming of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset Attempt to sticky program locked OTP word are silently ignored.
pub mod BSEC_OTP_SPLOCK0 {

    /// SPLOCK
    pub mod SPLOCK {
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

/// BSEC_OTP_SPLOCK0 is used to lock the programming of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset BSEC_OTP_SPLOCK1 is used to lock the programming of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset BSEC_OTP_SPLOCK2 is used to lock the programming of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset Attempt to sticky program locked OTP word are silently ignored.
pub mod BSEC_OTP_SPLOCK1 {
    pub use super::BSEC_OTP_SPLOCK0::SPLOCK;
}

/// BSEC_OTP_SPLOCK0 is used to lock the programming of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset BSEC_OTP_SPLOCK1 is used to lock the programming of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset BSEC_OTP_SPLOCK2 is used to lock the programming of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset Attempt to sticky program locked OTP word are silently ignored.
pub mod BSEC_OTP_SPLOCK2 {
    pub use super::BSEC_OTP_SPLOCK0::SPLOCK;
}

/// BSEC_OTP_SWLOCK0 is used to prevent writing to BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SWLOCK1 is used to prevent writing to BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SWLOCK2 is used to prevent writing to BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Write to shadow write locked BSEC_OTP_DATA word are silently ignored. Writing to OTP word 0 shadow is always prevented.
pub mod BSEC_OTP_SWLOCK0 {

    /// SWLOCK
    pub mod SWLOCK {
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

/// BSEC_OTP_SWLOCK0 is used to prevent writing to BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SWLOCK1 is used to prevent writing to BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SWLOCK2 is used to prevent writing to BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Write to shadow write locked BSEC_OTP_DATA word are silently ignored. Writing to OTP word 0 shadow is always prevented.
pub mod BSEC_OTP_SWLOCK1 {
    pub use super::BSEC_OTP_SWLOCK0::SWLOCK;
}

/// BSEC_OTP_SWLOCK0 is used to prevent writing to BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SWLOCK1 is used to prevent writing to BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SWLOCK2 is used to prevent writing to BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Write to shadow write locked BSEC_OTP_DATA word are silently ignored. Writing to OTP word 0 shadow is always prevented.
pub mod BSEC_OTP_SWLOCK2 {
    pub use super::BSEC_OTP_SWLOCK0::SWLOCK;
}

/// BSEC_OTP_SRLOCK0 is used to prevent reloading of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SRLOCK1 is used to prevent reloading of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SRLOCK2 is used to prevent reloading of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Setting SRLOCK bits or attempt to reload a locked OTP do not clear the corresponding BSEC_OTP_DATAx shadow register. BSEC_OTP_SRLOCK0 bit 0 is controlled by hardware according to fuse_ok, writing to this bit has no effect.
pub mod BSEC_OTP_SRLOCK0 {

    /// SRLOCK
    pub mod SRLOCK {
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

/// BSEC_OTP_SRLOCK0 is used to prevent reloading of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SRLOCK1 is used to prevent reloading of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SRLOCK2 is used to prevent reloading of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Setting SRLOCK bits or attempt to reload a locked OTP do not clear the corresponding BSEC_OTP_DATAx shadow register. BSEC_OTP_SRLOCK0 bit 0 is controlled by hardware according to fuse_ok, writing to this bit has no effect.
pub mod BSEC_OTP_SRLOCK1 {
    pub use super::BSEC_OTP_SRLOCK0::SRLOCK;
}

/// BSEC_OTP_SRLOCK0 is used to prevent reloading of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SRLOCK1 is used to prevent reloading of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SRLOCK2 is used to prevent reloading of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Setting SRLOCK bits or attempt to reload a locked OTP do not clear the corresponding BSEC_OTP_DATAx shadow register. BSEC_OTP_SRLOCK0 bit 0 is controlled by hardware according to fuse_ok, writing to this bit has no effect.
pub mod BSEC_OTP_SRLOCK2 {
    pub use super::BSEC_OTP_SRLOCK0::SRLOCK;
}

/// BSEC JTAG input register
pub mod BSEC_JTAGIN {

    /// DATA
    pub mod DATA {
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
}

/// BSEC JTAG output register
pub mod BSEC_JTAGOUT {

    /// DATA
    pub mod DATA {
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
}

/// BSEC scratch register
pub mod BSEC_SCRATCH {

    /// DATA
    pub mod DATA {
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

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA0 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA1 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA2 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA3 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA4 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA5 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA6 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA7 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA8 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA9 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA10 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA11 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA12 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA13 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA14 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA15 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA16 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA17 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA18 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA19 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA20 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA21 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA22 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA23 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA24 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA25 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA26 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA27 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA28 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA29 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA30 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA31 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA32 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA33 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA34 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA35 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA36 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA37 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA38 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA39 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA40 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA41 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA42 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA43 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA44 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA45 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA46 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA47 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA48 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA49 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA50 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA51 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA52 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA53 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA54 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA55 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA56 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA57 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA58 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA59 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA60 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA61 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA62 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA63 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA64 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA65 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA66 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA67 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA68 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA69 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA70 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA71 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA72 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA73 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA74 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA75 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA76 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA77 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA78 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA79 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA80 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA81 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA82 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA83 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA84 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA85 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA86 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA87 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA88 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA89 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA90 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA91 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA92 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA93 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA94 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
pub mod BSEC_OTP_DATA95 {
    pub use super::BSEC_SCRATCH::DATA;
}

/// BSEC hardware configuration register
pub mod BSEC_HWCFGR {

    /// SIZE
    pub mod SIZE {
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

    /// ECC_USE
    pub mod ECC_USE {
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

/// BSEC version register
pub mod BSEC_VERR {

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

/// BSEC identification register
pub mod BSEC_IPIDR {

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

/// BSEC size identification register
pub mod BSEC_SIDR {

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
    /// BSEC OTP configuration register
    pub BSEC_OTP_CONFIG: RWRegister<u32>,

    /// BSEC OTP control register
    pub BSEC_OTP_CONTROL: RWRegister<u32>,

    /// BSEC OTP write data register
    pub BSEC_OTP_WRDATA: RWRegister<u32>,

    /// BSEC OTP status register
    pub BSEC_OTP_STATUS: RORegister<u32>,

    /// BSEC OTP lock configuration register
    pub BSEC_OTP_LOCK: RWRegister<u32>,

    /// reset value depends on OTP secure mode according toTable18: BSEC_DENABLE default values after reset on page181.
    pub BSEC_DENABLE: RWRegister<u32>,

    _reserved1: [u8; 4],

    /// BSEC_OTP_DISTURBED0 is used to report disturbed state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP). BSEC_OTP_DISTURBED1 is used to report disturbed state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_DISTURBED2 is used to report disturbed state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95.
    pub BSEC_OTP_DISTURBED0: RORegister<u32>,

    /// BSEC_OTP_DISTURBED0 is used to report disturbed state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP). BSEC_OTP_DISTURBED1 is used to report disturbed state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_DISTURBED2 is used to report disturbed state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95.
    pub BSEC_OTP_DISTURBED1: RORegister<u32>,

    /// BSEC_OTP_DISTURBED0 is used to report disturbed state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP). BSEC_OTP_DISTURBED1 is used to report disturbed state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_DISTURBED2 is used to report disturbed state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95.
    pub BSEC_OTP_DISTURBED2: RORegister<u32>,

    _reserved2: [u8; 12],

    /// BSEC_OTP_ERROR0 is used to report error state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP which are protected by 2:1 redundancy). BSEC_OTP_ERROR1 is used to report error state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 which are protected by 6-bit ECC. BSEC_OTP_ERROR2 is used to report error state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 which are protected by 6-bit ECC.
    pub BSEC_OTP_ERROR0: RORegister<u32>,

    /// BSEC_OTP_ERROR0 is used to report error state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP which are protected by 2:1 redundancy). BSEC_OTP_ERROR1 is used to report error state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 which are protected by 6-bit ECC. BSEC_OTP_ERROR2 is used to report error state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 which are protected by 6-bit ECC.
    pub BSEC_OTP_ERROR1: RORegister<u32>,

    /// BSEC_OTP_ERROR0 is used to report error state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP which are protected by 2:1 redundancy). BSEC_OTP_ERROR1 is used to report error state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 which are protected by 6-bit ECC. BSEC_OTP_ERROR2 is used to report error state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 which are protected by 6-bit ECC.
    pub BSEC_OTP_ERROR2: RORegister<u32>,

    _reserved3: [u8; 12],

    /// BSEC_OTP_WLOCK0 is used to report permanent write lock of BSEC_OTP_DATA0 to BSEC_OTP_DATA31. BSEC_OTP_WLOCK1 is used to report permanent write lock of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_WLOCK2 is used to report permanent write lock of BSEC_OTP_DATA64 to BSEC_OTP_DATA95. Permanent write lock requires a programming sequence to lock a word (see section:Section3.3.6: OTP operations on page178).
    pub BSEC_OTP_WRLOCK0: RORegister<u32>,

    /// BSEC_OTP_WLOCK0 is used to report permanent write lock of BSEC_OTP_DATA0 to BSEC_OTP_DATA31. BSEC_OTP_WLOCK1 is used to report permanent write lock of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_WLOCK2 is used to report permanent write lock of BSEC_OTP_DATA64 to BSEC_OTP_DATA95. Permanent write lock requires a programming sequence to lock a word (see section:Section3.3.6: OTP operations on page178).
    pub BSEC_OTP_WRLOCK1: RORegister<u32>,

    /// BSEC_OTP_WLOCK0 is used to report permanent write lock of BSEC_OTP_DATA0 to BSEC_OTP_DATA31. BSEC_OTP_WLOCK1 is used to report permanent write lock of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_WLOCK2 is used to report permanent write lock of BSEC_OTP_DATA64 to BSEC_OTP_DATA95. Permanent write lock requires a programming sequence to lock a word (see section:Section3.3.6: OTP operations on page178).
    pub BSEC_OTP_WRLOCK2: RORegister<u32>,

    _reserved4: [u8; 12],

    /// BSEC_OTP_SPLOCK0 is used to lock the programming of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset BSEC_OTP_SPLOCK1 is used to lock the programming of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset BSEC_OTP_SPLOCK2 is used to lock the programming of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset Attempt to sticky program locked OTP word are silently ignored.
    pub BSEC_OTP_SPLOCK0: RWRegister<u32>,

    /// BSEC_OTP_SPLOCK0 is used to lock the programming of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset BSEC_OTP_SPLOCK1 is used to lock the programming of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset BSEC_OTP_SPLOCK2 is used to lock the programming of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset Attempt to sticky program locked OTP word are silently ignored.
    pub BSEC_OTP_SPLOCK1: RWRegister<u32>,

    /// BSEC_OTP_SPLOCK0 is used to lock the programming of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset BSEC_OTP_SPLOCK1 is used to lock the programming of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset BSEC_OTP_SPLOCK2 is used to lock the programming of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset Attempt to sticky program locked OTP word are silently ignored.
    pub BSEC_OTP_SPLOCK2: RWRegister<u32>,

    _reserved5: [u8; 12],

    /// BSEC_OTP_SWLOCK0 is used to prevent writing to BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SWLOCK1 is used to prevent writing to BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SWLOCK2 is used to prevent writing to BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Write to shadow write locked BSEC_OTP_DATA word are silently ignored. Writing to OTP word 0 shadow is always prevented.
    pub BSEC_OTP_SWLOCK0: RWRegister<u32>,

    /// BSEC_OTP_SWLOCK0 is used to prevent writing to BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SWLOCK1 is used to prevent writing to BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SWLOCK2 is used to prevent writing to BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Write to shadow write locked BSEC_OTP_DATA word are silently ignored. Writing to OTP word 0 shadow is always prevented.
    pub BSEC_OTP_SWLOCK1: RWRegister<u32>,

    /// BSEC_OTP_SWLOCK0 is used to prevent writing to BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SWLOCK1 is used to prevent writing to BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SWLOCK2 is used to prevent writing to BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Write to shadow write locked BSEC_OTP_DATA word are silently ignored. Writing to OTP word 0 shadow is always prevented.
    pub BSEC_OTP_SWLOCK2: RWRegister<u32>,

    _reserved6: [u8; 12],

    /// BSEC_OTP_SRLOCK0 is used to prevent reloading of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SRLOCK1 is used to prevent reloading of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SRLOCK2 is used to prevent reloading of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Setting SRLOCK bits or attempt to reload a locked OTP do not clear the corresponding BSEC_OTP_DATAx shadow register. BSEC_OTP_SRLOCK0 bit 0 is controlled by hardware according to fuse_ok, writing to this bit has no effect.
    pub BSEC_OTP_SRLOCK0: RWRegister<u32>,

    /// BSEC_OTP_SRLOCK0 is used to prevent reloading of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SRLOCK1 is used to prevent reloading of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SRLOCK2 is used to prevent reloading of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Setting SRLOCK bits or attempt to reload a locked OTP do not clear the corresponding BSEC_OTP_DATAx shadow register. BSEC_OTP_SRLOCK0 bit 0 is controlled by hardware according to fuse_ok, writing to this bit has no effect.
    pub BSEC_OTP_SRLOCK1: RWRegister<u32>,

    /// BSEC_OTP_SRLOCK0 is used to prevent reloading of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SRLOCK1 is used to prevent reloading of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SRLOCK2 is used to prevent reloading of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Setting SRLOCK bits or attempt to reload a locked OTP do not clear the corresponding BSEC_OTP_DATAx shadow register. BSEC_OTP_SRLOCK0 bit 0 is controlled by hardware according to fuse_ok, writing to this bit has no effect.
    pub BSEC_OTP_SRLOCK2: RWRegister<u32>,

    _reserved7: [u8; 12],

    /// BSEC JTAG input register
    pub BSEC_JTAGIN: RORegister<u32>,

    /// BSEC JTAG output register
    pub BSEC_JTAGOUT: RWRegister<u32>,

    /// BSEC scratch register
    pub BSEC_SCRATCH: RWRegister<u32>,

    _reserved8: [u8; 328],

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA0: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA1: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA2: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA3: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA4: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA5: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA6: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA7: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA8: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA9: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA10: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA11: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA12: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA13: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA14: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA15: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA16: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA17: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA18: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA19: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA20: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA21: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA22: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA23: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA24: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA25: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA26: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA27: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA28: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA29: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA30: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA31: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA32: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA33: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA34: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA35: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA36: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA37: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA38: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA39: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA40: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA41: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA42: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA43: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA44: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA45: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA46: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA47: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA48: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA49: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA50: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA51: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA52: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA53: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA54: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA55: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA56: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA57: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA58: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA59: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA60: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA61: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA62: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA63: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA64: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA65: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA66: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA67: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA68: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA69: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA70: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA71: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA72: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA73: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA74: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA75: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA76: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA77: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA78: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA79: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA80: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA81: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA82: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA83: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA84: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA85: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA86: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA87: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA88: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA89: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA90: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA91: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA92: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA93: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA94: RWRegister<u32>,

    /// Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\] (see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
    pub BSEC_OTP_DATA95: RWRegister<u32>,

    _reserved9: [u8; 3184],

    /// BSEC hardware configuration register
    pub BSEC_HWCFGR: RORegister<u32>,

    /// BSEC version register
    pub BSEC_VERR: RORegister<u32>,

    /// BSEC identification register
    pub BSEC_IPIDR: RORegister<u32>,

    /// BSEC size identification register
    pub BSEC_SIDR: RORegister<u32>,
}
pub struct ResetValues {
    pub BSEC_OTP_CONFIG: u32,
    pub BSEC_OTP_CONTROL: u32,
    pub BSEC_OTP_WRDATA: u32,
    pub BSEC_OTP_STATUS: u32,
    pub BSEC_OTP_LOCK: u32,
    pub BSEC_DENABLE: u32,
    pub BSEC_OTP_DISTURBED0: u32,
    pub BSEC_OTP_DISTURBED1: u32,
    pub BSEC_OTP_DISTURBED2: u32,
    pub BSEC_OTP_ERROR0: u32,
    pub BSEC_OTP_ERROR1: u32,
    pub BSEC_OTP_ERROR2: u32,
    pub BSEC_OTP_WRLOCK0: u32,
    pub BSEC_OTP_WRLOCK1: u32,
    pub BSEC_OTP_WRLOCK2: u32,
    pub BSEC_OTP_SPLOCK0: u32,
    pub BSEC_OTP_SPLOCK1: u32,
    pub BSEC_OTP_SPLOCK2: u32,
    pub BSEC_OTP_SWLOCK0: u32,
    pub BSEC_OTP_SWLOCK1: u32,
    pub BSEC_OTP_SWLOCK2: u32,
    pub BSEC_OTP_SRLOCK0: u32,
    pub BSEC_OTP_SRLOCK1: u32,
    pub BSEC_OTP_SRLOCK2: u32,
    pub BSEC_JTAGIN: u32,
    pub BSEC_JTAGOUT: u32,
    pub BSEC_SCRATCH: u32,
    pub BSEC_OTP_DATA0: u32,
    pub BSEC_OTP_DATA1: u32,
    pub BSEC_OTP_DATA2: u32,
    pub BSEC_OTP_DATA3: u32,
    pub BSEC_OTP_DATA4: u32,
    pub BSEC_OTP_DATA5: u32,
    pub BSEC_OTP_DATA6: u32,
    pub BSEC_OTP_DATA7: u32,
    pub BSEC_OTP_DATA8: u32,
    pub BSEC_OTP_DATA9: u32,
    pub BSEC_OTP_DATA10: u32,
    pub BSEC_OTP_DATA11: u32,
    pub BSEC_OTP_DATA12: u32,
    pub BSEC_OTP_DATA13: u32,
    pub BSEC_OTP_DATA14: u32,
    pub BSEC_OTP_DATA15: u32,
    pub BSEC_OTP_DATA16: u32,
    pub BSEC_OTP_DATA17: u32,
    pub BSEC_OTP_DATA18: u32,
    pub BSEC_OTP_DATA19: u32,
    pub BSEC_OTP_DATA20: u32,
    pub BSEC_OTP_DATA21: u32,
    pub BSEC_OTP_DATA22: u32,
    pub BSEC_OTP_DATA23: u32,
    pub BSEC_OTP_DATA24: u32,
    pub BSEC_OTP_DATA25: u32,
    pub BSEC_OTP_DATA26: u32,
    pub BSEC_OTP_DATA27: u32,
    pub BSEC_OTP_DATA28: u32,
    pub BSEC_OTP_DATA29: u32,
    pub BSEC_OTP_DATA30: u32,
    pub BSEC_OTP_DATA31: u32,
    pub BSEC_OTP_DATA32: u32,
    pub BSEC_OTP_DATA33: u32,
    pub BSEC_OTP_DATA34: u32,
    pub BSEC_OTP_DATA35: u32,
    pub BSEC_OTP_DATA36: u32,
    pub BSEC_OTP_DATA37: u32,
    pub BSEC_OTP_DATA38: u32,
    pub BSEC_OTP_DATA39: u32,
    pub BSEC_OTP_DATA40: u32,
    pub BSEC_OTP_DATA41: u32,
    pub BSEC_OTP_DATA42: u32,
    pub BSEC_OTP_DATA43: u32,
    pub BSEC_OTP_DATA44: u32,
    pub BSEC_OTP_DATA45: u32,
    pub BSEC_OTP_DATA46: u32,
    pub BSEC_OTP_DATA47: u32,
    pub BSEC_OTP_DATA48: u32,
    pub BSEC_OTP_DATA49: u32,
    pub BSEC_OTP_DATA50: u32,
    pub BSEC_OTP_DATA51: u32,
    pub BSEC_OTP_DATA52: u32,
    pub BSEC_OTP_DATA53: u32,
    pub BSEC_OTP_DATA54: u32,
    pub BSEC_OTP_DATA55: u32,
    pub BSEC_OTP_DATA56: u32,
    pub BSEC_OTP_DATA57: u32,
    pub BSEC_OTP_DATA58: u32,
    pub BSEC_OTP_DATA59: u32,
    pub BSEC_OTP_DATA60: u32,
    pub BSEC_OTP_DATA61: u32,
    pub BSEC_OTP_DATA62: u32,
    pub BSEC_OTP_DATA63: u32,
    pub BSEC_OTP_DATA64: u32,
    pub BSEC_OTP_DATA65: u32,
    pub BSEC_OTP_DATA66: u32,
    pub BSEC_OTP_DATA67: u32,
    pub BSEC_OTP_DATA68: u32,
    pub BSEC_OTP_DATA69: u32,
    pub BSEC_OTP_DATA70: u32,
    pub BSEC_OTP_DATA71: u32,
    pub BSEC_OTP_DATA72: u32,
    pub BSEC_OTP_DATA73: u32,
    pub BSEC_OTP_DATA74: u32,
    pub BSEC_OTP_DATA75: u32,
    pub BSEC_OTP_DATA76: u32,
    pub BSEC_OTP_DATA77: u32,
    pub BSEC_OTP_DATA78: u32,
    pub BSEC_OTP_DATA79: u32,
    pub BSEC_OTP_DATA80: u32,
    pub BSEC_OTP_DATA81: u32,
    pub BSEC_OTP_DATA82: u32,
    pub BSEC_OTP_DATA83: u32,
    pub BSEC_OTP_DATA84: u32,
    pub BSEC_OTP_DATA85: u32,
    pub BSEC_OTP_DATA86: u32,
    pub BSEC_OTP_DATA87: u32,
    pub BSEC_OTP_DATA88: u32,
    pub BSEC_OTP_DATA89: u32,
    pub BSEC_OTP_DATA90: u32,
    pub BSEC_OTP_DATA91: u32,
    pub BSEC_OTP_DATA92: u32,
    pub BSEC_OTP_DATA93: u32,
    pub BSEC_OTP_DATA94: u32,
    pub BSEC_OTP_DATA95: u32,
    pub BSEC_HWCFGR: u32,
    pub BSEC_VERR: u32,
    pub BSEC_IPIDR: u32,
    pub BSEC_SIDR: u32,
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
