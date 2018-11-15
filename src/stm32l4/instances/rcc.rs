#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Reset and clock control
//!
//! Used by: stm32l4x1, stm32l4x2

#[cfg(not(feature = "nosync"))]
pub use stm32l4::peripherals::rcc::Instance;
pub use stm32l4::peripherals::rcc::{RegisterBlock, ResetValues};
pub use stm32l4::peripherals::rcc::{
    AHB1ENR, AHB1RSTR, AHB1SMENR, AHB2ENR, AHB2RSTR, AHB2SMENR, AHB3ENR, AHB3RSTR, AHB3SMENR,
    APB1ENR1, APB1ENR2, APB1RSTR1, APB1RSTR2, APB1SMENR1, APB1SMENR2, APB2ENR, APB2RSTR, APB2SMENR,
    BDCR, CCIPR, CFGR, CICR, CIER, CIFR, CR, CRRCR, CSR, ICSCR, PLLCFGR, PLLSAI1CFGR,
};

/// Access functions for the RCC peripheral instance
pub mod RCC {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40021000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in RCC
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000063,
        ICSCR: 0x10000000,
        CFGR: 0x00000000,
        PLLCFGR: 0x00001000,
        PLLSAI1CFGR: 0x00001000,
        CIER: 0x00000000,
        CIFR: 0x00000000,
        CICR: 0x00000000,
        AHB1RSTR: 0x00000000,
        AHB2RSTR: 0x00000000,
        AHB3RSTR: 0x00000000,
        APB1RSTR1: 0x00000000,
        APB1RSTR2: 0x00000000,
        APB2RSTR: 0x00000000,
        AHB1ENR: 0x00000100,
        AHB2ENR: 0x00000000,
        AHB3ENR: 0x00000000,
        APB1ENR1: 0x00000000,
        APB1ENR2: 0x00000000,
        APB2ENR: 0x00000000,
        AHB1SMENR: 0x00011303,
        AHB2SMENR: 0x000532FF,
        AHB3SMENR: 0x00000101,
        APB1SMENR1: 0xF2FECA3F,
        APB1SMENR2: 0x00000025,
        APB2SMENR: 0x01677C01,
        CCIPR: 0x00000000,
        BDCR: 0x00000000,
        CSR: 0x0C000600,
        CRRCR: 0x0C000600,
    };

    #[cfg(not(feature = "nosync"))]
    #[no_mangle]
    static mut RCC_TAKEN: bool = false;

    /// Safe access to RCC
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
            if RCC_TAKEN {
                None
            } else {
                RCC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to RCC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if RCC_TAKEN && inst.addr == INSTANCE.addr {
                RCC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }
}

/// Raw pointer to RCC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const RCC: *const RegisterBlock = 0x40021000 as *const _;
