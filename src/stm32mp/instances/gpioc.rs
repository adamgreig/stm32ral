#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! GPIOC
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::gpioc::Instance;
pub use crate::stm32mp::peripherals::gpioc::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::gpioc::{
    GPIOC_AFRH, GPIOC_AFRL, GPIOC_BRR, GPIOC_BSRR, GPIOC_HWCFGR0, GPIOC_HWCFGR1, GPIOC_HWCFGR10,
    GPIOC_HWCFGR2, GPIOC_HWCFGR3, GPIOC_HWCFGR4, GPIOC_HWCFGR5, GPIOC_HWCFGR6, GPIOC_HWCFGR7,
    GPIOC_HWCFGR8, GPIOC_HWCFGR9, GPIOC_IDR, GPIOC_IPIDR, GPIOC_LCKR, GPIOC_MODER, GPIOC_ODR,
    GPIOC_OSPEEDR, GPIOC_OTYPER, GPIOC_PUPDR, GPIOC_SIDR, GPIOC_VERR,
};

/// Access functions for the GPIOC peripheral instance
pub mod GPIOC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50004000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in GPIOC
    pub const reset: ResetValues = ResetValues {
        GPIOC_MODER: 0xFFFFFFFF,
        GPIOC_OTYPER: 0x00000000,
        GPIOC_OSPEEDR: 0x00000000,
        GPIOC_PUPDR: 0x00000000,
        GPIOC_IDR: 0x00000000,
        GPIOC_ODR: 0x00000000,
        GPIOC_BSRR: 0x00000000,
        GPIOC_LCKR: 0x00000000,
        GPIOC_AFRL: 0x00000000,
        GPIOC_AFRH: 0x00000000,
        GPIOC_BRR: 0x00000000,
        GPIOC_HWCFGR10: 0x00011240,
        GPIOC_HWCFGR9: 0x000000FF,
        GPIOC_HWCFGR8: 0x00000000,
        GPIOC_HWCFGR7: 0xFFFFFFFF,
        GPIOC_HWCFGR6: 0xFFFFFFFF,
        GPIOC_HWCFGR5: 0x00000000,
        GPIOC_HWCFGR4: 0x00000000,
        GPIOC_HWCFGR3: 0x00000000,
        GPIOC_HWCFGR2: 0x00000000,
        GPIOC_HWCFGR1: 0x00000000,
        GPIOC_HWCFGR0: 0x00000000,
        GPIOC_VERR: 0x00000040,
        GPIOC_IPIDR: 0x000F0002,
        GPIOC_SIDR: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut GPIOC_TAKEN: bool = false;

    /// Safe access to GPIOC
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
            if GPIOC_TAKEN {
                None
            } else {
                GPIOC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to GPIOC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if GPIOC_TAKEN && inst.addr == INSTANCE.addr {
                GPIOC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal GPIOC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        GPIOC_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to GPIOC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPIOC: *const RegisterBlock = 0x50004000 as *const _;
