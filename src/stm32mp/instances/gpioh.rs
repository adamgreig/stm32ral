#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! GPIOH
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::gpioh::Instance;
pub use crate::stm32mp::peripherals::gpioh::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::gpioh::{
    GPIOH_AFRH, GPIOH_AFRL, GPIOH_BRR, GPIOH_BSRR, GPIOH_HWCFGR0, GPIOH_HWCFGR1, GPIOH_HWCFGR10,
    GPIOH_HWCFGR2, GPIOH_HWCFGR3, GPIOH_HWCFGR4, GPIOH_HWCFGR5, GPIOH_HWCFGR6, GPIOH_HWCFGR7,
    GPIOH_HWCFGR8, GPIOH_HWCFGR9, GPIOH_IDR, GPIOH_IPIDR, GPIOH_LCKR, GPIOH_MODER, GPIOH_ODR,
    GPIOH_OSPEEDR, GPIOH_OTYPER, GPIOH_PUPDR, GPIOH_SIDR, GPIOH_VERR,
};

/// Access functions for the GPIOH peripheral instance
pub mod GPIOH {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50009000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in GPIOH
    pub const reset: ResetValues = ResetValues {
        GPIOH_MODER: 0xFFFFFFFF,
        GPIOH_OTYPER: 0x00000000,
        GPIOH_OSPEEDR: 0x00000000,
        GPIOH_PUPDR: 0x00000000,
        GPIOH_IDR: 0x00000000,
        GPIOH_ODR: 0x00000000,
        GPIOH_BSRR: 0x00000000,
        GPIOH_LCKR: 0x00000000,
        GPIOH_AFRL: 0x00000000,
        GPIOH_AFRH: 0x00000000,
        GPIOH_BRR: 0x00000000,
        GPIOH_HWCFGR10: 0x00011240,
        GPIOH_HWCFGR9: 0x000000FF,
        GPIOH_HWCFGR8: 0x00000000,
        GPIOH_HWCFGR7: 0xFFFFFFFF,
        GPIOH_HWCFGR6: 0xFFFFFFFF,
        GPIOH_HWCFGR5: 0x00000000,
        GPIOH_HWCFGR4: 0x00000000,
        GPIOH_HWCFGR3: 0x00000000,
        GPIOH_HWCFGR2: 0x00000000,
        GPIOH_HWCFGR1: 0x00000000,
        GPIOH_HWCFGR0: 0x00000000,
        GPIOH_VERR: 0x00000040,
        GPIOH_IPIDR: 0x000F0002,
        GPIOH_SIDR: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut GPIOH_TAKEN: bool = false;

    /// Safe access to GPIOH
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
            if GPIOH_TAKEN {
                None
            } else {
                GPIOH_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to GPIOH
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if GPIOH_TAKEN && inst.addr == INSTANCE.addr {
                GPIOH_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal GPIOH
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        GPIOH_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to GPIOH
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPIOH: *const RegisterBlock = 0x50009000 as *const _;
