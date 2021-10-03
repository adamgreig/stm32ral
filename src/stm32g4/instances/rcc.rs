#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Reset and clock control
//!
//! Used by: stm32g431, stm32g441, stm32g471, stm32g473, stm32g474, stm32g483, stm32g484, stm32g491

#[cfg(not(feature = "nosync"))]
pub use crate::stm32g4::peripherals::rcc::Instance;
pub use crate::stm32g4::peripherals::rcc::{RegisterBlock, ResetValues};
pub use crate::stm32g4::peripherals::rcc::{
    AHB1ENR, AHB1RSTR, AHB1SMENR, AHB2ENR, AHB2RSTR, AHB2SMENR, AHB3ENR, AHB3RSTR, AHB3SMENR,
    APB1ENR1, APB1ENR2, APB1RSTR1, APB1RSTR2, APB1SMENR1, APB1SMENR2, APB2ENR, APB2RSTR, APB2SMENR,
    BDCR, CCIPR, CCIPR2, CFGR, CICR, CIER, CIFR, CR, CRRCR, CSR, ICSCR, PLLCFGR,
};

/// Access functions for the RCC peripheral instance
pub mod RCC {
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
        ICSCR: 0x40000000,
        CFGR: 0x00000005,
        PLLCFGR: 0x00001000,
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
        AHB1SMENR: 0x0000130F,
        AHB2SMENR: 0x050F667F,
        AHB3SMENR: 0x00000101,
        APB1SMENR1: 0xD2FECD3F,
        APB1SMENR2: 0x00000103,
        APB2SMENR: 0x0437F801,
        CCIPR: 0x00000000,
        BDCR: 0x00000000,
        CSR: 0x0C000000,
        CRRCR: 0x00000000,
        CCIPR2: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
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

    /// Unsafely steal RCC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        RCC_TAKEN = true;
        INSTANCE
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
