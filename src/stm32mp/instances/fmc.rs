#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! FMC register block
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::fmc::Instance;
pub use crate::stm32mp::peripherals::fmc::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::fmc::{
    FMC_BCHDSR0, FMC_BCHDSR1, FMC_BCHDSR2, FMC_BCHDSR3, FMC_BCHDSR4, FMC_BCHICR, FMC_BCHIER,
    FMC_BCHISR, FMC_BCHPBR1, FMC_BCHPBR2, FMC_BCHPBR3, FMC_BCHPBR4, FMC_BCR1, FMC_BCR2, FMC_BCR3,
    FMC_BCR4, FMC_BTR1, FMC_BTR2, FMC_BTR3, FMC_BTR4, FMC_BWTR1, FMC_BWTR2, FMC_BWTR3, FMC_BWTR4,
    FMC_CSQAR1, FMC_CSQAR2, FMC_CSQCFGR1, FMC_CSQCFGR2, FMC_CSQCFGR3, FMC_CSQCR, FMC_CSQEMSR,
    FMC_CSQICR, FMC_CSQIER, FMC_CSQISR, FMC_HECCR, FMC_HPR, FMC_HWCFGR1, FMC_HWCFGR2, FMC_IPIDR,
    FMC_PATT, FMC_PCR, FMC_PCSCNTR, FMC_PMEM, FMC_SIDR, FMC_SR, FMC_VERR,
};

/// Access functions for the FMC peripheral instance
pub mod FMC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x58002000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in FMC
    pub const reset: ResetValues = ResetValues {
        FMC_BCR1: 0x000030DB,
        FMC_BTR1: 0x0FFFFFFF,
        FMC_BCR2: 0x000030DB,
        FMC_BTR2: 0x0FFFFFFF,
        FMC_BCR3: 0x000030DB,
        FMC_BTR3: 0x0FFFFFFF,
        FMC_BCR4: 0x000030DB,
        FMC_BTR4: 0x0FFFFFFF,
        FMC_PCSCNTR: 0x00000000,
        FMC_PCR: 0x0007FE08,
        FMC_SR: 0x00000040,
        FMC_PMEM: 0x0A0A0A0A,
        FMC_PATT: 0x0A0A0A0A,
        FMC_HPR: 0x00000000,
        FMC_HECCR: 0x00000000,
        FMC_BWTR1: 0x000FFFFF,
        FMC_BWTR2: 0x000FFFFF,
        FMC_BWTR3: 0x000FFFFF,
        FMC_BWTR4: 0x000FFFFF,
        FMC_CSQCR: 0x00000000,
        FMC_CSQCFGR1: 0x00000000,
        FMC_CSQCFGR2: 0x00000000,
        FMC_CSQCFGR3: 0x00000000,
        FMC_CSQAR1: 0x00000000,
        FMC_CSQAR2: 0x00020000,
        FMC_CSQIER: 0x00000000,
        FMC_CSQISR: 0x00000000,
        FMC_CSQICR: 0x00000000,
        FMC_CSQEMSR: 0x00000000,
        FMC_BCHIER: 0x00000000,
        FMC_BCHISR: 0x00000000,
        FMC_BCHICR: 0x00000000,
        FMC_BCHPBR1: 0x00000000,
        FMC_BCHPBR2: 0x00000000,
        FMC_BCHPBR3: 0x00000000,
        FMC_BCHPBR4: 0x00000000,
        FMC_BCHDSR0: 0x00000000,
        FMC_BCHDSR1: 0x00000000,
        FMC_BCHDSR2: 0x00000000,
        FMC_BCHDSR3: 0x00000000,
        FMC_BCHDSR4: 0x00000000,
        FMC_HWCFGR2: 0x00DC8762,
        FMC_HWCFGR1: 0x2232B011,
        FMC_VERR: 0x00000011,
        FMC_IPIDR: 0x00140001,
        FMC_SIDR: 0xA3C5DD01,
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
pub const FMC: *const RegisterBlock = 0x58002000 as *const _;
