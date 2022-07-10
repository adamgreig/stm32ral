#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! MIPI DSI Host
//!
//! Used by: stm32h747cm4, stm32h747cm7

#[cfg(not(feature = "nosync"))]
pub use crate::stm32h7::peripherals::dsihost::Instance;
pub use crate::stm32h7::peripherals::dsihost::{RegisterBlock, ResetValues};
pub use crate::stm32h7::peripherals::dsihost::{
    CCR, CLCR, CLTCR, CMCR, CR, DLTCR, FIR0, FIR1, GHCR, GPDR, GPSR, GVCIDR, IER0, IER1, ISR0,
    ISR1, LCCCR, LCCR, LCOLCR, LCVCIDR, LPCR, LPMCCR, LPMCR, LVCIDR, MCR, PCONFR, PCR, PCTLR, PSR,
    PTTCR, PUCR, TCCR0, TCCR1, TCCR2, TCCR3, TCCR4, TCCR5, VCCCR, VCCR, VHBPCCR, VHBPCR, VHSACCR,
    VHSACR, VLCCR, VLCR, VMCCR, VMCR, VNPCCR, VNPCR, VPCCR, VPCR, VR, VSCR, VVACCR, VVACR, VVBPCR,
    VVFPCCR, VVFPCR, VVPBCCR, VVSACCR, VVSACR, WCFGR, WCR, WIER, WIFCR, WISR, WPCR0, WPCR1, WPCR2,
    WPCR3, WPCR4, WRPCR,
};

/// Access functions for the DSIHOST peripheral instance
pub mod DSIHOST {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50000000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DSIHOST
    pub const reset: ResetValues = ResetValues {
        VR: 0x3133302A,
        CR: 0x00000000,
        CCR: 0x00000000,
        LVCIDR: 0x00000000,
        LCOLCR: 0x00000000,
        LPCR: 0x00000000,
        LPMCR: 0x00000000,
        PCR: 0x00000000,
        GVCIDR: 0x00000000,
        MCR: 0x00000001,
        VMCR: 0x00000000,
        VPCR: 0x00000000,
        VCCR: 0x00000000,
        VNPCR: 0x00000000,
        VHSACR: 0x00000000,
        VHBPCR: 0x00000000,
        VLCR: 0x00000000,
        VVSACR: 0x00000000,
        VVBPCR: 0x00000000,
        VVFPCR: 0x00000000,
        VVACR: 0x00000000,
        LCCR: 0x00000000,
        CMCR: 0x00000000,
        GHCR: 0x00000000,
        GPDR: 0x00000000,
        GPSR: 0x00000015,
        TCCR0: 0x00000000,
        TCCR1: 0x00000000,
        TCCR2: 0x00000000,
        TCCR3: 0x00000000,
        TCCR4: 0x00000000,
        TCCR5: 0x00000000,
        CLCR: 0x00000000,
        CLTCR: 0x00000000,
        DLTCR: 0x00000000,
        PCTLR: 0x00000000,
        PCONFR: 0x00000001,
        PUCR: 0x00000000,
        PTTCR: 0x00000000,
        PSR: 0x00001528,
        ISR0: 0x00000000,
        ISR1: 0x00000000,
        IER0: 0x00000000,
        IER1: 0x00000000,
        FIR0: 0x00000000,
        FIR1: 0x00000000,
        VSCR: 0x00000000,
        LCVCIDR: 0x00000000,
        LCCCR: 0x00000000,
        LPMCCR: 0x00000000,
        VMCCR: 0x00000000,
        VPCCR: 0x00000000,
        VCCCR: 0x00000000,
        VNPCCR: 0x00000000,
        VHSACCR: 0x00000000,
        VHBPCCR: 0x00000000,
        VLCCR: 0x00000000,
        VVSACCR: 0x00000000,
        VVPBCCR: 0x00000000,
        VVFPCCR: 0x00000000,
        VVACCR: 0x00000000,
        WCFGR: 0x00000000,
        WCR: 0x00000000,
        WIER: 0x00000000,
        WISR: 0x00000000,
        WIFCR: 0x00000000,
        WPCR0: 0x00000000,
        WPCR1: 0x00000000,
        WPCR2: 0x00000000,
        WPCR3: 0x00000000,
        WPCR4: 0x00000000,
        WRPCR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut DSIHOST_TAKEN: bool = false;

    /// Safe access to DSIHOST
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
            if DSIHOST_TAKEN {
                None
            } else {
                DSIHOST_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to DSIHOST
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if DSIHOST_TAKEN && inst.addr == INSTANCE.addr {
                DSIHOST_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal DSIHOST
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        DSIHOST_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to DSIHOST
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DSIHOST: *const RegisterBlock = 0x50000000 as *const _;
