#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Controller area network
//!
//! Used by: stm32f102, stm32f103

#[cfg(not(feature = "nosync"))]
pub use stm32f1::peripherals::can::Instance;
pub use stm32f1::peripherals::can::{RegisterBlock, ResetValues};
pub use stm32f1::peripherals::can::{
    CAN_BTR, CAN_ESR, CAN_FA1R, CAN_FFA1R, CAN_FM1R, CAN_FMR, CAN_FS1R, CAN_IER, CAN_MCR, CAN_MSR,
    CAN_RDH0R, CAN_RDH1R, CAN_RDL0R, CAN_RDL1R, CAN_RDT0R, CAN_RDT1R, CAN_RF0R, CAN_RF1R, CAN_RI0R,
    CAN_RI1R, CAN_TDH0R, CAN_TDH1R, CAN_TDH2R, CAN_TDL0R, CAN_TDL1R, CAN_TDL2R, CAN_TDT0R,
    CAN_TDT1R, CAN_TDT2R, CAN_TI0R, CAN_TI1R, CAN_TI2R, CAN_TSR, F0R1, F0R2, F10R1, F10R2, F11R1,
    F11R2, F12R1, F12R2, F13R1, F13R2, F1R1, F1R2, F2R1, F2R2, F3R1, F3R2, F4R1, F4R2, F5R1, F5R2,
    F6R1, F6R2, F7R1, F7R2, F8R1, F8R2, F9R1, F9R2,
};

/// Access functions for the CAN1 peripheral instance
pub mod CAN1 {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40006400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in CAN1
    pub const reset: ResetValues = ResetValues {
        CAN_MCR: 0x00000000,
        CAN_MSR: 0x00000000,
        CAN_TSR: 0x00000000,
        CAN_RF0R: 0x00000000,
        CAN_RF1R: 0x00000000,
        CAN_IER: 0x00000000,
        CAN_ESR: 0x00000000,
        CAN_BTR: 0x00000000,
        CAN_TI0R: 0x00000000,
        CAN_TDT0R: 0x00000000,
        CAN_TDL0R: 0x00000000,
        CAN_TDH0R: 0x00000000,
        CAN_TI1R: 0x00000000,
        CAN_TDT1R: 0x00000000,
        CAN_TDL1R: 0x00000000,
        CAN_TDH1R: 0x00000000,
        CAN_TI2R: 0x00000000,
        CAN_TDT2R: 0x00000000,
        CAN_TDL2R: 0x00000000,
        CAN_TDH2R: 0x00000000,
        CAN_RI0R: 0x00000000,
        CAN_RDT0R: 0x00000000,
        CAN_RDL0R: 0x00000000,
        CAN_RDH0R: 0x00000000,
        CAN_RI1R: 0x00000000,
        CAN_RDT1R: 0x00000000,
        CAN_RDL1R: 0x00000000,
        CAN_RDH1R: 0x00000000,
        CAN_FMR: 0x00000000,
        CAN_FM1R: 0x00000000,
        CAN_FS1R: 0x00000000,
        CAN_FFA1R: 0x00000000,
        CAN_FA1R: 0x00000000,
        F0R1: 0x00000000,
        F0R2: 0x00000000,
        F1R1: 0x00000000,
        F1R2: 0x00000000,
        F2R1: 0x00000000,
        F2R2: 0x00000000,
        F3R1: 0x00000000,
        F3R2: 0x00000000,
        F4R1: 0x00000000,
        F4R2: 0x00000000,
        F5R1: 0x00000000,
        F5R2: 0x00000000,
        F6R1: 0x00000000,
        F6R2: 0x00000000,
        F7R1: 0x00000000,
        F7R2: 0x00000000,
        F8R1: 0x00000000,
        F8R2: 0x00000000,
        F9R1: 0x00000000,
        F9R2: 0x00000000,
        F10R1: 0x00000000,
        F10R2: 0x00000000,
        F11R1: 0x00000000,
        F11R2: 0x00000000,
        F12R1: 0x00000000,
        F12R2: 0x00000000,
        F13R1: 0x00000000,
        F13R2: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut CAN1_TAKEN: bool = false;

    /// Safe access to CAN1
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
            if CAN1_TAKEN {
                None
            } else {
                CAN1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to CAN1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if CAN1_TAKEN && inst.addr == INSTANCE.addr {
                CAN1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }
}

/// Raw pointer to CAN1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CAN1: *const RegisterBlock = 0x40006400 as *const _;

/// Access functions for the CAN2 peripheral instance
pub mod CAN2 {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40006800,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in CAN2
    pub const reset: ResetValues = ResetValues {
        CAN_MCR: 0x00000000,
        CAN_MSR: 0x00000000,
        CAN_TSR: 0x00000000,
        CAN_RF0R: 0x00000000,
        CAN_RF1R: 0x00000000,
        CAN_IER: 0x00000000,
        CAN_ESR: 0x00000000,
        CAN_BTR: 0x00000000,
        CAN_TI0R: 0x00000000,
        CAN_TDT0R: 0x00000000,
        CAN_TDL0R: 0x00000000,
        CAN_TDH0R: 0x00000000,
        CAN_TI1R: 0x00000000,
        CAN_TDT1R: 0x00000000,
        CAN_TDL1R: 0x00000000,
        CAN_TDH1R: 0x00000000,
        CAN_TI2R: 0x00000000,
        CAN_TDT2R: 0x00000000,
        CAN_TDL2R: 0x00000000,
        CAN_TDH2R: 0x00000000,
        CAN_RI0R: 0x00000000,
        CAN_RDT0R: 0x00000000,
        CAN_RDL0R: 0x00000000,
        CAN_RDH0R: 0x00000000,
        CAN_RI1R: 0x00000000,
        CAN_RDT1R: 0x00000000,
        CAN_RDL1R: 0x00000000,
        CAN_RDH1R: 0x00000000,
        CAN_FMR: 0x00000000,
        CAN_FM1R: 0x00000000,
        CAN_FS1R: 0x00000000,
        CAN_FFA1R: 0x00000000,
        CAN_FA1R: 0x00000000,
        F0R1: 0x00000000,
        F0R2: 0x00000000,
        F1R1: 0x00000000,
        F1R2: 0x00000000,
        F2R1: 0x00000000,
        F2R2: 0x00000000,
        F3R1: 0x00000000,
        F3R2: 0x00000000,
        F4R1: 0x00000000,
        F4R2: 0x00000000,
        F5R1: 0x00000000,
        F5R2: 0x00000000,
        F6R1: 0x00000000,
        F6R2: 0x00000000,
        F7R1: 0x00000000,
        F7R2: 0x00000000,
        F8R1: 0x00000000,
        F8R2: 0x00000000,
        F9R1: 0x00000000,
        F9R2: 0x00000000,
        F10R1: 0x00000000,
        F10R2: 0x00000000,
        F11R1: 0x00000000,
        F11R2: 0x00000000,
        F12R1: 0x00000000,
        F12R2: 0x00000000,
        F13R1: 0x00000000,
        F13R2: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut CAN2_TAKEN: bool = false;

    /// Safe access to CAN2
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
            if CAN2_TAKEN {
                None
            } else {
                CAN2_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to CAN2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if CAN2_TAKEN && inst.addr == INSTANCE.addr {
                CAN2_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }
}

/// Raw pointer to CAN2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CAN2: *const RegisterBlock = 0x40006800 as *const _;
