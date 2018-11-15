#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Management data input/output slave
//!
//! Used by: stm32f7x7, stm32f7x9

#[cfg(not(feature = "nosync"))]
pub use stm32f7::peripherals::mdios::Instance;
pub use stm32f7::peripherals::mdios::{RegisterBlock, ResetValues};
pub use stm32f7::peripherals::mdios::{
    MDIOS_CLRFR, MDIOS_CR, MDIOS_CRDFR, MDIOS_CWRFR, MDIOS_DINR0, MDIOS_DINR1, MDIOS_DINR10,
    MDIOS_DINR11, MDIOS_DINR12, MDIOS_DINR13, MDIOS_DINR14, MDIOS_DINR15, MDIOS_DINR16,
    MDIOS_DINR17, MDIOS_DINR18, MDIOS_DINR19, MDIOS_DINR2, MDIOS_DINR20, MDIOS_DINR21,
    MDIOS_DINR22, MDIOS_DINR23, MDIOS_DINR24, MDIOS_DINR25, MDIOS_DINR26, MDIOS_DINR27,
    MDIOS_DINR28, MDIOS_DINR29, MDIOS_DINR3, MDIOS_DINR30, MDIOS_DINR31, MDIOS_DINR4, MDIOS_DINR5,
    MDIOS_DINR6, MDIOS_DINR7, MDIOS_DINR8, MDIOS_DINR9, MDIOS_DOUTR0, MDIOS_DOUTR1, MDIOS_DOUTR10,
    MDIOS_DOUTR11, MDIOS_DOUTR12, MDIOS_DOUTR13, MDIOS_DOUTR14, MDIOS_DOUTR15, MDIOS_DOUTR16,
    MDIOS_DOUTR17, MDIOS_DOUTR18, MDIOS_DOUTR19, MDIOS_DOUTR2, MDIOS_DOUTR20, MDIOS_DOUTR21,
    MDIOS_DOUTR22, MDIOS_DOUTR23, MDIOS_DOUTR24, MDIOS_DOUTR25, MDIOS_DOUTR26, MDIOS_DOUTR27,
    MDIOS_DOUTR28, MDIOS_DOUTR29, MDIOS_DOUTR3, MDIOS_DOUTR30, MDIOS_DOUTR31, MDIOS_DOUTR4,
    MDIOS_DOUTR5, MDIOS_DOUTR6, MDIOS_DOUTR7, MDIOS_DOUTR8, MDIOS_DOUTR9, MDIOS_RDFR, MDIOS_SR,
    MDIOS_WRFR,
};

/// Access functions for the MDIOS peripheral instance
pub mod MDIOS {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40017800,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in MDIOS
    pub const reset: ResetValues = ResetValues {
        MDIOS_CR: 0x00000000,
        MDIOS_WRFR: 0x00000000,
        MDIOS_CWRFR: 0x00000000,
        MDIOS_RDFR: 0x00000000,
        MDIOS_CRDFR: 0x00000000,
        MDIOS_SR: 0x00000000,
        MDIOS_CLRFR: 0x00000000,
        MDIOS_DINR0: 0x00000000,
        MDIOS_DINR1: 0x00000000,
        MDIOS_DINR2: 0x00000000,
        MDIOS_DINR3: 0x00000000,
        MDIOS_DINR4: 0x00000000,
        MDIOS_DINR5: 0x00000000,
        MDIOS_DINR6: 0x00000000,
        MDIOS_DINR7: 0x00000000,
        MDIOS_DINR8: 0x00000000,
        MDIOS_DINR9: 0x00000000,
        MDIOS_DINR10: 0x00000000,
        MDIOS_DINR11: 0x00000000,
        MDIOS_DINR12: 0x00000000,
        MDIOS_DINR13: 0x00000000,
        MDIOS_DINR14: 0x00000000,
        MDIOS_DINR15: 0x00000000,
        MDIOS_DINR16: 0x00000000,
        MDIOS_DINR17: 0x00000000,
        MDIOS_DINR18: 0x00000000,
        MDIOS_DINR19: 0x00000000,
        MDIOS_DINR20: 0x00000000,
        MDIOS_DINR21: 0x00000000,
        MDIOS_DINR22: 0x00000000,
        MDIOS_DINR23: 0x00000000,
        MDIOS_DINR24: 0x00000000,
        MDIOS_DINR25: 0x00000000,
        MDIOS_DINR26: 0x00000000,
        MDIOS_DINR27: 0x00000000,
        MDIOS_DINR28: 0x00000000,
        MDIOS_DINR29: 0x00000000,
        MDIOS_DINR30: 0x00000000,
        MDIOS_DINR31: 0x00000000,
        MDIOS_DOUTR0: 0x00000000,
        MDIOS_DOUTR1: 0x00000000,
        MDIOS_DOUTR2: 0x00000000,
        MDIOS_DOUTR3: 0x00000000,
        MDIOS_DOUTR4: 0x00000000,
        MDIOS_DOUTR5: 0x00000000,
        MDIOS_DOUTR6: 0x00000000,
        MDIOS_DOUTR7: 0x00000000,
        MDIOS_DOUTR8: 0x00000000,
        MDIOS_DOUTR9: 0x00000000,
        MDIOS_DOUTR10: 0x00000000,
        MDIOS_DOUTR11: 0x00000000,
        MDIOS_DOUTR12: 0x00000000,
        MDIOS_DOUTR13: 0x00000000,
        MDIOS_DOUTR14: 0x00000000,
        MDIOS_DOUTR15: 0x00000000,
        MDIOS_DOUTR16: 0x00000000,
        MDIOS_DOUTR17: 0x00000000,
        MDIOS_DOUTR18: 0x00000000,
        MDIOS_DOUTR19: 0x00000000,
        MDIOS_DOUTR20: 0x00000000,
        MDIOS_DOUTR21: 0x00000000,
        MDIOS_DOUTR22: 0x00000000,
        MDIOS_DOUTR23: 0x00000000,
        MDIOS_DOUTR24: 0x00000000,
        MDIOS_DOUTR25: 0x00000000,
        MDIOS_DOUTR26: 0x00000000,
        MDIOS_DOUTR27: 0x00000000,
        MDIOS_DOUTR28: 0x00000000,
        MDIOS_DOUTR29: 0x00000000,
        MDIOS_DOUTR30: 0x00000000,
        MDIOS_DOUTR31: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[no_mangle]
    static mut MDIOS_TAKEN: bool = false;

    /// Safe access to MDIOS
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
            if MDIOS_TAKEN {
                None
            } else {
                MDIOS_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to MDIOS
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if MDIOS_TAKEN && inst.addr == INSTANCE.addr {
                MDIOS_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }
}

/// Raw pointer to MDIOS
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const MDIOS: *const RegisterBlock = 0x40017800 as *const _;
