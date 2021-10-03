#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! BSEC2
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::bsec::Instance;
pub use crate::stm32mp::peripherals::bsec::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::bsec::{
    BSEC_DENABLE, BSEC_HWCFGR, BSEC_IPIDR, BSEC_JTAGIN, BSEC_JTAGOUT, BSEC_OTP_CONFIG,
    BSEC_OTP_CONTROL, BSEC_OTP_DATA0, BSEC_OTP_DATA1, BSEC_OTP_DATA10, BSEC_OTP_DATA11,
    BSEC_OTP_DATA12, BSEC_OTP_DATA13, BSEC_OTP_DATA14, BSEC_OTP_DATA15, BSEC_OTP_DATA16,
    BSEC_OTP_DATA17, BSEC_OTP_DATA18, BSEC_OTP_DATA19, BSEC_OTP_DATA2, BSEC_OTP_DATA20,
    BSEC_OTP_DATA21, BSEC_OTP_DATA22, BSEC_OTP_DATA23, BSEC_OTP_DATA24, BSEC_OTP_DATA25,
    BSEC_OTP_DATA26, BSEC_OTP_DATA27, BSEC_OTP_DATA28, BSEC_OTP_DATA29, BSEC_OTP_DATA3,
    BSEC_OTP_DATA30, BSEC_OTP_DATA31, BSEC_OTP_DATA32, BSEC_OTP_DATA33, BSEC_OTP_DATA34,
    BSEC_OTP_DATA35, BSEC_OTP_DATA36, BSEC_OTP_DATA37, BSEC_OTP_DATA38, BSEC_OTP_DATA39,
    BSEC_OTP_DATA4, BSEC_OTP_DATA40, BSEC_OTP_DATA41, BSEC_OTP_DATA42, BSEC_OTP_DATA43,
    BSEC_OTP_DATA44, BSEC_OTP_DATA45, BSEC_OTP_DATA46, BSEC_OTP_DATA47, BSEC_OTP_DATA48,
    BSEC_OTP_DATA49, BSEC_OTP_DATA5, BSEC_OTP_DATA50, BSEC_OTP_DATA51, BSEC_OTP_DATA52,
    BSEC_OTP_DATA53, BSEC_OTP_DATA54, BSEC_OTP_DATA55, BSEC_OTP_DATA56, BSEC_OTP_DATA57,
    BSEC_OTP_DATA58, BSEC_OTP_DATA59, BSEC_OTP_DATA6, BSEC_OTP_DATA60, BSEC_OTP_DATA61,
    BSEC_OTP_DATA62, BSEC_OTP_DATA63, BSEC_OTP_DATA64, BSEC_OTP_DATA65, BSEC_OTP_DATA66,
    BSEC_OTP_DATA67, BSEC_OTP_DATA68, BSEC_OTP_DATA69, BSEC_OTP_DATA7, BSEC_OTP_DATA70,
    BSEC_OTP_DATA71, BSEC_OTP_DATA72, BSEC_OTP_DATA73, BSEC_OTP_DATA74, BSEC_OTP_DATA75,
    BSEC_OTP_DATA76, BSEC_OTP_DATA77, BSEC_OTP_DATA78, BSEC_OTP_DATA79, BSEC_OTP_DATA8,
    BSEC_OTP_DATA80, BSEC_OTP_DATA81, BSEC_OTP_DATA82, BSEC_OTP_DATA83, BSEC_OTP_DATA84,
    BSEC_OTP_DATA85, BSEC_OTP_DATA86, BSEC_OTP_DATA87, BSEC_OTP_DATA88, BSEC_OTP_DATA89,
    BSEC_OTP_DATA9, BSEC_OTP_DATA90, BSEC_OTP_DATA91, BSEC_OTP_DATA92, BSEC_OTP_DATA93,
    BSEC_OTP_DATA94, BSEC_OTP_DATA95, BSEC_OTP_DISTURBED0, BSEC_OTP_DISTURBED1,
    BSEC_OTP_DISTURBED2, BSEC_OTP_ERROR0, BSEC_OTP_ERROR1, BSEC_OTP_ERROR2, BSEC_OTP_LOCK,
    BSEC_OTP_SPLOCK0, BSEC_OTP_SPLOCK1, BSEC_OTP_SPLOCK2, BSEC_OTP_SRLOCK0, BSEC_OTP_SRLOCK1,
    BSEC_OTP_SRLOCK2, BSEC_OTP_STATUS, BSEC_OTP_SWLOCK0, BSEC_OTP_SWLOCK1, BSEC_OTP_SWLOCK2,
    BSEC_OTP_WRDATA, BSEC_OTP_WRLOCK0, BSEC_OTP_WRLOCK1, BSEC_OTP_WRLOCK2, BSEC_SCRATCH, BSEC_SIDR,
    BSEC_VERR,
};

/// Access functions for the BSEC peripheral instance
pub mod BSEC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x5c005000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in BSEC
    pub const reset: ResetValues = ResetValues {
        BSEC_OTP_CONFIG: 0x0000000E,
        BSEC_OTP_CONTROL: 0x00000000,
        BSEC_OTP_WRDATA: 0x00000000,
        BSEC_OTP_STATUS: 0x00000000,
        BSEC_OTP_LOCK: 0x00000000,
        BSEC_DENABLE: 0x00000000,
        BSEC_OTP_DISTURBED0: 0x00000000,
        BSEC_OTP_DISTURBED1: 0x00000000,
        BSEC_OTP_DISTURBED2: 0x00000000,
        BSEC_OTP_ERROR0: 0x00000000,
        BSEC_OTP_ERROR1: 0x00000000,
        BSEC_OTP_ERROR2: 0x00000000,
        BSEC_OTP_WRLOCK0: 0x00000000,
        BSEC_OTP_WRLOCK1: 0x00000000,
        BSEC_OTP_WRLOCK2: 0x00000000,
        BSEC_OTP_SPLOCK0: 0x00000000,
        BSEC_OTP_SPLOCK1: 0x00000000,
        BSEC_OTP_SPLOCK2: 0x00000000,
        BSEC_OTP_SWLOCK0: 0x00000001,
        BSEC_OTP_SWLOCK1: 0x00000001,
        BSEC_OTP_SWLOCK2: 0x00000001,
        BSEC_OTP_SRLOCK0: 0x00000000,
        BSEC_OTP_SRLOCK1: 0x00000000,
        BSEC_OTP_SRLOCK2: 0x00000000,
        BSEC_JTAGIN: 0x00000000,
        BSEC_JTAGOUT: 0x00000000,
        BSEC_SCRATCH: 0x00000000,
        BSEC_OTP_DATA0: 0x00000000,
        BSEC_OTP_DATA1: 0x00000000,
        BSEC_OTP_DATA2: 0x00000000,
        BSEC_OTP_DATA3: 0x00000000,
        BSEC_OTP_DATA4: 0x00000000,
        BSEC_OTP_DATA5: 0x00000000,
        BSEC_OTP_DATA6: 0x00000000,
        BSEC_OTP_DATA7: 0x00000000,
        BSEC_OTP_DATA8: 0x00000000,
        BSEC_OTP_DATA9: 0x00000000,
        BSEC_OTP_DATA10: 0x00000000,
        BSEC_OTP_DATA11: 0x00000000,
        BSEC_OTP_DATA12: 0x00000000,
        BSEC_OTP_DATA13: 0x00000000,
        BSEC_OTP_DATA14: 0x00000000,
        BSEC_OTP_DATA15: 0x00000000,
        BSEC_OTP_DATA16: 0x00000000,
        BSEC_OTP_DATA17: 0x00000000,
        BSEC_OTP_DATA18: 0x00000000,
        BSEC_OTP_DATA19: 0x00000000,
        BSEC_OTP_DATA20: 0x00000000,
        BSEC_OTP_DATA21: 0x00000000,
        BSEC_OTP_DATA22: 0x00000000,
        BSEC_OTP_DATA23: 0x00000000,
        BSEC_OTP_DATA24: 0x00000000,
        BSEC_OTP_DATA25: 0x00000000,
        BSEC_OTP_DATA26: 0x00000000,
        BSEC_OTP_DATA27: 0x00000000,
        BSEC_OTP_DATA28: 0x00000000,
        BSEC_OTP_DATA29: 0x00000000,
        BSEC_OTP_DATA30: 0x00000000,
        BSEC_OTP_DATA31: 0x00000000,
        BSEC_OTP_DATA32: 0x00000000,
        BSEC_OTP_DATA33: 0x00000000,
        BSEC_OTP_DATA34: 0x00000000,
        BSEC_OTP_DATA35: 0x00000000,
        BSEC_OTP_DATA36: 0x00000000,
        BSEC_OTP_DATA37: 0x00000000,
        BSEC_OTP_DATA38: 0x00000000,
        BSEC_OTP_DATA39: 0x00000000,
        BSEC_OTP_DATA40: 0x00000000,
        BSEC_OTP_DATA41: 0x00000000,
        BSEC_OTP_DATA42: 0x00000000,
        BSEC_OTP_DATA43: 0x00000000,
        BSEC_OTP_DATA44: 0x00000000,
        BSEC_OTP_DATA45: 0x00000000,
        BSEC_OTP_DATA46: 0x00000000,
        BSEC_OTP_DATA47: 0x00000000,
        BSEC_OTP_DATA48: 0x00000000,
        BSEC_OTP_DATA49: 0x00000000,
        BSEC_OTP_DATA50: 0x00000000,
        BSEC_OTP_DATA51: 0x00000000,
        BSEC_OTP_DATA52: 0x00000000,
        BSEC_OTP_DATA53: 0x00000000,
        BSEC_OTP_DATA54: 0x00000000,
        BSEC_OTP_DATA55: 0x00000000,
        BSEC_OTP_DATA56: 0x00000000,
        BSEC_OTP_DATA57: 0x00000000,
        BSEC_OTP_DATA58: 0x00000000,
        BSEC_OTP_DATA59: 0x00000000,
        BSEC_OTP_DATA60: 0x00000000,
        BSEC_OTP_DATA61: 0x00000000,
        BSEC_OTP_DATA62: 0x00000000,
        BSEC_OTP_DATA63: 0x00000000,
        BSEC_OTP_DATA64: 0x00000000,
        BSEC_OTP_DATA65: 0x00000000,
        BSEC_OTP_DATA66: 0x00000000,
        BSEC_OTP_DATA67: 0x00000000,
        BSEC_OTP_DATA68: 0x00000000,
        BSEC_OTP_DATA69: 0x00000000,
        BSEC_OTP_DATA70: 0x00000000,
        BSEC_OTP_DATA71: 0x00000000,
        BSEC_OTP_DATA72: 0x00000000,
        BSEC_OTP_DATA73: 0x00000000,
        BSEC_OTP_DATA74: 0x00000000,
        BSEC_OTP_DATA75: 0x00000000,
        BSEC_OTP_DATA76: 0x00000000,
        BSEC_OTP_DATA77: 0x00000000,
        BSEC_OTP_DATA78: 0x00000000,
        BSEC_OTP_DATA79: 0x00000000,
        BSEC_OTP_DATA80: 0x00000000,
        BSEC_OTP_DATA81: 0x00000000,
        BSEC_OTP_DATA82: 0x00000000,
        BSEC_OTP_DATA83: 0x00000000,
        BSEC_OTP_DATA84: 0x00000000,
        BSEC_OTP_DATA85: 0x00000000,
        BSEC_OTP_DATA86: 0x00000000,
        BSEC_OTP_DATA87: 0x00000000,
        BSEC_OTP_DATA88: 0x00000000,
        BSEC_OTP_DATA89: 0x00000000,
        BSEC_OTP_DATA90: 0x00000000,
        BSEC_OTP_DATA91: 0x00000000,
        BSEC_OTP_DATA92: 0x00000000,
        BSEC_OTP_DATA93: 0x00000000,
        BSEC_OTP_DATA94: 0x00000000,
        BSEC_OTP_DATA95: 0x00000000,
        BSEC_HWCFGR: 0x00000014,
        BSEC_VERR: 0x00000011,
        BSEC_IPIDR: 0x00100032,
        BSEC_SIDR: 0xA3C5DD04,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut BSEC_TAKEN: bool = false;

    /// Safe access to BSEC
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
            if BSEC_TAKEN {
                None
            } else {
                BSEC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to BSEC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if BSEC_TAKEN && inst.addr == INSTANCE.addr {
                BSEC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal BSEC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        BSEC_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to BSEC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const BSEC: *const RegisterBlock = 0x5c005000 as *const _;
