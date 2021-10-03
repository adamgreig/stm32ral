#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! GPIOZ
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::gpioz::Instance;
pub use crate::stm32mp::peripherals::gpioz::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::gpioz::{
    GPIOZ_AFRH, GPIOZ_AFRL, GPIOZ_BRR, GPIOZ_BSRR, GPIOZ_HWCFGR0, GPIOZ_HWCFGR1, GPIOZ_HWCFGR10,
    GPIOZ_HWCFGR2, GPIOZ_HWCFGR3, GPIOZ_HWCFGR4, GPIOZ_HWCFGR5, GPIOZ_HWCFGR6, GPIOZ_HWCFGR7,
    GPIOZ_HWCFGR8, GPIOZ_HWCFGR9, GPIOZ_IDR, GPIOZ_IPIDR, GPIOZ_LCKR, GPIOZ_MODER, GPIOZ_ODR,
    GPIOZ_OSPEEDR, GPIOZ_OTYPER, GPIOZ_PUPDR, GPIOZ_SECCFGR, GPIOZ_SIDR, GPIOZ_VERR,
};

/// Access functions for the GPIOZ peripheral instance
pub mod GPIOZ {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x54004000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in GPIOZ
    pub const reset: ResetValues = ResetValues {
        GPIOZ_MODER: 0xFFFFFFFF,
        GPIOZ_OTYPER: 0x00000000,
        GPIOZ_OSPEEDR: 0x00000000,
        GPIOZ_PUPDR: 0x00000000,
        GPIOZ_IDR: 0x00000000,
        GPIOZ_ODR: 0x00000000,
        GPIOZ_BSRR: 0x00000000,
        GPIOZ_LCKR: 0x00000000,
        GPIOZ_AFRL: 0x00000000,
        GPIOZ_AFRH: 0x00000000,
        GPIOZ_BRR: 0x00000000,
        GPIOZ_SECCFGR: 0x000000FF,
        GPIOZ_HWCFGR10: 0x00011240,
        GPIOZ_HWCFGR9: 0x000000FF,
        GPIOZ_HWCFGR8: 0x00000000,
        GPIOZ_HWCFGR7: 0xFFFFFFFF,
        GPIOZ_HWCFGR6: 0xFFFFFFFF,
        GPIOZ_HWCFGR5: 0x00000000,
        GPIOZ_HWCFGR4: 0x00000000,
        GPIOZ_HWCFGR3: 0x00000000,
        GPIOZ_HWCFGR2: 0x00000000,
        GPIOZ_HWCFGR1: 0x00000000,
        GPIOZ_HWCFGR0: 0x00000000,
        GPIOZ_VERR: 0x00000040,
        GPIOZ_IPIDR: 0x000F0002,
        GPIOZ_SIDR: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut GPIOZ_TAKEN: bool = false;

    /// Safe access to GPIOZ
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
            if GPIOZ_TAKEN {
                None
            } else {
                GPIOZ_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to GPIOZ
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if GPIOZ_TAKEN && inst.addr == INSTANCE.addr {
                GPIOZ_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal GPIOZ
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        GPIOZ_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to GPIOZ
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPIOZ: *const RegisterBlock = 0x54004000 as *const _;
