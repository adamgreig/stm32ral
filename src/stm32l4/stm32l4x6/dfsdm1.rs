#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Digital filter for sigma delta modulators

#[cfg(not(feature = "nosync"))]
pub use stm32l4::peripherals::dfsdm::Instance;
pub use stm32l4::peripherals::dfsdm::{RegisterBlock, ResetValues};
pub use stm32l4::peripherals::dfsdm::{
    AWSCD0R, AWSCD1R, AWSCD2R, AWSCD3R, AWSCD4R, AWSCD5R, AWSCD6R, AWSCD7R, CHCFG0R1, CHCFG0R2,
    CHCFG1R1, CHCFG1R2, CHCFG2R1, CHCFG2R2, CHCFG3R1, CHCFG3R2, CHCFG4R1, CHCFG4R2, CHCFG5R1,
    CHCFG5R2, CHCFG6R1, CHCFG6R2, CHCFG7R1, CHCFG7R2, CHDATIN0R, CHDATIN1R, CHDATIN2R, CHDATIN3R,
    CHDATIN4R, CHDATIN5R, CHDATIN6R, CHDATIN7R, CHWDAT0R, CHWDAT1R, CHWDAT2R, CHWDAT3R, CHWDAT4R,
    CHWDAT5R, CHWDAT6R, CHWDAT7R, DFSDM0_AWCFR, DFSDM0_AWHTR, DFSDM0_AWLTR, DFSDM0_AWSR,
    DFSDM0_CNVTIMR, DFSDM0_CR1, DFSDM0_CR2, DFSDM0_EXMAX, DFSDM0_EXMIN, DFSDM0_FCR, DFSDM0_ICR,
    DFSDM0_ISR, DFSDM0_JCHGR, DFSDM0_JDATAR, DFSDM0_RDATAR, DFSDM1_AWCFR, DFSDM1_AWHTR,
    DFSDM1_AWLTR, DFSDM1_AWSR, DFSDM1_CNVTIMR, DFSDM1_CR1, DFSDM1_CR2, DFSDM1_EXMAX, DFSDM1_EXMIN,
    DFSDM1_FCR, DFSDM1_ICR, DFSDM1_ISR, DFSDM1_JCHGR, DFSDM1_JDATAR, DFSDM1_RDATAR, DFSDM2_AWCFR,
    DFSDM2_AWHTR, DFSDM2_AWLTR, DFSDM2_AWSR, DFSDM2_CNVTIMR, DFSDM2_CR1, DFSDM2_CR2, DFSDM2_EXMAX,
    DFSDM2_EXMIN, DFSDM2_FCR, DFSDM2_ICR, DFSDM2_ISR, DFSDM2_JCHGR, DFSDM2_JDATAR, DFSDM2_RDATAR,
    DFSDM3_AWCFR, DFSDM3_AWHTR, DFSDM3_AWLTR, DFSDM3_AWSR, DFSDM3_CNVTIMR, DFSDM3_CR1, DFSDM3_CR2,
    DFSDM3_EXMAX, DFSDM3_EXMIN, DFSDM3_FCR, DFSDM3_ICR, DFSDM3_ISR, DFSDM3_JCHGR, DFSDM3_JDATAR,
    DFSDM3_RDATAR,
};

/// Access functions for the DFSDM1 peripheral instance
pub mod DFSDM1 {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

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
        CHCFG0R1: 0x00000000,
        CHCFG0R2: 0x00000000,
        AWSCD0R: 0x00000000,
        CHWDAT0R: 0x00000000,
        CHDATIN0R: 0x00000000,
        CHCFG1R1: 0x00000000,
        CHCFG1R2: 0x00000000,
        AWSCD1R: 0x00000000,
        CHWDAT1R: 0x00000000,
        CHDATIN1R: 0x00000000,
        CHCFG2R1: 0x00000000,
        CHCFG2R2: 0x00000000,
        AWSCD2R: 0x00000000,
        CHWDAT2R: 0x00000000,
        CHDATIN2R: 0x00000000,
        CHCFG3R1: 0x00000000,
        CHCFG3R2: 0x00000000,
        AWSCD3R: 0x00000000,
        CHWDAT3R: 0x00000000,
        CHDATIN3R: 0x00000000,
        CHCFG4R1: 0x00000000,
        CHCFG4R2: 0x00000000,
        AWSCD4R: 0x00000000,
        CHWDAT4R: 0x00000000,
        CHDATIN4R: 0x00000000,
        CHCFG5R1: 0x00000000,
        CHCFG5R2: 0x00000000,
        AWSCD5R: 0x00000000,
        CHWDAT5R: 0x00000000,
        CHDATIN5R: 0x00000000,
        CHCFG6R1: 0x00000000,
        CHCFG6R2: 0x00000000,
        AWSCD6R: 0x00000000,
        CHWDAT6R: 0x00000000,
        CHDATIN6R: 0x00000000,
        CHCFG7R1: 0x00000000,
        CHCFG7R2: 0x00000000,
        AWSCD7R: 0x00000000,
        CHWDAT7R: 0x00000000,
        CHDATIN7R: 0x00000000,
        DFSDM0_CR1: 0x00000000,
        DFSDM0_CR2: 0x00000000,
        DFSDM0_ISR: 0x00FF0000,
        DFSDM0_ICR: 0x00000000,
        DFSDM0_JCHGR: 0x00000001,
        DFSDM0_FCR: 0x00000000,
        DFSDM0_JDATAR: 0x00000000,
        DFSDM0_RDATAR: 0x00000000,
        DFSDM0_AWHTR: 0x00000000,
        DFSDM0_AWLTR: 0x00000000,
        DFSDM0_AWSR: 0x00000000,
        DFSDM0_AWCFR: 0x00000000,
        DFSDM0_EXMAX: 0x80000000,
        DFSDM0_EXMIN: 0x7FFFFF00,
        DFSDM0_CNVTIMR: 0x00000000,
        DFSDM1_CR1: 0x00000000,
        DFSDM1_CR2: 0x00000000,
        DFSDM1_ISR: 0x00FF0000,
        DFSDM1_ICR: 0x00000000,
        DFSDM1_JCHGR: 0x00000001,
        DFSDM1_FCR: 0x00000000,
        DFSDM1_JDATAR: 0x00000000,
        DFSDM1_RDATAR: 0x00000000,
        DFSDM1_AWHTR: 0x00000000,
        DFSDM1_AWLTR: 0x00000000,
        DFSDM1_AWSR: 0x00000000,
        DFSDM1_AWCFR: 0x00000000,
        DFSDM1_EXMAX: 0x80000000,
        DFSDM1_EXMIN: 0x7FFFFF00,
        DFSDM1_CNVTIMR: 0x00000000,
        DFSDM2_CR1: 0x00000000,
        DFSDM2_CR2: 0x00000000,
        DFSDM2_ISR: 0x00FF0000,
        DFSDM2_ICR: 0x00000000,
        DFSDM2_JCHGR: 0x00000001,
        DFSDM2_FCR: 0x00000000,
        DFSDM2_JDATAR: 0x00000000,
        DFSDM2_RDATAR: 0x00000000,
        DFSDM2_AWHTR: 0x00000000,
        DFSDM2_AWLTR: 0x00000000,
        DFSDM2_AWSR: 0x00000000,
        DFSDM2_AWCFR: 0x00000000,
        DFSDM2_EXMAX: 0x80000000,
        DFSDM2_EXMIN: 0x7FFFFF00,
        DFSDM2_CNVTIMR: 0x00000000,
        DFSDM3_CR1: 0x00000000,
        DFSDM3_CR2: 0x00000000,
        DFSDM3_ISR: 0x00FF0000,
        DFSDM3_ICR: 0x00000000,
        DFSDM3_JCHGR: 0x00000001,
        DFSDM3_FCR: 0x00000000,
        DFSDM3_JDATAR: 0x00000000,
        DFSDM3_RDATAR: 0x00000000,
        DFSDM3_AWHTR: 0x00000000,
        DFSDM3_AWLTR: 0x00000000,
        DFSDM3_AWSR: 0x00000000,
        DFSDM3_AWCFR: 0x00000000,
        DFSDM3_EXMAX: 0x80000000,
        DFSDM3_EXMIN: 0x7FFFFF00,
        DFSDM3_CNVTIMR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
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
