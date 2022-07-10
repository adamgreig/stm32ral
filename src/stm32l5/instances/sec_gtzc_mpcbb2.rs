#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! SEC_GTZC_MPCBB2
//!
//! Used by: stm32l552, stm32l562

#[cfg(not(feature = "nosync"))]
pub use crate::stm32l5::peripherals::sec_gtzc_mpcbb2::Instance;
pub use crate::stm32l5::peripherals::sec_gtzc_mpcbb2::{RegisterBlock, ResetValues};
pub use crate::stm32l5::peripherals::sec_gtzc_mpcbb2::{
    MPCBB2_CR, MPCBB2_LCKVTR1, MPCBB2_LCKVTR2, MPCBB2_VCTR0, MPCBB2_VCTR1, MPCBB2_VCTR10,
    MPCBB2_VCTR11, MPCBB2_VCTR12, MPCBB2_VCTR13, MPCBB2_VCTR14, MPCBB2_VCTR15, MPCBB2_VCTR16,
    MPCBB2_VCTR17, MPCBB2_VCTR18, MPCBB2_VCTR19, MPCBB2_VCTR2, MPCBB2_VCTR20, MPCBB2_VCTR21,
    MPCBB2_VCTR22, MPCBB2_VCTR23, MPCBB2_VCTR24, MPCBB2_VCTR25, MPCBB2_VCTR26, MPCBB2_VCTR27,
    MPCBB2_VCTR28, MPCBB2_VCTR29, MPCBB2_VCTR3, MPCBB2_VCTR30, MPCBB2_VCTR31, MPCBB2_VCTR32,
    MPCBB2_VCTR33, MPCBB2_VCTR34, MPCBB2_VCTR35, MPCBB2_VCTR36, MPCBB2_VCTR37, MPCBB2_VCTR38,
    MPCBB2_VCTR39, MPCBB2_VCTR4, MPCBB2_VCTR40, MPCBB2_VCTR41, MPCBB2_VCTR42, MPCBB2_VCTR43,
    MPCBB2_VCTR44, MPCBB2_VCTR45, MPCBB2_VCTR46, MPCBB2_VCTR47, MPCBB2_VCTR48, MPCBB2_VCTR49,
    MPCBB2_VCTR5, MPCBB2_VCTR50, MPCBB2_VCTR51, MPCBB2_VCTR52, MPCBB2_VCTR53, MPCBB2_VCTR54,
    MPCBB2_VCTR55, MPCBB2_VCTR56, MPCBB2_VCTR57, MPCBB2_VCTR58, MPCBB2_VCTR59, MPCBB2_VCTR6,
    MPCBB2_VCTR60, MPCBB2_VCTR61, MPCBB2_VCTR62, MPCBB2_VCTR63, MPCBB2_VCTR7, MPCBB2_VCTR8,
    MPCBB2_VCTR9,
};

/// Access functions for the SEC_GTZC_MPCBB2 peripheral instance
pub mod SEC_GTZC_MPCBB2 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50033000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SEC_GTZC_MPCBB2
    pub const reset: ResetValues = ResetValues {
        MPCBB2_CR: 0x00000000,
        MPCBB2_LCKVTR1: 0x00000000,
        MPCBB2_LCKVTR2: 0x00000000,
        MPCBB2_VCTR0: 0xFFFFFFFF,
        MPCBB2_VCTR1: 0xFFFFFFFF,
        MPCBB2_VCTR2: 0xFFFFFFFF,
        MPCBB2_VCTR3: 0xFFFFFFFF,
        MPCBB2_VCTR4: 0x00000000,
        MPCBB2_VCTR5: 0x00000000,
        MPCBB2_VCTR6: 0x00000000,
        MPCBB2_VCTR7: 0x00000000,
        MPCBB2_VCTR8: 0x00000000,
        MPCBB2_VCTR9: 0x00000000,
        MPCBB2_VCTR10: 0x00000000,
        MPCBB2_VCTR11: 0x00000000,
        MPCBB2_VCTR12: 0x00000000,
        MPCBB2_VCTR13: 0x00000000,
        MPCBB2_VCTR14: 0x00000000,
        MPCBB2_VCTR15: 0x00000000,
        MPCBB2_VCTR16: 0x00000000,
        MPCBB2_VCTR17: 0x00000000,
        MPCBB2_VCTR18: 0x00000000,
        MPCBB2_VCTR19: 0x00000000,
        MPCBB2_VCTR20: 0x00000000,
        MPCBB2_VCTR21: 0x00000000,
        MPCBB2_VCTR22: 0x00000000,
        MPCBB2_VCTR23: 0x00000000,
        MPCBB2_VCTR24: 0x00000000,
        MPCBB2_VCTR25: 0x00000000,
        MPCBB2_VCTR26: 0x00000000,
        MPCBB2_VCTR27: 0x00000000,
        MPCBB2_VCTR28: 0x00000000,
        MPCBB2_VCTR29: 0x00000000,
        MPCBB2_VCTR30: 0x00000000,
        MPCBB2_VCTR31: 0x00000000,
        MPCBB2_VCTR32: 0x00000000,
        MPCBB2_VCTR33: 0x00000000,
        MPCBB2_VCTR34: 0x00000000,
        MPCBB2_VCTR35: 0x00000000,
        MPCBB2_VCTR36: 0x00000000,
        MPCBB2_VCTR37: 0x00000000,
        MPCBB2_VCTR38: 0x00000000,
        MPCBB2_VCTR39: 0x00000000,
        MPCBB2_VCTR40: 0x00000000,
        MPCBB2_VCTR41: 0x00000000,
        MPCBB2_VCTR42: 0x00000000,
        MPCBB2_VCTR43: 0x00000000,
        MPCBB2_VCTR44: 0x00000000,
        MPCBB2_VCTR45: 0x00000000,
        MPCBB2_VCTR46: 0x00000000,
        MPCBB2_VCTR47: 0x00000000,
        MPCBB2_VCTR48: 0x00000000,
        MPCBB2_VCTR49: 0x00000000,
        MPCBB2_VCTR50: 0x00000000,
        MPCBB2_VCTR51: 0x00000000,
        MPCBB2_VCTR52: 0x00000000,
        MPCBB2_VCTR53: 0x00000000,
        MPCBB2_VCTR54: 0x00000000,
        MPCBB2_VCTR55: 0x00000000,
        MPCBB2_VCTR56: 0x00000000,
        MPCBB2_VCTR57: 0x00000000,
        MPCBB2_VCTR58: 0x00000000,
        MPCBB2_VCTR59: 0x00000000,
        MPCBB2_VCTR60: 0x00000000,
        MPCBB2_VCTR61: 0x00000000,
        MPCBB2_VCTR62: 0x00000000,
        MPCBB2_VCTR63: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SEC_GTZC_MPCBB2_TAKEN: bool = false;

    /// Safe access to SEC_GTZC_MPCBB2
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
            if SEC_GTZC_MPCBB2_TAKEN {
                None
            } else {
                SEC_GTZC_MPCBB2_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SEC_GTZC_MPCBB2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SEC_GTZC_MPCBB2_TAKEN && inst.addr == INSTANCE.addr {
                SEC_GTZC_MPCBB2_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SEC_GTZC_MPCBB2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SEC_GTZC_MPCBB2_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SEC_GTZC_MPCBB2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SEC_GTZC_MPCBB2: *const RegisterBlock = 0x50033000 as *const _;
