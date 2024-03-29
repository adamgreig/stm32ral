#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! USBPHYC
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::usbphyc::Instance;
pub use crate::stm32mp::peripherals::usbphyc::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::usbphyc::{
    USBPHYC_MISC, USBPHYC_PLL, USBPHYC_TUNE1, USBPHYC_TUNE2, USBPHYC_VERR,
};

/// Access functions for the USBPHYC peripheral instance
pub mod USBPHYC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x5a006000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in USBPHYC
    pub const reset: ResetValues = ResetValues {
        USBPHYC_PLL: 0xC0000000,
        USBPHYC_MISC: 0x00000000,
        USBPHYC_TUNE1: 0x04070004,
        USBPHYC_TUNE2: 0x04070004,
        USBPHYC_VERR: 0x00000010,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut USBPHYC_TAKEN: bool = false;

    /// Safe access to USBPHYC
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
            if USBPHYC_TAKEN {
                None
            } else {
                USBPHYC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to USBPHYC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if USBPHYC_TAKEN && inst.addr == INSTANCE.addr {
                USBPHYC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal USBPHYC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        USBPHYC_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to USBPHYC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const USBPHYC: *const RegisterBlock = 0x5a006000 as *const _;
