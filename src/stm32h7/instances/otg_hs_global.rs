#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! USB 1 on the go high speed
//!
//! Used by: stm32h743, stm32h743v, stm32h747cm4, stm32h747cm7, stm32h753, stm32h753v, stm32h757cm7

#[cfg(not(feature = "nosync"))]
pub use crate::stm32h7::peripherals::otg_hs_global::Instance;
pub use crate::stm32h7::peripherals::otg_hs_global::{
    HNPTXFSIZ_Host, CID, DIEPTXF1, DIEPTXF2, DIEPTXF3, DIEPTXF4, DIEPTXF5, DIEPTXF6, DIEPTXF7,
    GAHBCFG, GCCFG, GINTMSK, GINTSTS, GLPMCFG, GNPTXSTS, GOTGCTL, GOTGINT, GRSTCTL, GRXFSIZ,
    GRXSTSP, GRXSTSR, GUSBCFG, HPTXFSIZ,
};
pub use crate::stm32h7::peripherals::otg_hs_global::{RegisterBlock, ResetValues};

/// Access functions for the OTG1_HS_GLOBAL peripheral instance
pub mod OTG1_HS_GLOBAL {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40040000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in OTG1_HS_GLOBAL
    pub const reset: ResetValues = ResetValues {
        GOTGCTL: 0x00000800,
        GOTGINT: 0x00000000,
        GAHBCFG: 0x00000000,
        GUSBCFG: 0x00000A00,
        GRSTCTL: 0x20000000,
        GINTSTS: 0x04000020,
        GINTMSK: 0x00000000,
        GRXSTSR: 0x00000000,
        GRXSTSP: 0x00000000,
        GRXFSIZ: 0x00000200,
        HNPTXFSIZ_Host: 0x00000200,
        GNPTXSTS: 0x00080200,
        GCCFG: 0x00000000,
        CID: 0x00001200,
        HPTXFSIZ: 0x02000600,
        DIEPTXF1: 0x02000400,
        DIEPTXF2: 0x02000400,
        DIEPTXF3: 0x02000400,
        DIEPTXF4: 0x02000400,
        DIEPTXF5: 0x02000400,
        DIEPTXF6: 0x02000400,
        DIEPTXF7: 0x02000400,
        GLPMCFG: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut OTG1_HS_GLOBAL_TAKEN: bool = false;

    /// Safe access to OTG1_HS_GLOBAL
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
            if OTG1_HS_GLOBAL_TAKEN {
                None
            } else {
                OTG1_HS_GLOBAL_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to OTG1_HS_GLOBAL
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if OTG1_HS_GLOBAL_TAKEN && inst.addr == INSTANCE.addr {
                OTG1_HS_GLOBAL_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal OTG1_HS_GLOBAL
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        OTG1_HS_GLOBAL_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to OTG1_HS_GLOBAL
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const OTG1_HS_GLOBAL: *const RegisterBlock = 0x40040000 as *const _;

/// Access functions for the OTG2_HS_GLOBAL peripheral instance
pub mod OTG2_HS_GLOBAL {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40080000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in OTG2_HS_GLOBAL
    pub const reset: ResetValues = ResetValues {
        GOTGCTL: 0x00000800,
        GOTGINT: 0x00000000,
        GAHBCFG: 0x00000000,
        GUSBCFG: 0x00000A00,
        GRSTCTL: 0x20000000,
        GINTSTS: 0x04000020,
        GINTMSK: 0x00000000,
        GRXSTSR: 0x00000000,
        GRXSTSP: 0x00000000,
        GRXFSIZ: 0x00000200,
        HNPTXFSIZ_Host: 0x00000200,
        GNPTXSTS: 0x00080200,
        GCCFG: 0x00000000,
        CID: 0x00001200,
        HPTXFSIZ: 0x02000600,
        DIEPTXF1: 0x02000400,
        DIEPTXF2: 0x02000400,
        DIEPTXF3: 0x02000400,
        DIEPTXF4: 0x02000400,
        DIEPTXF5: 0x02000400,
        DIEPTXF6: 0x02000400,
        DIEPTXF7: 0x02000400,
        GLPMCFG: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut OTG2_HS_GLOBAL_TAKEN: bool = false;

    /// Safe access to OTG2_HS_GLOBAL
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
            if OTG2_HS_GLOBAL_TAKEN {
                None
            } else {
                OTG2_HS_GLOBAL_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to OTG2_HS_GLOBAL
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if OTG2_HS_GLOBAL_TAKEN && inst.addr == INSTANCE.addr {
                OTG2_HS_GLOBAL_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal OTG2_HS_GLOBAL
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        OTG2_HS_GLOBAL_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to OTG2_HS_GLOBAL
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const OTG2_HS_GLOBAL: *const RegisterBlock = 0x40080000 as *const _;
