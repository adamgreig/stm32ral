#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! USB on the go high speed
//!
//! Used by: stm32f215, stm32f217

#[cfg(not(feature = "nosync"))]
pub use crate::stm32f2::peripherals::otg_hs_global::Instance;
pub use crate::stm32f2::peripherals::otg_hs_global::{RegisterBlock, ResetValues};
pub use crate::stm32f2::peripherals::otg_hs_global::{
    OTG_HS, OTG_HS_CID, OTG_HS_DIEPTXF1, OTG_HS_DIEPTXF2, OTG_HS_DIEPTXF3, OTG_HS_DIEPTXF4,
    OTG_HS_DIEPTXF5, OTG_HS_DIEPTXF6, OTG_HS_DIEPTXF7, OTG_HS_GAHBCFG, OTG_HS_GCCFG,
    OTG_HS_GINTMSK, OTG_HS_GINTSTS, OTG_HS_GNPTXSTS, OTG_HS_GOTGCTL, OTG_HS_GOTGINT,
    OTG_HS_GRSTCTL, OTG_HS_GRXFSIZ, OTG_HS_GRXSTSP, OTG_HS_GRXSTSR, OTG_HS_GUSBCFG,
    OTG_HS_HPTXFSIZ,
};

/// Access functions for the OTG_HS_GLOBAL peripheral instance
pub mod OTG_HS_GLOBAL {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40040000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in OTG_HS_GLOBAL
    pub const reset: ResetValues = ResetValues {
        OTG_HS_GOTGCTL: 0x00000800,
        OTG_HS_GOTGINT: 0x00000000,
        OTG_HS_GAHBCFG: 0x00000000,
        OTG_HS_GUSBCFG: 0x00000A00,
        OTG_HS_GRSTCTL: 0x20000000,
        OTG_HS_GINTSTS: 0x04000020,
        OTG_HS_GINTMSK: 0x00000000,
        OTG_HS_GRXSTSR: 0x00000000,
        OTG_HS_GRXSTSP: 0x00000000,
        OTG_HS_GRXFSIZ: 0x00000200,
        OTG_HS: 0x00000200,
        OTG_HS_GNPTXSTS: 0x00080200,
        OTG_HS_GCCFG: 0x00000000,
        OTG_HS_CID: 0x00001200,
        OTG_HS_HPTXFSIZ: 0x02000600,
        OTG_HS_DIEPTXF1: 0x02000400,
        OTG_HS_DIEPTXF2: 0x02000400,
        OTG_HS_DIEPTXF3: 0x02000400,
        OTG_HS_DIEPTXF4: 0x02000400,
        OTG_HS_DIEPTXF5: 0x02000400,
        OTG_HS_DIEPTXF6: 0x02000400,
        OTG_HS_DIEPTXF7: 0x02000400,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut OTG_HS_GLOBAL_TAKEN: bool = false;

    /// Safe access to OTG_HS_GLOBAL
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
            if OTG_HS_GLOBAL_TAKEN {
                None
            } else {
                OTG_HS_GLOBAL_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to OTG_HS_GLOBAL
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if OTG_HS_GLOBAL_TAKEN && inst.addr == INSTANCE.addr {
                OTG_HS_GLOBAL_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal OTG_HS_GLOBAL
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        OTG_HS_GLOBAL_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to OTG_HS_GLOBAL
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const OTG_HS_GLOBAL: *const RegisterBlock = 0x40040000 as *const _;
