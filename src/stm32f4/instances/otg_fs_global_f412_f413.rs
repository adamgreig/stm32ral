#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! USB on the go full speed
//!
//! Used by: stm32f412, stm32f413

#[cfg(not(feature = "nosync"))]
pub use crate::stm32f4::peripherals::otg_fs_global_v2::Instance;
pub use crate::stm32f4::peripherals::otg_fs_global_v2::{RegisterBlock, ResetValues};
pub use crate::stm32f4::peripherals::otg_fs_global_v2::{
    CID, DIEPTXF1, DIEPTXF2, DIEPTXF3, GAHBCFG, GCCFG, GINTMSK, GINTSTS, GNPTXFSIZ, GNPTXSTS,
    GOTGCTL, GOTGINT, GRSTCTL, GRXFSIZ, GRXSTSR, GUSBCFG, HPTXFSIZ,
};

/// Access functions for the OTG_FS_GLOBAL peripheral instance
pub mod OTG_FS_GLOBAL {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50000000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in OTG_FS_GLOBAL
    pub const reset: ResetValues = ResetValues {
        GOTGCTL: 0x00000800,
        GOTGINT: 0x00000000,
        GAHBCFG: 0x00000000,
        GUSBCFG: 0x00000A00,
        GRSTCTL: 0x20000000,
        GINTSTS: 0x04000020,
        GINTMSK: 0x00000000,
        GRXSTSR: 0x00000000,
        GRXFSIZ: 0x00000200,
        GNPTXFSIZ: 0x00000200,
        GNPTXSTS: 0x00080200,
        GCCFG: 0x00000000,
        CID: 0x00001000,
        HPTXFSIZ: 0x02000600,
        DIEPTXF1: 0x02000400,
        DIEPTXF2: 0x02000400,
        DIEPTXF3: 0x02000400,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut OTG_FS_GLOBAL_TAKEN: bool = false;

    /// Safe access to OTG_FS_GLOBAL
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
            if OTG_FS_GLOBAL_TAKEN {
                None
            } else {
                OTG_FS_GLOBAL_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to OTG_FS_GLOBAL
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if OTG_FS_GLOBAL_TAKEN && inst.addr == INSTANCE.addr {
                OTG_FS_GLOBAL_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal OTG_FS_GLOBAL
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        OTG_FS_GLOBAL_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to OTG_FS_GLOBAL
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const OTG_FS_GLOBAL: *const RegisterBlock = 0x50000000 as *const _;
