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

/// RAMECC monitor x status register
pub mod M1SR {
    pub use super::M1CR::ECCDEBWIE;
    pub use super::M1CR::ECCDEIE;
    pub use super::M1CR::ECCELEN;
    pub use super::M1CR::ECCSEIE;
}

/// RAMECC monitor x failing address register
pub mod M1FAR {
    pub use super::M1CR::ECCDEBWIE;
    pub use super::M1CR::ECCDEIE;
    pub use super::M1CR::ECCELEN;
    pub use super::M1CR::ECCSEIE;
}

/// RAMECC monitor x failing data low register
pub mod M1FDRL {
    pub use super::M1CR::ECCDEBWIE;
    pub use super::M1CR::ECCDEIE;
    pub use super::M1CR::ECCELEN;
    pub use super::M1CR::ECCSEIE;
}

/// RAMECC monitor x failing data high register
pub mod M1FDRH {
    pub use super::M1CR::ECCDEBWIE;
    pub use super::M1CR::ECCDEIE;
    pub use super::M1CR::ECCELEN;
    pub use super::M1CR::ECCSEIE;
}

/// RAMECC monitor x failing ECC error code register
pub mod M1FECR {

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

/// RAMECC monitor x configuration register
pub mod M2CR {
    pub use super::M1FECR::DEBWDF;
    pub use super::M1FECR::DEDF;
    pub use super::M1FECR::SEDCF;
}

/// RAMECC monitor x status register
pub mod M2SR {
    pub use super::M1FECR::DEBWDF;
    pub use super::M1FECR::DEDF;
    pub use super::M1FECR::SEDCF;
}

/// RAMECC monitor x failing address register
pub mod M2FAR {
    pub use super::M1FECR::DEBWDF;
    pub use super::M1FECR::DEDF;
    pub use super::M1FECR::SEDCF;
}

/// RAMECC monitor x failing data low register
pub mod M2FDRL {
    pub use super::M1FECR::DEBWDF;
    pub use super::M1FECR::DEDF;
    pub use super::M1FECR::SEDCF;
}

/// RAMECC monitor x failing data high register
pub mod M2FDRH {

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

/// RAMECC monitor x failing ECC error code register
pub mod M2FECR {
    pub use super::M2FDRH::FADD;
}

/// RAMECC monitor x configuration register
pub mod M3CR {
    pub use super::M2FDRH::FADD;
}

/// RAMECC monitor x status register
pub mod M3SR {
    pub use super::M2FDRH::FADD;
}

/// RAMECC monitor x failing address register
pub mod M3FAR {

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

/// RAMECC monitor x failing data low register
pub mod M3FDRL {

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

/// RAMECC monitor x failing data high register
pub mod M3FDRH {
    pub use super::M3FDRL::FDATAL;
}

/// RAMECC monitor x failing ECC error code register
pub mod M3FECR {
    pub use super::M3FDRL::FDATAL;
}

/// RAMECC monitor x configuration register
pub mod M4CR {
    pub use super::M3FDRL::FDATAL;
}

/// RAMECC monitor x status register
pub mod M4SR {
    pub use super::M3FDRL::FDATAL;
}

/// RAMECC monitor x failing address register
pub mod M4FAR {

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

/// RAMECC monitor x failing data low register
pub mod M4FDRL {

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

/// M4FDRH and M4FECR
/// M4FDRH: RAMECC monitor x failing data high register
/// M4FECR: RAMECC monitor x failing ECC error code register
pub mod M4F {
    pub use super::M4FAR::FDATAH;
}

/// RAMECC monitor x configuration register
pub mod M5CR {

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

/// RAMECC monitor x status register
pub mod M5SR {

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

/// RAMECC monitor x failing address register
pub mod M5FAR {
    pub use super::M5SR::FEC;
}

/// RAMECC monitor x failing data low register
pub mod M5FDRL {
    pub use super::M5CR::FEC;
}

/// RAMECC monitor x failing data high register
pub mod M5FDRH {
    pub use super::M5CR::FEC;
}

/// RAMECC monitor x failing ECC error code register
pub mod M5FECR {
    pub use super::M5CR::FEC;
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
    pub M1FAR: RWRegister<u32>,

    /// RAMECC monitor x failing data low register
    pub M1FDRL: RWRegister<u32>,

    /// RAMECC monitor x failing data high register
    pub M1FDRH: RWRegister<u32>,

    /// RAMECC monitor x failing ECC error code register
    pub M1FECR: RWRegister<u32>,

    _reserved2: [u32; 2],

    /// RAMECC monitor x configuration register
    pub M2CR: RWRegister<u32>,

    /// RAMECC monitor x status register
    pub M2SR: RWRegister<u32>,

    /// RAMECC monitor x failing address register
    pub M2FAR: RWRegister<u32>,

    /// RAMECC monitor x failing data low register
    pub M2FDRL: RWRegister<u32>,

    /// RAMECC monitor x failing data high register
    pub M2FDRH: RORegister<u32>,

    _reserved3: [u32; 1],

    /// RAMECC monitor x failing ECC error code register
    pub M2FECR: RORegister<u32>,

    _reserved4: [u32; 1],

    /// RAMECC monitor x configuration register
    pub M3CR: RORegister<u32>,

    /// RAMECC monitor x status register
    pub M3SR: RORegister<u32>,

    /// RAMECC monitor x failing address register
    pub M3FAR: RWRegister<u32>,

    /// RAMECC monitor x failing data low register
    pub M3FDRL: RORegister<u32>,

    /// RAMECC monitor x failing data high register
    pub M3FDRH: RORegister<u32>,

    _reserved5: [u32; 2],

    /// RAMECC monitor x failing ECC error code register
    pub M3FECR: RORegister<u32>,

    /// RAMECC monitor x configuration register
    pub M4CR: RORegister<u32>,

    /// RAMECC monitor x status register
    pub M4SR: RORegister<u32>,

    /// RAMECC monitor x failing address register
    pub M4FAR: RORegister<u32>,

    /// RAMECC monitor x failing data low register
    pub M4FDRL: RWRegister<u32>,

    /// M4FDRH and M4FECR
    /// M4FDRH: RAMECC monitor x failing data high register
    /// M4FECR: RAMECC monitor x failing ECC error code register
    pub M4F: RWRegister<u32>,

    _reserved6: [u32; 3],

    /// RAMECC monitor x configuration register
    pub M5CR: RORegister<u32>,

    /// RAMECC monitor x status register
    pub M5SR: RWRegister<u32>,

    /// RAMECC monitor x failing address register
    pub M5FAR: RWRegister<u32>,

    /// RAMECC monitor x failing data low register
    pub M5FDRL: RORegister<u32>,

    /// RAMECC monitor x failing data high register
    pub M5FDRH: RORegister<u32>,

    /// RAMECC monitor x failing ECC error code register
    pub M5FECR: RORegister<u32>,
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
    pub M4F: u32,
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
#[cfg(feature = "rtfm")]
unsafe impl Send for Instance {}
