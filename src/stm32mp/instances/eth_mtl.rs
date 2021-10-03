#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! ETH_MTL
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::eth_mtl::Instance;
pub use crate::stm32mp::peripherals::eth_mtl::{
    ETH_MTLRxQ0CR, ETH_MTLRxQ0DR, ETH_MTLRxQ0MPOCR, ETH_MTLRxQ0OMR, ETH_MTLRxQ1CR, ETH_MTLRxQ1DR,
    ETH_MTLRxQ1MPOCR, ETH_MTLRxQ1OMR, ETH_MTLTxQ0DR, ETH_MTLTxQ0ESR, ETH_MTLTxQ0OMR, ETH_MTLTxQ0UR,
    ETH_MTLTxQ1DR, ETH_MTLTxQ1ECR, ETH_MTLTxQ1ESR, ETH_MTLTxQ1HCR, ETH_MTLTxQ1LCR, ETH_MTLTxQ1OMR,
    ETH_MTLTxQ1QWR, ETH_MTLTxQ1SSCR, ETH_MTLTxQ1UR, ETH_MTLISR, ETH_MTLOMR, ETH_MTLQ0ICSR,
    ETH_MTLQ1ICSR,
};
pub use crate::stm32mp::peripherals::eth_mtl::{RegisterBlock, ResetValues};

/// Access functions for the ETH_MTL peripheral instance
pub mod ETH_MTL {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x5800ac00,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in ETH_MTL
    pub const reset: ResetValues = ResetValues {
        ETH_MTLOMR: 0x00000000,
        ETH_MTLISR: 0x00000000,
        ETH_MTLTxQ0OMR: 0x00000000,
        ETH_MTLTxQ1OMR: 0x00000000,
        ETH_MTLTxQ0UR: 0x00000000,
        ETH_MTLTxQ1UR: 0x00000000,
        ETH_MTLTxQ0DR: 0x00000000,
        ETH_MTLTxQ1DR: 0x00000000,
        ETH_MTLTxQ0ESR: 0x00000000,
        ETH_MTLTxQ1ESR: 0x00000000,
        ETH_MTLQ0ICSR: 0x00000000,
        ETH_MTLQ1ICSR: 0x00000000,
        ETH_MTLRxQ0OMR: 0x00700000,
        ETH_MTLRxQ1OMR: 0x00700000,
        ETH_MTLRxQ0MPOCR: 0x00000000,
        ETH_MTLRxQ1MPOCR: 0x00000000,
        ETH_MTLRxQ0DR: 0x00000000,
        ETH_MTLRxQ1DR: 0x00000000,
        ETH_MTLRxQ0CR: 0x00000000,
        ETH_MTLRxQ1CR: 0x00000000,
        ETH_MTLTxQ1ECR: 0x00000000,
        ETH_MTLTxQ1QWR: 0x00000000,
        ETH_MTLTxQ1SSCR: 0x00000000,
        ETH_MTLTxQ1HCR: 0x00000000,
        ETH_MTLTxQ1LCR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut ETH_MTL_TAKEN: bool = false;

    /// Safe access to ETH_MTL
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
            if ETH_MTL_TAKEN {
                None
            } else {
                ETH_MTL_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to ETH_MTL
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if ETH_MTL_TAKEN && inst.addr == INSTANCE.addr {
                ETH_MTL_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal ETH_MTL
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        ETH_MTL_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to ETH_MTL
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const ETH_MTL: *const RegisterBlock = 0x5800ac00 as *const _;
