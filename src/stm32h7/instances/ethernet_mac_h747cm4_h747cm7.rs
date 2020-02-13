#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Ethernet: media access control (MAC)
//!
//! Used by: stm32h747cm4, stm32h747cm7

#[cfg(not(feature = "nosync"))]
pub use crate::stm32h7::peripherals::ethernet_mac_v2::Instance;
pub use crate::stm32h7::peripherals::ethernet_mac_v2::{
    MACQTxFCR, MACRxFCR, MACRxTxSR, MACTxTSSNR, MACTxTSSSR, MAC1USTCR, MACA0HR, MACA0LR, MACA1HR,
    MACA1LR, MACA2HR, MACA2LR, MACA3HR, MACA3LR, MACACR, MACARPAR, MACATSNR, MACATSSR, MACCR,
    MACDR, MACECR, MACHT0R, MACHT1R, MACHWF1R, MACHWF2R, MACIER, MACISR, MACIVIR, MACL3A00R,
    MACL3A01R, MACL3A10R, MACL3A11R, MACL3A20, MACL3A21R, MACL3A30, MACL3A31R, MACL3L4C0R,
    MACL3L4C1R, MACL4A0R, MACL4A1R, MACLCSR, MACLETR, MACLMIR, MACLTCR, MACMDIOAR, MACMDIODR,
    MACPCSR, MACPFR, MACPOCR, MACPPSCR, MACPPSIR, MACPPSTTNR, MACPPSTTSR, MACPPSWR, MACRWKPFR,
    MACSPI0R, MACSPI1R, MACSPI2R, MACSSIR, MACSTNR, MACSTNUR, MACSTSR, MACSTSUR, MACTSAR, MACTSCR,
    MACTSEACR, MACTSECNR, MACTSIACR, MACTSICNR, MACTSSR, MACVHTR, MACVIR, MACVR, MACVTR, MACWTR,
    MMC_CONTROL, MMC_RX_INTERRUPT, MMC_RX_INTERRUPT_MASK, MMC_TX_INTERRUPT, MMC_TX_INTERRUPT_MASK,
    RX_ALIGNMENT_ERROR_PACKETS, RX_CRC_ERROR_PACKETS, RX_LPI_TRAN_CNTR, RX_LPI_USEC_CNTR,
    RX_UNICAST_PACKETS_GOOD, TX_LPI_TRAN_CNTR, TX_LPI_USEC_CNTR,
    TX_MULTIPLE_COLLISION_GOOD_PACKETS, TX_PACKET_COUNT_GOOD, TX_SINGLE_COLLISION_GOOD_PACKETS,
};
pub use crate::stm32h7::peripherals::ethernet_mac_v2::{RegisterBlock, ResetValues};

/// Access functions for the Ethernet_MAC peripheral instance
pub mod Ethernet_MAC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40028000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in Ethernet_MAC
    pub const reset: ResetValues = ResetValues {
        MACCR: 0x00000000,
        MACECR: 0x00000000,
        MACPFR: 0x00000000,
        MACWTR: 0x00000000,
        MACHT0R: 0x00000000,
        MACHT1R: 0x00000000,
        MACVTR: 0x00000000,
        MACVHTR: 0x00000000,
        MACVIR: 0x00000000,
        MACIVIR: 0x00000000,
        MACQTxFCR: 0x00000000,
        MACRxFCR: 0x00000000,
        MACISR: 0x00000000,
        MACIER: 0x00000000,
        MACRxTxSR: 0x00000000,
        MACPCSR: 0x00000000,
        MACRWKPFR: 0x00000000,
        MACLCSR: 0x00000000,
        MACLTCR: 0x03E80000,
        MACLETR: 0x00000000,
        MAC1USTCR: 0x00000000,
        MACVR: 0x00003041,
        MACHWF1R: 0x11841904,
        MACHWF2R: 0x41000000,
        MACMDIOAR: 0x00000000,
        MACMDIODR: 0x00000000,
        MACARPAR: 0x00000000,
        MACA0HR: 0x8000FFFF,
        MACA0LR: 0xFFFFFFFF,
        MACA1LR: 0xFFFFFFFF,
        MACA2LR: 0xFFFFFFFF,
        MACA3LR: 0xFFFFFFFF,
        MACA1HR: 0x0000FFFF,
        MACA2HR: 0x0000FFFF,
        MACA3HR: 0x0000FFFF,
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
        MACL3L4C0R: 0x00000000,
        MACL4A0R: 0x00000000,
        MACDR: 0x00000000,
        MACL3A00R: 0x00000000,
        MACL3A10R: 0x00000000,
        MACL3A20: 0x00000000,
        MACL3A30: 0x00000000,
        MACL3L4C1R: 0x00000000,
        MACL4A1R: 0x00000000,
        MACL3A01R: 0x00000000,
        MACL3A11R: 0x00000000,
        MACL3A21R: 0x00000000,
        MACL3A31R: 0x00000000,
        MACTSCR: 0x00000200,
        MACSSIR: 0x00000000,
        MACSTSR: 0x00000000,
        MACSTNR: 0x00000000,
        MACSTSUR: 0x00000000,
        MACSTNUR: 0x00000000,
        MACTSAR: 0x00000000,
        MACTSSR: 0x00000000,
        MACTxTSSNR: 0x00000000,
        MACTxTSSSR: 0x00000000,
        MACACR: 0x00000000,
        MACATSNR: 0x00000000,
        MACATSSR: 0x00000000,
        MACTSIACR: 0x00000000,
        MACTSEACR: 0x00000000,
        MACTSICNR: 0x00000000,
        MACTSECNR: 0x00000000,
        MACPPSCR: 0x00000000,
        MACPPSTTSR: 0x00000000,
        MACPPSTTNR: 0x00000000,
        MACPPSIR: 0x00000000,
        MACPPSWR: 0x00000000,
        MACPOCR: 0x00000000,
        MACSPI0R: 0x00000000,
        MACSPI1R: 0x00000000,
        MACSPI2R: 0x00000000,
        MACLMIR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut Ethernet_MAC_TAKEN: bool = false;

    /// Safe access to Ethernet_MAC
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
            if Ethernet_MAC_TAKEN {
                None
            } else {
                Ethernet_MAC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to Ethernet_MAC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if Ethernet_MAC_TAKEN && inst.addr == INSTANCE.addr {
                Ethernet_MAC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal Ethernet_MAC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        Ethernet_MAC_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to Ethernet_MAC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const Ethernet_MAC: *const RegisterBlock = 0x40028000 as *const _;
