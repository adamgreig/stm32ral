#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! AXI interconnect registers
//!
//! Used by: stm32h743, stm32h743v, stm32h753, stm32h753v

#[cfg(not(feature = "nosync"))]
pub use crate::stm32h7::peripherals::axi_v1::Instance;
pub use crate::stm32h7::peripherals::axi_v1::{RegisterBlock, ResetValues};
pub use crate::stm32h7::peripherals::axi_v1::{
    COMP_ID_0, COMP_ID_1, COMP_ID_2, COMP_ID_3, INI1_FN_MOD, INI1_FN_MOD2, INI1_FN_MOD_AHB,
    INI1_READ_QOS, INI1_WRITE_QOS, INI2_FN_MOD, INI2_READ_QOS, INI2_WRITE_QOS, INI3_FN_MOD,
    INI3_FN_MOD2, INI3_FN_MOD_AHB, INI3_READ_QOS, INI3_WRITE_QOS, INI4_FN_MOD, INI4_READ_QOS,
    INI4_WRITE_QOS, INI5_FN_MOD, INI5_READ_QOS, INI5_WRITE_QOS, INI6_FN_MOD, INI6_READ_QOS,
    INI6_WRITE_QOS, PERIPH_ID_0, PERIPH_ID_1, PERIPH_ID_2, PERIPH_ID_3, PERIPH_ID_4, TARG1_FN_MOD,
    TARG1_FN_MOD2, TARG1_FN_MOD_ISS_BM, TARG1_FN_MOD_LB, TARG2_FN_MOD, TARG2_FN_MOD2,
    TARG2_FN_MOD_ISS_BM, TARG2_FN_MOD_LB, TARG3_FN_MOD_ISS_BM, TARG4_FN_MOD_ISS_BM,
    TARG5_FN_MOD_ISS_BM, TARG6_FN_MOD_ISS_BM, TARG7_FN_MOD, TARG7_FN_MOD2, TARG7_FN_MOD_ISS_BM,
};

/// Access functions for the AXI peripheral instance
pub mod AXI {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x51000000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in AXI
    pub const reset: ResetValues = ResetValues {
        PERIPH_ID_4: 0x00000004,
        PERIPH_ID_0: 0x00000004,
        PERIPH_ID_1: 0x00000004,
        PERIPH_ID_2: 0x00000004,
        PERIPH_ID_3: 0x00000004,
        COMP_ID_0: 0x00000004,
        COMP_ID_1: 0x00000004,
        COMP_ID_2: 0x00000004,
        COMP_ID_3: 0x00000004,
        TARG1_FN_MOD_ISS_BM: 0x00000004,
        TARG2_FN_MOD_ISS_BM: 0x00000004,
        TARG3_FN_MOD_ISS_BM: 0x00000004,
        TARG4_FN_MOD_ISS_BM: 0x00000004,
        TARG5_FN_MOD_ISS_BM: 0x00000004,
        TARG6_FN_MOD_ISS_BM: 0x00000004,
        TARG7_FN_MOD_ISS_BM: 0x00000004,
        TARG1_FN_MOD2: 0x00000004,
        TARG2_FN_MOD2: 0x00000004,
        TARG7_FN_MOD2: 0x00000004,
        TARG1_FN_MOD_LB: 0x00000004,
        TARG2_FN_MOD_LB: 0x00000004,
        TARG1_FN_MOD: 0x00000004,
        TARG2_FN_MOD: 0x00000004,
        TARG7_FN_MOD: 0x00000004,
        INI1_FN_MOD2: 0x00000004,
        INI3_FN_MOD2: 0x00000004,
        INI1_FN_MOD_AHB: 0x00000004,
        INI3_FN_MOD_AHB: 0x00000004,
        INI1_READ_QOS: 0x00000004,
        INI2_READ_QOS: 0x00000004,
        INI3_READ_QOS: 0x00000004,
        INI4_READ_QOS: 0x00000004,
        INI5_READ_QOS: 0x00000004,
        INI6_READ_QOS: 0x00000004,
        INI1_WRITE_QOS: 0x00000004,
        INI2_WRITE_QOS: 0x00000004,
        INI3_WRITE_QOS: 0x00000004,
        INI4_WRITE_QOS: 0x00000004,
        INI5_WRITE_QOS: 0x00000004,
        INI6_WRITE_QOS: 0x00000004,
        INI1_FN_MOD: 0x00000004,
        INI2_FN_MOD: 0x00000004,
        INI3_FN_MOD: 0x00000004,
        INI4_FN_MOD: 0x00000004,
        INI5_FN_MOD: 0x00000004,
        INI6_FN_MOD: 0x00000004,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut AXI_TAKEN: bool = false;

    /// Safe access to AXI
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
            if AXI_TAKEN {
                None
            } else {
                AXI_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to AXI
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if AXI_TAKEN && inst.addr == INSTANCE.addr {
                AXI_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal AXI
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        AXI_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to AXI
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const AXI: *const RegisterBlock = 0x51000000 as *const _;
