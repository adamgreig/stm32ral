#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Flexible memory controller
//!
//! Used by: stm32f303, stm32f3x8

#[cfg(not(feature = "nosync"))]
pub use crate::stm32f3::peripherals::fmc::Instance;
pub use crate::stm32f3::peripherals::fmc::{RegisterBlock, ResetValues};
pub use crate::stm32f3::peripherals::fmc::{
    BCR1, BCR2, BCR3, BCR4, BTR1, BTR2, BTR3, BTR4, BWTR1, BWTR2, BWTR3, BWTR4, ECCR2, ECCR3,
    PATT2, PATT3, PATT4, PCR2, PCR3, PCR4, PIO4, PMEM2, PMEM3, PMEM4, SR2, SR3, SR4,
};

/// Access functions for the FMC peripheral instance
pub mod FMC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0xa0000400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in FMC
    pub const reset: ResetValues = ResetValues {
        BCR1: 0x000030D0,
        BTR1: 0xFFFFFFFF,
        BCR2: 0x000030D0,
        BTR2: 0xFFFFFFFF,
        BCR3: 0x000030D0,
        BTR3: 0xFFFFFFFF,
        BCR4: 0x000030D0,
        BTR4: 0xFFFFFFFF,
        PCR2: 0x00000018,
        SR2: 0x00000040,
        PMEM2: 0xFCFCFCFC,
        PATT2: 0xFCFCFCFC,
        ECCR2: 0x00000000,
        PCR3: 0x00000018,
        SR3: 0x00000040,
        PMEM3: 0xFCFCFCFC,
        PATT3: 0xFCFCFCFC,
        ECCR3: 0x00000000,
        PCR4: 0x00000018,
        SR4: 0x00000040,
        PMEM4: 0xFCFCFCFC,
        PATT4: 0xFCFCFCFC,
        PIO4: 0xFCFCFCFC,
        BWTR1: 0x0FFFFFFF,
        BWTR2: 0x0FFFFFFF,
        BWTR3: 0x0FFFFFFF,
        BWTR4: 0x0FFFFFFF,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut FMC_TAKEN: bool = false;

    /// Safe access to FMC
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
            if FMC_TAKEN {
                None
            } else {
                FMC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to FMC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if FMC_TAKEN && inst.addr == INSTANCE.addr {
                FMC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal FMC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        FMC_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to FMC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const FMC: *const RegisterBlock = 0xa0000400 as *const _;
