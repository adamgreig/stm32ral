#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! I2C1
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::i2c::Instance;
pub use crate::stm32mp::peripherals::i2c::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::i2c::{
    I2C_CR1, I2C_CR2, I2C_HWCFGR, I2C_ICR, I2C_IPIDR, I2C_ISR, I2C_OAR1, I2C_OAR2, I2C_PECR,
    I2C_RXDR, I2C_SIDR, I2C_TIMEOUTR, I2C_TIMINGR, I2C_TXDR, I2C_VERR,
};

/// Access functions for the I2C1 peripheral instance
pub mod I2C1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40012000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in I2C1
    pub const reset: ResetValues = ResetValues {
        I2C_CR1: 0x00000000,
        I2C_CR2: 0x00000000,
        I2C_OAR1: 0x00000000,
        I2C_OAR2: 0x00000000,
        I2C_TIMINGR: 0x00000000,
        I2C_TIMEOUTR: 0x00000000,
        I2C_ISR: 0x00000001,
        I2C_ICR: 0x00000000,
        I2C_PECR: 0x00000000,
        I2C_RXDR: 0x00000000,
        I2C_TXDR: 0x00000000,
        I2C_HWCFGR: 0x00000111,
        I2C_VERR: 0x00000012,
        I2C_IPIDR: 0x00130012,
        I2C_SIDR: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut I2C1_TAKEN: bool = false;

    /// Safe access to I2C1
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
            if I2C1_TAKEN {
                None
            } else {
                I2C1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to I2C1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if I2C1_TAKEN && inst.addr == INSTANCE.addr {
                I2C1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal I2C1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        I2C1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to I2C1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const I2C1: *const RegisterBlock = 0x40012000 as *const _;

/// Access functions for the I2C2 peripheral instance
pub mod I2C2 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40013000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in I2C2
    pub const reset: ResetValues = ResetValues {
        I2C_CR1: 0x00000000,
        I2C_CR2: 0x00000000,
        I2C_OAR1: 0x00000000,
        I2C_OAR2: 0x00000000,
        I2C_TIMINGR: 0x00000000,
        I2C_TIMEOUTR: 0x00000000,
        I2C_ISR: 0x00000001,
        I2C_ICR: 0x00000000,
        I2C_PECR: 0x00000000,
        I2C_RXDR: 0x00000000,
        I2C_TXDR: 0x00000000,
        I2C_HWCFGR: 0x00000111,
        I2C_VERR: 0x00000012,
        I2C_IPIDR: 0x00130012,
        I2C_SIDR: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut I2C2_TAKEN: bool = false;

    /// Safe access to I2C2
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
            if I2C2_TAKEN {
                None
            } else {
                I2C2_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to I2C2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if I2C2_TAKEN && inst.addr == INSTANCE.addr {
                I2C2_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal I2C2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        I2C2_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to I2C2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const I2C2: *const RegisterBlock = 0x40013000 as *const _;

/// Access functions for the I2C3 peripheral instance
pub mod I2C3 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40014000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in I2C3
    pub const reset: ResetValues = ResetValues {
        I2C_CR1: 0x00000000,
        I2C_CR2: 0x00000000,
        I2C_OAR1: 0x00000000,
        I2C_OAR2: 0x00000000,
        I2C_TIMINGR: 0x00000000,
        I2C_TIMEOUTR: 0x00000000,
        I2C_ISR: 0x00000001,
        I2C_ICR: 0x00000000,
        I2C_PECR: 0x00000000,
        I2C_RXDR: 0x00000000,
        I2C_TXDR: 0x00000000,
        I2C_HWCFGR: 0x00000111,
        I2C_VERR: 0x00000012,
        I2C_IPIDR: 0x00130012,
        I2C_SIDR: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut I2C3_TAKEN: bool = false;

    /// Safe access to I2C3
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
            if I2C3_TAKEN {
                None
            } else {
                I2C3_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to I2C3
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if I2C3_TAKEN && inst.addr == INSTANCE.addr {
                I2C3_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal I2C3
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        I2C3_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to I2C3
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const I2C3: *const RegisterBlock = 0x40014000 as *const _;

/// Access functions for the I2C4 peripheral instance
pub mod I2C4 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x5c002000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in I2C4
    pub const reset: ResetValues = ResetValues {
        I2C_CR1: 0x00000000,
        I2C_CR2: 0x00000000,
        I2C_OAR1: 0x00000000,
        I2C_OAR2: 0x00000000,
        I2C_TIMINGR: 0x00000000,
        I2C_TIMEOUTR: 0x00000000,
        I2C_ISR: 0x00000001,
        I2C_ICR: 0x00000000,
        I2C_PECR: 0x00000000,
        I2C_RXDR: 0x00000000,
        I2C_TXDR: 0x00000000,
        I2C_HWCFGR: 0x00000111,
        I2C_VERR: 0x00000012,
        I2C_IPIDR: 0x00130012,
        I2C_SIDR: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut I2C4_TAKEN: bool = false;

    /// Safe access to I2C4
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
            if I2C4_TAKEN {
                None
            } else {
                I2C4_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to I2C4
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if I2C4_TAKEN && inst.addr == INSTANCE.addr {
                I2C4_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal I2C4
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        I2C4_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to I2C4
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const I2C4: *const RegisterBlock = 0x5c002000 as *const _;

/// Access functions for the I2C5 peripheral instance
pub mod I2C5 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40015000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in I2C5
    pub const reset: ResetValues = ResetValues {
        I2C_CR1: 0x00000000,
        I2C_CR2: 0x00000000,
        I2C_OAR1: 0x00000000,
        I2C_OAR2: 0x00000000,
        I2C_TIMINGR: 0x00000000,
        I2C_TIMEOUTR: 0x00000000,
        I2C_ISR: 0x00000001,
        I2C_ICR: 0x00000000,
        I2C_PECR: 0x00000000,
        I2C_RXDR: 0x00000000,
        I2C_TXDR: 0x00000000,
        I2C_HWCFGR: 0x00000111,
        I2C_VERR: 0x00000012,
        I2C_IPIDR: 0x00130012,
        I2C_SIDR: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut I2C5_TAKEN: bool = false;

    /// Safe access to I2C5
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
            if I2C5_TAKEN {
                None
            } else {
                I2C5_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to I2C5
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if I2C5_TAKEN && inst.addr == INSTANCE.addr {
                I2C5_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal I2C5
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        I2C5_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to I2C5
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const I2C5: *const RegisterBlock = 0x40015000 as *const _;

/// Access functions for the I2C6 peripheral instance
pub mod I2C6 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x5c009000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in I2C6
    pub const reset: ResetValues = ResetValues {
        I2C_CR1: 0x00000000,
        I2C_CR2: 0x00000000,
        I2C_OAR1: 0x00000000,
        I2C_OAR2: 0x00000000,
        I2C_TIMINGR: 0x00000000,
        I2C_TIMEOUTR: 0x00000000,
        I2C_ISR: 0x00000001,
        I2C_ICR: 0x00000000,
        I2C_PECR: 0x00000000,
        I2C_RXDR: 0x00000000,
        I2C_TXDR: 0x00000000,
        I2C_HWCFGR: 0x00000111,
        I2C_VERR: 0x00000012,
        I2C_IPIDR: 0x00130012,
        I2C_SIDR: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut I2C6_TAKEN: bool = false;

    /// Safe access to I2C6
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
            if I2C6_TAKEN {
                None
            } else {
                I2C6_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to I2C6
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if I2C6_TAKEN && inst.addr == INSTANCE.addr {
                I2C6_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal I2C6
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        I2C6_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to I2C6
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const I2C6: *const RegisterBlock = 0x5c009000 as *const _;
