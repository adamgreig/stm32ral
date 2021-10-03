#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! GPIOJ
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::gpioj::Instance;
pub use crate::stm32mp::peripherals::gpioj::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::gpioj::{
    GPIOJ_AFRH, GPIOJ_AFRL, GPIOJ_BRR, GPIOJ_BSRR, GPIOJ_HWCFGR0, GPIOJ_HWCFGR1, GPIOJ_HWCFGR10,
    GPIOJ_HWCFGR2, GPIOJ_HWCFGR3, GPIOJ_HWCFGR4, GPIOJ_HWCFGR5, GPIOJ_HWCFGR6, GPIOJ_HWCFGR7,
    GPIOJ_HWCFGR8, GPIOJ_HWCFGR9, GPIOJ_IDR, GPIOJ_IPIDR, GPIOJ_LCKR, GPIOJ_MODER, GPIOJ_ODR,
    GPIOJ_OSPEEDR, GPIOJ_OTYPER, GPIOJ_PUPDR, GPIOJ_SIDR, GPIOJ_VERR,
};

/// Access functions for the GPIOJ peripheral instance
pub mod GPIOJ {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x5000b000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in GPIOJ
    pub const reset: ResetValues = ResetValues {
        GPIOJ_MODER: 0xFFFFFFFF,
        GPIOJ_OTYPER: 0x00000000,
        GPIOJ_OSPEEDR: 0x00000000,
        GPIOJ_PUPDR: 0x00000000,
        GPIOJ_IDR: 0x00000000,
        GPIOJ_ODR: 0x00000000,
        GPIOJ_BSRR: 0x00000000,
        GPIOJ_LCKR: 0x00000000,
        GPIOJ_AFRL: 0x00000000,
        GPIOJ_AFRH: 0x00000000,
        GPIOJ_BRR: 0x00000000,
        GPIOJ_HWCFGR10: 0x00011240,
        GPIOJ_HWCFGR9: 0x000000FF,
        GPIOJ_HWCFGR8: 0x00000000,
        GPIOJ_HWCFGR7: 0xFFFFFFFF,
        GPIOJ_HWCFGR6: 0xFFFFFFFF,
        GPIOJ_HWCFGR5: 0x00000000,
        GPIOJ_HWCFGR4: 0x00000000,
        GPIOJ_HWCFGR3: 0x00000000,
        GPIOJ_HWCFGR2: 0x00000000,
        GPIOJ_HWCFGR1: 0x00000000,
        GPIOJ_HWCFGR0: 0x00000000,
        GPIOJ_VERR: 0x00000040,
        GPIOJ_IPIDR: 0x000F0002,
        GPIOJ_SIDR: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut GPIOJ_TAKEN: bool = false;

    /// Safe access to GPIOJ
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
            if GPIOJ_TAKEN {
                None
            } else {
                GPIOJ_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to GPIOJ
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if GPIOJ_TAKEN && inst.addr == INSTANCE.addr {
                GPIOJ_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal GPIOJ
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        GPIOJ_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to GPIOJ
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GPIOJ: *const RegisterBlock = 0x5000b000 as *const _;
