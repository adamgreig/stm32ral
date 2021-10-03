#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! GPIOE
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::gpioe::Instance;
pub use crate::stm32mp::peripherals::gpioe::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::gpioe::{
    GPIOE_AFRH, GPIOE_AFRL, GPIOE_BRR, GPIOE_BSRR, GPIOE_HWCFGR0, GPIOE_HWCFGR1, GPIOE_HWCFGR10,
    GPIOE_HWCFGR2, GPIOE_HWCFGR3, GPIOE_HWCFGR4, GPIOE_HWCFGR5, GPIOE_HWCFGR6, GPIOE_HWCFGR7,
    GPIOE_HWCFGR8, GPIOE_HWCFGR9, GPIOE_IDR, GPIOE_IPIDR, GPIOE_LCKR, GPIOE_MODER, GPIOE_ODR,
    GPIOE_OSPEEDR, GPIOE_OTYPER, GPIOE_PUPDR, GPIOE_SIDR, GPIOE_VERR,
};

/// Access functions for the GPIOE peripheral instance
pub mod GPIOE {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50006000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in GPIOE
    pub const reset: ResetValues = ResetValues {
        GPIOE_MODER: 0xFFFFFFFF,
        GPIOE_OTYPER: 0x00000000,
        GPIOE_OSPEEDR: 0x00000000,
        GPIOE_PUPDR: 0x00000000,
        GPIOE_IDR: 0x00000000,
        GPIOE_ODR: 0x00000000,
        GPIOE_BSRR: 0x00000000,
        GPIOE_LCKR: 0x00000000,
        GPIOE_AFRL: 0x00000000,
        GPIOE_AFRH: 0x00000000,
        GPIOE_BRR: 0x00000000,
        GPIOE_HWCFGR10: 0x00011240,
        GPIOE_HWCFGR9: 0x000000FF,
        GPIOE_HWCFGR8: 0x00000000,
        GPIOE_HWCFGR7: 0xFFFFFFFF,
        GPIOE_HWCFGR6: 0xFFFFFFFF,
        GPIOE_HWCFGR5: 0x00000000,
        GPIOE_HWCFGR4: 0x00000000,
        GPIOE_HWCFGR3: 0x00000000,
        GPIOE_HWCFGR2: 0x00000000,
        GPIOE_HWCFGR1: 0x00000000,
        GPIOE_HWCFGR0: 0x00000000,
        GPIOE_VERR: 0x00000040,
        GPIOE_IPIDR: 0x000F0002,
        GPIOE_SIDR: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut GPIOE_TAKEN: bool = false;

    /// Safe access to GPIOE
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
            if GPIOE_TAKEN {
                None
            } else {
                GPIOE_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to GPIOE
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if GPIOE_TAKEN && inst.addr == INSTANCE.addr {
                GPIOE_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal GPIOE
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        GPIOE_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to GPIOE
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPIOE: *const RegisterBlock = 0x50006000 as *const _;
