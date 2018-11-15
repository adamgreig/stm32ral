#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DSI Host
//!
//! Used by: stm32f7x7, stm32f7x9

#[cfg(not(feature = "nosync"))]
pub use stm32f7::peripherals::dsi::Instance;
pub use stm32f7::peripherals::dsi::{RegisterBlock, ResetValues};
pub use stm32f7::peripherals::dsi::{
    DSI_CCR, DSI_CLCR, DSI_CLTCR, DSI_CMCR, DSI_CR, DSI_DLTCR, DSI_FIR0, DSI_FIR1, DSI_GHCR,
    DSI_GPDR, DSI_GPSR, DSI_GVCIDR, DSI_IER0, DSI_IER1, DSI_ISR0, DSI_ISR1, DSI_LCCCR, DSI_LCCR,
    DSI_LCOLCR, DSI_LCVCIDR, DSI_LPCR, DSI_LPMCCR, DSI_LPMCR, DSI_LVCIDR, DSI_MCR, DSI_PCONFR,
    DSI_PCR, DSI_PCTLR, DSI_PSR, DSI_PTTCR, DSI_PUCR, DSI_TCCR0, DSI_TCCR1, DSI_TCCR2, DSI_TCCR3,
    DSI_TCCR4, DSI_TCCR5, DSI_VCCCR, DSI_VCCR, DSI_VHBPCCR, DSI_VHBPCR, DSI_VHSACCR, DSI_VHSACR,
    DSI_VLCCR, DSI_VLCR, DSI_VMCCR, DSI_VMCR, DSI_VNPCCR, DSI_VNPCR, DSI_VPCCR, DSI_VPCR, DSI_VR,
    DSI_VSCR, DSI_VVACCR, DSI_VVACR, DSI_VVBPCCR, DSI_VVBPCR, DSI_VVFPCCR, DSI_VVFPCR, DSI_VVSACCR,
    DSI_VVSACR, DSI_WCFGR, DSI_WCR, DSI_WIER, DSI_WIFCR, DSI_WISR, DSI_WPCR1, DSI_WPCR2, DSI_WPCR3,
    DSI_WPCR4, DSI_WPCR5, DSI_WRPCR,
};

/// Access functions for the DSI peripheral instance
pub mod DSI {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40016c00,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DSI
    pub const reset: ResetValues = ResetValues {
        DSI_VR: 0x3133302A,
        DSI_CR: 0x00000000,
        DSI_CCR: 0x00000000,
        DSI_LVCIDR: 0x00000000,
        DSI_LCOLCR: 0x00000000,
        DSI_LPCR: 0x00000000,
        DSI_LPMCR: 0x00000000,
        DSI_PCR: 0x00000000,
        DSI_GVCIDR: 0x00000000,
        DSI_MCR: 0x00000000,
        DSI_VMCR: 0x00000000,
        DSI_VPCR: 0x00000000,
        DSI_VCCR: 0x00000000,
        DSI_VNPCR: 0x00000000,
        DSI_VHSACR: 0x00000000,
        DSI_VHBPCR: 0x00000000,
        DSI_VLCR: 0x00000000,
        DSI_VVSACR: 0x00000000,
        DSI_VVBPCR: 0x00000000,
        DSI_VVFPCR: 0x00000000,
        DSI_VVACR: 0x00000000,
        DSI_LCCR: 0x00000000,
        DSI_CMCR: 0x00000000,
        DSI_GHCR: 0x00000000,
        DSI_GPDR: 0x00000000,
        DSI_GPSR: 0x00000000,
        DSI_TCCR0: 0x00000000,
        DSI_TCCR1: 0x00000000,
        DSI_TCCR2: 0x00000000,
        DSI_TCCR3: 0x00000000,
        DSI_TCCR4: 0x00000000,
        DSI_TCCR5: 0x00000000,
        DSI_CLCR: 0x00000000,
        DSI_CLTCR: 0x00000000,
        DSI_DLTCR: 0x00000000,
        DSI_PCTLR: 0x00000000,
        DSI_PCONFR: 0x00000000,
        DSI_PUCR: 0x00000000,
        DSI_PTTCR: 0x00000000,
        DSI_PSR: 0x00000000,
        DSI_ISR0: 0x00000000,
        DSI_ISR1: 0x00000000,
        DSI_IER0: 0x00000000,
        DSI_IER1: 0x00000000,
        DSI_FIR0: 0x00000000,
        DSI_FIR1: 0x00000000,
        DSI_VSCR: 0x00000000,
        DSI_LCVCIDR: 0x00000000,
        DSI_LCCCR: 0x00000000,
        DSI_LPMCCR: 0x00000000,
        DSI_VMCCR: 0x00000000,
        DSI_VPCCR: 0x00000000,
        DSI_VCCCR: 0x00000000,
        DSI_VNPCCR: 0x00000000,
        DSI_VHSACCR: 0x00000000,
        DSI_VHBPCCR: 0x00000000,
        DSI_VLCCR: 0x00000000,
        DSI_VVSACCR: 0x00000000,
        DSI_VVBPCCR: 0x00000000,
        DSI_VVFPCCR: 0x00000000,
        DSI_VVACCR: 0x00000000,
        DSI_WCFGR: 0x00000000,
        DSI_WCR: 0x00000000,
        DSI_WIER: 0x00000000,
        DSI_WISR: 0x00000000,
        DSI_WIFCR: 0x00000000,
        DSI_WPCR1: 0x00000000,
        DSI_WPCR2: 0x00000000,
        DSI_WPCR3: 0x00000000,
        DSI_WPCR4: 0x3133302A,
        DSI_WPCR5: 0x00000000,
        DSI_WRPCR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[no_mangle]
    static mut DSI_TAKEN: bool = false;

    /// Safe access to DSI
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
            if DSI_TAKEN {
                None
            } else {
                DSI_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to DSI
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if DSI_TAKEN && inst.addr == INSTANCE.addr {
                DSI_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }
}

/// Raw pointer to DSI
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DSI: *const RegisterBlock = 0x40016c00 as *const _;
