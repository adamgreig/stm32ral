#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! LPTIM1
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::lptim::Instance;
pub use crate::stm32mp::peripherals::lptim::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::lptim::{
    LPTIM1_HWCFGR, LPTIM_ARR, LPTIM_CFGR, LPTIM_CFGR2, LPTIM_CMP, LPTIM_CNT, LPTIM_CR, LPTIM_ICR,
    LPTIM_IER, LPTIM_ISR, LPTIM_PIDR, LPTIM_SIDR, LPTIM_VERR,
};

/// Access functions for the LPTIM1 peripheral instance
pub mod LPTIM1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40009000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in LPTIM1
    pub const reset: ResetValues = ResetValues {
        LPTIM_ISR: 0x00000000,
        LPTIM_ICR: 0x00000000,
        LPTIM_IER: 0x00000000,
        LPTIM_CFGR: 0x00000000,
        LPTIM_CR: 0x00000000,
        LPTIM_CMP: 0x00000000,
        LPTIM_ARR: 0x00000001,
        LPTIM_CNT: 0x00000000,
        LPTIM_CFGR2: 0x00000000,
        LPTIM1_HWCFGR: 0x00010804,
        LPTIM_VERR: 0x00000014,
        LPTIM_PIDR: 0x00120011,
        LPTIM_SIDR: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut LPTIM1_TAKEN: bool = false;

    /// Safe access to LPTIM1
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
            if LPTIM1_TAKEN {
                None
            } else {
                LPTIM1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to LPTIM1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if LPTIM1_TAKEN && inst.addr == INSTANCE.addr {
                LPTIM1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal LPTIM1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        LPTIM1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to LPTIM1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const LPTIM1: *const RegisterBlock = 0x40009000 as *const _;

/// Access functions for the LPTIM2 peripheral instance
pub mod LPTIM2 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50021000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in LPTIM2
    pub const reset: ResetValues = ResetValues {
        LPTIM_ISR: 0x00000000,
        LPTIM_ICR: 0x00000000,
        LPTIM_IER: 0x00000000,
        LPTIM_CFGR: 0x00000000,
        LPTIM_CR: 0x00000000,
        LPTIM_CMP: 0x00000000,
        LPTIM_ARR: 0x00000001,
        LPTIM_CNT: 0x00000000,
        LPTIM_CFGR2: 0x00000000,
        LPTIM1_HWCFGR: 0x00010804,
        LPTIM_VERR: 0x00000014,
        LPTIM_PIDR: 0x00120011,
        LPTIM_SIDR: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut LPTIM2_TAKEN: bool = false;

    /// Safe access to LPTIM2
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
            if LPTIM2_TAKEN {
                None
            } else {
                LPTIM2_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to LPTIM2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if LPTIM2_TAKEN && inst.addr == INSTANCE.addr {
                LPTIM2_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal LPTIM2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        LPTIM2_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to LPTIM2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const LPTIM2: *const RegisterBlock = 0x50021000 as *const _;

/// Access functions for the LPTIM3 peripheral instance
pub mod LPTIM3 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50022000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in LPTIM3
    pub const reset: ResetValues = ResetValues {
        LPTIM_ISR: 0x00000000,
        LPTIM_ICR: 0x00000000,
        LPTIM_IER: 0x00000000,
        LPTIM_CFGR: 0x00000000,
        LPTIM_CR: 0x00000000,
        LPTIM_CMP: 0x00000000,
        LPTIM_ARR: 0x00000001,
        LPTIM_CNT: 0x00000000,
        LPTIM_CFGR2: 0x00000000,
        LPTIM1_HWCFGR: 0x00010804,
        LPTIM_VERR: 0x00000014,
        LPTIM_PIDR: 0x00120011,
        LPTIM_SIDR: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut LPTIM3_TAKEN: bool = false;

    /// Safe access to LPTIM3
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
            if LPTIM3_TAKEN {
                None
            } else {
                LPTIM3_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to LPTIM3
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if LPTIM3_TAKEN && inst.addr == INSTANCE.addr {
                LPTIM3_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal LPTIM3
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        LPTIM3_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to LPTIM3
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const LPTIM3: *const RegisterBlock = 0x50022000 as *const _;

/// Access functions for the LPTIM4 peripheral instance
pub mod LPTIM4 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50023000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in LPTIM4
    pub const reset: ResetValues = ResetValues {
        LPTIM_ISR: 0x00000000,
        LPTIM_ICR: 0x00000000,
        LPTIM_IER: 0x00000000,
        LPTIM_CFGR: 0x00000000,
        LPTIM_CR: 0x00000000,
        LPTIM_CMP: 0x00000000,
        LPTIM_ARR: 0x00000001,
        LPTIM_CNT: 0x00000000,
        LPTIM_CFGR2: 0x00000000,
        LPTIM1_HWCFGR: 0x00010804,
        LPTIM_VERR: 0x00000014,
        LPTIM_PIDR: 0x00120011,
        LPTIM_SIDR: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut LPTIM4_TAKEN: bool = false;

    /// Safe access to LPTIM4
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
            if LPTIM4_TAKEN {
                None
            } else {
                LPTIM4_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to LPTIM4
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if LPTIM4_TAKEN && inst.addr == INSTANCE.addr {
                LPTIM4_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal LPTIM4
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        LPTIM4_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to LPTIM4
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const LPTIM4: *const RegisterBlock = 0x50023000 as *const _;

/// Access functions for the LPTIM5 peripheral instance
pub mod LPTIM5 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50024000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in LPTIM5
    pub const reset: ResetValues = ResetValues {
        LPTIM_ISR: 0x00000000,
        LPTIM_ICR: 0x00000000,
        LPTIM_IER: 0x00000000,
        LPTIM_CFGR: 0x00000000,
        LPTIM_CR: 0x00000000,
        LPTIM_CMP: 0x00000000,
        LPTIM_ARR: 0x00000001,
        LPTIM_CNT: 0x00000000,
        LPTIM_CFGR2: 0x00000000,
        LPTIM1_HWCFGR: 0x00010804,
        LPTIM_VERR: 0x00000014,
        LPTIM_PIDR: 0x00120011,
        LPTIM_SIDR: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut LPTIM5_TAKEN: bool = false;

    /// Safe access to LPTIM5
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
            if LPTIM5_TAKEN {
                None
            } else {
                LPTIM5_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to LPTIM5
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if LPTIM5_TAKEN && inst.addr == INSTANCE.addr {
                LPTIM5_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal LPTIM5
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        LPTIM5_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to LPTIM5
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const LPTIM5: *const RegisterBlock = 0x50024000 as *const _;
