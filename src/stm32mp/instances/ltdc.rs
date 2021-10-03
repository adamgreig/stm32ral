#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! LTDC
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::ltdc::Instance;
pub use crate::stm32mp::peripherals::ltdc::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::ltdc::{
    LTDC_AWCR, LTDC_BCCR, LTDC_BPCR, LTDC_CDSR, LTDC_CPSR, LTDC_GC1R, LTDC_GC2R, LTDC_GCR,
    LTDC_ICR, LTDC_IDR, LTDC_IER, LTDC_ISR, LTDC_L1BFCR, LTDC_L1CACR, LTDC_L1CFBAR, LTDC_L1CFBLNR,
    LTDC_L1CFBLR, LTDC_L1CKCR, LTDC_L1CLUTWR, LTDC_L1CR, LTDC_L1DCCR, LTDC_L1PFCR, LTDC_L1WHPCR,
    LTDC_L1WVPCR, LTDC_L2BFCR, LTDC_L2CACR, LTDC_L2CFBAR, LTDC_L2CFBLNR, LTDC_L2CFBLR, LTDC_L2CKCR,
    LTDC_L2CLUTWR, LTDC_L2CR, LTDC_L2DCCR, LTDC_L2PFCR, LTDC_L2WHPCR, LTDC_L2WVPCR, LTDC_LCR,
    LTDC_LIPCR, LTDC_SRCR, LTDC_SSCR, LTDC_TWCR,
};

/// Access functions for the LTDC peripheral instance
pub mod LTDC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x5a001000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in LTDC
    pub const reset: ResetValues = ResetValues {
        LTDC_IDR: 0x00010300,
        LTDC_LCR: 0x00000002,
        LTDC_SSCR: 0x00000000,
        LTDC_BPCR: 0x00000000,
        LTDC_AWCR: 0x00000000,
        LTDC_TWCR: 0x00000000,
        LTDC_GCR: 0x00002220,
        LTDC_GC1R: 0x6BE2D888,
        LTDC_GC2R: 0x00000030,
        LTDC_SRCR: 0x00000000,
        LTDC_BCCR: 0x00000000,
        LTDC_IER: 0x00000000,
        LTDC_ISR: 0x00000000,
        LTDC_ICR: 0x00000000,
        LTDC_LIPCR: 0x00000000,
        LTDC_CPSR: 0x00000000,
        LTDC_CDSR: 0x0000000F,
        LTDC_L1CR: 0x00000000,
        LTDC_L1WHPCR: 0x00000000,
        LTDC_L1WVPCR: 0x00000000,
        LTDC_L1CKCR: 0x00000000,
        LTDC_L1PFCR: 0x00000000,
        LTDC_L1CACR: 0x000000FF,
        LTDC_L1DCCR: 0x00000000,
        LTDC_L1BFCR: 0x00000607,
        LTDC_L1CFBAR: 0x00000000,
        LTDC_L1CFBLR: 0x00000000,
        LTDC_L1CFBLNR: 0x00000000,
        LTDC_L1CLUTWR: 0x00000000,
        LTDC_L2CR: 0x00000000,
        LTDC_L2WHPCR: 0x00000000,
        LTDC_L2WVPCR: 0x00000000,
        LTDC_L2CKCR: 0x00000000,
        LTDC_L2PFCR: 0x00000000,
        LTDC_L2CACR: 0x000000FF,
        LTDC_L2DCCR: 0x00000000,
        LTDC_L2BFCR: 0x00000607,
        LTDC_L2CFBAR: 0x00000000,
        LTDC_L2CFBLR: 0x00000000,
        LTDC_L2CFBLNR: 0x00000000,
        LTDC_L2CLUTWR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut LTDC_TAKEN: bool = false;

    /// Safe access to LTDC
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
            if LTDC_TAKEN {
                None
            } else {
                LTDC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to LTDC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if LTDC_TAKEN && inst.addr == INSTANCE.addr {
                LTDC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal LTDC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        LTDC_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to LTDC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const LTDC: *const RegisterBlock = 0x5a001000 as *const _;
