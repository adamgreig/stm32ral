#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! ETH_MAC_MMC
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::eth_mac_mmc::Instance;
pub use crate::stm32mp::peripherals::eth_mac_mmc::{
    ETH_MACQ0TxFCR, ETH_MACRxFCR, ETH_MACRxQC0R, ETH_MACRxQC1R, ETH_MACRxQC2R, ETH_MACRxTxSR,
    ETH_MACTxQPMR, ETH_MACTxTSSNR, ETH_MACTxTSSSR, ETH_MAC1USTCR, ETH_MACA0HR, ETH_MACA0LR,
    ETH_MACA1HR, ETH_MACA1LR, ETH_MACA2HR, ETH_MACA2LR, ETH_MACA3HR, ETH_MACA3LR, ETH_MACACR,
    ETH_MACARPAR, ETH_MACATSNR, ETH_MACATSSR, ETH_MACCR, ETH_MACDR, ETH_MACECR, ETH_MACHT0R,
    ETH_MACHT1R, ETH_MACHWF1R, ETH_MACHWF2R, ETH_MACIER, ETH_MACISR, ETH_MACIVIR, ETH_MACL3A00R,
    ETH_MACL3A01R, ETH_MACL3A10R, ETH_MACL3A11R, ETH_MACL3A20, ETH_MACL3A21R, ETH_MACL3A30,
    ETH_MACL3A31R, ETH_MACL3L4C0R, ETH_MACL3L4C1R, ETH_MACL4A0R, ETH_MACL4A1R, ETH_MACLCSR,
    ETH_MACLETR, ETH_MACLMIR, ETH_MACLTCR, ETH_MACMDIOAR, ETH_MACMDIODR, ETH_MACPCSR, ETH_MACPFR,
    ETH_MACPHYCSR, ETH_MACPOCR, ETH_MACPPSCR, ETH_MACPPSIR, ETH_MACPPSTTNR, ETH_MACPPSTTSR,
    ETH_MACPPSWR, ETH_MACRWKPFR, ETH_MACSPI0R, ETH_MACSPI1R, ETH_MACSPI2R, ETH_MACSSIR,
    ETH_MACSTNR, ETH_MACSTNUR, ETH_MACSTSR, ETH_MACSTSUR, ETH_MACTSAR, ETH_MACTSCR, ETH_MACTSEACR,
    ETH_MACTSECNR, ETH_MACTSIACR, ETH_MACTSICNR, ETH_MACTSSR, ETH_MACVHTR, ETH_MACVIR, ETH_MACVR,
    ETH_MACVTR, ETH_MACWTR, MMC_CONTROL, MMC_RX_INTERRUPT, MMC_RX_INTERRUPT_MASK, MMC_TX_INTERRUPT,
    MMC_TX_INTERRUPT_MASK, RX_ALIGNMENT_ERROR_PACKETS, RX_CRC_ERROR_PACKETS, RX_LPI_TRAN_CNTR,
    RX_LPI_USEC_CNTR, RX_UNICAST_PACKETS_GOOD, TX_LPI_TRAN_CNTR, TX_LPI_USEC_CNTR,
    TX_MULTIPLE_COLLISION_GOOD_PACKETS, TX_PACKET_COUNT_GOOD, TX_SINGLE_COLLISION_GOOD_PACKETS,
};
pub use crate::stm32mp::peripherals::eth_mac_mmc::{RegisterBlock, ResetValues};

/// Access functions for the ETH_MAC_MMC peripheral instance
pub mod ETH_MAC_MMC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x5800a000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in ETH_MAC_MMC
    pub const reset: ResetValues = ResetValues {
        ETH_MACCR: 0x00008000,
        ETH_MACECR: 0x00000000,
        ETH_MACPFR: 0x00000000,
        ETH_MACWTR: 0x00000000,
        ETH_MACHT0R: 0x00000000,
        ETH_MACHT1R: 0x00000000,
        ETH_MACVTR: 0x00000000,
        ETH_MACVHTR: 0x00000000,
        ETH_MACVIR: 0x00000000,
        ETH_MACIVIR: 0x00000000,
        ETH_MACQ0TxFCR: 0x00000000,
        ETH_MACRxFCR: 0x00000000,
        ETH_MACTxQPMR: 0x00000000,
        ETH_MACRxQC0R: 0x00000000,
        ETH_MACRxQC1R: 0x00000000,
        ETH_MACRxQC2R: 0x00000000,
        ETH_MACISR: 0x00000000,
        ETH_MACIER: 0x00000000,
        ETH_MACRxTxSR: 0x00000000,
        ETH_MACPCSR: 0x00000000,
        ETH_MACRWKPFR: 0x00000000,
        ETH_MACLCSR: 0x00000000,
        ETH_MACLTCR: 0x03E80000,
        ETH_MACLETR: 0x00000000,
        ETH_MAC1USTCR: 0x00000000,
        ETH_MACPHYCSR: 0x00000000,
        ETH_MACVR: 0x00004042,
        ETH_MACDR: 0x00000000,
        ETH_MACHWF1R: 0x11141945,
        ETH_MACHWF2R: 0x41040041,
        ETH_MACMDIOAR: 0x00000000,
        ETH_MACMDIODR: 0x00000000,
        ETH_MACA0HR: 0x8000FFFF,
        ETH_MACA0LR: 0xFFFFFFFF,
        ETH_MACA1HR: 0x0000FFFF,
        ETH_MACA1LR: 0xFFFFFFFF,
        ETH_MACA2HR: 0x0000FFFF,
        ETH_MACA2LR: 0xFFFFFFFF,
        ETH_MACA3HR: 0x0000FFFF,
        ETH_MACA3LR: 0xFFFFFFFF,
        MMC_CONTROL: 0x00000000,
        MMC_RX_INTERRUPT: 0x00000000,
        MMC_TX_INTERRUPT: 0x00000000,
        MMC_RX_INTERRUPT_MASK: 0x00000000,
        MMC_TX_INTERRUPT_MASK: 0x00000000,
        TX_SINGLE_COLLISION_GOOD_PACKETS: 0x00000000,
        TX_MULTIPLE_COLLISION_GOOD_PACKETS: 0x00000000,
        TX_PACKET_COUNT_GOOD: 0x00000000,
        RX_CRC_ERROR_PACKETS: 0x00000000,
        RX_ALIGNMENT_ERROR_PACKETS: 0x00000000,
        RX_UNICAST_PACKETS_GOOD: 0x00000000,
        TX_LPI_USEC_CNTR: 0x00000000,
        TX_LPI_TRAN_CNTR: 0x00000000,
        RX_LPI_USEC_CNTR: 0x00000000,
        RX_LPI_TRAN_CNTR: 0x00000000,
        ETH_MACL3L4C0R: 0x00000000,
        ETH_MACL4A0R: 0x00000000,
        ETH_MACL3A00R: 0x00000000,
        ETH_MACL3A10R: 0x00000000,
        ETH_MACL3A20: 0x00000000,
        ETH_MACL3A30: 0x00000000,
        ETH_MACL3L4C1R: 0x00000000,
        ETH_MACL4A1R: 0x00000000,
        ETH_MACL3A01R: 0x00000000,
        ETH_MACL3A11R: 0x00000000,
        ETH_MACL3A21R: 0x00000000,
        ETH_MACL3A31R: 0x00000000,
        ETH_MACARPAR: 0x00000000,
        ETH_MACTSCR: 0x00002000,
        ETH_MACSSIR: 0x00000000,
        ETH_MACSTSR: 0x00000000,
        ETH_MACSTNR: 0x00000000,
        ETH_MACSTSUR: 0x00000000,
        ETH_MACSTNUR: 0x00000000,
        ETH_MACTSAR: 0x00000000,
        ETH_MACTSSR: 0x00000000,
        ETH_MACTxTSSNR: 0x00000000,
        ETH_MACTxTSSSR: 0x00000000,
        ETH_MACACR: 0x00000000,
        ETH_MACATSNR: 0x00000000,
        ETH_MACATSSR: 0x00000000,
        ETH_MACTSIACR: 0x00000000,
        ETH_MACTSEACR: 0x00000000,
        ETH_MACTSICNR: 0x00000000,
        ETH_MACTSECNR: 0x00000000,
        ETH_MACPPSCR: 0x00000000,
        ETH_MACPPSTTSR: 0x00000000,
        ETH_MACPPSTTNR: 0x00000000,
        ETH_MACPPSIR: 0x00000000,
        ETH_MACPPSWR: 0x00000000,
        ETH_MACPOCR: 0x00000000,
        ETH_MACSPI0R: 0x00000000,
        ETH_MACSPI1R: 0x00000000,
        ETH_MACSPI2R: 0x00000000,
        ETH_MACLMIR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut ETH_MAC_MMC_TAKEN: bool = false;

    /// Safe access to ETH_MAC_MMC
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
            if ETH_MAC_MMC_TAKEN {
                None
            } else {
                ETH_MAC_MMC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to ETH_MAC_MMC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if ETH_MAC_MMC_TAKEN && inst.addr == INSTANCE.addr {
                ETH_MAC_MMC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal ETH_MAC_MMC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        ETH_MAC_MMC_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to ETH_MAC_MMC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const ETH_MAC_MMC: *const RegisterBlock = 0x5800a000 as *const _;
