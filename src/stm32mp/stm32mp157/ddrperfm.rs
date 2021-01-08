#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DDRPERFM

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Write-only register. A read request returns all zeros.
pub mod DDRPERFM_CTL {

    /// START
    pub mod START {
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

    /// STOP
    pub mod STOP {
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

/// DDRPERFM configurationl register
pub mod DDRPERFM_CFG {

    /// EN
    pub mod EN {
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

    /// SEL
    pub mod SEL {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (2 bits: 0b11 << 16)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DDRPERFM status register
pub mod DDRPERFM_STATUS {

    /// COVF
    pub mod COVF {
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

    /// BUSY
    pub mod BUSY {
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

    /// TOVF
    pub mod TOVF {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Write-only register. A read request returns all zeros
pub mod DDRPERFM_CCR {

    /// CCLR
    pub mod CCLR {
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

    /// TCLR
    pub mod TCLR {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DDRPERFM interrupt enable register
pub mod DDRPERFM_IER {

    /// OVFIE
    pub mod OVFIE {
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

/// DDRPERFM interrupt status register
pub mod DDRPERFM_ISR {

    /// OVFF
    pub mod OVFF {
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

/// Write-only register. A read request returns all zeros
pub mod DDRPERFM_ICR {

    /// OVF
    pub mod OVF {
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

/// DDRPERFM time counter register
pub mod DDRPERFM_TCNT {

    /// CNT
    pub mod CNT {
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

/// DDRPERFM event counter 0 register
pub mod DDRPERFM_CNT0 {
    pub use super::DDRPERFM_TCNT::CNT;
}

/// DDRPERFM event counter 1 register
pub mod DDRPERFM_CNT1 {
    pub use super::DDRPERFM_TCNT::CNT;
}

/// DDRPERFM event counter 2 register
pub mod DDRPERFM_CNT2 {
    pub use super::DDRPERFM_TCNT::CNT;
}

/// DDRPERFM event counter 3 register
pub mod DDRPERFM_CNT3 {
    pub use super::DDRPERFM_TCNT::CNT;
}

/// DDRPERFM hardware configuration register
pub mod DDRPERFM_HWCFG {

    /// NCNT
    pub mod NCNT {
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
}

/// DDRPERFM version register
pub mod DDRPERFM_VER {

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

/// DDRPERFM ID register
pub mod DDRPERFM_ID {

    /// ID
    pub mod ID {
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

/// DDRPERFM magic ID register
pub mod DDRPERFM_SID {

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
    /// Write-only register. A read request returns all zeros.
    pub DDRPERFM_CTL: WORegister<u32>,

    /// DDRPERFM configurationl register
    pub DDRPERFM_CFG: RWRegister<u32>,

    /// DDRPERFM status register
    pub DDRPERFM_STATUS: RORegister<u32>,

    /// Write-only register. A read request returns all zeros
    pub DDRPERFM_CCR: WORegister<u32>,

    /// DDRPERFM interrupt enable register
    pub DDRPERFM_IER: RWRegister<u32>,

    /// DDRPERFM interrupt status register
    pub DDRPERFM_ISR: RORegister<u32>,

    /// Write-only register. A read request returns all zeros
    pub DDRPERFM_ICR: WORegister<u32>,

    _reserved1: [u32; 1],

    /// DDRPERFM time counter register
    pub DDRPERFM_TCNT: RORegister<u32>,

    _reserved2: [u32; 15],

    /// DDRPERFM event counter 0 register
    pub DDRPERFM_CNT0: RORegister<u32>,

    _reserved3: [u32; 1],

    /// DDRPERFM event counter 1 register
    pub DDRPERFM_CNT1: RORegister<u32>,

    _reserved4: [u32; 1],

    /// DDRPERFM event counter 2 register
    pub DDRPERFM_CNT2: RORegister<u32>,

    _reserved5: [u32; 1],

    /// DDRPERFM event counter 3 register
    pub DDRPERFM_CNT3: RORegister<u32>,

    _reserved6: [u32; 221],

    /// DDRPERFM hardware configuration register
    pub DDRPERFM_HWCFG: RORegister<u32>,

    /// DDRPERFM version register
    pub DDRPERFM_VER: RORegister<u32>,

    /// DDRPERFM ID register
    pub DDRPERFM_ID: RORegister<u32>,

    /// DDRPERFM magic ID register
    pub DDRPERFM_SID: RORegister<u32>,
}
pub struct ResetValues {
    pub DDRPERFM_CTL: u32,
    pub DDRPERFM_CFG: u32,
    pub DDRPERFM_STATUS: u32,
    pub DDRPERFM_CCR: u32,
    pub DDRPERFM_IER: u32,
    pub DDRPERFM_ISR: u32,
    pub DDRPERFM_ICR: u32,
    pub DDRPERFM_TCNT: u32,
    pub DDRPERFM_CNT0: u32,
    pub DDRPERFM_CNT1: u32,
    pub DDRPERFM_CNT2: u32,
    pub DDRPERFM_CNT3: u32,
    pub DDRPERFM_HWCFG: u32,
    pub DDRPERFM_VER: u32,
    pub DDRPERFM_ID: u32,
    pub DDRPERFM_SID: u32,
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

/// Access functions for the DDRPERFM peripheral instance
pub mod DDRPERFM {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x5a007000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DDRPERFM
    pub const reset: ResetValues = ResetValues {
        DDRPERFM_CTL: 0x00000000,
        DDRPERFM_CFG: 0x00000000,
        DDRPERFM_STATUS: 0x00000000,
        DDRPERFM_CCR: 0x00000000,
        DDRPERFM_IER: 0x00000000,
        DDRPERFM_ISR: 0x00000000,
        DDRPERFM_ICR: 0x00000000,
        DDRPERFM_TCNT: 0x00000000,
        DDRPERFM_CNT0: 0x00000000,
        DDRPERFM_CNT1: 0x00000000,
        DDRPERFM_CNT2: 0x00000000,
        DDRPERFM_CNT3: 0x00000000,
        DDRPERFM_HWCFG: 0x00000004,
        DDRPERFM_VER: 0x00000010,
        DDRPERFM_ID: 0x00140061,
        DDRPERFM_SID: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut DDRPERFM_TAKEN: bool = false;

    /// Safe access to DDRPERFM
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
            if DDRPERFM_TAKEN {
                None
            } else {
                DDRPERFM_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to DDRPERFM
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if DDRPERFM_TAKEN && inst.addr == INSTANCE.addr {
                DDRPERFM_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal DDRPERFM
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        DDRPERFM_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to DDRPERFM
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DDRPERFM: *const RegisterBlock = 0x5a007000 as *const _;
