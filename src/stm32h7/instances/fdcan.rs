#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! FDCAN1
//!
//! Used by: stm32h743, stm32h743v, stm32h747cm7, stm32h753, stm32h753v, stm32h757cm7

#[cfg(not(feature = "nosync"))]
pub use crate::stm32h7::peripherals::fdcan::Instance;
pub use crate::stm32h7::peripherals::fdcan::{RegisterBlock, ResetValues};
pub use crate::stm32h7::peripherals::fdcan::{
    CAN_TTGTP, CCCR, CREL, DBTP, ECR, ENDN, GFC, HPMS, IE, ILE, ILS, IR, NBTP, NDAT1, NDAT2, PSR,
    RWD, RXBC, RXESC, RXF0A, RXF0C, RXF0S, RXF1A, RXF1C, RXF1S, SIDFC, TDCR, TEST, TOCC, TOCV,
    TSCC, TSCV, TTCPT, TTCSM, TTCTC, TTIE, TTILS, TTIR, TTLGT, TTMLM, TTOCF, TTOCN, TTOST, TTRMC,
    TTTMC, TTTMK, TTTS, TURCF, TURNA, TXBAR, TXBC, TXBCF, TXBCIE, TXBCR, TXBRP, TXBTIE, TXBTO,
    TXEFA, TXEFC, TXEFS, TXESC, TXFQS, XIDAM, XIDFC,
};

/// Access functions for the FDCAN1 peripheral instance
pub mod FDCAN1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x4000a000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in FDCAN1
    pub const reset: ResetValues = ResetValues {
        CREL: 0x00000000,
        ENDN: 0x00000000,
        DBTP: 0x00000000,
        TEST: 0x00000000,
        RWD: 0x00000000,
        CCCR: 0x00000000,
        NBTP: 0x00000000,
        TSCC: 0x00000000,
        TSCV: 0x00000000,
        TOCC: 0x00000000,
        TOCV: 0x00000000,
        ECR: 0x00000000,
        PSR: 0x00000000,
        TDCR: 0x00000000,
        IR: 0x00000000,
        IE: 0x00000000,
        ILS: 0x00000000,
        ILE: 0x00000000,
        GFC: 0x00000000,
        SIDFC: 0x00000000,
        XIDFC: 0x00000000,
        XIDAM: 0x00000000,
        HPMS: 0x00000000,
        NDAT1: 0x00000000,
        NDAT2: 0x00000000,
        RXF0C: 0x00000000,
        RXF0S: 0x00000000,
        RXF0A: 0x00000000,
        RXBC: 0x00000000,
        RXF1C: 0x00000000,
        RXF1S: 0x00000000,
        RXF1A: 0x00000000,
        RXESC: 0x00000000,
        TXBC: 0x00000000,
        TXFQS: 0x00000000,
        TXESC: 0x00000000,
        TXBRP: 0x00000000,
        TXBAR: 0x00000000,
        TXBCR: 0x00000000,
        TXBTO: 0x00000000,
        TXBCF: 0x00000000,
        TXBTIE: 0x00000000,
        TXBCIE: 0x00000000,
        TXEFC: 0x00000000,
        TXEFS: 0x00000000,
        TXEFA: 0x00000000,
        TTTMC: 0x00000000,
        TTRMC: 0x00000000,
        TTOCF: 0x00000000,
        TTMLM: 0x00000000,
        TURCF: 0x00000000,
        TTOCN: 0x00000000,
        CAN_TTGTP: 0x00000000,
        TTTMK: 0x00000000,
        TTIR: 0x00000000,
        TTIE: 0x00000000,
        TTILS: 0x00000000,
        TTOST: 0x00000000,
        TURNA: 0x00000000,
        TTLGT: 0x00000000,
        TTCTC: 0x00000000,
        TTCPT: 0x00000000,
        TTCSM: 0x00000000,
        TTTS: 0x00000000,
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
pub const FDCAN1: *const RegisterBlock = 0x4000a000 as *const _;

/// Access functions for the FDCAN2 peripheral instance
pub mod FDCAN2 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x4000a400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in FDCAN2
    pub const reset: ResetValues = ResetValues {
        CREL: 0x00000000,
        ENDN: 0x00000000,
        DBTP: 0x00000000,
        TEST: 0x00000000,
        RWD: 0x00000000,
        CCCR: 0x00000000,
        NBTP: 0x00000000,
        TSCC: 0x00000000,
        TSCV: 0x00000000,
        TOCC: 0x00000000,
        TOCV: 0x00000000,
        ECR: 0x00000000,
        PSR: 0x00000000,
        TDCR: 0x00000000,
        IR: 0x00000000,
        IE: 0x00000000,
        ILS: 0x00000000,
        ILE: 0x00000000,
        GFC: 0x00000000,
        SIDFC: 0x00000000,
        XIDFC: 0x00000000,
        XIDAM: 0x00000000,
        HPMS: 0x00000000,
        NDAT1: 0x00000000,
        NDAT2: 0x00000000,
        RXF0C: 0x00000000,
        RXF0S: 0x00000000,
        RXF0A: 0x00000000,
        RXBC: 0x00000000,
        RXF1C: 0x00000000,
        RXF1S: 0x00000000,
        RXF1A: 0x00000000,
        RXESC: 0x00000000,
        TXBC: 0x00000000,
        TXFQS: 0x00000000,
        TXESC: 0x00000000,
        TXBRP: 0x00000000,
        TXBAR: 0x00000000,
        TXBCR: 0x00000000,
        TXBTO: 0x00000000,
        TXBCF: 0x00000000,
        TXBTIE: 0x00000000,
        TXBCIE: 0x00000000,
        TXEFC: 0x00000000,
        TXEFS: 0x00000000,
        TXEFA: 0x00000000,
        TTTMC: 0x00000000,
        TTRMC: 0x00000000,
        TTOCF: 0x00000000,
        TTMLM: 0x00000000,
        TURCF: 0x00000000,
        TTOCN: 0x00000000,
        CAN_TTGTP: 0x00000000,
        TTTMK: 0x00000000,
        TTIR: 0x00000000,
        TTIE: 0x00000000,
        TTILS: 0x00000000,
        TTOST: 0x00000000,
        TURNA: 0x00000000,
        TTLGT: 0x00000000,
        TTCTC: 0x00000000,
        TTCPT: 0x00000000,
        TTCSM: 0x00000000,
        TTTS: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut FDCAN2_TAKEN: bool = false;

    /// Safe access to FDCAN2
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
            if FDCAN2_TAKEN {
                None
            } else {
                FDCAN2_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to FDCAN2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if FDCAN2_TAKEN && inst.addr == INSTANCE.addr {
                FDCAN2_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal FDCAN2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        FDCAN2_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to FDCAN2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const FDCAN2: *const RegisterBlock = 0x4000a400 as *const _;
