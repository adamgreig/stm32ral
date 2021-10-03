#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! ETZPC
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::etzpc::Instance;
pub use crate::stm32mp::peripherals::etzpc::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::etzpc::{
    ETZPC_DECPROT0, ETZPC_DECPROT1, ETZPC_DECPROT2, ETZPC_DECPROT3, ETZPC_DECPROT4, ETZPC_DECPROT5,
    ETZPC_DECPROT_LOCK0, ETZPC_DECPROT_LOCK1, ETZPC_DECPROT_LOCK2, ETZPC_HWCFGR, ETZPC_IDR,
    ETZPC_SIDR, ETZPC_TZMA0_SIZE, ETZPC_TZMA1_SIZE, ETZPC_VERR,
};

/// Access functions for the ETZPC peripheral instance
pub mod ETZPC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x5c007000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in ETZPC
    pub const reset: ResetValues = ResetValues {
        ETZPC_TZMA0_SIZE: 0x000003FF,
        ETZPC_TZMA1_SIZE: 0x000003FF,
        ETZPC_DECPROT0: 0x00000000,
        ETZPC_DECPROT1: 0x00000000,
        ETZPC_DECPROT2: 0x00000000,
        ETZPC_DECPROT3: 0x00000000,
        ETZPC_DECPROT4: 0x00000000,
        ETZPC_DECPROT5: 0x00000000,
        ETZPC_DECPROT_LOCK0: 0x00000000,
        ETZPC_DECPROT_LOCK1: 0x00000000,
        ETZPC_DECPROT_LOCK2: 0x00000000,
        ETZPC_HWCFGR: 0x00006002,
        ETZPC_VERR: 0x00000020,
        ETZPC_IDR: 0x00100061,
        ETZPC_SIDR: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut ETZPC_TAKEN: bool = false;

    /// Safe access to ETZPC
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
            if ETZPC_TAKEN {
                None
            } else {
                ETZPC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to ETZPC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if ETZPC_TAKEN && inst.addr == INSTANCE.addr {
                ETZPC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal ETZPC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        ETZPC_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to ETZPC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const ETZPC: *const RegisterBlock = 0x5c007000 as *const _;
