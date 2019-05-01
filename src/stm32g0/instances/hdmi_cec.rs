#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! HDMI-CEC
//!
//! Used by: stm32g0x0, stm32g0x1

#[cfg(not(feature = "nosync"))]
pub use stm32g0::peripherals::hdmi_cec::Instance;
pub use stm32g0::peripherals::hdmi_cec::{RegisterBlock, ResetValues};
pub use stm32g0::peripherals::hdmi_cec::{CEC_CFGR, CEC_CR, CEC_IER, CEC_ISR, CEC_RXDR, CEC_TXDR};

/// Access functions for the HDMI_CEC peripheral instance
pub mod HDMI_CEC {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40007800,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in HDMI_CEC
    pub const reset: ResetValues = ResetValues {
        CEC_CR: 0x00000000,
        CEC_CFGR: 0x00000000,
        CEC_TXDR: 0x00000000,
        CEC_RXDR: 0x00000000,
        CEC_ISR: 0x00000000,
        CEC_IER: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut HDMI_CEC_TAKEN: bool = false;

    /// Safe access to HDMI_CEC
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
            if HDMI_CEC_TAKEN {
                None
            } else {
                HDMI_CEC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to HDMI_CEC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if HDMI_CEC_TAKEN && inst.addr == INSTANCE.addr {
                HDMI_CEC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }
}

/// Raw pointer to HDMI_CEC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const HDMI_CEC: *const RegisterBlock = 0x40007800 as *const _;
