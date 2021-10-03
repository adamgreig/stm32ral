#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! ETH_DMA
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::eth_dma::Instance;
pub use crate::stm32mp::peripherals::eth_dma::{
    ETH_DMAA4RxACR, ETH_DMAA4TxACR, ETH_DMAC0CARxBR, ETH_DMAC0CARxDR, ETH_DMAC0CATxBR,
    ETH_DMAC0CATxDR, ETH_DMAC0RxCR, ETH_DMAC0RxDLAR, ETH_DMAC0RxDTPR, ETH_DMAC0RxIWTR,
    ETH_DMAC0RxRLR, ETH_DMAC0TxCR, ETH_DMAC0TxDLAR, ETH_DMAC0TxDTPR, ETH_DMAC0TxRLR,
    ETH_DMAC1CATxBR, ETH_DMAC1CATxDR, ETH_DMAC1TxCR, ETH_DMAC1TxDLAR, ETH_DMAC1TxDTPR,
    ETH_DMAC1TxRLR, ETH_DMAA4DACR, ETH_DMAC0CR, ETH_DMAC0IER, ETH_DMAC0MFCR, ETH_DMAC0SFCSR,
    ETH_DMAC0SR, ETH_DMAC1CR, ETH_DMAC1IER, ETH_DMAC1MFCR, ETH_DMAC1SFCSR, ETH_DMAC1SR, ETH_DMADSR,
    ETH_DMAISR, ETH_DMAMR, ETH_DMASBMR,
};
pub use crate::stm32mp::peripherals::eth_dma::{RegisterBlock, ResetValues};

/// Access functions for the ETH_DMA peripheral instance
pub mod ETH_DMA {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x5800b000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in ETH_DMA
    pub const reset: ResetValues = ResetValues {
        ETH_DMAMR: 0x00008000,
        ETH_DMASBMR: 0x00008000,
        ETH_DMAISR: 0x00008000,
        ETH_DMADSR: 0x00000000,
        ETH_DMAA4TxACR: 0x00000000,
        ETH_DMAA4RxACR: 0x00000000,
        ETH_DMAA4DACR: 0x00000000,
        ETH_DMAC0CR: 0x00000000,
        ETH_DMAC1CR: 0x00000000,
        ETH_DMAC0TxCR: 0x00000000,
        ETH_DMAC1TxCR: 0x00000000,
        ETH_DMAC0RxCR: 0x00008000,
        ETH_DMAC0TxDLAR: 0x00000000,
        ETH_DMAC1TxDLAR: 0x00000000,
        ETH_DMAC0RxDLAR: 0x00008000,
        ETH_DMAC0TxDTPR: 0x00000000,
        ETH_DMAC1TxDTPR: 0x00000000,
        ETH_DMAC0RxDTPR: 0x00000000,
        ETH_DMAC0TxRLR: 0x00000000,
        ETH_DMAC1TxRLR: 0x00000000,
        ETH_DMAC0RxRLR: 0x00008000,
        ETH_DMAC0IER: 0x00008000,
        ETH_DMAC1IER: 0x00008000,
        ETH_DMAC0RxIWTR: 0x00000000,
        ETH_DMAC0SFCSR: 0x00000000,
        ETH_DMAC1SFCSR: 0x00000000,
        ETH_DMAC0CATxDR: 0x00000000,
        ETH_DMAC1CATxDR: 0x00000000,
        ETH_DMAC0CARxDR: 0x00000000,
        ETH_DMAC0CATxBR: 0x00000000,
        ETH_DMAC1CATxBR: 0x00000000,
        ETH_DMAC0CARxBR: 0x00000000,
        ETH_DMAC0SR: 0x00000000,
        ETH_DMAC1SR: 0x00000000,
        ETH_DMAC0MFCR: 0x00000000,
        ETH_DMAC1MFCR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut ETH_DMA_TAKEN: bool = false;

    /// Safe access to ETH_DMA
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
            if ETH_DMA_TAKEN {
                None
            } else {
                ETH_DMA_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to ETH_DMA
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if ETH_DMA_TAKEN && inst.addr == INSTANCE.addr {
                ETH_DMA_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal ETH_DMA
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        ETH_DMA_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to ETH_DMA
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const ETH_DMA: *const RegisterBlock = 0x5800b000 as *const _;
