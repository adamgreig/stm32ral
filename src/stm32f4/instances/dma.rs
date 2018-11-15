#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DMA controller
//!
//! Used by: stm32f401, stm32f405, stm32f407, stm32f410, stm32f411, stm32f412, stm32f427, stm32f429, stm32f446, stm32f469

#[cfg(not(feature = "nosync"))]
pub use stm32f4::peripherals::dma::Instance;
pub use stm32f4::peripherals::dma::{RegisterBlock, ResetValues};
pub use stm32f4::peripherals::dma::{
    HIFCR, HISR, LIFCR, LISR, S0CR, S0FCR, S0M0AR, S0M1AR, S0NDTR, S0PAR, S1CR, S1FCR, S1M0AR,
    S1M1AR, S1NDTR, S1PAR, S2CR, S2FCR, S2M0AR, S2M1AR, S2NDTR, S2PAR, S3CR, S3FCR, S3M0AR, S3M1AR,
    S3NDTR, S3PAR, S4CR, S4FCR, S4M0AR, S4M1AR, S4NDTR, S4PAR, S5CR, S5FCR, S5M0AR, S5M1AR, S5NDTR,
    S5PAR, S6CR, S6FCR, S6M0AR, S6M1AR, S6NDTR, S6PAR, S7CR, S7FCR, S7M0AR, S7M1AR, S7NDTR, S7PAR,
};

/// Access functions for the DMA1 peripheral instance
pub mod DMA1 {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40026000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DMA1
    pub const reset: ResetValues = ResetValues {
        LISR: 0x00000000,
        HISR: 0x00000000,
        LIFCR: 0x00000000,
        HIFCR: 0x00000000,
        S0CR: 0x00000000,
        S0NDTR: 0x00000000,
        S0PAR: 0x00000000,
        S0M0AR: 0x00000000,
        S0M1AR: 0x00000000,
        S0FCR: 0x00000021,
        S1CR: 0x00000000,
        S1NDTR: 0x00000000,
        S1PAR: 0x00000000,
        S1M0AR: 0x00000000,
        S1M1AR: 0x00000000,
        S1FCR: 0x00000021,
        S2CR: 0x00000000,
        S2NDTR: 0x00000000,
        S2PAR: 0x00000000,
        S2M0AR: 0x00000000,
        S2M1AR: 0x00000000,
        S2FCR: 0x00000021,
        S3CR: 0x00000000,
        S3NDTR: 0x00000000,
        S3PAR: 0x00000000,
        S3M0AR: 0x00000000,
        S3M1AR: 0x00000000,
        S3FCR: 0x00000021,
        S4CR: 0x00000000,
        S4NDTR: 0x00000000,
        S4PAR: 0x00000000,
        S4M0AR: 0x00000000,
        S4M1AR: 0x00000000,
        S4FCR: 0x00000021,
        S5CR: 0x00000000,
        S5NDTR: 0x00000000,
        S5PAR: 0x00000000,
        S5M0AR: 0x00000000,
        S5M1AR: 0x00000000,
        S5FCR: 0x00000021,
        S6CR: 0x00000000,
        S6NDTR: 0x00000000,
        S6PAR: 0x00000000,
        S6M0AR: 0x00000000,
        S6M1AR: 0x00000000,
        S6FCR: 0x00000021,
        S7CR: 0x00000000,
        S7NDTR: 0x00000000,
        S7PAR: 0x00000000,
        S7M0AR: 0x00000000,
        S7M1AR: 0x00000000,
        S7FCR: 0x00000021,
    };

    #[cfg(not(feature = "nosync"))]
    #[no_mangle]
    static mut DMA1_TAKEN: bool = false;

    /// Safe access to DMA1
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
            if DMA1_TAKEN {
                None
            } else {
                DMA1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to DMA1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if DMA1_TAKEN && inst.addr == INSTANCE.addr {
                DMA1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }
}

/// Raw pointer to DMA1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DMA1: *const RegisterBlock = 0x40026000 as *const _;

/// Access functions for the DMA2 peripheral instance
pub mod DMA2 {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40026400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DMA2
    pub const reset: ResetValues = ResetValues {
        LISR: 0x00000000,
        HISR: 0x00000000,
        LIFCR: 0x00000000,
        HIFCR: 0x00000000,
        S0CR: 0x00000000,
        S0NDTR: 0x00000000,
        S0PAR: 0x00000000,
        S0M0AR: 0x00000000,
        S0M1AR: 0x00000000,
        S0FCR: 0x00000021,
        S1CR: 0x00000000,
        S1NDTR: 0x00000000,
        S1PAR: 0x00000000,
        S1M0AR: 0x00000000,
        S1M1AR: 0x00000000,
        S1FCR: 0x00000021,
        S2CR: 0x00000000,
        S2NDTR: 0x00000000,
        S2PAR: 0x00000000,
        S2M0AR: 0x00000000,
        S2M1AR: 0x00000000,
        S2FCR: 0x00000021,
        S3CR: 0x00000000,
        S3NDTR: 0x00000000,
        S3PAR: 0x00000000,
        S3M0AR: 0x00000000,
        S3M1AR: 0x00000000,
        S3FCR: 0x00000021,
        S4CR: 0x00000000,
        S4NDTR: 0x00000000,
        S4PAR: 0x00000000,
        S4M0AR: 0x00000000,
        S4M1AR: 0x00000000,
        S4FCR: 0x00000021,
        S5CR: 0x00000000,
        S5NDTR: 0x00000000,
        S5PAR: 0x00000000,
        S5M0AR: 0x00000000,
        S5M1AR: 0x00000000,
        S5FCR: 0x00000021,
        S6CR: 0x00000000,
        S6NDTR: 0x00000000,
        S6PAR: 0x00000000,
        S6M0AR: 0x00000000,
        S6M1AR: 0x00000000,
        S6FCR: 0x00000021,
        S7CR: 0x00000000,
        S7NDTR: 0x00000000,
        S7PAR: 0x00000000,
        S7M0AR: 0x00000000,
        S7M1AR: 0x00000000,
        S7FCR: 0x00000021,
    };

    #[cfg(not(feature = "nosync"))]
    #[no_mangle]
    static mut DMA2_TAKEN: bool = false;

    /// Safe access to DMA2
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
            if DMA2_TAKEN {
                None
            } else {
                DMA2_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to DMA2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if DMA2_TAKEN && inst.addr == INSTANCE.addr {
                DMA2_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }
}

/// Raw pointer to DMA2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DMA2: *const RegisterBlock = 0x40026400 as *const _;
