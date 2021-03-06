#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! System configuration controller
//!
//! Used by: stm32g071, stm32g07x, stm32g081

#[cfg(not(feature = "nosync"))]
pub use crate::stm32g0::peripherals::syscfg_vrefbuf::Instance;
pub use crate::stm32g0::peripherals::syscfg_vrefbuf::{RegisterBlock, ResetValues};
pub use crate::stm32g0::peripherals::syscfg_vrefbuf::{
    CFGR1, CFGR2, ITLINE0, ITLINE1, ITLINE10, ITLINE11, ITLINE12, ITLINE13, ITLINE14, ITLINE15,
    ITLINE16, ITLINE17, ITLINE18, ITLINE19, ITLINE2, ITLINE20, ITLINE21, ITLINE22, ITLINE23,
    ITLINE24, ITLINE25, ITLINE26, ITLINE27, ITLINE28, ITLINE29, ITLINE3, ITLINE30, ITLINE31,
    ITLINE4, ITLINE5, ITLINE6, ITLINE7, ITLINE8, ITLINE9, VREFBUF_CCR, VREFBUF_CSR,
};

/// Access functions for the SYSCFG_VREFBUF peripheral instance
pub mod SYSCFG_VREFBUF {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40010000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SYSCFG_VREFBUF
    pub const reset: ResetValues = ResetValues {
        VREFBUF_CSR: 0x00000002,
        VREFBUF_CCR: 0x00000000,
        CFGR1: 0x00000000,
        CFGR2: 0x00000000,
        ITLINE0: 0x00000000,
        ITLINE1: 0x00000000,
        ITLINE2: 0x00000000,
        ITLINE3: 0x00000000,
        ITLINE4: 0x00000000,
        ITLINE5: 0x00000000,
        ITLINE6: 0x00000000,
        ITLINE7: 0x00000000,
        ITLINE8: 0x00000000,
        ITLINE9: 0x00000000,
        ITLINE10: 0x00000000,
        ITLINE11: 0x00000000,
        ITLINE12: 0x00000000,
        ITLINE13: 0x00000000,
        ITLINE14: 0x00000000,
        ITLINE15: 0x00000000,
        ITLINE16: 0x00000000,
        ITLINE17: 0x00000000,
        ITLINE18: 0x00000000,
        ITLINE19: 0x00000000,
        ITLINE20: 0x00000000,
        ITLINE21: 0x00000000,
        ITLINE22: 0x00000000,
        ITLINE23: 0x00000000,
        ITLINE24: 0x00000000,
        ITLINE25: 0x00000000,
        ITLINE26: 0x00000000,
        ITLINE27: 0x00000000,
        ITLINE28: 0x00000000,
        ITLINE29: 0x00000000,
        ITLINE30: 0x00000000,
        ITLINE31: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SYSCFG_VREFBUF_TAKEN: bool = false;

    /// Safe access to SYSCFG_VREFBUF
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
            if SYSCFG_VREFBUF_TAKEN {
                None
            } else {
                SYSCFG_VREFBUF_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SYSCFG_VREFBUF
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SYSCFG_VREFBUF_TAKEN && inst.addr == INSTANCE.addr {
                SYSCFG_VREFBUF_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SYSCFG_VREFBUF
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SYSCFG_VREFBUF_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SYSCFG_VREFBUF
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SYSCFG_VREFBUF: *const RegisterBlock = 0x40010000 as *const _;
