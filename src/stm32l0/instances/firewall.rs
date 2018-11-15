#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Firewall
//!
//! Used by: stm32l0x1, stm32l0x2, stm32l0x3

#[cfg(not(feature = "nosync"))]
pub use stm32l0::peripherals::firewall::Instance;
pub use stm32l0::peripherals::firewall::{RegisterBlock, ResetValues};
pub use stm32l0::peripherals::firewall::{
    FIREWALL_CR, FIREWALL_CSL, FIREWALL_CSSA, FIREWALL_NVDSL, FIREWALL_NVDSSA, FIREWALL_VDSL,
    FIREWALL_VDSSA,
};

/// Access functions for the Firewall peripheral instance
pub mod Firewall {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40011c00,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in Firewall
    pub const reset: ResetValues = ResetValues {
        FIREWALL_CSSA: 0x00000000,
        FIREWALL_CSL: 0x00000000,
        FIREWALL_NVDSSA: 0x00000000,
        FIREWALL_NVDSL: 0x00000000,
        FIREWALL_VDSSA: 0x00000000,
        FIREWALL_VDSL: 0x00000000,
        FIREWALL_CR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[no_mangle]
    static mut Firewall_TAKEN: bool = false;

    /// Safe access to Firewall
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
            if Firewall_TAKEN {
                None
            } else {
                Firewall_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to Firewall
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if Firewall_TAKEN && inst.addr == INSTANCE.addr {
                Firewall_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }
}

/// Raw pointer to Firewall
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const Firewall: *const RegisterBlock = 0x40011c00 as *const _;
