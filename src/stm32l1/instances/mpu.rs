#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Memory protection unit
//!
//! Used by: stm32l100, stm32l151, stm32l162

#[cfg(not(feature = "nosync"))]
pub use stm32l1::peripherals::mpu::Instance;
pub use stm32l1::peripherals::mpu::{RegisterBlock, ResetValues};
pub use stm32l1::peripherals::mpu::{MPU_CTRL, MPU_RASR, MPU_RBAR, MPU_RNR, MPU_TYPER};

/// Access functions for the MPU peripheral instance
pub mod MPU {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0xe000ed90,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in MPU
    pub const reset: ResetValues = ResetValues {
        MPU_TYPER: 0x00000800,
        MPU_CTRL: 0x00000000,
        MPU_RNR: 0x00000000,
        MPU_RBAR: 0x00000000,
        MPU_RASR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut MPU_TAKEN: bool = false;

    /// Safe access to MPU
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
            if MPU_TAKEN {
                None
            } else {
                MPU_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to MPU
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if MPU_TAKEN && inst.addr == INSTANCE.addr {
                MPU_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal MPU
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        MPU_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to MPU
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const MPU: *const RegisterBlock = 0xe000ed90 as *const _;
