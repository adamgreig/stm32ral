#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DMA2D controller
//!
//! Used by: stm32l4r9, stm32l4x6

#[cfg(not(feature = "nosync"))]
pub use crate::stm32l4::peripherals::dma2d::Instance;
pub use crate::stm32l4::peripherals::dma2d::{RegisterBlock, ResetValues};
pub use crate::stm32l4::peripherals::dma2d::{
    AMTCR, BGCLUT, BGCMAR, BGCOLR, BGMAR, BGOR, BGPFCCR, CR, FGCLUT, FGCMAR, FGCOLR, FGMAR, FGOR,
    FGPFCCR, IFCR, ISR, LWR, NLR, OCOLR, OMAR, OOR, OPFCCR,
};

/// Access functions for the DMA2D peripheral instance
pub mod DMA2D {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x4002b000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DMA2D
    pub const reset: ResetValues = ResetValues {
        CR: 0x00000000,
        ISR: 0x00000000,
        IFCR: 0x00000000,
        FGMAR: 0x00000000,
        FGOR: 0x00000000,
        BGMAR: 0x00000000,
        BGOR: 0x00000000,
        FGPFCCR: 0x00000000,
        FGCOLR: 0x00000000,
        BGPFCCR: 0x00000000,
        BGCOLR: 0x00000000,
        FGCMAR: 0x00000000,
        BGCMAR: 0x00000000,
        OPFCCR: 0x00000000,
        OCOLR: 0x00000000,
        OMAR: 0x00000000,
        OOR: 0x00000000,
        NLR: 0x00000000,
        LWR: 0x00000000,
        AMTCR: 0x00000000,
        FGCLUT: 0x00000000,
        BGCLUT: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut DMA2D_TAKEN: bool = false;

    /// Safe access to DMA2D
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
            if DMA2D_TAKEN {
                None
            } else {
                DMA2D_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to DMA2D
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if DMA2D_TAKEN && inst.addr == INSTANCE.addr {
                DMA2D_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal DMA2D
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        DMA2D_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to DMA2D
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DMA2D: *const RegisterBlock = 0x4002b000 as *const _;
