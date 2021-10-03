#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DAC
//!
//! Used by: stm32g071, stm32g07x, stm32g081

#[cfg(not(feature = "nosync"))]
pub use crate::stm32g0::peripherals::dac_v2::Instance;
pub use crate::stm32g0::peripherals::dac_v2::{RegisterBlock, ResetValues};
pub use crate::stm32g0::peripherals::dac_v2::{
    DAC_CCR, DAC_CR, DAC_DHR12L1, DAC_DHR12L2, DAC_DHR12LD, DAC_DHR12R1, DAC_DHR12R2, DAC_DHR12RD,
    DAC_DHR8R1, DAC_DHR8R2, DAC_DHR8RD, DAC_DOR1, DAC_DOR2, DAC_MCR, DAC_SHHR, DAC_SHRR, DAC_SHSR1,
    DAC_SHSR2, DAC_SR, DAC_SWTRGR, IPIDR, IP_HWCFGR0, SIDR, VERR,
};

/// Access functions for the DAC peripheral instance
pub mod DAC {
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
        DAC_CR: 0x00000000,
        DAC_SWTRGR: 0x00000000,
        DAC_DHR12R1: 0x00000000,
        DAC_DHR12L1: 0x00000000,
        DAC_DHR8R1: 0x00000000,
        DAC_DHR12R2: 0x00000000,
        DAC_DHR12L2: 0x00000000,
        DAC_DHR8R2: 0x00000000,
        DAC_DHR12RD: 0x00000000,
        DAC_DHR12LD: 0x00000000,
        DAC_DHR8RD: 0x00000000,
        DAC_DOR1: 0x00000000,
        DAC_DOR2: 0x00000000,
        DAC_SR: 0x00000000,
        DAC_CCR: 0x00000000,
        DAC_MCR: 0x00000000,
        DAC_SHSR1: 0x00000000,
        DAC_SHSR2: 0x00000000,
        DAC_SHHR: 0x00010001,
        DAC_SHRR: 0x00010001,
        IP_HWCFGR0: 0x00001111,
        VERR: 0x00000031,
        IPIDR: 0x00110011,
        SIDR: 0xA3C5DD01,
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

    /// Unsafely steal DAC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        DAC_TAKEN = true;
        INSTANCE
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
