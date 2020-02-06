#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! UCPD1
//!
//! Used by: stm32g431, stm32g441, stm32g471, stm32g473, stm32g474, stm32g483, stm32g484

#[cfg(not(feature = "nosync"))]
pub use crate::stm32g4::peripherals::ucpd1::Instance;
pub use crate::stm32g4::peripherals::ucpd1::{RegisterBlock, ResetValues};
pub use crate::stm32g4::peripherals::ucpd1::{
    CFG1, CFG2, CR, ICR, IMR, RXDR, RX_ORDEXT1, RX_ORDEXT2, RX_ORDSET, RX_PAYSZ, SR, TXDR,
    TX_ORDSET, TX_PAYSZ,
};

/// Access functions for the UCPD1 peripheral instance
pub mod UCPD1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x4000a000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in UCPD1
    pub const reset: ResetValues = ResetValues {
        CFG1: 0x00000000,
        CFG2: 0x00000000,
        CR: 0x00000000,
        IMR: 0x00000000,
        SR: 0x00000000,
        ICR: 0x00000000,
        TX_ORDSET: 0x00000000,
        TX_PAYSZ: 0x00000000,
        TXDR: 0x00000000,
        RX_ORDSET: 0x00000000,
        RX_PAYSZ: 0x00000000,
        RXDR: 0x00000000,
        RX_ORDEXT1: 0x00000000,
        RX_ORDEXT2: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut UCPD1_TAKEN: bool = false;

    /// Safe access to UCPD1
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
            if UCPD1_TAKEN {
                None
            } else {
                UCPD1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to UCPD1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if UCPD1_TAKEN && inst.addr == INSTANCE.addr {
                UCPD1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal UCPD1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        UCPD1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to UCPD1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const UCPD1: *const RegisterBlock = 0x4000a000 as *const _;
