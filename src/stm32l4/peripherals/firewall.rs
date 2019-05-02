#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Firewall
//!
//! Used by: stm32l4x1, stm32l4x2, stm32l4x3, stm32l4x5, stm32l4x6

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Code segment start address
pub mod CSSA {

    /// code segment start address
    pub mod ADD {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (16 bits: 0xffff << 8)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Code segment length
pub mod CSL {

    /// code segment length
    pub mod LENG {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (14 bits: 0x3fff << 8)
        pub const mask: u32 = 0x3fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Non-volatile data segment start address
pub mod NVDSSA {
    pub use super::CSSA::ADD;
}

/// Non-volatile data segment length
pub mod NVDSL {
    pub use super::CSL::LENG;
}

/// Volatile data segment start address
pub mod VDSSA {

    /// Volatile data segment start address
    pub mod ADD {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (10 bits: 0x3ff << 6)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Volatile data segment length
pub mod VDSL {

    /// Non-volatile data segment length
    pub mod LENG {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (10 bits: 0x3ff << 6)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Configuration register
pub mod CR {

    /// Volatile data execution
    pub mod VDE {
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

    /// Volatile data shared
    pub mod VDS {
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

    /// Firewall pre alarm
    pub mod FPA {
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
pub struct RegisterBlock {
    /// Code segment start address
    pub CSSA: RWRegister<u32>,

    /// Code segment length
    pub CSL: RWRegister<u32>,

    /// Non-volatile data segment start address
    pub NVDSSA: RWRegister<u32>,

    /// Non-volatile data segment length
    pub NVDSL: RWRegister<u32>,

    /// Volatile data segment start address
    pub VDSSA: RWRegister<u32>,

    /// Volatile data segment length
    pub VDSL: RWRegister<u32>,

    _reserved1: [u32; 2],

    /// Configuration register
    pub CR: RWRegister<u32>,
}
pub struct ResetValues {
    pub CSSA: u32,
    pub CSL: u32,
    pub NVDSSA: u32,
    pub NVDSL: u32,
    pub VDSSA: u32,
    pub VDSL: u32,
    pub CR: u32,
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
