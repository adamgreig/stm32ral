#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! CRYP1
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::cryp::Instance;
pub use crate::stm32mp::peripherals::cryp::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::cryp::{
    CRYP_CR, CRYP_CSGCM0R, CRYP_CSGCM1R, CRYP_CSGCM2R, CRYP_CSGCM3R, CRYP_CSGCM4R, CRYP_CSGCM5R,
    CRYP_CSGCM6R, CRYP_CSGCM7R, CRYP_CSGCMCCM0R, CRYP_CSGCMCCM1R, CRYP_CSGCMCCM2R, CRYP_CSGCMCCM3R,
    CRYP_CSGCMCCM4R, CRYP_CSGCMCCM5R, CRYP_CSGCMCCM6R, CRYP_CSGCMCCM7R, CRYP_DIN, CRYP_DMACR,
    CRYP_DOUT, CRYP_HWCFGR, CRYP_IMSCR, CRYP_IPIDR, CRYP_IV0LR, CRYP_IV0RR, CRYP_IV1LR, CRYP_IV1RR,
    CRYP_K0LR, CRYP_K0RR, CRYP_K1LR, CRYP_K1RR, CRYP_K2LR, CRYP_K2RR, CRYP_K3LR, CRYP_K3RR,
    CRYP_MID, CRYP_MISR, CRYP_RISR, CRYP_SR, CRYP_VERR,
};

/// Access functions for the CRYP1 peripheral instance
pub mod CRYP1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x54001000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in CRYP1
    pub const reset: ResetValues = ResetValues {
        CRYP_CR: 0x00000000,
        CRYP_SR: 0x00000003,
        CRYP_DIN: 0x00000000,
        CRYP_DOUT: 0x00000000,
        CRYP_DMACR: 0x00000000,
        CRYP_IMSCR: 0x00000000,
        CRYP_RISR: 0x00000001,
        CRYP_MISR: 0x00000000,
        CRYP_K0LR: 0x00000000,
        CRYP_K0RR: 0x00000000,
        CRYP_K1LR: 0x00000000,
        CRYP_K1RR: 0x00000000,
        CRYP_K2LR: 0x00000000,
        CRYP_K2RR: 0x00000000,
        CRYP_K3LR: 0x00000000,
        CRYP_K3RR: 0x00000000,
        CRYP_IV0LR: 0x00000000,
        CRYP_IV0RR: 0x00000000,
        CRYP_IV1LR: 0x00000000,
        CRYP_IV1RR: 0x00000000,
        CRYP_CSGCMCCM0R: 0x00000000,
        CRYP_CSGCMCCM1R: 0x00000000,
        CRYP_CSGCMCCM2R: 0x00000000,
        CRYP_CSGCMCCM3R: 0x00000000,
        CRYP_CSGCMCCM4R: 0x00000000,
        CRYP_CSGCMCCM5R: 0x00000000,
        CRYP_CSGCMCCM6R: 0x00000000,
        CRYP_CSGCMCCM7R: 0x00000000,
        CRYP_CSGCM0R: 0x00000000,
        CRYP_CSGCM1R: 0x00000000,
        CRYP_CSGCM2R: 0x00000000,
        CRYP_CSGCM3R: 0x00000000,
        CRYP_CSGCM4R: 0x00000000,
        CRYP_CSGCM5R: 0x00000000,
        CRYP_CSGCM6R: 0x00000000,
        CRYP_CSGCM7R: 0x00000000,
        CRYP_HWCFGR: 0x00000131,
        CRYP_VERR: 0x00000022,
        CRYP_IPIDR: 0x00170011,
        CRYP_MID: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut CRYP1_TAKEN: bool = false;

    /// Safe access to CRYP1
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
            if CRYP1_TAKEN {
                None
            } else {
                CRYP1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to CRYP1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if CRYP1_TAKEN && inst.addr == INSTANCE.addr {
                CRYP1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal CRYP1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        CRYP1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to CRYP1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CRYP1: *const RegisterBlock = 0x54001000 as *const _;

/// Access functions for the CRYP2 peripheral instance
pub mod CRYP2 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x4c005000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in CRYP2
    pub const reset: ResetValues = ResetValues {
        CRYP_CR: 0x00000000,
        CRYP_SR: 0x00000003,
        CRYP_DIN: 0x00000000,
        CRYP_DOUT: 0x00000000,
        CRYP_DMACR: 0x00000000,
        CRYP_IMSCR: 0x00000000,
        CRYP_RISR: 0x00000001,
        CRYP_MISR: 0x00000000,
        CRYP_K0LR: 0x00000000,
        CRYP_K0RR: 0x00000000,
        CRYP_K1LR: 0x00000000,
        CRYP_K1RR: 0x00000000,
        CRYP_K2LR: 0x00000000,
        CRYP_K2RR: 0x00000000,
        CRYP_K3LR: 0x00000000,
        CRYP_K3RR: 0x00000000,
        CRYP_IV0LR: 0x00000000,
        CRYP_IV0RR: 0x00000000,
        CRYP_IV1LR: 0x00000000,
        CRYP_IV1RR: 0x00000000,
        CRYP_CSGCMCCM0R: 0x00000000,
        CRYP_CSGCMCCM1R: 0x00000000,
        CRYP_CSGCMCCM2R: 0x00000000,
        CRYP_CSGCMCCM3R: 0x00000000,
        CRYP_CSGCMCCM4R: 0x00000000,
        CRYP_CSGCMCCM5R: 0x00000000,
        CRYP_CSGCMCCM6R: 0x00000000,
        CRYP_CSGCMCCM7R: 0x00000000,
        CRYP_CSGCM0R: 0x00000000,
        CRYP_CSGCM1R: 0x00000000,
        CRYP_CSGCM2R: 0x00000000,
        CRYP_CSGCM3R: 0x00000000,
        CRYP_CSGCM4R: 0x00000000,
        CRYP_CSGCM5R: 0x00000000,
        CRYP_CSGCM6R: 0x00000000,
        CRYP_CSGCM7R: 0x00000000,
        CRYP_HWCFGR: 0x00000131,
        CRYP_VERR: 0x00000022,
        CRYP_IPIDR: 0x00170011,
        CRYP_MID: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut CRYP2_TAKEN: bool = false;

    /// Safe access to CRYP2
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
            if CRYP2_TAKEN {
                None
            } else {
                CRYP2_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to CRYP2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if CRYP2_TAKEN && inst.addr == INSTANCE.addr {
                CRYP2_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal CRYP2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        CRYP2_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to CRYP2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CRYP2: *const RegisterBlock = 0x4c005000 as *const _;
