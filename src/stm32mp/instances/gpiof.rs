#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! GPIOF
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::gpiof::Instance;
pub use crate::stm32mp::peripherals::gpiof::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::gpiof::{
    GPIOF_AFRH, GPIOF_AFRL, GPIOF_BRR, GPIOF_BSRR, GPIOF_HWCFGR0, GPIOF_HWCFGR1, GPIOF_HWCFGR10,
    GPIOF_HWCFGR2, GPIOF_HWCFGR3, GPIOF_HWCFGR4, GPIOF_HWCFGR5, GPIOF_HWCFGR6, GPIOF_HWCFGR7,
    GPIOF_HWCFGR8, GPIOF_HWCFGR9, GPIOF_IDR, GPIOF_IPIDR, GPIOF_LCKR, GPIOF_MODER, GPIOF_ODR,
    GPIOF_OSPEEDR, GPIOF_OTYPER, GPIOF_PUPDR, GPIOF_SIDR, GPIOF_VERR,
};

/// Access functions for the GPIOF peripheral instance
pub mod GPIOF {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50007000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in GPIOF
    pub const reset: ResetValues = ResetValues {
        GPIOF_MODER: 0xFFFFFFFF,
        GPIOF_OTYPER: 0x00000000,
        GPIOF_OSPEEDR: 0x00000000,
        GPIOF_PUPDR: 0x00000000,
        GPIOF_IDR: 0x00000000,
        GPIOF_ODR: 0x00000000,
        GPIOF_BSRR: 0x00000000,
        GPIOF_LCKR: 0x00000000,
        GPIOF_AFRL: 0x00000000,
        GPIOF_AFRH: 0x00000000,
        GPIOF_BRR: 0x00000000,
        GPIOF_HWCFGR10: 0x00011240,
        GPIOF_HWCFGR9: 0x000000FF,
        GPIOF_HWCFGR8: 0x00000000,
        GPIOF_HWCFGR7: 0xFFFFFFFF,
        GPIOF_HWCFGR6: 0xFFFFFFFF,
        GPIOF_HWCFGR5: 0x00000000,
        GPIOF_HWCFGR4: 0x00000000,
        GPIOF_HWCFGR3: 0x00000000,
        GPIOF_HWCFGR2: 0x00000000,
        GPIOF_HWCFGR1: 0x00000000,
        GPIOF_HWCFGR0: 0x00000000,
        GPIOF_VERR: 0x00000040,
        GPIOF_IPIDR: 0x000F0002,
        GPIOF_SIDR: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut GPIOF_TAKEN: bool = false;

    /// Safe access to GPIOF
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
            if GPIOF_TAKEN {
                None
            } else {
                GPIOF_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to GPIOF
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if GPIOF_TAKEN && inst.addr == INSTANCE.addr {
                GPIOF_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal GPIOF
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        GPIOF_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to GPIOF
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPIOF: *const RegisterBlock = 0x50007000 as *const _;
