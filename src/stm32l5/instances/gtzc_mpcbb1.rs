#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! GTZC_MPCBB1
//!
//! Used by: stm32l552, stm32l562

#[cfg(not(feature = "nosync"))]
pub use crate::stm32l5::peripherals::gtzc_mpcbb1::Instance;
pub use crate::stm32l5::peripherals::gtzc_mpcbb1::{RegisterBlock, ResetValues};
pub use crate::stm32l5::peripherals::gtzc_mpcbb1::{
    MPCBB1_CR, MPCBB1_LCKVTR1, MPCBB1_LCKVTR2, MPCBB1_VCTR0, MPCBB1_VCTR1, MPCBB1_VCTR10,
    MPCBB1_VCTR11, MPCBB1_VCTR12, MPCBB1_VCTR13, MPCBB1_VCTR14, MPCBB1_VCTR15, MPCBB1_VCTR16,
    MPCBB1_VCTR17, MPCBB1_VCTR18, MPCBB1_VCTR19, MPCBB1_VCTR2, MPCBB1_VCTR20, MPCBB1_VCTR21,
    MPCBB1_VCTR22, MPCBB1_VCTR23, MPCBB1_VCTR24, MPCBB1_VCTR25, MPCBB1_VCTR26, MPCBB1_VCTR27,
    MPCBB1_VCTR28, MPCBB1_VCTR29, MPCBB1_VCTR3, MPCBB1_VCTR30, MPCBB1_VCTR31, MPCBB1_VCTR32,
    MPCBB1_VCTR33, MPCBB1_VCTR34, MPCBB1_VCTR35, MPCBB1_VCTR36, MPCBB1_VCTR37, MPCBB1_VCTR38,
    MPCBB1_VCTR39, MPCBB1_VCTR4, MPCBB1_VCTR40, MPCBB1_VCTR41, MPCBB1_VCTR42, MPCBB1_VCTR43,
    MPCBB1_VCTR44, MPCBB1_VCTR45, MPCBB1_VCTR46, MPCBB1_VCTR47, MPCBB1_VCTR48, MPCBB1_VCTR49,
    MPCBB1_VCTR5, MPCBB1_VCTR50, MPCBB1_VCTR51, MPCBB1_VCTR52, MPCBB1_VCTR53, MPCBB1_VCTR54,
    MPCBB1_VCTR55, MPCBB1_VCTR56, MPCBB1_VCTR57, MPCBB1_VCTR58, MPCBB1_VCTR59, MPCBB1_VCTR6,
    MPCBB1_VCTR60, MPCBB1_VCTR61, MPCBB1_VCTR62, MPCBB1_VCTR63, MPCBB1_VCTR7, MPCBB1_VCTR8,
    MPCBB1_VCTR9,
};

/// Access functions for the GTZC_MPCBB1 peripheral instance
pub mod GTZC_MPCBB1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40032c00,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in GTZC_MPCBB1
    pub const reset: ResetValues = ResetValues {
        MPCBB1_CR: 0x00000000,
        MPCBB1_LCKVTR1: 0x00000000,
        MPCBB1_LCKVTR2: 0x00000000,
        MPCBB1_VCTR0: 0xFFFFFFFF,
        MPCBB1_VCTR1: 0xFFFFFFFF,
        MPCBB1_VCTR2: 0xFFFFFFFF,
        MPCBB1_VCTR3: 0xFFFFFFFF,
        MPCBB1_VCTR4: 0xFFFFFFFF,
        MPCBB1_VCTR5: 0xFFFFFFFF,
        MPCBB1_VCTR6: 0xFFFFFFFF,
        MPCBB1_VCTR7: 0xFFFFFFFF,
        MPCBB1_VCTR8: 0xFFFFFFFF,
        MPCBB1_VCTR9: 0xFFFFFFFF,
        MPCBB1_VCTR10: 0xFFFFFFFF,
        MPCBB1_VCTR11: 0xFFFFFFFF,
        MPCBB1_VCTR12: 0xFFFFFFFF,
        MPCBB1_VCTR13: 0xFFFFFFFF,
        MPCBB1_VCTR14: 0xFFFFFFFF,
        MPCBB1_VCTR15: 0xFFFFFFFF,
        MPCBB1_VCTR16: 0xFFFFFFFF,
        MPCBB1_VCTR17: 0xFFFFFFFF,
        MPCBB1_VCTR18: 0xFFFFFFFF,
        MPCBB1_VCTR19: 0xFFFFFFFF,
        MPCBB1_VCTR20: 0xFFFFFFFF,
        MPCBB1_VCTR21: 0xFFFFFFFF,
        MPCBB1_VCTR22: 0xFFFFFFFF,
        MPCBB1_VCTR23: 0xFFFFFFFF,
        MPCBB1_VCTR24: 0xFFFFFFFF,
        MPCBB1_VCTR25: 0xFFFFFFFF,
        MPCBB1_VCTR26: 0xFFFFFFFF,
        MPCBB1_VCTR27: 0xFFFFFFFF,
        MPCBB1_VCTR28: 0xFFFFFFFF,
        MPCBB1_VCTR29: 0xFFFFFFFF,
        MPCBB1_VCTR30: 0xFFFFFFFF,
        MPCBB1_VCTR31: 0xFFFFFFFF,
        MPCBB1_VCTR32: 0xFFFFFFFF,
        MPCBB1_VCTR33: 0xFFFFFFFF,
        MPCBB1_VCTR34: 0xFFFFFFFF,
        MPCBB1_VCTR35: 0xFFFFFFFF,
        MPCBB1_VCTR36: 0xFFFFFFFF,
        MPCBB1_VCTR37: 0xFFFFFFFF,
        MPCBB1_VCTR38: 0xFFFFFFFF,
        MPCBB1_VCTR39: 0xFFFFFFFF,
        MPCBB1_VCTR40: 0xFFFFFFFF,
        MPCBB1_VCTR41: 0xFFFFFFFF,
        MPCBB1_VCTR42: 0xFFFFFFFF,
        MPCBB1_VCTR43: 0xFFFFFFFF,
        MPCBB1_VCTR44: 0xFFFFFFFF,
        MPCBB1_VCTR45: 0xFFFFFFFF,
        MPCBB1_VCTR46: 0xFFFFFFFF,
        MPCBB1_VCTR47: 0xFFFFFFFF,
        MPCBB1_VCTR48: 0xFFFFFFFF,
        MPCBB1_VCTR49: 0xFFFFFFFF,
        MPCBB1_VCTR50: 0xFFFFFFFF,
        MPCBB1_VCTR51: 0xFFFFFFFF,
        MPCBB1_VCTR52: 0xFFFFFFFF,
        MPCBB1_VCTR53: 0xFFFFFFFF,
        MPCBB1_VCTR54: 0xFFFFFFFF,
        MPCBB1_VCTR55: 0xFFFFFFFF,
        MPCBB1_VCTR56: 0xFFFFFFFF,
        MPCBB1_VCTR57: 0xFFFFFFFF,
        MPCBB1_VCTR58: 0xFFFFFFFF,
        MPCBB1_VCTR59: 0xFFFFFFFF,
        MPCBB1_VCTR60: 0xFFFFFFFF,
        MPCBB1_VCTR61: 0xFFFFFFFF,
        MPCBB1_VCTR62: 0xFFFFFFFF,
        MPCBB1_VCTR63: 0xFFFFFFFF,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut GTZC_MPCBB1_TAKEN: bool = false;

    /// Safe access to GTZC_MPCBB1
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
            if GTZC_MPCBB1_TAKEN {
                None
            } else {
                GTZC_MPCBB1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to GTZC_MPCBB1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if GTZC_MPCBB1_TAKEN && inst.addr == INSTANCE.addr {
                GTZC_MPCBB1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal GTZC_MPCBB1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        GTZC_MPCBB1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to GTZC_MPCBB1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GTZC_MPCBB1: *const RegisterBlock = 0x40032c00 as *const _;

/// Access functions for the SEC_GTZC_MPCBB1 peripheral instance
pub mod SEC_GTZC_MPCBB1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50032c00,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SEC_GTZC_MPCBB1
    pub const reset: ResetValues = ResetValues {
        MPCBB1_CR: 0x00000000,
        MPCBB1_LCKVTR1: 0x00000000,
        MPCBB1_LCKVTR2: 0x00000000,
        MPCBB1_VCTR0: 0xFFFFFFFF,
        MPCBB1_VCTR1: 0xFFFFFFFF,
        MPCBB1_VCTR2: 0xFFFFFFFF,
        MPCBB1_VCTR3: 0xFFFFFFFF,
        MPCBB1_VCTR4: 0x00000000,
        MPCBB1_VCTR5: 0x00000000,
        MPCBB1_VCTR6: 0x00000000,
        MPCBB1_VCTR7: 0x00000000,
        MPCBB1_VCTR8: 0x00000000,
        MPCBB1_VCTR9: 0x00000000,
        MPCBB1_VCTR10: 0x00000000,
        MPCBB1_VCTR11: 0x00000000,
        MPCBB1_VCTR12: 0x00000000,
        MPCBB1_VCTR13: 0x00000000,
        MPCBB1_VCTR14: 0x00000000,
        MPCBB1_VCTR15: 0x00000000,
        MPCBB1_VCTR16: 0x00000000,
        MPCBB1_VCTR17: 0x00000000,
        MPCBB1_VCTR18: 0x00000000,
        MPCBB1_VCTR19: 0x00000000,
        MPCBB1_VCTR20: 0x00000000,
        MPCBB1_VCTR21: 0x00000000,
        MPCBB1_VCTR22: 0x00000000,
        MPCBB1_VCTR23: 0x00000000,
        MPCBB1_VCTR24: 0x00000000,
        MPCBB1_VCTR25: 0x00000000,
        MPCBB1_VCTR26: 0x00000000,
        MPCBB1_VCTR27: 0x00000000,
        MPCBB1_VCTR28: 0x00000000,
        MPCBB1_VCTR29: 0x00000000,
        MPCBB1_VCTR30: 0x00000000,
        MPCBB1_VCTR31: 0x00000000,
        MPCBB1_VCTR32: 0x00000000,
        MPCBB1_VCTR33: 0x00000000,
        MPCBB1_VCTR34: 0x00000000,
        MPCBB1_VCTR35: 0x00000000,
        MPCBB1_VCTR36: 0x00000000,
        MPCBB1_VCTR37: 0x00000000,
        MPCBB1_VCTR38: 0x00000000,
        MPCBB1_VCTR39: 0x00000000,
        MPCBB1_VCTR40: 0x00000000,
        MPCBB1_VCTR41: 0x00000000,
        MPCBB1_VCTR42: 0x00000000,
        MPCBB1_VCTR43: 0x00000000,
        MPCBB1_VCTR44: 0x00000000,
        MPCBB1_VCTR45: 0x00000000,
        MPCBB1_VCTR46: 0x00000000,
        MPCBB1_VCTR47: 0x00000000,
        MPCBB1_VCTR48: 0x00000000,
        MPCBB1_VCTR49: 0x00000000,
        MPCBB1_VCTR50: 0x00000000,
        MPCBB1_VCTR51: 0x00000000,
        MPCBB1_VCTR52: 0x00000000,
        MPCBB1_VCTR53: 0x00000000,
        MPCBB1_VCTR54: 0x00000000,
        MPCBB1_VCTR55: 0x00000000,
        MPCBB1_VCTR56: 0x00000000,
        MPCBB1_VCTR57: 0x00000000,
        MPCBB1_VCTR58: 0x00000000,
        MPCBB1_VCTR59: 0x00000000,
        MPCBB1_VCTR60: 0x00000000,
        MPCBB1_VCTR61: 0x00000000,
        MPCBB1_VCTR62: 0x00000000,
        MPCBB1_VCTR63: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SEC_GTZC_MPCBB1_TAKEN: bool = false;

    /// Safe access to SEC_GTZC_MPCBB1
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
            if SEC_GTZC_MPCBB1_TAKEN {
                None
            } else {
                SEC_GTZC_MPCBB1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SEC_GTZC_MPCBB1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SEC_GTZC_MPCBB1_TAKEN && inst.addr == INSTANCE.addr {
                SEC_GTZC_MPCBB1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SEC_GTZC_MPCBB1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SEC_GTZC_MPCBB1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SEC_GTZC_MPCBB1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SEC_GTZC_MPCBB1: *const RegisterBlock = 0x50032c00 as *const _;
