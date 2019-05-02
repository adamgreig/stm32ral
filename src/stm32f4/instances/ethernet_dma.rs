#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Ethernet: DMA controller operation
//!
//! Used by: stm32f407, stm32f427, stm32f429, stm32f469

#[cfg(not(feature = "nosync"))]
pub use stm32f4::peripherals::ethernet_dma::Instance;
pub use stm32f4::peripherals::ethernet_dma::{RegisterBlock, ResetValues};
pub use stm32f4::peripherals::ethernet_dma::{
    DMABMR, DMACHRBAR, DMACHRDR, DMACHTBAR, DMACHTDR, DMAIER, DMAMFBOCR, DMAOMR, DMARDLAR, DMARPDR,
    DMARSWTR, DMASR, DMATDLAR, DMATPDR,
};

/// Access functions for the Ethernet_DMA peripheral instance
pub mod Ethernet_DMA {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

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
        DMABMR: 0x00002101,
        DMATPDR: 0x00000000,
        DMARPDR: 0x00000000,
        DMARDLAR: 0x00000000,
        DMATDLAR: 0x00000000,
        DMASR: 0x00000000,
        DMAOMR: 0x00000000,
        DMAIER: 0x00000000,
        DMAMFBOCR: 0x00000000,
        DMARSWTR: 0x00000000,
        DMACHTDR: 0x00000000,
        DMACHRDR: 0x00000000,
        DMACHTBAR: 0x00000000,
        DMACHRBAR: 0x00000000,
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
