#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! ECC controller is associated to each RAM area
//!
//! Used by: stm32h747cm4, stm32h747cm7

#[cfg(not(feature = "nosync"))]
pub use crate::stm32h7::peripherals::ramecc3::Instance;
pub use crate::stm32h7::peripherals::ramecc3::{RegisterBlock, ResetValues};
pub use crate::stm32h7::peripherals::ramecc3::{
    IER, M1CR, M1FAR, M1FDRH, M1FDRL, M1FECR, M1SR, M2CR, M2FAR, M2FDRH, M2FDRL, M2FECR, M2SR,
};

/// Access functions for the RAMECC3 peripheral instance
pub mod RAMECC3 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x58027000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in RAMECC3
    pub const reset: ResetValues = ResetValues {
        IER: 0x00000000,
        M1CR: 0x00000000,
        M2CR: 0x00000000,
        M1SR: 0x00000000,
        M2SR: 0x00000000,
        M1FAR: 0x00000000,
        M2FAR: 0x00000000,
        M1FDRL: 0x00000000,
        M2FDRL: 0x00000000,
        M1FDRH: 0x00000000,
        M2FDRH: 0x00000000,
        M1FECR: 0x00000000,
        M2FECR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut RAMECC3_TAKEN: bool = false;

    /// Safe access to RAMECC3
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
            if RAMECC3_TAKEN {
                None
            } else {
                RAMECC3_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to RAMECC3
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if RAMECC3_TAKEN && inst.addr == INSTANCE.addr {
                RAMECC3_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal RAMECC3
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        RAMECC3_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to RAMECC3
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const RAMECC3: *const RegisterBlock = 0x58027000 as *const _;
