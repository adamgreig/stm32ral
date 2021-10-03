#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Digital-to-analog converter
//!
//! Used by: stm32g431, stm32g441, stm32g471, stm32g473, stm32g474, stm32g483, stm32g484, stm32g491, stm32g4a1

#[cfg(not(feature = "nosync"))]
pub use crate::stm32g4::peripherals::dac::Instance;
pub use crate::stm32g4::peripherals::dac::{RegisterBlock, ResetValues};
pub use crate::stm32g4::peripherals::dac::{
    DAC_CCR, DAC_CR, DAC_DHR12L1, DAC_DHR12L2, DAC_DHR12LD, DAC_DHR12R1, DAC_DHR12R2, DAC_DHR12RD,
    DAC_DHR8R1, DAC_DHR8R2, DAC_DHR8RD, DAC_DOR1, DAC_DOR2, DAC_MCR, DAC_SHHR, DAC_SHRR, DAC_SHSR1,
    DAC_SHSR2, DAC_SR, DAC_STMODR, DAC_STR1, DAC_STR2, DAC_SWTRGR,
};

/// Access functions for the DAC1 peripheral instance
pub mod DAC1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50000800,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DAC1
    pub const reset: ResetValues = ResetValues {
        DAC_CR: 0x00000000,
        DAC_SWTRGR: 0x00000000,
        DAC_DHR12R1: 0x00000000,
        DAC_DHR12L1: 0x00000000,
        DAC_DHR8R1: 0x00000000,
        DAC_DHR12R2: 0x00000000,
        DAC_DHR12L2: 0x00000000,
        DAC_DHR8R2: 0x00000000,
        DAC_DHR12RD: 0x00000000,
        DAC_DHR12LD: 0x00000000,
        DAC_DHR8RD: 0x00000000,
        DAC_DOR1: 0x00000000,
        DAC_DOR2: 0x00000000,
        DAC_SR: 0x00000000,
        DAC_CCR: 0x00000000,
        DAC_MCR: 0x00000000,
        DAC_SHSR1: 0x00000000,
        DAC_SHSR2: 0x00000000,
        DAC_SHHR: 0x00010001,
        DAC_SHRR: 0x00010001,
        DAC_STR1: 0x00000000,
        DAC_STR2: 0x00000000,
        DAC_STMODR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut DAC1_TAKEN: bool = false;

    /// Safe access to DAC1
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
            if DAC1_TAKEN {
                None
            } else {
                DAC1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to DAC1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if DAC1_TAKEN && inst.addr == INSTANCE.addr {
                DAC1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal DAC1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        DAC1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to DAC1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DAC1: *const RegisterBlock = 0x50000800 as *const _;

/// Access functions for the DAC2 peripheral instance
pub mod DAC2 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50000c00,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DAC2
    pub const reset: ResetValues = ResetValues {
        DAC_CR: 0x00000000,
        DAC_SWTRGR: 0x00000000,
        DAC_DHR12R1: 0x00000000,
        DAC_DHR12L1: 0x00000000,
        DAC_DHR8R1: 0x00000000,
        DAC_DHR12R2: 0x00000000,
        DAC_DHR12L2: 0x00000000,
        DAC_DHR8R2: 0x00000000,
        DAC_DHR12RD: 0x00000000,
        DAC_DHR12LD: 0x00000000,
        DAC_DHR8RD: 0x00000000,
        DAC_DOR1: 0x00000000,
        DAC_DOR2: 0x00000000,
        DAC_SR: 0x00000000,
        DAC_CCR: 0x00000000,
        DAC_MCR: 0x00000000,
        DAC_SHSR1: 0x00000000,
        DAC_SHSR2: 0x00000000,
        DAC_SHHR: 0x00010001,
        DAC_SHRR: 0x00010001,
        DAC_STR1: 0x00000000,
        DAC_STR2: 0x00000000,
        DAC_STMODR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut DAC2_TAKEN: bool = false;

    /// Safe access to DAC2
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
            if DAC2_TAKEN {
                None
            } else {
                DAC2_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to DAC2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if DAC2_TAKEN && inst.addr == INSTANCE.addr {
                DAC2_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal DAC2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        DAC2_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to DAC2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DAC2: *const RegisterBlock = 0x50000c00 as *const _;

/// Access functions for the DAC3 peripheral instance
pub mod DAC3 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50001000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DAC3
    pub const reset: ResetValues = ResetValues {
        DAC_CR: 0x00000000,
        DAC_SWTRGR: 0x00000000,
        DAC_DHR12R1: 0x00000000,
        DAC_DHR12L1: 0x00000000,
        DAC_DHR8R1: 0x00000000,
        DAC_DHR12R2: 0x00000000,
        DAC_DHR12L2: 0x00000000,
        DAC_DHR8R2: 0x00000000,
        DAC_DHR12RD: 0x00000000,
        DAC_DHR12LD: 0x00000000,
        DAC_DHR8RD: 0x00000000,
        DAC_DOR1: 0x00000000,
        DAC_DOR2: 0x00000000,
        DAC_SR: 0x00000000,
        DAC_CCR: 0x00000000,
        DAC_MCR: 0x00000000,
        DAC_SHSR1: 0x00000000,
        DAC_SHSR2: 0x00000000,
        DAC_SHHR: 0x00010001,
        DAC_SHRR: 0x00010001,
        DAC_STR1: 0x00000000,
        DAC_STR2: 0x00000000,
        DAC_STMODR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut DAC3_TAKEN: bool = false;

    /// Safe access to DAC3
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
            if DAC3_TAKEN {
                None
            } else {
                DAC3_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to DAC3
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if DAC3_TAKEN && inst.addr == INSTANCE.addr {
                DAC3_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal DAC3
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        DAC3_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to DAC3
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DAC3: *const RegisterBlock = 0x50001000 as *const _;

/// Access functions for the DAC4 peripheral instance
pub mod DAC4 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50001400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DAC4
    pub const reset: ResetValues = ResetValues {
        DAC_CR: 0x00000000,
        DAC_SWTRGR: 0x00000000,
        DAC_DHR12R1: 0x00000000,
        DAC_DHR12L1: 0x00000000,
        DAC_DHR8R1: 0x00000000,
        DAC_DHR12R2: 0x00000000,
        DAC_DHR12L2: 0x00000000,
        DAC_DHR8R2: 0x00000000,
        DAC_DHR12RD: 0x00000000,
        DAC_DHR12LD: 0x00000000,
        DAC_DHR8RD: 0x00000000,
        DAC_DOR1: 0x00000000,
        DAC_DOR2: 0x00000000,
        DAC_SR: 0x00000000,
        DAC_CCR: 0x00000000,
        DAC_MCR: 0x00000000,
        DAC_SHSR1: 0x00000000,
        DAC_SHSR2: 0x00000000,
        DAC_SHHR: 0x00010001,
        DAC_SHRR: 0x00010001,
        DAC_STR1: 0x00000000,
        DAC_STR2: 0x00000000,
        DAC_STMODR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut DAC4_TAKEN: bool = false;

    /// Safe access to DAC4
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
            if DAC4_TAKEN {
                None
            } else {
                DAC4_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to DAC4
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if DAC4_TAKEN && inst.addr == INSTANCE.addr {
                DAC4_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal DAC4
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        DAC4_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to DAC4
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DAC4: *const RegisterBlock = 0x50001400 as *const _;
