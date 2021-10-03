#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! IWDG1
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::iwdg::Instance;
pub use crate::stm32mp::peripherals::iwdg::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::iwdg::{
    IWDG_HWCFGR, IWDG_IDR, IWDG_KR, IWDG_PR, IWDG_RLR, IWDG_SIDR, IWDG_SR, IWDG_VERR, IWDG_WINR,
};

/// Access functions for the IWDG1 peripheral instance
pub mod IWDG1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x5c003000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in IWDG1
    pub const reset: ResetValues = ResetValues {
        IWDG_KR: 0x00000000,
        IWDG_PR: 0x00000007,
        IWDG_RLR: 0x00000FFF,
        IWDG_SR: 0x00000000,
        IWDG_WINR: 0x00000FFF,
        IWDG_HWCFGR: 0x00000071,
        IWDG_VERR: 0x00000023,
        IWDG_IDR: 0x00120041,
        IWDG_SIDR: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut IWDG1_TAKEN: bool = false;

    /// Safe access to IWDG1
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
            if IWDG1_TAKEN {
                None
            } else {
                IWDG1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to IWDG1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if IWDG1_TAKEN && inst.addr == INSTANCE.addr {
                IWDG1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal IWDG1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        IWDG1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to IWDG1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const IWDG1: *const RegisterBlock = 0x5c003000 as *const _;

/// Access functions for the IWDG2 peripheral instance
pub mod IWDG2 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x5a002000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in IWDG2
    pub const reset: ResetValues = ResetValues {
        IWDG_KR: 0x00000000,
        IWDG_PR: 0x00000007,
        IWDG_RLR: 0x00000FFF,
        IWDG_SR: 0x00000000,
        IWDG_WINR: 0x00000FFF,
        IWDG_HWCFGR: 0x00000071,
        IWDG_VERR: 0x00000023,
        IWDG_IDR: 0x00120041,
        IWDG_SIDR: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut IWDG2_TAKEN: bool = false;

    /// Safe access to IWDG2
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
            if IWDG2_TAKEN {
                None
            } else {
                IWDG2_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to IWDG2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if IWDG2_TAKEN && inst.addr == INSTANCE.addr {
                IWDG2_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal IWDG2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        IWDG2_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to IWDG2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const IWDG2: *const RegisterBlock = 0x5a002000 as *const _;
