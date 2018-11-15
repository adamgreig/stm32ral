#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Routing interface
//!
//! Used by: stm32l100, stm32l151, stm32l162

#[cfg(not(feature = "nosync"))]
pub use stm32l1::peripherals::ri::Instance;
pub use stm32l1::peripherals::ri::{RegisterBlock, ResetValues};
pub use stm32l1::peripherals::ri::{ASCR1, ASCR2, HYSCR1, HYSCR2, HYSCR3, HYSCR4, ICR};

/// Access functions for the RI peripheral instance
pub mod RI {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40007c04,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in RI
    pub const reset: ResetValues = ResetValues {
        ICR: 0x00000000,
        ASCR1: 0x00000000,
        ASCR2: 0x00000000,
        HYSCR1: 0x00000000,
        HYSCR2: 0x00000000,
        HYSCR3: 0x00000000,
        HYSCR4: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[no_mangle]
    static mut RI_TAKEN: bool = false;

    /// Safe access to RI
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
            if RI_TAKEN {
                None
            } else {
                RI_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to RI
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if RI_TAKEN && inst.addr == INSTANCE.addr {
                RI_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }
}

/// Raw pointer to RI
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const RI: *const RegisterBlock = 0x40007c04 as *const _;
