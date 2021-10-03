#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! GPIOB
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::gpiob::Instance;
pub use crate::stm32mp::peripherals::gpiob::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::gpiob::{
    GPIOB_AFRH, GPIOB_AFRL, GPIOB_BRR, GPIOB_BSRR, GPIOB_HWCFGR0, GPIOB_HWCFGR1, GPIOB_HWCFGR10,
    GPIOB_HWCFGR2, GPIOB_HWCFGR3, GPIOB_HWCFGR4, GPIOB_HWCFGR5, GPIOB_HWCFGR6, GPIOB_HWCFGR7,
    GPIOB_HWCFGR8, GPIOB_HWCFGR9, GPIOB_IDR, GPIOB_IPIDR, GPIOB_LCKR, GPIOB_MODER, GPIOB_ODR,
    GPIOB_OSPEEDR, GPIOB_OTYPER, GPIOB_PUPDR, GPIOB_SIDR, GPIOB_VERR,
};

/// Access functions for the GPIOB peripheral instance
pub mod GPIOB {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50003000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in GPIOB
    pub const reset: ResetValues = ResetValues {
        GPIOB_MODER: 0xFFFFFFFF,
        GPIOB_OTYPER: 0x00000000,
        GPIOB_OSPEEDR: 0x00000000,
        GPIOB_PUPDR: 0x00000000,
        GPIOB_IDR: 0x00000000,
        GPIOB_ODR: 0x00000000,
        GPIOB_BSRR: 0x00000000,
        GPIOB_LCKR: 0x00000000,
        GPIOB_AFRL: 0x00000000,
        GPIOB_AFRH: 0x00000000,
        GPIOB_BRR: 0x00000000,
        GPIOB_HWCFGR10: 0x00011240,
        GPIOB_HWCFGR9: 0x000000FF,
        GPIOB_HWCFGR8: 0x00000000,
        GPIOB_HWCFGR7: 0xFFFFFFFF,
        GPIOB_HWCFGR6: 0xFFFFFFFF,
        GPIOB_HWCFGR5: 0x00000000,
        GPIOB_HWCFGR4: 0x00000000,
        GPIOB_HWCFGR3: 0x00000000,
        GPIOB_HWCFGR2: 0x00000000,
        GPIOB_HWCFGR1: 0x00000000,
        GPIOB_HWCFGR0: 0x00000000,
        GPIOB_VERR: 0x00000040,
        GPIOB_IPIDR: 0x000F0002,
        GPIOB_SIDR: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut GPIOB_TAKEN: bool = false;

    /// Safe access to GPIOB
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
            if GPIOB_TAKEN {
                None
            } else {
                GPIOB_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to GPIOB
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if GPIOB_TAKEN && inst.addr == INSTANCE.addr {
                GPIOB_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal GPIOB
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        GPIOB_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to GPIOB
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPIOB: *const RegisterBlock = 0x50003000 as *const _;
