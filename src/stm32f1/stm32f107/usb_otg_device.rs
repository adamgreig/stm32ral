#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! USB on the go full speed

#[cfg(not(feature = "nosync"))]
pub use stm32f1::peripherals::otg_fs_device::Instance;
pub use stm32f1::peripherals::otg_fs_device::{RegisterBlock, ResetValues};
pub use stm32f1::peripherals::otg_fs_device::{
    DIEPCTL1, DIEPCTL2, DIEPCTL3, DIEPEMPMSK, DIEPINT0, DIEPINT1, DIEPINT2, DIEPINT3, DIEPTSIZ0,
    DIEPTSIZ1, DIEPTSIZ2, DIEPTSIZ3, DOEPCTL0, DOEPCTL1, DOEPCTL2, DOEPCTL3, DOEPINT0, DOEPINT1,
    DOEPINT2, DOEPINT3, DOEPTSIZ0, DOEPTSIZ1, DOEPTSIZ2, DOEPTSIZ3, DTXFSTS0, DTXFSTS1, DTXFSTS2,
    DTXFSTS3, DVBUSDIS, DVBUSPULSE, FS_DAINT, FS_DAINTMSK, FS_DCFG, FS_DCTL, FS_DIEPCTL0,
    FS_DIEPMSK, FS_DOEPMSK, FS_DSTS,
};

/// Access functions for the USB_OTG_DEVICE peripheral instance
pub mod USB_OTG_DEVICE {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50000800,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in USB_OTG_DEVICE
    pub const reset: ResetValues = ResetValues {
        FS_DCFG: 0x02200000,
        FS_DCTL: 0x00000000,
        FS_DSTS: 0x00000010,
        FS_DIEPMSK: 0x00000000,
        FS_DOEPMSK: 0x00000000,
        FS_DAINT: 0x00000000,
        FS_DAINTMSK: 0x00000000,
        DVBUSDIS: 0x000017D7,
        DVBUSPULSE: 0x000005B8,
        DIEPEMPMSK: 0x00000000,
        FS_DIEPCTL0: 0x00000000,
        DIEPCTL1: 0x00000000,
        DIEPCTL2: 0x00000000,
        DIEPCTL3: 0x00000000,
        DOEPCTL0: 0x00008000,
        DOEPCTL1: 0x00000000,
        DOEPCTL2: 0x00000000,
        DOEPCTL3: 0x00000000,
        DIEPINT0: 0x00000080,
        DIEPINT1: 0x00000080,
        DIEPINT2: 0x00000080,
        DIEPINT3: 0x00000080,
        DOEPINT0: 0x00000080,
        DOEPINT1: 0x00000080,
        DOEPINT2: 0x00000080,
        DOEPINT3: 0x00000080,
        DIEPTSIZ0: 0x00000000,
        DOEPTSIZ0: 0x00000000,
        DIEPTSIZ1: 0x00000000,
        DIEPTSIZ2: 0x00000000,
        DIEPTSIZ3: 0x00000000,
        DTXFSTS0: 0x00000000,
        DTXFSTS1: 0x00000000,
        DTXFSTS2: 0x00000000,
        DTXFSTS3: 0x00000000,
        DOEPTSIZ1: 0x00000000,
        DOEPTSIZ2: 0x00000000,
        DOEPTSIZ3: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut USB_OTG_DEVICE_TAKEN: bool = false;

    /// Safe access to USB_OTG_DEVICE
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
            if USB_OTG_DEVICE_TAKEN {
                None
            } else {
                USB_OTG_DEVICE_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to USB_OTG_DEVICE
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if USB_OTG_DEVICE_TAKEN && inst.addr == INSTANCE.addr {
                USB_OTG_DEVICE_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal USB_OTG_DEVICE
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        USB_OTG_DEVICE_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to USB_OTG_DEVICE
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const USB_OTG_DEVICE: *const RegisterBlock = 0x50000800 as *const _;
