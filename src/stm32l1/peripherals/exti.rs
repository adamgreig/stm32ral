#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! External interrupt/event controller
//!
//! Used by: stm32l151, stm32l162

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// IMR
pub mod IMR {

    /// Interrupt mask on line x
    pub mod MR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (23 bits: 0x7fffff << 0)
        pub const mask: u32 = 0x7fffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// EMR
pub mod EMR {
    pub use super::IMR::MR;
}

/// RTSR
pub mod RTSR {

    /// Rising edge trigger event configuration bit of line x
    pub mod TR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (23 bits: 0x7fffff << 0)
        pub const mask: u32 = 0x7fffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// FTSR
pub mod FTSR {
    pub use super::RTSR::TR;
}

/// SWIER
pub mod SWIER {

    /// Software interrupt on line x
    pub mod SWIER {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (23 bits: 0x7fffff << 0)
        pub const mask: u32 = 0x7fffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// PR
pub mod PR {

    /// Pending bit
    pub mod PR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (23 bits: 0x7fffff << 0)
        pub const mask: u32 = 0x7fffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}
pub struct RegisterBlock {
    /// IMR
    pub IMR: RWRegister<u32>,

    /// EMR
    pub EMR: RWRegister<u32>,

    /// RTSR
    pub RTSR: RWRegister<u32>,

    /// FTSR
    pub FTSR: RWRegister<u32>,

    /// SWIER
    pub SWIER: RWRegister<u32>,

    /// PR
    pub PR: RWRegister<u32>,
}
pub struct ResetValues {
    pub IMR: u32,
    pub EMR: u32,
    pub RTSR: u32,
    pub FTSR: u32,
    pub SWIER: u32,
    pub PR: u32,
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
