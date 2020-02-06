#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Filter Math Accelerator
//!
//! Used by: stm32g431, stm32g441, stm32g471, stm32g473, stm32g474, stm32g483, stm32g484

#[cfg(not(feature = "nosync"))]
pub use crate::stm32g4::peripherals::fmac::Instance;
pub use crate::stm32g4::peripherals::fmac::{RegisterBlock, ResetValues};
pub use crate::stm32g4::peripherals::fmac::{
    CR, PARAM, RDATA, SR, WDATA, X1BUFCFG, X2BUFCFG, YBUFCFG,
};

/// Access functions for the FMAC peripheral instance
pub mod FMAC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40021400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in FMAC
    pub const reset: ResetValues = ResetValues {
        X1BUFCFG: 0x00000000,
        X2BUFCFG: 0x00000000,
        YBUFCFG: 0x00000000,
        PARAM: 0x00000000,
        CR: 0x00000000,
        SR: 0x00000000,
        WDATA: 0x00000000,
        RDATA: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut FMAC_TAKEN: bool = false;

    /// Safe access to FMAC
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
            if FMAC_TAKEN {
                None
            } else {
                FMAC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to FMAC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if FMAC_TAKEN && inst.addr == INSTANCE.addr {
                FMAC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal FMAC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        FMAC_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to FMAC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const FMAC: *const RegisterBlock = 0x40021400 as *const _;
