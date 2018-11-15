#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Ethernet: media access control (MAC)
//!
//! Used by: stm32f215, stm32f217

#[cfg(not(feature = "nosync"))]
pub use stm32f2::peripherals::ethernet_mac::Instance;
pub use stm32f2::peripherals::ethernet_mac::{RegisterBlock, ResetValues};
pub use stm32f2::peripherals::ethernet_mac::{
    MACA0HR, MACA0LR, MACA1HR, MACA1LR, MACA2HR, MACA2LR, MACA3HR, MACA3LR, MACCR, MACDBGR, MACFCR,
    MACFFR, MACHTHR, MACHTLR, MACIMR, MACMIIAR, MACMIIDR, MACPMTCSR, MACRWUFFR, MACSR, MACVLANTR,
};

/// Access functions for the Ethernet_MAC peripheral instance
pub mod Ethernet_MAC {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40028000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in Ethernet_MAC
    pub const reset: ResetValues = ResetValues {
        MACCR: 0x00008000,
        MACFFR: 0x00000000,
        MACHTHR: 0x00000000,
        MACHTLR: 0x00000000,
        MACMIIAR: 0x00000000,
        MACMIIDR: 0x00000000,
        MACFCR: 0x00000000,
        MACVLANTR: 0x00000000,
        MACPMTCSR: 0x00000000,
        MACDBGR: 0x00000000,
        MACSR: 0x00000000,
        MACIMR: 0x00000000,
        MACA0HR: 0x0010FFFF,
        MACA0LR: 0xFFFFFFFF,
        MACA1HR: 0x0000FFFF,
        MACA1LR: 0xFFFFFFFF,
        MACA2HR: 0x0000FFFF,
        MACA2LR: 0xFFFFFFFF,
        MACA3HR: 0x0000FFFF,
        MACA3LR: 0xFFFFFFFF,
        MACRWUFFR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut Ethernet_MAC_TAKEN: bool = false;

    /// Safe access to Ethernet_MAC
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
            if Ethernet_MAC_TAKEN {
                None
            } else {
                Ethernet_MAC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to Ethernet_MAC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if Ethernet_MAC_TAKEN && inst.addr == INSTANCE.addr {
                Ethernet_MAC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }
}

/// Raw pointer to Ethernet_MAC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const Ethernet_MAC: *const RegisterBlock = 0x40028000 as *const _;
