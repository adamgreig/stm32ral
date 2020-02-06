#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! LCD-TFT Controller
//!
//! Used by: stm32h743, stm32h743v, stm32h747cm7, stm32h753, stm32h753v, stm32h757cm7

#[cfg(not(feature = "nosync"))]
pub use crate::stm32h7::peripherals::ltdc::Instance;
pub use crate::stm32h7::peripherals::ltdc::{RegisterBlock, ResetValues};
pub use crate::stm32h7::peripherals::ltdc::{
    AWCR, BCCR, BFCR1, BFCR2, BPCR, CACR1, CACR2, CDSR, CFBAR1, CFBAR2, CFBLNR1, CFBLNR2, CFBLR1,
    CFBLR2, CKCR1, CKCR2, CLUTWR1, CLUTWR2, CPSR, CR1, CR2, DCCR1, DCCR2, GCR, ICR, IER, ISR,
    LIPCR, PFCR1, PFCR2, SRCR, SSCR, TWCR, WHPCR1, WHPCR2, WVPCR1, WVPCR2,
};

/// Access functions for the LTDC peripheral instance
pub mod LTDC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50001000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in LTDC
    pub const reset: ResetValues = ResetValues {
        SSCR: 0x00000000,
        BPCR: 0x00000000,
        AWCR: 0x00000000,
        TWCR: 0x00000000,
        GCR: 0x00002220,
        SRCR: 0x00000000,
        BCCR: 0x00000000,
        IER: 0x00000000,
        ISR: 0x00000000,
        ICR: 0x00000000,
        LIPCR: 0x00000000,
        CPSR: 0x00000000,
        CDSR: 0x0000000F,
        CR1: 0x00000000,
        WHPCR1: 0x00000000,
        WVPCR1: 0x00000000,
        CKCR1: 0x00000000,
        PFCR1: 0x00000000,
        CACR1: 0x00000000,
        DCCR1: 0x00000000,
        BFCR1: 0x00000607,
        CFBAR1: 0x00000000,
        CFBLR1: 0x00000000,
        CFBLNR1: 0x00000000,
        CLUTWR1: 0x00000000,
        CR2: 0x00000000,
        WHPCR2: 0x00000000,
        WVPCR2: 0x00000000,
        CKCR2: 0x00000000,
        PFCR2: 0x00000000,
        CACR2: 0x00000000,
        DCCR2: 0x00000000,
        BFCR2: 0x00000607,
        CFBAR2: 0x00000000,
        CFBLR2: 0x00000000,
        CFBLNR2: 0x00000000,
        CLUTWR2: 0x00000000,
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
pub const LTDC: *const RegisterBlock = 0x50001000 as *const _;
