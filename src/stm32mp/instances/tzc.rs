#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! TZC
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::tzc::Instance;
pub use crate::stm32mp::peripherals::tzc::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::tzc::{
    TZC_ACTION, TZC_BUILD_CONFIG, TZC_CID0, TZC_CID1, TZC_CID2, TZC_CID3, TZC_FAIL_ADDRESS_HIGH0,
    TZC_FAIL_ADDRESS_HIGH1, TZC_FAIL_ADDRESS_LOW0, TZC_FAIL_ADDRESS_LOW1, TZC_FAIL_CONTROL0,
    TZC_FAIL_CONTROL1, TZC_FAIL_ID0, TZC_FAIL_ID1, TZC_GATE_KEEPER, TZC_INT_CLEAR, TZC_INT_STATUS,
    TZC_PID0, TZC_PID1, TZC_PID2, TZC_PID3, TZC_PID4, TZC_PID5, TZC_PID6, TZC_PID7,
    TZC_REGION_ATTRIBUTE0, TZC_REGION_ATTRIBUTE1, TZC_REGION_ATTRIBUTE2, TZC_REGION_ATTRIBUTE3,
    TZC_REGION_ATTRIBUTE4, TZC_REGION_ATTRIBUTE5, TZC_REGION_ATTRIBUTE6, TZC_REGION_ATTRIBUTE7,
    TZC_REGION_ATTRIBUTE8, TZC_REGION_BASE_HIGH0, TZC_REGION_BASE_HIGH1, TZC_REGION_BASE_HIGH2,
    TZC_REGION_BASE_HIGH3, TZC_REGION_BASE_HIGH4, TZC_REGION_BASE_HIGH5, TZC_REGION_BASE_HIGH6,
    TZC_REGION_BASE_HIGH7, TZC_REGION_BASE_HIGH8, TZC_REGION_BASE_LOW1, TZC_REGION_BASE_LOW2,
    TZC_REGION_BASE_LOW3, TZC_REGION_BASE_LOW4, TZC_REGION_BASE_LOW5, TZC_REGION_BASE_LOW6,
    TZC_REGION_BASE_LOW7, TZC_REGION_BASE_LOW8, TZC_REGION_ID_ACCESS0, TZC_REGION_ID_ACCESS1,
    TZC_REGION_ID_ACCESS2, TZC_REGION_ID_ACCESS3, TZC_REGION_ID_ACCESS4, TZC_REGION_ID_ACCESS5,
    TZC_REGION_ID_ACCESS6, TZC_REGION_ID_ACCESS7, TZC_REGION_ID_ACCESS8, TZC_REGION_TOP_HIGH0,
    TZC_REGION_TOP_HIGH1, TZC_REGION_TOP_HIGH2, TZC_REGION_TOP_HIGH3, TZC_REGION_TOP_HIGH4,
    TZC_REGION_TOP_HIGH5, TZC_REGION_TOP_HIGH6, TZC_REGION_TOP_HIGH7, TZC_REGION_TOP_HIGH8,
    TZC_REGION_TOP_LOW0, TZC_REGION_TOP_LOW1, TZC_REGION_TOP_LOW2, TZC_REGION_TOP_LOW3,
    TZC_REGION_TOP_LOW4, TZC_REGION_TOP_LOW5, TZC_REGION_TOP_LOW6, TZC_REGION_TOP_LOW7,
    TZC_REGION_TOP_LOW8, TZC_SPECULATION_CTRL,
};

/// Access functions for the TZC peripheral instance
pub mod TZC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x5c006000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in TZC
    pub const reset: ResetValues = ResetValues {
        TZC_BUILD_CONFIG: 0x01001F08,
        TZC_ACTION: 0x00000000,
        TZC_GATE_KEEPER: 0x00000000,
        TZC_SPECULATION_CTRL: 0x00000000,
        TZC_INT_STATUS: 0x00000000,
        TZC_INT_CLEAR: 0x00000000,
        TZC_FAIL_CONTROL0: 0x00000000,
        TZC_FAIL_ID0: 0x00000000,
        TZC_FAIL_CONTROL1: 0x00000000,
        TZC_FAIL_ID1: 0x00000000,
        TZC_REGION_ATTRIBUTE0: 0x00000003,
        TZC_REGION_ATTRIBUTE1: 0x00000000,
        TZC_REGION_ATTRIBUTE2: 0x00000000,
        TZC_REGION_ATTRIBUTE3: 0x00000000,
        TZC_REGION_ATTRIBUTE4: 0x00000000,
        TZC_REGION_ATTRIBUTE5: 0x00000000,
        TZC_REGION_ATTRIBUTE6: 0x00000000,
        TZC_REGION_ATTRIBUTE7: 0x00000000,
        TZC_REGION_ATTRIBUTE8: 0x00000000,
        TZC_PID4: 0x00000004,
        TZC_PID5: 0x00000000,
        TZC_PID6: 0x00000000,
        TZC_PID7: 0x00000000,
        TZC_PID0: 0x00000060,
        TZC_PID1: 0x000000B4,
        TZC_PID2: 0x0000002B,
        TZC_PID3: 0x00000000,
        TZC_CID0: 0x0000000D,
        TZC_CID1: 0x000000F0,
        TZC_CID2: 0x00000005,
        TZC_CID3: 0x000000B1,
        TZC_FAIL_ADDRESS_LOW0: 0x00000000,
        TZC_FAIL_ADDRESS_HIGH0: 0x00000000,
        TZC_FAIL_ADDRESS_LOW1: 0x00000000,
        TZC_FAIL_ADDRESS_HIGH1: 0x00000000,
        TZC_REGION_BASE_HIGH0: 0x00000000,
        TZC_REGION_TOP_LOW0: 0xFFFFFFFF,
        TZC_REGION_TOP_HIGH0: 0x00000000,
        TZC_REGION_ID_ACCESS0: 0x00000000,
        TZC_REGION_BASE_LOW1: 0x00000000,
        TZC_REGION_BASE_HIGH1: 0x00000000,
        TZC_REGION_TOP_LOW1: 0x00000FFF,
        TZC_REGION_TOP_HIGH1: 0x00000000,
        TZC_REGION_ID_ACCESS1: 0x00000000,
        TZC_REGION_BASE_LOW2: 0x00000000,
        TZC_REGION_BASE_HIGH2: 0x00000000,
        TZC_REGION_TOP_LOW2: 0x00000FFF,
        TZC_REGION_TOP_HIGH2: 0x00000000,
        TZC_REGION_ID_ACCESS2: 0x00000000,
        TZC_REGION_BASE_LOW3: 0x00000000,
        TZC_REGION_BASE_HIGH3: 0x00000000,
        TZC_REGION_TOP_LOW3: 0x00000FFF,
        TZC_REGION_TOP_HIGH3: 0x00000000,
        TZC_REGION_ID_ACCESS3: 0x00000000,
        TZC_REGION_BASE_LOW4: 0x00000000,
        TZC_REGION_BASE_HIGH4: 0x00000000,
        TZC_REGION_TOP_LOW4: 0x00000FFF,
        TZC_REGION_TOP_HIGH4: 0x00000000,
        TZC_REGION_ID_ACCESS4: 0x00000000,
        TZC_REGION_BASE_LOW5: 0x00000000,
        TZC_REGION_BASE_HIGH5: 0x00000000,
        TZC_REGION_TOP_LOW5: 0x00000FFF,
        TZC_REGION_TOP_HIGH5: 0x00000000,
        TZC_REGION_ID_ACCESS5: 0x00000000,
        TZC_REGION_BASE_LOW6: 0x00000000,
        TZC_REGION_BASE_HIGH6: 0x00000000,
        TZC_REGION_TOP_LOW6: 0x00000FFF,
        TZC_REGION_TOP_HIGH6: 0x00000000,
        TZC_REGION_ID_ACCESS6: 0x00000000,
        TZC_REGION_BASE_LOW7: 0x00000000,
        TZC_REGION_BASE_HIGH7: 0x00000000,
        TZC_REGION_TOP_LOW7: 0x00000FFF,
        TZC_REGION_TOP_HIGH7: 0x00000000,
        TZC_REGION_ID_ACCESS7: 0x00000000,
        TZC_REGION_BASE_LOW8: 0x00000000,
        TZC_REGION_BASE_HIGH8: 0x00000000,
        TZC_REGION_TOP_LOW8: 0x00000FFF,
        TZC_REGION_TOP_HIGH8: 0x00000000,
        TZC_REGION_ID_ACCESS8: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut TZC_TAKEN: bool = false;

    /// Safe access to TZC
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
            if TZC_TAKEN {
                None
            } else {
                TZC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to TZC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if TZC_TAKEN && inst.addr == INSTANCE.addr {
                TZC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal TZC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        TZC_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to TZC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const TZC: *const RegisterBlock = 0x5c006000 as *const _;
