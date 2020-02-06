#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! CCU registers
//!
//! Used by: stm32h743, stm32h743v, stm32h747cm7, stm32h753, stm32h753v, stm32h757cm7

#[cfg(not(feature = "nosync"))]
pub use crate::stm32h7::peripherals::can_ccu::Instance;
pub use crate::stm32h7::peripherals::can_ccu::{RegisterBlock, ResetValues};
pub use crate::stm32h7::peripherals::can_ccu::{CCFG, CREL, CSTAT, CWD, IE, IR};

/// Access functions for the CAN_CCU peripheral instance
pub mod CAN_CCU {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x4000a800,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in CAN_CCU
    pub const reset: ResetValues = ResetValues {
        CREL: 0x00000000,
        CCFG: 0x00000000,
        CSTAT: 0x00000000,
        CWD: 0x00000000,
        IR: 0x00000000,
        IE: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut CAN_CCU_TAKEN: bool = false;

    /// Safe access to CAN_CCU
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
            if CAN_CCU_TAKEN {
                None
            } else {
                CAN_CCU_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to CAN_CCU
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if CAN_CCU_TAKEN && inst.addr == INSTANCE.addr {
                CAN_CCU_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal CAN_CCU
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        CAN_CCU_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to CAN_CCU
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CAN_CCU: *const RegisterBlock = 0x4000a800 as *const _;
