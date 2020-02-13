#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! USB 1 on the go high speed
//!
//! Used by: stm32h743, stm32h743v, stm32h747cm4, stm32h747cm7, stm32h753, stm32h753v, stm32h757cm7

#[cfg(not(feature = "nosync"))]
pub use crate::stm32h7::peripherals::otg_hs_host::Instance;
pub use crate::stm32h7::peripherals::otg_hs_host::{RegisterBlock, ResetValues};
pub use crate::stm32h7::peripherals::otg_hs_host::{
    HAINT, HAINTMSK, HCCHAR0, HCCHAR1, HCCHAR10, HCCHAR11, HCCHAR12, HCCHAR13, HCCHAR14, HCCHAR15,
    HCCHAR2, HCCHAR3, HCCHAR4, HCCHAR5, HCCHAR6, HCCHAR7, HCCHAR8, HCCHAR9, HCDMA0, HCDMA1,
    HCDMA10, HCDMA11, HCDMA12, HCDMA13, HCDMA14, HCDMA15, HCDMA2, HCDMA3, HCDMA4, HCDMA5, HCDMA6,
    HCDMA7, HCDMA8, HCDMA9, HCFG, HCINT0, HCINT1, HCINT10, HCINT11, HCINT12, HCINT13, HCINT14,
    HCINT15, HCINT2, HCINT3, HCINT4, HCINT5, HCINT6, HCINT7, HCINT8, HCINT9, HCINTMSK0, HCINTMSK1,
    HCINTMSK10, HCINTMSK11, HCINTMSK12, HCINTMSK13, HCINTMSK14, HCINTMSK15, HCINTMSK2, HCINTMSK3,
    HCINTMSK4, HCINTMSK5, HCINTMSK6, HCINTMSK7, HCINTMSK8, HCINTMSK9, HCSPLT0, HCSPLT1, HCSPLT10,
    HCSPLT11, HCSPLT12, HCSPLT13, HCSPLT14, HCSPLT15, HCSPLT2, HCSPLT3, HCSPLT4, HCSPLT5, HCSPLT6,
    HCSPLT7, HCSPLT8, HCSPLT9, HCTSIZ0, HCTSIZ1, HCTSIZ10, HCTSIZ11, HCTSIZ12, HCTSIZ13, HCTSIZ14,
    HCTSIZ15, HCTSIZ2, HCTSIZ3, HCTSIZ4, HCTSIZ5, HCTSIZ6, HCTSIZ7, HCTSIZ8, HCTSIZ9, HFIR, HFNUM,
    HPRT, HPTXSTS,
};

/// Access functions for the OTG1_HS_HOST peripheral instance
pub mod OTG1_HS_HOST {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40040400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in OTG1_HS_HOST
    pub const reset: ResetValues = ResetValues {
        HCFG: 0x00000000,
        HFIR: 0x0000EA60,
        HFNUM: 0x00003FFF,
        HPTXSTS: 0x00080100,
        HAINT: 0x00000000,
        HAINTMSK: 0x00000000,
        HPRT: 0x00000000,
        HCCHAR0: 0x00000000,
        HCCHAR1: 0x00000000,
        HCCHAR2: 0x00000000,
        HCCHAR3: 0x00000000,
        HCCHAR4: 0x00000000,
        HCCHAR5: 0x00000000,
        HCCHAR6: 0x00000000,
        HCCHAR7: 0x00000000,
        HCCHAR8: 0x00000000,
        HCCHAR9: 0x00000000,
        HCCHAR10: 0x00000000,
        HCCHAR11: 0x00000000,
        HCSPLT0: 0x00000000,
        HCSPLT1: 0x00000000,
        HCSPLT2: 0x00000000,
        HCSPLT3: 0x00000000,
        HCSPLT4: 0x00000000,
        HCSPLT5: 0x00000000,
        HCSPLT6: 0x00000000,
        HCSPLT7: 0x00000000,
        HCSPLT8: 0x00000000,
        HCSPLT9: 0x00000000,
        HCSPLT10: 0x00000000,
        HCSPLT11: 0x00000000,
        HCINT0: 0x00000000,
        HCINT1: 0x00000000,
        HCINT2: 0x00000000,
        HCINT3: 0x00000000,
        HCINT4: 0x00000000,
        HCINT5: 0x00000000,
        HCINT6: 0x00000000,
        HCINT7: 0x00000000,
        HCINT8: 0x00000000,
        HCINT9: 0x00000000,
        HCINT10: 0x00000000,
        HCINT11: 0x00000000,
        HCINTMSK0: 0x00000000,
        HCINTMSK1: 0x00000000,
        HCINTMSK2: 0x00000000,
        HCINTMSK3: 0x00000000,
        HCINTMSK4: 0x00000000,
        HCINTMSK5: 0x00000000,
        HCINTMSK6: 0x00000000,
        HCINTMSK7: 0x00000000,
        HCINTMSK8: 0x00000000,
        HCINTMSK9: 0x00000000,
        HCINTMSK10: 0x00000000,
        HCINTMSK11: 0x00000000,
        HCTSIZ0: 0x00000000,
        HCTSIZ1: 0x00000000,
        HCTSIZ2: 0x00000000,
        HCTSIZ3: 0x00000000,
        HCTSIZ4: 0x00000000,
        HCTSIZ5: 0x00000000,
        HCTSIZ6: 0x00000000,
        HCTSIZ7: 0x00000000,
        HCTSIZ8: 0x00000000,
        HCTSIZ9: 0x00000000,
        HCTSIZ10: 0x00000000,
        HCTSIZ11: 0x00000000,
        HCDMA0: 0x00000000,
        HCDMA1: 0x00000000,
        HCDMA2: 0x00000000,
        HCDMA3: 0x00000000,
        HCDMA4: 0x00000000,
        HCDMA5: 0x00000000,
        HCDMA6: 0x00000000,
        HCDMA7: 0x00000000,
        HCDMA8: 0x00000000,
        HCDMA9: 0x00000000,
        HCDMA10: 0x00000000,
        HCDMA11: 0x00000000,
        HCCHAR12: 0x00000000,
        HCSPLT12: 0x00000000,
        HCINT12: 0x00000000,
        HCINTMSK12: 0x00000000,
        HCTSIZ12: 0x00000000,
        HCDMA12: 0x00000000,
        HCCHAR13: 0x00000000,
        HCSPLT13: 0x00000000,
        HCINT13: 0x00000000,
        HCINTMSK13: 0x00000000,
        HCTSIZ13: 0x00000000,
        HCDMA13: 0x00000000,
        HCCHAR14: 0x00000000,
        HCSPLT14: 0x00000000,
        HCINT14: 0x00000000,
        HCINTMSK14: 0x00000000,
        HCTSIZ14: 0x00000000,
        HCDMA14: 0x00000000,
        HCCHAR15: 0x00000000,
        HCSPLT15: 0x00000000,
        HCINT15: 0x00000000,
        HCINTMSK15: 0x00000000,
        HCTSIZ15: 0x00000000,
        HCDMA15: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut OTG1_HS_HOST_TAKEN: bool = false;

    /// Safe access to OTG1_HS_HOST
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
            if OTG1_HS_HOST_TAKEN {
                None
            } else {
                OTG1_HS_HOST_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to OTG1_HS_HOST
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if OTG1_HS_HOST_TAKEN && inst.addr == INSTANCE.addr {
                OTG1_HS_HOST_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal OTG1_HS_HOST
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        OTG1_HS_HOST_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to OTG1_HS_HOST
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const OTG1_HS_HOST: *const RegisterBlock = 0x40040400 as *const _;

/// Access functions for the OTG2_HS_HOST peripheral instance
pub mod OTG2_HS_HOST {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40080400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in OTG2_HS_HOST
    pub const reset: ResetValues = ResetValues {
        HCFG: 0x00000000,
        HFIR: 0x0000EA60,
        HFNUM: 0x00003FFF,
        HPTXSTS: 0x00080100,
        HAINT: 0x00000000,
        HAINTMSK: 0x00000000,
        HPRT: 0x00000000,
        HCCHAR0: 0x00000000,
        HCCHAR1: 0x00000000,
        HCCHAR2: 0x00000000,
        HCCHAR3: 0x00000000,
        HCCHAR4: 0x00000000,
        HCCHAR5: 0x00000000,
        HCCHAR6: 0x00000000,
        HCCHAR7: 0x00000000,
        HCCHAR8: 0x00000000,
        HCCHAR9: 0x00000000,
        HCCHAR10: 0x00000000,
        HCCHAR11: 0x00000000,
        HCSPLT0: 0x00000000,
        HCSPLT1: 0x00000000,
        HCSPLT2: 0x00000000,
        HCSPLT3: 0x00000000,
        HCSPLT4: 0x00000000,
        HCSPLT5: 0x00000000,
        HCSPLT6: 0x00000000,
        HCSPLT7: 0x00000000,
        HCSPLT8: 0x00000000,
        HCSPLT9: 0x00000000,
        HCSPLT10: 0x00000000,
        HCSPLT11: 0x00000000,
        HCINT0: 0x00000000,
        HCINT1: 0x00000000,
        HCINT2: 0x00000000,
        HCINT3: 0x00000000,
        HCINT4: 0x00000000,
        HCINT5: 0x00000000,
        HCINT6: 0x00000000,
        HCINT7: 0x00000000,
        HCINT8: 0x00000000,
        HCINT9: 0x00000000,
        HCINT10: 0x00000000,
        HCINT11: 0x00000000,
        HCINTMSK0: 0x00000000,
        HCINTMSK1: 0x00000000,
        HCINTMSK2: 0x00000000,
        HCINTMSK3: 0x00000000,
        HCINTMSK4: 0x00000000,
        HCINTMSK5: 0x00000000,
        HCINTMSK6: 0x00000000,
        HCINTMSK7: 0x00000000,
        HCINTMSK8: 0x00000000,
        HCINTMSK9: 0x00000000,
        HCINTMSK10: 0x00000000,
        HCINTMSK11: 0x00000000,
        HCTSIZ0: 0x00000000,
        HCTSIZ1: 0x00000000,
        HCTSIZ2: 0x00000000,
        HCTSIZ3: 0x00000000,
        HCTSIZ4: 0x00000000,
        HCTSIZ5: 0x00000000,
        HCTSIZ6: 0x00000000,
        HCTSIZ7: 0x00000000,
        HCTSIZ8: 0x00000000,
        HCTSIZ9: 0x00000000,
        HCTSIZ10: 0x00000000,
        HCTSIZ11: 0x00000000,
        HCDMA0: 0x00000000,
        HCDMA1: 0x00000000,
        HCDMA2: 0x00000000,
        HCDMA3: 0x00000000,
        HCDMA4: 0x00000000,
        HCDMA5: 0x00000000,
        HCDMA6: 0x00000000,
        HCDMA7: 0x00000000,
        HCDMA8: 0x00000000,
        HCDMA9: 0x00000000,
        HCDMA10: 0x00000000,
        HCDMA11: 0x00000000,
        HCCHAR12: 0x00000000,
        HCSPLT12: 0x00000000,
        HCINT12: 0x00000000,
        HCINTMSK12: 0x00000000,
        HCTSIZ12: 0x00000000,
        HCDMA12: 0x00000000,
        HCCHAR13: 0x00000000,
        HCSPLT13: 0x00000000,
        HCINT13: 0x00000000,
        HCINTMSK13: 0x00000000,
        HCTSIZ13: 0x00000000,
        HCDMA13: 0x00000000,
        HCCHAR14: 0x00000000,
        HCSPLT14: 0x00000000,
        HCINT14: 0x00000000,
        HCINTMSK14: 0x00000000,
        HCTSIZ14: 0x00000000,
        HCDMA14: 0x00000000,
        HCCHAR15: 0x00000000,
        HCSPLT15: 0x00000000,
        HCINT15: 0x00000000,
        HCINTMSK15: 0x00000000,
        HCTSIZ15: 0x00000000,
        HCDMA15: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut OTG2_HS_HOST_TAKEN: bool = false;

    /// Safe access to OTG2_HS_HOST
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
            if OTG2_HS_HOST_TAKEN {
                None
            } else {
                OTG2_HS_HOST_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to OTG2_HS_HOST
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if OTG2_HS_HOST_TAKEN && inst.addr == INSTANCE.addr {
                OTG2_HS_HOST_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal OTG2_HS_HOST
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        OTG2_HS_HOST_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to OTG2_HS_HOST
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const OTG2_HS_HOST: *const RegisterBlock = 0x40080400 as *const _;
