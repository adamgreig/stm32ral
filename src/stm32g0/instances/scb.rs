#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! System control block
//!
//! Used by: stm32g0x0, stm32g0x1

#[cfg(not(feature = "nosync"))]
pub use stm32g0::peripherals::scb::Instance;
pub use stm32g0::peripherals::scb::{RegisterBlock, ResetValues};
pub use stm32g0::peripherals::scb::{AIRCR, CCR, CPUID, ICSR, SCR, SHPR2, SHPR3, VTOR};

/// Access functions for the SCB peripheral instance
pub mod SCB {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0xe000ed00,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SCB
    pub const reset: ResetValues = ResetValues {
        CPUID: 0x410FC241,
        ICSR: 0x00000000,
        VTOR: 0x00000000,
        AIRCR: 0x00000000,
        SCR: 0x00000000,
        CCR: 0x00000000,
        SHPR2: 0x00000000,
        SHPR3: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SCB_TAKEN: bool = false;

    /// Safe access to SCB
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
            if SCB_TAKEN {
                None
            } else {
                SCB_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SCB
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SCB_TAKEN && inst.addr == INSTANCE.addr {
                SCB_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }
}

/// Raw pointer to SCB
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SCB: *const RegisterBlock = 0xe000ed00 as *const _;
