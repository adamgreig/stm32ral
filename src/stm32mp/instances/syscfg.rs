#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! SYSCFG
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::syscfg::Instance;
pub use crate::stm32mp::peripherals::syscfg::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::syscfg::{
    SYSCFG_BOOTR, SYSCFG_CBR, SYSCFG_CMPCR, SYSCFG_CMPENCLRR, SYSCFG_CMPENSETR, SYSCFG_ICNR,
    SYSCFG_IOCTRLCLRR, SYSCFG_IOCTRLSETR, SYSCFG_IPIDR, SYSCFG_PMCCLRR, SYSCFG_PMCSETR,
    SYSCFG_SIDR, SYSCFG_VERR,
};

/// Access functions for the SYSCFG peripheral instance
pub mod SYSCFG {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50020000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SYSCFG
    pub const reset: ResetValues = ResetValues {
        SYSCFG_BOOTR: 0x00000000,
        SYSCFG_PMCSETR: 0x00000000,
        SYSCFG_IOCTRLSETR: 0x00000000,
        SYSCFG_ICNR: 0x00000000,
        SYSCFG_CMPCR: 0x00870000,
        SYSCFG_CMPENSETR: 0x00000000,
        SYSCFG_CMPENCLRR: 0x00000000,
        SYSCFG_CBR: 0x00000000,
        SYSCFG_PMCCLRR: 0x00000000,
        SYSCFG_IOCTRLCLRR: 0x00000000,
        SYSCFG_VERR: 0x00000020,
        SYSCFG_IPIDR: 0x00030001,
        SYSCFG_SIDR: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SYSCFG_TAKEN: bool = false;

    /// Safe access to SYSCFG
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
            if SYSCFG_TAKEN {
                None
            } else {
                SYSCFG_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SYSCFG
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SYSCFG_TAKEN && inst.addr == INSTANCE.addr {
                SYSCFG_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SYSCFG
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SYSCFG_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SYSCFG
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SYSCFG: *const RegisterBlock = 0x50020000 as *const _;
