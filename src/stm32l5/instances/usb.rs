#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Universal serial bus full-speed device interface
//!
//! Used by: stm32l552, stm32l562

#[cfg(not(feature = "nosync"))]
pub use crate::stm32l5::peripherals::usb::Instance;
pub use crate::stm32l5::peripherals::usb::{RegisterBlock, ResetValues};
pub use crate::stm32l5::peripherals::usb::{
    ADDR0_RX, ADDR1_RX, ADDR2_RX, ADDR3_RX, ADDR4_RX, ADDR5_RX, ADDR6_RX, ADDR7_RX, BCDR, BTABLE,
    CNTR, COUNT0_RX, COUNT0_TX, COUNT1_RX, COUNT1_TX, COUNT2_RX, COUNT2_TX, COUNT3_RX, COUNT3_TX,
    COUNT4_RX, COUNT4_TX, COUNT5_RX, COUNT5_TX, COUNT6_RX, COUNT6_TX, COUNT7_RX, COUNT7_TX, DADDR,
    EP0R, EP1R, EP2R, EP3R, EP4R, EP5R, EP6R, EP7R, FNR, ISTR, LPMCSR,
};

/// Access functions for the SEC_USB peripheral instance
pub mod SEC_USB {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x5000d400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SEC_USB
    pub const reset: ResetValues = ResetValues {
        EP0R: 0x00000000,
        EP1R: 0x00000000,
        EP2R: 0x00000000,
        EP3R: 0x00000000,
        EP4R: 0x00000000,
        EP5R: 0x00000000,
        EP6R: 0x00000000,
        EP7R: 0x00000000,
        CNTR: 0x00000003,
        ISTR: 0x00000000,
        FNR: 0x00000000,
        DADDR: 0x00000000,
        BTABLE: 0x00000000,
        LPMCSR: 0x00000000,
        BCDR: 0x00000000,
        COUNT0_TX: 0x00000000,
        COUNT1_TX: 0x00000000,
        COUNT2_TX: 0x00000000,
        COUNT3_TX: 0x00000000,
        COUNT4_TX: 0x00000000,
        COUNT5_TX: 0x00000000,
        COUNT6_TX: 0x00000000,
        COUNT7_TX: 0x00000000,
        ADDR0_RX: 0x00000000,
        ADDR1_RX: 0x00000000,
        ADDR2_RX: 0x00000000,
        ADDR3_RX: 0x00000000,
        ADDR4_RX: 0x00000000,
        ADDR5_RX: 0x00000000,
        ADDR6_RX: 0x00000000,
        ADDR7_RX: 0x00000000,
        COUNT0_RX: 0x00000000,
        COUNT1_RX: 0x00000000,
        COUNT2_RX: 0x00000000,
        COUNT3_RX: 0x00000000,
        COUNT4_RX: 0x00000000,
        COUNT5_RX: 0x00000000,
        COUNT6_RX: 0x00000000,
        COUNT7_RX: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SEC_USB_TAKEN: bool = false;

    /// Safe access to SEC_USB
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
            if SEC_USB_TAKEN {
                None
            } else {
                SEC_USB_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SEC_USB
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SEC_USB_TAKEN && inst.addr == INSTANCE.addr {
                SEC_USB_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SEC_USB
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SEC_USB_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SEC_USB
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SEC_USB: *const RegisterBlock = 0x5000d400 as *const _;

/// Access functions for the USB peripheral instance
pub mod USB {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x4000d400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in USB
    pub const reset: ResetValues = ResetValues {
        EP0R: 0x00000000,
        EP1R: 0x00000000,
        EP2R: 0x00000000,
        EP3R: 0x00000000,
        EP4R: 0x00000000,
        EP5R: 0x00000000,
        EP6R: 0x00000000,
        EP7R: 0x00000000,
        CNTR: 0x00000003,
        ISTR: 0x00000000,
        FNR: 0x00000000,
        DADDR: 0x00000000,
        BTABLE: 0x00000000,
        LPMCSR: 0x00000000,
        BCDR: 0x00000000,
        COUNT0_TX: 0x00000000,
        COUNT1_TX: 0x00000000,
        COUNT2_TX: 0x00000000,
        COUNT3_TX: 0x00000000,
        COUNT4_TX: 0x00000000,
        COUNT5_TX: 0x00000000,
        COUNT6_TX: 0x00000000,
        COUNT7_TX: 0x00000000,
        ADDR0_RX: 0x00000000,
        ADDR1_RX: 0x00000000,
        ADDR2_RX: 0x00000000,
        ADDR3_RX: 0x00000000,
        ADDR4_RX: 0x00000000,
        ADDR5_RX: 0x00000000,
        ADDR6_RX: 0x00000000,
        ADDR7_RX: 0x00000000,
        COUNT0_RX: 0x00000000,
        COUNT1_RX: 0x00000000,
        COUNT2_RX: 0x00000000,
        COUNT3_RX: 0x00000000,
        COUNT4_RX: 0x00000000,
        COUNT5_RX: 0x00000000,
        COUNT6_RX: 0x00000000,
        COUNT7_RX: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut USB_TAKEN: bool = false;

    /// Safe access to USB
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
            if USB_TAKEN {
                None
            } else {
                USB_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to USB
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if USB_TAKEN && inst.addr == INSTANCE.addr {
                USB_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal USB
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        USB_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to USB
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const USB: *const RegisterBlock = 0x4000d400 as *const _;
