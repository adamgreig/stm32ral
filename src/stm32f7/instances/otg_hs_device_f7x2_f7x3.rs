#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! USB on the go high speed
//!
//! Used by: stm32f7x2, stm32f7x3

#[cfg(not(feature = "nosync"))]
pub use crate::stm32f7::peripherals::otg_hs_device_v1::Instance;
pub use crate::stm32f7::peripherals::otg_hs_device_v1::{RegisterBlock, ResetValues};
pub use crate::stm32f7::peripherals::otg_hs_device_v1::{
    OTG_HS_DAINT, OTG_HS_DAINTMSK, OTG_HS_DCFG, OTG_HS_DCTL, OTG_HS_DEACHINT, OTG_HS_DEACHINTMSK,
    OTG_HS_DIEPCTL0, OTG_HS_DIEPCTL1, OTG_HS_DIEPCTL2, OTG_HS_DIEPCTL3, OTG_HS_DIEPCTL4,
    OTG_HS_DIEPCTL5, OTG_HS_DIEPCTL6, OTG_HS_DIEPCTL7, OTG_HS_DIEPDMA0, OTG_HS_DIEPDMA1,
    OTG_HS_DIEPDMA10, OTG_HS_DIEPDMA11, OTG_HS_DIEPDMA12, OTG_HS_DIEPDMA13, OTG_HS_DIEPDMA14,
    OTG_HS_DIEPDMA15, OTG_HS_DIEPDMA2, OTG_HS_DIEPDMA3, OTG_HS_DIEPDMA4, OTG_HS_DIEPDMA5,
    OTG_HS_DIEPDMA6, OTG_HS_DIEPDMA7, OTG_HS_DIEPDMA8, OTG_HS_DIEPDMA9, OTG_HS_DIEPEMPMSK,
    OTG_HS_DIEPINT0, OTG_HS_DIEPINT1, OTG_HS_DIEPINT2, OTG_HS_DIEPINT3, OTG_HS_DIEPINT4,
    OTG_HS_DIEPINT5, OTG_HS_DIEPINT6, OTG_HS_DIEPINT7, OTG_HS_DIEPMSK, OTG_HS_DIEPTSIZ0,
    OTG_HS_DIEPTSIZ1, OTG_HS_DIEPTSIZ2, OTG_HS_DIEPTSIZ3, OTG_HS_DIEPTSIZ4, OTG_HS_DIEPTSIZ5,
    OTG_HS_DIEPTSIZ6, OTG_HS_DIEPTSIZ7, OTG_HS_DOEPCTL0, OTG_HS_DOEPCTL1, OTG_HS_DOEPCTL2,
    OTG_HS_DOEPCTL3, OTG_HS_DOEPCTL4, OTG_HS_DOEPCTL5, OTG_HS_DOEPCTL6, OTG_HS_DOEPCTL7,
    OTG_HS_DOEPDMA0, OTG_HS_DOEPDMA1, OTG_HS_DOEPDMA10, OTG_HS_DOEPDMA11, OTG_HS_DOEPDMA12,
    OTG_HS_DOEPDMA13, OTG_HS_DOEPDMA14, OTG_HS_DOEPDMA15, OTG_HS_DOEPDMA2, OTG_HS_DOEPDMA3,
    OTG_HS_DOEPDMA4, OTG_HS_DOEPDMA5, OTG_HS_DOEPDMA6, OTG_HS_DOEPDMA7, OTG_HS_DOEPDMA8,
    OTG_HS_DOEPDMA9, OTG_HS_DOEPINT0, OTG_HS_DOEPINT1, OTG_HS_DOEPINT2, OTG_HS_DOEPINT3,
    OTG_HS_DOEPINT4, OTG_HS_DOEPINT5, OTG_HS_DOEPINT6, OTG_HS_DOEPINT7, OTG_HS_DOEPMSK,
    OTG_HS_DOEPTSIZ0, OTG_HS_DOEPTSIZ1, OTG_HS_DOEPTSIZ2, OTG_HS_DOEPTSIZ3, OTG_HS_DOEPTSIZ4,
    OTG_HS_DOEPTSIZ5, OTG_HS_DOEPTSIZ6, OTG_HS_DOEPTSIZ7, OTG_HS_DSTS, OTG_HS_DTHRCTL,
    OTG_HS_DTXFSTS0, OTG_HS_DTXFSTS1, OTG_HS_DTXFSTS2, OTG_HS_DTXFSTS3, OTG_HS_DTXFSTS4,
    OTG_HS_DTXFSTS5, OTG_HS_DTXFSTS6, OTG_HS_DTXFSTS7, OTG_HS_DVBUSDIS, OTG_HS_DVBUSPULSE,
};

/// Access functions for the OTG_HS_DEVICE peripheral instance
pub mod OTG_HS_DEVICE {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40040800,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in OTG_HS_DEVICE
    pub const reset: ResetValues = ResetValues {
        OTG_HS_DCFG: 0x02200000,
        OTG_HS_DCTL: 0x00000000,
        OTG_HS_DSTS: 0x00000010,
        OTG_HS_DIEPMSK: 0x00000000,
        OTG_HS_DOEPMSK: 0x00000000,
        OTG_HS_DAINT: 0x00000000,
        OTG_HS_DAINTMSK: 0x00000000,
        OTG_HS_DVBUSDIS: 0x000017D7,
        OTG_HS_DVBUSPULSE: 0x000005B8,
        OTG_HS_DTHRCTL: 0x00000000,
        OTG_HS_DIEPEMPMSK: 0x00000000,
        OTG_HS_DEACHINT: 0x00000000,
        OTG_HS_DEACHINTMSK: 0x00000000,
        OTG_HS_DIEPCTL0: 0x00000000,
        OTG_HS_DIEPCTL1: 0x00000000,
        OTG_HS_DIEPCTL2: 0x00000000,
        OTG_HS_DIEPCTL3: 0x00000000,
        OTG_HS_DIEPCTL4: 0x00000000,
        OTG_HS_DIEPCTL5: 0x00000000,
        OTG_HS_DIEPCTL6: 0x00000000,
        OTG_HS_DIEPCTL7: 0x00000000,
        OTG_HS_DIEPINT0: 0x00000080,
        OTG_HS_DIEPINT1: 0x00000000,
        OTG_HS_DIEPINT2: 0x00000000,
        OTG_HS_DIEPINT3: 0x00000000,
        OTG_HS_DIEPINT4: 0x00000000,
        OTG_HS_DIEPINT5: 0x00000000,
        OTG_HS_DIEPINT6: 0x00000000,
        OTG_HS_DIEPINT7: 0x00000000,
        OTG_HS_DIEPTSIZ0: 0x00000000,
        OTG_HS_DIEPDMA0: 0x00000000,
        OTG_HS_DIEPDMA1: 0x00000000,
        OTG_HS_DIEPDMA2: 0x00000000,
        OTG_HS_DIEPDMA3: 0x00000000,
        OTG_HS_DIEPDMA4: 0x00000000,
        OTG_HS_DTXFSTS0: 0x00000000,
        OTG_HS_DTXFSTS1: 0x00000000,
        OTG_HS_DTXFSTS2: 0x00000000,
        OTG_HS_DTXFSTS3: 0x00000000,
        OTG_HS_DTXFSTS4: 0x00000000,
        OTG_HS_DTXFSTS5: 0x00000000,
        OTG_HS_DIEPTSIZ1: 0x00000000,
        OTG_HS_DIEPTSIZ2: 0x00000000,
        OTG_HS_DIEPTSIZ3: 0x00000000,
        OTG_HS_DIEPTSIZ4: 0x00000000,
        OTG_HS_DIEPTSIZ5: 0x00000000,
        OTG_HS_DOEPCTL0: 0x00008000,
        OTG_HS_DOEPCTL1: 0x00000000,
        OTG_HS_DOEPCTL2: 0x00000000,
        OTG_HS_DOEPCTL3: 0x00000000,
        OTG_HS_DOEPINT0: 0x00000080,
        OTG_HS_DOEPINT1: 0x00000000,
        OTG_HS_DOEPINT2: 0x00000000,
        OTG_HS_DOEPINT3: 0x00000000,
        OTG_HS_DOEPINT4: 0x00000000,
        OTG_HS_DOEPINT5: 0x00000000,
        OTG_HS_DOEPINT6: 0x00000000,
        OTG_HS_DOEPINT7: 0x00000000,
        OTG_HS_DOEPTSIZ0: 0x00000000,
        OTG_HS_DOEPTSIZ1: 0x00000000,
        OTG_HS_DOEPTSIZ2: 0x00000000,
        OTG_HS_DOEPTSIZ3: 0x00000000,
        OTG_HS_DOEPTSIZ4: 0x00000000,
        OTG_HS_DIEPTSIZ6: 0x00000000,
        OTG_HS_DTXFSTS6: 0x00000000,
        OTG_HS_DIEPTSIZ7: 0x00000000,
        OTG_HS_DTXFSTS7: 0x00000000,
        OTG_HS_DOEPCTL4: 0x00000000,
        OTG_HS_DOEPCTL5: 0x00000000,
        OTG_HS_DOEPCTL6: 0x00000000,
        OTG_HS_DOEPCTL7: 0x00000000,
        OTG_HS_DOEPTSIZ5: 0x00000000,
        OTG_HS_DOEPTSIZ6: 0x00000000,
        OTG_HS_DOEPTSIZ7: 0x00000000,
        OTG_HS_DOEPDMA0: 0x00000000,
        OTG_HS_DOEPDMA1: 0x00000000,
        OTG_HS_DOEPDMA2: 0x00000000,
        OTG_HS_DOEPDMA3: 0x00000000,
        OTG_HS_DOEPDMA4: 0x00000000,
        OTG_HS_DOEPDMA5: 0x00000000,
        OTG_HS_DOEPDMA6: 0x00000000,
        OTG_HS_DOEPDMA7: 0x00000000,
        OTG_HS_DOEPDMA8: 0x00000000,
        OTG_HS_DOEPDMA9: 0x00000000,
        OTG_HS_DOEPDMA10: 0x00000000,
        OTG_HS_DOEPDMA11: 0x00000000,
        OTG_HS_DOEPDMA12: 0x00000000,
        OTG_HS_DOEPDMA13: 0x00000000,
        OTG_HS_DOEPDMA14: 0x00000000,
        OTG_HS_DOEPDMA15: 0x00000000,
        OTG_HS_DIEPDMA5: 0x00000000,
        OTG_HS_DIEPDMA6: 0x00000000,
        OTG_HS_DIEPDMA7: 0x00000000,
        OTG_HS_DIEPDMA8: 0x00000000,
        OTG_HS_DIEPDMA9: 0x00000000,
        OTG_HS_DIEPDMA10: 0x00000000,
        OTG_HS_DIEPDMA11: 0x00000000,
        OTG_HS_DIEPDMA12: 0x00000000,
        OTG_HS_DIEPDMA13: 0x00000000,
        OTG_HS_DIEPDMA14: 0x00000000,
        OTG_HS_DIEPDMA15: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut OTG_HS_DEVICE_TAKEN: bool = false;

    /// Safe access to OTG_HS_DEVICE
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
            if OTG_HS_DEVICE_TAKEN {
                None
            } else {
                OTG_HS_DEVICE_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to OTG_HS_DEVICE
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if OTG_HS_DEVICE_TAKEN && inst.addr == INSTANCE.addr {
                OTG_HS_DEVICE_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal OTG_HS_DEVICE
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        OTG_HS_DEVICE_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to OTG_HS_DEVICE
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const OTG_HS_DEVICE: *const RegisterBlock = 0x40040800 as *const _;
