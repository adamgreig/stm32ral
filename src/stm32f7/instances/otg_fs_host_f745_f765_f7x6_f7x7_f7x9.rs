#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! USB on the go full speed
//!
//! Used by: stm32f745, stm32f765, stm32f7x6, stm32f7x7, stm32f7x9

#[cfg(not(feature = "nosync"))]
pub use crate::stm32f7::peripherals::otg_fs_host_v1::Instance;
pub use crate::stm32f7::peripherals::otg_fs_host_v1::{RegisterBlock, ResetValues};
pub use crate::stm32f7::peripherals::otg_fs_host_v1::{
    OTG_FS_HAINT, OTG_FS_HAINTMSK, OTG_FS_HCCHAR0, OTG_FS_HCCHAR1, OTG_FS_HCCHAR10,
    OTG_FS_HCCHAR11, OTG_FS_HCCHAR2, OTG_FS_HCCHAR3, OTG_FS_HCCHAR4, OTG_FS_HCCHAR5,
    OTG_FS_HCCHAR6, OTG_FS_HCCHAR7, OTG_FS_HCCHAR8, OTG_FS_HCCHAR9, OTG_FS_HCFG, OTG_FS_HCINT0,
    OTG_FS_HCINT1, OTG_FS_HCINT10, OTG_FS_HCINT11, OTG_FS_HCINT2, OTG_FS_HCINT3, OTG_FS_HCINT4,
    OTG_FS_HCINT5, OTG_FS_HCINT6, OTG_FS_HCINT7, OTG_FS_HCINT8, OTG_FS_HCINT9, OTG_FS_HCINTMSK0,
    OTG_FS_HCINTMSK1, OTG_FS_HCINTMSK10, OTG_FS_HCINTMSK11, OTG_FS_HCINTMSK2, OTG_FS_HCINTMSK3,
    OTG_FS_HCINTMSK4, OTG_FS_HCINTMSK5, OTG_FS_HCINTMSK6, OTG_FS_HCINTMSK7, OTG_FS_HCINTMSK8,
    OTG_FS_HCINTMSK9, OTG_FS_HCTSIZ0, OTG_FS_HCTSIZ1, OTG_FS_HCTSIZ10, OTG_FS_HCTSIZ11,
    OTG_FS_HCTSIZ2, OTG_FS_HCTSIZ3, OTG_FS_HCTSIZ4, OTG_FS_HCTSIZ5, OTG_FS_HCTSIZ6, OTG_FS_HCTSIZ7,
    OTG_FS_HCTSIZ8, OTG_FS_HCTSIZ9, OTG_FS_HFIR, OTG_FS_HFNUM, OTG_FS_HPRT, OTG_FS_HPTXSTS,
};

/// Access functions for the OTG_FS_HOST peripheral instance
pub mod OTG_FS_HOST {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50000400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in OTG_FS_HOST
    pub const reset: ResetValues = ResetValues {
        OTG_FS_HCFG: 0x00000000,
        OTG_FS_HFIR: 0x0000EA60,
        OTG_FS_HFNUM: 0x00003FFF,
        OTG_FS_HPTXSTS: 0x00080100,
        OTG_FS_HAINT: 0x00000000,
        OTG_FS_HAINTMSK: 0x00000000,
        OTG_FS_HPRT: 0x00000000,
        OTG_FS_HCCHAR0: 0x00000000,
        OTG_FS_HCCHAR1: 0x00000000,
        OTG_FS_HCCHAR2: 0x00000000,
        OTG_FS_HCCHAR3: 0x00000000,
        OTG_FS_HCCHAR4: 0x00000000,
        OTG_FS_HCCHAR5: 0x00000000,
        OTG_FS_HCCHAR6: 0x00000000,
        OTG_FS_HCCHAR7: 0x00000000,
        OTG_FS_HCINT0: 0x00000000,
        OTG_FS_HCINT1: 0x00000000,
        OTG_FS_HCINT2: 0x00000000,
        OTG_FS_HCINT3: 0x00000000,
        OTG_FS_HCINT4: 0x00000000,
        OTG_FS_HCINT5: 0x00000000,
        OTG_FS_HCINT6: 0x00000000,
        OTG_FS_HCINT7: 0x00000000,
        OTG_FS_HCINTMSK0: 0x00000000,
        OTG_FS_HCINTMSK1: 0x00000000,
        OTG_FS_HCINTMSK2: 0x00000000,
        OTG_FS_HCINTMSK3: 0x00000000,
        OTG_FS_HCINTMSK4: 0x00000000,
        OTG_FS_HCINTMSK5: 0x00000000,
        OTG_FS_HCINTMSK6: 0x00000000,
        OTG_FS_HCINTMSK7: 0x00000000,
        OTG_FS_HCTSIZ0: 0x00000000,
        OTG_FS_HCTSIZ1: 0x00000000,
        OTG_FS_HCTSIZ2: 0x00000000,
        OTG_FS_HCTSIZ3: 0x00000000,
        OTG_FS_HCTSIZ4: 0x00000000,
        OTG_FS_HCTSIZ5: 0x00000000,
        OTG_FS_HCTSIZ6: 0x00000000,
        OTG_FS_HCTSIZ7: 0x00000000,
        OTG_FS_HCCHAR8: 0x00000000,
        OTG_FS_HCINT8: 0x00000000,
        OTG_FS_HCINTMSK8: 0x00000000,
        OTG_FS_HCTSIZ8: 0x00000000,
        OTG_FS_HCCHAR9: 0x00000000,
        OTG_FS_HCINT9: 0x00000000,
        OTG_FS_HCINTMSK9: 0x00000000,
        OTG_FS_HCTSIZ9: 0x00000000,
        OTG_FS_HCCHAR10: 0x00000000,
        OTG_FS_HCINT10: 0x00000000,
        OTG_FS_HCINTMSK10: 0x00000000,
        OTG_FS_HCTSIZ10: 0x00000000,
        OTG_FS_HCCHAR11: 0x00000000,
        OTG_FS_HCINT11: 0x00000000,
        OTG_FS_HCINTMSK11: 0x00000000,
        OTG_FS_HCTSIZ11: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut OTG_FS_HOST_TAKEN: bool = false;

    /// Safe access to OTG_FS_HOST
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
            if OTG_FS_HOST_TAKEN {
                None
            } else {
                OTG_FS_HOST_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to OTG_FS_HOST
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if OTG_FS_HOST_TAKEN && inst.addr == INSTANCE.addr {
                OTG_FS_HOST_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal OTG_FS_HOST
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        OTG_FS_HOST_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to OTG_FS_HOST
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const OTG_FS_HOST: *const RegisterBlock = 0x50000400 as *const _;
