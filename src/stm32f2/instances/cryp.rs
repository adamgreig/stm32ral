#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Cryptographic processor
//!
//! Used by: stm32f215, stm32f217

#[cfg(not(feature = "nosync"))]
pub use stm32f2::peripherals::cryp::Instance;
pub use stm32f2::peripherals::cryp::{RegisterBlock, ResetValues};
pub use stm32f2::peripherals::cryp::{
    CR, DIN, DMACR, DOUT, IMSCR, IV0LR, IV0RR, IV1LR, IV1RR, K0LR, K0RR, K1LR, K1RR, K2LR, K2RR,
    K3LR, K3RR, MISR, RISR, SR,
};

/// Access functions for the CRYP peripheral instance
pub mod CRYP {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50060000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in CRYP
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000000,
        SR: 0x00000003,
        DIN: 0x00000000,
        DOUT: 0x00000000,
        DMACR: 0x00000000,
        IMSCR: 0x00000000,
        RISR: 0x00000001,
        MISR: 0x00000000,
        K0LR: 0x00000000,
        K0RR: 0x00000000,
        K1LR: 0x00000000,
        K1RR: 0x00000000,
        K2LR: 0x00000000,
        K2RR: 0x00000000,
        K3LR: 0x00000000,
        K3RR: 0x00000000,
        IV0LR: 0x00000000,
        IV0RR: 0x00000000,
        IV1LR: 0x00000000,
        IV1RR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut CRYP_TAKEN: bool = false;

    /// Safe access to CRYP
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
            if CRYP_TAKEN {
                None
            } else {
                CRYP_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to CRYP
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if CRYP_TAKEN && inst.addr == INSTANCE.addr {
                CRYP_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }
}

/// Raw pointer to CRYP
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CRYP: *const RegisterBlock = 0x50060000 as *const _;
