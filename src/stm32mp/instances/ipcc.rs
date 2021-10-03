#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! IPCC
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::ipcc::Instance;
pub use crate::stm32mp::peripherals::ipcc::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::ipcc::{
    IPCC_C1CR, IPCC_C1MR, IPCC_C1SCR, IPCC_C1TOC2SR, IPCC_C2CR, IPCC_C2MR, IPCC_C2SCR,
    IPCC_C2TOC1SR, IPCC_HWCFGR, IPCC_ID, IPCC_SID, IPCC_VER,
};

/// Access functions for the IPCC peripheral instance
pub mod IPCC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x4c001000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in IPCC
    pub const reset: ResetValues = ResetValues {
        IPCC_C1CR: 0x00000000,
        IPCC_C1MR: 0xFFFFFFFF,
        IPCC_C1SCR: 0x00000000,
        IPCC_C1TOC2SR: 0x00000000,
        IPCC_C2CR: 0x00000000,
        IPCC_C2MR: 0xFFFFFFFF,
        IPCC_C2SCR: 0x00000000,
        IPCC_C2TOC1SR: 0x00000000,
        IPCC_HWCFGR: 0x00000002,
        IPCC_VER: 0x00000010,
        IPCC_ID: 0x00100071,
        IPCC_SID: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut IPCC_TAKEN: bool = false;

    /// Safe access to IPCC
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
            if IPCC_TAKEN {
                None
            } else {
                IPCC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to IPCC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if IPCC_TAKEN && inst.addr == INSTANCE.addr {
                IPCC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal IPCC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        IPCC_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to IPCC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const IPCC: *const RegisterBlock = 0x4c001000 as *const _;
