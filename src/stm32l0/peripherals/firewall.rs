#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Firewall
//!
//! Used by: stm32l0x2, stm32l0x3

#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;
use RWRegister;

/// Code segment start address
pub mod FIREWALL_CSSA {

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
pub mod FIREWALL_CSL {

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
pub mod FIREWALL_NVDSSA {
    pub use super::FIREWALL_CSSA::ADD;
}

/// Non-volatile data segment length
pub mod FIREWALL_NVDSL {
    pub use super::FIREWALL_CSL::LENG;
}

/// Volatile data segment start address
pub mod FIREWALL_VDSSA {

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
pub mod FIREWALL_VDSL {

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
pub mod FIREWALL_CR {

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
    pub FIREWALL_CSSA: RWRegister<u32>,

    /// Code segment length
    pub FIREWALL_CSL: RWRegister<u32>,

    /// Non-volatile data segment start address
    pub FIREWALL_NVDSSA: RWRegister<u32>,

    /// Non-volatile data segment length
    pub FIREWALL_NVDSL: RWRegister<u32>,

    /// Volatile data segment start address
    pub FIREWALL_VDSSA: RWRegister<u32>,

    /// Volatile data segment length
    pub FIREWALL_VDSL: RWRegister<u32>,

    _reserved1: [u32; 2],

    /// Configuration register
    pub FIREWALL_CR: RWRegister<u32>,
}
pub struct ResetValues {
    pub FIREWALL_CSSA: u32,
    pub FIREWALL_CSL: u32,
    pub FIREWALL_NVDSSA: u32,
    pub FIREWALL_NVDSL: u32,
    pub FIREWALL_VDSSA: u32,
    pub FIREWALL_VDSL: u32,
    pub FIREWALL_CR: u32,
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
