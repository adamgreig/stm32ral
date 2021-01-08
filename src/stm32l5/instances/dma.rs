#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Direct memory access controller
//!
//! Used by: stm32l552, stm32l562

#[cfg(not(feature = "nosync"))]
pub use crate::stm32l5::peripherals::dma::Instance;
pub use crate::stm32l5::peripherals::dma::{RegisterBlock, ResetValues};
pub use crate::stm32l5::peripherals::dma::{
    CCR1, CCR2, CCR3, CCR4, CCR5, CCR6, CCR7, CCR8, CM0AR1, CM0AR2, CM0AR3, CM0AR4, CM0AR5, CM0AR6,
    CM0AR7, CM0AR8, CM1AR1, CM1AR2, CM1AR3, CM1AR4, CM1AR5, CM1AR6, CM1AR7, CM1AR8, CNDTR1, CNDTR2,
    CNDTR3, CNDTR4, CNDTR5, CNDTR6, CNDTR7, CNDTR8, CPAR1, CPAR2, CPAR3, CPAR4, CPAR5, CPAR6,
    CPAR7, CPAR8, CSELR, IFCR, ISR,
};

/// Access functions for the DMA1 peripheral instance
pub mod DMA1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40020000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DMA1
    pub const reset: ResetValues = ResetValues {
        ISR: 0x00000000,
        IFCR: 0x00000000,
        CCR1: 0x00000000,
        CNDTR1: 0x00000000,
        CPAR1: 0x00000000,
        CM0AR1: 0x00000000,
        CM1AR1: 0x00000000,
        CCR2: 0x00000000,
        CNDTR2: 0x00000000,
        CPAR2: 0x00000000,
        CM0AR2: 0x00000000,
        CM1AR2: 0x00000000,
        CCR3: 0x00000000,
        CNDTR3: 0x00000000,
        CPAR3: 0x00000000,
        CM0AR3: 0x00000000,
        CM1AR3: 0x00000000,
        CCR4: 0x00000000,
        CNDTR4: 0x00000000,
        CPAR4: 0x00000000,
        CM0AR4: 0x00000000,
        CM1AR4: 0x00000000,
        CCR5: 0x00000000,
        CNDTR5: 0x00000000,
        CPAR5: 0x00000000,
        CM0AR5: 0x00000000,
        CM1AR5: 0x00000000,
        CCR6: 0x00000000,
        CNDTR6: 0x00000000,
        CPAR6: 0x00000000,
        CM0AR6: 0x00000000,
        CM1AR6: 0x00000000,
        CCR7: 0x00000000,
        CNDTR7: 0x00000000,
        CPAR7: 0x00000000,
        CM0AR7: 0x00000000,
        CM1AR7: 0x00000000,
        CCR8: 0x00000000,
        CNDTR8: 0x00000000,
        CPAR8: 0x00000000,
        CM0AR8: 0x00000000,
        CM1AR8: 0x00000000,
        CSELR: 0x00000000,
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
pub const DMA1: *const RegisterBlock = 0x40020000 as *const _;

/// Access functions for the DMA2 peripheral instance
pub mod DMA2 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40020400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DMA2
    pub const reset: ResetValues = ResetValues {
        ISR: 0x00000000,
        IFCR: 0x00000000,
        CCR1: 0x00000000,
        CNDTR1: 0x00000000,
        CPAR1: 0x00000000,
        CM0AR1: 0x00000000,
        CM1AR1: 0x00000000,
        CCR2: 0x00000000,
        CNDTR2: 0x00000000,
        CPAR2: 0x00000000,
        CM0AR2: 0x00000000,
        CM1AR2: 0x00000000,
        CCR3: 0x00000000,
        CNDTR3: 0x00000000,
        CPAR3: 0x00000000,
        CM0AR3: 0x00000000,
        CM1AR3: 0x00000000,
        CCR4: 0x00000000,
        CNDTR4: 0x00000000,
        CPAR4: 0x00000000,
        CM0AR4: 0x00000000,
        CM1AR4: 0x00000000,
        CCR5: 0x00000000,
        CNDTR5: 0x00000000,
        CPAR5: 0x00000000,
        CM0AR5: 0x00000000,
        CM1AR5: 0x00000000,
        CCR6: 0x00000000,
        CNDTR6: 0x00000000,
        CPAR6: 0x00000000,
        CM0AR6: 0x00000000,
        CM1AR6: 0x00000000,
        CCR7: 0x00000000,
        CNDTR7: 0x00000000,
        CPAR7: 0x00000000,
        CM0AR7: 0x00000000,
        CM1AR7: 0x00000000,
        CCR8: 0x00000000,
        CNDTR8: 0x00000000,
        CPAR8: 0x00000000,
        CM0AR8: 0x00000000,
        CM1AR8: 0x00000000,
        CSELR: 0x00000000,
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
pub const DMA2: *const RegisterBlock = 0x40020400 as *const _;

/// Access functions for the SEC_DMA1 peripheral instance
pub mod SEC_DMA1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50020000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SEC_DMA1
    pub const reset: ResetValues = ResetValues {
        ISR: 0x00000000,
        IFCR: 0x00000000,
        CCR1: 0x00000000,
        CNDTR1: 0x00000000,
        CPAR1: 0x00000000,
        CM0AR1: 0x00000000,
        CM1AR1: 0x00000000,
        CCR2: 0x00000000,
        CNDTR2: 0x00000000,
        CPAR2: 0x00000000,
        CM0AR2: 0x00000000,
        CM1AR2: 0x00000000,
        CCR3: 0x00000000,
        CNDTR3: 0x00000000,
        CPAR3: 0x00000000,
        CM0AR3: 0x00000000,
        CM1AR3: 0x00000000,
        CCR4: 0x00000000,
        CNDTR4: 0x00000000,
        CPAR4: 0x00000000,
        CM0AR4: 0x00000000,
        CM1AR4: 0x00000000,
        CCR5: 0x00000000,
        CNDTR5: 0x00000000,
        CPAR5: 0x00000000,
        CM0AR5: 0x00000000,
        CM1AR5: 0x00000000,
        CCR6: 0x00000000,
        CNDTR6: 0x00000000,
        CPAR6: 0x00000000,
        CM0AR6: 0x00000000,
        CM1AR6: 0x00000000,
        CCR7: 0x00000000,
        CNDTR7: 0x00000000,
        CPAR7: 0x00000000,
        CM0AR7: 0x00000000,
        CM1AR7: 0x00000000,
        CCR8: 0x00000000,
        CNDTR8: 0x00000000,
        CPAR8: 0x00000000,
        CM0AR8: 0x00000000,
        CM1AR8: 0x00000000,
        CSELR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SEC_DMA1_TAKEN: bool = false;

    /// Safe access to SEC_DMA1
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
            if SEC_DMA1_TAKEN {
                None
            } else {
                SEC_DMA1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SEC_DMA1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SEC_DMA1_TAKEN && inst.addr == INSTANCE.addr {
                SEC_DMA1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SEC_DMA1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SEC_DMA1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SEC_DMA1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SEC_DMA1: *const RegisterBlock = 0x50020000 as *const _;

/// Access functions for the SEC_DMA2 peripheral instance
pub mod SEC_DMA2 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50020400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SEC_DMA2
    pub const reset: ResetValues = ResetValues {
        ISR: 0x00000000,
        IFCR: 0x00000000,
        CCR1: 0x00000000,
        CNDTR1: 0x00000000,
        CPAR1: 0x00000000,
        CM0AR1: 0x00000000,
        CM1AR1: 0x00000000,
        CCR2: 0x00000000,
        CNDTR2: 0x00000000,
        CPAR2: 0x00000000,
        CM0AR2: 0x00000000,
        CM1AR2: 0x00000000,
        CCR3: 0x00000000,
        CNDTR3: 0x00000000,
        CPAR3: 0x00000000,
        CM0AR3: 0x00000000,
        CM1AR3: 0x00000000,
        CCR4: 0x00000000,
        CNDTR4: 0x00000000,
        CPAR4: 0x00000000,
        CM0AR4: 0x00000000,
        CM1AR4: 0x00000000,
        CCR5: 0x00000000,
        CNDTR5: 0x00000000,
        CPAR5: 0x00000000,
        CM0AR5: 0x00000000,
        CM1AR5: 0x00000000,
        CCR6: 0x00000000,
        CNDTR6: 0x00000000,
        CPAR6: 0x00000000,
        CM0AR6: 0x00000000,
        CM1AR6: 0x00000000,
        CCR7: 0x00000000,
        CNDTR7: 0x00000000,
        CPAR7: 0x00000000,
        CM0AR7: 0x00000000,
        CM1AR7: 0x00000000,
        CCR8: 0x00000000,
        CNDTR8: 0x00000000,
        CPAR8: 0x00000000,
        CM0AR8: 0x00000000,
        CM1AR8: 0x00000000,
        CSELR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SEC_DMA2_TAKEN: bool = false;

    /// Safe access to SEC_DMA2
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
            if SEC_DMA2_TAKEN {
                None
            } else {
                SEC_DMA2_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SEC_DMA2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SEC_DMA2_TAKEN && inst.addr == INSTANCE.addr {
                SEC_DMA2_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SEC_DMA2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SEC_DMA2_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SEC_DMA2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SEC_DMA2: *const RegisterBlock = 0x50020400 as *const _;
