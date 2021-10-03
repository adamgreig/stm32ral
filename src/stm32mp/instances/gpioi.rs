#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! GPIOI
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::gpioi::Instance;
pub use crate::stm32mp::peripherals::gpioi::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::gpioi::{
    GPIOI_AFRH, GPIOI_AFRL, GPIOI_BRR, GPIOI_BSRR, GPIOI_HWCFGR0, GPIOI_HWCFGR1, GPIOI_HWCFGR10,
    GPIOI_HWCFGR2, GPIOI_HWCFGR3, GPIOI_HWCFGR4, GPIOI_HWCFGR5, GPIOI_HWCFGR6, GPIOI_HWCFGR7,
    GPIOI_HWCFGR8, GPIOI_HWCFGR9, GPIOI_IDR, GPIOI_IPIDR, GPIOI_LCKR, GPIOI_MODER, GPIOI_ODR,
    GPIOI_OSPEEDR, GPIOI_OTYPER, GPIOI_PUPDR, GPIOI_SIDR, GPIOI_VERR,
};

/// Access functions for the GPIOI peripheral instance
pub mod GPIOI {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x5000a000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in GPIOI
    pub const reset: ResetValues = ResetValues {
        GPIOI_MODER: 0xFFFFFFFF,
        GPIOI_OTYPER: 0x00000000,
        GPIOI_OSPEEDR: 0x00000000,
        GPIOI_PUPDR: 0x00000000,
        GPIOI_IDR: 0x00000000,
        GPIOI_ODR: 0x00000000,
        GPIOI_BSRR: 0x00000000,
        GPIOI_LCKR: 0x00000000,
        GPIOI_AFRL: 0x00000000,
        GPIOI_AFRH: 0x00000000,
        GPIOI_BRR: 0x00000000,
        GPIOI_HWCFGR10: 0x00011240,
        GPIOI_HWCFGR9: 0x000000FF,
        GPIOI_HWCFGR8: 0x00000000,
        GPIOI_HWCFGR7: 0xFFFFFFFF,
        GPIOI_HWCFGR6: 0xFFFFFFFF,
        GPIOI_HWCFGR5: 0x00000000,
        GPIOI_HWCFGR4: 0x00000000,
        GPIOI_HWCFGR3: 0x00000000,
        GPIOI_HWCFGR2: 0x00000000,
        GPIOI_HWCFGR1: 0x00000000,
        GPIOI_HWCFGR0: 0x00000000,
        GPIOI_VERR: 0x00000040,
        GPIOI_IPIDR: 0x000F0002,
        GPIOI_SIDR: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut GPIOI_TAKEN: bool = false;

    /// Safe access to GPIOI
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
            if GPIOI_TAKEN {
                None
            } else {
                GPIOI_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to GPIOI
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if GPIOI_TAKEN && inst.addr == INSTANCE.addr {
                GPIOI_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal GPIOI
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        GPIOI_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to GPIOI
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPIOI: *const RegisterBlock = 0x5000a000 as *const _;
