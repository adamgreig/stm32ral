#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! GPIOD
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::gpiod::Instance;
pub use crate::stm32mp::peripherals::gpiod::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::gpiod::{
    GPIOD_AFRH, GPIOD_AFRL, GPIOD_BRR, GPIOD_BSRR, GPIOD_HWCFGR0, GPIOD_HWCFGR1, GPIOD_HWCFGR10,
    GPIOD_HWCFGR2, GPIOD_HWCFGR3, GPIOD_HWCFGR4, GPIOD_HWCFGR5, GPIOD_HWCFGR6, GPIOD_HWCFGR7,
    GPIOD_HWCFGR8, GPIOD_HWCFGR9, GPIOD_IDR, GPIOD_IPIDR, GPIOD_LCKR, GPIOD_MODER, GPIOD_ODR,
    GPIOD_OSPEEDR, GPIOD_OTYPER, GPIOD_PUPDR, GPIOD_SIDR, GPIOD_VERR,
};

/// Access functions for the GPIOD peripheral instance
pub mod GPIOD {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50005000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in GPIOD
    pub const reset: ResetValues = ResetValues {
        GPIOD_MODER: 0xFFFFFFFF,
        GPIOD_OTYPER: 0x00000000,
        GPIOD_OSPEEDR: 0x00000000,
        GPIOD_PUPDR: 0x00000000,
        GPIOD_IDR: 0x00000000,
        GPIOD_ODR: 0x00000000,
        GPIOD_BSRR: 0x00000000,
        GPIOD_LCKR: 0x00000000,
        GPIOD_AFRL: 0x00000000,
        GPIOD_AFRH: 0x00000000,
        GPIOD_BRR: 0x00000000,
        GPIOD_HWCFGR10: 0x00011240,
        GPIOD_HWCFGR9: 0x000000FF,
        GPIOD_HWCFGR8: 0x00000000,
        GPIOD_HWCFGR7: 0xFFFFFFFF,
        GPIOD_HWCFGR6: 0xFFFFFFFF,
        GPIOD_HWCFGR5: 0x00000000,
        GPIOD_HWCFGR4: 0x00000000,
        GPIOD_HWCFGR3: 0x00000000,
        GPIOD_HWCFGR2: 0x00000000,
        GPIOD_HWCFGR1: 0x00000000,
        GPIOD_HWCFGR0: 0x00000000,
        GPIOD_VERR: 0x00000040,
        GPIOD_IPIDR: 0x000F0002,
        GPIOD_SIDR: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut GPIOD_TAKEN: bool = false;

    /// Safe access to GPIOD
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
            if GPIOD_TAKEN {
                None
            } else {
                GPIOD_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to GPIOD
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if GPIOD_TAKEN && inst.addr == INSTANCE.addr {
                GPIOD_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal GPIOD
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        GPIOD_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to GPIOD
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPIOD: *const RegisterBlock = 0x50005000 as *const _;
