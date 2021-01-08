#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! External interrupt/event controller
//!
//! Used by: stm32g070, stm32g071

#[cfg(not(feature = "nosync"))]
pub use crate::stm32g0::peripherals::exti_v2::Instance;
pub use crate::stm32g0::peripherals::exti_v2::{RegisterBlock, ResetValues};
pub use crate::stm32g0::peripherals::exti_v2::{
    EMR1, EMR2, EXTICR1, EXTICR2, EXTICR3, EXTICR4, FPR1, FTSR1, HWCFGR1, HWCFGR2, HWCFGR3,
    HWCFGR4, HWCFGR5, HWCFGR6, HWCFGR7, IMR1, IMR2, RPR1, RTSR1, SWIER1,
};

/// Access functions for the EXTI peripheral instance
pub mod EXTI {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40021800,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in EXTI
    pub const reset: ResetValues = ResetValues {
        RTSR1: 0x00000000,
        FTSR1: 0x00000000,
        SWIER1: 0x00000000,
        RPR1: 0x00000000,
        FPR1: 0x00000000,
        EXTICR1: 0x00000000,
        EXTICR2: 0x00000000,
        EXTICR3: 0x00000000,
        EXTICR4: 0x00000000,
        IMR1: 0xFFF80000,
        EMR1: 0x00000000,
        IMR2: 0xFFFFFFFF,
        EMR2: 0x00000000,
        HWCFGR7: 0x00000000,
        HWCFGR6: 0x00000003,
        HWCFGR5: 0xFEAFFFFF,
        HWCFGR4: 0x00000000,
        HWCFGR3: 0x00000000,
        HWCFGR2: 0x0007FFFF,
        HWCFGR1: 0x00051021,
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
pub const EXTI: *const RegisterBlock = 0x40021800 as *const _;
