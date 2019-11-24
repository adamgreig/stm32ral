#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DMA controller
//!
//! Used by: stm32f745, stm32f765, stm32f7x6, stm32f7x7, stm32f7x9

#[cfg(not(feature = "nosync"))]
pub use crate::stm32f7::peripherals::dma_v1::Instance;
pub use crate::stm32f7::peripherals::dma_v1::{RegisterBlock, ResetValues};
pub use crate::stm32f7::peripherals::dma_v1::{
    CR0, CR1, CR2, CR3, CR4, CR5, CR6, CR7, FCR0, FCR1, FCR2, FCR3, FCR4, FCR5, FCR6, FCR7, HIFCR,
    HISR, LIFCR, LISR, M0AR0, M0AR1, M0AR2, M0AR3, M0AR4, M0AR5, M0AR6, M0AR7, M1AR0, M1AR1, M1AR2,
    M1AR3, M1AR4, M1AR5, M1AR6, M1AR7, NDTR0, NDTR1, NDTR2, NDTR3, NDTR4, NDTR5, NDTR6, NDTR7,
    PAR0, PAR1, PAR2, PAR3, PAR4, PAR5, PAR6, PAR7,
};

/// Access functions for the DMA1 peripheral instance
pub mod DMA1 {
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
        CR0: 0x00000000,
        NDTR0: 0x00000000,
        PAR0: 0x00000000,
        M0AR0: 0x00000000,
        M1AR0: 0x00000000,
        FCR0: 0x00000021,
        CR1: 0x00000000,
        NDTR1: 0x00000000,
        PAR1: 0x00000000,
        M0AR1: 0x00000000,
        M1AR1: 0x00000000,
        FCR1: 0x00000021,
        CR2: 0x00000000,
        NDTR2: 0x00000000,
        PAR2: 0x00000000,
        M0AR2: 0x00000000,
        M1AR2: 0x00000000,
        FCR2: 0x00000021,
        CR3: 0x00000000,
        NDTR3: 0x00000000,
        PAR3: 0x00000000,
        M0AR3: 0x00000000,
        M1AR3: 0x00000000,
        FCR3: 0x00000021,
        CR4: 0x00000000,
        NDTR4: 0x00000000,
        PAR4: 0x00000000,
        M0AR4: 0x00000000,
        M1AR4: 0x00000000,
        FCR4: 0x00000021,
        CR5: 0x00000000,
        NDTR5: 0x00000000,
        PAR5: 0x00000000,
        M0AR5: 0x00000000,
        M1AR5: 0x00000000,
        FCR5: 0x00000021,
        CR6: 0x00000000,
        NDTR6: 0x00000000,
        PAR6: 0x00000000,
        M0AR6: 0x00000000,
        M1AR6: 0x00000000,
        FCR6: 0x00000021,
        CR7: 0x00000000,
        NDTR7: 0x00000000,
        PAR7: 0x00000000,
        M0AR7: 0x00000000,
        M1AR7: 0x00000000,
        FCR7: 0x00000021,
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
pub const DMA1: *const RegisterBlock = 0x40026000 as *const _;

/// Access functions for the DMA2 peripheral instance
pub mod DMA2 {
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
        CR0: 0x00000000,
        NDTR0: 0x00000000,
        PAR0: 0x00000000,
        M0AR0: 0x00000000,
        M1AR0: 0x00000000,
        FCR0: 0x00000021,
        CR1: 0x00000000,
        NDTR1: 0x00000000,
        PAR1: 0x00000000,
        M0AR1: 0x00000000,
        M1AR1: 0x00000000,
        FCR1: 0x00000021,
        CR2: 0x00000000,
        NDTR2: 0x00000000,
        PAR2: 0x00000000,
        M0AR2: 0x00000000,
        M1AR2: 0x00000000,
        FCR2: 0x00000021,
        CR3: 0x00000000,
        NDTR3: 0x00000000,
        PAR3: 0x00000000,
        M0AR3: 0x00000000,
        M1AR3: 0x00000000,
        FCR3: 0x00000021,
        CR4: 0x00000000,
        NDTR4: 0x00000000,
        PAR4: 0x00000000,
        M0AR4: 0x00000000,
        M1AR4: 0x00000000,
        FCR4: 0x00000021,
        CR5: 0x00000000,
        NDTR5: 0x00000000,
        PAR5: 0x00000000,
        M0AR5: 0x00000000,
        M1AR5: 0x00000000,
        FCR5: 0x00000021,
        CR6: 0x00000000,
        NDTR6: 0x00000000,
        PAR6: 0x00000000,
        M0AR6: 0x00000000,
        M1AR6: 0x00000000,
        FCR6: 0x00000021,
        CR7: 0x00000000,
        NDTR7: 0x00000000,
        PAR7: 0x00000000,
        M0AR7: 0x00000000,
        M1AR7: 0x00000000,
        FCR7: 0x00000021,
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
pub const DMA2: *const RegisterBlock = 0x40026400 as *const _;
