#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Random number generator
//!
//! Used by: stm32f405, stm32f407, stm32f412, stm32f413, stm32f427, stm32f429, stm32f469

#[cfg(not(feature = "nosync"))]
pub use stm32f4::peripherals::rng::Instance;
pub use stm32f4::peripherals::rng::{RegisterBlock, ResetValues};
pub use stm32f4::peripherals::rng::{CR, DR, SR};

/// Access functions for the RNG peripheral instance
pub mod RNG {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50060800,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in RNG
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000000,
        SR: 0x00000000,
        DR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut RNG_TAKEN: bool = false;

    /// Safe access to RNG
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
            if RNG_TAKEN {
                None
            } else {
                RNG_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to RNG
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if RNG_TAKEN && inst.addr == INSTANCE.addr {
                RNG_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal RNG
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        RNG_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to RNG
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const RNG: *const RegisterBlock = 0x50060800 as *const _;
