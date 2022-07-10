#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! OctoSPI
//!
//! Used by: stm32l4r5, stm32l4r9

#[cfg(not(feature = "nosync"))]
pub use crate::stm32l4::peripherals::octospi::Instance;
pub use crate::stm32l4::peripherals::octospi::{RegisterBlock, ResetValues};
pub use crate::stm32l4::peripherals::octospi::{
    ABR, AR, CCR, CR, DCR1, DCR2, DCR3, DLR, DR, FCR, HLCR, HWCFGR, ID, IR, LPTR, MID, PIR, PSMAR,
    PSMKR, SR, TCR, VER, WABR, WCCR, WIR, WTCR,
};

/// Access functions for the OCTOSPI1 peripheral instance
pub mod OCTOSPI1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0xa0001000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in OCTOSPI1
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000000,
        DCR1: 0x00000000,
        DCR2: 0x00000000,
        DCR3: 0x00000000,
        SR: 0x00000000,
        FCR: 0x00000000,
        DLR: 0x00000000,
        AR: 0x00000000,
        DR: 0x00000000,
        PSMKR: 0x00000000,
        PSMAR: 0x00000000,
        PIR: 0x00000000,
        CCR: 0x00000000,
        TCR: 0x00000000,
        IR: 0x00000000,
        ABR: 0x00000000,
        LPTR: 0x00000000,
        WCCR: 0x00000000,
        WTCR: 0x00000000,
        WIR: 0x00000000,
        WABR: 0x00000000,
        HLCR: 0x00000000,
        HWCFGR: 0x11300080,
        VER: 0x00000010,
        ID: 0x00140041,
        MID: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut OCTOSPI1_TAKEN: bool = false;

    /// Safe access to OCTOSPI1
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
            if OCTOSPI1_TAKEN {
                None
            } else {
                OCTOSPI1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to OCTOSPI1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if OCTOSPI1_TAKEN && inst.addr == INSTANCE.addr {
                OCTOSPI1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal OCTOSPI1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        OCTOSPI1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to OCTOSPI1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const OCTOSPI1: *const RegisterBlock = 0xa0001000 as *const _;

/// Access functions for the OCTOSPI2 peripheral instance
pub mod OCTOSPI2 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0xa0001400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in OCTOSPI2
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000000,
        DCR1: 0x00000000,
        DCR2: 0x00000000,
        DCR3: 0x00000000,
        SR: 0x00000000,
        FCR: 0x00000000,
        DLR: 0x00000000,
        AR: 0x00000000,
        DR: 0x00000000,
        PSMKR: 0x00000000,
        PSMAR: 0x00000000,
        PIR: 0x00000000,
        CCR: 0x00000000,
        TCR: 0x00000000,
        IR: 0x00000000,
        ABR: 0x00000000,
        LPTR: 0x00000000,
        WCCR: 0x00000000,
        WTCR: 0x00000000,
        WIR: 0x00000000,
        WABR: 0x00000000,
        HLCR: 0x00000000,
        HWCFGR: 0x11300080,
        VER: 0x00000010,
        ID: 0x00140041,
        MID: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut OCTOSPI2_TAKEN: bool = false;

    /// Safe access to OCTOSPI2
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
            if OCTOSPI2_TAKEN {
                None
            } else {
                OCTOSPI2_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to OCTOSPI2
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if OCTOSPI2_TAKEN && inst.addr == INSTANCE.addr {
                OCTOSPI2_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal OCTOSPI2
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        OCTOSPI2_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to OCTOSPI2
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const OCTOSPI2: *const RegisterBlock = 0xa0001400 as *const _;
