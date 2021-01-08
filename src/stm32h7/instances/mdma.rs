#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! MDMA
//!
//! Used by: stm32h743, stm32h743v, stm32h747cm4, stm32h747cm7, stm32h753, stm32h753v, stm32h7b3

#[cfg(not(feature = "nosync"))]
pub use crate::stm32h7::peripherals::mdma::Instance;
pub use crate::stm32h7::peripherals::mdma::{RegisterBlock, ResetValues};
pub use crate::stm32h7::peripherals::mdma::{
    BNDTR0, BNDTR1, BNDTR10, BNDTR11, BNDTR12, BNDTR13, BNDTR14, BNDTR15, BNDTR2, BNDTR3, BNDTR4,
    BNDTR5, BNDTR6, BNDTR7, BNDTR8, BNDTR9, BRUR0, BRUR1, BRUR10, BRUR11, BRUR12, BRUR13, BRUR14,
    BRUR15, BRUR2, BRUR3, BRUR4, BRUR5, BRUR6, BRUR7, BRUR8, BRUR9, CR0, CR1, CR10, CR11, CR12,
    CR13, CR14, CR15, CR2, CR3, CR4, CR5, CR6, CR7, CR8, CR9, DAR0, DAR1, DAR10, DAR11, DAR12,
    DAR13, DAR14, DAR15, DAR2, DAR3, DAR4, DAR5, DAR6, DAR7, DAR8, DAR9, ESR0, ESR1, ESR10, ESR11,
    ESR12, ESR13, ESR14, ESR15, ESR2, ESR3, ESR4, ESR5, ESR6, ESR7, ESR8, ESR9, GISR0, IFCR0,
    IFCR1, IFCR10, IFCR11, IFCR12, IFCR13, IFCR14, IFCR15, IFCR2, IFCR3, IFCR4, IFCR5, IFCR6,
    IFCR7, IFCR8, IFCR9, ISR0, ISR1, ISR10, ISR11, ISR12, ISR13, ISR14, ISR15, ISR2, ISR3, ISR4,
    ISR5, ISR6, ISR7, ISR8, ISR9, LAR0, LAR1, LAR10, LAR11, LAR12, LAR13, LAR14, LAR15, LAR2, LAR3,
    LAR4, LAR5, LAR6, LAR7, LAR8, LAR9, MAR0, MAR1, MAR10, MAR11, MAR12, MAR13, MAR14, MAR15, MAR2,
    MAR3, MAR4, MAR5, MAR6, MAR7, MAR8, MAR9, MDR0, MDR1, MDR10, MDR11, MDR12, MDR13, MDR14, MDR15,
    MDR2, MDR3, MDR4, MDR5, MDR6, MDR7, MDR8, MDR9, SAR0, SAR1, SAR10, SAR11, SAR12, SAR13, SAR14,
    SAR15, SAR2, SAR3, SAR4, SAR5, SAR6, SAR7, SAR8, SAR9, TBR0, TBR1, TBR10, TBR11, TBR12, TBR13,
    TBR14, TBR15, TBR2, TBR3, TBR4, TBR5, TBR6, TBR7, TBR8, TBR9, TCR0, TCR1, TCR10, TCR11, TCR12,
    TCR13, TCR14, TCR15, TCR2, TCR3, TCR4, TCR5, TCR6, TCR7, TCR8, TCR9,
};

/// Access functions for the MDMA peripheral instance
pub mod MDMA {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x52000000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in MDMA
    pub const reset: ResetValues = ResetValues {
        GISR0: 0x00000000,
        ISR0: 0x00000000,
        IFCR0: 0x00000000,
        ESR0: 0x00000000,
        CR0: 0x00000000,
        TCR0: 0x00000000,
        BNDTR0: 0x00000000,
        SAR0: 0x00000000,
        DAR0: 0x00000000,
        BRUR0: 0x00000000,
        LAR0: 0x00000000,
        TBR0: 0x00000000,
        MAR0: 0x00000000,
        MDR0: 0x00000000,
        ISR1: 0x00000000,
        IFCR1: 0x00000000,
        ESR1: 0x00000000,
        CR1: 0x00000000,
        TCR1: 0x00000000,
        BNDTR1: 0x00000000,
        SAR1: 0x00000000,
        DAR1: 0x00000000,
        BRUR1: 0x00000000,
        LAR1: 0x00000000,
        TBR1: 0x00000000,
        MAR1: 0x00000000,
        MDR1: 0x00000000,
        ISR2: 0x00000000,
        IFCR2: 0x00000000,
        ESR2: 0x00000000,
        CR2: 0x00000000,
        TCR2: 0x00000000,
        BNDTR2: 0x00000000,
        SAR2: 0x00000000,
        DAR2: 0x00000000,
        BRUR2: 0x00000000,
        LAR2: 0x00000000,
        TBR2: 0x00000000,
        MAR2: 0x00000000,
        MDR2: 0x00000000,
        ISR3: 0x00000000,
        IFCR3: 0x00000000,
        ESR3: 0x00000000,
        CR3: 0x00000000,
        TCR3: 0x00000000,
        BNDTR3: 0x00000000,
        SAR3: 0x00000000,
        DAR3: 0x00000000,
        BRUR3: 0x00000000,
        LAR3: 0x00000000,
        TBR3: 0x00000000,
        MAR3: 0x00000000,
        MDR3: 0x00000000,
        ISR4: 0x00000000,
        IFCR4: 0x00000000,
        ESR4: 0x00000000,
        CR4: 0x00000000,
        TCR4: 0x00000000,
        BNDTR4: 0x00000000,
        SAR4: 0x00000000,
        DAR4: 0x00000000,
        BRUR4: 0x00000000,
        LAR4: 0x00000000,
        TBR4: 0x00000000,
        MAR4: 0x00000000,
        MDR4: 0x00000000,
        ISR5: 0x00000000,
        IFCR5: 0x00000000,
        ESR5: 0x00000000,
        CR5: 0x00000000,
        TCR5: 0x00000000,
        BNDTR5: 0x00000000,
        SAR5: 0x00000000,
        DAR5: 0x00000000,
        BRUR5: 0x00000000,
        LAR5: 0x00000000,
        TBR5: 0x00000000,
        MAR5: 0x00000000,
        MDR5: 0x00000000,
        ISR6: 0x00000000,
        IFCR6: 0x00000000,
        ESR6: 0x00000000,
        CR6: 0x00000000,
        TCR6: 0x00000000,
        BNDTR6: 0x00000000,
        SAR6: 0x00000000,
        DAR6: 0x00000000,
        BRUR6: 0x00000000,
        LAR6: 0x00000000,
        TBR6: 0x00000000,
        MAR6: 0x00000000,
        MDR6: 0x00000000,
        ISR7: 0x00000000,
        IFCR7: 0x00000000,
        ESR7: 0x00000000,
        CR7: 0x00000000,
        TCR7: 0x00000000,
        BNDTR7: 0x00000000,
        SAR7: 0x00000000,
        DAR7: 0x00000000,
        BRUR7: 0x00000000,
        LAR7: 0x00000000,
        TBR7: 0x00000000,
        MAR7: 0x00000000,
        MDR7: 0x00000000,
        ISR8: 0x00000000,
        IFCR8: 0x00000000,
        ESR8: 0x00000000,
        CR8: 0x00000000,
        TCR8: 0x00000000,
        BNDTR8: 0x00000000,
        SAR8: 0x00000000,
        DAR8: 0x00000000,
        BRUR8: 0x00000000,
        LAR8: 0x00000000,
        TBR8: 0x00000000,
        MAR8: 0x00000000,
        MDR8: 0x00000000,
        ISR9: 0x00000000,
        IFCR9: 0x00000000,
        ESR9: 0x00000000,
        CR9: 0x00000000,
        TCR9: 0x00000000,
        BNDTR9: 0x00000000,
        SAR9: 0x00000000,
        DAR9: 0x00000000,
        BRUR9: 0x00000000,
        LAR9: 0x00000000,
        TBR9: 0x00000000,
        MAR9: 0x00000000,
        MDR9: 0x00000000,
        ISR10: 0x00000000,
        IFCR10: 0x00000000,
        ESR10: 0x00000000,
        CR10: 0x00000000,
        TCR10: 0x00000000,
        BNDTR10: 0x00000000,
        SAR10: 0x00000000,
        DAR10: 0x00000000,
        BRUR10: 0x00000000,
        LAR10: 0x00000000,
        TBR10: 0x00000000,
        MAR10: 0x00000000,
        MDR10: 0x00000000,
        ISR11: 0x00000000,
        IFCR11: 0x00000000,
        ESR11: 0x00000000,
        CR11: 0x00000000,
        TCR11: 0x00000000,
        BNDTR11: 0x00000000,
        SAR11: 0x00000000,
        DAR11: 0x00000000,
        BRUR11: 0x00000000,
        LAR11: 0x00000000,
        TBR11: 0x00000000,
        MAR11: 0x00000000,
        MDR11: 0x00000000,
        ISR12: 0x00000000,
        IFCR12: 0x00000000,
        ESR12: 0x00000000,
        CR12: 0x00000000,
        TCR12: 0x00000000,
        BNDTR12: 0x00000000,
        SAR12: 0x00000000,
        DAR12: 0x00000000,
        BRUR12: 0x00000000,
        LAR12: 0x00000000,
        TBR12: 0x00000000,
        MAR12: 0x00000000,
        MDR12: 0x00000000,
        ISR13: 0x00000000,
        IFCR13: 0x00000000,
        ESR13: 0x00000000,
        CR13: 0x00000000,
        TCR13: 0x00000000,
        BNDTR13: 0x00000000,
        SAR13: 0x00000000,
        DAR13: 0x00000000,
        BRUR13: 0x00000000,
        LAR13: 0x00000000,
        TBR13: 0x00000000,
        MAR13: 0x00000000,
        MDR13: 0x00000000,
        ISR14: 0x00000000,
        IFCR14: 0x00000000,
        ESR14: 0x00000000,
        CR14: 0x00000000,
        TCR14: 0x00000000,
        BNDTR14: 0x00000000,
        SAR14: 0x00000000,
        DAR14: 0x00000000,
        BRUR14: 0x00000000,
        LAR14: 0x00000000,
        TBR14: 0x00000000,
        MAR14: 0x00000000,
        MDR14: 0x00000000,
        ISR15: 0x00000000,
        IFCR15: 0x00000000,
        ESR15: 0x00000000,
        CR15: 0x00000000,
        TCR15: 0x00000000,
        BNDTR15: 0x00000000,
        SAR15: 0x00000000,
        DAR15: 0x00000000,
        BRUR15: 0x00000000,
        LAR15: 0x00000000,
        TBR15: 0x00000000,
        MAR15: 0x00000000,
        MDR15: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut MDMA_TAKEN: bool = false;

    /// Safe access to MDMA
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
            if MDMA_TAKEN {
                None
            } else {
                MDMA_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to MDMA
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if MDMA_TAKEN && inst.addr == INSTANCE.addr {
                MDMA_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal MDMA
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        MDMA_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to MDMA
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const MDMA: *const RegisterBlock = 0x52000000 as *const _;
