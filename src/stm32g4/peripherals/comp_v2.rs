#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Comparator control and status register
//!
//! Used by: stm32g473, stm32g474, stm32g483, stm32g484

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Comparator control/status register
pub mod COMP_C1CSR {

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

    /// COMP_DEGLITCH_EN
    pub mod COMP_DEGLITCH_EN {
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

    /// INMSEL
    pub mod INMSEL {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (3 bits: 0b111 << 4)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// INPSEL
    pub mod INPSEL {
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

    /// POL
    pub mod POL {
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

    /// HYST
    pub mod HYST {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (3 bits: 0b111 << 16)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// BLANKSEL
    pub mod BLANKSEL {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (3 bits: 0b111 << 19)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// BRGEN
    pub mod BRGEN {
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

    /// SCALEN
    pub mod SCALEN {
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

    /// VALUE
    pub mod VALUE {
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

/// Comparator control/status register
pub mod COMP_C2CSR {
    pub use super::COMP_C1CSR::BLANKSEL;
    pub use super::COMP_C1CSR::BRGEN;
    pub use super::COMP_C1CSR::COMP_DEGLITCH_EN;
    pub use super::COMP_C1CSR::EN;
    pub use super::COMP_C1CSR::HYST;
    pub use super::COMP_C1CSR::INMSEL;
    pub use super::COMP_C1CSR::INPSEL;
    pub use super::COMP_C1CSR::LOCK;
    pub use super::COMP_C1CSR::POL;
    pub use super::COMP_C1CSR::SCALEN;
    pub use super::COMP_C1CSR::VALUE;
}

/// Comparator control/status register
pub mod COMP_C3CSR {
    pub use super::COMP_C1CSR::BLANKSEL;
    pub use super::COMP_C1CSR::BRGEN;
    pub use super::COMP_C1CSR::COMP_DEGLITCH_EN;
    pub use super::COMP_C1CSR::EN;
    pub use super::COMP_C1CSR::HYST;
    pub use super::COMP_C1CSR::INMSEL;
    pub use super::COMP_C1CSR::INPSEL;
    pub use super::COMP_C1CSR::LOCK;
    pub use super::COMP_C1CSR::POL;
    pub use super::COMP_C1CSR::SCALEN;
    pub use super::COMP_C1CSR::VALUE;
}

/// Comparator control/status register
pub mod COMP_C4CSR {
    pub use super::COMP_C1CSR::BLANKSEL;
    pub use super::COMP_C1CSR::BRGEN;
    pub use super::COMP_C1CSR::COMP_DEGLITCH_EN;
    pub use super::COMP_C1CSR::EN;
    pub use super::COMP_C1CSR::HYST;
    pub use super::COMP_C1CSR::INMSEL;
    pub use super::COMP_C1CSR::INPSEL;
    pub use super::COMP_C1CSR::LOCK;
    pub use super::COMP_C1CSR::POL;
    pub use super::COMP_C1CSR::SCALEN;
    pub use super::COMP_C1CSR::VALUE;
}

/// Comparator control/status register
pub mod COMP_C5CSR {
    pub use super::COMP_C1CSR::BLANKSEL;
    pub use super::COMP_C1CSR::BRGEN;
    pub use super::COMP_C1CSR::COMP_DEGLITCH_EN;
    pub use super::COMP_C1CSR::EN;
    pub use super::COMP_C1CSR::HYST;
    pub use super::COMP_C1CSR::INMSEL;
    pub use super::COMP_C1CSR::INPSEL;
    pub use super::COMP_C1CSR::LOCK;
    pub use super::COMP_C1CSR::POL;
    pub use super::COMP_C1CSR::SCALEN;
    pub use super::COMP_C1CSR::VALUE;
}

/// Comparator control/status register
pub mod COMP_C6CSR {
    pub use super::COMP_C1CSR::BLANKSEL;
    pub use super::COMP_C1CSR::BRGEN;
    pub use super::COMP_C1CSR::COMP_DEGLITCH_EN;
    pub use super::COMP_C1CSR::EN;
    pub use super::COMP_C1CSR::HYST;
    pub use super::COMP_C1CSR::INMSEL;
    pub use super::COMP_C1CSR::INPSEL;
    pub use super::COMP_C1CSR::LOCK;
    pub use super::COMP_C1CSR::POL;
    pub use super::COMP_C1CSR::SCALEN;
    pub use super::COMP_C1CSR::VALUE;
}

/// Comparator control/status register
pub mod COMP_C7CSR {
    pub use super::COMP_C1CSR::BLANKSEL;
    pub use super::COMP_C1CSR::BRGEN;
    pub use super::COMP_C1CSR::COMP_DEGLITCH_EN;
    pub use super::COMP_C1CSR::EN;
    pub use super::COMP_C1CSR::HYST;
    pub use super::COMP_C1CSR::INMSEL;
    pub use super::COMP_C1CSR::INPSEL;
    pub use super::COMP_C1CSR::LOCK;
    pub use super::COMP_C1CSR::POL;
    pub use super::COMP_C1CSR::SCALEN;
    pub use super::COMP_C1CSR::VALUE;
}
#[repr(C)]
pub struct RegisterBlock {
    /// Comparator control/status register
    pub COMP_C1CSR: RWRegister<u32>,

    /// Comparator control/status register
    pub COMP_C2CSR: RWRegister<u32>,

    /// Comparator control/status register
    pub COMP_C3CSR: RWRegister<u32>,

    /// Comparator control/status register
    pub COMP_C4CSR: RWRegister<u32>,

    /// Comparator control/status register
    pub COMP_C5CSR: RWRegister<u32>,

    /// Comparator control/status register
    pub COMP_C6CSR: RWRegister<u32>,

    /// Comparator control/status register
    pub COMP_C7CSR: RWRegister<u32>,
}
pub struct ResetValues {
    pub COMP_C1CSR: u32,
    pub COMP_C2CSR: u32,
    pub COMP_C3CSR: u32,
    pub COMP_C4CSR: u32,
    pub COMP_C5CSR: u32,
    pub COMP_C6CSR: u32,
    pub COMP_C7CSR: u32,
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
