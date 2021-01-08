#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DMAMUX
//!
//! Used by: stm32g070, stm32g071, stm32g07x, stm32g081

#[cfg(not(feature = "nosync"))]
pub use crate::stm32g0::peripherals::dmamux_v2::Instance;
pub use crate::stm32g0::peripherals::dmamux_v2::{RegisterBlock, ResetValues};
pub use crate::stm32g0::peripherals::dmamux_v2::{
    DMAMUX_C0CR, DMAMUX_C1CR, DMAMUX_C2CR, DMAMUX_C3CR, DMAMUX_C4CR, DMAMUX_C5CR, DMAMUX_C6CR,
    DMAMUX_CFR, DMAMUX_CSR, DMAMUX_HWCFGR1, DMAMUX_HWCFGR2, DMAMUX_IPIDR, DMAMUX_RG0CR,
    DMAMUX_RG1CR, DMAMUX_RG2CR, DMAMUX_RG3CR, DMAMUX_RGCFR, DMAMUX_RGSR, DMAMUX_SIDR, DMAMUX_VERR,
};

/// Access functions for the DMAMUX peripheral instance
pub mod DMAMUX {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40020800,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DMAMUX
    pub const reset: ResetValues = ResetValues {
        DMAMUX_C0CR: 0x00000000,
        DMAMUX_C1CR: 0x00000000,
        DMAMUX_C2CR: 0x00000000,
        DMAMUX_C3CR: 0x00000000,
        DMAMUX_C4CR: 0x00000000,
        DMAMUX_C5CR: 0x00000000,
        DMAMUX_C6CR: 0x00000000,
        DMAMUX_RG0CR: 0x00000000,
        DMAMUX_RG1CR: 0x00000000,
        DMAMUX_RG2CR: 0x00000000,
        DMAMUX_RG3CR: 0x00000000,
        DMAMUX_RGSR: 0x00000000,
        DMAMUX_RGCFR: 0x00000000,
        DMAMUX_CSR: 0x00000000,
        DMAMUX_CFR: 0x00000000,
        DMAMUX_SIDR: 0xA3C5DD01,
        DMAMUX_IPIDR: 0x00100011,
        DMAMUX_VERR: 0x00000011,
        DMAMUX_HWCFGR1: 0x04173907,
        DMAMUX_HWCFGR2: 0x00000017,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut DMAMUX_TAKEN: bool = false;

    /// Safe access to DMAMUX
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
            if DMAMUX_TAKEN {
                None
            } else {
                DMAMUX_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to DMAMUX
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if DMAMUX_TAKEN && inst.addr == INSTANCE.addr {
                DMAMUX_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal DMAMUX
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        DMAMUX_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to DMAMUX
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DMAMUX: *const RegisterBlock = 0x40020800 as *const _;
