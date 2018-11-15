#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Sigma-delta analog-to-digital converter
//!
//! Used by: stm32f301, stm32f373, stm32f3x8

#[cfg(not(feature = "nosync"))]
pub use stm32f3::peripherals::sdadc::Instance;
pub use stm32f3::peripherals::sdadc::{RegisterBlock, ResetValues};
pub use stm32f3::peripherals::sdadc::{
    CLRISR, CONF0R, CONF1R, CONF2R, CONFCHR1, CONFCHR2, CR1, CR2, ISR, JCHGR, JDATA12R, JDATA13R,
    JDATAR, RDATA12R, RDATA13R, RDATAR,
};

/// Access functions for the SDADC1 peripheral instance
pub mod SDADC1 {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40016000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SDADC1
    pub const reset: ResetValues = ResetValues {
        CR1: 0x00000000,
        CR2: 0x00000000,
        ISR: 0x00000000,
        CLRISR: 0x00000000,
        JCHGR: 0x00000001,
        CONF0R: 0x00000000,
        CONF1R: 0x00000000,
        CONF2R: 0x00000000,
        CONFCHR1: 0x00000000,
        CONFCHR2: 0x00000000,
        JDATAR: 0x00000000,
        RDATAR: 0x00000000,
        JDATA12R: 0x00000000,
        RDATA12R: 0x00000000,
        JDATA13R: 0x00000000,
        RDATA13R: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[no_mangle]
    static mut SDADC1_TAKEN: bool = false;

    /// Safe access to SDADC1
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
            if SDADC1_TAKEN {
                None
            } else {
                SDADC1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SDADC1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SDADC1_TAKEN && inst.addr == INSTANCE.addr {
                SDADC1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }
}

/// Raw pointer to SDADC1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SDADC1: *const RegisterBlock = 0x40016000 as *const _;

/// Access functions for the SDADC2 peripheral instance
pub mod SDADC2 {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40016400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SDADC2
    pub const reset: ResetValues = ResetValues {
        CR1: 0x00000000,
        CR2: 0x00000000,
        ISR: 0x00000000,
        CLRISR: 0x00000000,
        JCHGR: 0x00000001,
        CONF0R: 0x00000000,
        CONF1R: 0x00000000,
        CONF2R: 0x00000000,
        CONFCHR1: 0x00000000,
        CONFCHR2: 0x00000000,
        JDATAR: 0x00000000,
        RDATAR: 0x00000000,
        JDATA12R: 0x00000000,
        RDATA12R: 0x00000000,
        JDATA13R: 0x00000000,
        RDATA13R: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[no_mangle]
    static mut SDADC2_TAKEN: bool = false;

    /// Safe access to SDADC2
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
            if SDADC2_TAKEN {
                None
            } else {
                SDADC2_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SDADC2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SDADC2_TAKEN && inst.addr == INSTANCE.addr {
                SDADC2_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }
}

/// Raw pointer to SDADC2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SDADC2: *const RegisterBlock = 0x40016400 as *const _;

/// Access functions for the SDADC3 peripheral instance
pub mod SDADC3 {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40016800,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SDADC3
    pub const reset: ResetValues = ResetValues {
        CR1: 0x00000000,
        CR2: 0x00000000,
        ISR: 0x00000000,
        CLRISR: 0x00000000,
        JCHGR: 0x00000001,
        CONF0R: 0x00000000,
        CONF1R: 0x00000000,
        CONF2R: 0x00000000,
        CONFCHR1: 0x00000000,
        CONFCHR2: 0x00000000,
        JDATAR: 0x00000000,
        RDATAR: 0x00000000,
        JDATA12R: 0x00000000,
        RDATA12R: 0x00000000,
        JDATA13R: 0x00000000,
        RDATA13R: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[no_mangle]
    static mut SDADC3_TAKEN: bool = false;

    /// Safe access to SDADC3
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
            if SDADC3_TAKEN {
                None
            } else {
                SDADC3_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SDADC3
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SDADC3_TAKEN && inst.addr == INSTANCE.addr {
                SDADC3_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }
}

/// Raw pointer to SDADC3
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SDADC3: *const RegisterBlock = 0x40016800 as *const _;
