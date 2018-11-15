#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Controller area network
//!
//! Used by: stm32f301, stm32f302, stm32f303, stm32f373, stm32f3x4, stm32f3x8

#[cfg(not(feature = "nosync"))]
pub use stm32f3::peripherals::can::Instance;
pub use stm32f3::peripherals::can::{RegisterBlock, ResetValues};
pub use stm32f3::peripherals::can::{
    BTR, ESR, F0R1, F0R2, F10R1, F10R2, F11R1, F11R2, F12R1, F12R2, F13R1, F13R2, F14R1, F14R2,
    F15R1, F15R2, F16R1, F16R2, F17R1, F17R2, F18R1, F18R2, F19R1, F19R2, F1R1, F1R2, F20R1, F20R2,
    F21R1, F21R2, F22R1, F22R2, F23R1, F23R2, F24R1, F24R2, F25R1, F25R2, F26R1, F26R2, F27R1,
    F27R2, F2R1, F2R2, F3R1, F3R2, F4R1, F4R2, F5R1, F5R2, F6R1, F6R2, F7R1, F7R2, F8R1, F8R2,
    F9R1, F9R2, FA1R, FFA1R, FM1R, FMR, FS1R, IER, MCR, MSR, RDH0R, RDH1R, RDL0R, RDL1R, RDT0R,
    RDT1R, RF0R, RF1R, RI0R, RI1R, TDH0R, TDH1R, TDH2R, TDL0R, TDL1R, TDL2R, TDT0R, TDT1R, TDT2R,
    TI0R, TI1R, TI2R, TSR,
};

/// Access functions for the CAN peripheral instance
pub mod CAN {
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

    /// Reset values for each field in CAN
    pub const reset: ResetValues = ResetValues {
        MCR: 0x00010002,
        MSR: 0x00000C02,
        TSR: 0x1C000000,
        RF0R: 0x00000000,
        RF1R: 0x00000000,
        IER: 0x00000000,
        ESR: 0x00000000,
        BTR: 0x01230000,
        TI0R: 0x00000000,
        TDT0R: 0x00000000,
        TDL0R: 0x00000000,
        TDH0R: 0x00000000,
        TI1R: 0x00000000,
        TDT1R: 0x00000000,
        TDL1R: 0x00000000,
        TDH1R: 0x00000000,
        TI2R: 0x00000000,
        TDT2R: 0x00000000,
        TDL2R: 0x00000000,
        TDH2R: 0x00000000,
        RI0R: 0x00000000,
        RDT0R: 0x00000000,
        RDL0R: 0x00000000,
        RDH0R: 0x00000000,
        RI1R: 0x00000000,
        RDT1R: 0x00000000,
        RDL1R: 0x00000000,
        RDH1R: 0x00000000,
        FMR: 0x2A1C0E01,
        FM1R: 0x00000000,
        FS1R: 0x00000000,
        FFA1R: 0x00000000,
        FA1R: 0x00000000,
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
        F14R1: 0x00000000,
        F14R2: 0x00000000,
        F15R1: 0x00000000,
        F15R2: 0x00000000,
        F16R1: 0x00000000,
        F16R2: 0x00000000,
        F17R1: 0x00000000,
        F17R2: 0x00000000,
        F18R1: 0x00000000,
        F18R2: 0x00000000,
        F19R1: 0x00000000,
        F19R2: 0x00000000,
        F20R1: 0x00000000,
        F20R2: 0x00000000,
        F21R1: 0x00000000,
        F21R2: 0x00000000,
        F22R1: 0x00000000,
        F22R2: 0x00000000,
        F23R1: 0x00000000,
        F23R2: 0x00000000,
        F24R1: 0x00000000,
        F24R2: 0x00000000,
        F25R1: 0x00000000,
        F25R2: 0x00000000,
        F26R1: 0x00000000,
        F26R2: 0x00000000,
        F27R1: 0x00000000,
        F27R2: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[no_mangle]
    static mut CAN_TAKEN: bool = false;

    /// Safe access to CAN
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
            if CAN_TAKEN {
                None
            } else {
                CAN_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to CAN
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if CAN_TAKEN && inst.addr == INSTANCE.addr {
                CAN_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }
}

/// Raw pointer to CAN
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CAN: *const RegisterBlock = 0x40006400 as *const _;
