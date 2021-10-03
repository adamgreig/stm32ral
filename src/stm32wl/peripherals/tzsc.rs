#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Global TrustZone Controller
//!
//! Used by: stm32wl5x_cm0p, stm32wl5x_cm4

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// TZSC control register
pub mod TZSC_CR {

    /// LCK
    pub mod LCK {
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

/// TZSC security configuration register
pub mod TZSC_SECCFGR1 {

    /// AESSEC
    pub mod AESSEC {
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

    /// RNGSEC
    pub mod RNGSEC {
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

    /// PKASEC
    pub mod PKASEC {
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
}

/// TZSC privilege configuration register 1
pub mod TZSC_PRIVCFGR1 {

    /// AESPRIV
    pub mod AESPRIV {
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

    /// RNGPRIV
    pub mod RNGPRIV {
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

    /// SUBGHZSPIPRIV
    pub mod SUBGHZSPIPRIV {
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

    /// PKAPRIV
    pub mod PKAPRIV {
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
}

/// Unprivileged Water Mark 1 register
pub mod TZSC_MPCWM1_UPWMR {

    /// LGTH
    pub mod LGTH {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (12 bits: 0xfff << 16)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Unprivileged Writable Water Mark 1 register
pub mod TZSC_MPCWM1_UPWWMR {
    pub use super::TZSC_MPCWM1_UPWMR::LGTH;
}

/// Unprivileged Water Mark 2 register
pub mod TZSC_MPCWM2_UPWMR {
    pub use super::TZSC_MPCWM1_UPWMR::LGTH;
}

/// Unprivileged Water Mark 3 register
pub mod TZSC_MPCWM3_UPWMR {
    pub use super::TZSC_MPCWM1_UPWMR::LGTH;
}
#[repr(C)]
pub struct RegisterBlock {
    /// TZSC control register
    pub TZSC_CR: RWRegister<u32>,

    _reserved1: [u32; 3],

    /// TZSC security configuration register
    pub TZSC_SECCFGR1: RWRegister<u32>,

    _reserved2: [u32; 3],

    /// TZSC privilege configuration register 1
    pub TZSC_PRIVCFGR1: RWRegister<u32>,

    _reserved3: [u32; 67],

    /// Unprivileged Water Mark 1 register
    pub TZSC_MPCWM1_UPWMR: RWRegister<u32>,

    /// Unprivileged Writable Water Mark 1 register
    pub TZSC_MPCWM1_UPWWMR: RWRegister<u32>,

    /// Unprivileged Water Mark 2 register
    pub TZSC_MPCWM2_UPWMR: RWRegister<u32>,

    _reserved4: [u32; 1],

    /// Unprivileged Water Mark 3 register
    pub TZSC_MPCWM3_UPWMR: RWRegister<u32>,
}
pub struct ResetValues {
    pub TZSC_CR: u32,
    pub TZSC_SECCFGR1: u32,
    pub TZSC_PRIVCFGR1: u32,
    pub TZSC_MPCWM1_UPWMR: u32,
    pub TZSC_MPCWM1_UPWWMR: u32,
    pub TZSC_MPCWM2_UPWMR: u32,
    pub TZSC_MPCWM3_UPWMR: u32,
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
