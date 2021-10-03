#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! GPIOK
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::gpiok::Instance;
pub use crate::stm32mp::peripherals::gpiok::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::gpiok::{
    GPIOK_AFRH, GPIOK_AFRL, GPIOK_BRR, GPIOK_BSRR, GPIOK_HWCFGR0, GPIOK_HWCFGR1, GPIOK_HWCFGR10,
    GPIOK_HWCFGR2, GPIOK_HWCFGR3, GPIOK_HWCFGR4, GPIOK_HWCFGR5, GPIOK_HWCFGR6, GPIOK_HWCFGR7,
    GPIOK_HWCFGR8, GPIOK_HWCFGR9, GPIOK_IDR, GPIOK_IPIDR, GPIOK_LCKR, GPIOK_MODER, GPIOK_ODR,
    GPIOK_OSPEEDR, GPIOK_OTYPER, GPIOK_PUPDR, GPIOK_SIDR, GPIOK_VERR,
};

/// Access functions for the GPIOK peripheral instance
pub mod GPIOK {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x5000c000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in GPIOK
    pub const reset: ResetValues = ResetValues {
        GPIOK_MODER: 0xFFFFFFFF,
        GPIOK_OTYPER: 0x00000000,
        GPIOK_OSPEEDR: 0x00000000,
        GPIOK_PUPDR: 0x00000000,
        GPIOK_IDR: 0x00000000,
        GPIOK_ODR: 0x00000000,
        GPIOK_BSRR: 0x00000000,
        GPIOK_LCKR: 0x00000000,
        GPIOK_AFRL: 0x00000000,
        GPIOK_AFRH: 0x00000000,
        GPIOK_BRR: 0x00000000,
        GPIOK_HWCFGR10: 0x00011240,
        GPIOK_HWCFGR9: 0x000000FF,
        GPIOK_HWCFGR8: 0x00000000,
        GPIOK_HWCFGR7: 0xFFFFFFFF,
        GPIOK_HWCFGR6: 0xFFFFFFFF,
        GPIOK_HWCFGR5: 0x00000000,
        GPIOK_HWCFGR4: 0x00000000,
        GPIOK_HWCFGR3: 0x00000000,
        GPIOK_HWCFGR2: 0x00000000,
        GPIOK_HWCFGR1: 0x00000000,
        GPIOK_HWCFGR0: 0x00000000,
        GPIOK_VERR: 0x00000040,
        GPIOK_IPIDR: 0x000F0002,
        GPIOK_SIDR: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut GPIOK_TAKEN: bool = false;

    /// Safe access to GPIOK
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
            if GPIOK_TAKEN {
                None
            } else {
                GPIOK_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to GPIOK
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if GPIOK_TAKEN && inst.addr == INSTANCE.addr {
                GPIOK_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal GPIOK
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        GPIOK_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to GPIOK
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPIOK: *const RegisterBlock = 0x5000c000 as *const _;
