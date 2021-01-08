#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! RAM ECC monitoring
//!
//! Used by: stm32h743, stm32h743v, stm32h747cm4, stm32h747cm7, stm32h753, stm32h753v

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// RAMECC interrupt enable register
pub mod IER {

    /// Global ECC double error on byte write interrupt enable
    pub mod GECCDEBWIE {
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

    /// Global ECC double error interrupt enable
    pub mod GECCDEIE {
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

    /// Global ECC single error interrupt enable
    pub mod GECCSEIE {
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

    /// Global interrupt enable
    pub mod GIE {
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

/// RAMECC monitor 1 configuration register
pub mod M1CR {

    /// ECC error context latching enable
    pub mod ECCELEN {
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

    /// ECC double error on byte write interrupt enable
    pub mod ECCDEBWIE {
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

    /// ECC double error interrupt enable
    pub mod ECCDEIE {
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

    /// ECC single error interrupt enable
    pub mod ECCSEIE {
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
}

/// RAMECC monitor 1 status register
pub mod M1SR {

    /// ECC double error on byte write flag
    pub mod DEBWDF {
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

    /// ECC double error detected flag
    pub mod DEDF {
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

    /// ECC single error detected flag
    pub mod SEDCF {
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

/// RAMECC monitor 1 failing address register
pub mod M1FAR {

    /// ECC failing address
    pub mod FADD {
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

/// RAMECC monitor 1 failing data low register
pub mod M1FDRL {

    /// ECC failing data low
    pub mod FDATAL {
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

/// RAMECC monitor 1 failing data high register
pub mod M1FDRH {

    /// ECC failing data high
    pub mod FDATAH {
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

/// RAMECC monitor 1 failing error code register
pub mod M1FECR {

    /// ECC failing code
    pub mod FEC {
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

/// RAMECC monitor 2 configuration register
pub mod M2CR {
    pub use super::M1CR::ECCDEBWIE;
    pub use super::M1CR::ECCDEIE;
    pub use super::M1CR::ECCELEN;
    pub use super::M1CR::ECCSEIE;
}

/// RAMECC monitor 2 status register
pub mod M2SR {
    pub use super::M1SR::DEBWDF;
    pub use super::M1SR::DEDF;
    pub use super::M1SR::SEDCF;
}

/// RAMECC monitor 2 failing address register
pub mod M2FAR {
    pub use super::M1FAR::FADD;
}

/// RAMECC monitor 2 failing data low register
pub mod M2FDRL {
    pub use super::M1FDRL::FDATAL;
}

/// RAMECC monitor 2 failing data high register
pub mod M2FDRH {
    pub use super::M1FDRH::FDATAH;
}

/// RAMECC monitor 2 failing error code register
pub mod M2FECR {
    pub use super::M1FECR::FEC;
}

/// RAMECC monitor 3 configuration register
pub mod M3CR {
    pub use super::M1CR::ECCDEBWIE;
    pub use super::M1CR::ECCDEIE;
    pub use super::M1CR::ECCELEN;
    pub use super::M1CR::ECCSEIE;
}

/// RAMECC monitor 3 status register
pub mod M3SR {
    pub use super::M1SR::DEBWDF;
    pub use super::M1SR::DEDF;
    pub use super::M1SR::SEDCF;
}

/// RAMECC monitor 3 failing address register
pub mod M3FAR {
    pub use super::M1FAR::FADD;
}

/// RAMECC monitor 3 failing data low register
pub mod M3FDRL {
    pub use super::M1FDRL::FDATAL;
}

/// RAMECC monitor 3 failing data high register
pub mod M3FDRH {
    pub use super::M1FDRH::FDATAH;
}

/// RAMECC monitor 3 failing error code register
pub mod M3FECR {
    pub use super::M1FECR::FEC;
}

/// RAMECC monitor 4 configuration register
pub mod M4CR {
    pub use super::M1CR::ECCDEBWIE;
    pub use super::M1CR::ECCDEIE;
    pub use super::M1CR::ECCELEN;
    pub use super::M1CR::ECCSEIE;
}

/// RAMECC monitor 4 status register
pub mod M4SR {
    pub use super::M1SR::DEBWDF;
    pub use super::M1SR::DEDF;
    pub use super::M1SR::SEDCF;
}

/// RAMECC monitor 4 failing address register
pub mod M4FAR {
    pub use super::M1FAR::FADD;
}

/// RAMECC monitor 4 failing data low register
pub mod M4FDRL {
    pub use super::M1FDRL::FDATAL;
}

/// RAMECC monitor 4 failing data high register
pub mod M4FDRH {
    pub use super::M1FDRH::FDATAH;
}

/// RAMECC monitor 4 failing error code register
pub mod M4FECR {
    pub use super::M1FECR::FEC;
}

/// RAMECC monitor 5 configuration register
pub mod M5CR {
    pub use super::M1CR::ECCDEBWIE;
    pub use super::M1CR::ECCDEIE;
    pub use super::M1CR::ECCELEN;
    pub use super::M1CR::ECCSEIE;
}

/// RAMECC monitor 5 status register
pub mod M5SR {
    pub use super::M1SR::DEBWDF;
    pub use super::M1SR::DEDF;
    pub use super::M1SR::SEDCF;
}

/// RAMECC monitor 5 failing address register
pub mod M5FAR {
    pub use super::M1FAR::FADD;
}

/// RAMECC monitor 5 failing data low register
pub mod M5FDRL {
    pub use super::M1FDRL::FDATAL;
}

/// RAMECC monitor 5 failing data high register
pub mod M5FDRH {
    pub use super::M1FDRH::FDATAH;
}

/// RAMECC monitor 5 failing error code register
pub mod M5FECR {
    pub use super::M1FECR::FEC;
}
#[repr(C)]
pub struct RegisterBlock {
    /// RAMECC interrupt enable register
    pub IER: RWRegister<u32>,

    _reserved1: [u32; 7],

    /// RAMECC monitor 1 configuration register
    pub M1CR: RWRegister<u32>,

    /// RAMECC monitor 1 status register
    pub M1SR: RWRegister<u32>,

    /// RAMECC monitor 1 failing address register
    pub M1FAR: RWRegister<u32>,

    /// RAMECC monitor 1 failing data low register
    pub M1FDRL: RWRegister<u32>,

    /// RAMECC monitor 1 failing data high register
    pub M1FDRH: RWRegister<u32>,

    /// RAMECC monitor 1 failing error code register
    pub M1FECR: RWRegister<u32>,

    _reserved2: [u32; 2],

    /// RAMECC monitor 2 configuration register
    pub M2CR: RWRegister<u32>,

    /// RAMECC monitor 2 status register
    pub M2SR: RWRegister<u32>,

    /// RAMECC monitor 2 failing address register
    pub M2FAR: RWRegister<u32>,

    /// RAMECC monitor 2 failing data low register
    pub M2FDRL: RWRegister<u32>,

    /// RAMECC monitor 2 failing data high register
    pub M2FDRH: RWRegister<u32>,

    /// RAMECC monitor 2 failing error code register
    pub M2FECR: RWRegister<u32>,

    _reserved3: [u32; 2],

    /// RAMECC monitor 3 configuration register
    pub M3CR: RWRegister<u32>,

    /// RAMECC monitor 3 status register
    pub M3SR: RWRegister<u32>,

    /// RAMECC monitor 3 failing address register
    pub M3FAR: RWRegister<u32>,

    /// RAMECC monitor 3 failing data low register
    pub M3FDRL: RWRegister<u32>,

    /// RAMECC monitor 3 failing data high register
    pub M3FDRH: RWRegister<u32>,

    /// RAMECC monitor 3 failing error code register
    pub M3FECR: RWRegister<u32>,

    _reserved4: [u32; 2],

    /// RAMECC monitor 4 configuration register
    pub M4CR: RWRegister<u32>,

    /// RAMECC monitor 4 status register
    pub M4SR: RWRegister<u32>,

    /// RAMECC monitor 4 failing address register
    pub M4FAR: RWRegister<u32>,

    /// RAMECC monitor 4 failing data low register
    pub M4FDRL: RWRegister<u32>,

    /// RAMECC monitor 4 failing data high register
    pub M4FDRH: RWRegister<u32>,

    /// RAMECC monitor 4 failing error code register
    pub M4FECR: RWRegister<u32>,

    _reserved5: [u32; 2],

    /// RAMECC monitor 5 configuration register
    pub M5CR: RWRegister<u32>,

    /// RAMECC monitor 5 status register
    pub M5SR: RWRegister<u32>,

    /// RAMECC monitor 5 failing address register
    pub M5FAR: RWRegister<u32>,

    /// RAMECC monitor 5 failing data low register
    pub M5FDRL: RWRegister<u32>,

    /// RAMECC monitor 5 failing data high register
    pub M5FDRH: RWRegister<u32>,

    /// RAMECC monitor 5 failing error code register
    pub M5FECR: RWRegister<u32>,
}
pub struct ResetValues {
    pub IER: u32,
    pub M1CR: u32,
    pub M1SR: u32,
    pub M1FAR: u32,
    pub M1FDRL: u32,
    pub M1FDRH: u32,
    pub M1FECR: u32,
    pub M2CR: u32,
    pub M2SR: u32,
    pub M2FAR: u32,
    pub M2FDRL: u32,
    pub M2FDRH: u32,
    pub M2FECR: u32,
    pub M3CR: u32,
    pub M3SR: u32,
    pub M3FAR: u32,
    pub M3FDRL: u32,
    pub M3FDRH: u32,
    pub M3FECR: u32,
    pub M4CR: u32,
    pub M4SR: u32,
    pub M4FAR: u32,
    pub M4FDRL: u32,
    pub M4FDRH: u32,
    pub M4FECR: u32,
    pub M5CR: u32,
    pub M5SR: u32,
    pub M5FAR: u32,
    pub M5FDRL: u32,
    pub M5FDRH: u32,
    pub M5FECR: u32,
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
