#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Secure digital input/output interface
//!
//! Used by: stm32l412, stm32l4x1, stm32l4x2, stm32l4x3, stm32l4x5

#[cfg(not(feature = "nosync"))]
pub use crate::stm32l4::peripherals::sdmmc::Instance;
pub use crate::stm32l4::peripherals::sdmmc::{RegisterBlock, ResetValues};
pub use crate::stm32l4::peripherals::sdmmc::{
    ARG, CLKCR, CMD, DCOUNT, DCTRL, DLEN, DTIMER, FIFO, FIFOCNT, ICR, MASK, POWER, RESP1, RESP2,
    RESP3, RESP4, RESPCMD, STA,
};

/// Access functions for the SDMMC peripheral instance
pub mod SDMMC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40012800,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SDMMC
    pub const reset: ResetValues = ResetValues {
        POWER: 0x00000000,
        CLKCR: 0x00000000,
        ARG: 0x00000000,
        CMD: 0x00000000,
        RESPCMD: 0x00000000,
        RESP1: 0x00000000,
        RESP2: 0x00000000,
        RESP3: 0x00000000,
        RESP4: 0x00000000,
        DTIMER: 0x00000000,
        DLEN: 0x00000000,
        DCTRL: 0x00000000,
        DCOUNT: 0x00000000,
        STA: 0x00000000,
        ICR: 0x00000000,
        MASK: 0x00000000,
        FIFOCNT: 0x00000000,
        FIFO: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SDMMC_TAKEN: bool = false;

    /// Safe access to SDMMC
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
            if SDMMC_TAKEN {
                None
            } else {
                SDMMC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SDMMC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SDMMC_TAKEN && inst.addr == INSTANCE.addr {
                SDMMC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SDMMC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SDMMC_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SDMMC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SDMMC: *const RegisterBlock = 0x40012800 as *const _;
