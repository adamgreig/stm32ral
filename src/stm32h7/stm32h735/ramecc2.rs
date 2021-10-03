#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! ECC controller is associated to each RAM area

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

/// RAMECC monitor x configuration register
pub mod M3CR {
    pub use super::M1CR::ECCDEBWIE;
    pub use super::M1CR::ECCDEIE;
    pub use super::M1CR::ECCELEN;
    pub use super::M1CR::ECCSEIE;
}

/// RAMECC monitor x configuration register
pub mod M4CR {
    pub use super::M1CR::ECCDEBWIE;
    pub use super::M1CR::ECCDEIE;
    pub use super::M1CR::ECCELEN;
    pub use super::M1CR::ECCSEIE;
}

/// RAMECC monitor x configuration register
pub mod M5CR {
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

/// RAMECC monitor x status register
pub mod M3SR {
    pub use super::M1SR::DEBWDF;
    pub use super::M1SR::DEDF;
    pub use super::M1SR::SEDCF;
}

/// RAMECC monitor x status register
pub mod M4SR {
    pub use super::M1SR::DEBWDF;
    pub use super::M1SR::DEDF;
    pub use super::M1SR::SEDCF;
}

/// RAMECC monitor x status register
pub mod M5SR {
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

/// RAMECC monitor x failing address register
pub mod M3FAR {
    pub use super::M1FAR::FADD;
}

/// RAMECC monitor x failing address register
pub mod M4FAR {
    pub use super::M1FAR::FADD;
}

/// RAMECC monitor x failing address register
pub mod M5FAR {

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

/// RAMECC monitor x failing data low register
pub mod M3FDRL {
    pub use super::M1FDRL::FDATAL;
}

/// RAMECC monitor x failing data low register
pub mod M4FDRL {
    pub use super::M1FDRL::FDATAL;
}

/// RAMECC monitor x failing data low register
pub mod M5FDRL {
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

/// RAMECC monitor x failing data high register
pub mod M3FDRH {
    pub use super::M1FDRH::FDATAH;
}

/// M4FDRH and M4FECR
/// M4FDRH: RAMECC monitor x failing data high register
/// M4FECR: RAMECC monitor x failing ECC error code register
pub mod M4F {

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

/// RAMECC monitor x failing data high register
pub mod M5FDRH {

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

/// RAMECC monitor x failing ECC error code register
pub mod M3FECR {
    pub use super::M5FDRH::FEC;
}

/// RAMECC monitor x failing ECC error code register
pub mod M5FECR {
    pub use super::M5FDRH::FEC;
}
#[repr(C)]
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

    _reserved4: [u32; 1],

    /// RAMECC monitor x configuration register
    pub M3CR: RWRegister<u32>,

    /// RAMECC monitor x status register
    pub M3SR: RWRegister<u32>,

    /// RAMECC monitor x failing address register
    pub M3FAR: RORegister<u32>,

    /// RAMECC monitor x failing data low register
    pub M3FDRL: RORegister<u32>,

    /// RAMECC monitor x failing data high register
    pub M3FDRH: RORegister<u32>,

    _reserved5: [u32; 2],

    /// RAMECC monitor x failing ECC error code register
    pub M3FECR: RORegister<u32>,

    /// RAMECC monitor x configuration register
    pub M4CR: RWRegister<u32>,

    /// RAMECC monitor x status register
    pub M4SR: RWRegister<u32>,

    /// RAMECC monitor x failing address register
    pub M4FAR: RORegister<u32>,

    /// RAMECC monitor x failing data low register
    pub M4FDRL: RORegister<u32>,

    /// M4FDRH and M4FECR
    /// M4FDRH: RAMECC monitor x failing data high register
    /// M4FECR: RAMECC monitor x failing ECC error code register
    pub M4F: RWRegister<u32>,

    _reserved6: [u32; 3],

    /// RAMECC monitor x configuration register
    pub M5CR: RWRegister<u32>,

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
#[cfg(feature = "rtic")]
unsafe impl Send for Instance {}

/// Access functions for the RAMECC2 peripheral instance
pub mod RAMECC2 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x48023000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in RAMECC2
    pub const reset: ResetValues = ResetValues {
        IER: 0x00000000,
        M1CR: 0x00000000,
        M2CR: 0x00000000,
        M3CR: 0x00000000,
        M4CR: 0x00000000,
        M5CR: 0x00000000,
        M1SR: 0x00000000,
        M2SR: 0x00000000,
        M3SR: 0x00000000,
        M4SR: 0x00000000,
        M5SR: 0x00000000,
        M1FAR: 0x00000000,
        M2FAR: 0x00000000,
        M3FAR: 0x00000000,
        M4FAR: 0x00000000,
        M5FAR: 0x00000000,
        M1FDRL: 0x00000000,
        M2FDRL: 0x00000000,
        M3FDRL: 0x00000000,
        M4FDRL: 0x00000000,
        M5FDRL: 0x00000000,
        M1FDRH: 0x00000000,
        M2FDRH: 0x00000000,
        M3FDRH: 0x00000000,
        M4F: 0x00000000,
        M5FDRH: 0x00000000,
        M1FECR: 0x00000000,
        M2FECR: 0x00000000,
        M3FECR: 0x00000000,
        M5FECR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut RAMECC2_TAKEN: bool = false;

    /// Safe access to RAMECC2
    ///
    /// This function returns `Some(Instance)` if this instance is not
    /// currently taken, and `None` if it is. This ensures that if you
    /// do get `Some(Instance)`, you are ensured unique access to
    /// the peripheral and there cannot be data races (unless other
    /// code uses `unsafe`, of course). You can then pass the
    /// `Instance` around to other functions as required. When you're
    /// done with it, you can call `release(instance)` to return it.
    ///
    /// `Instance` itself dereferences to a `RegisterBlock`, which
    /// provides access to the peripheral's registers.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn take() -> Option<Instance> {
        external_cortex_m::interrupt::free(|_| unsafe {
            if RAMECC2_TAKEN {
                None
            } else {
                RAMECC2_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to RAMECC2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if RAMECC2_TAKEN && inst.addr == INSTANCE.addr {
                RAMECC2_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal RAMECC2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        RAMECC2_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to RAMECC2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const RAMECC2: *const RegisterBlock = 0x48023000 as *const _;
