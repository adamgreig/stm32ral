#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DMAMUX2
//!
//! Used by: stm32h735, stm32h743, stm32h743v, stm32h747cm4, stm32h747cm7, stm32h753, stm32h753v, stm32h7b3

#[cfg(not(feature = "nosync"))]
pub use crate::stm32h7::peripherals::dmamux2::Instance;
pub use crate::stm32h7::peripherals::dmamux2::{RegisterBlock, ResetValues};
pub use crate::stm32h7::peripherals::dmamux2::{
    CCR0, CCR1, CCR2, CCR3, CCR4, CCR5, CCR6, CCR7, CFR, CSR, RGCFR, RGCR0, RGCR1, RGCR2, RGCR3,
    RGCR4, RGCR5, RGCR6, RGCR7, RGSR,
};

/// Access functions for the DMAMUX2 peripheral instance
pub mod DMAMUX2 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x58025800,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DMAMUX2
    pub const reset: ResetValues = ResetValues {
        CCR0: 0x00000000,
        CCR1: 0x00000000,
        CCR2: 0x00000000,
        CCR3: 0x00000000,
        CCR4: 0x00000000,
        CCR5: 0x00000000,
        CCR6: 0x00000000,
        CCR7: 0x00000000,
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
    static mut DMAMUX2_TAKEN: bool = false;

    /// Safe access to DMAMUX2
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
            if DMAMUX2_TAKEN {
                None
            } else {
                DMAMUX2_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to DMAMUX2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if DMAMUX2_TAKEN && inst.addr == INSTANCE.addr {
                DMAMUX2_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal DMAMUX2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        DMAMUX2_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to DMAMUX2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DMAMUX2: *const RegisterBlock = 0x58025800 as *const _;
