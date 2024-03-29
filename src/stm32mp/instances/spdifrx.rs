#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! SPDIFRX
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::spdifrx::Instance;
pub use crate::stm32mp::peripherals::spdifrx::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::spdifrx::{
    SPDIFRX_CR, SPDIFRX_CSR, SPDIFRX_DIR, SPDIFRX_FMT0_DR, SPDIFRX_IFCR, SPDIFRX_IMR,
    SPDIFRX_IPIDR, SPDIFRX_SIDR, SPDIFRX_SR, SPDIFRX_VERR,
};

/// Access functions for the SPDIFRX peripheral instance
pub mod SPDIFRX {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x4000d000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SPDIFRX
    pub const reset: ResetValues = ResetValues {
        SPDIFRX_CR: 0x00000000,
        SPDIFRX_IMR: 0x00000000,
        SPDIFRX_SR: 0x00000000,
        SPDIFRX_IFCR: 0x00000000,
        SPDIFRX_FMT0_DR: 0x00000000,
        SPDIFRX_CSR: 0x00000000,
        SPDIFRX_DIR: 0x00000000,
        SPDIFRX_VERR: 0x00000012,
        SPDIFRX_IPIDR: 0x00130041,
        SPDIFRX_SIDR: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SPDIFRX_TAKEN: bool = false;

    /// Safe access to SPDIFRX
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
            if SPDIFRX_TAKEN {
                None
            } else {
                SPDIFRX_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SPDIFRX
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SPDIFRX_TAKEN && inst.addr == INSTANCE.addr {
                SPDIFRX_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SPDIFRX
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SPDIFRX_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SPDIFRX
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SPDIFRX: *const RegisterBlock = 0x4000d000 as *const _;
