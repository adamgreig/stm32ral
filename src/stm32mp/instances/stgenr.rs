#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! STGENR
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::stgenr::Instance;
pub use crate::stm32mp::peripherals::stgenr::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::stgenr::{
    STGENR_CIDR0, STGENR_CIDR1, STGENR_CIDR2, STGENR_CIDR3, STGENR_CNTCVL, STGENR_CNTCVU,
    STGENR_PIDR0, STGENR_PIDR1, STGENR_PIDR2, STGENR_PIDR3, STGENR_PIDR4, STGENR_PIDR5,
    STGENR_PIDR6, STGENR_PIDR7,
};

/// Access functions for the STGENR peripheral instance
pub mod STGENR {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x5a005000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in STGENR
    pub const reset: ResetValues = ResetValues {
        STGENR_CNTCVL: 0x00000000,
        STGENR_CNTCVU: 0x00000000,
        STGENR_PIDR4: 0x00000004,
        STGENR_PIDR5: 0x00000000,
        STGENR_PIDR6: 0x00000000,
        STGENR_PIDR7: 0x00000000,
        STGENR_PIDR0: 0x00000001,
        STGENR_PIDR1: 0x000000B1,
        STGENR_PIDR2: 0x0000001B,
        STGENR_PIDR3: 0x00000000,
        STGENR_CIDR0: 0x0000000D,
        STGENR_CIDR1: 0x000000F0,
        STGENR_CIDR2: 0x00000050,
        STGENR_CIDR3: 0x000000B1,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut STGENR_TAKEN: bool = false;

    /// Safe access to STGENR
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
            if STGENR_TAKEN {
                None
            } else {
                STGENR_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to STGENR
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if STGENR_TAKEN && inst.addr == INSTANCE.addr {
                STGENR_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal STGENR
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        STGENR_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to STGENR
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const STGENR: *const RegisterBlock = 0x5a005000 as *const _;
