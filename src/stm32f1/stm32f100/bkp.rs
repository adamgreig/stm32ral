#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Backup registers

#[cfg(not(feature = "nosync"))]
pub use stm32f1::peripherals::bkp::Instance;
pub use stm32f1::peripherals::bkp::{RegisterBlock, ResetValues};
pub use stm32f1::peripherals::bkp::{
    CR, CSR, DR1, DR10, DR11, DR12, DR13, DR14, DR15, DR16, DR17, DR18, DR19, DR2, DR20, DR21,
    DR22, DR23, DR24, DR25, DR26, DR27, DR28, DR29, DR3, DR30, DR31, DR32, DR33, DR34, DR35, DR36,
    DR37, DR38, DR39, DR4, DR40, DR41, DR42, DR5, DR6, DR7, DR8, DR9, RTCCR,
};

/// Access functions for the BKP peripheral instance
pub mod BKP {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

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
        DR1: 0x00000000,
        DR2: 0x00000000,
        DR3: 0x00000000,
        DR4: 0x00000000,
        DR5: 0x00000000,
        DR6: 0x00000000,
        DR7: 0x00000000,
        DR8: 0x00000000,
        DR9: 0x00000000,
        DR10: 0x00000000,
        DR11: 0x00000000,
        DR12: 0x00000000,
        DR13: 0x00000000,
        DR14: 0x00000000,
        DR15: 0x00000000,
        DR16: 0x00000000,
        DR17: 0x00000000,
        DR18: 0x00000000,
        DR19: 0x00000000,
        DR20: 0x00000000,
        DR21: 0x00000000,
        DR22: 0x00000000,
        DR23: 0x00000000,
        DR24: 0x00000000,
        DR25: 0x00000000,
        DR26: 0x00000000,
        DR27: 0x00000000,
        DR28: 0x00000000,
        DR29: 0x00000000,
        DR30: 0x00000000,
        DR31: 0x00000000,
        DR32: 0x00000000,
        DR33: 0x00000000,
        DR34: 0x00000000,
        DR35: 0x00000000,
        DR36: 0x00000000,
        DR37: 0x00000000,
        DR38: 0x00000000,
        DR39: 0x00000000,
        DR40: 0x00000000,
        DR41: 0x00000000,
        DR42: 0x00000000,
        RTCCR: 0x00000000,
        CR: 0x00000000,
        CSR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
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
