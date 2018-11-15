#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Ethernet: Precision time protocol
//!
//! Used by: stm32f101, stm32f103, stm32f107

#[cfg(not(feature = "nosync"))]
pub use stm32f1::peripherals::ethernet_ptp::Instance;
pub use stm32f1::peripherals::ethernet_ptp::{RegisterBlock, ResetValues};
pub use stm32f1::peripherals::ethernet_ptp::{
    PTPSSIR, PTPTSAR, PTPTSCR, PTPTSHR, PTPTSHUR, PTPTSLR, PTPTSLUR, PTPTTHR, PTPTTLR,
};

/// Access functions for the Ethernet_PTP peripheral instance
pub mod Ethernet_PTP {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40028700,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in Ethernet_PTP
    pub const reset: ResetValues = ResetValues {
        PTPTSCR: 0x00000000,
        PTPSSIR: 0x00000000,
        PTPTSHR: 0x00000000,
        PTPTSLR: 0x00000000,
        PTPTSHUR: 0x00000000,
        PTPTSLUR: 0x00000000,
        PTPTSAR: 0x00000000,
        PTPTTHR: 0x00000000,
        PTPTTLR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut Ethernet_PTP_TAKEN: bool = false;

    /// Safe access to Ethernet_PTP
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
            if Ethernet_PTP_TAKEN {
                None
            } else {
                Ethernet_PTP_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to Ethernet_PTP
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if Ethernet_PTP_TAKEN && inst.addr == INSTANCE.addr {
                Ethernet_PTP_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }
}

/// Raw pointer to Ethernet_PTP
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const Ethernet_PTP: *const RegisterBlock = 0x40028700 as *const _;
