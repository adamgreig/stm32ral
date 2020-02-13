#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Reset and clock control
//!
//! Used by: stm32h747cm4, stm32h747cm7

#[cfg(not(feature = "nosync"))]
pub use crate::stm32h7::peripherals::rcc_v3::Instance;
pub use crate::stm32h7::peripherals::rcc_v3::{RegisterBlock, ResetValues};
pub use crate::stm32h7::peripherals::rcc_v3::{
    AHB1ENR, AHB1LPENR, AHB1RSTR, AHB2ENR, AHB2LPENR, AHB2RSTR, AHB3ENR, AHB3LPENR, AHB3RSTR,
    AHB4ENR, AHB4LPENR, AHB4RSTR, APB1HENR, APB1HLPENR, APB1HRSTR, APB1LENR, APB1LLPENR, APB1LRSTR,
    APB2ENR, APB2LPENR, APB2RSTR, APB3ENR, APB3LPENR, APB3RSTR, APB4ENR, APB4LPENR, APB4RSTR, BDCR,
    C1_AHB1ENR, C1_AHB1LPENR, C1_AHB2ENR, C1_AHB2LPENR, C1_AHB3ENR, C1_AHB3LPENR, C1_AHB4ENR,
    C1_AHB4LPENR, C1_APB1HENR, C1_APB1HLPENR, C1_APB1LENR, C1_APB1LLPENR, C1_APB2ENR, C1_APB2LPENR,
    C1_APB3ENR, C1_APB3LPENR, C1_APB4ENR, C1_APB4LPENR, C1_RSR, CFGR, CICR, CIER, CIFR, CR, CRRCR,
    CSICFGR, CSR, D1CCIPR, D1CFGR, D2CCIP1R, D2CCIP2R, D2CFGR, D3AMR, D3CCIPR, D3CFGR, GCR,
    HSICFGR, PLL1DIVR, PLL1FRACR, PLL2DIVR, PLL2FRACR, PLL3DIVR, PLL3FRACR, PLLCFGR, PLLCKSELR,
    RSR,
};

/// Access functions for the RCC peripheral instance
pub mod RCC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x58024400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in RCC
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000083,
        CRRCR: 0x00000000,
        CFGR: 0x00000000,
        D1CFGR: 0x00000000,
        D2CFGR: 0x00000000,
        D3CFGR: 0x00000000,
        PLLCKSELR: 0x02020200,
        PLLCFGR: 0x01FF0000,
        PLL1DIVR: 0x01010280,
        PLL1FRACR: 0x00000000,
        PLL2DIVR: 0x01010280,
        PLL2FRACR: 0x00000000,
        PLL3DIVR: 0x01010280,
        PLL3FRACR: 0x00000000,
        D1CCIPR: 0x00000000,
        D2CCIP1R: 0x00000000,
        D2CCIP2R: 0x00000000,
        D3CCIPR: 0x00000000,
        CIER: 0x00000000,
        CIFR: 0x00000000,
        CICR: 0x00000000,
        BDCR: 0x00000000,
        CSR: 0x00000000,
        AHB3RSTR: 0x00000000,
        AHB1RSTR: 0x00000000,
        AHB2RSTR: 0x00000000,
        AHB4RSTR: 0x00000000,
        APB3RSTR: 0x00000000,
        APB1LRSTR: 0x00000000,
        APB1HRSTR: 0x00000000,
        APB2RSTR: 0x00000000,
        APB4RSTR: 0x00000000,
        GCR: 0x00000000,
        D3AMR: 0x00000000,
        RSR: 0x00000000,
        C1_RSR: 0x00000000,
        C1_AHB3ENR: 0x00000000,
        AHB3ENR: 0x00000000,
        AHB1ENR: 0x00000000,
        C1_AHB1ENR: 0x00000000,
        C1_AHB2ENR: 0x00000000,
        AHB2ENR: 0x00000000,
        AHB4ENR: 0x00000000,
        C1_AHB4ENR: 0x00000000,
        C1_APB3ENR: 0x00000000,
        APB3ENR: 0x00000000,
        APB1LENR: 0x00000000,
        C1_APB1LENR: 0x00000000,
        APB1HENR: 0x00000000,
        C1_APB1HENR: 0x00000000,
        C1_APB2ENR: 0x00000000,
        APB2ENR: 0x00000000,
        APB4ENR: 0x00000000,
        C1_APB4ENR: 0x00000000,
        C1_AHB3LPENR: 0x00000000,
        AHB3LPENR: 0x00000000,
        AHB1LPENR: 0x00000000,
        C1_AHB1LPENR: 0x00000000,
        C1_AHB2LPENR: 0x00000000,
        AHB2LPENR: 0x00000000,
        AHB4LPENR: 0x00000000,
        C1_AHB4LPENR: 0x00000000,
        C1_APB3LPENR: 0x00000000,
        APB3LPENR: 0x00000000,
        APB1LLPENR: 0x00000000,
        C1_APB1LLPENR: 0x00000000,
        C1_APB1HLPENR: 0x00000000,
        APB1HLPENR: 0x00000000,
        APB2LPENR: 0x00000000,
        C1_APB2LPENR: 0x00000000,
        C1_APB4LPENR: 0x00000000,
        APB4LPENR: 0x00000000,
        HSICFGR: 0x00000000,
        CSICFGR: 0x00000000,
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
pub const RCC: *const RegisterBlock = 0x58024400 as *const _;
