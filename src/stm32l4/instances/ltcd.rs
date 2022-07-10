#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Liquid crystal display controller
//!
//! Used by: stm32l4r5, stm32l4r9

#[cfg(not(feature = "nosync"))]
pub use crate::stm32l4::peripherals::ltcd::Instance;
pub use crate::stm32l4::peripherals::ltcd::{RegisterBlock, ResetValues};
pub use crate::stm32l4::peripherals::ltcd::{
    AWCR, BCCR, BPCR, CDSR, CPSR, GCR, ICR, IER, ISR, L1BFCR, L1CACR, L1CFBAR, L1CFBLNR, L1CFBLR,
    L1CKCR, L1CLUTWR, L1CR, L1DCCR, L1PFCR, L1WHPCR, L1WVPCR, L2BFCR, L2CACR, L2CFBAR, L2CFBLNR,
    L2CFBLR, L2CKCR, L2CLUTWR, L2CR, L2DCCR, L2PFCR, L2WHPCR, L2WVPCR, LIPCR, SRCR, SSCR, TWCR,
};

/// Access functions for the LTCD peripheral instance
pub mod LTCD {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40016800,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in LTCD
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
        L1CR: 0x00000000,
        L2CR: 0x00000000,
        L1WHPCR: 0x00000000,
        L2WHPCR: 0x00000000,
        L1WVPCR: 0x00000000,
        L2WVPCR: 0x00000000,
        L1CKCR: 0x00000000,
        L2CKCR: 0x00000000,
        L1PFCR: 0x00000000,
        L2PFCR: 0x00000000,
        L1CACR: 0x00000000,
        L2CACR: 0x00000000,
        L1DCCR: 0x00000000,
        L2DCCR: 0x00000000,
        L1BFCR: 0x00000000,
        L2BFCR: 0x00000000,
        L1CFBAR: 0x00000000,
        L2CFBAR: 0x00000000,
        L1CFBLR: 0x00000000,
        L2CFBLR: 0x00000000,
        L1CFBLNR: 0x00000000,
        L2CFBLNR: 0x00000000,
        L1CLUTWR: 0x00000000,
        L2CLUTWR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut LTCD_TAKEN: bool = false;

    /// Safe access to LTCD
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
            if LTCD_TAKEN {
                None
            } else {
                LTCD_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to LTCD
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if LTCD_TAKEN && inst.addr == INSTANCE.addr {
                LTCD_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal LTCD
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        LTCD_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to LTCD
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const LTCD: *const RegisterBlock = 0x40016800 as *const _;
