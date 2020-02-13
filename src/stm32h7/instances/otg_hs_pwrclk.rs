#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! USB 1 on the go high speed
//!
//! Used by: stm32h743, stm32h743v, stm32h747cm4, stm32h747cm7, stm32h753, stm32h753v, stm32h757cm7

#[cfg(not(feature = "nosync"))]
pub use crate::stm32h7::peripherals::otg_hs_pwrclk::Instance;
pub use crate::stm32h7::peripherals::otg_hs_pwrclk::PCGCR;
pub use crate::stm32h7::peripherals::otg_hs_pwrclk::{RegisterBlock, ResetValues};

/// Access functions for the OTG1_HS_PWRCLK peripheral instance
pub mod OTG1_HS_PWRCLK {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40040e00,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in OTG1_HS_PWRCLK
    pub const reset: ResetValues = ResetValues { PCGCR: 0x00000000 };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut OTG1_HS_PWRCLK_TAKEN: bool = false;

    /// Safe access to OTG1_HS_PWRCLK
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
            if OTG1_HS_PWRCLK_TAKEN {
                None
            } else {
                OTG1_HS_PWRCLK_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to OTG1_HS_PWRCLK
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if OTG1_HS_PWRCLK_TAKEN && inst.addr == INSTANCE.addr {
                OTG1_HS_PWRCLK_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal OTG1_HS_PWRCLK
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        OTG1_HS_PWRCLK_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to OTG1_HS_PWRCLK
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const OTG1_HS_PWRCLK: *const RegisterBlock = 0x40040e00 as *const _;

/// Access functions for the OTG2_HS_PWRCLK peripheral instance
pub mod OTG2_HS_PWRCLK {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40080e00,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in OTG2_HS_PWRCLK
    pub const reset: ResetValues = ResetValues { PCGCR: 0x00000000 };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut OTG2_HS_PWRCLK_TAKEN: bool = false;

    /// Safe access to OTG2_HS_PWRCLK
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
            if OTG2_HS_PWRCLK_TAKEN {
                None
            } else {
                OTG2_HS_PWRCLK_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to OTG2_HS_PWRCLK
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if OTG2_HS_PWRCLK_TAKEN && inst.addr == INSTANCE.addr {
                OTG2_HS_PWRCLK_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal OTG2_HS_PWRCLK
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        OTG2_HS_PWRCLK_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to OTG2_HS_PWRCLK
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const OTG2_HS_PWRCLK: *const RegisterBlock = 0x40080e00 as *const _;
