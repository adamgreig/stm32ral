#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! GICV
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::gicv::Instance;
pub use crate::stm32mp::peripherals::gicv::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::gicv::{
    GICV_ABPR, GICV_AEOIR, GICV_AHPPIR, GICV_AIAR, GICV_APR0, GICV_BPR, GICV_CTLR, GICV_DIR,
    GICV_EOIR, GICV_HPPIR, GICV_IAR, GICV_IIDR, GICV_PMR, GICV_RPR,
};

/// Access functions for the GICV peripheral instance
pub mod GICV {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0xa0026000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in GICV
    pub const reset: ResetValues = ResetValues {
        GICV_CTLR: 0x00000000,
        GICV_PMR: 0x00000000,
        GICV_BPR: 0x00000002,
        GICV_IAR: 0x000003FF,
        GICV_EOIR: 0x00000000,
        GICV_RPR: 0x000000FF,
        GICV_HPPIR: 0x000003FF,
        GICV_ABPR: 0x00000003,
        GICV_AIAR: 0x000003FF,
        GICV_AEOIR: 0x00000000,
        GICV_AHPPIR: 0x000003FF,
        GICV_APR0: 0x00000000,
        GICV_IIDR: 0x0102143B,
        GICV_DIR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut GICV_TAKEN: bool = false;

    /// Safe access to GICV
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
            if GICV_TAKEN {
                None
            } else {
                GICV_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to GICV
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if GICV_TAKEN && inst.addr == INSTANCE.addr {
                GICV_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal GICV
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        GICV_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to GICV
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const GICV: *const RegisterBlock = 0xa0026000 as *const _;
