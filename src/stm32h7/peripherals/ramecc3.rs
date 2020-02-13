#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! ECC controller is associated to each RAM area
//!
//! Used by: stm32h747cm4, stm32h747cm7, stm32h757cm7

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// RAMECC interrupt enable register
pub mod IER {

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

    /// Global ECC single error interrupt enable
    pub mod GECCSEIE_ {
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

    /// Global ECC double error on byte write (BW) interrupt enable
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
}

/// RAMECC monitor x configuration register
pub mod M1CR {

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

    /// ECC double error on byte write (BW) interrupt enable
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

    /// ECC error latching enable
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
}

/// RAMECC monitor x configuration register
pub mod M2CR {
    pub use super::M1CR::ECCDEBWIE;
    pub use super::M1CR::ECCDEIE;
    pub use super::M1CR::ECCELEN;
    pub use super::M1CR::ECCSEIE;
}

/// RAMECC monitor x status register
pub mod M1SR {

    /// ECC single error detected and corrected flag
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

    /// ECC double error on byte write (BW) detected flag
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
}

/// RAMECC monitor x status register
pub mod M2SR {
    pub use super::M1SR::DEBWDF;
    pub use super::M1SR::DEDF;
    pub use super::M1SR::SEDCF;
}

/// RAMECC monitor x failing address register
pub mod M1FAR {

    /// ECC error failing address
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

/// RAMECC monitor x failing address register
pub mod M2FAR {
    pub use super::M1FAR::FADD;
}

/// RAMECC monitor x failing data low register
pub mod M1FDRL {

    /// Failing data low
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

/// RAMECC monitor x failing data low register
pub mod M2FDRL {
    pub use super::M1FDRL::FDATAL;
}

/// RAMECC monitor x failing data high register
pub mod M1FDRH {

    /// Failing data high (64-bit memory)
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

/// RAMECC monitor x failing data high register
pub mod M2FDRH {

    /// Failing data high (64-bit memory)
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

/// RAMECC monitor x failing ECC error code register
pub mod M1FECR {

    /// Failing error code
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

/// RAMECC monitor x failing ECC error code register
pub mod M2FECR {
    pub use super::M1FECR::FEC;
}
pub struct RegisterBlock {
    /// RAMECC interrupt enable register
    pub IER: RWRegister<u32>,

    _reserved1: [u32; 7],

    /// RAMECC monitor x configuration register
    pub M1CR: RWRegister<u32>,

    /// RAMECC monitor x status register
    pub M1SR: RWRegister<u32>,

    /// RAMECC monitor x failing address register
    pub M1FAR: RORegister<u32>,

    /// RAMECC monitor x failing data low register
    pub M1FDRL: RORegister<u32>,

    /// RAMECC monitor x failing data high register
    pub M1FDRH: RORegister<u32>,

    /// RAMECC monitor x failing ECC error code register
    pub M1FECR: RWRegister<u32>,

    _reserved2: [u32; 2],

    /// RAMECC monitor x configuration register
    pub M2CR: RWRegister<u32>,

    /// RAMECC monitor x status register
    pub M2SR: RWRegister<u32>,

    /// RAMECC monitor x failing address register
    pub M2FAR: RORegister<u32>,

    /// RAMECC monitor x failing data low register
    pub M2FDRL: RORegister<u32>,

    /// RAMECC monitor x failing data high register
    pub M2FDRH: RWRegister<u32>,

    _reserved3: [u32; 1],

    /// RAMECC monitor x failing ECC error code register
    pub M2FECR: RWRegister<u32>,
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
