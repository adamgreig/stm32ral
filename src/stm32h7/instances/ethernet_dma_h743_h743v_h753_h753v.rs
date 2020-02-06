#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Ethernet: DMA mode register (DMA)
//!
//! Used by: stm32h743, stm32h743v, stm32h753, stm32h753v

#[cfg(not(feature = "nosync"))]
pub use crate::stm32h7::peripherals::ethernet_dma_v1::Instance;
pub use crate::stm32h7::peripherals::ethernet_dma_v1::{
    DMACCARxBR, DMACCARxDR, DMACCATxBR, DMACCATxDR, DMACRxCR, DMACRxDLAR, DMACRxDTPR, DMACRxIWTR,
    DMACRxRLR, DMACTxCR, DMACTxDLAR, DMACTxDTPR, DMACTxRLR, DMACCR, DMACIER, DMACMFCR, DMACSR,
    DMADSR, DMAISR, DMAMR, DMASBMR,
};
pub use crate::stm32h7::peripherals::ethernet_dma_v1::{RegisterBlock, ResetValues};

/// Access functions for the Ethernet_DMA peripheral instance
pub mod Ethernet_DMA {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40029000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in Ethernet_DMA
    pub const reset: ResetValues = ResetValues {
        DMAMR: 0x00000000,
        DMASBMR: 0x01010000,
        DMAISR: 0x00000000,
        DMADSR: 0x00000000,
        DMACCR: 0x00000000,
        DMACTxCR: 0x00000000,
        DMACRxCR: 0x00000000,
        DMACTxDLAR: 0x00000000,
        DMACRxDLAR: 0x00000000,
        DMACTxDTPR: 0x00000000,
        DMACRxDTPR: 0x00000000,
        DMACTxRLR: 0x00000000,
        DMACRxRLR: 0x00000000,
        DMACIER: 0x00000000,
        DMACRxIWTR: 0x00000000,
        DMACCATxDR: 0x00000000,
        DMACCARxDR: 0x00000000,
        DMACCATxBR: 0x00000000,
        DMACCARxBR: 0x00000000,
        DMACSR: 0x00000000,
        DMACMFCR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut Ethernet_DMA_TAKEN: bool = false;

    /// Safe access to Ethernet_DMA
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
            if Ethernet_DMA_TAKEN {
                None
            } else {
                Ethernet_DMA_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to Ethernet_DMA
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if Ethernet_DMA_TAKEN && inst.addr == INSTANCE.addr {
                Ethernet_DMA_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal Ethernet_DMA
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        Ethernet_DMA_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to Ethernet_DMA
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const Ethernet_DMA: *const RegisterBlock = 0x40029000 as *const _;
