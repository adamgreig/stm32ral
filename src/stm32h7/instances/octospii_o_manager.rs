#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! OctoSPI IO Manager
//!
//! Used by: stm32h735, stm32h7b3

#[cfg(not(feature = "nosync"))]
pub use crate::stm32h7::peripherals::octospii_o_manager::Instance;
pub use crate::stm32h7::peripherals::octospii_o_manager::{RegisterBlock, ResetValues};
pub use crate::stm32h7::peripherals::octospii_o_manager::{CR, P1CR, P2CR};

/// Access functions for the OctoSPII_O_Manager peripheral instance
pub mod OctoSPII_O_Manager {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x5200b400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in OctoSPII_O_Manager
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000000,
        P1CR: 0x03010111,
        P2CR: 0x07050333,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut OctoSPII_O_Manager_TAKEN: bool = false;

    /// Safe access to OctoSPII_O_Manager
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
            if OctoSPII_O_Manager_TAKEN {
                None
            } else {
                OctoSPII_O_Manager_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to OctoSPII_O_Manager
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if OctoSPII_O_Manager_TAKEN && inst.addr == INSTANCE.addr {
                OctoSPII_O_Manager_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal OctoSPII_O_Manager
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        OctoSPII_O_Manager_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to OctoSPII_O_Manager
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const OctoSPII_O_Manager: *const RegisterBlock = 0x5200b400 as *const _;
