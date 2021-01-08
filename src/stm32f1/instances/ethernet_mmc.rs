#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Ethernet: MAC management counters
//!
//! Used by: stm32f101, stm32f103

#[cfg(not(feature = "nosync"))]
pub use crate::stm32f1::peripherals::ethernet_mmc::Instance;
pub use crate::stm32f1::peripherals::ethernet_mmc::{RegisterBlock, ResetValues};
pub use crate::stm32f1::peripherals::ethernet_mmc::{
    MMCCR, MMCRFAECR, MMCRFCECR, MMCRGUFCR, MMCRIMR, MMCRIR, MMCTGFCR, MMCTGFMSCCR, MMCTGFSCCR,
    MMCTIMR, MMCTIR,
};

/// Access functions for the Ethernet_MMC peripheral instance
pub mod Ethernet_MMC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40028100,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in Ethernet_MMC
    pub const reset: ResetValues = ResetValues {
        MMCCR: 0x00000000,
        MMCRIR: 0x00000000,
        MMCTIR: 0x00000000,
        MMCRIMR: 0x00000000,
        MMCTIMR: 0x00000000,
        MMCTGFSCCR: 0x00000000,
        MMCTGFMSCCR: 0x00000000,
        MMCTGFCR: 0x00000000,
        MMCRFCECR: 0x00000000,
        MMCRFAECR: 0x00000000,
        MMCRGUFCR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut Ethernet_MMC_TAKEN: bool = false;

    /// Safe access to Ethernet_MMC
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
            if Ethernet_MMC_TAKEN {
                None
            } else {
                Ethernet_MMC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to Ethernet_MMC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if Ethernet_MMC_TAKEN && inst.addr == INSTANCE.addr {
                Ethernet_MMC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal Ethernet_MMC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        Ethernet_MMC_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to Ethernet_MMC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const Ethernet_MMC: *const RegisterBlock = 0x40028100 as *const _;
