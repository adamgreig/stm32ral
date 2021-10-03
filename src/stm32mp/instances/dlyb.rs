#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DLYBQS
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::dlyb::Instance;
pub use crate::stm32mp::peripherals::dlyb::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::dlyb::{DLYB_CFGR, DLYB_CR, DLYB_IPIDR, DLYB_SIDR, DLYB_VERR};

/// Access functions for the DLYBQS peripheral instance
pub mod DLYBQS {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x58004000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DLYBQS
    pub const reset: ResetValues = ResetValues {
        DLYB_CR: 0x00000000,
        DLYB_CFGR: 0x00000000,
        DLYB_VERR: 0x00000011,
        DLYB_IPIDR: 0x00140051,
        DLYB_SIDR: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut DLYBQS_TAKEN: bool = false;

    /// Safe access to DLYBQS
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
            if DLYBQS_TAKEN {
                None
            } else {
                DLYBQS_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to DLYBQS
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if DLYBQS_TAKEN && inst.addr == INSTANCE.addr {
                DLYBQS_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal DLYBQS
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        DLYBQS_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to DLYBQS
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DLYBQS: *const RegisterBlock = 0x58004000 as *const _;

/// Access functions for the DLYBSD1 peripheral instance
pub mod DLYBSD1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x58006000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DLYBSD1
    pub const reset: ResetValues = ResetValues {
        DLYB_CR: 0x00000000,
        DLYB_CFGR: 0x00000000,
        DLYB_VERR: 0x00000011,
        DLYB_IPIDR: 0x00140051,
        DLYB_SIDR: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut DLYBSD1_TAKEN: bool = false;

    /// Safe access to DLYBSD1
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
            if DLYBSD1_TAKEN {
                None
            } else {
                DLYBSD1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to DLYBSD1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if DLYBSD1_TAKEN && inst.addr == INSTANCE.addr {
                DLYBSD1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal DLYBSD1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        DLYBSD1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to DLYBSD1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DLYBSD1: *const RegisterBlock = 0x58006000 as *const _;

/// Access functions for the DLYBSD2 peripheral instance
pub mod DLYBSD2 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x58008000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DLYBSD2
    pub const reset: ResetValues = ResetValues {
        DLYB_CR: 0x00000000,
        DLYB_CFGR: 0x00000000,
        DLYB_VERR: 0x00000011,
        DLYB_IPIDR: 0x00140051,
        DLYB_SIDR: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut DLYBSD2_TAKEN: bool = false;

    /// Safe access to DLYBSD2
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
            if DLYBSD2_TAKEN {
                None
            } else {
                DLYBSD2_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to DLYBSD2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if DLYBSD2_TAKEN && inst.addr == INSTANCE.addr {
                DLYBSD2_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal DLYBSD2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        DLYBSD2_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to DLYBSD2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DLYBSD2: *const RegisterBlock = 0x58008000 as *const _;

/// Access functions for the DLYBSD3 peripheral instance
pub mod DLYBSD3 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x48005000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DLYBSD3
    pub const reset: ResetValues = ResetValues {
        DLYB_CR: 0x00000000,
        DLYB_CFGR: 0x00000000,
        DLYB_VERR: 0x00000011,
        DLYB_IPIDR: 0x00140051,
        DLYB_SIDR: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut DLYBSD3_TAKEN: bool = false;

    /// Safe access to DLYBSD3
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
            if DLYBSD3_TAKEN {
                None
            } else {
                DLYBSD3_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to DLYBSD3
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if DLYBSD3_TAKEN && inst.addr == INSTANCE.addr {
                DLYBSD3_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal DLYBSD3
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        DLYBSD3_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to DLYBSD3
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DLYBSD3: *const RegisterBlock = 0x48005000 as *const _;
