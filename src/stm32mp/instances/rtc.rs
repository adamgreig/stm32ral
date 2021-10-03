#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! RTC
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::rtc::Instance;
pub use crate::stm32mp::peripherals::rtc::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::rtc::{
    RTC_ALRMAR, RTC_ALRMASSR, RTC_ALRMBR, RTC_ALRMBSSR, RTC_CALR, RTC_CFGR, RTC_CR, RTC_DR,
    RTC_HWCFGR, RTC_ICSR, RTC_IPIDR, RTC_MISR, RTC_PRER, RTC_SCR, RTC_SHIFTR, RTC_SIDR, RTC_SMCR,
    RTC_SMISR, RTC_SR, RTC_SSR, RTC_TR, RTC_TSDR, RTC_TSSSR, RTC_TSTR, RTC_VERR, RTC_WPR, RTC_WUTR,
};

/// Access functions for the RTC peripheral instance
pub mod RTC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x5c004000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in RTC
    pub const reset: ResetValues = ResetValues {
        RTC_TR: 0x00000000,
        RTC_DR: 0x00002101,
        RTC_SSR: 0x00000000,
        RTC_ICSR: 0x00000007,
        RTC_PRER: 0x007F00FF,
        RTC_WUTR: 0x0000FFFF,
        RTC_CR: 0x00000000,
        RTC_SMCR: 0x0000E00F,
        RTC_WPR: 0x00000000,
        RTC_CALR: 0x00000000,
        RTC_SHIFTR: 0x00000000,
        RTC_TSTR: 0x00000000,
        RTC_TSDR: 0x00000000,
        RTC_TSSSR: 0x00000000,
        RTC_ALRMAR: 0x00000000,
        RTC_ALRMASSR: 0x00000000,
        RTC_ALRMBR: 0x00000000,
        RTC_ALRMBSSR: 0x00000000,
        RTC_SR: 0x00000000,
        RTC_MISR: 0x00000000,
        RTC_SMISR: 0x00000000,
        RTC_SCR: 0x00000000,
        RTC_CFGR: 0x00000000,
        RTC_HWCFGR: 0x01031111,
        RTC_VERR: 0x00000010,
        RTC_IPIDR: 0x00120033,
        RTC_SIDR: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut RTC_TAKEN: bool = false;

    /// Safe access to RTC
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
            if RTC_TAKEN {
                None
            } else {
                RTC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to RTC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if RTC_TAKEN && inst.addr == INSTANCE.addr {
                RTC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal RTC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        RTC_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to RTC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const RTC: *const RegisterBlock = 0x5c004000 as *const _;
