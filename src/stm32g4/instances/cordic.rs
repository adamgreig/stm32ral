#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! CORDIC Co-processor
//!
//! Used by: stm32g431, stm32g441, stm32g471, stm32g473, stm32g474, stm32g483, stm32g484, stm32g491, stm32g4a1

#[cfg(not(feature = "nosync"))]
pub use crate::stm32g4::peripherals::cordic::Instance;
pub use crate::stm32g4::peripherals::cordic::{RegisterBlock, ResetValues};
pub use crate::stm32g4::peripherals::cordic::{CSR, RDATA, WDATA};

/// Access functions for the CORDIC peripheral instance
pub mod CORDIC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40020c00,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in CORDIC
    pub const reset: ResetValues = ResetValues {
        CSR: 0x00000000,
        WDATA: 0x00000000,
        RDATA: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut CORDIC_TAKEN: bool = false;

    /// Safe access to CORDIC
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
            if CORDIC_TAKEN {
                None
            } else {
                CORDIC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to CORDIC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if CORDIC_TAKEN && inst.addr == INSTANCE.addr {
                CORDIC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal CORDIC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        CORDIC_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to CORDIC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CORDIC: *const RegisterBlock = 0x40020c00 as *const _;
