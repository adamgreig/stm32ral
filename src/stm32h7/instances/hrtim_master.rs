#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! High Resolution Timer: Master Timers
//!
//! Used by: stm32h743, stm32h743v, stm32h747cm4, stm32h747cm7, stm32h753, stm32h753v, stm32h7b3

#[cfg(not(feature = "nosync"))]
pub use crate::stm32h7::peripherals::hrtim_master::Instance;
pub use crate::stm32h7::peripherals::hrtim_master::{RegisterBlock, ResetValues};
pub use crate::stm32h7::peripherals::hrtim_master::{
    MCMP1R, MCMP2R, MCMP3R, MCMP4R, MCNTR, MCR, MDIER4, MICR, MISR, MPER, MREP,
};

/// Access functions for the HRTIM_Master peripheral instance
pub mod HRTIM_Master {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40017400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in HRTIM_Master
    pub const reset: ResetValues = ResetValues {
        MCR: 0x00000000,
        MISR: 0x00000000,
        MICR: 0x00000000,
        MDIER4: 0x00000000,
        MCNTR: 0x00000000,
        MPER: 0x0000FFFF,
        MREP: 0x00000000,
        MCMP1R: 0x00000000,
        MCMP2R: 0x00000000,
        MCMP3R: 0x00000000,
        MCMP4R: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut HRTIM_Master_TAKEN: bool = false;

    /// Safe access to HRTIM_Master
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
            if HRTIM_Master_TAKEN {
                None
            } else {
                HRTIM_Master_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to HRTIM_Master
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if HRTIM_Master_TAKEN && inst.addr == INSTANCE.addr {
                HRTIM_Master_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal HRTIM_Master
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        HRTIM_Master_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to HRTIM_Master
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const HRTIM_Master: *const RegisterBlock = 0x40017400 as *const _;
