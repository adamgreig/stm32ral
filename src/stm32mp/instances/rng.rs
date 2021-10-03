#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! RNG1
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::rng::Instance;
pub use crate::stm32mp::peripherals::rng::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::rng::{
    RNG_CR, RNG_DR, RNG_HWCFGR, RNG_IPIDR, RNG_SIDR, RNG_SR, RNG_VERR,
};

/// Access functions for the RNG1 peripheral instance
pub mod RNG1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x54003000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in RNG1
    pub const reset: ResetValues = ResetValues {
        RNG_CR: 0x00000000,
        RNG_SR: 0x00000000,
        RNG_DR: 0x00000000,
        RNG_HWCFGR: 0x00000006,
        RNG_VERR: 0x00000021,
        RNG_IPIDR: 0x00170041,
        RNG_SIDR: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut RNG1_TAKEN: bool = false;

    /// Safe access to RNG1
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
            if RNG1_TAKEN {
                None
            } else {
                RNG1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to RNG1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if RNG1_TAKEN && inst.addr == INSTANCE.addr {
                RNG1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal RNG1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        RNG1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to RNG1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const RNG1: *const RegisterBlock = 0x54003000 as *const _;

/// Access functions for the RNG2 peripheral instance
pub mod RNG2 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x4c003000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in RNG2
    pub const reset: ResetValues = ResetValues {
        RNG_CR: 0x00000000,
        RNG_SR: 0x00000000,
        RNG_DR: 0x00000000,
        RNG_HWCFGR: 0x00000006,
        RNG_VERR: 0x00000021,
        RNG_IPIDR: 0x00170041,
        RNG_SIDR: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut RNG2_TAKEN: bool = false;

    /// Safe access to RNG2
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
            if RNG2_TAKEN {
                None
            } else {
                RNG2_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to RNG2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if RNG2_TAKEN && inst.addr == INSTANCE.addr {
                RNG2_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal RNG2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        RNG2_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to RNG2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const RNG2: *const RegisterBlock = 0x4c003000 as *const _;
