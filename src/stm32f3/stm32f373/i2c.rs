#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Inter-integrated circuit

#[cfg(not(feature = "nosync"))]
pub use stm32f3::peripherals::i2c::Instance;
pub use stm32f3::peripherals::i2c::{RegisterBlock, ResetValues};
pub use stm32f3::peripherals::i2c::{
    CR1, CR2, ICR, ISR, OAR1, OAR2, PECR, RXDR, TIMEOUTR, TIMINGR, TXDR,
};

/// Access functions for the I2C1 peripheral instance
pub mod I2C1 {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

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
        TIMINGR: 0x00000000,
        TIMEOUTR: 0x00000000,
        ISR: 0x00000001,
        ICR: 0x00000000,
        PECR: 0x00000000,
        RXDR: 0x00000000,
        TXDR: 0x00000000,
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
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

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
        TIMINGR: 0x00000000,
        TIMEOUTR: 0x00000000,
        ISR: 0x00000001,
        ICR: 0x00000000,
        PECR: 0x00000000,
        RXDR: 0x00000000,
        TXDR: 0x00000000,
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
