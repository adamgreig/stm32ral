#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! USB on the go full speed

#[cfg(not(feature = "nosync"))]
pub use crate::stm32f1::peripherals::otg_fs_pwrclk::Instance;
pub use crate::stm32f1::peripherals::otg_fs_pwrclk::FS_PCGCCTL;
pub use crate::stm32f1::peripherals::otg_fs_pwrclk::{RegisterBlock, ResetValues};

/// Access functions for the USB_OTG_PWRCLK peripheral instance
pub mod USB_OTG_PWRCLK {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50000e00,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in USB_OTG_PWRCLK
    pub const reset: ResetValues = ResetValues {
        FS_PCGCCTL: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut USB_OTG_PWRCLK_TAKEN: bool = false;

    /// Safe access to USB_OTG_PWRCLK
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
            if USB_OTG_PWRCLK_TAKEN {
                None
            } else {
                USB_OTG_PWRCLK_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to USB_OTG_PWRCLK
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if USB_OTG_PWRCLK_TAKEN && inst.addr == INSTANCE.addr {
                USB_OTG_PWRCLK_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal USB_OTG_PWRCLK
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        USB_OTG_PWRCLK_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to USB_OTG_PWRCLK
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const USB_OTG_PWRCLK: *const RegisterBlock = 0x50000e00 as *const _;
