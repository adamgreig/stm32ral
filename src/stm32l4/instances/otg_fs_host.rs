#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! USB on the go full speed
//!
//! Used by: stm32l4r5, stm32l4r9

#[cfg(not(feature = "nosync"))]
pub use crate::stm32l4::peripherals::otg_fs_host::Instance;
pub use crate::stm32l4::peripherals::otg_fs_host::{RegisterBlock, ResetValues};
pub use crate::stm32l4::peripherals::otg_fs_host::{
    HAINT, HAINTMSK, HCCHAR0, HCCHAR1, HCCHAR2, HCCHAR3, HCCHAR4, HCCHAR5, HCCHAR6, HCCHAR7, HCFG,
    HCINT0, HCINT1, HCINT2, HCINT3, HCINT4, HCINT5, HCINT6, HCINT7, HCINTMSK0, HCINTMSK1,
    HCINTMSK2, HCINTMSK3, HCINTMSK4, HCINTMSK5, HCINTMSK6, HCINTMSK7, HCTSIZ0, HCTSIZ1, HCTSIZ2,
    HCTSIZ3, HCTSIZ4, HCTSIZ5, HCTSIZ6, HCTSIZ7, HFIR, HFNUM, HPRT, HPTXSTS,
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
        HCINT0: 0x00000000,
        HCINT1: 0x00000000,
        HCINT2: 0x00000000,
        HCINT3: 0x00000000,
        HCINT4: 0x00000000,
        HCINT5: 0x00000000,
        HCINT6: 0x00000000,
        HCINT7: 0x00000000,
        HCINTMSK0: 0x00000000,
        HCINTMSK1: 0x00000000,
        HCINTMSK2: 0x00000000,
        HCINTMSK3: 0x00000000,
        HCINTMSK4: 0x00000000,
        HCINTMSK5: 0x00000000,
        HCINTMSK6: 0x00000000,
        HCINTMSK7: 0x00000000,
        HCTSIZ0: 0x00000000,
        HCTSIZ1: 0x00000000,
        HCTSIZ2: 0x00000000,
        HCTSIZ3: 0x00000000,
        HCTSIZ4: 0x00000000,
        HCTSIZ5: 0x00000000,
        HCTSIZ6: 0x00000000,
        HCTSIZ7: 0x00000000,
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
