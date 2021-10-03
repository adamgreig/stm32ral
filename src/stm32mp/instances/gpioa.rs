#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! GPIOA
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::gpioa::Instance;
pub use crate::stm32mp::peripherals::gpioa::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::gpioa::{
    GPIOA_AFRH, GPIOA_AFRL, GPIOA_BRR, GPIOA_BSRR, GPIOA_HWCFGR0, GPIOA_HWCFGR1, GPIOA_HWCFGR10,
    GPIOA_HWCFGR2, GPIOA_HWCFGR3, GPIOA_HWCFGR4, GPIOA_HWCFGR5, GPIOA_HWCFGR6, GPIOA_HWCFGR7,
    GPIOA_HWCFGR8, GPIOA_HWCFGR9, GPIOA_IDR, GPIOA_IPIDR, GPIOA_LCKR, GPIOA_MODER, GPIOA_ODR,
    GPIOA_OSPEEDR, GPIOA_OTYPER, GPIOA_PUPDR, GPIOA_SIDR, GPIOA_VERR,
};

/// Access functions for the GPIOA peripheral instance
pub mod GPIOA {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50002000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in GPIOA
    pub const reset: ResetValues = ResetValues {
        GPIOA_MODER: 0xFFFFFFFF,
        GPIOA_OTYPER: 0x00000000,
        GPIOA_OSPEEDR: 0x00000000,
        GPIOA_PUPDR: 0x00000000,
        GPIOA_IDR: 0x00000000,
        GPIOA_ODR: 0x00000000,
        GPIOA_BSRR: 0x00000000,
        GPIOA_LCKR: 0x00000000,
        GPIOA_AFRL: 0x00000000,
        GPIOA_AFRH: 0x00000000,
        GPIOA_BRR: 0x00000000,
        GPIOA_HWCFGR10: 0x00011240,
        GPIOA_HWCFGR9: 0x000000FF,
        GPIOA_HWCFGR8: 0x00000000,
        GPIOA_HWCFGR7: 0xFFFFFFFF,
        GPIOA_HWCFGR6: 0xFFFFFFFF,
        GPIOA_HWCFGR5: 0x00000000,
        GPIOA_HWCFGR4: 0x00000000,
        GPIOA_HWCFGR3: 0x00000000,
        GPIOA_HWCFGR2: 0x00000000,
        GPIOA_HWCFGR1: 0x00000000,
        GPIOA_HWCFGR0: 0x00000000,
        GPIOA_VERR: 0x00000040,
        GPIOA_IPIDR: 0x000F0002,
        GPIOA_SIDR: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut GPIOA_TAKEN: bool = false;

    /// Safe access to GPIOA
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
            if GPIOA_TAKEN {
                None
            } else {
                GPIOA_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to GPIOA
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if GPIOA_TAKEN && inst.addr == INSTANCE.addr {
                GPIOA_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal GPIOA
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        GPIOA_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to GPIOA
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPIOA: *const RegisterBlock = 0x50002000 as *const _;
