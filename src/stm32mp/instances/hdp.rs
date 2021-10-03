#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! HDP
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::hdp::Instance;
pub use crate::stm32mp::peripherals::hdp::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::hdp::{
    HDP_CTRL, HDP_GPOCLR, HDP_GPOSET, HDP_GPOVAL, HDP_IPIDR, HDP_MUX, HDP_SIDR, HDP_VAL, HDP_VERR,
};

/// Access functions for the HDP peripheral instance
pub mod HDP {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x5002a000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in HDP
    pub const reset: ResetValues = ResetValues {
        HDP_CTRL: 0x00000000,
        HDP_MUX: 0x00000000,
        HDP_VAL: 0x00000000,
        HDP_GPOSET: 0x00000000,
        HDP_GPOCLR: 0x00000000,
        HDP_GPOVAL: 0x00000000,
        HDP_VERR: 0x00000010,
        HDP_IPIDR: 0x00030002,
        HDP_SIDR: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut HDP_TAKEN: bool = false;

    /// Safe access to HDP
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
            if HDP_TAKEN {
                None
            } else {
                HDP_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to HDP
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if HDP_TAKEN && inst.addr == INSTANCE.addr {
                HDP_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal HDP
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        HDP_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to HDP
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const HDP: *const RegisterBlock = 0x5002a000 as *const _;
