#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DTS register block
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::dts::Instance;
pub use crate::stm32mp::peripherals::dts::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::dts::{
    DTS_CFGR1, DTS_DR, DTS_ICIFR, DTS_ITENR, DTS_ITR1, DTS_OR, DTS_RAMPVALR, DTS_SR, DTS_T0VALR1,
};

/// Access functions for the DTS peripheral instance
pub mod DTS {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50028000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DTS
    pub const reset: ResetValues = ResetValues {
        DTS_CFGR1: 0x00000000,
        DTS_T0VALR1: 0x00000000,
        DTS_RAMPVALR: 0x00000000,
        DTS_ITR1: 0x00000000,
        DTS_DR: 0x00000000,
        DTS_SR: 0x00000000,
        DTS_ITENR: 0x00000000,
        DTS_ICIFR: 0x00000000,
        DTS_OR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut DTS_TAKEN: bool = false;

    /// Safe access to DTS
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
            if DTS_TAKEN {
                None
            } else {
                DTS_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to DTS
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if DTS_TAKEN && inst.addr == INSTANCE.addr {
                DTS_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal DTS
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        DTS_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to DTS
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DTS: *const RegisterBlock = 0x50028000 as *const _;
