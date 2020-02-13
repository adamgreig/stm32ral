#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! WWDG
//!
//! Used by: stm32h747cm4, stm32h747cm7, stm32h757cm7

#[cfg(not(feature = "nosync"))]
pub use crate::stm32h7::peripherals::wwdg::Instance;
pub use crate::stm32h7::peripherals::wwdg::{RegisterBlock, ResetValues};
pub use crate::stm32h7::peripherals::wwdg::{CFR, CR, SR};

/// Access functions for the WWDG1 peripheral instance
pub mod WWDG1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50003000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in WWDG1
    pub const reset: ResetValues = ResetValues {
        CR: 0x0000007F,
        CFR: 0x0000007F,
        SR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut WWDG1_TAKEN: bool = false;

    /// Safe access to WWDG1
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
            if WWDG1_TAKEN {
                None
            } else {
                WWDG1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to WWDG1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if WWDG1_TAKEN && inst.addr == INSTANCE.addr {
                WWDG1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal WWDG1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        WWDG1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to WWDG1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const WWDG1: *const RegisterBlock = 0x50003000 as *const _;

/// Access functions for the WWDG2 peripheral instance
pub mod WWDG2 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40002c00,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in WWDG2
    pub const reset: ResetValues = ResetValues {
        CR: 0x0000007F,
        CFR: 0x0000007F,
        SR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut WWDG2_TAKEN: bool = false;

    /// Safe access to WWDG2
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
            if WWDG2_TAKEN {
                None
            } else {
                WWDG2_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to WWDG2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if WWDG2_TAKEN && inst.addr == INSTANCE.addr {
                WWDG2_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal WWDG2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        WWDG2_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to WWDG2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const WWDG2: *const RegisterBlock = 0x40002c00 as *const _;
