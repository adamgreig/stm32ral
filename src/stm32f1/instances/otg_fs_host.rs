#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! USB on the go full speed
//!
//! Used by: stm32f101, stm32f102, stm32f103

#[cfg(not(feature = "nosync"))]
pub use stm32f1::peripherals::otg_fs_host::Instance;
pub use stm32f1::peripherals::otg_fs_host::{RegisterBlock, ResetValues};
pub use stm32f1::peripherals::otg_fs_host::{
    FS_HCCHAR0, FS_HCCHAR1, FS_HCCHAR2, FS_HCCHAR3, FS_HCCHAR4, FS_HCCHAR5, FS_HCCHAR6, FS_HCCHAR7,
    FS_HCFG, FS_HCINT0, FS_HCINT1, FS_HCINT2, FS_HCINT3, FS_HCINT4, FS_HCINT5, FS_HCINT6,
    FS_HCINT7, FS_HCINTMSK0, FS_HCINTMSK1, FS_HCINTMSK2, FS_HCINTMSK3, FS_HCINTMSK4, FS_HCINTMSK5,
    FS_HCINTMSK6, FS_HCINTMSK7, FS_HCTSIZ0, FS_HCTSIZ1, FS_HCTSIZ2, FS_HCTSIZ3, FS_HCTSIZ4,
    FS_HCTSIZ5, FS_HCTSIZ6, FS_HCTSIZ7, FS_HFNUM, FS_HPRT, FS_HPTXSTS, HAINT, HAINTMSK, HFIR,
};

/// Access functions for the OTG_FS_HOST peripheral instance
pub mod OTG_FS_HOST {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

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
        FS_HCFG: 0x00000000,
        HFIR: 0x0000EA60,
        FS_HFNUM: 0x00003FFF,
        FS_HPTXSTS: 0x00080100,
        HAINT: 0x00000000,
        HAINTMSK: 0x00000000,
        FS_HPRT: 0x00000000,
        FS_HCCHAR0: 0x00000000,
        FS_HCCHAR1: 0x00000000,
        FS_HCCHAR2: 0x00000000,
        FS_HCCHAR3: 0x00000000,
        FS_HCCHAR4: 0x00000000,
        FS_HCCHAR5: 0x00000000,
        FS_HCCHAR6: 0x00000000,
        FS_HCCHAR7: 0x00000000,
        FS_HCINT0: 0x00000000,
        FS_HCINT1: 0x00000000,
        FS_HCINT2: 0x00000000,
        FS_HCINT3: 0x00000000,
        FS_HCINT4: 0x00000000,
        FS_HCINT5: 0x00000000,
        FS_HCINT6: 0x00000000,
        FS_HCINT7: 0x00000000,
        FS_HCINTMSK0: 0x00000000,
        FS_HCINTMSK1: 0x00000000,
        FS_HCINTMSK2: 0x00000000,
        FS_HCINTMSK3: 0x00000000,
        FS_HCINTMSK4: 0x00000000,
        FS_HCINTMSK5: 0x00000000,
        FS_HCINTMSK6: 0x00000000,
        FS_HCINTMSK7: 0x00000000,
        FS_HCTSIZ0: 0x00000000,
        FS_HCTSIZ1: 0x00000000,
        FS_HCTSIZ2: 0x00000000,
        FS_HCTSIZ3: 0x00000000,
        FS_HCTSIZ4: 0x00000000,
        FS_HCTSIZ5: 0x00000000,
        FS_HCTSIZ6: 0x00000000,
        FS_HCTSIZ7: 0x00000000,
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
