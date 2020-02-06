#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Tamper and backup registers
//!
//! Used by: stm32g07x, stm32g081

#[cfg(not(feature = "nosync"))]
pub use crate::stm32g0::peripherals::tamp_v2::Instance;
pub use crate::stm32g0::peripherals::tamp_v2::{RegisterBlock, ResetValues};
pub use crate::stm32g0::peripherals::tamp_v2::{
    BKP0R, BKP1R, BKP2R, BKP3R, BKP4R, CR1, CR2, FLTCR, HWCFGR1, HWCFGR2, IER, IPIDR, MISR, SCR,
    SIDR, SR, VERR,
};

/// Access functions for the TAMP peripheral instance
pub mod TAMP {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x4000b000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in TAMP
    pub const reset: ResetValues = ResetValues {
        CR1: 0xFFFF0000,
        CR2: 0x00000000,
        FLTCR: 0x00000000,
        IER: 0x00000000,
        SR: 0x00000000,
        MISR: 0x00000000,
        SCR: 0x00000000,
        BKP0R: 0x00000000,
        BKP1R: 0x00000000,
        BKP2R: 0x00000000,
        BKP3R: 0x00000000,
        BKP4R: 0x00000000,
        HWCFGR2: 0x00000000,
        HWCFGR1: 0x00000000,
        VERR: 0x00000000,
        IPIDR: 0x00000000,
        SIDR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut TAMP_TAKEN: bool = false;

    /// Safe access to TAMP
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
            if TAMP_TAKEN {
                None
            } else {
                TAMP_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to TAMP
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if TAMP_TAKEN && inst.addr == INSTANCE.addr {
                TAMP_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal TAMP
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        TAMP_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to TAMP
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const TAMP: *const RegisterBlock = 0x4000b000 as *const _;
