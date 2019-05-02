#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Controller area network
//!
//! Used by: stm32f7x5, stm32f7x6

#[cfg(not(feature = "nosync"))]
pub use crate::stm32f7::peripherals::can::Instance;
pub use crate::stm32f7::peripherals::can::{RegisterBlock, ResetValues};
pub use crate::stm32f7::peripherals::can::{
    BTR, ESR, FA1R, FFA1R, FM1R, FMR, FR10, FR11, FR110, FR111, FR112, FR113, FR114, FR115, FR116,
    FR117, FR118, FR119, FR12, FR120, FR121, FR122, FR123, FR124, FR125, FR126, FR127, FR13, FR14,
    FR15, FR16, FR17, FR18, FR19, FR20, FR21, FR210, FR211, FR212, FR213, FR214, FR215, FR216,
    FR217, FR218, FR219, FR22, FR220, FR221, FR222, FR223, FR224, FR225, FR226, FR227, FR23, FR24,
    FR25, FR26, FR27, FR28, FR29, FS1R, IER, MCR, MSR, RDHR0, RDHR1, RDLR0, RDLR1, RDTR0, RDTR1,
    RF0R, RF1R, RIR0, RIR1, TDHR0, TDHR1, TDHR2, TDLR0, TDLR1, TDLR2, TDTR0, TDTR1, TDTR2, TIR0,
    TIR1, TIR2, TSR,
};

/// Access functions for the CAN1 peripheral instance
pub mod CAN1 {
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
        MCR: 0x00010002,
        MSR: 0x00000C02,
        TSR: 0x1C000000,
        RF0R: 0x00000000,
        RF1R: 0x00000000,
        IER: 0x00000000,
        ESR: 0x00000000,
        BTR: 0x00000000,
        FMR: 0x2A1C0E01,
        FM1R: 0x00000000,
        FS1R: 0x00000000,
        FFA1R: 0x00000000,
        FA1R: 0x00000000,
        TIR0: 0x00000000,
        TDTR0: 0x00000000,
        TDLR0: 0x00000000,
        TDHR0: 0x00000000,
        TIR1: 0x00000000,
        TDTR1: 0x00000000,
        TDLR1: 0x00000000,
        TDHR1: 0x00000000,
        TIR2: 0x00000000,
        TDTR2: 0x00000000,
        TDLR2: 0x00000000,
        TDHR2: 0x00000000,
        RIR0: 0x00000000,
        RDTR0: 0x00000000,
        RDLR0: 0x00000000,
        RDHR0: 0x00000000,
        RIR1: 0x00000000,
        RDTR1: 0x00000000,
        RDLR1: 0x00000000,
        RDHR1: 0x00000000,
        FR10: 0x00000000,
        FR20: 0x00000000,
        FR11: 0x00000000,
        FR21: 0x00000000,
        FR12: 0x00000000,
        FR22: 0x00000000,
        FR13: 0x00000000,
        FR23: 0x00000000,
        FR14: 0x00000000,
        FR24: 0x00000000,
        FR15: 0x00000000,
        FR25: 0x00000000,
        FR16: 0x00000000,
        FR26: 0x00000000,
        FR17: 0x00000000,
        FR27: 0x00000000,
        FR18: 0x00000000,
        FR28: 0x00000000,
        FR19: 0x00000000,
        FR29: 0x00000000,
        FR110: 0x00000000,
        FR210: 0x00000000,
        FR111: 0x00000000,
        FR211: 0x00000000,
        FR112: 0x00000000,
        FR212: 0x00000000,
        FR113: 0x00000000,
        FR213: 0x00000000,
        FR114: 0x00000000,
        FR214: 0x00000000,
        FR115: 0x00000000,
        FR215: 0x00000000,
        FR116: 0x00000000,
        FR216: 0x00000000,
        FR117: 0x00000000,
        FR217: 0x00000000,
        FR118: 0x00000000,
        FR218: 0x00000000,
        FR119: 0x00000000,
        FR219: 0x00000000,
        FR120: 0x00000000,
        FR220: 0x00000000,
        FR121: 0x00000000,
        FR221: 0x00000000,
        FR122: 0x00000000,
        FR222: 0x00000000,
        FR123: 0x00000000,
        FR223: 0x00000000,
        FR124: 0x00000000,
        FR224: 0x00000000,
        FR125: 0x00000000,
        FR225: 0x00000000,
        FR126: 0x00000000,
        FR226: 0x00000000,
        FR127: 0x00000000,
        FR227: 0x00000000,
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

    /// Unsafely steal CAN1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        CAN1_TAKEN = true;
        INSTANCE
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
        MCR: 0x00010002,
        MSR: 0x00000C02,
        TSR: 0x1C000000,
        RF0R: 0x00000000,
        RF1R: 0x00000000,
        IER: 0x00000000,
        ESR: 0x00000000,
        BTR: 0x00000000,
        FMR: 0x2A1C0E01,
        FM1R: 0x00000000,
        FS1R: 0x00000000,
        FFA1R: 0x00000000,
        FA1R: 0x00000000,
        TIR0: 0x00000000,
        TDTR0: 0x00000000,
        TDLR0: 0x00000000,
        TDHR0: 0x00000000,
        TIR1: 0x00000000,
        TDTR1: 0x00000000,
        TDLR1: 0x00000000,
        TDHR1: 0x00000000,
        TIR2: 0x00000000,
        TDTR2: 0x00000000,
        TDLR2: 0x00000000,
        TDHR2: 0x00000000,
        RIR0: 0x00000000,
        RDTR0: 0x00000000,
        RDLR0: 0x00000000,
        RDHR0: 0x00000000,
        RIR1: 0x00000000,
        RDTR1: 0x00000000,
        RDLR1: 0x00000000,
        RDHR1: 0x00000000,
        FR10: 0x00000000,
        FR20: 0x00000000,
        FR11: 0x00000000,
        FR21: 0x00000000,
        FR12: 0x00000000,
        FR22: 0x00000000,
        FR13: 0x00000000,
        FR23: 0x00000000,
        FR14: 0x00000000,
        FR24: 0x00000000,
        FR15: 0x00000000,
        FR25: 0x00000000,
        FR16: 0x00000000,
        FR26: 0x00000000,
        FR17: 0x00000000,
        FR27: 0x00000000,
        FR18: 0x00000000,
        FR28: 0x00000000,
        FR19: 0x00000000,
        FR29: 0x00000000,
        FR110: 0x00000000,
        FR210: 0x00000000,
        FR111: 0x00000000,
        FR211: 0x00000000,
        FR112: 0x00000000,
        FR212: 0x00000000,
        FR113: 0x00000000,
        FR213: 0x00000000,
        FR114: 0x00000000,
        FR214: 0x00000000,
        FR115: 0x00000000,
        FR215: 0x00000000,
        FR116: 0x00000000,
        FR216: 0x00000000,
        FR117: 0x00000000,
        FR217: 0x00000000,
        FR118: 0x00000000,
        FR218: 0x00000000,
        FR119: 0x00000000,
        FR219: 0x00000000,
        FR120: 0x00000000,
        FR220: 0x00000000,
        FR121: 0x00000000,
        FR221: 0x00000000,
        FR122: 0x00000000,
        FR222: 0x00000000,
        FR123: 0x00000000,
        FR223: 0x00000000,
        FR124: 0x00000000,
        FR224: 0x00000000,
        FR125: 0x00000000,
        FR225: 0x00000000,
        FR126: 0x00000000,
        FR226: 0x00000000,
        FR127: 0x00000000,
        FR227: 0x00000000,
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

    /// Unsafely steal CAN2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        CAN2_TAKEN = true;
        INSTANCE
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
