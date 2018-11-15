#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Floating point unit CPACR
//!
//! Used by: stm32l4x1, stm32l4x2, stm32l4x3, stm32l4x5, stm32l4x6

#[cfg(not(feature = "nosync"))]
pub use stm32l4::peripherals::fpu_cpacr::Instance;
pub use stm32l4::peripherals::fpu_cpacr::CPACR;
pub use stm32l4::peripherals::fpu_cpacr::{RegisterBlock, ResetValues};

/// Access functions for the FPU_CPACR peripheral instance
pub mod FPU_CPACR {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0xe000ed88,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in FPU_CPACR
    pub const reset: ResetValues = ResetValues { CPACR: 0x00000000 };

    #[cfg(not(feature = "nosync"))]
    #[no_mangle]
    static mut FPU_CPACR_TAKEN: bool = false;

    /// Safe access to FPU_CPACR
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
            if FPU_CPACR_TAKEN {
                None
            } else {
                FPU_CPACR_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to FPU_CPACR
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if FPU_CPACR_TAKEN && inst.addr == INSTANCE.addr {
                FPU_CPACR_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }
}

/// Raw pointer to FPU_CPACR
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const FPU_CPACR: *const RegisterBlock = 0xe000ed88 as *const _;
