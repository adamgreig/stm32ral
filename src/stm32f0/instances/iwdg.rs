#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Independent watchdog
//!
//! Used by: stm32f0x0, stm32f0x1, stm32f0x2, stm32f0x8

#[cfg(not(feature = "nosync"))]
pub use stm32f0::peripherals::iwdg::Instance;
pub use stm32f0::peripherals::iwdg::{RegisterBlock, ResetValues};
pub use stm32f0::peripherals::iwdg::{KR, PR, RLR, SR, WINR};

/// Access functions for the IWDG peripheral instance
pub mod IWDG {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40003000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in IWDG
    pub const reset: ResetValues = ResetValues {
        KR: 0x00000000,
        PR: 0x00000000,
        RLR: 0x00000FFF,
        SR: 0x00000000,
        WINR: 0x00000FFF,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut IWDG_TAKEN: bool = false;

    /// Safe access to IWDG
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
            if IWDG_TAKEN {
                None
            } else {
                IWDG_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to IWDG
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if IWDG_TAKEN && inst.addr == INSTANCE.addr {
                IWDG_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }
}

/// Raw pointer to IWDG
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const IWDG: *const RegisterBlock = 0x40003000 as *const _;
