#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! System configuration controller
//!
//! Used by: stm32f0x1, stm32f0x2, stm32f0x8

#[cfg(not(feature = "nosync"))]
pub use stm32f0::peripherals::syscfg_comp::Instance;
pub use stm32f0::peripherals::syscfg_comp::{RegisterBlock, ResetValues};
pub use stm32f0::peripherals::syscfg_comp::{
    COMP_CSR, SYSCFG_CFGR1, SYSCFG_CFGR2, SYSCFG_EXTICR1, SYSCFG_EXTICR2, SYSCFG_EXTICR3,
    SYSCFG_EXTICR4,
};

/// Access functions for the SYSCFG_COMP peripheral instance
pub mod SYSCFG_COMP {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40010000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SYSCFG_COMP
    pub const reset: ResetValues = ResetValues {
        SYSCFG_CFGR1: 0x00000000,
        SYSCFG_EXTICR1: 0x00000000,
        SYSCFG_EXTICR2: 0x00000000,
        SYSCFG_EXTICR3: 0x00000000,
        SYSCFG_EXTICR4: 0x00000000,
        SYSCFG_CFGR2: 0x00000000,
        COMP_CSR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[no_mangle]
    static mut SYSCFG_COMP_TAKEN: bool = false;

    /// Safe access to SYSCFG_COMP
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
            if SYSCFG_COMP_TAKEN {
                None
            } else {
                SYSCFG_COMP_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SYSCFG_COMP
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SYSCFG_COMP_TAKEN && inst.addr == INSTANCE.addr {
                SYSCFG_COMP_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }
}

/// Raw pointer to SYSCFG_COMP
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SYSCFG_COMP: *const RegisterBlock = 0x40010000 as *const _;
