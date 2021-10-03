#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! STGENC
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::stgenc::Instance;
pub use crate::stm32mp::peripherals::stgenc::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::stgenc::{
    STGENC_CIDR0, STGENC_CIDR1, STGENC_CIDR2, STGENC_CIDR3, STGENC_CNTCR, STGENC_CNTCVL,
    STGENC_CNTCVU, STGENC_CNTFID0, STGENC_CNTSR, STGENC_PIDR0, STGENC_PIDR1, STGENC_PIDR2,
    STGENC_PIDR3, STGENC_PIDR4, STGENC_PIDR5, STGENC_PIDR6, STGENC_PIDR7,
};

/// Access functions for the STGENC peripheral instance
pub mod STGENC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x5c008000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in STGENC
    pub const reset: ResetValues = ResetValues {
        STGENC_CNTCR: 0x00000000,
        STGENC_CNTSR: 0x00000000,
        STGENC_CNTCVL: 0x00000000,
        STGENC_CNTCVU: 0x00000000,
        STGENC_CNTFID0: 0x00000000,
        STGENC_PIDR4: 0x00000004,
        STGENC_PIDR5: 0x00000000,
        STGENC_PIDR6: 0x00000000,
        STGENC_PIDR7: 0x00000000,
        STGENC_PIDR0: 0x00000001,
        STGENC_PIDR1: 0x000000B1,
        STGENC_PIDR2: 0x0000001B,
        STGENC_PIDR3: 0x00000000,
        STGENC_CIDR0: 0x0000000D,
        STGENC_CIDR1: 0x000000F0,
        STGENC_CIDR2: 0x00000050,
        STGENC_CIDR3: 0x000000B1,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut STGENC_TAKEN: bool = false;

    /// Safe access to STGENC
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
            if STGENC_TAKEN {
                None
            } else {
                STGENC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to STGENC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if STGENC_TAKEN && inst.addr == INSTANCE.addr {
                STGENC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal STGENC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        STGENC_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to STGENC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const STGENC: *const RegisterBlock = 0x5c008000 as *const _;
