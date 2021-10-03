#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Real-time clock
//!
//! Used by: stm32l4r9, stm32l4x1, stm32l4x2, stm32l4x3, stm32l4x5, stm32l4x6

#[cfg(not(feature = "nosync"))]
pub use crate::stm32l4::peripherals::rtc::Instance;
pub use crate::stm32l4::peripherals::rtc::{RegisterBlock, ResetValues};
pub use crate::stm32l4::peripherals::rtc::{
    ALRMAR, ALRMASSR, ALRMBR, ALRMBSSR, BKP0R, BKP10R, BKP11R, BKP12R, BKP13R, BKP14R, BKP15R,
    BKP16R, BKP17R, BKP18R, BKP19R, BKP1R, BKP20R, BKP21R, BKP22R, BKP23R, BKP24R, BKP25R, BKP26R,
    BKP27R, BKP28R, BKP29R, BKP2R, BKP30R, BKP31R, BKP3R, BKP4R, BKP5R, BKP6R, BKP7R, BKP8R, BKP9R,
    CALR, CR, DR, ISR, OR, PRER, SHIFTR, SSR, TAMPCR, TR, TSDR, TSSSR, TSTR, WPR, WUTR,
};

/// Access functions for the RTC peripheral instance
pub mod RTC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40002800,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in RTC
    pub const reset: ResetValues = ResetValues {
        TR: 0x00000000,
        DR: 0x00002101,
        CR: 0x00000000,
        ISR: 0x00000007,
        PRER: 0x007F00FF,
        WUTR: 0x0000FFFF,
        ALRMAR: 0x00000000,
        ALRMBR: 0x00000000,
        WPR: 0x00000000,
        SSR: 0x00000000,
        SHIFTR: 0x00000000,
        TSTR: 0x00000000,
        TSDR: 0x00000000,
        TSSSR: 0x00000000,
        CALR: 0x00000000,
        TAMPCR: 0x00000000,
        ALRMASSR: 0x00000000,
        ALRMBSSR: 0x00000000,
        OR: 0x00000000,
        BKP0R: 0x00000000,
        BKP1R: 0x00000000,
        BKP2R: 0x00000000,
        BKP3R: 0x00000000,
        BKP4R: 0x00000000,
        BKP5R: 0x00000000,
        BKP6R: 0x00000000,
        BKP7R: 0x00000000,
        BKP8R: 0x00000000,
        BKP9R: 0x00000000,
        BKP10R: 0x00000000,
        BKP11R: 0x00000000,
        BKP12R: 0x00000000,
        BKP13R: 0x00000000,
        BKP14R: 0x00000000,
        BKP15R: 0x00000000,
        BKP16R: 0x00000000,
        BKP17R: 0x00000000,
        BKP18R: 0x00000000,
        BKP19R: 0x00000000,
        BKP20R: 0x00000000,
        BKP21R: 0x00000000,
        BKP22R: 0x00000000,
        BKP23R: 0x00000000,
        BKP24R: 0x00000000,
        BKP25R: 0x00000000,
        BKP26R: 0x00000000,
        BKP27R: 0x00000000,
        BKP28R: 0x00000000,
        BKP29R: 0x00000000,
        BKP30R: 0x00000000,
        BKP31R: 0x00000000,
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
pub const RTC: *const RegisterBlock = 0x40002800 as *const _;
