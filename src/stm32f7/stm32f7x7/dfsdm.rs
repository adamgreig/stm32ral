#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Digital filter for sigma delta modulators

#[cfg(not(feature = "nosync"))]
pub use crate::stm32f7::peripherals::dfsdm1::Instance;
pub use crate::stm32f7::peripherals::dfsdm1::{RegisterBlock, ResetValues};
pub use crate::stm32f7::peripherals::dfsdm1::{
    DFSDM0_AWCFR, DFSDM0_AWHTR, DFSDM0_AWLTR, DFSDM0_AWSR, DFSDM0_CNVTIMR, DFSDM0_CR1, DFSDM0_CR2,
    DFSDM0_EXMAX, DFSDM0_EXMIN, DFSDM0_FCR, DFSDM0_ICR, DFSDM0_ISR, DFSDM0_JCHGR, DFSDM0_JDATAR,
    DFSDM0_RDATAR, DFSDM1_AWCFR, DFSDM1_AWHTR, DFSDM1_AWLTR, DFSDM1_AWSR, DFSDM1_CNVTIMR,
    DFSDM1_CR1, DFSDM1_CR2, DFSDM1_DATAR, DFSDM1_EXMAX, DFSDM1_EXMIN, DFSDM1_FCR, DFSDM1_ICR,
    DFSDM1_ISR, DFSDM1_JCHGR, DFSDM2_AWCFR, DFSDM2_AWHTR, DFSDM2_AWLTR, DFSDM2_AWSR,
    DFSDM2_CNVTIMR, DFSDM2_CR1, DFSDM2_CR2, DFSDM2_DATAR, DFSDM2_EXMAX, DFSDM2_EXMIN, DFSDM2_FCR,
    DFSDM2_ICR, DFSDM2_ISR, DFSDM2_JCHGR, DFSDM3_AWCFR, DFSDM3_AWHTR, DFSDM3_AWLTR, DFSDM3_AWSR,
    DFSDM3_CNVTIMR, DFSDM3_CR1, DFSDM3_CR2, DFSDM3_DATAR, DFSDM3_EXMAX, DFSDM3_EXMIN, DFSDM3_FCR,
    DFSDM3_ICR, DFSDM3_ISR, DFSDM3_JCHGR, DFSDM_AWSCD0R, DFSDM_AWSCD1R, DFSDM_AWSCD2R,
    DFSDM_AWSCD3R, DFSDM_AWSCD4R, DFSDM_AWSCD5R, DFSDM_AWSCD6R, DFSDM_AWSCD7R, DFSDM_CHCFG0R1,
    DFSDM_CHCFG0R2, DFSDM_CHCFG1R1, DFSDM_CHCFG1R2, DFSDM_CHCFG2R1, DFSDM_CHCFG2R2, DFSDM_CHCFG3R1,
    DFSDM_CHCFG3R2, DFSDM_CHCFG4R1, DFSDM_CHCFG4R2, DFSDM_CHCFG5R1, DFSDM_CHCFG5R2, DFSDM_CHCFG6R1,
    DFSDM_CHCFG6R2, DFSDM_CHCFG7R1, DFSDM_CHCFG7R2, DFSDM_CHDATIN0R, DFSDM_CHDATIN1R,
    DFSDM_CHDATIN2R, DFSDM_CHDATIN3R, DFSDM_CHDATIN4R, DFSDM_CHDATIN5R, DFSDM_CHDATIN6R,
    DFSDM_CHDATIN7R, DFSDM_CHWDAT0R, DFSDM_CHWDAT1R, DFSDM_CHWDAT2R, DFSDM_CHWDAT3R,
    DFSDM_CHWDAT4R, DFSDM_CHWDAT5R, DFSDM_CHWDAT6R, DFSDM_CHWDAT7R,
};

/// Access functions for the DFSDM peripheral instance
pub mod DFSDM {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40017400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DFSDM
    pub const reset: ResetValues = ResetValues {
        DFSDM_CHCFG0R1: 0x00000000,
        DFSDM_CHCFG1R1: 0x00000000,
        DFSDM_CHCFG2R1: 0x00000000,
        DFSDM_CHCFG3R1: 0x00000000,
        DFSDM_CHCFG4R1: 0x00000000,
        DFSDM_CHCFG5R1: 0x00000000,
        DFSDM_CHCFG6R1: 0x00000000,
        DFSDM_CHCFG7R1: 0x00000000,
        DFSDM_CHCFG0R2: 0x00000000,
        DFSDM_CHCFG1R2: 0x00000000,
        DFSDM_CHCFG2R2: 0x00000000,
        DFSDM_CHCFG3R2: 0x00000000,
        DFSDM_CHCFG4R2: 0x00000000,
        DFSDM_CHCFG5R2: 0x00000000,
        DFSDM_CHCFG6R2: 0x00000000,
        DFSDM_CHCFG7R2: 0x00000000,
        DFSDM_AWSCD0R: 0x00000000,
        DFSDM_AWSCD1R: 0x00000000,
        DFSDM_AWSCD2R: 0x00000000,
        DFSDM_AWSCD3R: 0x00000000,
        DFSDM_AWSCD4R: 0x00000000,
        DFSDM_AWSCD5R: 0x00000000,
        DFSDM_AWSCD6R: 0x00000000,
        DFSDM_AWSCD7R: 0x00000000,
        DFSDM_CHWDAT0R: 0x00000000,
        DFSDM_CHWDAT1R: 0x00000000,
        DFSDM_CHWDAT2R: 0x00000000,
        DFSDM_CHWDAT3R: 0x00000000,
        DFSDM_CHWDAT4R: 0x00000000,
        DFSDM_CHWDAT5R: 0x00000000,
        DFSDM_CHWDAT6R: 0x00000000,
        DFSDM_CHWDAT7R: 0x00000000,
        DFSDM_CHDATIN0R: 0x00000000,
        DFSDM_CHDATIN1R: 0x00000000,
        DFSDM_CHDATIN2R: 0x00000000,
        DFSDM_CHDATIN3R: 0x00000000,
        DFSDM_CHDATIN4R: 0x00000000,
        DFSDM_CHDATIN5R: 0x00000000,
        DFSDM_CHDATIN6R: 0x00000000,
        DFSDM_CHDATIN7R: 0x00000000,
        DFSDM0_CR1: 0x00000000,
        DFSDM1_CR1: 0x00000000,
        DFSDM2_CR1: 0x00000000,
        DFSDM3_CR1: 0x00000000,
        DFSDM0_CR2: 0x00000000,
        DFSDM1_CR2: 0x00000000,
        DFSDM2_CR2: 0x00000000,
        DFSDM3_CR2: 0x00000000,
        DFSDM0_ISR: 0x00FF0000,
        DFSDM1_ISR: 0x00FF0000,
        DFSDM2_ISR: 0x00FF0000,
        DFSDM3_ISR: 0x00FF0000,
        DFSDM0_ICR: 0x00000000,
        DFSDM1_ICR: 0x00000000,
        DFSDM2_ICR: 0x00000000,
        DFSDM3_ICR: 0x00000000,
        DFSDM0_JCHGR: 0x00000001,
        DFSDM1_JCHGR: 0x00000001,
        DFSDM2_JCHGR: 0x00000001,
        DFSDM3_JCHGR: 0x00000001,
        DFSDM0_FCR: 0x00000000,
        DFSDM1_FCR: 0x00000000,
        DFSDM2_FCR: 0x00000000,
        DFSDM3_FCR: 0x00000000,
        DFSDM0_JDATAR: 0x00000000,
        DFSDM1_DATAR: 0x00000000,
        DFSDM2_DATAR: 0x00000000,
        DFSDM3_DATAR: 0x00000000,
        DFSDM0_RDATAR: 0x00000000,
        DFSDM0_AWHTR: 0x00000000,
        DFSDM1_AWHTR: 0x00000000,
        DFSDM2_AWHTR: 0x00000000,
        DFSDM3_AWHTR: 0x00000000,
        DFSDM0_AWLTR: 0x00000000,
        DFSDM1_AWLTR: 0x00000000,
        DFSDM2_AWLTR: 0x00000000,
        DFSDM3_AWLTR: 0x00000000,
        DFSDM0_AWSR: 0x00000000,
        DFSDM1_AWSR: 0x00000000,
        DFSDM2_AWSR: 0x00000000,
        DFSDM3_AWSR: 0x00000000,
        DFSDM0_AWCFR: 0x00000000,
        DFSDM1_AWCFR: 0x00000000,
        DFSDM2_AWCFR: 0x00000000,
        DFSDM3_AWCFR: 0x00000000,
        DFSDM0_EXMAX: 0x00000000,
        DFSDM1_EXMAX: 0x00000000,
        DFSDM2_EXMAX: 0x00000000,
        DFSDM3_EXMAX: 0x00000000,
        DFSDM0_EXMIN: 0x7FFFFF00,
        DFSDM1_EXMIN: 0x7FFFFF00,
        DFSDM2_EXMIN: 0x7FFFFF00,
        DFSDM3_EXMIN: 0x7FFFFF00,
        DFSDM0_CNVTIMR: 0x00000000,
        DFSDM1_CNVTIMR: 0x00000000,
        DFSDM2_CNVTIMR: 0x00000000,
        DFSDM3_CNVTIMR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut DFSDM_TAKEN: bool = false;

    /// Safe access to DFSDM
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
            if DFSDM_TAKEN {
                None
            } else {
                DFSDM_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to DFSDM
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if DFSDM_TAKEN && inst.addr == INSTANCE.addr {
                DFSDM_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal DFSDM
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        DFSDM_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to DFSDM
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DFSDM: *const RegisterBlock = 0x40017400 as *const _;
