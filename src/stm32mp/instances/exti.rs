#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! EXTI
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::exti::Instance;
pub use crate::stm32mp::peripherals::exti::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::exti::{
    EXTI_C2EMR1, EXTI_C2EMR2, EXTI_C2EMR3, EXTI_C2IMR1, EXTI_C2IMR2, EXTI_C2IMR3, EXTI_EMR1,
    EXTI_EMR2, EXTI_EMR3, EXTI_EXTICR1, EXTI_EXTICR2, EXTI_EXTICR3, EXTI_EXTICR4, EXTI_FPR1,
    EXTI_FPR2, EXTI_FPR3, EXTI_FTSR1, EXTI_FTSR2, EXTI_FTSR3, EXTI_HWCFGR1, EXTI_HWCFGR10,
    EXTI_HWCFGR11, EXTI_HWCFGR12, EXTI_HWCFGR13, EXTI_HWCFGR2, EXTI_HWCFGR3, EXTI_HWCFGR4,
    EXTI_HWCFGR5, EXTI_HWCFGR6, EXTI_HWCFGR7, EXTI_HWCFGR8, EXTI_HWCFGR9, EXTI_IMR1, EXTI_IMR2,
    EXTI_IMR3, EXTI_IPIDR, EXTI_RPR1, EXTI_RPR2, EXTI_RPR3, EXTI_RTSR1, EXTI_RTSR2, EXTI_RTSR3,
    EXTI_SIDR, EXTI_SWIER1, EXTI_SWIER2, EXTI_SWIER3, EXTI_TZENR1, EXTI_TZENR2, EXTI_TZENR3,
    EXTI_VERR,
};

/// Access functions for the EXTI peripheral instance
pub mod EXTI {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x5000d000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in EXTI
    pub const reset: ResetValues = ResetValues {
        EXTI_RTSR1: 0x00000000,
        EXTI_FTSR1: 0x00000000,
        EXTI_SWIER1: 0x00000000,
        EXTI_RPR1: 0x00000000,
        EXTI_FPR1: 0x00000000,
        EXTI_TZENR1: 0x00000000,
        EXTI_RTSR2: 0x00000000,
        EXTI_FTSR2: 0x00000000,
        EXTI_SWIER2: 0x00000000,
        EXTI_RPR2: 0x00000000,
        EXTI_FPR2: 0x00000000,
        EXTI_TZENR2: 0x00000000,
        EXTI_RTSR3: 0x00000000,
        EXTI_FTSR3: 0x00000000,
        EXTI_SWIER3: 0x00000000,
        EXTI_RPR3: 0x00000000,
        EXTI_FPR3: 0x00000000,
        EXTI_TZENR3: 0x00000000,
        EXTI_EXTICR1: 0x00000000,
        EXTI_EXTICR2: 0x00000000,
        EXTI_EXTICR3: 0x00000000,
        EXTI_EXTICR4: 0x00000000,
        EXTI_IMR1: 0xFFFE0000,
        EXTI_EMR1: 0x00000000,
        EXTI_IMR2: 0xFFFFFFFF,
        EXTI_EMR2: 0x00000000,
        EXTI_IMR3: 0x00000DE9,
        EXTI_EMR3: 0x00000000,
        EXTI_C2IMR1: 0xFFFE0000,
        EXTI_C2EMR1: 0x00000000,
        EXTI_C2IMR2: 0xFFFFFFFF,
        EXTI_C2EMR2: 0x00000000,
        EXTI_C2IMR3: 0x00000DE9,
        EXTI_C2EMR3: 0x00000000,
        EXTI_HWCFGR13: 0x050EFFFF,
        EXTI_HWCFGR12: 0x050EFFFF,
        EXTI_HWCFGR11: 0x050EFFFF,
        EXTI_HWCFGR10: 0x00000000,
        EXTI_HWCFGR9: 0x00000000,
        EXTI_HWCFGR8: 0x00000000,
        EXTI_HWCFGR7: 0x000EFFFF,
        EXTI_HWCFGR6: 0x000EFFFF,
        EXTI_HWCFGR5: 0x000EFFFF,
        EXTI_HWCFGR4: 0x0001FFFF,
        EXTI_HWCFGR3: 0x0001FFFF,
        EXTI_HWCFGR2: 0x0001FFFF,
        EXTI_HWCFGR1: 0x000B214B,
        EXTI_VERR: 0x00000030,
        EXTI_IPIDR: 0x000E0001,
        EXTI_SIDR: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut EXTI_TAKEN: bool = false;

    /// Safe access to EXTI
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
            if EXTI_TAKEN {
                None
            } else {
                EXTI_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to EXTI
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if EXTI_TAKEN && inst.addr == INSTANCE.addr {
                EXTI_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal EXTI
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        EXTI_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to EXTI
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const EXTI: *const RegisterBlock = 0x5000d000 as *const _;
