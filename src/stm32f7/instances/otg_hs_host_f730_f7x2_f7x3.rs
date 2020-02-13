#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! USB on the go high speed
//!
//! Used by: stm32f730, stm32f7x2, stm32f7x3

#[cfg(not(feature = "nosync"))]
pub use crate::stm32f7::peripherals::otg_hs_host_v1::Instance;
pub use crate::stm32f7::peripherals::otg_hs_host_v1::{RegisterBlock, ResetValues};
pub use crate::stm32f7::peripherals::otg_hs_host_v1::{
    OTG_HS_HAINT, OTG_HS_HAINTMSK, OTG_HS_HCCHAR0, OTG_HS_HCCHAR1, OTG_HS_HCCHAR10,
    OTG_HS_HCCHAR11, OTG_HS_HCCHAR12, OTG_HS_HCCHAR13, OTG_HS_HCCHAR14, OTG_HS_HCCHAR15,
    OTG_HS_HCCHAR2, OTG_HS_HCCHAR3, OTG_HS_HCCHAR4, OTG_HS_HCCHAR5, OTG_HS_HCCHAR6, OTG_HS_HCCHAR7,
    OTG_HS_HCCHAR8, OTG_HS_HCCHAR9, OTG_HS_HCDMA0, OTG_HS_HCDMA1, OTG_HS_HCDMA10, OTG_HS_HCDMA11,
    OTG_HS_HCDMA12, OTG_HS_HCDMA13, OTG_HS_HCDMA14, OTG_HS_HCDMA15, OTG_HS_HCDMA2, OTG_HS_HCDMA3,
    OTG_HS_HCDMA4, OTG_HS_HCDMA5, OTG_HS_HCDMA6, OTG_HS_HCDMA7, OTG_HS_HCDMA8, OTG_HS_HCDMA9,
    OTG_HS_HCFG, OTG_HS_HCINT0, OTG_HS_HCINT1, OTG_HS_HCINT10, OTG_HS_HCINT11, OTG_HS_HCINT12,
    OTG_HS_HCINT13, OTG_HS_HCINT14, OTG_HS_HCINT15, OTG_HS_HCINT2, OTG_HS_HCINT3, OTG_HS_HCINT4,
    OTG_HS_HCINT5, OTG_HS_HCINT6, OTG_HS_HCINT7, OTG_HS_HCINT8, OTG_HS_HCINT9, OTG_HS_HCINTMSK0,
    OTG_HS_HCINTMSK1, OTG_HS_HCINTMSK10, OTG_HS_HCINTMSK11, OTG_HS_HCINTMSK12, OTG_HS_HCINTMSK13,
    OTG_HS_HCINTMSK14, OTG_HS_HCINTMSK15, OTG_HS_HCINTMSK2, OTG_HS_HCINTMSK3, OTG_HS_HCINTMSK4,
    OTG_HS_HCINTMSK5, OTG_HS_HCINTMSK6, OTG_HS_HCINTMSK7, OTG_HS_HCINTMSK8, OTG_HS_HCINTMSK9,
    OTG_HS_HCSPLT0, OTG_HS_HCSPLT1, OTG_HS_HCSPLT10, OTG_HS_HCSPLT11, OTG_HS_HCSPLT12,
    OTG_HS_HCSPLT13, OTG_HS_HCSPLT14, OTG_HS_HCSPLT15, OTG_HS_HCSPLT2, OTG_HS_HCSPLT3,
    OTG_HS_HCSPLT4, OTG_HS_HCSPLT5, OTG_HS_HCSPLT6, OTG_HS_HCSPLT7, OTG_HS_HCSPLT8, OTG_HS_HCSPLT9,
    OTG_HS_HCTSIZ0, OTG_HS_HCTSIZ1, OTG_HS_HCTSIZ10, OTG_HS_HCTSIZ11, OTG_HS_HCTSIZ12,
    OTG_HS_HCTSIZ13, OTG_HS_HCTSIZ14, OTG_HS_HCTSIZ15, OTG_HS_HCTSIZ2, OTG_HS_HCTSIZ3,
    OTG_HS_HCTSIZ4, OTG_HS_HCTSIZ5, OTG_HS_HCTSIZ6, OTG_HS_HCTSIZ7, OTG_HS_HCTSIZ8, OTG_HS_HCTSIZ9,
    OTG_HS_HFIR, OTG_HS_HFNUM, OTG_HS_HPRT, OTG_HS_HPTXSTS,
};

/// Access functions for the OTG_HS_HOST peripheral instance
pub mod OTG_HS_HOST {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40040400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in OTG_HS_HOST
    pub const reset: ResetValues = ResetValues {
        OTG_HS_HCFG: 0x00000000,
        OTG_HS_HFIR: 0x0000EA60,
        OTG_HS_HFNUM: 0x00003FFF,
        OTG_HS_HPTXSTS: 0x00080100,
        OTG_HS_HAINT: 0x00000000,
        OTG_HS_HAINTMSK: 0x00000000,
        OTG_HS_HPRT: 0x00000000,
        OTG_HS_HCCHAR0: 0x00000000,
        OTG_HS_HCCHAR1: 0x00000000,
        OTG_HS_HCCHAR2: 0x00000000,
        OTG_HS_HCCHAR3: 0x00000000,
        OTG_HS_HCCHAR4: 0x00000000,
        OTG_HS_HCCHAR5: 0x00000000,
        OTG_HS_HCCHAR6: 0x00000000,
        OTG_HS_HCCHAR7: 0x00000000,
        OTG_HS_HCCHAR8: 0x00000000,
        OTG_HS_HCCHAR9: 0x00000000,
        OTG_HS_HCCHAR10: 0x00000000,
        OTG_HS_HCCHAR11: 0x00000000,
        OTG_HS_HCSPLT0: 0x00000000,
        OTG_HS_HCSPLT1: 0x00000000,
        OTG_HS_HCSPLT2: 0x00000000,
        OTG_HS_HCSPLT3: 0x00000000,
        OTG_HS_HCSPLT4: 0x00000000,
        OTG_HS_HCSPLT5: 0x00000000,
        OTG_HS_HCSPLT6: 0x00000000,
        OTG_HS_HCSPLT7: 0x00000000,
        OTG_HS_HCSPLT8: 0x00000000,
        OTG_HS_HCSPLT9: 0x00000000,
        OTG_HS_HCSPLT10: 0x00000000,
        OTG_HS_HCSPLT11: 0x00000000,
        OTG_HS_HCINT0: 0x00000000,
        OTG_HS_HCINT1: 0x00000000,
        OTG_HS_HCINT2: 0x00000000,
        OTG_HS_HCINT3: 0x00000000,
        OTG_HS_HCINT4: 0x00000000,
        OTG_HS_HCINT5: 0x00000000,
        OTG_HS_HCINT6: 0x00000000,
        OTG_HS_HCINT7: 0x00000000,
        OTG_HS_HCINT8: 0x00000000,
        OTG_HS_HCINT9: 0x00000000,
        OTG_HS_HCINT10: 0x00000000,
        OTG_HS_HCINT11: 0x00000000,
        OTG_HS_HCINTMSK0: 0x00000000,
        OTG_HS_HCINTMSK1: 0x00000000,
        OTG_HS_HCINTMSK2: 0x00000000,
        OTG_HS_HCINTMSK3: 0x00000000,
        OTG_HS_HCINTMSK4: 0x00000000,
        OTG_HS_HCINTMSK5: 0x00000000,
        OTG_HS_HCINTMSK6: 0x00000000,
        OTG_HS_HCINTMSK7: 0x00000000,
        OTG_HS_HCINTMSK8: 0x00000000,
        OTG_HS_HCINTMSK9: 0x00000000,
        OTG_HS_HCINTMSK10: 0x00000000,
        OTG_HS_HCINTMSK11: 0x00000000,
        OTG_HS_HCTSIZ0: 0x00000000,
        OTG_HS_HCTSIZ1: 0x00000000,
        OTG_HS_HCTSIZ2: 0x00000000,
        OTG_HS_HCTSIZ3: 0x00000000,
        OTG_HS_HCTSIZ4: 0x00000000,
        OTG_HS_HCTSIZ5: 0x00000000,
        OTG_HS_HCTSIZ6: 0x00000000,
        OTG_HS_HCTSIZ7: 0x00000000,
        OTG_HS_HCTSIZ8: 0x00000000,
        OTG_HS_HCTSIZ9: 0x00000000,
        OTG_HS_HCTSIZ10: 0x00000000,
        OTG_HS_HCTSIZ11: 0x00000000,
        OTG_HS_HCDMA0: 0x00000000,
        OTG_HS_HCDMA1: 0x00000000,
        OTG_HS_HCDMA2: 0x00000000,
        OTG_HS_HCDMA3: 0x00000000,
        OTG_HS_HCDMA4: 0x00000000,
        OTG_HS_HCDMA5: 0x00000000,
        OTG_HS_HCDMA6: 0x00000000,
        OTG_HS_HCDMA7: 0x00000000,
        OTG_HS_HCDMA8: 0x00000000,
        OTG_HS_HCDMA9: 0x00000000,
        OTG_HS_HCDMA10: 0x00000000,
        OTG_HS_HCDMA11: 0x00000000,
        OTG_HS_HCCHAR12: 0x00000000,
        OTG_HS_HCSPLT12: 0x00000000,
        OTG_HS_HCINT12: 0x00000000,
        OTG_HS_HCINTMSK12: 0x00000000,
        OTG_HS_HCTSIZ12: 0x00000000,
        OTG_HS_HCDMA12: 0x00000000,
        OTG_HS_HCCHAR13: 0x00000000,
        OTG_HS_HCSPLT13: 0x00000000,
        OTG_HS_HCINT13: 0x00000000,
        OTG_HS_HCINTMSK13: 0x00000000,
        OTG_HS_HCTSIZ13: 0x00000000,
        OTG_HS_HCDMA13: 0x00000000,
        OTG_HS_HCCHAR14: 0x00000000,
        OTG_HS_HCSPLT14: 0x00000000,
        OTG_HS_HCINT14: 0x00000000,
        OTG_HS_HCINTMSK14: 0x00000000,
        OTG_HS_HCTSIZ14: 0x00000000,
        OTG_HS_HCDMA14: 0x00000000,
        OTG_HS_HCCHAR15: 0x00000000,
        OTG_HS_HCSPLT15: 0x00000000,
        OTG_HS_HCINT15: 0x00000000,
        OTG_HS_HCINTMSK15: 0x00000000,
        OTG_HS_HCTSIZ15: 0x00000000,
        OTG_HS_HCDMA15: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut OTG_HS_HOST_TAKEN: bool = false;

    /// Safe access to OTG_HS_HOST
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
            if OTG_HS_HOST_TAKEN {
                None
            } else {
                OTG_HS_HOST_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to OTG_HS_HOST
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if OTG_HS_HOST_TAKEN && inst.addr == INSTANCE.addr {
                OTG_HS_HOST_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal OTG_HS_HOST
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        OTG_HS_HOST_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to OTG_HS_HOST
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const OTG_HS_HOST: *const RegisterBlock = 0x40040400 as *const _;
