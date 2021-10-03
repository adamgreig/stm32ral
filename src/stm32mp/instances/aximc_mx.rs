#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! AXIMC_Mx
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::aximc_mx::Instance;
pub use crate::stm32mp::peripherals::aximc_mx::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::aximc_mx::{
    AXIMC_COMP_ID_0, AXIMC_COMP_ID_1, AXIMC_COMP_ID_2, AXIMC_COMP_ID_3, AXIMC_FN_MOD_LB,
    AXIMC_M0_FN_MOD, AXIMC_M0_FN_MOD2, AXIMC_M0_FN_MOD_AHB, AXIMC_M0_READ_QOS, AXIMC_M0_WRITE_QOS,
    AXIMC_M10_FN_MOD, AXIMC_M10_READ_QOS, AXIMC_M10_WRITE_QOS, AXIMC_M1_FN_MOD, AXIMC_M1_FN_MOD2,
    AXIMC_M1_FN_MOD_AHB, AXIMC_M1_READ_QOS, AXIMC_M1_WRITE_QOS, AXIMC_M2_FN_MOD, AXIMC_M2_FN_MOD2,
    AXIMC_M2_FN_MOD_AHB, AXIMC_M2_READ_QOS, AXIMC_M2_WRITE_QOS, AXIMC_M3_FN_MOD, AXIMC_M3_READ_QOS,
    AXIMC_M3_WRITE_QOS, AXIMC_M4_FN_MOD, AXIMC_M4_FN_MOD2, AXIMC_M4_READ_QOS, AXIMC_M4_WRITE_QOS,
    AXIMC_M5_FN_MOD, AXIMC_M5_FN_MOD2, AXIMC_M5_FN_MOD_AHB, AXIMC_M5_READ_QOS, AXIMC_M5_WRITE_QOS,
    AXIMC_M6_FN_MOD, AXIMC_M6_FN_MOD2, AXIMC_M6_FN_MOD_AHB, AXIMC_M6_READ_QOS, AXIMC_M6_WRITE_QOS,
    AXIMC_M7_FN_MOD, AXIMC_M7_READ_QOS, AXIMC_M7_WRITE_QOS, AXIMC_M8_FN_MOD, AXIMC_M8_READ_QOS,
    AXIMC_M8_WRITE_QOS, AXIMC_M9_FN_MOD, AXIMC_M9_READ_QOS, AXIMC_M9_WRITE_QOS, AXIMC_PERIPH_ID_0,
    AXIMC_PERIPH_ID_1, AXIMC_PERIPH_ID_2, AXIMC_PERIPH_ID_3, AXIMC_PERIPH_ID_4, AXIMC_PERIPH_ID_5,
    AXIMC_PERIPH_ID_6, AXIMC_PERIPH_ID_7,
};

/// Access functions for the AXIMC_Mx peripheral instance
pub mod AXIMC_Mx {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x57042024,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in AXIMC_Mx
    pub const reset: ResetValues = ResetValues {
        AXIMC_M0_FN_MOD2: 0x00000000,
        AXIMC_M0_READ_QOS: 0x00000006,
        AXIMC_M0_WRITE_QOS: 0x00000006,
        AXIMC_M0_FN_MOD: 0x00000000,
        AXIMC_M1_FN_MOD2: 0x00000000,
        AXIMC_M1_READ_QOS: 0x00000006,
        AXIMC_M1_WRITE_QOS: 0x00000006,
        AXIMC_M1_FN_MOD: 0x00000000,
        AXIMC_M2_FN_MOD2: 0x00000000,
        AXIMC_M2_READ_QOS: 0x00000006,
        AXIMC_M2_WRITE_QOS: 0x00000006,
        AXIMC_M2_FN_MOD: 0x00000000,
        AXIMC_M5_FN_MOD2: 0x00000000,
        AXIMC_M5_READ_QOS: 0x00000004,
        AXIMC_M5_WRITE_QOS: 0x00000004,
        AXIMC_M5_FN_MOD: 0x00000000,
        AXIMC_M3_READ_QOS: 0x00000007,
        AXIMC_M3_WRITE_QOS: 0x00000007,
        AXIMC_M3_FN_MOD: 0x00000000,
        AXIMC_M7_READ_QOS: 0x00000008,
        AXIMC_M7_WRITE_QOS: 0x00000008,
        AXIMC_M7_FN_MOD: 0x00000000,
        AXIMC_M8_READ_QOS: 0x00000008,
        AXIMC_M8_WRITE_QOS: 0x00000008,
        AXIMC_M8_FN_MOD: 0x00000000,
        AXIMC_M4_FN_MOD2: 0x00000000,
        AXIMC_M4_READ_QOS: 0x00000007,
        AXIMC_M4_WRITE_QOS: 0x00000007,
        AXIMC_M4_FN_MOD: 0x00000000,
        AXIMC_M9_READ_QOS: 0x0000000B,
        AXIMC_M9_WRITE_QOS: 0x0000000B,
        AXIMC_M9_FN_MOD: 0x00000000,
        AXIMC_M10_READ_QOS: 0x0000000B,
        AXIMC_M10_WRITE_QOS: 0x0000000B,
        AXIMC_M10_FN_MOD: 0x00000000,
        AXIMC_M6_FN_MOD2: 0x00000000,
        AXIMC_M6_READ_QOS: 0x00000004,
        AXIMC_M6_WRITE_QOS: 0x00000004,
        AXIMC_M6_FN_MOD: 0x00000000,
        AXIMC_PERIPH_ID_4: 0x00000004,
        AXIMC_PERIPH_ID_5: 0x00000000,
        AXIMC_PERIPH_ID_6: 0x00000000,
        AXIMC_PERIPH_ID_7: 0x00000000,
        AXIMC_PERIPH_ID_0: 0x00000000,
        AXIMC_PERIPH_ID_1: 0x000000B4,
        AXIMC_PERIPH_ID_2: 0x0000003B,
        AXIMC_PERIPH_ID_3: 0x00000000,
        AXIMC_COMP_ID_0: 0x0000000D,
        AXIMC_COMP_ID_1: 0x000000F0,
        AXIMC_COMP_ID_2: 0x00000005,
        AXIMC_COMP_ID_3: 0x000000B1,
        AXIMC_M0_FN_MOD_AHB: 0x00000000,
        AXIMC_M1_FN_MOD_AHB: 0x00000000,
        AXIMC_M2_FN_MOD_AHB: 0x00000000,
        AXIMC_M5_FN_MOD_AHB: 0x00000000,
        AXIMC_M6_FN_MOD_AHB: 0x00000000,
        AXIMC_FN_MOD_LB: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut AXIMC_Mx_TAKEN: bool = false;

    /// Safe access to AXIMC_Mx
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
            if AXIMC_Mx_TAKEN {
                None
            } else {
                AXIMC_Mx_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to AXIMC_Mx
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if AXIMC_Mx_TAKEN && inst.addr == INSTANCE.addr {
                AXIMC_Mx_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal AXIMC_Mx
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        AXIMC_Mx_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to AXIMC_Mx
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const AXIMC_Mx: *const RegisterBlock = 0x57042024 as *const _;
