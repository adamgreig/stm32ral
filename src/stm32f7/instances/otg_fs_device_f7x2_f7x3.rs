#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! USB on the go full speed
//!
//! Used by: stm32f7x2, stm32f7x3

#[cfg(not(feature = "nosync"))]
pub use crate::stm32f7::peripherals::otg_fs_device_v1::Instance;
pub use crate::stm32f7::peripherals::otg_fs_device_v1::{RegisterBlock, ResetValues};
pub use crate::stm32f7::peripherals::otg_fs_device_v1::{
    OTG_FS_DAINT, OTG_FS_DAINTMSK, OTG_FS_DCFG, OTG_FS_DCTL, OTG_FS_DIEPCTL0, OTG_FS_DIEPCTL1,
    OTG_FS_DIEPCTL2, OTG_FS_DIEPCTL3, OTG_FS_DIEPCTL4, OTG_FS_DIEPCTL5, OTG_FS_DIEPEMPMSK,
    OTG_FS_DIEPINT0, OTG_FS_DIEPINT1, OTG_FS_DIEPINT2, OTG_FS_DIEPINT3, OTG_FS_DIEPINT4,
    OTG_FS_DIEPINT5, OTG_FS_DIEPMSK, OTG_FS_DIEPTSIZ0, OTG_FS_DIEPTSIZ1, OTG_FS_DIEPTSIZ2,
    OTG_FS_DIEPTSIZ3, OTG_FS_DIEPTSIZ4, OTG_FS_DIEPTSIZ5, OTG_FS_DOEPCTL0, OTG_FS_DOEPCTL1,
    OTG_FS_DOEPCTL2, OTG_FS_DOEPCTL3, OTG_FS_DOEPCTL4, OTG_FS_DOEPCTL5, OTG_FS_DOEPINT0,
    OTG_FS_DOEPINT1, OTG_FS_DOEPINT2, OTG_FS_DOEPINT3, OTG_FS_DOEPINT4, OTG_FS_DOEPINT5,
    OTG_FS_DOEPMSK, OTG_FS_DOEPTSIZ0, OTG_FS_DOEPTSIZ1, OTG_FS_DOEPTSIZ2, OTG_FS_DOEPTSIZ3,
    OTG_FS_DOEPTSIZ4, OTG_FS_DOEPTSIZ5, OTG_FS_DSTS, OTG_FS_DTXFSTS0, OTG_FS_DTXFSTS1,
    OTG_FS_DTXFSTS2, OTG_FS_DTXFSTS3, OTG_FS_DTXFSTS4, OTG_FS_DTXFSTS5, OTG_FS_DVBUSDIS,
    OTG_FS_DVBUSPULSE,
};

/// Access functions for the OTG_FS_DEVICE peripheral instance
pub mod OTG_FS_DEVICE {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50000800,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in OTG_FS_DEVICE
    pub const reset: ResetValues = ResetValues {
        OTG_FS_DCFG: 0x02200000,
        OTG_FS_DCTL: 0x00000000,
        OTG_FS_DSTS: 0x00000010,
        OTG_FS_DIEPMSK: 0x00000000,
        OTG_FS_DOEPMSK: 0x00000000,
        OTG_FS_DAINT: 0x00000000,
        OTG_FS_DAINTMSK: 0x00000000,
        OTG_FS_DVBUSDIS: 0x000017D7,
        OTG_FS_DVBUSPULSE: 0x000005B8,
        OTG_FS_DIEPEMPMSK: 0x00000000,
        OTG_FS_DIEPCTL0: 0x00000000,
        OTG_FS_DIEPCTL1: 0x00000000,
        OTG_FS_DIEPCTL2: 0x00000000,
        OTG_FS_DIEPCTL3: 0x00000000,
        OTG_FS_DOEPCTL0: 0x00008000,
        OTG_FS_DOEPCTL1: 0x00000000,
        OTG_FS_DOEPCTL2: 0x00000000,
        OTG_FS_DOEPCTL3: 0x00000000,
        OTG_FS_DIEPINT0: 0x00000080,
        OTG_FS_DIEPINT1: 0x00000080,
        OTG_FS_DIEPINT2: 0x00000080,
        OTG_FS_DIEPINT3: 0x00000080,
        OTG_FS_DOEPINT0: 0x00000080,
        OTG_FS_DOEPINT1: 0x00000080,
        OTG_FS_DOEPINT2: 0x00000080,
        OTG_FS_DOEPINT3: 0x00000080,
        OTG_FS_DIEPTSIZ0: 0x00000000,
        OTG_FS_DOEPTSIZ0: 0x00000000,
        OTG_FS_DIEPTSIZ1: 0x00000000,
        OTG_FS_DIEPTSIZ2: 0x00000000,
        OTG_FS_DIEPTSIZ3: 0x00000000,
        OTG_FS_DTXFSTS0: 0x00000000,
        OTG_FS_DTXFSTS1: 0x00000000,
        OTG_FS_DTXFSTS2: 0x00000000,
        OTG_FS_DTXFSTS3: 0x00000000,
        OTG_FS_DOEPTSIZ1: 0x00000000,
        OTG_FS_DOEPTSIZ2: 0x00000000,
        OTG_FS_DOEPTSIZ3: 0x00000000,
        OTG_FS_DIEPCTL4: 0x00000000,
        OTG_FS_DIEPINT4: 0x00000000,
        OTG_FS_DIEPTSIZ4: 0x00000000,
        OTG_FS_DTXFSTS4: 0x00000000,
        OTG_FS_DIEPCTL5: 0x00000000,
        OTG_FS_DIEPINT5: 0x00000000,
        OTG_FS_DIEPTSIZ5: 0x00000000,
        OTG_FS_DTXFSTS5: 0x00000000,
        OTG_FS_DOEPCTL4: 0x00000000,
        OTG_FS_DOEPINT4: 0x00000000,
        OTG_FS_DOEPTSIZ4: 0x00000000,
        OTG_FS_DOEPCTL5: 0x00000000,
        OTG_FS_DOEPINT5: 0x00000000,
        OTG_FS_DOEPTSIZ5: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut OTG_FS_DEVICE_TAKEN: bool = false;

    /// Safe access to OTG_FS_DEVICE
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
            if OTG_FS_DEVICE_TAKEN {
                None
            } else {
                OTG_FS_DEVICE_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to OTG_FS_DEVICE
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if OTG_FS_DEVICE_TAKEN && inst.addr == INSTANCE.addr {
                OTG_FS_DEVICE_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal OTG_FS_DEVICE
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        OTG_FS_DEVICE_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to OTG_FS_DEVICE
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const OTG_FS_DEVICE: *const RegisterBlock = 0x50000800 as *const _;
