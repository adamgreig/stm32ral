#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DMAMUX
//!
//! Used by: stm32h743, stm32h743v, stm32h747cm7, stm32h753, stm32h753v, stm32h757cm7

#[cfg(not(feature = "nosync"))]
pub use crate::stm32h7::peripherals::dmamux1::Instance;
pub use crate::stm32h7::peripherals::dmamux1::{RegisterBlock, ResetValues};
pub use crate::stm32h7::peripherals::dmamux1::{
    CCR0, CCR1, CCR10, CCR11, CCR12, CCR13, CCR14, CCR15, CCR2, CCR3, CCR4, CCR5, CCR6, CCR7, CCR8,
    CCR9, CFR, CSR, RGCFR, RGCR0, RGCR1, RGCR2, RGCR3, RGCR4, RGCR5, RGCR6, RGCR7, RGSR,
};

/// Access functions for the DMAMUX1 peripheral instance
pub mod DMAMUX1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40020800,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DMAMUX1
    pub const reset: ResetValues = ResetValues {
        CCR0: 0x00000000,
        CCR1: 0x00000000,
        CCR2: 0x00000000,
        CCR3: 0x00000000,
        CCR4: 0x00000000,
        CCR5: 0x00000000,
        CCR6: 0x00000000,
        CCR7: 0x00000000,
        CCR8: 0x00000000,
        CCR9: 0x00000000,
        CCR10: 0x00000000,
        CCR11: 0x00000000,
        CCR12: 0x00000000,
        CCR13: 0x00000000,
        CCR14: 0x00000000,
        CCR15: 0x00000000,
        RGCR0: 0x00000000,
        RGCR1: 0x00000000,
        RGCR2: 0x00000000,
        RGCR3: 0x00000000,
        RGCR4: 0x00000000,
        RGCR5: 0x00000000,
        RGCR6: 0x00000000,
        RGCR7: 0x00000000,
        RGSR: 0x00000000,
        RGCFR: 0x00000000,
        CSR: 0x00000000,
        CFR: 0x00000000,
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
pub const DMAMUX1: *const RegisterBlock = 0x40020800 as *const _;
