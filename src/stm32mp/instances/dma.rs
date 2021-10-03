#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DMA1
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::dma::Instance;
pub use crate::stm32mp::peripherals::dma::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::dma::{
    DMA_HIFCR, DMA_HISR, DMA_HWCFGR1, DMA_HWCFGR2, DMA_IPDR, DMA_LIFCR, DMA_LISR, DMA_S0CR,
    DMA_S0FCR, DMA_S0M0AR, DMA_S0M1AR, DMA_S0NDTR, DMA_S0PAR, DMA_S1CR, DMA_S1FCR, DMA_S1M0AR,
    DMA_S1M1AR, DMA_S1NDTR, DMA_S1PAR, DMA_S2CR, DMA_S2FCR, DMA_S2M0AR, DMA_S2M1AR, DMA_S2NDTR,
    DMA_S2PAR, DMA_S3CR, DMA_S3FCR, DMA_S3M0AR, DMA_S3M1AR, DMA_S3NDTR, DMA_S3PAR, DMA_S4CR,
    DMA_S4FCR, DMA_S4M0AR, DMA_S4M1AR, DMA_S4NDTR, DMA_S4PAR, DMA_S5CR, DMA_S5FCR, DMA_S5M0AR,
    DMA_S5M1AR, DMA_S5NDTR, DMA_S5PAR, DMA_S6CR, DMA_S6FCR, DMA_S6M0AR, DMA_S6M1AR, DMA_S6NDTR,
    DMA_S6PAR, DMA_S7CR, DMA_S7FCR, DMA_S7M0AR, DMA_S7M1AR, DMA_S7NDTR, DMA_S7PAR, DMA_SIDR,
    DMA_VERR,
};

/// Access functions for the DMA1 peripheral instance
pub mod DMA1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x48000000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DMA1
    pub const reset: ResetValues = ResetValues {
        DMA_LISR: 0x00000000,
        DMA_HISR: 0x00000000,
        DMA_LIFCR: 0x00000000,
        DMA_HIFCR: 0x00000000,
        DMA_S0CR: 0x00000000,
        DMA_S0NDTR: 0x00000000,
        DMA_S0PAR: 0x00000000,
        DMA_S0M0AR: 0x00000000,
        DMA_S0M1AR: 0x00000000,
        DMA_S0FCR: 0x00000021,
        DMA_S1CR: 0x00000000,
        DMA_S1NDTR: 0x00000000,
        DMA_S1PAR: 0x00000000,
        DMA_S1M0AR: 0x00000000,
        DMA_S1M1AR: 0x00000000,
        DMA_S1FCR: 0x00000021,
        DMA_S2CR: 0x00000000,
        DMA_S2NDTR: 0x00000000,
        DMA_S2PAR: 0x00000000,
        DMA_S2M0AR: 0x00000000,
        DMA_S2M1AR: 0x00000000,
        DMA_S2FCR: 0x00000021,
        DMA_S3CR: 0x00000000,
        DMA_S3NDTR: 0x00000000,
        DMA_S3PAR: 0x00000000,
        DMA_S3M0AR: 0x00000000,
        DMA_S3M1AR: 0x00000000,
        DMA_S3FCR: 0x00000021,
        DMA_S4CR: 0x00000000,
        DMA_S4NDTR: 0x00000000,
        DMA_S4PAR: 0x00000000,
        DMA_S4M0AR: 0x00000000,
        DMA_S4M1AR: 0x00000000,
        DMA_S4FCR: 0x00000021,
        DMA_S5CR: 0x00000000,
        DMA_S5NDTR: 0x00000000,
        DMA_S5PAR: 0x00000000,
        DMA_S5M0AR: 0x00000000,
        DMA_S5M1AR: 0x00000000,
        DMA_S5FCR: 0x00000021,
        DMA_S6CR: 0x00000000,
        DMA_S6NDTR: 0x00000000,
        DMA_S6PAR: 0x00000000,
        DMA_S6M0AR: 0x00000000,
        DMA_S6M1AR: 0x00000000,
        DMA_S6FCR: 0x00000021,
        DMA_S7CR: 0x00000000,
        DMA_S7NDTR: 0x00000000,
        DMA_S7PAR: 0x00000000,
        DMA_S7M0AR: 0x00000000,
        DMA_S7M1AR: 0x00000000,
        DMA_S7FCR: 0x00000021,
        DMA_HWCFGR2: 0x00000001,
        DMA_HWCFGR1: 0x22222222,
        DMA_VERR: 0x00000014,
        DMA_IPDR: 0x00100002,
        DMA_SIDR: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
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

    /// Unsafely steal DMA1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        DMA1_TAKEN = true;
        INSTANCE
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
pub const DMA1: *const RegisterBlock = 0x48000000 as *const _;

/// Access functions for the DMA2 peripheral instance
pub mod DMA2 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x48001000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DMA2
    pub const reset: ResetValues = ResetValues {
        DMA_LISR: 0x00000000,
        DMA_HISR: 0x00000000,
        DMA_LIFCR: 0x00000000,
        DMA_HIFCR: 0x00000000,
        DMA_S0CR: 0x00000000,
        DMA_S0NDTR: 0x00000000,
        DMA_S0PAR: 0x00000000,
        DMA_S0M0AR: 0x00000000,
        DMA_S0M1AR: 0x00000000,
        DMA_S0FCR: 0x00000021,
        DMA_S1CR: 0x00000000,
        DMA_S1NDTR: 0x00000000,
        DMA_S1PAR: 0x00000000,
        DMA_S1M0AR: 0x00000000,
        DMA_S1M1AR: 0x00000000,
        DMA_S1FCR: 0x00000021,
        DMA_S2CR: 0x00000000,
        DMA_S2NDTR: 0x00000000,
        DMA_S2PAR: 0x00000000,
        DMA_S2M0AR: 0x00000000,
        DMA_S2M1AR: 0x00000000,
        DMA_S2FCR: 0x00000021,
        DMA_S3CR: 0x00000000,
        DMA_S3NDTR: 0x00000000,
        DMA_S3PAR: 0x00000000,
        DMA_S3M0AR: 0x00000000,
        DMA_S3M1AR: 0x00000000,
        DMA_S3FCR: 0x00000021,
        DMA_S4CR: 0x00000000,
        DMA_S4NDTR: 0x00000000,
        DMA_S4PAR: 0x00000000,
        DMA_S4M0AR: 0x00000000,
        DMA_S4M1AR: 0x00000000,
        DMA_S4FCR: 0x00000021,
        DMA_S5CR: 0x00000000,
        DMA_S5NDTR: 0x00000000,
        DMA_S5PAR: 0x00000000,
        DMA_S5M0AR: 0x00000000,
        DMA_S5M1AR: 0x00000000,
        DMA_S5FCR: 0x00000021,
        DMA_S6CR: 0x00000000,
        DMA_S6NDTR: 0x00000000,
        DMA_S6PAR: 0x00000000,
        DMA_S6M0AR: 0x00000000,
        DMA_S6M1AR: 0x00000000,
        DMA_S6FCR: 0x00000021,
        DMA_S7CR: 0x00000000,
        DMA_S7NDTR: 0x00000000,
        DMA_S7PAR: 0x00000000,
        DMA_S7M0AR: 0x00000000,
        DMA_S7M1AR: 0x00000000,
        DMA_S7FCR: 0x00000021,
        DMA_HWCFGR2: 0x00000001,
        DMA_HWCFGR1: 0x22222222,
        DMA_VERR: 0x00000014,
        DMA_IPDR: 0x00100002,
        DMA_SIDR: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
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

    /// Unsafely steal DMA2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        DMA2_TAKEN = true;
        INSTANCE
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
pub const DMA2: *const RegisterBlock = 0x48001000 as *const _;
