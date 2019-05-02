#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Backup registers

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Backup data register (BKP_DR)
pub mod DR1 {

    /// Backup data
    pub mod D {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Backup data register (BKP_DR)
pub mod DR2 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod DR3 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod DR4 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod DR5 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod DR6 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod DR7 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod DR8 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod DR9 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod DR10 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR11 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR12 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR13 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR14 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR15 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR16 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR17 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR18 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR19 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR20 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR21 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR22 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR23 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR24 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR25 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR26 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR27 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR28 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR29 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR30 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR31 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR32 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR33 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR34 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR35 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR36 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR37 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR38 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR39 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR40 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR41 {
    pub use super::DR1::D;
}

/// Backup data register (BKP_DR)
pub mod BKP_DR42 {
    pub use super::DR1::D;
}

/// RTC clock calibration register (BKP_RTCCR)
pub mod RTCCR {

    /// Calibration value
    pub mod CAL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Calibration Clock Output
    pub mod CCO {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Alarm or second output enable
    pub mod ASOE {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Alarm or second output selection
    pub mod ASOS {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Backup control register (BKP_CR)
pub mod CR {

    /// Tamper pin enable
    pub mod TPE {
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

    /// Tamper pin active level
    pub mod TPAL {
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

/// BKP_CSR control/status register (BKP_CSR)
pub mod CSR {

    /// Clear Tamper event
    pub mod CTE {
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

    /// Clear Tamper Interrupt
    pub mod CTI {
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

    /// Tamper Pin interrupt enable
    pub mod TPIE {
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

    /// Tamper Event Flag
    pub mod TEF {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Tamper Interrupt Flag
    pub mod TIF {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
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
    /// Backup data register (BKP_DR)
    pub DR1: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub DR2: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub DR3: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub DR4: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub DR5: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub DR6: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub DR7: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub DR8: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub DR9: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub DR10: RWRegister<u32>,

    /// RTC clock calibration register (BKP_RTCCR)
    pub RTCCR: RWRegister<u32>,

    /// Backup control register (BKP_CR)
    pub CR: RWRegister<u32>,

    /// BKP_CSR control/status register (BKP_CSR)
    pub CSR: RWRegister<u32>,

    _reserved1: [u32; 2],

    /// Backup data register (BKP_DR)
    pub BKP_DR11: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR12: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR13: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR14: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR15: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR16: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR17: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR18: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR19: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR20: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR21: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR22: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR23: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR24: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR25: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR26: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR27: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR28: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR29: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR30: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR31: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR32: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR33: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR34: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR35: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR36: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR37: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR38: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR39: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR40: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR41: RWRegister<u32>,

    /// Backup data register (BKP_DR)
    pub BKP_DR42: RWRegister<u32>,
}
pub struct ResetValues {
    pub DR1: u32,
    pub DR2: u32,
    pub DR3: u32,
    pub DR4: u32,
    pub DR5: u32,
    pub DR6: u32,
    pub DR7: u32,
    pub DR8: u32,
    pub DR9: u32,
    pub DR10: u32,
    pub RTCCR: u32,
    pub CR: u32,
    pub CSR: u32,
    pub BKP_DR11: u32,
    pub BKP_DR12: u32,
    pub BKP_DR13: u32,
    pub BKP_DR14: u32,
    pub BKP_DR15: u32,
    pub BKP_DR16: u32,
    pub BKP_DR17: u32,
    pub BKP_DR18: u32,
    pub BKP_DR19: u32,
    pub BKP_DR20: u32,
    pub BKP_DR21: u32,
    pub BKP_DR22: u32,
    pub BKP_DR23: u32,
    pub BKP_DR24: u32,
    pub BKP_DR25: u32,
    pub BKP_DR26: u32,
    pub BKP_DR27: u32,
    pub BKP_DR28: u32,
    pub BKP_DR29: u32,
    pub BKP_DR30: u32,
    pub BKP_DR31: u32,
    pub BKP_DR32: u32,
    pub BKP_DR33: u32,
    pub BKP_DR34: u32,
    pub BKP_DR35: u32,
    pub BKP_DR36: u32,
    pub BKP_DR37: u32,
    pub BKP_DR38: u32,
    pub BKP_DR39: u32,
    pub BKP_DR40: u32,
    pub BKP_DR41: u32,
    pub BKP_DR42: u32,
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

/// Access functions for the BKP peripheral instance
pub mod BKP {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40006c00,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in BKP
    pub const reset: ResetValues = ResetValues {
        DR1: 0x00000000,
        DR2: 0x00000000,
        DR3: 0x00000000,
        DR4: 0x00000000,
        DR5: 0x00000000,
        DR6: 0x00000000,
        DR7: 0x00000000,
        DR8: 0x00000000,
        DR9: 0x00000000,
        DR10: 0x00000000,
        BKP_DR11: 0x00000000,
        BKP_DR12: 0x00000000,
        BKP_DR13: 0x00000000,
        BKP_DR14: 0x00000000,
        BKP_DR15: 0x00000000,
        BKP_DR16: 0x00000000,
        BKP_DR17: 0x00000000,
        BKP_DR18: 0x00000000,
        BKP_DR19: 0x00000000,
        BKP_DR20: 0x00000000,
        BKP_DR21: 0x00000000,
        BKP_DR22: 0x00000000,
        BKP_DR23: 0x00000000,
        BKP_DR24: 0x00000000,
        BKP_DR25: 0x00000000,
        BKP_DR26: 0x00000000,
        BKP_DR27: 0x00000000,
        BKP_DR28: 0x00000000,
        BKP_DR29: 0x00000000,
        BKP_DR30: 0x00000000,
        BKP_DR31: 0x00000000,
        BKP_DR32: 0x00000000,
        BKP_DR33: 0x00000000,
        BKP_DR34: 0x00000000,
        BKP_DR35: 0x00000000,
        BKP_DR36: 0x00000000,
        BKP_DR37: 0x00000000,
        BKP_DR38: 0x00000000,
        BKP_DR39: 0x00000000,
        BKP_DR40: 0x00000000,
        BKP_DR41: 0x00000000,
        BKP_DR42: 0x00000000,
        RTCCR: 0x00000000,
        CR: 0x00000000,
        CSR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut BKP_TAKEN: bool = false;

    /// Safe access to BKP
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
            if BKP_TAKEN {
                None
            } else {
                BKP_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to BKP
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if BKP_TAKEN && inst.addr == INSTANCE.addr {
                BKP_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal BKP
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        BKP_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to BKP
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const BKP: *const RegisterBlock = 0x40006c00 as *const _;
