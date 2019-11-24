#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Backup registers
//!
//! Used by: stm32f100, stm32f101, stm32f102, stm32f103, stm32f107

#[cfg(not(feature = "nosync"))]
pub use crate::stm32f1::peripherals::bkp::Instance;
pub use crate::stm32f1::peripherals::bkp::{RegisterBlock, ResetValues};
pub use crate::stm32f1::peripherals::bkp::{
    BKP_DR0, BKP_DR1, BKP_DR10, BKP_DR11, BKP_DR12, BKP_DR13, BKP_DR14, BKP_DR15, BKP_DR16,
    BKP_DR17, BKP_DR18, BKP_DR19, BKP_DR2, BKP_DR20, BKP_DR21, BKP_DR22, BKP_DR23, BKP_DR24,
    BKP_DR25, BKP_DR26, BKP_DR27, BKP_DR28, BKP_DR29, BKP_DR3, BKP_DR30, BKP_DR31, BKP_DR4,
    BKP_DR5, BKP_DR6, BKP_DR7, BKP_DR8, BKP_DR9, CR, CSR, DR0, DR1, DR2, DR3, DR4, DR5, DR6, DR7,
    DR8, DR9, RTCCR,
};

/// Access functions for the BKP peripheral instance
pub mod BKP {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40006c04,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in BKP
    pub const reset: ResetValues = ResetValues {
        DR0: 0x00000000,
        DR1: 0x00000000,
        DR2: 0x00000000,
        DR3: 0x00000000,
        DR4: 0x00000000,
        DR5: 0x00000000,
        DR6: 0x00000000,
        DR7: 0x00000000,
        DR8: 0x00000000,
        DR9: 0x00000000,
        BKP_DR0: 0x00000000,
        BKP_DR1: 0x00000000,
        BKP_DR2: 0x00000000,
        BKP_DR3: 0x00000000,
        BKP_DR4: 0x00000000,
        BKP_DR5: 0x00000000,
        BKP_DR6: 0x00000000,
        BKP_DR7: 0x00000000,
        BKP_DR8: 0x00000000,
        BKP_DR9: 0x00000000,
        BKP_DR10: 0x00000000,
        BKP_DR11: 0x00000000,
        BKP_DR12: 0x00000000,
        BKP_DR13: 0x00000000,
        BKP_DR14: 0x00000000,
        BKP_DR15: 0x00000000,
        BKP_DR16: 0x00000000,
        BKP_DR17: 0x00000000,
        BKP_DR18: 0x00000000,
        BKP_DR19: 0x00000000,
        BKP_DR20: 0x00000000,
        BKP_DR21: 0x00000000,
        BKP_DR22: 0x00000000,
        BKP_DR23: 0x00000000,
        BKP_DR24: 0x00000000,
        BKP_DR25: 0x00000000,
        BKP_DR26: 0x00000000,
        BKP_DR27: 0x00000000,
        BKP_DR28: 0x00000000,
        BKP_DR29: 0x00000000,
        BKP_DR30: 0x00000000,
        BKP_DR31: 0x00000000,
        RTCCR: 0x00000000,
        CR: 0x00000000,
        CSR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut BKP_TAKEN: bool = false;

    /// Safe access to BKP
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
            if BKP_TAKEN {
                None
            } else {
                BKP_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to BKP
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if BKP_TAKEN && inst.addr == INSTANCE.addr {
                BKP_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal BKP
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        BKP_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to BKP
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const BKP: *const RegisterBlock = 0x40006c04 as *const _;
