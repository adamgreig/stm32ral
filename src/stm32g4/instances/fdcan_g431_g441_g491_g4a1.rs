#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! FDCAN
//!
//! Used by: stm32g431, stm32g441, stm32g491, stm32g4a1

#[cfg(not(feature = "nosync"))]
pub use crate::stm32g4::peripherals::fdcan::Instance;
pub use crate::stm32g4::peripherals::fdcan::{RegisterBlock, ResetValues};
pub use crate::stm32g4::peripherals::fdcan::{
    CCCR, CKDIV, CREL, DBTP, ECR, ENDN, HPMS, IE, ILE, ILS, IR, NBTP, PSR, RWD, RXF0A, RXF0S,
    RXF1A, RXF1S, RXGFC, TDCR, TEST, TOCC, TOCV, TSCC, TSCV, TXBAR, TXBC, TXBCF, TXBCIE, TXBCR,
    TXBRP, TXBTIE, TXBTO, TXEFA, TXEFS, TXFQS, XIDAM,
};

/// Access functions for the FDCAN peripheral instance
pub mod FDCAN {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x4000a400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in FDCAN
    pub const reset: ResetValues = ResetValues {
        CREL: 0x11111111,
        ENDN: 0x87654321,
        DBTP: 0x00000A33,
        TEST: 0x00000000,
        RWD: 0x00000000,
        CCCR: 0x00000001,
        NBTP: 0x00000A33,
        TSCC: 0x00000000,
        TSCV: 0x00000000,
        TOCC: 0xFFFF0000,
        TOCV: 0x0000FFFF,
        ECR: 0x00000000,
        PSR: 0x00000707,
        TDCR: 0x00000000,
        IR: 0x00000000,
        IE: 0x00000000,
        ILS: 0x00000000,
        ILE: 0x00000000,
        RXGFC: 0x00000000,
        XIDAM: 0x1FFFFFFF,
        HPMS: 0x00000000,
        RXF0S: 0x00000000,
        RXF0A: 0x00000000,
        RXF1S: 0x00000000,
        RXF1A: 0x00000000,
        TXBC: 0x00000000,
        TXFQS: 0x00000000,
        TXBRP: 0x00000000,
        TXBAR: 0x00000000,
        TXBCR: 0x00000000,
        TXBTO: 0x00000000,
        TXBCF: 0x00000000,
        TXBTIE: 0x00000000,
        TXBCIE: 0x00000000,
        TXEFS: 0x00000000,
        TXEFA: 0x00000000,
        CKDIV: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut FDCAN_TAKEN: bool = false;

    /// Safe access to FDCAN
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
            if FDCAN_TAKEN {
                None
            } else {
                FDCAN_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to FDCAN
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if FDCAN_TAKEN && inst.addr == INSTANCE.addr {
                FDCAN_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal FDCAN
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        FDCAN_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to FDCAN
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const FDCAN: *const RegisterBlock = 0x4000a400 as *const _;

/// Access functions for the FDCAN1 peripheral instance
pub mod FDCAN1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40006400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in FDCAN1
    pub const reset: ResetValues = ResetValues {
        CREL: 0x11111111,
        ENDN: 0x87654321,
        DBTP: 0x00000A33,
        TEST: 0x00000000,
        RWD: 0x00000000,
        CCCR: 0x00000001,
        NBTP: 0x00000A33,
        TSCC: 0x00000000,
        TSCV: 0x00000000,
        TOCC: 0xFFFF0000,
        TOCV: 0x0000FFFF,
        ECR: 0x00000000,
        PSR: 0x00000707,
        TDCR: 0x00000000,
        IR: 0x00000000,
        IE: 0x00000000,
        ILS: 0x00000000,
        ILE: 0x00000000,
        RXGFC: 0x00000000,
        XIDAM: 0x1FFFFFFF,
        HPMS: 0x00000000,
        RXF0S: 0x00000000,
        RXF0A: 0x00000000,
        RXF1S: 0x00000000,
        RXF1A: 0x00000000,
        TXBC: 0x00000000,
        TXFQS: 0x00000000,
        TXBRP: 0x00000000,
        TXBAR: 0x00000000,
        TXBCR: 0x00000000,
        TXBTO: 0x00000000,
        TXBCF: 0x00000000,
        TXBTIE: 0x00000000,
        TXBCIE: 0x00000000,
        TXEFS: 0x00000000,
        TXEFA: 0x00000000,
        CKDIV: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut FDCAN1_TAKEN: bool = false;

    /// Safe access to FDCAN1
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
            if FDCAN1_TAKEN {
                None
            } else {
                FDCAN1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to FDCAN1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if FDCAN1_TAKEN && inst.addr == INSTANCE.addr {
                FDCAN1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal FDCAN1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        FDCAN1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to FDCAN1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const FDCAN1: *const RegisterBlock = 0x40006400 as *const _;
