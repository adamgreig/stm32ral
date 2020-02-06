#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! WinWATCHDOG
//!
//! Used by: stm32g431, stm32g441, stm32g471, stm32g473, stm32g474, stm32g483, stm32g484

#[cfg(not(feature = "nosync"))]
pub use crate::stm32g4::peripherals::wwdg::Instance;
pub use crate::stm32g4::peripherals::wwdg::{RegisterBlock, ResetValues};
pub use crate::stm32g4::peripherals::wwdg::{KR, PR, RLR, SR, WINR};

/// Access functions for the WWDG peripheral instance
pub mod WWDG {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40002c00,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in WWDG
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
    static mut WWDG_TAKEN: bool = false;

    /// Safe access to WWDG
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
            if WWDG_TAKEN {
                None
            } else {
                WWDG_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to WWDG
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if WWDG_TAKEN && inst.addr == INSTANCE.addr {
                WWDG_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal WWDG
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        WWDG_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to WWDG
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const WWDG: *const RegisterBlock = 0x40002c00 as *const _;
