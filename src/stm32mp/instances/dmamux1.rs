#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DMAMUX1
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::dmamux1::Instance;
pub use crate::stm32mp::peripherals::dmamux1::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::dmamux1::{
    DMAMUX_C0CR, DMAMUX_C10CR, DMAMUX_C11CR, DMAMUX_C12CR, DMAMUX_C13CR, DMAMUX_C14CR,
    DMAMUX_C15CR, DMAMUX_C1CR, DMAMUX_C2CR, DMAMUX_C3CR, DMAMUX_C4CR, DMAMUX_C5CR, DMAMUX_C6CR,
    DMAMUX_C7CR, DMAMUX_C8CR, DMAMUX_C9CR, DMAMUX_CFR, DMAMUX_CSR, DMAMUX_HWCFGR1, DMAMUX_HWCFGR2,
    DMAMUX_IPIDR, DMAMUX_RG0CR, DMAMUX_RG1CR, DMAMUX_RG2CR, DMAMUX_RG3CR, DMAMUX_RG4CR,
    DMAMUX_RG5CR, DMAMUX_RG6CR, DMAMUX_RG7CR, DMAMUX_RGCFR, DMAMUX_RGSR, DMAMUX_SIDR, DMAMUX_VERR,
};

/// Access functions for the DMAMUX1 peripheral instance
pub mod DMAMUX1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x48002000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DMAMUX1
    pub const reset: ResetValues = ResetValues {
        DMAMUX_C0CR: 0x00000000,
        DMAMUX_C1CR: 0x00000000,
        DMAMUX_C2CR: 0x00000000,
        DMAMUX_C3CR: 0x00000000,
        DMAMUX_C4CR: 0x00000000,
        DMAMUX_C5CR: 0x00000000,
        DMAMUX_C6CR: 0x00000000,
        DMAMUX_C7CR: 0x00000000,
        DMAMUX_C8CR: 0x00000000,
        DMAMUX_C9CR: 0x00000000,
        DMAMUX_C10CR: 0x00000000,
        DMAMUX_C11CR: 0x00000000,
        DMAMUX_C12CR: 0x00000000,
        DMAMUX_C13CR: 0x00000000,
        DMAMUX_C14CR: 0x00000000,
        DMAMUX_C15CR: 0x00000000,
        DMAMUX_CSR: 0x00000000,
        DMAMUX_CFR: 0x00000000,
        DMAMUX_RG0CR: 0x00000000,
        DMAMUX_RG1CR: 0x00000000,
        DMAMUX_RG2CR: 0x00000000,
        DMAMUX_RG3CR: 0x00000000,
        DMAMUX_RG4CR: 0x00000000,
        DMAMUX_RG5CR: 0x00000000,
        DMAMUX_RG6CR: 0x00000000,
        DMAMUX_RG7CR: 0x00000000,
        DMAMUX_RGSR: 0x00000000,
        DMAMUX_RGCFR: 0x00000000,
        DMAMUX_HWCFGR2: 0x00000008,
        DMAMUX_HWCFGR1: 0x08086C10,
        DMAMUX_VERR: 0x00000011,
        DMAMUX_IPIDR: 0x00100011,
        DMAMUX_SIDR: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut DMAMUX1_TAKEN: bool = false;

    /// Safe access to DMAMUX1
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
            if DMAMUX1_TAKEN {
                None
            } else {
                DMAMUX1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to DMAMUX1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if DMAMUX1_TAKEN && inst.addr == INSTANCE.addr {
                DMAMUX1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal DMAMUX1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        DMAMUX1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to DMAMUX1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DMAMUX1: *const RegisterBlock = 0x48002000 as *const _;
