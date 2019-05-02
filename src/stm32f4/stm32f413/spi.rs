#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Serial peripheral interface

#[cfg(not(feature = "nosync"))]
pub use stm32f4::peripherals::spi::Instance;
pub use stm32f4::peripherals::spi::{RegisterBlock, ResetValues};
pub use stm32f4::peripherals::spi::{CR1, CR2, CRCPR, DR, I2SCFGR, I2SPR, RXCRCR, SR, TXCRCR};

/// Access functions for the SPI1 peripheral instance
pub mod SPI1 {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40013000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SPI1
    pub const reset: ResetValues = ResetValues {
        CR1: 0x00000000,
        CR2: 0x00000000,
        SR: 0x00000002,
        DR: 0x00000000,
        CRCPR: 0x00000007,
        RXCRCR: 0x00000000,
        TXCRCR: 0x00000000,
        I2SCFGR: 0x00000000,
        I2SPR: 0x0000000A,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SPI1_TAKEN: bool = false;

    /// Safe access to SPI1
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
            if SPI1_TAKEN {
                None
            } else {
                SPI1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SPI1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SPI1_TAKEN && inst.addr == INSTANCE.addr {
                SPI1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SPI1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SPI1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SPI1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SPI1: *const RegisterBlock = 0x40013000 as *const _;

/// Access functions for the SPI2 peripheral instance
pub mod SPI2 {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40003800,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SPI2
    pub const reset: ResetValues = ResetValues {
        CR1: 0x00000000,
        CR2: 0x00000000,
        SR: 0x00000002,
        DR: 0x00000000,
        CRCPR: 0x00000007,
        RXCRCR: 0x00000000,
        TXCRCR: 0x00000000,
        I2SCFGR: 0x00000000,
        I2SPR: 0x0000000A,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SPI2_TAKEN: bool = false;

    /// Safe access to SPI2
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
            if SPI2_TAKEN {
                None
            } else {
                SPI2_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SPI2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SPI2_TAKEN && inst.addr == INSTANCE.addr {
                SPI2_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SPI2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SPI2_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SPI2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SPI2: *const RegisterBlock = 0x40003800 as *const _;

/// Access functions for the SPI3 peripheral instance
pub mod SPI3 {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40003c00,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SPI3
    pub const reset: ResetValues = ResetValues {
        CR1: 0x00000000,
        CR2: 0x00000000,
        SR: 0x00000002,
        DR: 0x00000000,
        CRCPR: 0x00000007,
        RXCRCR: 0x00000000,
        TXCRCR: 0x00000000,
        I2SCFGR: 0x00000000,
        I2SPR: 0x0000000A,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SPI3_TAKEN: bool = false;

    /// Safe access to SPI3
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
            if SPI3_TAKEN {
                None
            } else {
                SPI3_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SPI3
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SPI3_TAKEN && inst.addr == INSTANCE.addr {
                SPI3_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SPI3
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SPI3_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SPI3
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SPI3: *const RegisterBlock = 0x40003c00 as *const _;

/// Access functions for the SPI4 peripheral instance
pub mod SPI4 {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40013400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SPI4
    pub const reset: ResetValues = ResetValues {
        CR1: 0x00000000,
        CR2: 0x00000000,
        SR: 0x00000002,
        DR: 0x00000000,
        CRCPR: 0x00000007,
        RXCRCR: 0x00000000,
        TXCRCR: 0x00000000,
        I2SCFGR: 0x00000000,
        I2SPR: 0x0000000A,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SPI4_TAKEN: bool = false;

    /// Safe access to SPI4
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
            if SPI4_TAKEN {
                None
            } else {
                SPI4_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SPI4
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SPI4_TAKEN && inst.addr == INSTANCE.addr {
                SPI4_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SPI4
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SPI4_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SPI4
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SPI4: *const RegisterBlock = 0x40013400 as *const _;

/// Access functions for the SPI5 peripheral instance
pub mod SPI5 {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40015000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SPI5
    pub const reset: ResetValues = ResetValues {
        CR1: 0x00000000,
        CR2: 0x00000000,
        SR: 0x00000002,
        DR: 0x00000000,
        CRCPR: 0x00000007,
        RXCRCR: 0x00000000,
        TXCRCR: 0x00000000,
        I2SCFGR: 0x00000000,
        I2SPR: 0x0000000A,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SPI5_TAKEN: bool = false;

    /// Safe access to SPI5
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
            if SPI5_TAKEN {
                None
            } else {
                SPI5_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SPI5
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SPI5_TAKEN && inst.addr == INSTANCE.addr {
                SPI5_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SPI5
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SPI5_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SPI5
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SPI5: *const RegisterBlock = 0x40015000 as *const _;

/// Access functions for the SPI6 peripheral instance
pub mod SPI6 {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40015400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SPI6
    pub const reset: ResetValues = ResetValues {
        CR1: 0x00000000,
        CR2: 0x00000000,
        SR: 0x00000002,
        DR: 0x00000000,
        CRCPR: 0x00000007,
        RXCRCR: 0x00000000,
        TXCRCR: 0x00000000,
        I2SCFGR: 0x00000000,
        I2SPR: 0x0000000A,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SPI6_TAKEN: bool = false;

    /// Safe access to SPI6
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
            if SPI6_TAKEN {
                None
            } else {
                SPI6_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SPI6
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SPI6_TAKEN && inst.addr == INSTANCE.addr {
                SPI6_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SPI6
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SPI6_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SPI6
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SPI6: *const RegisterBlock = 0x40015400 as *const _;
