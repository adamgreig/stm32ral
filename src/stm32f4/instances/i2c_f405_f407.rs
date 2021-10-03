#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Inter-integrated circuit
//!
//! Used by: stm32f405, stm32f407

#[cfg(not(feature = "nosync"))]
pub use crate::stm32f4::peripherals::i2c_v2::Instance;
pub use crate::stm32f4::peripherals::i2c_v2::{RegisterBlock, ResetValues};
pub use crate::stm32f4::peripherals::i2c_v2::{CCR, CR1, CR2, DR, OAR1, OAR2, SR1, SR2, TRISE};

/// Access functions for the I2C1 peripheral instance
pub mod I2C1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40005400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in I2C1
    pub const reset: ResetValues = ResetValues {
        CR1: 0x00000000,
        CR2: 0x00000000,
        OAR1: 0x00000000,
        OAR2: 0x00000000,
        DR: 0x00000000,
        SR1: 0x00000000,
        SR2: 0x00000000,
        CCR: 0x00000000,
        TRISE: 0x00000002,
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
pub const I2C1: *const RegisterBlock = 0x40005400 as *const _;

/// Access functions for the I2C2 peripheral instance
pub mod I2C2 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40005800,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in I2C2
    pub const reset: ResetValues = ResetValues {
        CR1: 0x00000000,
        CR2: 0x00000000,
        OAR1: 0x00000000,
        OAR2: 0x00000000,
        DR: 0x00000000,
        SR1: 0x00000000,
        SR2: 0x00000000,
        CCR: 0x00000000,
        TRISE: 0x00000002,
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
pub const I2C2: *const RegisterBlock = 0x40005800 as *const _;

/// Access functions for the I2C3 peripheral instance
pub mod I2C3 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40005c00,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in I2C3
    pub const reset: ResetValues = ResetValues {
        CR1: 0x00000000,
        CR2: 0x00000000,
        OAR1: 0x00000000,
        OAR2: 0x00000000,
        DR: 0x00000000,
        SR1: 0x00000000,
        SR2: 0x00000000,
        CCR: 0x00000000,
        TRISE: 0x00000002,
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
pub const I2C3: *const RegisterBlock = 0x40005c00 as *const _;
