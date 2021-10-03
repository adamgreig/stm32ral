#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! GICH
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::gich::Instance;
pub use crate::stm32mp::peripherals::gich::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::gich::{
    GICH_APR0, GICH_EISR0, GICH_ELSR0, GICH_HCR, GICH_LR0, GICH_LR1, GICH_LR2, GICH_LR3, GICH_MISR,
    GICH_VMCR, GICH_VTR,
};

/// Access functions for the GICH peripheral instance
pub mod GICH {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0xa0024000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in GICH
    pub const reset: ResetValues = ResetValues {
        GICH_HCR: 0x00000000,
        GICH_VTR: 0x90000003,
        GICH_VMCR: 0x004D0000,
        GICH_MISR: 0x00000000,
        GICH_EISR0: 0x00000000,
        GICH_ELSR0: 0x0000000F,
        GICH_APR0: 0x00000000,
        GICH_LR0: 0x00000000,
        GICH_LR1: 0x00000000,
        GICH_LR2: 0x00000000,
        GICH_LR3: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut GICH_TAKEN: bool = false;

    /// Safe access to GICH
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
            if GICH_TAKEN {
                None
            } else {
                GICH_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to GICH
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if GICH_TAKEN && inst.addr == INSTANCE.addr {
                GICH_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal GICH
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        GICH_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to GICH
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GICH: *const RegisterBlock = 0xa0024000 as *const _;
