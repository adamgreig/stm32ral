#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DFSDM1
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::dfsdm1::Instance;
pub use crate::stm32mp::peripherals::dfsdm1::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::dfsdm1::{
    DFSDM_CH0AWSCDR, DFSDM_CH0CFGR1, DFSDM_CH0CFGR2, DFSDM_CH0DATINR, DFSDM_CH0DLYR,
    DFSDM_CH0WDATR, DFSDM_CH1AWSCDR, DFSDM_CH1CFGR1, DFSDM_CH1CFGR2, DFSDM_CH1DATINR,
    DFSDM_CH1DLYR, DFSDM_CH1WDATR, DFSDM_CH2AWSCDR, DFSDM_CH2CFGR1, DFSDM_CH2CFGR2,
    DFSDM_CH2DATINR, DFSDM_CH2DLYR, DFSDM_CH2WDATR, DFSDM_CH3AWSCDR, DFSDM_CH3CFGR1,
    DFSDM_CH3CFGR2, DFSDM_CH3DATINR, DFSDM_CH3DLYR, DFSDM_CH3WDATR, DFSDM_CH4AWSCDR,
    DFSDM_CH4CFGR1, DFSDM_CH4CFGR2, DFSDM_CH4DATINR, DFSDM_CH4DLYR, DFSDM_CH4WDATR,
    DFSDM_CH5AWSCDR, DFSDM_CH5CFGR1, DFSDM_CH5CFGR2, DFSDM_CH5DATINR, DFSDM_CH5DLYR,
    DFSDM_CH5WDATR, DFSDM_CH6AWSCDR, DFSDM_CH6CFGR1, DFSDM_CH6CFGR2, DFSDM_CH6DATINR,
    DFSDM_CH6DLYR, DFSDM_CH6WDATR, DFSDM_CH7AWSCDR, DFSDM_CH7CFGR1, DFSDM_CH7CFGR2,
    DFSDM_CH7DATINR, DFSDM_CH7DLYR, DFSDM_CH7WDATR, DFSDM_FLT0AWCFR, DFSDM_FLT0AWHTR,
    DFSDM_FLT0AWLTR, DFSDM_FLT0AWSR, DFSDM_FLT0CNVTIMR, DFSDM_FLT0CR1, DFSDM_FLT0CR2,
    DFSDM_FLT0EXMAX, DFSDM_FLT0EXMIN, DFSDM_FLT0FCR, DFSDM_FLT0ICR, DFSDM_FLT0ISR, DFSDM_FLT0JCHGR,
    DFSDM_FLT0JDATAR, DFSDM_FLT0RDATAR, DFSDM_FLT1AWCFR, DFSDM_FLT1AWHTR, DFSDM_FLT1AWLTR,
    DFSDM_FLT1AWSR, DFSDM_FLT1CNVTIMR, DFSDM_FLT1CR1, DFSDM_FLT1CR2, DFSDM_FLT1EXMAX,
    DFSDM_FLT1EXMIN, DFSDM_FLT1FCR, DFSDM_FLT1ICR, DFSDM_FLT1ISR, DFSDM_FLT1JCHGR,
    DFSDM_FLT1JDATAR, DFSDM_FLT1RDATAR, DFSDM_FLT2AWCFR, DFSDM_FLT2AWHTR, DFSDM_FLT2AWLTR,
    DFSDM_FLT2AWSR, DFSDM_FLT2CNVTIMR, DFSDM_FLT2CR1, DFSDM_FLT2CR2, DFSDM_FLT2EXMAX,
    DFSDM_FLT2EXMIN, DFSDM_FLT2FCR, DFSDM_FLT2ICR, DFSDM_FLT2ISR, DFSDM_FLT2JCHGR,
    DFSDM_FLT2JDATAR, DFSDM_FLT2RDATAR, DFSDM_FLT3AWCFR, DFSDM_FLT3AWHTR, DFSDM_FLT3AWLTR,
    DFSDM_FLT3AWSR, DFSDM_FLT3CNVTIMR, DFSDM_FLT3CR1, DFSDM_FLT3CR2, DFSDM_FLT3EXMAX,
    DFSDM_FLT3EXMIN, DFSDM_FLT3FCR, DFSDM_FLT3ICR, DFSDM_FLT3ISR, DFSDM_FLT3JCHGR,
    DFSDM_FLT3JDATAR, DFSDM_FLT3RDATAR, DFSDM_FLT4AWCFR, DFSDM_FLT4AWHTR, DFSDM_FLT4AWLTR,
    DFSDM_FLT4AWSR, DFSDM_FLT4CNVTIMR, DFSDM_FLT4CR1, DFSDM_FLT4CR2, DFSDM_FLT4EXMAX,
    DFSDM_FLT4EXMIN, DFSDM_FLT4FCR, DFSDM_FLT4ICR, DFSDM_FLT4ISR, DFSDM_FLT4JCHGR,
    DFSDM_FLT4JDATAR, DFSDM_FLT4RDATAR, DFSDM_FLT5AWCFR, DFSDM_FLT5AWHTR, DFSDM_FLT5AWLTR,
    DFSDM_FLT5AWSR, DFSDM_FLT5CNVTIMR, DFSDM_FLT5CR1, DFSDM_FLT5CR2, DFSDM_FLT5EXMAX,
    DFSDM_FLT5EXMIN, DFSDM_FLT5FCR, DFSDM_FLT5ICR, DFSDM_FLT5ISR, DFSDM_FLT5JCHGR,
    DFSDM_FLT5JDATAR, DFSDM_FLT5RDATAR, DFSDM_HWCFGR, DFSDM_IPIDR, DFSDM_SIDR, DFSDM_VERR,
};

/// Access functions for the DFSDM1 peripheral instance
pub mod DFSDM1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x4400d000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DFSDM1
    pub const reset: ResetValues = ResetValues {
        DFSDM_CH0CFGR1: 0x00000000,
        DFSDM_CH0CFGR2: 0x00000000,
        DFSDM_CH0AWSCDR: 0x00000000,
        DFSDM_CH0WDATR: 0x00000000,
        DFSDM_CH0DATINR: 0x00000000,
        DFSDM_CH0DLYR: 0x00000000,
        DFSDM_CH1CFGR1: 0x00000000,
        DFSDM_CH1CFGR2: 0x00000000,
        DFSDM_CH1AWSCDR: 0x00000000,
        DFSDM_CH1WDATR: 0x00000000,
        DFSDM_CH1DATINR: 0x00000000,
        DFSDM_CH1DLYR: 0x00000000,
        DFSDM_CH2CFGR1: 0x00000000,
        DFSDM_CH2CFGR2: 0x00000000,
        DFSDM_CH2AWSCDR: 0x00000000,
        DFSDM_CH2WDATR: 0x00000000,
        DFSDM_CH2DATINR: 0x00000000,
        DFSDM_CH2DLYR: 0x00000000,
        DFSDM_CH3CFGR1: 0x00000000,
        DFSDM_CH3CFGR2: 0x00000000,
        DFSDM_CH3AWSCDR: 0x00000000,
        DFSDM_CH3WDATR: 0x00000000,
        DFSDM_CH3DATINR: 0x00000000,
        DFSDM_CH3DLYR: 0x00000000,
        DFSDM_CH4CFGR1: 0x00000000,
        DFSDM_CH4CFGR2: 0x00000000,
        DFSDM_CH4AWSCDR: 0x00000000,
        DFSDM_CH4WDATR: 0x00000000,
        DFSDM_CH4DATINR: 0x00000000,
        DFSDM_CH4DLYR: 0x00000000,
        DFSDM_CH5CFGR1: 0x00000000,
        DFSDM_CH5CFGR2: 0x00000000,
        DFSDM_CH5AWSCDR: 0x00000000,
        DFSDM_CH5WDATR: 0x00000000,
        DFSDM_CH5DATINR: 0x00000000,
        DFSDM_CH5DLYR: 0x00000000,
        DFSDM_CH6CFGR1: 0x00000000,
        DFSDM_CH6CFGR2: 0x00000000,
        DFSDM_CH6AWSCDR: 0x00000000,
        DFSDM_CH6WDATR: 0x00000000,
        DFSDM_CH6DATINR: 0x00000000,
        DFSDM_CH6DLYR: 0x00000000,
        DFSDM_CH7CFGR1: 0x00000000,
        DFSDM_CH7CFGR2: 0x00000000,
        DFSDM_CH7AWSCDR: 0x00000000,
        DFSDM_CH7WDATR: 0x00000000,
        DFSDM_CH7DATINR: 0x00000000,
        DFSDM_CH7DLYR: 0x00000000,
        DFSDM_FLT0CR1: 0x00000000,
        DFSDM_FLT0CR2: 0x00000000,
        DFSDM_FLT0ISR: 0x00FF0000,
        DFSDM_FLT0ICR: 0x00000000,
        DFSDM_FLT0JCHGR: 0x00000001,
        DFSDM_FLT0FCR: 0x00000000,
        DFSDM_FLT0JDATAR: 0x00000000,
        DFSDM_FLT0RDATAR: 0x00000000,
        DFSDM_FLT0AWHTR: 0x00000000,
        DFSDM_FLT0AWLTR: 0x00000000,
        DFSDM_FLT0AWSR: 0x00000000,
        DFSDM_FLT0AWCFR: 0x00000000,
        DFSDM_FLT0EXMAX: 0x80000000,
        DFSDM_FLT0EXMIN: 0x7FFFFF00,
        DFSDM_FLT0CNVTIMR: 0x00000000,
        DFSDM_FLT1CR1: 0x00000000,
        DFSDM_FLT1CR2: 0x00000000,
        DFSDM_FLT1ISR: 0x00FF0000,
        DFSDM_FLT1ICR: 0x00000000,
        DFSDM_FLT1JCHGR: 0x00000001,
        DFSDM_FLT1FCR: 0x00000000,
        DFSDM_FLT1JDATAR: 0x00000000,
        DFSDM_FLT1RDATAR: 0x00000000,
        DFSDM_FLT1AWHTR: 0x00000000,
        DFSDM_FLT1AWLTR: 0x00000000,
        DFSDM_FLT1AWSR: 0x00000000,
        DFSDM_FLT1AWCFR: 0x00000000,
        DFSDM_FLT1EXMAX: 0x80000000,
        DFSDM_FLT1EXMIN: 0x7FFFFF00,
        DFSDM_FLT1CNVTIMR: 0x00000000,
        DFSDM_FLT2CR1: 0x00000000,
        DFSDM_FLT2CR2: 0x00000000,
        DFSDM_FLT2ISR: 0x00FF0000,
        DFSDM_FLT2ICR: 0x00000000,
        DFSDM_FLT2JCHGR: 0x00000001,
        DFSDM_FLT2FCR: 0x00000000,
        DFSDM_FLT2JDATAR: 0x00000000,
        DFSDM_FLT2RDATAR: 0x00000000,
        DFSDM_FLT2AWHTR: 0x00000000,
        DFSDM_FLT2AWLTR: 0x00000000,
        DFSDM_FLT2AWSR: 0x00000000,
        DFSDM_FLT2AWCFR: 0x00000000,
        DFSDM_FLT2EXMAX: 0x80000000,
        DFSDM_FLT2EXMIN: 0x7FFFFF00,
        DFSDM_FLT2CNVTIMR: 0x00000000,
        DFSDM_FLT3CR1: 0x00000000,
        DFSDM_FLT3CR2: 0x00000000,
        DFSDM_FLT3ISR: 0x00FF0000,
        DFSDM_FLT3ICR: 0x00000000,
        DFSDM_FLT3JCHGR: 0x00000001,
        DFSDM_FLT3FCR: 0x00000000,
        DFSDM_FLT3JDATAR: 0x00000000,
        DFSDM_FLT3RDATAR: 0x00000000,
        DFSDM_FLT3AWHTR: 0x00000000,
        DFSDM_FLT3AWLTR: 0x00000000,
        DFSDM_FLT3AWSR: 0x00000000,
        DFSDM_FLT3AWCFR: 0x00000000,
        DFSDM_FLT3EXMAX: 0x80000000,
        DFSDM_FLT3EXMIN: 0x7FFFFF00,
        DFSDM_FLT3CNVTIMR: 0x00000000,
        DFSDM_FLT4CR1: 0x00000000,
        DFSDM_FLT4CR2: 0x00000000,
        DFSDM_FLT4ISR: 0x00FF0000,
        DFSDM_FLT4ICR: 0x00000000,
        DFSDM_FLT4JCHGR: 0x00000001,
        DFSDM_FLT4FCR: 0x00000000,
        DFSDM_FLT4JDATAR: 0x00000000,
        DFSDM_FLT4RDATAR: 0x00000000,
        DFSDM_FLT4AWHTR: 0x00000000,
        DFSDM_FLT4AWLTR: 0x00000000,
        DFSDM_FLT4AWSR: 0x00000000,
        DFSDM_FLT4AWCFR: 0x00000000,
        DFSDM_FLT4EXMAX: 0x80000000,
        DFSDM_FLT4EXMIN: 0x7FFFFF00,
        DFSDM_FLT4CNVTIMR: 0x00000000,
        DFSDM_FLT5CR1: 0x00000000,
        DFSDM_FLT5CR2: 0x00000000,
        DFSDM_FLT5ISR: 0x00FF0000,
        DFSDM_FLT5ICR: 0x00000000,
        DFSDM_FLT5JCHGR: 0x00000001,
        DFSDM_FLT5FCR: 0x00000000,
        DFSDM_FLT5JDATAR: 0x00000000,
        DFSDM_FLT5RDATAR: 0x00000000,
        DFSDM_FLT5AWHTR: 0x00000000,
        DFSDM_FLT5AWLTR: 0x00000000,
        DFSDM_FLT5AWSR: 0x00000000,
        DFSDM_FLT5AWCFR: 0x00000000,
        DFSDM_FLT5EXMAX: 0x80000000,
        DFSDM_FLT5EXMIN: 0x7FFFFF00,
        DFSDM_FLT5CNVTIMR: 0x00000000,
        DFSDM_HWCFGR: 0x00000608,
        DFSDM_VERR: 0x00000021,
        DFSDM_IPIDR: 0x00110031,
        DFSDM_SIDR: 0xA3C5DD02,
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
pub const DFSDM1: *const RegisterBlock = 0x4400d000 as *const _;
