#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Analog-to-Digital Converter
//!
//! Used by: stm32g473, stm32g474, stm32g483, stm32g484

#[cfg(not(feature = "nosync"))]
pub use crate::stm32g4::peripherals::adc::Instance;
pub use crate::stm32g4::peripherals::adc::{RegisterBlock, ResetValues};
pub use crate::stm32g4::peripherals::adc::{
    AWD2CR, AWD3CR, CALFACT, CFGR, CFGR2, CR, DIFSEL, DR, GCOMP, IER, ISR, JDR1, JDR2, JDR3, JDR4,
    JSQR, OFR1, OFR2, OFR3, OFR4, SMPR1, SMPR2, SQR1, SQR2, SQR3, SQR4, TR1, TR2, TR3,
};

/// Access functions for the ADC1 peripheral instance
pub mod ADC1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50000000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in ADC1
    pub const reset: ResetValues = ResetValues {
        ISR: 0x00000000,
        IER: 0x00000000,
        CR: 0x20000000,
        CFGR: 0x80000000,
        CFGR2: 0x00000000,
        SMPR1: 0x00000000,
        SMPR2: 0x00000000,
        TR1: 0x0FFF0000,
        TR2: 0x00FF0000,
        TR3: 0x00FF0000,
        SQR1: 0x00000000,
        SQR2: 0x00000000,
        SQR3: 0x00000000,
        SQR4: 0x00000000,
        DR: 0x00000000,
        JSQR: 0x00000000,
        OFR1: 0x00000000,
        OFR2: 0x00000000,
        OFR3: 0x00000000,
        OFR4: 0x00000000,
        JDR1: 0x00000000,
        JDR2: 0x00000000,
        JDR3: 0x00000000,
        JDR4: 0x00000000,
        AWD2CR: 0x00000000,
        AWD3CR: 0x00000000,
        DIFSEL: 0x00000000,
        CALFACT: 0x00000000,
        GCOMP: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut ADC1_TAKEN: bool = false;

    /// Safe access to ADC1
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
            if ADC1_TAKEN {
                None
            } else {
                ADC1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to ADC1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if ADC1_TAKEN && inst.addr == INSTANCE.addr {
                ADC1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal ADC1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        ADC1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to ADC1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const ADC1: *const RegisterBlock = 0x50000000 as *const _;

/// Access functions for the ADC2 peripheral instance
pub mod ADC2 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50000100,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in ADC2
    pub const reset: ResetValues = ResetValues {
        ISR: 0x00000000,
        IER: 0x00000000,
        CR: 0x20000000,
        CFGR: 0x80000000,
        CFGR2: 0x00000000,
        SMPR1: 0x00000000,
        SMPR2: 0x00000000,
        TR1: 0x0FFF0000,
        TR2: 0x00FF0000,
        TR3: 0x00FF0000,
        SQR1: 0x00000000,
        SQR2: 0x00000000,
        SQR3: 0x00000000,
        SQR4: 0x00000000,
        DR: 0x00000000,
        JSQR: 0x00000000,
        OFR1: 0x00000000,
        OFR2: 0x00000000,
        OFR3: 0x00000000,
        OFR4: 0x00000000,
        JDR1: 0x00000000,
        JDR2: 0x00000000,
        JDR3: 0x00000000,
        JDR4: 0x00000000,
        AWD2CR: 0x00000000,
        AWD3CR: 0x00000000,
        DIFSEL: 0x00000000,
        CALFACT: 0x00000000,
        GCOMP: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut ADC2_TAKEN: bool = false;

    /// Safe access to ADC2
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
            if ADC2_TAKEN {
                None
            } else {
                ADC2_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to ADC2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if ADC2_TAKEN && inst.addr == INSTANCE.addr {
                ADC2_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal ADC2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        ADC2_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to ADC2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const ADC2: *const RegisterBlock = 0x50000100 as *const _;

/// Access functions for the ADC3 peripheral instance
pub mod ADC3 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50000400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in ADC3
    pub const reset: ResetValues = ResetValues {
        ISR: 0x00000000,
        IER: 0x00000000,
        CR: 0x20002000,
        CFGR: 0x80000000,
        CFGR2: 0x00000000,
        SMPR1: 0x00000000,
        SMPR2: 0x00000000,
        TR1: 0x0FFF0000,
        TR2: 0x00FF0000,
        TR3: 0x00FF0000,
        SQR1: 0x00000000,
        SQR2: 0x00000000,
        SQR3: 0x00000000,
        SQR4: 0x00000000,
        DR: 0x00000000,
        JSQR: 0x00000000,
        OFR1: 0x00000000,
        OFR2: 0x00000000,
        OFR3: 0x00000000,
        OFR4: 0x00000000,
        JDR1: 0x00000000,
        JDR2: 0x00000000,
        JDR3: 0x00000000,
        JDR4: 0x00000000,
        AWD2CR: 0x00000000,
        AWD3CR: 0x00000000,
        DIFSEL: 0x00000000,
        CALFACT: 0x00000000,
        GCOMP: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut ADC3_TAKEN: bool = false;

    /// Safe access to ADC3
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
            if ADC3_TAKEN {
                None
            } else {
                ADC3_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to ADC3
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if ADC3_TAKEN && inst.addr == INSTANCE.addr {
                ADC3_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal ADC3
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        ADC3_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to ADC3
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const ADC3: *const RegisterBlock = 0x50000400 as *const _;

/// Access functions for the ADC4 peripheral instance
pub mod ADC4 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50000500,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in ADC4
    pub const reset: ResetValues = ResetValues {
        ISR: 0x00000000,
        IER: 0x00000000,
        CR: 0x20000000,
        CFGR: 0x80000000,
        CFGR2: 0x00000000,
        SMPR1: 0x00000000,
        SMPR2: 0x00000000,
        TR1: 0x0FFF0000,
        TR2: 0x00FF0000,
        TR3: 0x00FF0000,
        SQR1: 0x00000000,
        SQR2: 0x00000000,
        SQR3: 0x00000000,
        SQR4: 0x00000000,
        DR: 0x00000000,
        JSQR: 0x00000000,
        OFR1: 0x00000000,
        OFR2: 0x00000000,
        OFR3: 0x00000000,
        OFR4: 0x00000000,
        JDR1: 0x00000000,
        JDR2: 0x00000000,
        JDR3: 0x00000000,
        JDR4: 0x00000000,
        AWD2CR: 0x00000000,
        AWD3CR: 0x00000000,
        DIFSEL: 0x00000000,
        CALFACT: 0x00000000,
        GCOMP: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut ADC4_TAKEN: bool = false;

    /// Safe access to ADC4
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
            if ADC4_TAKEN {
                None
            } else {
                ADC4_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to ADC4
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if ADC4_TAKEN && inst.addr == INSTANCE.addr {
                ADC4_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal ADC4
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        ADC4_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to ADC4
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const ADC4: *const RegisterBlock = 0x50000500 as *const _;

/// Access functions for the ADC5 peripheral instance
pub mod ADC5 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50000600,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in ADC5
    pub const reset: ResetValues = ResetValues {
        ISR: 0x00000000,
        IER: 0x00000000,
        CR: 0x20002000,
        CFGR: 0x80000000,
        CFGR2: 0x00000000,
        SMPR1: 0x00000000,
        SMPR2: 0x00000000,
        TR1: 0x0FFF0000,
        TR2: 0x00FF0000,
        TR3: 0x00FF0000,
        SQR1: 0x00000000,
        SQR2: 0x00000000,
        SQR3: 0x00000000,
        SQR4: 0x00000000,
        DR: 0x00000000,
        JSQR: 0x00000000,
        OFR1: 0x00000000,
        OFR2: 0x00000000,
        OFR3: 0x00000000,
        OFR4: 0x00000000,
        JDR1: 0x00000000,
        JDR2: 0x00000000,
        JDR3: 0x00000000,
        JDR4: 0x00000000,
        AWD2CR: 0x00000000,
        AWD3CR: 0x00000000,
        DIFSEL: 0x00000000,
        CALFACT: 0x00000000,
        GCOMP: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut ADC5_TAKEN: bool = false;

    /// Safe access to ADC5
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
            if ADC5_TAKEN {
                None
            } else {
                ADC5_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to ADC5
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if ADC5_TAKEN && inst.addr == INSTANCE.addr {
                ADC5_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal ADC5
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        ADC5_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to ADC5
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const ADC5: *const RegisterBlock = 0x50000600 as *const _;
