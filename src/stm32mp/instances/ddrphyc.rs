#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DDRPHYC
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::ddrphyc::Instance;
pub use crate::stm32mp::peripherals::ddrphyc::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::ddrphyc::{
    DDRPHYC_ACDLLCR, DDRPHYC_ACIOCR, DDRPHYC_DCR, DDRPHYC_DDR3_MR0, DDRPHYC_DDR3_MR1,
    DDRPHYC_DDR3_MR2, DDRPHYC_DDR3_MR3, DDRPHYC_DLLGCR, DDRPHYC_DSGCR, DDRPHYC_DTAR, DDRPHYC_DTDR0,
    DDRPHYC_DTDR1, DDRPHYC_DTPR0, DDRPHYC_DTPR1, DDRPHYC_DTPR2, DDRPHYC_DX0DLLCR, DDRPHYC_DX0DQSTR,
    DDRPHYC_DX0DQTR, DDRPHYC_DX0GCR, DDRPHYC_DX0GSR0, DDRPHYC_DX0GSR1, DDRPHYC_DX1DLLCR,
    DDRPHYC_DX1DQSTR, DDRPHYC_DX1DQTR, DDRPHYC_DX1GCR, DDRPHYC_DX1GSR0, DDRPHYC_DX1GSR1,
    DDRPHYC_DX2DLLCR, DDRPHYC_DX2DQSTR, DDRPHYC_DX2DQTR, DDRPHYC_DX2GCR, DDRPHYC_DX2GSR0,
    DDRPHYC_DX2GSR1, DDRPHYC_DX3DLLCR, DDRPHYC_DX3DQSTR, DDRPHYC_DX3DQTR, DDRPHYC_DX3GCR,
    DDRPHYC_DX3GSR0, DDRPHYC_DX3GSR1, DDRPHYC_DXCCR, DDRPHYC_GPR0, DDRPHYC_GPR1, DDRPHYC_ODTCR,
    DDRPHYC_PGCR, DDRPHYC_PGSR, DDRPHYC_PIR, DDRPHYC_PTR0, DDRPHYC_PTR1, DDRPHYC_PTR2,
    DDRPHYC_RIDR, DDRPHYC_ZQ0CR0, DDRPHYC_ZQ0CR1, DDRPHYC_ZQ0SR0, DDRPHYC_ZQ0SR1,
};

/// Access functions for the DDRPHYC peripheral instance
pub mod DDRPHYC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x5a004000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DDRPHYC
    pub const reset: ResetValues = ResetValues {
        DDRPHYC_RIDR: 0x00410010,
        DDRPHYC_PIR: 0x00000000,
        DDRPHYC_PGCR: 0x01BC2E04,
        DDRPHYC_PGSR: 0x00000000,
        DDRPHYC_DLLGCR: 0x03737000,
        DDRPHYC_ACDLLCR: 0x40000000,
        DDRPHYC_PTR0: 0x0022AF9B,
        DDRPHYC_PTR1: 0x0604111D,
        DDRPHYC_PTR2: 0x042DA072,
        DDRPHYC_ACIOCR: 0x33C03812,
        DDRPHYC_DXCCR: 0x00000800,
        DDRPHYC_DSGCR: 0xFA00001F,
        DDRPHYC_DCR: 0x0000000B,
        DDRPHYC_DTPR0: 0x3012666E,
        DDRPHYC_DTPR1: 0x0A030090,
        DDRPHYC_DTPR2: 0x20040D84,
        DDRPHYC_DDR3_MR0: 0x00000A52,
        DDRPHYC_DDR3_MR1: 0x00000000,
        DDRPHYC_DDR3_MR2: 0x00000000,
        DDRPHYC_DDR3_MR3: 0x00000000,
        DDRPHYC_ODTCR: 0x84210000,
        DDRPHYC_DTAR: 0x00000000,
        DDRPHYC_DTDR0: 0xDD22EE11,
        DDRPHYC_DTDR1: 0x7788BB44,
        DDRPHYC_GPR0: 0x00000000,
        DDRPHYC_GPR1: 0x00000000,
        DDRPHYC_ZQ0CR0: 0x0000014A,
        DDRPHYC_ZQ0CR1: 0x0000007B,
        DDRPHYC_ZQ0SR0: 0x0000014A,
        DDRPHYC_ZQ0SR1: 0x00000000,
        DDRPHYC_DX0GCR: 0x0000EE81,
        DDRPHYC_DX0GSR0: 0x00000000,
        DDRPHYC_DX0GSR1: 0x00000000,
        DDRPHYC_DX0DLLCR: 0x40000000,
        DDRPHYC_DX0DQTR: 0xFFFFFFFF,
        DDRPHYC_DX0DQSTR: 0x3DB02000,
        DDRPHYC_DX1GCR: 0x0000EE81,
        DDRPHYC_DX1GSR0: 0x00000000,
        DDRPHYC_DX1GSR1: 0x00000000,
        DDRPHYC_DX1DLLCR: 0x40000000,
        DDRPHYC_DX1DQTR: 0xFFFFFFFF,
        DDRPHYC_DX1DQSTR: 0x3DB02000,
        DDRPHYC_DX2GCR: 0x0000EE81,
        DDRPHYC_DX2GSR0: 0x00000000,
        DDRPHYC_DX2GSR1: 0x00000000,
        DDRPHYC_DX2DLLCR: 0x40000000,
        DDRPHYC_DX2DQTR: 0xFFFFFFFF,
        DDRPHYC_DX2DQSTR: 0x3DB02000,
        DDRPHYC_DX3GCR: 0x0000EE81,
        DDRPHYC_DX3GSR0: 0x00000000,
        DDRPHYC_DX3GSR1: 0x00000000,
        DDRPHYC_DX3DLLCR: 0x40000000,
        DDRPHYC_DX3DQTR: 0xFFFFFFFF,
        DDRPHYC_DX3DQSTR: 0x3DB02000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut DDRPHYC_TAKEN: bool = false;

    /// Safe access to DDRPHYC
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
            if DDRPHYC_TAKEN {
                None
            } else {
                DDRPHYC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to DDRPHYC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if DDRPHYC_TAKEN && inst.addr == INSTANCE.addr {
                DDRPHYC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal DDRPHYC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        DDRPHYC_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to DDRPHYC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DDRPHYC: *const RegisterBlock = 0x5a004000 as *const _;
