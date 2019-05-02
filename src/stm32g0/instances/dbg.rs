#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Debug support
//!
//! Used by: stm32g0x0, stm32g0x1

#[cfg(not(feature = "nosync"))]
pub use stm32g0::peripherals::dbg::Instance;
pub use stm32g0::peripherals::dbg::{RegisterBlock, ResetValues};
pub use stm32g0::peripherals::dbg::{APB_FZ1, APB_FZ2, CR, IDCODE};

/// Access functions for the DBG peripheral instance
pub mod DBG {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40015800,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DBG
    pub const reset: ResetValues = ResetValues {
        IDCODE: 0x00000000,
        CR: 0x00000000,
        APB_FZ1: 0x00000000,
        APB_FZ2: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut DBG_TAKEN: bool = false;

    /// Safe access to DBG
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
            if DBG_TAKEN {
                None
            } else {
                DBG_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to DBG
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if DBG_TAKEN && inst.addr == INSTANCE.addr {
                DBG_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal DBG
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        DBG_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to DBG
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DBG: *const RegisterBlock = 0x40015800 as *const _;
