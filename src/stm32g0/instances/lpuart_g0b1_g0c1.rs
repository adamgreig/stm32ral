#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Low-power universal asynchronous receiver transmitter
//!
//! Used by: stm32g0b1, stm32g0c1

#[cfg(not(feature = "nosync"))]
pub use crate::stm32g0::peripherals::lpuart_v3::Instance;
pub use crate::stm32g0::peripherals::lpuart_v3::{RegisterBlock, ResetValues};
pub use crate::stm32g0::peripherals::lpuart_v3::{
    BRR, CR1, CR2, CR3, ICR, ISR, PRESC, RDR, RQR, TDR,
};

/// Access functions for the LPUART1 peripheral instance
pub mod LPUART1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40008000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in LPUART1
    pub const reset: ResetValues = ResetValues {
        CR1: 0x00000000,
        CR2: 0x00000000,
        CR3: 0x00000000,
        BRR: 0x00000000,
        RQR: 0x00000000,
        ISR: 0x008000C0,
        ICR: 0x00000000,
        RDR: 0x00000000,
        TDR: 0x00000000,
        PRESC: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut LPUART1_TAKEN: bool = false;

    /// Safe access to LPUART1
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
            if LPUART1_TAKEN {
                None
            } else {
                LPUART1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to LPUART1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if LPUART1_TAKEN && inst.addr == INSTANCE.addr {
                LPUART1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal LPUART1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        LPUART1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to LPUART1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const LPUART1: *const RegisterBlock = 0x40008000 as *const _;

/// Access functions for the LPUART2 peripheral instance
pub mod LPUART2 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40008400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in LPUART2
    pub const reset: ResetValues = ResetValues {
        CR1: 0x00000000,
        CR2: 0x00000000,
        CR3: 0x00000000,
        BRR: 0x00000000,
        RQR: 0x00000000,
        ISR: 0x008000C0,
        ICR: 0x00000000,
        RDR: 0x00000000,
        TDR: 0x00000000,
        PRESC: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut LPUART2_TAKEN: bool = false;

    /// Safe access to LPUART2
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
            if LPUART2_TAKEN {
                None
            } else {
                LPUART2_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to LPUART2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if LPUART2_TAKEN && inst.addr == INSTANCE.addr {
                LPUART2_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal LPUART2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        LPUART2_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to LPUART2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const LPUART2: *const RegisterBlock = 0x40008400 as *const _;
