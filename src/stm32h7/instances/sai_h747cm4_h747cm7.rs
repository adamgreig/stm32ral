#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! SAI
//!
//! Used by: stm32h747cm4, stm32h747cm7

#[cfg(not(feature = "nosync"))]
pub use crate::stm32h7::peripherals::sai_v2::Instance;
pub use crate::stm32h7::peripherals::sai_v2::{RegisterBlock, ResetValues};
pub use crate::stm32h7::peripherals::sai_v2::{
    ACLRFR, ACR1, ACR2, ADR, AFRCR, AIM, ASLOTR, ASR, BCLRFR, BCR1, BCR2, BDR, BFRCR, BIM, BSLOTR,
    BSR, GCR, PDMCR, PDMDLY,
};

/// Access functions for the SAI1 peripheral instance
pub mod SAI1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40015800,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SAI1
    pub const reset: ResetValues = ResetValues {
        GCR: 0x00000000,
        ACR1: 0x00000040,
        ACR2: 0x00000000,
        AFRCR: 0x00000007,
        ASLOTR: 0x00000000,
        AIM: 0x00000000,
        ASR: 0x00000008,
        ACLRFR: 0x00000000,
        ADR: 0x00000000,
        BCR1: 0x00000040,
        BCR2: 0x00000000,
        BFRCR: 0x00000007,
        BSLOTR: 0x00000000,
        BIM: 0x00000000,
        BSR: 0x00000008,
        BCLRFR: 0x00000000,
        BDR: 0x00000000,
        PDMCR: 0x00000000,
        PDMDLY: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SAI1_TAKEN: bool = false;

    /// Safe access to SAI1
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
            if SAI1_TAKEN {
                None
            } else {
                SAI1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SAI1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SAI1_TAKEN && inst.addr == INSTANCE.addr {
                SAI1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SAI1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SAI1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SAI1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SAI1: *const RegisterBlock = 0x40015800 as *const _;

/// Access functions for the SAI2 peripheral instance
pub mod SAI2 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40015c00,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SAI2
    pub const reset: ResetValues = ResetValues {
        GCR: 0x00000000,
        ACR1: 0x00000040,
        ACR2: 0x00000000,
        AFRCR: 0x00000007,
        ASLOTR: 0x00000000,
        AIM: 0x00000000,
        ASR: 0x00000008,
        ACLRFR: 0x00000000,
        ADR: 0x00000000,
        BCR1: 0x00000040,
        BCR2: 0x00000000,
        BFRCR: 0x00000007,
        BSLOTR: 0x00000000,
        BIM: 0x00000000,
        BSR: 0x00000008,
        BCLRFR: 0x00000000,
        BDR: 0x00000000,
        PDMCR: 0x00000000,
        PDMDLY: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SAI2_TAKEN: bool = false;

    /// Safe access to SAI2
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
            if SAI2_TAKEN {
                None
            } else {
                SAI2_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SAI2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SAI2_TAKEN && inst.addr == INSTANCE.addr {
                SAI2_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SAI2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SAI2_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SAI2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SAI2: *const RegisterBlock = 0x40015c00 as *const _;

/// Access functions for the SAI3 peripheral instance
pub mod SAI3 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40016000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SAI3
    pub const reset: ResetValues = ResetValues {
        GCR: 0x00000000,
        ACR1: 0x00000040,
        ACR2: 0x00000000,
        AFRCR: 0x00000007,
        ASLOTR: 0x00000000,
        AIM: 0x00000000,
        ASR: 0x00000008,
        ACLRFR: 0x00000000,
        ADR: 0x00000000,
        BCR1: 0x00000040,
        BCR2: 0x00000000,
        BFRCR: 0x00000007,
        BSLOTR: 0x00000000,
        BIM: 0x00000000,
        BSR: 0x00000008,
        BCLRFR: 0x00000000,
        BDR: 0x00000000,
        PDMCR: 0x00000000,
        PDMDLY: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SAI3_TAKEN: bool = false;

    /// Safe access to SAI3
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
            if SAI3_TAKEN {
                None
            } else {
                SAI3_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SAI3
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SAI3_TAKEN && inst.addr == INSTANCE.addr {
                SAI3_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SAI3
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SAI3_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SAI3
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SAI3: *const RegisterBlock = 0x40016000 as *const _;

/// Access functions for the SAI4 peripheral instance
pub mod SAI4 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x58005400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SAI4
    pub const reset: ResetValues = ResetValues {
        GCR: 0x00000000,
        ACR1: 0x00000040,
        ACR2: 0x00000000,
        AFRCR: 0x00000007,
        ASLOTR: 0x00000000,
        AIM: 0x00000000,
        ASR: 0x00000008,
        ACLRFR: 0x00000000,
        ADR: 0x00000000,
        BCR1: 0x00000040,
        BCR2: 0x00000000,
        BFRCR: 0x00000007,
        BSLOTR: 0x00000000,
        BIM: 0x00000000,
        BSR: 0x00000008,
        BCLRFR: 0x00000000,
        BDR: 0x00000000,
        PDMCR: 0x00000000,
        PDMDLY: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SAI4_TAKEN: bool = false;

    /// Safe access to SAI4
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
            if SAI4_TAKEN {
                None
            } else {
                SAI4_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SAI4
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SAI4_TAKEN && inst.addr == INSTANCE.addr {
                SAI4_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SAI4
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SAI4_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SAI4
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SAI4: *const RegisterBlock = 0x58005400 as *const _;
