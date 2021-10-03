#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! CCU
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::ccu::Instance;
pub use crate::stm32mp::peripherals::ccu::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::ccu::{
    FCCAN_CCU_CCFG, FCCAN_CCU_CREL, FCCAN_CCU_CSTAT, FCCAN_CCU_CWD, FCCAN_CCU_IE, FCCAN_CCU_IR,
};

/// Access functions for the CCU peripheral instance
pub mod CCU {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x44010000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in CCU
    pub const reset: ResetValues = ResetValues {
        FCCAN_CCU_CREL: 0x11141218,
        FCCAN_CCU_CCFG: 0x00000004,
        FCCAN_CCU_CSTAT: 0x0203FFFF,
        FCCAN_CCU_CWD: 0x00000000,
        FCCAN_CCU_IR: 0x00000000,
        FCCAN_CCU_IE: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut CCU_TAKEN: bool = false;

    /// Safe access to CCU
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
            if CCU_TAKEN {
                None
            } else {
                CCU_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to CCU
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if CCU_TAKEN && inst.addr == INSTANCE.addr {
                CCU_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal CCU
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        CCU_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to CCU
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CCU: *const RegisterBlock = 0x44010000 as *const _;
