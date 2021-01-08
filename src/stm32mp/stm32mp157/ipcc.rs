#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! IPCC

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// IPCC Processor 1 control register
pub mod IPCC_C1CR {

    /// RXOIE
    pub mod RXOIE {
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

    /// TXFIE
    pub mod TXFIE {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// IPCC Processor 1 mask register
pub mod IPCC_C1MR {

    /// CHxOM
    pub mod CHxOM {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CHxFM
    pub mod CHxFM {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (6 bits: 0x3f << 16)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Reading this register will always return 0x0000 0000.
pub mod IPCC_C1SCR {

    /// CHxC
    pub mod CHxC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CHxS
    pub mod CHxS {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (6 bits: 0x3f << 16)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// IPCC processor 1 to processor 2 status register
pub mod IPCC_C1TOC2SR {

    /// CHxF
    pub mod CHxF {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// IPCC Processor 2 control register
pub mod IPCC_C2CR {
    pub use super::IPCC_C1CR::RXOIE;
    pub use super::IPCC_C1CR::TXFIE;
}

/// IPCC Processor 2 mask register
pub mod IPCC_C2MR {
    pub use super::IPCC_C1MR::CHxFM;
    pub use super::IPCC_C1MR::CHxOM;
}

/// Reading this register will always return 0x0000 0000.
pub mod IPCC_C2SCR {
    pub use super::IPCC_C1SCR::CHxC;
    pub use super::IPCC_C1SCR::CHxS;
}

/// IPCC processor 2 to processor 1 status register
pub mod IPCC_C2TOC1SR {
    pub use super::IPCC_C1TOC2SR::CHxF;
}

/// IPCC Hardware configuration register
pub mod IPCC_HWCFGR {

    /// CHANNELS
    pub mod CHANNELS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// IPCC IP Version register
pub mod IPCC_VER {

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

/// IPCC IP Identification register
pub mod IPCC_ID {

    /// IPID
    pub mod IPID {
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

/// IPCC Size ID register
pub mod IPCC_SID {

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
    /// IPCC Processor 1 control register
    pub IPCC_C1CR: RWRegister<u32>,

    /// IPCC Processor 1 mask register
    pub IPCC_C1MR: RWRegister<u32>,

    /// Reading this register will always return 0x0000 0000.
    pub IPCC_C1SCR: RWRegister<u32>,

    /// IPCC processor 1 to processor 2 status register
    pub IPCC_C1TOC2SR: RORegister<u32>,

    /// IPCC Processor 2 control register
    pub IPCC_C2CR: RWRegister<u32>,

    /// IPCC Processor 2 mask register
    pub IPCC_C2MR: RWRegister<u32>,

    /// Reading this register will always return 0x0000 0000.
    pub IPCC_C2SCR: RWRegister<u32>,

    /// IPCC processor 2 to processor 1 status register
    pub IPCC_C2TOC1SR: RORegister<u32>,

    _reserved1: [u32; 244],

    /// IPCC Hardware configuration register
    pub IPCC_HWCFGR: RORegister<u32>,

    /// IPCC IP Version register
    pub IPCC_VER: RORegister<u32>,

    /// IPCC IP Identification register
    pub IPCC_ID: RORegister<u32>,

    /// IPCC Size ID register
    pub IPCC_SID: RORegister<u32>,
}
pub struct ResetValues {
    pub IPCC_C1CR: u32,
    pub IPCC_C1MR: u32,
    pub IPCC_C1SCR: u32,
    pub IPCC_C1TOC2SR: u32,
    pub IPCC_C2CR: u32,
    pub IPCC_C2MR: u32,
    pub IPCC_C2SCR: u32,
    pub IPCC_C2TOC1SR: u32,
    pub IPCC_HWCFGR: u32,
    pub IPCC_VER: u32,
    pub IPCC_ID: u32,
    pub IPCC_SID: u32,
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

/// Access functions for the IPCC peripheral instance
pub mod IPCC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x4c001000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in IPCC
    pub const reset: ResetValues = ResetValues {
        IPCC_C1CR: 0x00000000,
        IPCC_C1MR: 0xFFFFFFFF,
        IPCC_C1SCR: 0x00000000,
        IPCC_C1TOC2SR: 0x00000000,
        IPCC_C2CR: 0x00000000,
        IPCC_C2MR: 0xFFFFFFFF,
        IPCC_C2SCR: 0x00000000,
        IPCC_C2TOC1SR: 0x00000000,
        IPCC_HWCFGR: 0x00000002,
        IPCC_VER: 0x00000010,
        IPCC_ID: 0x00100071,
        IPCC_SID: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut IPCC_TAKEN: bool = false;

    /// Safe access to IPCC
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
            if IPCC_TAKEN {
                None
            } else {
                IPCC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to IPCC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if IPCC_TAKEN && inst.addr == INSTANCE.addr {
                IPCC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal IPCC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        IPCC_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to IPCC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const IPCC: *const RegisterBlock = 0x4c001000 as *const _;
