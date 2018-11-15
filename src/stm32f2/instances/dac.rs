#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Digital-to-analog converter
//!
//! Used by: stm32f215, stm32f217

#[cfg(not(feature = "nosync"))]
pub use stm32f2::peripherals::dac::Instance;
pub use stm32f2::peripherals::dac::{RegisterBlock, ResetValues};
pub use stm32f2::peripherals::dac::{
    CR, DHR12L1, DHR12L2, DHR12LD, DHR12R1, DHR12R2, DHR12RD, DHR8R1, DHR8R2, DHR8RD, DOR1, DOR2,
    SR, SWTRIGR,
};

/// Access functions for the DAC peripheral instance
pub mod DAC {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40007400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DAC
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000000,
        SWTRIGR: 0x00000000,
        DHR12R1: 0x00000000,
        DHR12L1: 0x00000000,
        DHR8R1: 0x00000000,
        DHR12R2: 0x00000000,
        DHR12L2: 0x00000000,
        DHR8R2: 0x00000000,
        DHR12RD: 0x00000000,
        DHR12LD: 0x00000000,
        DHR8RD: 0x00000000,
        DOR1: 0x00000000,
        DOR2: 0x00000000,
        SR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut DAC_TAKEN: bool = false;

    /// Safe access to DAC
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
            if DAC_TAKEN {
                None
            } else {
                DAC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to DAC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if DAC_TAKEN && inst.addr == INSTANCE.addr {
                DAC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }
}

/// Raw pointer to DAC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DAC: *const RegisterBlock = 0x40007400 as *const _;
