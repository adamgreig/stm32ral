#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! FDCAN1
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::fdcan::Instance;
pub use crate::stm32mp::peripherals::fdcan::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::fdcan::{
    FDCAN_CCCR, FDCAN_CREL, FDCAN_DBTP, FDCAN_ECR, FDCAN_ENDN, FDCAN_GFC, FDCAN_HPMS, FDCAN_IE,
    FDCAN_ILE, FDCAN_ILS, FDCAN_IR, FDCAN_NBTP, FDCAN_NDAT1, FDCAN_NDAT2, FDCAN_PSR, FDCAN_RWD,
    FDCAN_RXBC, FDCAN_RXESC, FDCAN_RXF0A, FDCAN_RXF0C, FDCAN_RXF0S, FDCAN_RXF1A, FDCAN_RXF1C,
    FDCAN_RXF1S, FDCAN_SIDFC, FDCAN_TDCR, FDCAN_TEST, FDCAN_TOCC, FDCAN_TOCV, FDCAN_TSCC,
    FDCAN_TSCV, FDCAN_TTCPT, FDCAN_TTCSM, FDCAN_TTCTC, FDCAN_TTGTP, FDCAN_TTIE, FDCAN_TTILS,
    FDCAN_TTIR, FDCAN_TTLGT, FDCAN_TTMLM, FDCAN_TTOCF, FDCAN_TTOCN, FDCAN_TTOST, FDCAN_TTRMC,
    FDCAN_TTTMC, FDCAN_TTTMK, FDCAN_TTTS, FDCAN_TURCF, FDCAN_TURNA, FDCAN_TXBAR, FDCAN_TXBC,
    FDCAN_TXBCF, FDCAN_TXBCIE, FDCAN_TXBCR, FDCAN_TXBTIE, FDCAN_TXBTO, FDCAN_TXEFA, FDCAN_TXEFC,
    FDCAN_TXEFS, FDCAN_TXESC, FDCAN_TXFQS, FDCAN_XIDAM, FDCAN_XIDFC,
};

/// Access functions for the FDCAN1 peripheral instance
pub mod FDCAN1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x4400e000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in FDCAN1
    pub const reset: ResetValues = ResetValues {
        FDCAN_CREL: 0x32141218,
        FDCAN_ENDN: 0x87654321,
        FDCAN_DBTP: 0x00000A33,
        FDCAN_TEST: 0x00000000,
        FDCAN_RWD: 0x00000000,
        FDCAN_CCCR: 0x00000001,
        FDCAN_NBTP: 0x00000A33,
        FDCAN_TSCC: 0x00000000,
        FDCAN_TSCV: 0x00000000,
        FDCAN_TOCC: 0xFFFF0000,
        FDCAN_TOCV: 0x0000FFFF,
        FDCAN_ECR: 0x00000000,
        FDCAN_PSR: 0x00000707,
        FDCAN_TDCR: 0x00000000,
        FDCAN_IR: 0x00000000,
        FDCAN_IE: 0x00000000,
        FDCAN_ILS: 0x00000000,
        FDCAN_ILE: 0x00000000,
        FDCAN_GFC: 0x00000000,
        FDCAN_SIDFC: 0x00000000,
        FDCAN_XIDFC: 0x00000000,
        FDCAN_XIDAM: 0x1FFFFFFF,
        FDCAN_HPMS: 0x00000000,
        FDCAN_NDAT1: 0x00000000,
        FDCAN_NDAT2: 0x00000000,
        FDCAN_RXF0C: 0x00000000,
        FDCAN_RXF0S: 0x00000000,
        FDCAN_RXF0A: 0x00000000,
        FDCAN_RXBC: 0x00000000,
        FDCAN_RXF1C: 0x00000000,
        FDCAN_RXF1S: 0x00000000,
        FDCAN_RXF1A: 0x00000000,
        FDCAN_RXESC: 0x00000000,
        FDCAN_TXBC: 0x00000000,
        FDCAN_TXFQS: 0x00000000,
        FDCAN_TXESC: 0x00000000,
        FDCAN_TXBAR: 0x00000000,
        FDCAN_TXBCR: 0x00000000,
        FDCAN_TXBTO: 0x00000000,
        FDCAN_TXBCF: 0x00000000,
        FDCAN_TXBTIE: 0x00000000,
        FDCAN_TXBCIE: 0x00000000,
        FDCAN_TXEFC: 0x00000000,
        FDCAN_TXEFS: 0x00000000,
        FDCAN_TXEFA: 0x00000000,
        FDCAN_TTTMC: 0x00000000,
        FDCAN_TTRMC: 0x00000000,
        FDCAN_TTOCF: 0x00010000,
        FDCAN_TTMLM: 0x00000000,
        FDCAN_TURCF: 0x00000000,
        FDCAN_TTOCN: 0x00000000,
        FDCAN_TTGTP: 0x00000000,
        FDCAN_TTTMK: 0x00000000,
        FDCAN_TTIR: 0x00000000,
        FDCAN_TTIE: 0x00000000,
        FDCAN_TTILS: 0x00000000,
        FDCAN_TTOST: 0x00000080,
        FDCAN_TURNA: 0x00000000,
        FDCAN_TTLGT: 0x00000000,
        FDCAN_TTCTC: 0x003F0000,
        FDCAN_TTCPT: 0x00000000,
        FDCAN_TTCSM: 0x00000000,
        FDCAN_TTTS: 0x00000000,
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
pub const FDCAN1: *const RegisterBlock = 0x4400e000 as *const _;

/// Access functions for the FDCAN2 peripheral instance
pub mod FDCAN2 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x4400f000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in FDCAN2
    pub const reset: ResetValues = ResetValues {
        FDCAN_CREL: 0x32141218,
        FDCAN_ENDN: 0x87654321,
        FDCAN_DBTP: 0x00000A33,
        FDCAN_TEST: 0x00000000,
        FDCAN_RWD: 0x00000000,
        FDCAN_CCCR: 0x00000001,
        FDCAN_NBTP: 0x00000A33,
        FDCAN_TSCC: 0x00000000,
        FDCAN_TSCV: 0x00000000,
        FDCAN_TOCC: 0xFFFF0000,
        FDCAN_TOCV: 0x0000FFFF,
        FDCAN_ECR: 0x00000000,
        FDCAN_PSR: 0x00000707,
        FDCAN_TDCR: 0x00000000,
        FDCAN_IR: 0x00000000,
        FDCAN_IE: 0x00000000,
        FDCAN_ILS: 0x00000000,
        FDCAN_ILE: 0x00000000,
        FDCAN_GFC: 0x00000000,
        FDCAN_SIDFC: 0x00000000,
        FDCAN_XIDFC: 0x00000000,
        FDCAN_XIDAM: 0x1FFFFFFF,
        FDCAN_HPMS: 0x00000000,
        FDCAN_NDAT1: 0x00000000,
        FDCAN_NDAT2: 0x00000000,
        FDCAN_RXF0C: 0x00000000,
        FDCAN_RXF0S: 0x00000000,
        FDCAN_RXF0A: 0x00000000,
        FDCAN_RXBC: 0x00000000,
        FDCAN_RXF1C: 0x00000000,
        FDCAN_RXF1S: 0x00000000,
        FDCAN_RXF1A: 0x00000000,
        FDCAN_RXESC: 0x00000000,
        FDCAN_TXBC: 0x00000000,
        FDCAN_TXFQS: 0x00000000,
        FDCAN_TXESC: 0x00000000,
        FDCAN_TXBAR: 0x00000000,
        FDCAN_TXBCR: 0x00000000,
        FDCAN_TXBTO: 0x00000000,
        FDCAN_TXBCF: 0x00000000,
        FDCAN_TXBTIE: 0x00000000,
        FDCAN_TXBCIE: 0x00000000,
        FDCAN_TXEFC: 0x00000000,
        FDCAN_TXEFS: 0x00000000,
        FDCAN_TXEFA: 0x00000000,
        FDCAN_TTTMC: 0x00000000,
        FDCAN_TTRMC: 0x00000000,
        FDCAN_TTOCF: 0x00010000,
        FDCAN_TTMLM: 0x00000000,
        FDCAN_TURCF: 0x00000000,
        FDCAN_TTOCN: 0x00000000,
        FDCAN_TTGTP: 0x00000000,
        FDCAN_TTTMK: 0x00000000,
        FDCAN_TTIR: 0x00000000,
        FDCAN_TTIE: 0x00000000,
        FDCAN_TTILS: 0x00000000,
        FDCAN_TTOST: 0x00000080,
        FDCAN_TURNA: 0x00000000,
        FDCAN_TTLGT: 0x00000000,
        FDCAN_TTCTC: 0x003F0000,
        FDCAN_TTCPT: 0x00000000,
        FDCAN_TTCSM: 0x00000000,
        FDCAN_TTTS: 0x00000000,
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
pub const FDCAN2: *const RegisterBlock = 0x4400f000 as *const _;
