#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! STGENC

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// STGENC control register
pub mod STGENC_CNTCR {

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

    /// HLTDBG
    pub mod HLTDBG {
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
}

/// STGENC status register
pub mod STGENC_CNTSR {

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

    /// HLTDBG
    pub mod HLTDBG {
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
}

/// the control interface must clear the STGENC_CNTCR.EN bit before writing to this register.
pub mod STGENC_CNTCVL {

    /// CNTCVL_L_32
    pub mod CNTCVL_L_32 {
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

/// the control interface must clear the STGENC_CNTCR.EN bit before writing to this register.
pub mod STGENC_CNTCVU {

    /// CNTCVU_U_32
    pub mod CNTCVU_U_32 {
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

/// the control interface must clear the STGEN_CNTCR.EN bit before writing to this register.
pub mod STGENC_CNTFID0 {

    /// FREQ
    pub mod FREQ {
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

/// STGENC peripheral ID4 register
pub mod STGENC_PIDR4 {

    /// DES_2
    pub mod DES_2 {
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

    /// SIZE
    pub mod SIZE {
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

/// STGENC peripheral ID5 register
pub mod STGENC_PIDR5 {

    /// PIDR5
    pub mod PIDR5 {
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

/// STGENC peripheral ID6 register
pub mod STGENC_PIDR6 {

    /// PIDR6
    pub mod PIDR6 {
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

/// STGENC peripheral ID7 register
pub mod STGENC_PIDR7 {

    /// PIDR7
    pub mod PIDR7 {
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

/// STGENC peripheral ID0 register
pub mod STGENC_PIDR0 {

    /// PART_0
    pub mod PART_0 {
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

/// STGENC peripheral ID1 register
pub mod STGENC_PIDR1 {

    /// PART_1
    pub mod PART_1 {
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

    /// DES_0
    pub mod DES_0 {
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

/// STGENC peripheral ID2 register
pub mod STGENC_PIDR2 {

    /// DES_1
    pub mod DES_1 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// JEDEC
    pub mod JEDEC {
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

    /// REVISION
    pub mod REVISION {
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

/// STGENC peripheral ID3 register
pub mod STGENC_PIDR3 {

    /// CMOD
    pub mod CMOD {
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

    /// REVAND
    pub mod REVAND {
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

/// STGENC component ID0 register
pub mod STGENC_CIDR0 {

    /// PRMBL_0
    pub mod PRMBL_0 {
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

/// STGENC component ID1 register
pub mod STGENC_CIDR1 {

    /// PRMBL_1
    pub mod PRMBL_1 {
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

    /// CLASS
    pub mod CLASS {
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

/// STGENC component ID2 register
pub mod STGENC_CIDR2 {

    /// PRMBL_2
    pub mod PRMBL_2 {
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

/// STGENC component ID3 register
pub mod STGENC_CIDR3 {

    /// PRMBL_3
    pub mod PRMBL_3 {
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
#[repr(C)]
pub struct RegisterBlock {
    /// STGENC control register
    pub STGENC_CNTCR: RWRegister<u32>,

    /// STGENC status register
    pub STGENC_CNTSR: RORegister<u32>,

    /// the control interface must clear the STGENC_CNTCR.EN bit before writing to this register.
    pub STGENC_CNTCVL: RWRegister<u32>,

    /// the control interface must clear the STGENC_CNTCR.EN bit before writing to this register.
    pub STGENC_CNTCVU: RWRegister<u32>,

    _reserved1: [u32; 4],

    /// the control interface must clear the STGEN_CNTCR.EN bit before writing to this register.
    pub STGENC_CNTFID0: RWRegister<u32>,

    _reserved2: [u32; 1003],

    /// STGENC peripheral ID4 register
    pub STGENC_PIDR4: RORegister<u32>,

    /// STGENC peripheral ID5 register
    pub STGENC_PIDR5: RORegister<u32>,

    /// STGENC peripheral ID6 register
    pub STGENC_PIDR6: RORegister<u32>,

    /// STGENC peripheral ID7 register
    pub STGENC_PIDR7: RORegister<u32>,

    /// STGENC peripheral ID0 register
    pub STGENC_PIDR0: RORegister<u32>,

    /// STGENC peripheral ID1 register
    pub STGENC_PIDR1: RORegister<u32>,

    /// STGENC peripheral ID2 register
    pub STGENC_PIDR2: RORegister<u32>,

    /// STGENC peripheral ID3 register
    pub STGENC_PIDR3: RORegister<u32>,

    /// STGENC component ID0 register
    pub STGENC_CIDR0: RORegister<u32>,

    /// STGENC component ID1 register
    pub STGENC_CIDR1: RORegister<u32>,

    /// STGENC component ID2 register
    pub STGENC_CIDR2: RORegister<u32>,

    /// STGENC component ID3 register
    pub STGENC_CIDR3: RORegister<u32>,
}
pub struct ResetValues {
    pub STGENC_CNTCR: u32,
    pub STGENC_CNTSR: u32,
    pub STGENC_CNTCVL: u32,
    pub STGENC_CNTCVU: u32,
    pub STGENC_CNTFID0: u32,
    pub STGENC_PIDR4: u32,
    pub STGENC_PIDR5: u32,
    pub STGENC_PIDR6: u32,
    pub STGENC_PIDR7: u32,
    pub STGENC_PIDR0: u32,
    pub STGENC_PIDR1: u32,
    pub STGENC_PIDR2: u32,
    pub STGENC_PIDR3: u32,
    pub STGENC_CIDR0: u32,
    pub STGENC_CIDR1: u32,
    pub STGENC_CIDR2: u32,
    pub STGENC_CIDR3: u32,
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

/// Access functions for the STGENC peripheral instance
pub mod STGENC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x5c008000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in STGENC
    pub const reset: ResetValues = ResetValues {
        STGENC_CNTCR: 0x00000000,
        STGENC_CNTSR: 0x00000000,
        STGENC_CNTCVL: 0x00000000,
        STGENC_CNTCVU: 0x00000000,
        STGENC_CNTFID0: 0x00000000,
        STGENC_PIDR4: 0x00000004,
        STGENC_PIDR5: 0x00000000,
        STGENC_PIDR6: 0x00000000,
        STGENC_PIDR7: 0x00000000,
        STGENC_PIDR0: 0x00000001,
        STGENC_PIDR1: 0x000000B1,
        STGENC_PIDR2: 0x0000001B,
        STGENC_PIDR3: 0x00000000,
        STGENC_CIDR0: 0x0000000D,
        STGENC_CIDR1: 0x000000F0,
        STGENC_CIDR2: 0x00000050,
        STGENC_CIDR3: 0x000000B1,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut STGENC_TAKEN: bool = false;

    /// Safe access to STGENC
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
            if STGENC_TAKEN {
                None
            } else {
                STGENC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to STGENC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if STGENC_TAKEN && inst.addr == INSTANCE.addr {
                STGENC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal STGENC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        STGENC_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to STGENC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const STGENC: *const RegisterBlock = 0x5c008000 as *const _;
