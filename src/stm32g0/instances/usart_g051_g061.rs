#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Universal synchronous asynchronous receiver transmitter
//!
//! Used by: stm32g051, stm32g061

#[cfg(not(feature = "nosync"))]
pub use crate::stm32g0::peripherals::usart_v2::Instance;
pub use crate::stm32g0::peripherals::usart_v2::{RegisterBlock, ResetValues};
pub use crate::stm32g0::peripherals::usart_v2::{
    BRR, CR1_FIFO, CR2, CR3, GTPR, ICR, ISR_FIFO, PRESC, RDR, RQR, RTOR, TDR,
};

/// Access functions for the USART1 peripheral instance
pub mod USART1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40013800,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in USART1
    pub const reset: ResetValues = ResetValues {
        CR1_FIFO: 0x00000000,
        CR2: 0x00000000,
        CR3: 0x00000000,
        BRR: 0x00000000,
        GTPR: 0x00000000,
        RTOR: 0x00000000,
        RQR: 0x00000000,
        ISR_FIFO: 0x000000C0,
        ICR: 0x00000000,
        RDR: 0x00000000,
        TDR: 0x00000000,
        PRESC: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut USART1_TAKEN: bool = false;

    /// Safe access to USART1
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
            if USART1_TAKEN {
                None
            } else {
                USART1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to USART1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if USART1_TAKEN && inst.addr == INSTANCE.addr {
                USART1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal USART1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        USART1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to USART1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const USART1: *const RegisterBlock = 0x40013800 as *const _;

/// Access functions for the USART2 peripheral instance
pub mod USART2 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40004400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in USART2
    pub const reset: ResetValues = ResetValues {
        CR1_FIFO: 0x00000000,
        CR2: 0x00000000,
        CR3: 0x00000000,
        BRR: 0x00000000,
        GTPR: 0x00000000,
        RTOR: 0x00000000,
        RQR: 0x00000000,
        ISR_FIFO: 0x000000C0,
        ICR: 0x00000000,
        RDR: 0x00000000,
        TDR: 0x00000000,
        PRESC: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut USART2_TAKEN: bool = false;

    /// Safe access to USART2
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
            if USART2_TAKEN {
                None
            } else {
                USART2_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to USART2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if USART2_TAKEN && inst.addr == INSTANCE.addr {
                USART2_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal USART2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        USART2_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to USART2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const USART2: *const RegisterBlock = 0x40004400 as *const _;
