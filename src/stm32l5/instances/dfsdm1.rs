#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Digital filter for sigma delta modulators
//!
//! Used by: stm32l552, stm32l562

#[cfg(not(feature = "nosync"))]
pub use crate::stm32l5::peripherals::dfsdm1::Instance;
pub use crate::stm32l5::peripherals::dfsdm1::{RegisterBlock, ResetValues};
pub use crate::stm32l5::peripherals::dfsdm1::{
    CH0AWSCDR, CH0CFGR1, CH0CFGR2, CH0DATINR, CH0DLYR, CH0WDATR, CH1AWSCDR, CH1CFGR1, CH1CFGR2,
    CH1DATINR, CH1DLYR, CH1WDATR, CH2AWSCDR, CH2CFGR1, CH2CFGR2, CH2DATINR, CH2DLYR, CH2WDATR,
    CH3AWSCDR, CH3CFGR1, CH3CFGR2, CH3DATINR, CH3DLYR, CH3WDATR, CH4AWSCDR, CH4CFGR1, CH4CFGR2,
    CH4DATINR, CH4DLYR, CH4WDATR, CH5AWSCDR, CH5CFGR1, CH5CFGR2, CH5DATINR, CH5DLYR, CH5WDATR,
    CH6AWSCDR, CH6CFGR1, CH6CFGR2, CH6DATINR, CH6DLYR, CH6WDATR, CH7AWSCDR, CH7CFGR1, CH7CFGR2,
    CH7DATINR, CH7DLYR, CH7WDATR, FLT0AWCFR, FLT0AWHTR, FLT0AWLTR, FLT0AWSR, FLT0CNVTIMR, FLT0CR1,
    FLT0CR2, FLT0EXMAX, FLT0EXMIN, FLT0FCR, FLT0ICR, FLT0ISR, FLT0JCHGR, FLT0JDATAR, FLT0RDATAR,
    FLT1AW, FLT1AWLTR, FLT1AWSR, FLT1CNVTIMR, FLT1CR1, FLT1CR2, FLT1EXMAX, FLT1EXMIN, FLT1FCR,
    FLT1ICR, FLT1ISR, FLT1JCHGR, FLT1JDATAR, FLT1RDATAR, FLT2AWCFR, FLT2AWHTR, FLT2AWLTR, FLT2AWSR,
    FLT2CNVTIMR, FLT2CR1, FLT2CR2, FLT2EXMAX, FLT2EXMIN, FLT2FCR, FLT2ICR, FLT2ISR, FLT2JCHGR,
    FLT2JDATAR, FLT2RDATAR, FLT3AWCFR, FLT3AWHTR, FLT3AWLTR, FLT3AWSR, FLT3CNVTIMR, FLT3CR1,
    FLT3CR2, FLT3EXMAX, FLT3EXMIN, FLT3FCR, FLT3ICR, FLT3ISR, FLT3JCHGR, FLT3JDATAR, FLT3RDATAR,
};

/// Access functions for the DFSDM1 peripheral instance
pub mod DFSDM1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40016000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DFSDM1
    pub const reset: ResetValues = ResetValues {
        CH0CFGR1: 0x00000000,
        CH0CFGR2: 0x00000000,
        CH0AWSCDR: 0x00000000,
        CH0WDATR: 0x00000000,
        CH0DATINR: 0x00000000,
        CH1CFGR1: 0x00000000,
        CH1CFGR2: 0x00000000,
        CH1AWSCDR: 0x00000000,
        CH1WDATR: 0x00000000,
        CH1DATINR: 0x00000000,
        CH2CFGR1: 0x00000000,
        CH2CFGR2: 0x00000000,
        CH2AWSCDR: 0x00000000,
        CH2WDATR: 0x00000000,
        CH2DATINR: 0x00000000,
        CH3CFGR1: 0x00000000,
        CH3CFGR2: 0x00000000,
        CH3AWSCDR: 0x00000000,
        CH3WDATR: 0x00000000,
        CH3DATINR: 0x00000000,
        CH4CFGR1: 0x00000000,
        CH4CFGR2: 0x00000000,
        CH4AWSCDR: 0x00000000,
        CH4WDATR: 0x00000000,
        CH4DATINR: 0x00000000,
        CH5CFGR1: 0x00000000,
        CH5CFGR2: 0x00000000,
        CH5AWSCDR: 0x00000000,
        CH5WDATR: 0x00000000,
        CH5DATINR: 0x00000000,
        CH6CFGR1: 0x00000000,
        CH6CFGR2: 0x00000000,
        CH6AWSCDR: 0x00000000,
        CH6WDATR: 0x00000000,
        CH6DATINR: 0x00000000,
        CH7CFGR1: 0x00000000,
        CH7CFGR2: 0x00000000,
        CH7AWSCDR: 0x00000000,
        CH7WDATR: 0x00000000,
        CH7DATINR: 0x00000000,
        FLT0CR1: 0x00000000,
        FLT0CR2: 0x00000000,
        FLT0ISR: 0x00FF0000,
        FLT0ICR: 0x00000000,
        FLT0JCHGR: 0x00000001,
        FLT0FCR: 0x00000000,
        FLT0JDATAR: 0x00000000,
        FLT0RDATAR: 0x00000000,
        FLT0AWHTR: 0x00000000,
        FLT0AWLTR: 0x00000000,
        FLT0AWSR: 0x00000000,
        FLT0AWCFR: 0x00000000,
        FLT0EXMAX: 0x80000000,
        FLT0EXMIN: 0x7FFFFF00,
        FLT0CNVTIMR: 0x00000000,
        FLT1CR1: 0x00000000,
        FLT1CR2: 0x00000000,
        FLT1ISR: 0x00FF0000,
        FLT1ICR: 0x00000000,
        FLT1JCHGR: 0x00000001,
        FLT1FCR: 0x00000000,
        FLT1JDATAR: 0x00000000,
        FLT1RDATAR: 0x00000000,
        FLT1AW: 0x00000000,
        FLT1AWLTR: 0x00000000,
        FLT1AWSR: 0x00000000,
        FLT1EXMAX: 0x80000000,
        FLT1EXMIN: 0x7FFFFF00,
        FLT1CNVTIMR: 0x00000000,
        FLT2CR1: 0x00000000,
        FLT2CR2: 0x00000000,
        FLT2ISR: 0x00FF0000,
        FLT2ICR: 0x00000000,
        FLT2JCHGR: 0x00000001,
        FLT2FCR: 0x00000000,
        FLT2JDATAR: 0x00000000,
        FLT2RDATAR: 0x00000000,
        FLT2AWHTR: 0x00000000,
        FLT2AWLTR: 0x00000000,
        FLT2AWSR: 0x00000000,
        FLT2AWCFR: 0x00000000,
        FLT2EXMAX: 0x80000000,
        FLT2EXMIN: 0x7FFFFF00,
        FLT2CNVTIMR: 0x00000000,
        FLT3CR1: 0x00000000,
        FLT3CR2: 0x00000000,
        FLT3ISR: 0x00FF0000,
        FLT3ICR: 0x00000000,
        FLT3JCHGR: 0x00000001,
        FLT3FCR: 0x00000000,
        FLT3JDATAR: 0x00000000,
        FLT3RDATAR: 0x00000000,
        FLT3AWHTR: 0x00000000,
        FLT3AWLTR: 0x00000000,
        FLT3AWSR: 0x00000000,
        FLT3AWCFR: 0x00000000,
        FLT3EXMAX: 0x80000000,
        FLT3EXMIN: 0x7FFFFF00,
        FLT3CNVTIMR: 0x00000000,
        CH0DLYR: 0x00000000,
        CH1DLYR: 0x00000000,
        CH2DLYR: 0x00000000,
        CH3DLYR: 0x00000000,
        CH4DLYR: 0x00000000,
        CH5DLYR: 0x00000000,
        CH6DLYR: 0x00000000,
        CH7DLYR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut DFSDM1_TAKEN: bool = false;

    /// Safe access to DFSDM1
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
            if DFSDM1_TAKEN {
                None
            } else {
                DFSDM1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to DFSDM1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if DFSDM1_TAKEN && inst.addr == INSTANCE.addr {
                DFSDM1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal DFSDM1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        DFSDM1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to DFSDM1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DFSDM1: *const RegisterBlock = 0x40016000 as *const _;

/// Access functions for the SEC_DFSDM1 peripheral instance
pub mod SEC_DFSDM1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50016000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SEC_DFSDM1
    pub const reset: ResetValues = ResetValues {
        CH0CFGR1: 0x00000000,
        CH0CFGR2: 0x00000000,
        CH0AWSCDR: 0x00000000,
        CH0WDATR: 0x00000000,
        CH0DATINR: 0x00000000,
        CH1CFGR1: 0x00000000,
        CH1CFGR2: 0x00000000,
        CH1AWSCDR: 0x00000000,
        CH1WDATR: 0x00000000,
        CH1DATINR: 0x00000000,
        CH2CFGR1: 0x00000000,
        CH2CFGR2: 0x00000000,
        CH2AWSCDR: 0x00000000,
        CH2WDATR: 0x00000000,
        CH2DATINR: 0x00000000,
        CH3CFGR1: 0x00000000,
        CH3CFGR2: 0x00000000,
        CH3AWSCDR: 0x00000000,
        CH3WDATR: 0x00000000,
        CH3DATINR: 0x00000000,
        CH4CFGR1: 0x00000000,
        CH4CFGR2: 0x00000000,
        CH4AWSCDR: 0x00000000,
        CH4WDATR: 0x00000000,
        CH4DATINR: 0x00000000,
        CH5CFGR1: 0x00000000,
        CH5CFGR2: 0x00000000,
        CH5AWSCDR: 0x00000000,
        CH5WDATR: 0x00000000,
        CH5DATINR: 0x00000000,
        CH6CFGR1: 0x00000000,
        CH6CFGR2: 0x00000000,
        CH6AWSCDR: 0x00000000,
        CH6WDATR: 0x00000000,
        CH6DATINR: 0x00000000,
        CH7CFGR1: 0x00000000,
        CH7CFGR2: 0x00000000,
        CH7AWSCDR: 0x00000000,
        CH7WDATR: 0x00000000,
        CH7DATINR: 0x00000000,
        FLT0CR1: 0x00000000,
        FLT0CR2: 0x00000000,
        FLT0ISR: 0x00FF0000,
        FLT0ICR: 0x00000000,
        FLT0JCHGR: 0x00000001,
        FLT0FCR: 0x00000000,
        FLT0JDATAR: 0x00000000,
        FLT0RDATAR: 0x00000000,
        FLT0AWHTR: 0x00000000,
        FLT0AWLTR: 0x00000000,
        FLT0AWSR: 0x00000000,
        FLT0AWCFR: 0x00000000,
        FLT0EXMAX: 0x80000000,
        FLT0EXMIN: 0x7FFFFF00,
        FLT0CNVTIMR: 0x00000000,
        FLT1CR1: 0x00000000,
        FLT1CR2: 0x00000000,
        FLT1ISR: 0x00FF0000,
        FLT1ICR: 0x00000000,
        FLT1JCHGR: 0x00000001,
        FLT1FCR: 0x00000000,
        FLT1JDATAR: 0x00000000,
        FLT1RDATAR: 0x00000000,
        FLT1AW: 0x00000000,
        FLT1AWLTR: 0x00000000,
        FLT1AWSR: 0x00000000,
        FLT1EXMAX: 0x80000000,
        FLT1EXMIN: 0x7FFFFF00,
        FLT1CNVTIMR: 0x00000000,
        FLT2CR1: 0x00000000,
        FLT2CR2: 0x00000000,
        FLT2ISR: 0x00FF0000,
        FLT2ICR: 0x00000000,
        FLT2JCHGR: 0x00000001,
        FLT2FCR: 0x00000000,
        FLT2JDATAR: 0x00000000,
        FLT2RDATAR: 0x00000000,
        FLT2AWHTR: 0x00000000,
        FLT2AWLTR: 0x00000000,
        FLT2AWSR: 0x00000000,
        FLT2AWCFR: 0x00000000,
        FLT2EXMAX: 0x80000000,
        FLT2EXMIN: 0x7FFFFF00,
        FLT2CNVTIMR: 0x00000000,
        FLT3CR1: 0x00000000,
        FLT3CR2: 0x00000000,
        FLT3ISR: 0x00FF0000,
        FLT3ICR: 0x00000000,
        FLT3JCHGR: 0x00000001,
        FLT3FCR: 0x00000000,
        FLT3JDATAR: 0x00000000,
        FLT3RDATAR: 0x00000000,
        FLT3AWHTR: 0x00000000,
        FLT3AWLTR: 0x00000000,
        FLT3AWSR: 0x00000000,
        FLT3AWCFR: 0x00000000,
        FLT3EXMAX: 0x80000000,
        FLT3EXMIN: 0x7FFFFF00,
        FLT3CNVTIMR: 0x00000000,
        CH0DLYR: 0x00000000,
        CH1DLYR: 0x00000000,
        CH2DLYR: 0x00000000,
        CH3DLYR: 0x00000000,
        CH4DLYR: 0x00000000,
        CH5DLYR: 0x00000000,
        CH6DLYR: 0x00000000,
        CH7DLYR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SEC_DFSDM1_TAKEN: bool = false;

    /// Safe access to SEC_DFSDM1
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
            if SEC_DFSDM1_TAKEN {
                None
            } else {
                SEC_DFSDM1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SEC_DFSDM1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SEC_DFSDM1_TAKEN && inst.addr == INSTANCE.addr {
                SEC_DFSDM1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SEC_DFSDM1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SEC_DFSDM1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SEC_DFSDM1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SEC_DFSDM1: *const RegisterBlock = 0x50016000 as *const _;
