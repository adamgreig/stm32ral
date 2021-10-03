#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Digital filter for sigma delta modulators

#[cfg(not(feature = "nosync"))]
pub use crate::stm32h7::peripherals::dfsdm_v1::Instance;
pub use crate::stm32h7::peripherals::dfsdm_v1::{RegisterBlock, ResetValues};
pub use crate::stm32h7::peripherals::dfsdm_v1::{
    AWCFR0, AWCFR1, AWCFR2, AWCFR3, AWHTR0, AWHTR1, AWHTR2, AWHTR3, AWLTR0, AWLTR1, AWLTR2, AWLTR3,
    AWSCDR0, AWSCDR1, AWSCDR2, AWSCDR3, AWSCDR4, AWSCDR5, AWSCDR6, AWSCDR7, AWSR0, AWSR1, AWSR2,
    AWSR3, CFGR10, CFGR11, CFGR12, CFGR13, CFGR14, CFGR15, CFGR16, CFGR17, CFGR20, CFGR21, CFGR22,
    CFGR23, CFGR24, CFGR25, CFGR26, CFGR27, CR10, CR11, CR12, CR13, CR20, CR21, CR22, CR23,
    DATINR0, DATINR1, DATINR2, DATINR3, DATINR4, DATINR5, DATINR6, DATINR7, EXMAX0, EXMAX1, EXMAX2,
    EXMAX3, EXMIN0, EXMIN1, EXMIN2, EXMIN3, FCR0, FCR1, FCR2, FCR3, FLTCNVTIMR0, FLTCNVTIMR1,
    FLTCNVTIMR2, FLTCNVTIMR3, ICR0, ICR1, ICR2, ICR3, ISR0, ISR1, ISR2, ISR3, JCHGR0, JCHGR1,
    JCHGR2, JCHGR3, JDATAR0, JDATAR1, JDATAR2, JDATAR3, RDATAR0, RDATAR1, RDATAR2, RDATAR3, WDATR0,
    WDATR1, WDATR2, WDATR3, WDATR4, WDATR5, WDATR6, WDATR7,
};

/// Access functions for the DFSDM peripheral instance
pub mod DFSDM {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40017800,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DFSDM
    pub const reset: ResetValues = ResetValues {
        CFGR10: 0x00000000,
        CFGR20: 0x00000000,
        AWSCDR0: 0x00000000,
        WDATR0: 0x00000000,
        DATINR0: 0x00000000,
        CFGR11: 0x00000000,
        CFGR21: 0x00000000,
        AWSCDR1: 0x00000000,
        WDATR1: 0x00000000,
        DATINR1: 0x00000000,
        CFGR12: 0x00000000,
        CFGR22: 0x00000000,
        AWSCDR2: 0x00000000,
        WDATR2: 0x00000000,
        DATINR2: 0x00000000,
        CFGR13: 0x00000000,
        CFGR23: 0x00000000,
        AWSCDR3: 0x00000000,
        WDATR3: 0x00000000,
        DATINR3: 0x00000000,
        CFGR14: 0x00000000,
        CFGR24: 0x00000000,
        AWSCDR4: 0x00000000,
        WDATR4: 0x00000000,
        DATINR4: 0x00000000,
        CFGR15: 0x00000000,
        CFGR25: 0x00000000,
        AWSCDR5: 0x00000000,
        WDATR5: 0x00000000,
        DATINR5: 0x00000000,
        CFGR16: 0x00000000,
        CFGR26: 0x00000000,
        AWSCDR6: 0x00000000,
        WDATR6: 0x00000000,
        DATINR6: 0x00000000,
        CFGR17: 0x00000000,
        CFGR27: 0x00000000,
        AWSCDR7: 0x00000000,
        WDATR7: 0x00000000,
        DATINR7: 0x00000000,
        CR10: 0x00000000,
        CR20: 0x00000000,
        ISR0: 0x00FF0000,
        ICR0: 0x00000000,
        JCHGR0: 0x00000001,
        FCR0: 0x00000000,
        JDATAR0: 0x00000000,
        RDATAR0: 0x00000000,
        AWHTR0: 0x00000000,
        AWLTR0: 0x00000000,
        AWSR0: 0x00000000,
        AWCFR0: 0x00000000,
        EXMAX0: 0x80000000,
        EXMIN0: 0x7FFFFF00,
        FLTCNVTIMR0: 0x00000000,
        CR11: 0x00000000,
        CR21: 0x00000000,
        ISR1: 0x00FF0000,
        ICR1: 0x00000000,
        JCHGR1: 0x00000001,
        FCR1: 0x00000000,
        JDATAR1: 0x00000000,
        RDATAR1: 0x00000000,
        AWHTR1: 0x00000000,
        AWLTR1: 0x00000000,
        AWSR1: 0x00000000,
        AWCFR1: 0x00000000,
        EXMAX1: 0x80000000,
        EXMIN1: 0x7FFFFF00,
        FLTCNVTIMR1: 0x00000000,
        CR12: 0x00000000,
        CR22: 0x00000000,
        ISR2: 0x00FF0000,
        ICR2: 0x00000000,
        JCHGR2: 0x00000001,
        FCR2: 0x00000000,
        JDATAR2: 0x00000000,
        RDATAR2: 0x00000000,
        AWHTR2: 0x00000000,
        AWLTR2: 0x00000000,
        AWSR2: 0x00000000,
        AWCFR2: 0x00000000,
        EXMAX2: 0x80000000,
        EXMIN2: 0x7FFFFF00,
        FLTCNVTIMR2: 0x00000000,
        CR13: 0x00000000,
        CR23: 0x00000000,
        ISR3: 0x00FF0000,
        ICR3: 0x00000000,
        JCHGR3: 0x00000001,
        FCR3: 0x00000000,
        JDATAR3: 0x00000000,
        RDATAR3: 0x00000000,
        AWHTR3: 0x00000000,
        AWLTR3: 0x00000000,
        AWSR3: 0x00000000,
        AWCFR3: 0x00000000,
        EXMAX3: 0x80000000,
        EXMIN3: 0x7FFFFF00,
        FLTCNVTIMR3: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut DFSDM_TAKEN: bool = false;

    /// Safe access to DFSDM
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
            if DFSDM_TAKEN {
                None
            } else {
                DFSDM_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to DFSDM
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if DFSDM_TAKEN && inst.addr == INSTANCE.addr {
                DFSDM_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal DFSDM
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        DFSDM_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to DFSDM
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DFSDM: *const RegisterBlock = 0x40017800 as *const _;
