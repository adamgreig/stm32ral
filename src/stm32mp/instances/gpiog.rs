#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! GPIOG
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::gpiog::Instance;
pub use crate::stm32mp::peripherals::gpiog::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::gpiog::{
    GPIOG_AFRH, GPIOG_AFRL, GPIOG_BRR, GPIOG_BSRR, GPIOG_HWCFGR0, GPIOG_HWCFGR1, GPIOG_HWCFGR10,
    GPIOG_HWCFGR2, GPIOG_HWCFGR3, GPIOG_HWCFGR4, GPIOG_HWCFGR5, GPIOG_HWCFGR6, GPIOG_HWCFGR7,
    GPIOG_HWCFGR8, GPIOG_HWCFGR9, GPIOG_IDR, GPIOG_IPIDR, GPIOG_LCKR, GPIOG_MODER, GPIOG_ODR,
    GPIOG_OSPEEDR, GPIOG_OTYPER, GPIOG_PUPDR, GPIOG_SIDR, GPIOG_VERR,
};

/// Access functions for the GPIOG peripheral instance
pub mod GPIOG {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50008000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in GPIOG
    pub const reset: ResetValues = ResetValues {
        GPIOG_MODER: 0xFFFFFFFF,
        GPIOG_OTYPER: 0x00000000,
        GPIOG_OSPEEDR: 0x00000000,
        GPIOG_PUPDR: 0x00000000,
        GPIOG_IDR: 0x00000000,
        GPIOG_ODR: 0x00000000,
        GPIOG_BSRR: 0x00000000,
        GPIOG_LCKR: 0x00000000,
        GPIOG_AFRL: 0x00000000,
        GPIOG_AFRH: 0x00000000,
        GPIOG_BRR: 0x00000000,
        GPIOG_HWCFGR10: 0x00011240,
        GPIOG_HWCFGR9: 0x000000FF,
        GPIOG_HWCFGR8: 0x00000000,
        GPIOG_HWCFGR7: 0xFFFFFFFF,
        GPIOG_HWCFGR6: 0xFFFFFFFF,
        GPIOG_HWCFGR5: 0x00000000,
        GPIOG_HWCFGR4: 0x00000000,
        GPIOG_HWCFGR3: 0x00000000,
        GPIOG_HWCFGR2: 0x00000000,
        GPIOG_HWCFGR1: 0x00000000,
        GPIOG_HWCFGR0: 0x00000000,
        GPIOG_VERR: 0x00000040,
        GPIOG_IPIDR: 0x000F0002,
        GPIOG_SIDR: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut GPIOG_TAKEN: bool = false;

    /// Safe access to GPIOG
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
            if GPIOG_TAKEN {
                None
            } else {
                GPIOG_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to GPIOG
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if GPIOG_TAKEN && inst.addr == INSTANCE.addr {
                GPIOG_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal GPIOG
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        GPIOG_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to GPIOG
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPIOG: *const RegisterBlock = 0x50008000 as *const _;
