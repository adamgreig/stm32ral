#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Management data input/output slave
//!
//! Used by: stm32h743, stm32h743v, stm32h747cm7, stm32h753, stm32h753v, stm32h757cm7

#[cfg(not(feature = "nosync"))]
pub use crate::stm32h7::peripherals::mdios::Instance;
pub use crate::stm32h7::peripherals::mdios::{RegisterBlock, ResetValues};
pub use crate::stm32h7::peripherals::mdios::{
    CLRFR, CR, CRDFR, CWRFR, DINR0, DINR1, DINR10, DINR11, DINR12, DINR13, DINR14, DINR15, DINR16,
    DINR17, DINR18, DINR19, DINR2, DINR20, DINR21, DINR22, DINR23, DINR24, DINR25, DINR26, DINR27,
    DINR28, DINR29, DINR3, DINR30, DINR31, DINR4, DINR5, DINR6, DINR7, DINR8, DINR9, DOUTR0,
    DOUTR1, DOUTR10, DOUTR11, DOUTR12, DOUTR13, DOUTR14, DOUTR15, DOUTR16, DOUTR17, DOUTR18,
    DOUTR19, DOUTR2, DOUTR20, DOUTR21, DOUTR22, DOUTR23, DOUTR24, DOUTR25, DOUTR26, DOUTR27,
    DOUTR28, DOUTR29, DOUTR3, DOUTR30, DOUTR31, DOUTR4, DOUTR5, DOUTR6, DOUTR7, DOUTR8, DOUTR9,
    RDFR, SR, WRFR,
};

/// Access functions for the MDIOS peripheral instance
pub mod MDIOS {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40009400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in MDIOS
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000000,
        WRFR: 0x00000000,
        CWRFR: 0x00000000,
        RDFR: 0x00000000,
        CRDFR: 0x00000000,
        SR: 0x00000000,
        CLRFR: 0x00000000,
        DINR0: 0x00000000,
        DINR1: 0x00000000,
        DINR2: 0x00000000,
        DINR3: 0x00000000,
        DINR4: 0x00000000,
        DINR5: 0x00000000,
        DINR6: 0x00000000,
        DINR7: 0x00000000,
        DINR8: 0x00000000,
        DINR9: 0x00000000,
        DINR10: 0x00000000,
        DINR11: 0x00000000,
        DINR12: 0x00000000,
        DINR13: 0x00000000,
        DINR14: 0x00000000,
        DINR15: 0x00000000,
        DINR16: 0x00000000,
        DINR17: 0x00000000,
        DINR18: 0x00000000,
        DINR19: 0x00000000,
        DINR20: 0x00000000,
        DINR21: 0x00000000,
        DINR22: 0x00000000,
        DINR23: 0x00000000,
        DINR24: 0x00000000,
        DINR25: 0x00000000,
        DINR26: 0x00000000,
        DINR27: 0x00000000,
        DINR28: 0x00000000,
        DINR29: 0x00000000,
        DINR30: 0x00000000,
        DINR31: 0x00000000,
        DOUTR0: 0x00000000,
        DOUTR1: 0x00000000,
        DOUTR2: 0x00000000,
        DOUTR3: 0x00000000,
        DOUTR4: 0x00000000,
        DOUTR5: 0x00000000,
        DOUTR6: 0x00000000,
        DOUTR7: 0x00000000,
        DOUTR8: 0x00000000,
        DOUTR9: 0x00000000,
        DOUTR10: 0x00000000,
        DOUTR11: 0x00000000,
        DOUTR12: 0x00000000,
        DOUTR13: 0x00000000,
        DOUTR14: 0x00000000,
        DOUTR15: 0x00000000,
        DOUTR16: 0x00000000,
        DOUTR17: 0x00000000,
        DOUTR18: 0x00000000,
        DOUTR19: 0x00000000,
        DOUTR20: 0x00000000,
        DOUTR21: 0x00000000,
        DOUTR22: 0x00000000,
        DOUTR23: 0x00000000,
        DOUTR24: 0x00000000,
        DOUTR25: 0x00000000,
        DOUTR26: 0x00000000,
        DOUTR27: 0x00000000,
        DOUTR28: 0x00000000,
        DOUTR29: 0x00000000,
        DOUTR30: 0x00000000,
        DOUTR31: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
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

    /// Unsafely steal MDIOS
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        MDIOS_TAKEN = true;
        INSTANCE
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
pub const MDIOS: *const RegisterBlock = 0x40009400 as *const _;
