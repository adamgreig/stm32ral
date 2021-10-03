#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DDRPERFM
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::ddrperfm::Instance;
pub use crate::stm32mp::peripherals::ddrperfm::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::ddrperfm::{
    DDRPERFM_CCR, DDRPERFM_CFG, DDRPERFM_CNT0, DDRPERFM_CNT1, DDRPERFM_CNT2, DDRPERFM_CNT3,
    DDRPERFM_CTL, DDRPERFM_HWCFG, DDRPERFM_ICR, DDRPERFM_ID, DDRPERFM_IER, DDRPERFM_ISR,
    DDRPERFM_SID, DDRPERFM_STATUS, DDRPERFM_TCNT, DDRPERFM_VER,
};

/// Access functions for the DDRPERFM peripheral instance
pub mod DDRPERFM {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x5a007000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DDRPERFM
    pub const reset: ResetValues = ResetValues {
        DDRPERFM_CTL: 0x00000000,
        DDRPERFM_CFG: 0x00000000,
        DDRPERFM_STATUS: 0x00000000,
        DDRPERFM_CCR: 0x00000000,
        DDRPERFM_IER: 0x00000000,
        DDRPERFM_ISR: 0x00000000,
        DDRPERFM_ICR: 0x00000000,
        DDRPERFM_TCNT: 0x00000000,
        DDRPERFM_CNT0: 0x00000000,
        DDRPERFM_CNT1: 0x00000000,
        DDRPERFM_CNT2: 0x00000000,
        DDRPERFM_CNT3: 0x00000000,
        DDRPERFM_HWCFG: 0x00000004,
        DDRPERFM_VER: 0x00000010,
        DDRPERFM_ID: 0x00140061,
        DDRPERFM_SID: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut DDRPERFM_TAKEN: bool = false;

    /// Safe access to DDRPERFM
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
            if DDRPERFM_TAKEN {
                None
            } else {
                DDRPERFM_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to DDRPERFM
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if DDRPERFM_TAKEN && inst.addr == INSTANCE.addr {
                DDRPERFM_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal DDRPERFM
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        DDRPERFM_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to DDRPERFM
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DDRPERFM: *const RegisterBlock = 0x5a007000 as *const _;
