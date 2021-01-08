#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! OTG

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// The OTG_GOTGCTL register controls the behavior and reflects the status of the OTG function of the core.
pub mod OTG_GOTGCTL {

    /// SRQSCS
    pub mod SRQSCS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SRQ
    pub mod SRQ {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// VBVALOEN
    pub mod VBVALOEN {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// VBVALOVAL
    pub mod VBVALOVAL {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// AVALOEN
    pub mod AVALOEN {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// AVALOVAL
    pub mod AVALOVAL {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// BVALOEN
    pub mod BVALOEN {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// BVALOVAL
    pub mod BVALOVAL {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// HNGSCS
    pub mod HNGSCS {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// HNPRQ
    pub mod HNPRQ {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// HSHNPEN
    pub mod HSHNPEN {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DHNPEN
    pub mod DHNPEN {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// EHEN
    pub mod EHEN {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CIDSTS
    pub mod CIDSTS {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DBCT
    pub mod DBCT {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ASVLD
    pub mod ASVLD {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// BSVLD
    pub mod BSVLD {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// OTGVER
    pub mod OTGVER {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CURMOD
    pub mod CURMOD {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// The application reads this register whenever there is an OTG interrupt and clears the bits in this register to clear the OTG interrupt.
pub mod OTG_GOTGINT {

    /// SEDET
    pub mod SEDET {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SRSSCHG
    pub mod SRSSCHG {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// HNSSCHG
    pub mod HNSSCHG {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// HNGDET
    pub mod HNGDET {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ADTOCHG
    pub mod ADTOCHG {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DBCDNE
    pub mod DBCDNE {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// IDCHNG
    pub mod IDCHNG {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register can be used to configure the core after power-on or a change in mode. This register mainly contains AHB system-related configuration parameters. Do not change this register after the initial programming. The application must program this register before starting any transactions on either the AHB or the USB.
pub mod OTG_GAHBCFG {

    /// GINTMSK
    pub mod GINTMSK {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// HBSTLEN
    pub mod HBSTLEN {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (4 bits: 0b1111 << 1)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DMAEN
    pub mod DMAEN {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TXFELVL
    pub mod TXFELVL {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PTXFELVL
    pub mod PTXFELVL {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register can be used to configure the core after power-on or a changing to host mode or device mode. It contains USB and USB-PHY related configuration parameters. The application must program this register before starting any transactions on either the AHB or the USB. Do not make changes to this register after the initial programming.
pub mod OTG_GUSBCFG {

    /// TOCAL
    pub mod TOCAL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PHYSEL
    pub mod PHYSEL {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SRPCAP
    pub mod SRPCAP {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// HNPCAP
    pub mod HNPCAP {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TRDT
    pub mod TRDT {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (4 bits: 0b1111 << 10)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PHYLPC
    pub mod PHYLPC {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TSDPS
    pub mod TSDPS {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// FHMOD
    pub mod FHMOD {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// FDMOD
    pub mod FDMOD {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// The application uses this register to reset various hardware features inside the core.
pub mod OTG_GRSTCTL {

    /// CSRST
    pub mod CSRST {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PSRST
    pub mod PSRST {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RXFFLSH
    pub mod RXFFLSH {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TXFFLSH
    pub mod TXFFLSH {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TXFNUM
    pub mod TXFNUM {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (5 bits: 0b11111 << 6)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DMAREQ
    pub mod DMAREQ {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// AHBIDL
    pub mod AHBIDL {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register interrupts the application for system-level events in the current mode (device mode or host mode). Some of the bits in this register are valid only in host mode, while others are valid in device mode only. This register also indicates the current mode. To clear the interrupt status bits of the rc_w1 type, the application must write 1 into the bit. The FIFO status interrupts are read-only; once software reads from or writes to the FIFO while servicing these interrupts, FIFO interrupt conditions are cleared automatically. The application must clear the OTG_GINTSTS register at initialization before unmasking the interrupt bit to avoid any interrupts generated prior to initialization.
pub mod OTG_GINTSTS {

    /// CMOD
    pub mod CMOD {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// MMIS
    pub mod MMIS {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// OTGINT
    pub mod OTGINT {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SOF
    pub mod SOF {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RXFLVL
    pub mod RXFLVL {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// NPTXFE
    pub mod NPTXFE {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// GINAKEFF
    pub mod GINAKEFF {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// GONAKEFF
    pub mod GONAKEFF {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ESUSP
    pub mod ESUSP {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// USBSUSP
    pub mod USBSUSP {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// USBRST
    pub mod USBRST {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ENUMDNE
    pub mod ENUMDNE {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ISOODRP
    pub mod ISOODRP {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// EOPF
    pub mod EOPF {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// IEPINT
    pub mod IEPINT {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// OEPINT
    pub mod OEPINT {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// IISOIXFR
    pub mod IISOIXFR {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// IPXFR
    pub mod IPXFR {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DATAFSUSP
    pub mod DATAFSUSP {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// HPRTINT
    pub mod HPRTINT {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// HCINT
    pub mod HCINT {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PTXFE
    pub mod PTXFE {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CIDSCHG
    pub mod CIDSCHG {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DISCINT
    pub mod DISCINT {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SRQINT
    pub mod SRQINT {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// WKUPINT
    pub mod WKUPINT {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register works with the core interrupt register to interrupt the application. When an interrupt bit is masked, the interrupt associated with that bit is not generated. However, the core interrupt (OTG_GINTSTS) register bit corresponding to that interrupt is still set.
pub mod OTG_GINTMSK {

    /// MMISM
    pub mod MMISM {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// OTGINT
    pub mod OTGINT {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SOFM
    pub mod SOFM {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RXFLVLM
    pub mod RXFLVLM {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// NPTXFEM
    pub mod NPTXFEM {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// GINAKEFFM
    pub mod GINAKEFFM {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// GONAKEFFM
    pub mod GONAKEFFM {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ESUSPM
    pub mod ESUSPM {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// USBSUSPM
    pub mod USBSUSPM {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// USBRST
    pub mod USBRST {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ENUMDNEM
    pub mod ENUMDNEM {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ISOODRPM
    pub mod ISOODRPM {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// EOPFM
    pub mod EOPFM {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// IEPINT
    pub mod IEPINT {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// OEPINT
    pub mod OEPINT {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// IISOIXFRM
    pub mod IISOIXFRM {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// IPXFRM
    pub mod IPXFRM {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// FSUSPM
    pub mod FSUSPM {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RSTDETM
    pub mod RSTDETM {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PRTIM
    pub mod PRTIM {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// HCIM
    pub mod HCIM {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PTXFEM
    pub mod PTXFEM {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LPMINTM
    pub mod LPMINTM {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CIDSCHGM
    pub mod CIDSCHGM {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DISCINT
    pub mod DISCINT {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SRQIM
    pub mod SRQIM {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// WUIM
    pub mod WUIM {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This description is for register OTG_GRXSTSR in Device mode. A read to the receive status debug read register returns the contents of the top of the receive FIFO. The core ignores the receive status read when the receive FIFO is empty and returns a value of 0x00000000.
pub mod OTG_GRXSTSR {

    /// EPNUM
    pub mod EPNUM {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// BCNT
    pub mod BCNT {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (11 bits: 0x7ff << 4)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DPID
    pub mod DPID {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (2 bits: 0b11 << 15)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PKTSTS
    pub mod PKTSTS {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (4 bits: 0b1111 << 17)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// FRMNUM
    pub mod FRMNUM {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (4 bits: 0b1111 << 21)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// STSPHST
    pub mod STSPHST {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This description is for register OTG_GRXSTSP in Device mode. Similarly to OTG_GRXSTSR (receive status debug read register) where a read returns the contents of the top of the receive FIFO, a read to OTG_GRXSTSP (receive status read and pop register) additionally pops the top data entry out of the Rx FIFO. The core ignores the receive status pop/read when the receive FIFO is empty and returns a value of 0x00000000. The application must only pop the receive status FIFO when the receive FIFO non-empty bit of the core interrupt register (RXFLVL bit in OTG_GINTSTS) is asserted.
pub mod OTG_GRXSTSP {
    pub use super::OTG_GRXSTSR::BCNT;
    pub use super::OTG_GRXSTSR::DPID;
    pub use super::OTG_GRXSTSR::EPNUM;
    pub use super::OTG_GRXSTSR::FRMNUM;
    pub use super::OTG_GRXSTSR::PKTSTS;
    pub use super::OTG_GRXSTSR::STSPHST;
}

/// The application can program the RAM size that must be allocated to the Rx FIFO.
pub mod OTG_GRXFSIZ {

    /// RXFD
    pub mod RXFD {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Host mode
pub mod OTG_HNPTXFSIZ {

    /// NPTXFSA
    pub mod NPTXFSA {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// NPTXFD
    pub mod NPTXFD {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (16 bits: 0xffff << 16)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// In device mode, this register is not valid. This read-only register contains the free space information for the non-periodic Tx FIFO and the non-periodic transmit request queue.
pub mod OTG_HNPTXSTS {

    /// NPTXFSAV
    pub mod NPTXFSAV {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// NPTQXSAV
    pub mod NPTQXSAV {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// NPTXQTOP
    pub mod NPTXQTOP {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (7 bits: 0x7f << 24)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// OTG general core configuration register
pub mod OTG_GCCFG {

    /// PDET
    pub mod PDET {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SDET
    pub mod SDET {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PS2DET
    pub mod PS2DET {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PWRDWN
    pub mod PWRDWN {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// BCDEN
    pub mod BCDEN {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PDEN
    pub mod PDEN {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SDEN
    pub mod SDEN {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// VBDEN
    pub mod VBDEN {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// IDEN
    pub mod IDEN {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This is a register containing the Product ID as reset value.
pub mod OTG_CID {

    /// PRODUCT_ID
    pub mod PRODUCT_ID {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// OTG core LPM configuration register
pub mod OTG_GLPMCFG {

    /// LPMEN
    pub mod LPMEN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LPMACK
    pub mod LPMACK {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// BESL
    pub mod BESL {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (4 bits: 0b1111 << 2)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// REMWAKE
    pub mod REMWAKE {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// L1SSEN
    pub mod L1SSEN {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// BESLTHRS
    pub mod BESLTHRS {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// L1DSEN
    pub mod L1DSEN {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LPMRSP
    pub mod LPMRSP {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (2 bits: 0b11 << 13)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SLPSTS
    pub mod SLPSTS {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// L1RSMOK
    pub mod L1RSMOK {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LPMCHIDX
    pub mod LPMCHIDX {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (4 bits: 0b1111 << 17)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LPMRCNT
    pub mod LPMRCNT {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (3 bits: 0b111 << 21)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SNDLPM
    pub mod SNDLPM {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LPMRCNTSTS
    pub mod LPMRCNTSTS {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (3 bits: 0b111 << 25)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ENBESL
    pub mod ENBESL {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// OTG host periodic transmit FIFO size register
pub mod OTG_HPTXFSIZ {

    /// PTXSA
    pub mod PTXSA {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PTXFSIZ
    pub mod PTXFSIZ {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (16 bits: 0xffff << 16)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// OTG device IN endpoint transmit FIFO 1 size register
pub mod OTG_DIEPTXF1 {

    /// INEPTXSA
    pub mod INEPTXSA {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// INEPTXFD
    pub mod INEPTXFD {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (16 bits: 0xffff << 16)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// OTG device IN endpoint transmit FIFO 2 size register
pub mod OTG_DIEPTXF2 {
    pub use super::OTG_DIEPTXF1::INEPTXFD;
    pub use super::OTG_DIEPTXF1::INEPTXSA;
}

/// OTG device IN endpoint transmit FIFO 3 size register
pub mod OTG_DIEPTXF3 {
    pub use super::OTG_DIEPTXF1::INEPTXFD;
    pub use super::OTG_DIEPTXF1::INEPTXSA;
}

/// OTG device IN endpoint transmit FIFO 4 size register
pub mod OTG_DIEPTXF4 {
    pub use super::OTG_DIEPTXF1::INEPTXFD;
    pub use super::OTG_DIEPTXF1::INEPTXSA;
}

/// OTG device IN endpoint transmit FIFO 5 size register
pub mod OTG_DIEPTXF5 {
    pub use super::OTG_DIEPTXF1::INEPTXFD;
    pub use super::OTG_DIEPTXF1::INEPTXSA;
}

/// OTG device IN endpoint transmit FIFO 6 size register
pub mod OTG_DIEPTXF6 {
    pub use super::OTG_DIEPTXF1::INEPTXFD;
    pub use super::OTG_DIEPTXF1::INEPTXSA;
}

/// OTG device IN endpoint transmit FIFO 7 size register
pub mod OTG_DIEPTXF7 {
    pub use super::OTG_DIEPTXF1::INEPTXFD;
    pub use super::OTG_DIEPTXF1::INEPTXSA;
}

/// OTG device IN endpoint transmit FIFO 8 size register
pub mod OTG_DIEPTXF8 {
    pub use super::OTG_DIEPTXF1::INEPTXFD;
    pub use super::OTG_DIEPTXF1::INEPTXSA;
}

/// This register configures the core after power-on. Do not make changes to this register after initializing the host.
pub mod OTG_HCFG {

    /// FSLSPCS
    pub mod FSLSPCS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// FSLSS
    pub mod FSLSS {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DESCDMA
    pub mod DESCDMA {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// FRLSTEN
    pub mod FRLSTEN {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (2 bits: 0b11 << 24)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PERSSCHEDENA
    pub mod PERSSCHEDENA {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register stores the frame interval information for the current speed to which the OTG controller has enumerated.
pub mod OTG_HFIR {

    /// FRIVL
    pub mod FRIVL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RLDCTRL
    pub mod RLDCTRL {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register indicates the current frame number. It also indicates the time remaining (in terms of the number of PHY clocks) in the current frame.
pub mod OTG_HFNUM {

    /// FRNUM
    pub mod FRNUM {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// FTREM
    pub mod FTREM {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (16 bits: 0xffff << 16)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This read-only register contains the free space information for the periodic Tx FIFO and the periodic transmit request queue.
pub mod OTG_HPTXSTS {

    /// PTXFSAVL
    pub mod PTXFSAVL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PTXQSAV
    pub mod PTXQSAV {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PTXQTOP
    pub mod PTXQTOP {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (8 bits: 0xff << 24)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// When a significant event occurs on a channel, the host all channels interrupt register interrupts the application using the host channels interrupt bit of the core interrupt register (HCINT bit in OTG_GINTSTS). This is shown in Figure724. There is one interrupt bit per channel, up to a maximum of 16 bits. Bits in this register are set and cleared when the application sets and clears bits in the corresponding host channel-x interrupt register.
pub mod OTG_HAINT {

    /// HAINT
    pub mod HAINT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// The host all channel interrupt mask register works with the host all channel interrupt register to interrupt the application when an event occurs on a channel. There is one interrupt mask bit per channel, up to a maximum of 16 bits.
pub mod OTG_HAINTMSK {

    /// HAINTM
    pub mod HAINTM {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register holds the starting address of the frame list information (scatter/gather mode).
pub mod OTG_HFLBADDR {

    /// HFLBADDR
    pub mod HFLBADDR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register is available only in host mode. Currently, the OTG host supports only one port. A single register holds USB port-related information such as USB reset, enable, suspend, resume, connect status, and test mode for each port. It is shown in Figure724. The rc_w1 bits in this register can trigger an interrupt to the application through the host port interrupt bit of the core interrupt register (HPRTINT bit in OTG_GINTSTS). On a port interrupt, the application must read this register and clear the bit that caused the interrupt. For the rc_w1 bits, the application must write a 1 to the bit to clear the interrupt.
pub mod OTG_HPRT {

    /// PCSTS
    pub mod PCSTS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PCDET
    pub mod PCDET {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PENA
    pub mod PENA {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PENCHNG
    pub mod PENCHNG {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// POCA
    pub mod POCA {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// POCCHNG
    pub mod POCCHNG {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PRES
    pub mod PRES {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PSUSP
    pub mod PSUSP {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PRST
    pub mod PRST {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PLSTS
    pub mod PLSTS {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (2 bits: 0b11 << 10)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PPWR
    pub mod PPWR {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PTCTL
    pub mod PTCTL {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (4 bits: 0b1111 << 13)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PSPD
    pub mod PSPD {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (2 bits: 0b11 << 17)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// OTG host channel 0 characteristics register
pub mod OTG_HCCHAR0 {

    /// MPSIZ
    pub mod MPSIZ {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (11 bits: 0x7ff << 0)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// EPNUM
    pub mod EPNUM {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (4 bits: 0b1111 << 11)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// EPDIR
    pub mod EPDIR {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LSDEV
    pub mod LSDEV {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// EPTYP
    pub mod EPTYP {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (2 bits: 0b11 << 18)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// MCNT
    pub mod MCNT {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (2 bits: 0b11 << 20)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DAD
    pub mod DAD {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (7 bits: 0x7f << 22)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CHDIS
    pub mod CHDIS {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CHENA
    pub mod CHENA {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// OTG host channel 0 split control register
pub mod OTG_HCSPLT0 {

    /// PRTADDR
    pub mod PRTADDR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// HUBADDR
    pub mod HUBADDR {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (7 bits: 0x7f << 7)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// XACTPOS
    pub mod XACTPOS {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (2 bits: 0b11 << 14)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// COMPLSPLT
    pub mod COMPLSPLT {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SPLITEN
    pub mod SPLITEN {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.
pub mod OTG_HCINT0 {

    /// XFRC
    pub mod XFRC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CHH
    pub mod CHH {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// AHBERR
    pub mod AHBERR {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// STALL
    pub mod STALL {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// NAK
    pub mod NAK {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ACK
    pub mod ACK {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// NYET
    pub mod NYET {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TXERR
    pub mod TXERR {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// BBERR
    pub mod BBERR {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// FRMOR
    pub mod FRMOR {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DTERR
    pub mod DTERR {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// BNA
    pub mod BNA {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// XCSXACTERR
    pub mod XCSXACTERR {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DESCLSTROLL
    pub mod DESCLSTROLL {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register reflects the mask for each channel status described in the previous section.
pub mod OTG_HCINTMSK0 {

    /// XFRCM
    pub mod XFRCM {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CHHM
    pub mod CHHM {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// AHBERRM
    pub mod AHBERRM {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// STALLM
    pub mod STALLM {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// NAKM
    pub mod NAKM {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ACKM
    pub mod ACKM {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// NYET
    pub mod NYET {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TXERRM
    pub mod TXERRM {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// BBERRM
    pub mod BBERRM {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// FRMORM
    pub mod FRMORM {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DTERRM
    pub mod DTERRM {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// BNAMSK
    pub mod BNAMSK {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DESCLSTROLLMSK
    pub mod DESCLSTROLLMSK {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// OTG host channel 0 transfer size register
pub mod OTG_HCTSIZ0 {

    /// XFRSIZ
    pub mod XFRSIZ {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (19 bits: 0x7ffff << 0)
        pub const mask: u32 = 0x7ffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PKTCNT
    pub mod PKTCNT {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (10 bits: 0x3ff << 19)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DPID
    pub mod DPID {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (2 bits: 0b11 << 29)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// OTG host channel 0 DMA address register in buffer DMA \[alternate\]
pub mod OTG_HCDMA0 {

    /// DMAADDR
    pub mod DMAADDR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// OTG host channel-n DMA address buffer register
pub mod OTG_HCDMAB0 {

    /// HCDMAB
    pub mod HCDMAB {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// OTG host channel 1 characteristics register
pub mod OTG_HCCHAR1 {
    pub use super::OTG_HCCHAR0::CHDIS;
    pub use super::OTG_HCCHAR0::CHENA;
    pub use super::OTG_HCCHAR0::DAD;
    pub use super::OTG_HCCHAR0::EPDIR;
    pub use super::OTG_HCCHAR0::EPNUM;
    pub use super::OTG_HCCHAR0::EPTYP;
    pub use super::OTG_HCCHAR0::LSDEV;
    pub use super::OTG_HCCHAR0::MCNT;
    pub use super::OTG_HCCHAR0::MPSIZ;
}

/// OTG host channel 1 split control register
pub mod OTG_HCSPLT1 {
    pub use super::OTG_HCSPLT0::COMPLSPLT;
    pub use super::OTG_HCSPLT0::HUBADDR;
    pub use super::OTG_HCSPLT0::PRTADDR;
    pub use super::OTG_HCSPLT0::SPLITEN;
    pub use super::OTG_HCSPLT0::XACTPOS;
}

/// This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.
pub mod OTG_HCINT1 {
    pub use super::OTG_HCINT0::ACK;
    pub use super::OTG_HCINT0::AHBERR;
    pub use super::OTG_HCINT0::BBERR;
    pub use super::OTG_HCINT0::BNA;
    pub use super::OTG_HCINT0::CHH;
    pub use super::OTG_HCINT0::DESCLSTROLL;
    pub use super::OTG_HCINT0::DTERR;
    pub use super::OTG_HCINT0::FRMOR;
    pub use super::OTG_HCINT0::NAK;
    pub use super::OTG_HCINT0::NYET;
    pub use super::OTG_HCINT0::STALL;
    pub use super::OTG_HCINT0::TXERR;
    pub use super::OTG_HCINT0::XCSXACTERR;
    pub use super::OTG_HCINT0::XFRC;
}

/// This register reflects the mask for each channel status described in the previous section.
pub mod OTG_HCINTMSK1 {
    pub use super::OTG_HCINTMSK0::ACKM;
    pub use super::OTG_HCINTMSK0::AHBERRM;
    pub use super::OTG_HCINTMSK0::BBERRM;
    pub use super::OTG_HCINTMSK0::BNAMSK;
    pub use super::OTG_HCINTMSK0::CHHM;
    pub use super::OTG_HCINTMSK0::DESCLSTROLLMSK;
    pub use super::OTG_HCINTMSK0::DTERRM;
    pub use super::OTG_HCINTMSK0::FRMORM;
    pub use super::OTG_HCINTMSK0::NAKM;
    pub use super::OTG_HCINTMSK0::NYET;
    pub use super::OTG_HCINTMSK0::STALLM;
    pub use super::OTG_HCINTMSK0::TXERRM;
    pub use super::OTG_HCINTMSK0::XFRCM;
}

/// OTG host channel 1 transfer size register
pub mod OTG_HCTSIZ1 {
    pub use super::OTG_HCTSIZ0::DPID;
    pub use super::OTG_HCTSIZ0::PKTCNT;
    pub use super::OTG_HCTSIZ0::XFRSIZ;
}

/// OTG host channel 1 DMA address register in buffer DMA \[alternate\]
pub mod OTG_HCDMA1 {
    pub use super::OTG_HCDMA0::DMAADDR;
}

/// OTG host channel-n DMA address buffer register
pub mod OTG_HCDMAB1 {
    pub use super::OTG_HCDMAB0::HCDMAB;
}

/// OTG host channel 2 characteristics register
pub mod OTG_HCCHAR2 {
    pub use super::OTG_HCCHAR0::CHDIS;
    pub use super::OTG_HCCHAR0::CHENA;
    pub use super::OTG_HCCHAR0::DAD;
    pub use super::OTG_HCCHAR0::EPDIR;
    pub use super::OTG_HCCHAR0::EPNUM;
    pub use super::OTG_HCCHAR0::EPTYP;
    pub use super::OTG_HCCHAR0::LSDEV;
    pub use super::OTG_HCCHAR0::MCNT;
    pub use super::OTG_HCCHAR0::MPSIZ;
}

/// OTG host channel 2 split control register
pub mod OTG_HCSPLT2 {
    pub use super::OTG_HCSPLT0::COMPLSPLT;
    pub use super::OTG_HCSPLT0::HUBADDR;
    pub use super::OTG_HCSPLT0::PRTADDR;
    pub use super::OTG_HCSPLT0::SPLITEN;
    pub use super::OTG_HCSPLT0::XACTPOS;
}

/// This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.
pub mod OTG_HCINT2 {
    pub use super::OTG_HCINT0::ACK;
    pub use super::OTG_HCINT0::AHBERR;
    pub use super::OTG_HCINT0::BBERR;
    pub use super::OTG_HCINT0::BNA;
    pub use super::OTG_HCINT0::CHH;
    pub use super::OTG_HCINT0::DESCLSTROLL;
    pub use super::OTG_HCINT0::DTERR;
    pub use super::OTG_HCINT0::FRMOR;
    pub use super::OTG_HCINT0::NAK;
    pub use super::OTG_HCINT0::NYET;
    pub use super::OTG_HCINT0::STALL;
    pub use super::OTG_HCINT0::TXERR;
    pub use super::OTG_HCINT0::XCSXACTERR;
    pub use super::OTG_HCINT0::XFRC;
}

/// This register reflects the mask for each channel status described in the previous section.
pub mod OTG_HCINTMSK2 {
    pub use super::OTG_HCINTMSK0::ACKM;
    pub use super::OTG_HCINTMSK0::AHBERRM;
    pub use super::OTG_HCINTMSK0::BBERRM;
    pub use super::OTG_HCINTMSK0::BNAMSK;
    pub use super::OTG_HCINTMSK0::CHHM;
    pub use super::OTG_HCINTMSK0::DESCLSTROLLMSK;
    pub use super::OTG_HCINTMSK0::DTERRM;
    pub use super::OTG_HCINTMSK0::FRMORM;
    pub use super::OTG_HCINTMSK0::NAKM;
    pub use super::OTG_HCINTMSK0::NYET;
    pub use super::OTG_HCINTMSK0::STALLM;
    pub use super::OTG_HCINTMSK0::TXERRM;
    pub use super::OTG_HCINTMSK0::XFRCM;
}

/// OTG host channel 2 transfer size register
pub mod OTG_HCTSIZ2 {
    pub use super::OTG_HCTSIZ0::DPID;
    pub use super::OTG_HCTSIZ0::PKTCNT;
    pub use super::OTG_HCTSIZ0::XFRSIZ;
}

/// OTG host channel 2 DMA address register in buffer DMA \[alternate\]
pub mod OTG_HCDMA2 {
    pub use super::OTG_HCDMA0::DMAADDR;
}

/// OTG host channel-n DMA address buffer register
pub mod OTG_HCDMAB2 {
    pub use super::OTG_HCDMAB0::HCDMAB;
}

/// OTG host channel 3 characteristics register
pub mod OTG_HCCHAR3 {
    pub use super::OTG_HCCHAR0::CHDIS;
    pub use super::OTG_HCCHAR0::CHENA;
    pub use super::OTG_HCCHAR0::DAD;
    pub use super::OTG_HCCHAR0::EPDIR;
    pub use super::OTG_HCCHAR0::EPNUM;
    pub use super::OTG_HCCHAR0::EPTYP;
    pub use super::OTG_HCCHAR0::LSDEV;
    pub use super::OTG_HCCHAR0::MCNT;
    pub use super::OTG_HCCHAR0::MPSIZ;
}

/// OTG host channel 3 split control register
pub mod OTG_HCSPLT3 {
    pub use super::OTG_HCSPLT0::COMPLSPLT;
    pub use super::OTG_HCSPLT0::HUBADDR;
    pub use super::OTG_HCSPLT0::PRTADDR;
    pub use super::OTG_HCSPLT0::SPLITEN;
    pub use super::OTG_HCSPLT0::XACTPOS;
}

/// This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.
pub mod OTG_HCINT3 {
    pub use super::OTG_HCINT0::ACK;
    pub use super::OTG_HCINT0::AHBERR;
    pub use super::OTG_HCINT0::BBERR;
    pub use super::OTG_HCINT0::BNA;
    pub use super::OTG_HCINT0::CHH;
    pub use super::OTG_HCINT0::DESCLSTROLL;
    pub use super::OTG_HCINT0::DTERR;
    pub use super::OTG_HCINT0::FRMOR;
    pub use super::OTG_HCINT0::NAK;
    pub use super::OTG_HCINT0::NYET;
    pub use super::OTG_HCINT0::STALL;
    pub use super::OTG_HCINT0::TXERR;
    pub use super::OTG_HCINT0::XCSXACTERR;
    pub use super::OTG_HCINT0::XFRC;
}

/// This register reflects the mask for each channel status described in the previous section.
pub mod OTG_HCINTMSK3 {
    pub use super::OTG_HCINTMSK0::ACKM;
    pub use super::OTG_HCINTMSK0::AHBERRM;
    pub use super::OTG_HCINTMSK0::BBERRM;
    pub use super::OTG_HCINTMSK0::BNAMSK;
    pub use super::OTG_HCINTMSK0::CHHM;
    pub use super::OTG_HCINTMSK0::DESCLSTROLLMSK;
    pub use super::OTG_HCINTMSK0::DTERRM;
    pub use super::OTG_HCINTMSK0::FRMORM;
    pub use super::OTG_HCINTMSK0::NAKM;
    pub use super::OTG_HCINTMSK0::NYET;
    pub use super::OTG_HCINTMSK0::STALLM;
    pub use super::OTG_HCINTMSK0::TXERRM;
    pub use super::OTG_HCINTMSK0::XFRCM;
}

/// OTG host channel 3 transfer size register
pub mod OTG_HCTSIZ3 {
    pub use super::OTG_HCTSIZ0::DPID;
    pub use super::OTG_HCTSIZ0::PKTCNT;
    pub use super::OTG_HCTSIZ0::XFRSIZ;
}

/// OTG host channel 3 DMA address register in buffer DMA \[alternate\]
pub mod OTG_HCDMA3 {
    pub use super::OTG_HCDMA0::DMAADDR;
}

/// OTG host channel-n DMA address buffer register
pub mod OTG_HCDMAB3 {
    pub use super::OTG_HCDMAB0::HCDMAB;
}

/// OTG host channel 4 characteristics register
pub mod OTG_HCCHAR4 {
    pub use super::OTG_HCCHAR0::CHDIS;
    pub use super::OTG_HCCHAR0::CHENA;
    pub use super::OTG_HCCHAR0::DAD;
    pub use super::OTG_HCCHAR0::EPDIR;
    pub use super::OTG_HCCHAR0::EPNUM;
    pub use super::OTG_HCCHAR0::EPTYP;
    pub use super::OTG_HCCHAR0::LSDEV;
    pub use super::OTG_HCCHAR0::MCNT;
    pub use super::OTG_HCCHAR0::MPSIZ;
}

/// OTG host channel 4 split control register
pub mod OTG_HCSPLT4 {
    pub use super::OTG_HCSPLT0::COMPLSPLT;
    pub use super::OTG_HCSPLT0::HUBADDR;
    pub use super::OTG_HCSPLT0::PRTADDR;
    pub use super::OTG_HCSPLT0::SPLITEN;
    pub use super::OTG_HCSPLT0::XACTPOS;
}

/// This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.
pub mod OTG_HCINT4 {
    pub use super::OTG_HCINT0::ACK;
    pub use super::OTG_HCINT0::AHBERR;
    pub use super::OTG_HCINT0::BBERR;
    pub use super::OTG_HCINT0::BNA;
    pub use super::OTG_HCINT0::CHH;
    pub use super::OTG_HCINT0::DESCLSTROLL;
    pub use super::OTG_HCINT0::DTERR;
    pub use super::OTG_HCINT0::FRMOR;
    pub use super::OTG_HCINT0::NAK;
    pub use super::OTG_HCINT0::NYET;
    pub use super::OTG_HCINT0::STALL;
    pub use super::OTG_HCINT0::TXERR;
    pub use super::OTG_HCINT0::XCSXACTERR;
    pub use super::OTG_HCINT0::XFRC;
}

/// This register reflects the mask for each channel status described in the previous section.
pub mod OTG_HCINTMSK4 {
    pub use super::OTG_HCINTMSK0::ACKM;
    pub use super::OTG_HCINTMSK0::AHBERRM;
    pub use super::OTG_HCINTMSK0::BBERRM;
    pub use super::OTG_HCINTMSK0::BNAMSK;
    pub use super::OTG_HCINTMSK0::CHHM;
    pub use super::OTG_HCINTMSK0::DESCLSTROLLMSK;
    pub use super::OTG_HCINTMSK0::DTERRM;
    pub use super::OTG_HCINTMSK0::FRMORM;
    pub use super::OTG_HCINTMSK0::NAKM;
    pub use super::OTG_HCINTMSK0::NYET;
    pub use super::OTG_HCINTMSK0::STALLM;
    pub use super::OTG_HCINTMSK0::TXERRM;
    pub use super::OTG_HCINTMSK0::XFRCM;
}

/// OTG host channel 4 transfer size register
pub mod OTG_HCTSIZ4 {
    pub use super::OTG_HCTSIZ0::DPID;
    pub use super::OTG_HCTSIZ0::PKTCNT;
    pub use super::OTG_HCTSIZ0::XFRSIZ;
}

/// OTG host channel 4 DMA address register in buffer DMA \[alternate\]
pub mod OTG_HCDMA4 {
    pub use super::OTG_HCDMA0::DMAADDR;
}

/// OTG host channel-n DMA address buffer register
pub mod OTG_HCDMAB4 {
    pub use super::OTG_HCDMAB0::HCDMAB;
}

/// OTG host channel 5 characteristics register
pub mod OTG_HCCHAR5 {
    pub use super::OTG_HCCHAR0::CHDIS;
    pub use super::OTG_HCCHAR0::CHENA;
    pub use super::OTG_HCCHAR0::DAD;
    pub use super::OTG_HCCHAR0::EPDIR;
    pub use super::OTG_HCCHAR0::EPNUM;
    pub use super::OTG_HCCHAR0::EPTYP;
    pub use super::OTG_HCCHAR0::LSDEV;
    pub use super::OTG_HCCHAR0::MCNT;
    pub use super::OTG_HCCHAR0::MPSIZ;
}

/// OTG host channel 5 split control register
pub mod OTG_HCSPLT5 {
    pub use super::OTG_HCSPLT0::COMPLSPLT;
    pub use super::OTG_HCSPLT0::HUBADDR;
    pub use super::OTG_HCSPLT0::PRTADDR;
    pub use super::OTG_HCSPLT0::SPLITEN;
    pub use super::OTG_HCSPLT0::XACTPOS;
}

/// This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.
pub mod OTG_HCINT5 {
    pub use super::OTG_HCINT0::ACK;
    pub use super::OTG_HCINT0::AHBERR;
    pub use super::OTG_HCINT0::BBERR;
    pub use super::OTG_HCINT0::BNA;
    pub use super::OTG_HCINT0::CHH;
    pub use super::OTG_HCINT0::DESCLSTROLL;
    pub use super::OTG_HCINT0::DTERR;
    pub use super::OTG_HCINT0::FRMOR;
    pub use super::OTG_HCINT0::NAK;
    pub use super::OTG_HCINT0::NYET;
    pub use super::OTG_HCINT0::STALL;
    pub use super::OTG_HCINT0::TXERR;
    pub use super::OTG_HCINT0::XCSXACTERR;
    pub use super::OTG_HCINT0::XFRC;
}

/// This register reflects the mask for each channel status described in the previous section.
pub mod OTG_HCINTMSK5 {
    pub use super::OTG_HCINTMSK0::ACKM;
    pub use super::OTG_HCINTMSK0::AHBERRM;
    pub use super::OTG_HCINTMSK0::BBERRM;
    pub use super::OTG_HCINTMSK0::BNAMSK;
    pub use super::OTG_HCINTMSK0::CHHM;
    pub use super::OTG_HCINTMSK0::DESCLSTROLLMSK;
    pub use super::OTG_HCINTMSK0::DTERRM;
    pub use super::OTG_HCINTMSK0::FRMORM;
    pub use super::OTG_HCINTMSK0::NAKM;
    pub use super::OTG_HCINTMSK0::NYET;
    pub use super::OTG_HCINTMSK0::STALLM;
    pub use super::OTG_HCINTMSK0::TXERRM;
    pub use super::OTG_HCINTMSK0::XFRCM;
}

/// OTG host channel 5 transfer size register
pub mod OTG_HCTSIZ5 {
    pub use super::OTG_HCTSIZ0::DPID;
    pub use super::OTG_HCTSIZ0::PKTCNT;
    pub use super::OTG_HCTSIZ0::XFRSIZ;
}

/// OTG host channel 5 DMA address register in buffer DMA \[alternate\]
pub mod OTG_HCDMA5 {
    pub use super::OTG_HCDMA0::DMAADDR;
}

/// OTG host channel-n DMA address buffer register
pub mod OTG_HCDMAB5 {
    pub use super::OTG_HCDMAB0::HCDMAB;
}

/// OTG host channel 6 characteristics register
pub mod OTG_HCCHAR6 {
    pub use super::OTG_HCCHAR0::CHDIS;
    pub use super::OTG_HCCHAR0::CHENA;
    pub use super::OTG_HCCHAR0::DAD;
    pub use super::OTG_HCCHAR0::EPDIR;
    pub use super::OTG_HCCHAR0::EPNUM;
    pub use super::OTG_HCCHAR0::EPTYP;
    pub use super::OTG_HCCHAR0::LSDEV;
    pub use super::OTG_HCCHAR0::MCNT;
    pub use super::OTG_HCCHAR0::MPSIZ;
}

/// OTG host channel 6 split control register
pub mod OTG_HCSPLT6 {
    pub use super::OTG_HCSPLT0::COMPLSPLT;
    pub use super::OTG_HCSPLT0::HUBADDR;
    pub use super::OTG_HCSPLT0::PRTADDR;
    pub use super::OTG_HCSPLT0::SPLITEN;
    pub use super::OTG_HCSPLT0::XACTPOS;
}

/// This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.
pub mod OTG_HCINT6 {
    pub use super::OTG_HCINT0::ACK;
    pub use super::OTG_HCINT0::AHBERR;
    pub use super::OTG_HCINT0::BBERR;
    pub use super::OTG_HCINT0::BNA;
    pub use super::OTG_HCINT0::CHH;
    pub use super::OTG_HCINT0::DESCLSTROLL;
    pub use super::OTG_HCINT0::DTERR;
    pub use super::OTG_HCINT0::FRMOR;
    pub use super::OTG_HCINT0::NAK;
    pub use super::OTG_HCINT0::NYET;
    pub use super::OTG_HCINT0::STALL;
    pub use super::OTG_HCINT0::TXERR;
    pub use super::OTG_HCINT0::XCSXACTERR;
    pub use super::OTG_HCINT0::XFRC;
}

/// This register reflects the mask for each channel status described in the previous section.
pub mod OTG_HCINTMSK6 {
    pub use super::OTG_HCINTMSK0::ACKM;
    pub use super::OTG_HCINTMSK0::AHBERRM;
    pub use super::OTG_HCINTMSK0::BBERRM;
    pub use super::OTG_HCINTMSK0::BNAMSK;
    pub use super::OTG_HCINTMSK0::CHHM;
    pub use super::OTG_HCINTMSK0::DESCLSTROLLMSK;
    pub use super::OTG_HCINTMSK0::DTERRM;
    pub use super::OTG_HCINTMSK0::FRMORM;
    pub use super::OTG_HCINTMSK0::NAKM;
    pub use super::OTG_HCINTMSK0::NYET;
    pub use super::OTG_HCINTMSK0::STALLM;
    pub use super::OTG_HCINTMSK0::TXERRM;
    pub use super::OTG_HCINTMSK0::XFRCM;
}

/// OTG host channel 6 transfer size register
pub mod OTG_HCTSIZ6 {
    pub use super::OTG_HCTSIZ0::DPID;
    pub use super::OTG_HCTSIZ0::PKTCNT;
    pub use super::OTG_HCTSIZ0::XFRSIZ;
}

/// OTG host channel 6 DMA address register in buffer DMA \[alternate\]
pub mod OTG_HCDMA6 {
    pub use super::OTG_HCDMA0::DMAADDR;
}

/// OTG host channel-n DMA address buffer register
pub mod OTG_HCDMAB6 {
    pub use super::OTG_HCDMAB0::HCDMAB;
}

/// OTG host channel 7 characteristics register
pub mod OTG_HCCHAR7 {
    pub use super::OTG_HCCHAR0::CHDIS;
    pub use super::OTG_HCCHAR0::CHENA;
    pub use super::OTG_HCCHAR0::DAD;
    pub use super::OTG_HCCHAR0::EPDIR;
    pub use super::OTG_HCCHAR0::EPNUM;
    pub use super::OTG_HCCHAR0::EPTYP;
    pub use super::OTG_HCCHAR0::LSDEV;
    pub use super::OTG_HCCHAR0::MCNT;
    pub use super::OTG_HCCHAR0::MPSIZ;
}

/// OTG host channel 7 split control register
pub mod OTG_HCSPLT7 {
    pub use super::OTG_HCSPLT0::COMPLSPLT;
    pub use super::OTG_HCSPLT0::HUBADDR;
    pub use super::OTG_HCSPLT0::PRTADDR;
    pub use super::OTG_HCSPLT0::SPLITEN;
    pub use super::OTG_HCSPLT0::XACTPOS;
}

/// This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.
pub mod OTG_HCINT7 {
    pub use super::OTG_HCINT0::ACK;
    pub use super::OTG_HCINT0::AHBERR;
    pub use super::OTG_HCINT0::BBERR;
    pub use super::OTG_HCINT0::BNA;
    pub use super::OTG_HCINT0::CHH;
    pub use super::OTG_HCINT0::DESCLSTROLL;
    pub use super::OTG_HCINT0::DTERR;
    pub use super::OTG_HCINT0::FRMOR;
    pub use super::OTG_HCINT0::NAK;
    pub use super::OTG_HCINT0::NYET;
    pub use super::OTG_HCINT0::STALL;
    pub use super::OTG_HCINT0::TXERR;
    pub use super::OTG_HCINT0::XCSXACTERR;
    pub use super::OTG_HCINT0::XFRC;
}

/// This register reflects the mask for each channel status described in the previous section.
pub mod OTG_HCINTMSK7 {
    pub use super::OTG_HCINTMSK0::ACKM;
    pub use super::OTG_HCINTMSK0::AHBERRM;
    pub use super::OTG_HCINTMSK0::BBERRM;
    pub use super::OTG_HCINTMSK0::BNAMSK;
    pub use super::OTG_HCINTMSK0::CHHM;
    pub use super::OTG_HCINTMSK0::DESCLSTROLLMSK;
    pub use super::OTG_HCINTMSK0::DTERRM;
    pub use super::OTG_HCINTMSK0::FRMORM;
    pub use super::OTG_HCINTMSK0::NAKM;
    pub use super::OTG_HCINTMSK0::NYET;
    pub use super::OTG_HCINTMSK0::STALLM;
    pub use super::OTG_HCINTMSK0::TXERRM;
    pub use super::OTG_HCINTMSK0::XFRCM;
}

/// OTG host channel 7 transfer size register
pub mod OTG_HCTSIZ7 {
    pub use super::OTG_HCTSIZ0::DPID;
    pub use super::OTG_HCTSIZ0::PKTCNT;
    pub use super::OTG_HCTSIZ0::XFRSIZ;
}

/// OTG host channel 7 DMA address register in buffer DMA \[alternate\]
pub mod OTG_HCDMA7 {
    pub use super::OTG_HCDMA0::DMAADDR;
}

/// OTG host channel-n DMA address buffer register
pub mod OTG_HCDMAB7 {
    pub use super::OTG_HCDMAB0::HCDMAB;
}

/// OTG host channel 8 characteristics register
pub mod OTG_HCCHAR8 {
    pub use super::OTG_HCCHAR0::CHDIS;
    pub use super::OTG_HCCHAR0::CHENA;
    pub use super::OTG_HCCHAR0::DAD;
    pub use super::OTG_HCCHAR0::EPDIR;
    pub use super::OTG_HCCHAR0::EPNUM;
    pub use super::OTG_HCCHAR0::EPTYP;
    pub use super::OTG_HCCHAR0::LSDEV;
    pub use super::OTG_HCCHAR0::MCNT;
    pub use super::OTG_HCCHAR0::MPSIZ;
}

/// OTG host channel 8 split control register
pub mod OTG_HCSPLT8 {
    pub use super::OTG_HCSPLT0::COMPLSPLT;
    pub use super::OTG_HCSPLT0::HUBADDR;
    pub use super::OTG_HCSPLT0::PRTADDR;
    pub use super::OTG_HCSPLT0::SPLITEN;
    pub use super::OTG_HCSPLT0::XACTPOS;
}

/// This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.
pub mod OTG_HCINT8 {
    pub use super::OTG_HCINT0::ACK;
    pub use super::OTG_HCINT0::AHBERR;
    pub use super::OTG_HCINT0::BBERR;
    pub use super::OTG_HCINT0::BNA;
    pub use super::OTG_HCINT0::CHH;
    pub use super::OTG_HCINT0::DESCLSTROLL;
    pub use super::OTG_HCINT0::DTERR;
    pub use super::OTG_HCINT0::FRMOR;
    pub use super::OTG_HCINT0::NAK;
    pub use super::OTG_HCINT0::NYET;
    pub use super::OTG_HCINT0::STALL;
    pub use super::OTG_HCINT0::TXERR;
    pub use super::OTG_HCINT0::XCSXACTERR;
    pub use super::OTG_HCINT0::XFRC;
}

/// This register reflects the mask for each channel status described in the previous section.
pub mod OTG_HCINTMSK8 {
    pub use super::OTG_HCINTMSK0::ACKM;
    pub use super::OTG_HCINTMSK0::AHBERRM;
    pub use super::OTG_HCINTMSK0::BBERRM;
    pub use super::OTG_HCINTMSK0::BNAMSK;
    pub use super::OTG_HCINTMSK0::CHHM;
    pub use super::OTG_HCINTMSK0::DESCLSTROLLMSK;
    pub use super::OTG_HCINTMSK0::DTERRM;
    pub use super::OTG_HCINTMSK0::FRMORM;
    pub use super::OTG_HCINTMSK0::NAKM;
    pub use super::OTG_HCINTMSK0::NYET;
    pub use super::OTG_HCINTMSK0::STALLM;
    pub use super::OTG_HCINTMSK0::TXERRM;
    pub use super::OTG_HCINTMSK0::XFRCM;
}

/// OTG host channel 8 transfer size register
pub mod OTG_HCTSIZ8 {
    pub use super::OTG_HCTSIZ0::DPID;
    pub use super::OTG_HCTSIZ0::PKTCNT;
    pub use super::OTG_HCTSIZ0::XFRSIZ;
}

/// OTG host channel 8 DMA address register in buffer DMA \[alternate\]
pub mod OTG_HCDMA8 {
    pub use super::OTG_HCDMA0::DMAADDR;
}

/// OTG host channel-n DMA address buffer register
pub mod OTG_HCDMAB8 {
    pub use super::OTG_HCDMAB0::HCDMAB;
}

/// OTG host channel 9 characteristics register
pub mod OTG_HCCHAR9 {
    pub use super::OTG_HCCHAR0::CHDIS;
    pub use super::OTG_HCCHAR0::CHENA;
    pub use super::OTG_HCCHAR0::DAD;
    pub use super::OTG_HCCHAR0::EPDIR;
    pub use super::OTG_HCCHAR0::EPNUM;
    pub use super::OTG_HCCHAR0::EPTYP;
    pub use super::OTG_HCCHAR0::LSDEV;
    pub use super::OTG_HCCHAR0::MCNT;
    pub use super::OTG_HCCHAR0::MPSIZ;
}

/// OTG host channel 9 split control register
pub mod OTG_HCSPLT9 {
    pub use super::OTG_HCSPLT0::COMPLSPLT;
    pub use super::OTG_HCSPLT0::HUBADDR;
    pub use super::OTG_HCSPLT0::PRTADDR;
    pub use super::OTG_HCSPLT0::SPLITEN;
    pub use super::OTG_HCSPLT0::XACTPOS;
}

/// This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.
pub mod OTG_HCINT9 {
    pub use super::OTG_HCINT0::ACK;
    pub use super::OTG_HCINT0::AHBERR;
    pub use super::OTG_HCINT0::BBERR;
    pub use super::OTG_HCINT0::BNA;
    pub use super::OTG_HCINT0::CHH;
    pub use super::OTG_HCINT0::DESCLSTROLL;
    pub use super::OTG_HCINT0::DTERR;
    pub use super::OTG_HCINT0::FRMOR;
    pub use super::OTG_HCINT0::NAK;
    pub use super::OTG_HCINT0::NYET;
    pub use super::OTG_HCINT0::STALL;
    pub use super::OTG_HCINT0::TXERR;
    pub use super::OTG_HCINT0::XCSXACTERR;
    pub use super::OTG_HCINT0::XFRC;
}

/// This register reflects the mask for each channel status described in the previous section.
pub mod OTG_HCINTMSK9 {
    pub use super::OTG_HCINTMSK0::ACKM;
    pub use super::OTG_HCINTMSK0::AHBERRM;
    pub use super::OTG_HCINTMSK0::BBERRM;
    pub use super::OTG_HCINTMSK0::BNAMSK;
    pub use super::OTG_HCINTMSK0::CHHM;
    pub use super::OTG_HCINTMSK0::DESCLSTROLLMSK;
    pub use super::OTG_HCINTMSK0::DTERRM;
    pub use super::OTG_HCINTMSK0::FRMORM;
    pub use super::OTG_HCINTMSK0::NAKM;
    pub use super::OTG_HCINTMSK0::NYET;
    pub use super::OTG_HCINTMSK0::STALLM;
    pub use super::OTG_HCINTMSK0::TXERRM;
    pub use super::OTG_HCINTMSK0::XFRCM;
}

/// OTG host channel 9 transfer size register
pub mod OTG_HCTSIZ9 {
    pub use super::OTG_HCTSIZ0::DPID;
    pub use super::OTG_HCTSIZ0::PKTCNT;
    pub use super::OTG_HCTSIZ0::XFRSIZ;
}

/// OTG host channel 9 DMA address register in buffer DMA \[alternate\]
pub mod OTG_HCDMA9 {
    pub use super::OTG_HCDMA0::DMAADDR;
}

/// OTG host channel-n DMA address buffer register
pub mod OTG_HCDMAB9 {
    pub use super::OTG_HCDMAB0::HCDMAB;
}

/// OTG host channel 10 characteristics register
pub mod OTG_HCCHAR10 {
    pub use super::OTG_HCCHAR0::CHDIS;
    pub use super::OTG_HCCHAR0::CHENA;
    pub use super::OTG_HCCHAR0::DAD;
    pub use super::OTG_HCCHAR0::EPDIR;
    pub use super::OTG_HCCHAR0::EPNUM;
    pub use super::OTG_HCCHAR0::EPTYP;
    pub use super::OTG_HCCHAR0::LSDEV;
    pub use super::OTG_HCCHAR0::MCNT;
    pub use super::OTG_HCCHAR0::MPSIZ;
}

/// OTG host channel 10 split control register
pub mod OTG_HCSPLT10 {
    pub use super::OTG_HCSPLT0::COMPLSPLT;
    pub use super::OTG_HCSPLT0::HUBADDR;
    pub use super::OTG_HCSPLT0::PRTADDR;
    pub use super::OTG_HCSPLT0::SPLITEN;
    pub use super::OTG_HCSPLT0::XACTPOS;
}

/// This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.
pub mod OTG_HCINT10 {
    pub use super::OTG_HCINT0::ACK;
    pub use super::OTG_HCINT0::AHBERR;
    pub use super::OTG_HCINT0::BBERR;
    pub use super::OTG_HCINT0::BNA;
    pub use super::OTG_HCINT0::CHH;
    pub use super::OTG_HCINT0::DESCLSTROLL;
    pub use super::OTG_HCINT0::DTERR;
    pub use super::OTG_HCINT0::FRMOR;
    pub use super::OTG_HCINT0::NAK;
    pub use super::OTG_HCINT0::NYET;
    pub use super::OTG_HCINT0::STALL;
    pub use super::OTG_HCINT0::TXERR;
    pub use super::OTG_HCINT0::XCSXACTERR;
    pub use super::OTG_HCINT0::XFRC;
}

/// This register reflects the mask for each channel status described in the previous section.
pub mod OTG_HCINTMSK10 {
    pub use super::OTG_HCINTMSK0::ACKM;
    pub use super::OTG_HCINTMSK0::AHBERRM;
    pub use super::OTG_HCINTMSK0::BBERRM;
    pub use super::OTG_HCINTMSK0::BNAMSK;
    pub use super::OTG_HCINTMSK0::CHHM;
    pub use super::OTG_HCINTMSK0::DESCLSTROLLMSK;
    pub use super::OTG_HCINTMSK0::DTERRM;
    pub use super::OTG_HCINTMSK0::FRMORM;
    pub use super::OTG_HCINTMSK0::NAKM;
    pub use super::OTG_HCINTMSK0::NYET;
    pub use super::OTG_HCINTMSK0::STALLM;
    pub use super::OTG_HCINTMSK0::TXERRM;
    pub use super::OTG_HCINTMSK0::XFRCM;
}

/// OTG host channel 10 transfer size register
pub mod OTG_HCTSIZ10 {
    pub use super::OTG_HCTSIZ0::DPID;
    pub use super::OTG_HCTSIZ0::PKTCNT;
    pub use super::OTG_HCTSIZ0::XFRSIZ;
}

/// OTG host channel 10 DMA address register in buffer DMA \[alternate\]
pub mod OTG_HCDMA10 {
    pub use super::OTG_HCDMA0::DMAADDR;
}

/// OTG host channel-n DMA address buffer register
pub mod OTG_HCDMAB10 {
    pub use super::OTG_HCDMAB0::HCDMAB;
}

/// OTG host channel 11 characteristics register
pub mod OTG_HCCHAR11 {
    pub use super::OTG_HCCHAR0::CHDIS;
    pub use super::OTG_HCCHAR0::CHENA;
    pub use super::OTG_HCCHAR0::DAD;
    pub use super::OTG_HCCHAR0::EPDIR;
    pub use super::OTG_HCCHAR0::EPNUM;
    pub use super::OTG_HCCHAR0::EPTYP;
    pub use super::OTG_HCCHAR0::LSDEV;
    pub use super::OTG_HCCHAR0::MCNT;
    pub use super::OTG_HCCHAR0::MPSIZ;
}

/// OTG host channel 11 split control register
pub mod OTG_HCSPLT11 {
    pub use super::OTG_HCSPLT0::COMPLSPLT;
    pub use super::OTG_HCSPLT0::HUBADDR;
    pub use super::OTG_HCSPLT0::PRTADDR;
    pub use super::OTG_HCSPLT0::SPLITEN;
    pub use super::OTG_HCSPLT0::XACTPOS;
}

/// This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.
pub mod OTG_HCINT11 {
    pub use super::OTG_HCINT0::ACK;
    pub use super::OTG_HCINT0::AHBERR;
    pub use super::OTG_HCINT0::BBERR;
    pub use super::OTG_HCINT0::BNA;
    pub use super::OTG_HCINT0::CHH;
    pub use super::OTG_HCINT0::DESCLSTROLL;
    pub use super::OTG_HCINT0::DTERR;
    pub use super::OTG_HCINT0::FRMOR;
    pub use super::OTG_HCINT0::NAK;
    pub use super::OTG_HCINT0::NYET;
    pub use super::OTG_HCINT0::STALL;
    pub use super::OTG_HCINT0::TXERR;
    pub use super::OTG_HCINT0::XCSXACTERR;
    pub use super::OTG_HCINT0::XFRC;
}

/// This register reflects the mask for each channel status described in the previous section.
pub mod OTG_HCINTMSK11 {
    pub use super::OTG_HCINTMSK0::ACKM;
    pub use super::OTG_HCINTMSK0::AHBERRM;
    pub use super::OTG_HCINTMSK0::BBERRM;
    pub use super::OTG_HCINTMSK0::BNAMSK;
    pub use super::OTG_HCINTMSK0::CHHM;
    pub use super::OTG_HCINTMSK0::DESCLSTROLLMSK;
    pub use super::OTG_HCINTMSK0::DTERRM;
    pub use super::OTG_HCINTMSK0::FRMORM;
    pub use super::OTG_HCINTMSK0::NAKM;
    pub use super::OTG_HCINTMSK0::NYET;
    pub use super::OTG_HCINTMSK0::STALLM;
    pub use super::OTG_HCINTMSK0::TXERRM;
    pub use super::OTG_HCINTMSK0::XFRCM;
}

/// OTG host channel 11 transfer size register
pub mod OTG_HCTSIZ11 {
    pub use super::OTG_HCTSIZ0::DPID;
    pub use super::OTG_HCTSIZ0::PKTCNT;
    pub use super::OTG_HCTSIZ0::XFRSIZ;
}

/// OTG host channel 11 DMA address register in buffer DMA \[alternate\]
pub mod OTG_HCDMA11 {
    pub use super::OTG_HCDMA0::DMAADDR;
}

/// OTG host channel-n DMA address buffer register
pub mod OTG_HCDMAB11 {
    pub use super::OTG_HCDMAB0::HCDMAB;
}

/// OTG host channel 12 characteristics register
pub mod OTG_HCCHAR12 {
    pub use super::OTG_HCCHAR0::CHDIS;
    pub use super::OTG_HCCHAR0::CHENA;
    pub use super::OTG_HCCHAR0::DAD;
    pub use super::OTG_HCCHAR0::EPDIR;
    pub use super::OTG_HCCHAR0::EPNUM;
    pub use super::OTG_HCCHAR0::EPTYP;
    pub use super::OTG_HCCHAR0::LSDEV;
    pub use super::OTG_HCCHAR0::MCNT;
    pub use super::OTG_HCCHAR0::MPSIZ;
}

/// OTG host channel 12 split control register
pub mod OTG_HCSPLT12 {
    pub use super::OTG_HCSPLT0::COMPLSPLT;
    pub use super::OTG_HCSPLT0::HUBADDR;
    pub use super::OTG_HCSPLT0::PRTADDR;
    pub use super::OTG_HCSPLT0::SPLITEN;
    pub use super::OTG_HCSPLT0::XACTPOS;
}

/// This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.
pub mod OTG_HCINT12 {
    pub use super::OTG_HCINT0::ACK;
    pub use super::OTG_HCINT0::AHBERR;
    pub use super::OTG_HCINT0::BBERR;
    pub use super::OTG_HCINT0::BNA;
    pub use super::OTG_HCINT0::CHH;
    pub use super::OTG_HCINT0::DESCLSTROLL;
    pub use super::OTG_HCINT0::DTERR;
    pub use super::OTG_HCINT0::FRMOR;
    pub use super::OTG_HCINT0::NAK;
    pub use super::OTG_HCINT0::NYET;
    pub use super::OTG_HCINT0::STALL;
    pub use super::OTG_HCINT0::TXERR;
    pub use super::OTG_HCINT0::XCSXACTERR;
    pub use super::OTG_HCINT0::XFRC;
}

/// This register reflects the mask for each channel status described in the previous section.
pub mod OTG_HCINTMSK12 {
    pub use super::OTG_HCINTMSK0::ACKM;
    pub use super::OTG_HCINTMSK0::AHBERRM;
    pub use super::OTG_HCINTMSK0::BBERRM;
    pub use super::OTG_HCINTMSK0::BNAMSK;
    pub use super::OTG_HCINTMSK0::CHHM;
    pub use super::OTG_HCINTMSK0::DESCLSTROLLMSK;
    pub use super::OTG_HCINTMSK0::DTERRM;
    pub use super::OTG_HCINTMSK0::FRMORM;
    pub use super::OTG_HCINTMSK0::NAKM;
    pub use super::OTG_HCINTMSK0::NYET;
    pub use super::OTG_HCINTMSK0::STALLM;
    pub use super::OTG_HCINTMSK0::TXERRM;
    pub use super::OTG_HCINTMSK0::XFRCM;
}

/// OTG host channel 12 transfer size register
pub mod OTG_HCTSIZ12 {
    pub use super::OTG_HCTSIZ0::DPID;
    pub use super::OTG_HCTSIZ0::PKTCNT;
    pub use super::OTG_HCTSIZ0::XFRSIZ;
}

/// OTG host channel 12 DMA address register in buffer DMA \[alternate\]
pub mod OTG_HCDMA12 {
    pub use super::OTG_HCDMA0::DMAADDR;
}

/// OTG host channel-n DMA address buffer register
pub mod OTG_HCDMAB12 {
    pub use super::OTG_HCDMAB0::HCDMAB;
}

/// OTG host channel 13 characteristics register
pub mod OTG_HCCHAR13 {
    pub use super::OTG_HCCHAR0::CHDIS;
    pub use super::OTG_HCCHAR0::CHENA;
    pub use super::OTG_HCCHAR0::DAD;
    pub use super::OTG_HCCHAR0::EPDIR;
    pub use super::OTG_HCCHAR0::EPNUM;
    pub use super::OTG_HCCHAR0::EPTYP;
    pub use super::OTG_HCCHAR0::LSDEV;
    pub use super::OTG_HCCHAR0::MCNT;
    pub use super::OTG_HCCHAR0::MPSIZ;
}

/// OTG host channel 13 split control register
pub mod OTG_HCSPLT13 {
    pub use super::OTG_HCSPLT0::COMPLSPLT;
    pub use super::OTG_HCSPLT0::HUBADDR;
    pub use super::OTG_HCSPLT0::PRTADDR;
    pub use super::OTG_HCSPLT0::SPLITEN;
    pub use super::OTG_HCSPLT0::XACTPOS;
}

/// This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.
pub mod OTG_HCINT13 {
    pub use super::OTG_HCINT0::ACK;
    pub use super::OTG_HCINT0::AHBERR;
    pub use super::OTG_HCINT0::BBERR;
    pub use super::OTG_HCINT0::BNA;
    pub use super::OTG_HCINT0::CHH;
    pub use super::OTG_HCINT0::DESCLSTROLL;
    pub use super::OTG_HCINT0::DTERR;
    pub use super::OTG_HCINT0::FRMOR;
    pub use super::OTG_HCINT0::NAK;
    pub use super::OTG_HCINT0::NYET;
    pub use super::OTG_HCINT0::STALL;
    pub use super::OTG_HCINT0::TXERR;
    pub use super::OTG_HCINT0::XCSXACTERR;
    pub use super::OTG_HCINT0::XFRC;
}

/// This register reflects the mask for each channel status described in the previous section.
pub mod OTG_HCINTMSK13 {
    pub use super::OTG_HCINTMSK0::ACKM;
    pub use super::OTG_HCINTMSK0::AHBERRM;
    pub use super::OTG_HCINTMSK0::BBERRM;
    pub use super::OTG_HCINTMSK0::BNAMSK;
    pub use super::OTG_HCINTMSK0::CHHM;
    pub use super::OTG_HCINTMSK0::DESCLSTROLLMSK;
    pub use super::OTG_HCINTMSK0::DTERRM;
    pub use super::OTG_HCINTMSK0::FRMORM;
    pub use super::OTG_HCINTMSK0::NAKM;
    pub use super::OTG_HCINTMSK0::NYET;
    pub use super::OTG_HCINTMSK0::STALLM;
    pub use super::OTG_HCINTMSK0::TXERRM;
    pub use super::OTG_HCINTMSK0::XFRCM;
}

/// OTG host channel 13 transfer size register
pub mod OTG_HCTSIZ13 {
    pub use super::OTG_HCTSIZ0::DPID;
    pub use super::OTG_HCTSIZ0::PKTCNT;
    pub use super::OTG_HCTSIZ0::XFRSIZ;
}

/// OTG host channel 13 DMA address register in buffer DMA \[alternate\]
pub mod OTG_HCDMA13 {
    pub use super::OTG_HCDMA0::DMAADDR;
}

/// OTG host channel-n DMA address buffer register
pub mod OTG_HCDMAB13 {
    pub use super::OTG_HCDMAB0::HCDMAB;
}

/// OTG host channel 14 characteristics register
pub mod OTG_HCCHAR14 {
    pub use super::OTG_HCCHAR0::CHDIS;
    pub use super::OTG_HCCHAR0::CHENA;
    pub use super::OTG_HCCHAR0::DAD;
    pub use super::OTG_HCCHAR0::EPDIR;
    pub use super::OTG_HCCHAR0::EPNUM;
    pub use super::OTG_HCCHAR0::EPTYP;
    pub use super::OTG_HCCHAR0::LSDEV;
    pub use super::OTG_HCCHAR0::MCNT;
    pub use super::OTG_HCCHAR0::MPSIZ;
}

/// OTG host channel 14 split control register
pub mod OTG_HCSPLT14 {
    pub use super::OTG_HCSPLT0::COMPLSPLT;
    pub use super::OTG_HCSPLT0::HUBADDR;
    pub use super::OTG_HCSPLT0::PRTADDR;
    pub use super::OTG_HCSPLT0::SPLITEN;
    pub use super::OTG_HCSPLT0::XACTPOS;
}

/// This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.
pub mod OTG_HCINT14 {
    pub use super::OTG_HCINT0::ACK;
    pub use super::OTG_HCINT0::AHBERR;
    pub use super::OTG_HCINT0::BBERR;
    pub use super::OTG_HCINT0::BNA;
    pub use super::OTG_HCINT0::CHH;
    pub use super::OTG_HCINT0::DESCLSTROLL;
    pub use super::OTG_HCINT0::DTERR;
    pub use super::OTG_HCINT0::FRMOR;
    pub use super::OTG_HCINT0::NAK;
    pub use super::OTG_HCINT0::NYET;
    pub use super::OTG_HCINT0::STALL;
    pub use super::OTG_HCINT0::TXERR;
    pub use super::OTG_HCINT0::XCSXACTERR;
    pub use super::OTG_HCINT0::XFRC;
}

/// This register reflects the mask for each channel status described in the previous section.
pub mod OTG_HCINTMSK14 {
    pub use super::OTG_HCINTMSK0::ACKM;
    pub use super::OTG_HCINTMSK0::AHBERRM;
    pub use super::OTG_HCINTMSK0::BBERRM;
    pub use super::OTG_HCINTMSK0::BNAMSK;
    pub use super::OTG_HCINTMSK0::CHHM;
    pub use super::OTG_HCINTMSK0::DESCLSTROLLMSK;
    pub use super::OTG_HCINTMSK0::DTERRM;
    pub use super::OTG_HCINTMSK0::FRMORM;
    pub use super::OTG_HCINTMSK0::NAKM;
    pub use super::OTG_HCINTMSK0::NYET;
    pub use super::OTG_HCINTMSK0::STALLM;
    pub use super::OTG_HCINTMSK0::TXERRM;
    pub use super::OTG_HCINTMSK0::XFRCM;
}

/// OTG host channel 14 transfer size register
pub mod OTG_HCTSIZ14 {
    pub use super::OTG_HCTSIZ0::DPID;
    pub use super::OTG_HCTSIZ0::PKTCNT;
    pub use super::OTG_HCTSIZ0::XFRSIZ;
}

/// OTG host channel 14 DMA address register in buffer DMA \[alternate\]
pub mod OTG_HCDMA14 {
    pub use super::OTG_HCDMA0::DMAADDR;
}

/// OTG host channel-n DMA address buffer register
pub mod OTG_HCDMAB14 {
    pub use super::OTG_HCDMAB0::HCDMAB;
}

/// OTG host channel 15 characteristics register
pub mod OTG_HCCHAR15 {
    pub use super::OTG_HCCHAR0::CHDIS;
    pub use super::OTG_HCCHAR0::CHENA;
    pub use super::OTG_HCCHAR0::DAD;
    pub use super::OTG_HCCHAR0::EPDIR;
    pub use super::OTG_HCCHAR0::EPNUM;
    pub use super::OTG_HCCHAR0::EPTYP;
    pub use super::OTG_HCCHAR0::LSDEV;
    pub use super::OTG_HCCHAR0::MCNT;
    pub use super::OTG_HCCHAR0::MPSIZ;
}

/// OTG host channel 15 split control register
pub mod OTG_HCSPLT15 {
    pub use super::OTG_HCSPLT0::COMPLSPLT;
    pub use super::OTG_HCSPLT0::HUBADDR;
    pub use super::OTG_HCSPLT0::PRTADDR;
    pub use super::OTG_HCSPLT0::SPLITEN;
    pub use super::OTG_HCSPLT0::XACTPOS;
}

/// This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.
pub mod OTG_HCINT15 {
    pub use super::OTG_HCINT0::ACK;
    pub use super::OTG_HCINT0::AHBERR;
    pub use super::OTG_HCINT0::BBERR;
    pub use super::OTG_HCINT0::BNA;
    pub use super::OTG_HCINT0::CHH;
    pub use super::OTG_HCINT0::DESCLSTROLL;
    pub use super::OTG_HCINT0::DTERR;
    pub use super::OTG_HCINT0::FRMOR;
    pub use super::OTG_HCINT0::NAK;
    pub use super::OTG_HCINT0::NYET;
    pub use super::OTG_HCINT0::STALL;
    pub use super::OTG_HCINT0::TXERR;
    pub use super::OTG_HCINT0::XCSXACTERR;
    pub use super::OTG_HCINT0::XFRC;
}

/// This register reflects the mask for each channel status described in the previous section.
pub mod OTG_HCINTMSK15 {
    pub use super::OTG_HCINTMSK0::ACKM;
    pub use super::OTG_HCINTMSK0::AHBERRM;
    pub use super::OTG_HCINTMSK0::BBERRM;
    pub use super::OTG_HCINTMSK0::BNAMSK;
    pub use super::OTG_HCINTMSK0::CHHM;
    pub use super::OTG_HCINTMSK0::DESCLSTROLLMSK;
    pub use super::OTG_HCINTMSK0::DTERRM;
    pub use super::OTG_HCINTMSK0::FRMORM;
    pub use super::OTG_HCINTMSK0::NAKM;
    pub use super::OTG_HCINTMSK0::NYET;
    pub use super::OTG_HCINTMSK0::STALLM;
    pub use super::OTG_HCINTMSK0::TXERRM;
    pub use super::OTG_HCINTMSK0::XFRCM;
}

/// OTG host channel 15 transfer size register
pub mod OTG_HCTSIZ15 {
    pub use super::OTG_HCTSIZ0::DPID;
    pub use super::OTG_HCTSIZ0::PKTCNT;
    pub use super::OTG_HCTSIZ0::XFRSIZ;
}

/// OTG host channel 15 DMA address register in buffer DMA \[alternate\]
pub mod OTG_HCDMA15 {
    pub use super::OTG_HCDMA0::DMAADDR;
}

/// OTG host channel-n DMA address buffer register
pub mod OTG_HCDMAB15 {
    pub use super::OTG_HCDMAB0::HCDMAB;
}

/// This register configures the core in device mode after power-on or after certain control commands or enumeration. Do not make changes to this register after initial programming.
pub mod OTG_DCFG {

    /// DSPD
    pub mod DSPD {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// NZLSOHSK
    pub mod NZLSOHSK {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DAD
    pub mod DAD {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (7 bits: 0x7f << 4)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PFIVL
    pub mod PFIVL {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (2 bits: 0b11 << 11)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// XCVRDLY
    pub mod XCVRDLY {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ERRATIM
    pub mod ERRATIM {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PERSCHIVL
    pub mod PERSCHIVL {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (2 bits: 0b11 << 24)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// OTG device control register
pub mod OTG_DCTL {

    /// RWUSIG
    pub mod RWUSIG {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SDIS
    pub mod SDIS {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// GINSTS
    pub mod GINSTS {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// GONSTS
    pub mod GONSTS {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TCTL
    pub mod TCTL {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (3 bits: 0b111 << 4)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SGINAK
    pub mod SGINAK {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CGINAK
    pub mod CGINAK {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SGONAK
    pub mod SGONAK {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CGONAK
    pub mod CGONAK {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// POPRGDNE
    pub mod POPRGDNE {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DSBESLRJCT
    pub mod DSBESLRJCT {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register indicates the status of the core with respect to USB-related events. It must be read on interrupts from the device all interrupts (OTG_DAINT) register.
pub mod OTG_DSTS {

    /// SUSPSTS
    pub mod SUSPSTS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ENUMSPD
    pub mod ENUMSPD {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (2 bits: 0b11 << 1)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// EERR
    pub mod EERR {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// FNSOF
    pub mod FNSOF {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (14 bits: 0x3fff << 8)
        pub const mask: u32 = 0x3fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DEVLNSTS
    pub mod DEVLNSTS {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (2 bits: 0b11 << 22)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register works with each of the OTG_DIEPINTx registers for all endpoints to generate an interrupt per IN endpoint. The IN endpoint interrupt for a specific status in the OTG_DIEPINTx register can be masked by writing to the corresponding bit in this register. Status bits are masked by default.
pub mod OTG_DIEPMSK {

    /// XFRCM
    pub mod XFRCM {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// EPDM
    pub mod EPDM {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// AHBERRM
    pub mod AHBERRM {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TOM
    pub mod TOM {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ITTXFEMSK
    pub mod ITTXFEMSK {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// INEPNMM
    pub mod INEPNMM {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// INEPNEM
    pub mod INEPNEM {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TXFURM
    pub mod TXFURM {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// BNAM
    pub mod BNAM {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// NAKM
    pub mod NAKM {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register works with each of the OTG_DOEPINTx registers for all endpoints to generate an interrupt per OUT endpoint. The OUT endpoint interrupt for a specific status in the OTG_DOEPINTx register can be masked by writing into the corresponding bit in this register. Status bits are masked by default.
pub mod OTG_DOEPMSK {

    /// XFRCM
    pub mod XFRCM {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// EPDM
    pub mod EPDM {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// AHBERRM
    pub mod AHBERRM {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// STUPM
    pub mod STUPM {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// OTEPDM
    pub mod OTEPDM {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// STSPHSRXM
    pub mod STSPHSRXM {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// B2BSTUPM
    pub mod B2BSTUPM {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// OUTPKTERRM
    pub mod OUTPKTERRM {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// BNAM
    pub mod BNAM {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// BERRM
    pub mod BERRM {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// NAKMSK
    pub mod NAKMSK {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// NYETMSK
    pub mod NYETMSK {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// When a significant event occurs on an endpoint, a OTG_DAINT register interrupts the application using the device OUT endpoints interrupt bit or device IN endpoints interrupt bit of the OTG_GINTSTS register (OEPINT or IEPINT in OTG_GINTSTS, respectively). There is one interrupt bit per endpoint, up to a maximum of 16 bits for OUT endpoints and 16 bits for IN endpoints. For a bidirectional endpoint, the corresponding IN and OUT interrupt bits are used. Bits in this register are set and cleared when the application sets and clears bits in the corresponding device endpoint-x interrupt register (OTG_DIEPINTx/OTG_DOEPINTx).
pub mod OTG_DAINT {

    /// IEPINT
    pub mod IEPINT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// OEPINT
    pub mod OEPINT {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (16 bits: 0xffff << 16)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// The OTG_DAINTMSK register works with the device endpoint interrupt register to interrupt the application when an event occurs on a device endpoint. However, the OTG_DAINT register bit corresponding to that interrupt is still set.
pub mod OTG_DAINTMSK {

    /// IEPM
    pub mod IEPM {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// OEPM
    pub mod OEPM {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (16 bits: 0xffff << 16)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register specifies the VBUS discharge time after VBUS pulsing during SRP.
pub mod OTG_DVBUSDIS {

    /// VBUSDT
    pub mod VBUSDT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register specifies the VBUS pulsing time during SRP.
pub mod OTG_DVBUSPULSE {

    /// DVBUSP
    pub mod DVBUSP {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// OTG device threshold control register
pub mod OTG_DTHRCTL {

    /// NONISOTHREN
    pub mod NONISOTHREN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ISOTHREN
    pub mod ISOTHREN {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TXTHRLEN
    pub mod TXTHRLEN {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (9 bits: 0x1ff << 2)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RXTHREN
    pub mod RXTHREN {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RXTHRLEN
    pub mod RXTHRLEN {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (9 bits: 0x1ff << 17)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ARPEN
    pub mod ARPEN {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register is used to control the IN endpoint FIFO empty interrupt generation (TXFE_OTG_DIEPINTx).
pub mod OTG_DIEPEMPMSK {

    /// INEPTXFEM
    pub mod INEPTXFEM {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// OTG device each endpoint interrupt register
pub mod OTG_DEACHINT {

    /// IEP1INT
    pub mod IEP1INT {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// OEP1INT
    pub mod OEP1INT {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// There is one interrupt bit for endpoint 1 IN and one interrupt bit for endpoint 1 OUT.
pub mod OTG_DEACHINTMSK {

    /// IEP1INTM
    pub mod IEP1INTM {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// OEP1INTM
    pub mod OEP1INTM {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register works with the OTG_DIEPINT1 register to generate a dedicated interrupt OTG_HS_EP1_IN for endpoint #1. The IN endpoint interrupt for a specific status in the OTG_DOEPINT1 register can be masked by writing into the corresponding bit in this register. Status bits are masked by default.
pub mod OTG_HS_DIEPEACHMSK1 {

    /// XFRCM
    pub mod XFRCM {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// EPDM
    pub mod EPDM {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// AHBERRM
    pub mod AHBERRM {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TOM
    pub mod TOM {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ITTXFEMSK
    pub mod ITTXFEMSK {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// INEPNEM
    pub mod INEPNEM {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TXFURM
    pub mod TXFURM {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// BNAM
    pub mod BNAM {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// NAKM
    pub mod NAKM {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register works with the OTG_DOEPINT1 register to generate a dedicated interrupt OTG_HS_EP1_OUT for endpoint #1. The OUT endpoint interrupt for a specific status in the OTG_DOEPINT1 register can be masked by writing into the corresponding bit in this register. Status bits are masked by default.
pub mod OTG_HS_DOEPEACHMSK1 {

    /// XFRCM
    pub mod XFRCM {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// EPDM
    pub mod EPDM {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// AHBERRM
    pub mod AHBERRM {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// STUPM
    pub mod STUPM {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// OTEPDM
    pub mod OTEPDM {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// B2BSTUPM
    pub mod B2BSTUPM {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// OUTPKTERRM
    pub mod OUTPKTERRM {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// BNAM
    pub mod BNAM {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// BERRM
    pub mod BERRM {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// NAKMSK
    pub mod NAKMSK {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// NYETMSK
    pub mod NYETMSK {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
pub mod OTG_DIEPCTL0 {

    /// MPSIZ
    pub mod MPSIZ {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (11 bits: 0x7ff << 0)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// USBAEP
    pub mod USBAEP {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// EONUM_DPIP
    pub mod EONUM_DPIP {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// NAKSTS
    pub mod NAKSTS {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// EPTYP
    pub mod EPTYP {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (2 bits: 0b11 << 18)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// STALL
    pub mod STALL {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TXFNUM
    pub mod TXFNUM {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (4 bits: 0b1111 << 22)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CNAK
    pub mod CNAK {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SNAK
    pub mod SNAK {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SD0PID_SEVNFRM
    pub mod SD0PID_SEVNFRM {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SODDFRM
    pub mod SODDFRM {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// EPDIS
    pub mod EPDIS {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// EPENA
    pub mod EPENA {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.
pub mod OTG_DIEPINT0 {

    /// XFRC
    pub mod XFRC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// EPDISD
    pub mod EPDISD {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// AHBERR
    pub mod AHBERR {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TOC
    pub mod TOC {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ITTXFE
    pub mod ITTXFE {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// INEPNM
    pub mod INEPNM {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// INEPNE
    pub mod INEPNE {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TXFE
    pub mod TXFE {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TXFIFOUDRN
    pub mod TXFIFOUDRN {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// BNA
    pub mod BNA {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PKTDRPSTS
    pub mod PKTDRPSTS {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// NAK
    pub mod NAK {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// The application must modify this register before enabling endpoint 0.
pub mod OTG_DIEPTSIZ0 {

    /// XFRSIZ
    pub mod XFRSIZ {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PKTCNT
    pub mod PKTCNT {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (2 bits: 0b11 << 19)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// OTG device IN endpoint 0 DMA address register
pub mod OTG_DIEPDMA0 {
    pub use super::OTG_HCDMA0::DMAADDR;
}

/// This read-only register contains the free space information for the device IN endpoint Tx FIFO.
pub mod OTG_DTXFSTS0 {

    /// INEPTFSAV
    pub mod INEPTFSAV {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
pub mod OTG_DIEPCTL1 {
    pub use super::OTG_DIEPCTL0::CNAK;
    pub use super::OTG_DIEPCTL0::EONUM_DPIP;
    pub use super::OTG_DIEPCTL0::EPDIS;
    pub use super::OTG_DIEPCTL0::EPENA;
    pub use super::OTG_DIEPCTL0::EPTYP;
    pub use super::OTG_DIEPCTL0::MPSIZ;
    pub use super::OTG_DIEPCTL0::NAKSTS;
    pub use super::OTG_DIEPCTL0::SD0PID_SEVNFRM;
    pub use super::OTG_DIEPCTL0::SNAK;
    pub use super::OTG_DIEPCTL0::SODDFRM;
    pub use super::OTG_DIEPCTL0::STALL;
    pub use super::OTG_DIEPCTL0::TXFNUM;
    pub use super::OTG_DIEPCTL0::USBAEP;
}

/// This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.
pub mod OTG_DIEPINT1 {
    pub use super::OTG_DIEPINT0::AHBERR;
    pub use super::OTG_DIEPINT0::BNA;
    pub use super::OTG_DIEPINT0::EPDISD;
    pub use super::OTG_DIEPINT0::INEPNE;
    pub use super::OTG_DIEPINT0::INEPNM;
    pub use super::OTG_DIEPINT0::ITTXFE;
    pub use super::OTG_DIEPINT0::NAK;
    pub use super::OTG_DIEPINT0::PKTDRPSTS;
    pub use super::OTG_DIEPINT0::TOC;
    pub use super::OTG_DIEPINT0::TXFE;
    pub use super::OTG_DIEPINT0::TXFIFOUDRN;
    pub use super::OTG_DIEPINT0::XFRC;
}

/// The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
pub mod OTG_DIEPTSIZ1 {

    /// XFRSIZ
    pub mod XFRSIZ {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (19 bits: 0x7ffff << 0)
        pub const mask: u32 = 0x7ffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PKTCNT
    pub mod PKTCNT {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (10 bits: 0x3ff << 19)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// MCNT
    pub mod MCNT {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (2 bits: 0b11 << 29)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// OTG device IN endpoint 1 DMA address register
pub mod OTG_DIEPDMA1 {
    pub use super::OTG_HCDMA0::DMAADDR;
}

/// This read-only register contains the free space information for the device IN endpoint Tx FIFO.
pub mod OTG_DTXFSTS1 {
    pub use super::OTG_DTXFSTS0::INEPTFSAV;
}

/// The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
pub mod OTG_DIEPCTL2 {
    pub use super::OTG_DIEPCTL0::CNAK;
    pub use super::OTG_DIEPCTL0::EONUM_DPIP;
    pub use super::OTG_DIEPCTL0::EPDIS;
    pub use super::OTG_DIEPCTL0::EPENA;
    pub use super::OTG_DIEPCTL0::EPTYP;
    pub use super::OTG_DIEPCTL0::MPSIZ;
    pub use super::OTG_DIEPCTL0::NAKSTS;
    pub use super::OTG_DIEPCTL0::SD0PID_SEVNFRM;
    pub use super::OTG_DIEPCTL0::SNAK;
    pub use super::OTG_DIEPCTL0::SODDFRM;
    pub use super::OTG_DIEPCTL0::STALL;
    pub use super::OTG_DIEPCTL0::TXFNUM;
    pub use super::OTG_DIEPCTL0::USBAEP;
}

/// This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.
pub mod OTG_DIEPINT2 {
    pub use super::OTG_DIEPINT0::AHBERR;
    pub use super::OTG_DIEPINT0::BNA;
    pub use super::OTG_DIEPINT0::EPDISD;
    pub use super::OTG_DIEPINT0::INEPNE;
    pub use super::OTG_DIEPINT0::INEPNM;
    pub use super::OTG_DIEPINT0::ITTXFE;
    pub use super::OTG_DIEPINT0::NAK;
    pub use super::OTG_DIEPINT0::PKTDRPSTS;
    pub use super::OTG_DIEPINT0::TOC;
    pub use super::OTG_DIEPINT0::TXFE;
    pub use super::OTG_DIEPINT0::TXFIFOUDRN;
    pub use super::OTG_DIEPINT0::XFRC;
}

/// The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
pub mod OTG_DIEPTSIZ2 {
    pub use super::OTG_DIEPTSIZ1::MCNT;
    pub use super::OTG_DIEPTSIZ1::PKTCNT;
    pub use super::OTG_DIEPTSIZ1::XFRSIZ;
}

/// OTG device IN endpoint 2 DMA address register
pub mod OTG_DIEPDMA2 {
    pub use super::OTG_HCDMA0::DMAADDR;
}

/// This read-only register contains the free space information for the device IN endpoint Tx FIFO.
pub mod OTG_DTXFSTS2 {
    pub use super::OTG_DTXFSTS0::INEPTFSAV;
}

/// The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
pub mod OTG_DIEPCTL3 {
    pub use super::OTG_DIEPCTL0::CNAK;
    pub use super::OTG_DIEPCTL0::EONUM_DPIP;
    pub use super::OTG_DIEPCTL0::EPDIS;
    pub use super::OTG_DIEPCTL0::EPENA;
    pub use super::OTG_DIEPCTL0::EPTYP;
    pub use super::OTG_DIEPCTL0::MPSIZ;
    pub use super::OTG_DIEPCTL0::NAKSTS;
    pub use super::OTG_DIEPCTL0::SD0PID_SEVNFRM;
    pub use super::OTG_DIEPCTL0::SNAK;
    pub use super::OTG_DIEPCTL0::SODDFRM;
    pub use super::OTG_DIEPCTL0::STALL;
    pub use super::OTG_DIEPCTL0::TXFNUM;
    pub use super::OTG_DIEPCTL0::USBAEP;
}

/// This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.
pub mod OTG_DIEPINT3 {
    pub use super::OTG_DIEPINT0::AHBERR;
    pub use super::OTG_DIEPINT0::BNA;
    pub use super::OTG_DIEPINT0::EPDISD;
    pub use super::OTG_DIEPINT0::INEPNE;
    pub use super::OTG_DIEPINT0::INEPNM;
    pub use super::OTG_DIEPINT0::ITTXFE;
    pub use super::OTG_DIEPINT0::NAK;
    pub use super::OTG_DIEPINT0::PKTDRPSTS;
    pub use super::OTG_DIEPINT0::TOC;
    pub use super::OTG_DIEPINT0::TXFE;
    pub use super::OTG_DIEPINT0::TXFIFOUDRN;
    pub use super::OTG_DIEPINT0::XFRC;
}

/// The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
pub mod OTG_DIEPTSIZ3 {
    pub use super::OTG_DIEPTSIZ1::MCNT;
    pub use super::OTG_DIEPTSIZ1::PKTCNT;
    pub use super::OTG_DIEPTSIZ1::XFRSIZ;
}

/// OTG device IN endpoint 3 DMA address register
pub mod OTG_DIEPDMA3 {
    pub use super::OTG_HCDMA0::DMAADDR;
}

/// This read-only register contains the free space information for the device IN endpoint Tx FIFO.
pub mod OTG_DTXFSTS3 {
    pub use super::OTG_DTXFSTS0::INEPTFSAV;
}

/// The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
pub mod OTG_DIEPCTL4 {
    pub use super::OTG_DIEPCTL0::CNAK;
    pub use super::OTG_DIEPCTL0::EONUM_DPIP;
    pub use super::OTG_DIEPCTL0::EPDIS;
    pub use super::OTG_DIEPCTL0::EPENA;
    pub use super::OTG_DIEPCTL0::EPTYP;
    pub use super::OTG_DIEPCTL0::MPSIZ;
    pub use super::OTG_DIEPCTL0::NAKSTS;
    pub use super::OTG_DIEPCTL0::SD0PID_SEVNFRM;
    pub use super::OTG_DIEPCTL0::SNAK;
    pub use super::OTG_DIEPCTL0::SODDFRM;
    pub use super::OTG_DIEPCTL0::STALL;
    pub use super::OTG_DIEPCTL0::TXFNUM;
    pub use super::OTG_DIEPCTL0::USBAEP;
}

/// This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.
pub mod OTG_DIEPINT4 {
    pub use super::OTG_DIEPINT0::AHBERR;
    pub use super::OTG_DIEPINT0::BNA;
    pub use super::OTG_DIEPINT0::EPDISD;
    pub use super::OTG_DIEPINT0::INEPNE;
    pub use super::OTG_DIEPINT0::INEPNM;
    pub use super::OTG_DIEPINT0::ITTXFE;
    pub use super::OTG_DIEPINT0::NAK;
    pub use super::OTG_DIEPINT0::PKTDRPSTS;
    pub use super::OTG_DIEPINT0::TOC;
    pub use super::OTG_DIEPINT0::TXFE;
    pub use super::OTG_DIEPINT0::TXFIFOUDRN;
    pub use super::OTG_DIEPINT0::XFRC;
}

/// The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
pub mod OTG_DIEPTSIZ4 {
    pub use super::OTG_DIEPTSIZ1::MCNT;
    pub use super::OTG_DIEPTSIZ1::PKTCNT;
    pub use super::OTG_DIEPTSIZ1::XFRSIZ;
}

/// OTG device IN endpoint 4 DMA address register
pub mod OTG_DIEPDMA4 {
    pub use super::OTG_HCDMA0::DMAADDR;
}

/// This read-only register contains the free space information for the device IN endpoint Tx FIFO.
pub mod OTG_DTXFSTS4 {
    pub use super::OTG_DTXFSTS0::INEPTFSAV;
}

/// The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
pub mod OTG_DIEPCTL5 {
    pub use super::OTG_DIEPCTL0::CNAK;
    pub use super::OTG_DIEPCTL0::EONUM_DPIP;
    pub use super::OTG_DIEPCTL0::EPDIS;
    pub use super::OTG_DIEPCTL0::EPENA;
    pub use super::OTG_DIEPCTL0::EPTYP;
    pub use super::OTG_DIEPCTL0::MPSIZ;
    pub use super::OTG_DIEPCTL0::NAKSTS;
    pub use super::OTG_DIEPCTL0::SD0PID_SEVNFRM;
    pub use super::OTG_DIEPCTL0::SNAK;
    pub use super::OTG_DIEPCTL0::SODDFRM;
    pub use super::OTG_DIEPCTL0::STALL;
    pub use super::OTG_DIEPCTL0::TXFNUM;
    pub use super::OTG_DIEPCTL0::USBAEP;
}

/// This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.
pub mod OTG_DIEPINT5 {
    pub use super::OTG_DIEPINT0::AHBERR;
    pub use super::OTG_DIEPINT0::BNA;
    pub use super::OTG_DIEPINT0::EPDISD;
    pub use super::OTG_DIEPINT0::INEPNE;
    pub use super::OTG_DIEPINT0::INEPNM;
    pub use super::OTG_DIEPINT0::ITTXFE;
    pub use super::OTG_DIEPINT0::NAK;
    pub use super::OTG_DIEPINT0::PKTDRPSTS;
    pub use super::OTG_DIEPINT0::TOC;
    pub use super::OTG_DIEPINT0::TXFE;
    pub use super::OTG_DIEPINT0::TXFIFOUDRN;
    pub use super::OTG_DIEPINT0::XFRC;
}

/// The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
pub mod OTG_DIEPTSIZ5 {
    pub use super::OTG_DIEPTSIZ1::MCNT;
    pub use super::OTG_DIEPTSIZ1::PKTCNT;
    pub use super::OTG_DIEPTSIZ1::XFRSIZ;
}

/// OTG device IN endpoint 5 DMA address register
pub mod OTG_DIEPDMA5 {
    pub use super::OTG_HCDMA0::DMAADDR;
}

/// This read-only register contains the free space information for the device IN endpoint Tx FIFO.
pub mod OTG_DTXFSTS5 {
    pub use super::OTG_DTXFSTS0::INEPTFSAV;
}

/// The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
pub mod OTG_DIEPCTL6 {
    pub use super::OTG_DIEPCTL0::CNAK;
    pub use super::OTG_DIEPCTL0::EONUM_DPIP;
    pub use super::OTG_DIEPCTL0::EPDIS;
    pub use super::OTG_DIEPCTL0::EPENA;
    pub use super::OTG_DIEPCTL0::EPTYP;
    pub use super::OTG_DIEPCTL0::MPSIZ;
    pub use super::OTG_DIEPCTL0::NAKSTS;
    pub use super::OTG_DIEPCTL0::SD0PID_SEVNFRM;
    pub use super::OTG_DIEPCTL0::SNAK;
    pub use super::OTG_DIEPCTL0::SODDFRM;
    pub use super::OTG_DIEPCTL0::STALL;
    pub use super::OTG_DIEPCTL0::TXFNUM;
    pub use super::OTG_DIEPCTL0::USBAEP;
}

/// This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.
pub mod OTG_DIEPINT6 {
    pub use super::OTG_DIEPINT0::AHBERR;
    pub use super::OTG_DIEPINT0::BNA;
    pub use super::OTG_DIEPINT0::EPDISD;
    pub use super::OTG_DIEPINT0::INEPNE;
    pub use super::OTG_DIEPINT0::INEPNM;
    pub use super::OTG_DIEPINT0::ITTXFE;
    pub use super::OTG_DIEPINT0::NAK;
    pub use super::OTG_DIEPINT0::PKTDRPSTS;
    pub use super::OTG_DIEPINT0::TOC;
    pub use super::OTG_DIEPINT0::TXFE;
    pub use super::OTG_DIEPINT0::TXFIFOUDRN;
    pub use super::OTG_DIEPINT0::XFRC;
}

/// The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
pub mod OTG_DIEPTSIZ6 {
    pub use super::OTG_DIEPTSIZ1::MCNT;
    pub use super::OTG_DIEPTSIZ1::PKTCNT;
    pub use super::OTG_DIEPTSIZ1::XFRSIZ;
}

/// OTG device IN endpoint 6 DMA address register
pub mod OTG_DIEPDMA6 {
    pub use super::OTG_HCDMA0::DMAADDR;
}

/// This read-only register contains the free space information for the device IN endpoint Tx FIFO.
pub mod OTG_DTXFSTS6 {
    pub use super::OTG_DTXFSTS0::INEPTFSAV;
}

/// The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
pub mod OTG_DIEPCTL7 {
    pub use super::OTG_DIEPCTL0::CNAK;
    pub use super::OTG_DIEPCTL0::EONUM_DPIP;
    pub use super::OTG_DIEPCTL0::EPDIS;
    pub use super::OTG_DIEPCTL0::EPENA;
    pub use super::OTG_DIEPCTL0::EPTYP;
    pub use super::OTG_DIEPCTL0::MPSIZ;
    pub use super::OTG_DIEPCTL0::NAKSTS;
    pub use super::OTG_DIEPCTL0::SD0PID_SEVNFRM;
    pub use super::OTG_DIEPCTL0::SNAK;
    pub use super::OTG_DIEPCTL0::SODDFRM;
    pub use super::OTG_DIEPCTL0::STALL;
    pub use super::OTG_DIEPCTL0::TXFNUM;
    pub use super::OTG_DIEPCTL0::USBAEP;
}

/// This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.
pub mod OTG_DIEPINT7 {
    pub use super::OTG_DIEPINT0::AHBERR;
    pub use super::OTG_DIEPINT0::BNA;
    pub use super::OTG_DIEPINT0::EPDISD;
    pub use super::OTG_DIEPINT0::INEPNE;
    pub use super::OTG_DIEPINT0::INEPNM;
    pub use super::OTG_DIEPINT0::ITTXFE;
    pub use super::OTG_DIEPINT0::NAK;
    pub use super::OTG_DIEPINT0::PKTDRPSTS;
    pub use super::OTG_DIEPINT0::TOC;
    pub use super::OTG_DIEPINT0::TXFE;
    pub use super::OTG_DIEPINT0::TXFIFOUDRN;
    pub use super::OTG_DIEPINT0::XFRC;
}

/// The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
pub mod OTG_DIEPTSIZ7 {
    pub use super::OTG_DIEPTSIZ1::MCNT;
    pub use super::OTG_DIEPTSIZ1::PKTCNT;
    pub use super::OTG_DIEPTSIZ1::XFRSIZ;
}

/// OTG device IN endpoint 7 DMA address register
pub mod OTG_DIEPDMA7 {
    pub use super::OTG_HCDMA0::DMAADDR;
}

/// This read-only register contains the free space information for the device IN endpoint Tx FIFO.
pub mod OTG_DTXFSTS7 {
    pub use super::OTG_DTXFSTS0::INEPTFSAV;
}

/// The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
pub mod OTG_DIEPCTL8 {
    pub use super::OTG_DIEPCTL0::CNAK;
    pub use super::OTG_DIEPCTL0::EONUM_DPIP;
    pub use super::OTG_DIEPCTL0::EPDIS;
    pub use super::OTG_DIEPCTL0::EPENA;
    pub use super::OTG_DIEPCTL0::EPTYP;
    pub use super::OTG_DIEPCTL0::MPSIZ;
    pub use super::OTG_DIEPCTL0::NAKSTS;
    pub use super::OTG_DIEPCTL0::SD0PID_SEVNFRM;
    pub use super::OTG_DIEPCTL0::SNAK;
    pub use super::OTG_DIEPCTL0::SODDFRM;
    pub use super::OTG_DIEPCTL0::STALL;
    pub use super::OTG_DIEPCTL0::TXFNUM;
    pub use super::OTG_DIEPCTL0::USBAEP;
}

/// This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.
pub mod OTG_DIEPINT8 {
    pub use super::OTG_DIEPINT0::AHBERR;
    pub use super::OTG_DIEPINT0::BNA;
    pub use super::OTG_DIEPINT0::EPDISD;
    pub use super::OTG_DIEPINT0::INEPNE;
    pub use super::OTG_DIEPINT0::INEPNM;
    pub use super::OTG_DIEPINT0::ITTXFE;
    pub use super::OTG_DIEPINT0::NAK;
    pub use super::OTG_DIEPINT0::PKTDRPSTS;
    pub use super::OTG_DIEPINT0::TOC;
    pub use super::OTG_DIEPINT0::TXFE;
    pub use super::OTG_DIEPINT0::TXFIFOUDRN;
    pub use super::OTG_DIEPINT0::XFRC;
}

/// The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
pub mod OTG_DIEPTSIZ8 {
    pub use super::OTG_DIEPTSIZ1::MCNT;
    pub use super::OTG_DIEPTSIZ1::PKTCNT;
    pub use super::OTG_DIEPTSIZ1::XFRSIZ;
}

/// OTG device IN endpoint 8 DMA address register
pub mod OTG_DIEPDMA8 {
    pub use super::OTG_HCDMA0::DMAADDR;
}

/// This read-only register contains the free space information for the device IN endpoint Tx FIFO.
pub mod OTG_DTXFSTS8 {
    pub use super::OTG_DTXFSTS0::INEPTFSAV;
}

/// This section describes the OTG_DOEPCTL0 register.
pub mod OTG_DOEPCTL0 {

    /// MPSIZ
    pub mod MPSIZ {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// USBAEP
    pub mod USBAEP {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// NAKSTS
    pub mod NAKSTS {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// EPTYP
    pub mod EPTYP {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (2 bits: 0b11 << 18)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SNPM
    pub mod SNPM {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// STALL
    pub mod STALL {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CNAK
    pub mod CNAK {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SNAK
    pub mod SNAK {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// EPDIS
    pub mod EPDIS {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// EPENA
    pub mod EPENA {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.
pub mod OTG_DOEPINT0 {

    /// XFRC
    pub mod XFRC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// EPDISD
    pub mod EPDISD {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// AHBERR
    pub mod AHBERR {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// STUP
    pub mod STUP {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// OTEPDIS
    pub mod OTEPDIS {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// STSPHSRX
    pub mod STSPHSRX {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// B2BSTUP
    pub mod B2BSTUP {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// OUTPKTERR
    pub mod OUTPKTERR {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// BNA
    pub mod BNA {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// BERR
    pub mod BERR {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// NAK
    pub mod NAK {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// NYET
    pub mod NYET {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// STPKTRX
    pub mod STPKTRX {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// The application must modify this register before enabling endpoint 0.
pub mod OTG_DOEPTSIZ0 {

    /// XFRSIZ
    pub mod XFRSIZ {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u32 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PKTCNT
    pub mod PKTCNT {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// STUPCNT
    pub mod STUPCNT {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (2 bits: 0b11 << 29)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// OTG device OUT endpoint 0 DMA address register
pub mod OTG_DOEPDMA0 {
    pub use super::OTG_HCDMA0::DMAADDR;
}

/// The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
pub mod OTG_DOEPCTL1 {

    /// MPSIZ
    pub mod MPSIZ {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (11 bits: 0x7ff << 0)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// USBAEP
    pub mod USBAEP {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// EONUM_DPIP
    pub mod EONUM_DPIP {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// NAKSTS
    pub mod NAKSTS {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// EPTYP
    pub mod EPTYP {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (2 bits: 0b11 << 18)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SNPM
    pub mod SNPM {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// STALL
    pub mod STALL {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CNAK
    pub mod CNAK {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SNAK
    pub mod SNAK {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SD0PID_SEVNFRM
    pub mod SD0PID_SEVNFRM {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SD1PID_SODDFRM
    pub mod SD1PID_SODDFRM {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// EPDIS
    pub mod EPDIS {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// EPENA
    pub mod EPENA {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.
pub mod OTG_DOEPINT1 {
    pub use super::OTG_DOEPINT0::AHBERR;
    pub use super::OTG_DOEPINT0::B2BSTUP;
    pub use super::OTG_DOEPINT0::BERR;
    pub use super::OTG_DOEPINT0::BNA;
    pub use super::OTG_DOEPINT0::EPDISD;
    pub use super::OTG_DOEPINT0::NAK;
    pub use super::OTG_DOEPINT0::NYET;
    pub use super::OTG_DOEPINT0::OTEPDIS;
    pub use super::OTG_DOEPINT0::OUTPKTERR;
    pub use super::OTG_DOEPINT0::STPKTRX;
    pub use super::OTG_DOEPINT0::STSPHSRX;
    pub use super::OTG_DOEPINT0::STUP;
    pub use super::OTG_DOEPINT0::XFRC;
}

/// The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
pub mod OTG_DOEPTSIZ1 {

    /// XFRSIZ
    pub mod XFRSIZ {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (19 bits: 0x7ffff << 0)
        pub const mask: u32 = 0x7ffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PKTCNT
    pub mod PKTCNT {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (10 bits: 0x3ff << 19)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RXDPID_STUPCNT
    pub mod RXDPID_STUPCNT {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (2 bits: 0b11 << 29)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// OTG device OUT endpoint 1 DMA address register
pub mod OTG_DOEPDMA1 {
    pub use super::OTG_HCDMA0::DMAADDR;
}

/// The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
pub mod OTG_DOEPCTL2 {
    pub use super::OTG_DOEPCTL1::CNAK;
    pub use super::OTG_DOEPCTL1::EONUM_DPIP;
    pub use super::OTG_DOEPCTL1::EPDIS;
    pub use super::OTG_DOEPCTL1::EPENA;
    pub use super::OTG_DOEPCTL1::EPTYP;
    pub use super::OTG_DOEPCTL1::MPSIZ;
    pub use super::OTG_DOEPCTL1::NAKSTS;
    pub use super::OTG_DOEPCTL1::SD0PID_SEVNFRM;
    pub use super::OTG_DOEPCTL1::SD1PID_SODDFRM;
    pub use super::OTG_DOEPCTL1::SNAK;
    pub use super::OTG_DOEPCTL1::SNPM;
    pub use super::OTG_DOEPCTL1::STALL;
    pub use super::OTG_DOEPCTL1::USBAEP;
}

/// This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.
pub mod OTG_DOEPINT2 {
    pub use super::OTG_DOEPINT0::AHBERR;
    pub use super::OTG_DOEPINT0::B2BSTUP;
    pub use super::OTG_DOEPINT0::BERR;
    pub use super::OTG_DOEPINT0::BNA;
    pub use super::OTG_DOEPINT0::EPDISD;
    pub use super::OTG_DOEPINT0::NAK;
    pub use super::OTG_DOEPINT0::NYET;
    pub use super::OTG_DOEPINT0::OTEPDIS;
    pub use super::OTG_DOEPINT0::OUTPKTERR;
    pub use super::OTG_DOEPINT0::STPKTRX;
    pub use super::OTG_DOEPINT0::STSPHSRX;
    pub use super::OTG_DOEPINT0::STUP;
    pub use super::OTG_DOEPINT0::XFRC;
}

/// The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
pub mod OTG_DOEPTSIZ2 {
    pub use super::OTG_DOEPTSIZ1::PKTCNT;
    pub use super::OTG_DOEPTSIZ1::RXDPID_STUPCNT;
    pub use super::OTG_DOEPTSIZ1::XFRSIZ;
}

/// OTG device OUT endpoint 2 DMA address register
pub mod OTG_DOEPDMA2 {
    pub use super::OTG_HCDMA0::DMAADDR;
}

/// The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
pub mod OTG_DOEPCTL3 {
    pub use super::OTG_DOEPCTL1::CNAK;
    pub use super::OTG_DOEPCTL1::EONUM_DPIP;
    pub use super::OTG_DOEPCTL1::EPDIS;
    pub use super::OTG_DOEPCTL1::EPENA;
    pub use super::OTG_DOEPCTL1::EPTYP;
    pub use super::OTG_DOEPCTL1::MPSIZ;
    pub use super::OTG_DOEPCTL1::NAKSTS;
    pub use super::OTG_DOEPCTL1::SD0PID_SEVNFRM;
    pub use super::OTG_DOEPCTL1::SD1PID_SODDFRM;
    pub use super::OTG_DOEPCTL1::SNAK;
    pub use super::OTG_DOEPCTL1::SNPM;
    pub use super::OTG_DOEPCTL1::STALL;
    pub use super::OTG_DOEPCTL1::USBAEP;
}

/// This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.
pub mod OTG_DOEPINT3 {
    pub use super::OTG_DOEPINT0::AHBERR;
    pub use super::OTG_DOEPINT0::B2BSTUP;
    pub use super::OTG_DOEPINT0::BERR;
    pub use super::OTG_DOEPINT0::BNA;
    pub use super::OTG_DOEPINT0::EPDISD;
    pub use super::OTG_DOEPINT0::NAK;
    pub use super::OTG_DOEPINT0::NYET;
    pub use super::OTG_DOEPINT0::OTEPDIS;
    pub use super::OTG_DOEPINT0::OUTPKTERR;
    pub use super::OTG_DOEPINT0::STPKTRX;
    pub use super::OTG_DOEPINT0::STSPHSRX;
    pub use super::OTG_DOEPINT0::STUP;
    pub use super::OTG_DOEPINT0::XFRC;
}

/// The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
pub mod OTG_DOEPTSIZ3 {
    pub use super::OTG_DOEPTSIZ1::PKTCNT;
    pub use super::OTG_DOEPTSIZ1::RXDPID_STUPCNT;
    pub use super::OTG_DOEPTSIZ1::XFRSIZ;
}

/// OTG device OUT endpoint 3 DMA address register
pub mod OTG_DOEPDMA3 {
    pub use super::OTG_HCDMA0::DMAADDR;
}

/// The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
pub mod OTG_DOEPCTL4 {
    pub use super::OTG_DOEPCTL1::CNAK;
    pub use super::OTG_DOEPCTL1::EONUM_DPIP;
    pub use super::OTG_DOEPCTL1::EPDIS;
    pub use super::OTG_DOEPCTL1::EPENA;
    pub use super::OTG_DOEPCTL1::EPTYP;
    pub use super::OTG_DOEPCTL1::MPSIZ;
    pub use super::OTG_DOEPCTL1::NAKSTS;
    pub use super::OTG_DOEPCTL1::SD0PID_SEVNFRM;
    pub use super::OTG_DOEPCTL1::SD1PID_SODDFRM;
    pub use super::OTG_DOEPCTL1::SNAK;
    pub use super::OTG_DOEPCTL1::SNPM;
    pub use super::OTG_DOEPCTL1::STALL;
    pub use super::OTG_DOEPCTL1::USBAEP;
}

/// This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.
pub mod OTG_DOEPINT4 {
    pub use super::OTG_DOEPINT0::AHBERR;
    pub use super::OTG_DOEPINT0::B2BSTUP;
    pub use super::OTG_DOEPINT0::BERR;
    pub use super::OTG_DOEPINT0::BNA;
    pub use super::OTG_DOEPINT0::EPDISD;
    pub use super::OTG_DOEPINT0::NAK;
    pub use super::OTG_DOEPINT0::NYET;
    pub use super::OTG_DOEPINT0::OTEPDIS;
    pub use super::OTG_DOEPINT0::OUTPKTERR;
    pub use super::OTG_DOEPINT0::STPKTRX;
    pub use super::OTG_DOEPINT0::STSPHSRX;
    pub use super::OTG_DOEPINT0::STUP;
    pub use super::OTG_DOEPINT0::XFRC;
}

/// The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
pub mod OTG_DOEPTSIZ4 {
    pub use super::OTG_DOEPTSIZ1::PKTCNT;
    pub use super::OTG_DOEPTSIZ1::RXDPID_STUPCNT;
    pub use super::OTG_DOEPTSIZ1::XFRSIZ;
}

/// OTG device OUT endpoint 4 DMA address register
pub mod OTG_DOEPDMA4 {
    pub use super::OTG_HCDMA0::DMAADDR;
}

/// The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
pub mod OTG_DOEPCTL5 {
    pub use super::OTG_DOEPCTL1::CNAK;
    pub use super::OTG_DOEPCTL1::EONUM_DPIP;
    pub use super::OTG_DOEPCTL1::EPDIS;
    pub use super::OTG_DOEPCTL1::EPENA;
    pub use super::OTG_DOEPCTL1::EPTYP;
    pub use super::OTG_DOEPCTL1::MPSIZ;
    pub use super::OTG_DOEPCTL1::NAKSTS;
    pub use super::OTG_DOEPCTL1::SD0PID_SEVNFRM;
    pub use super::OTG_DOEPCTL1::SD1PID_SODDFRM;
    pub use super::OTG_DOEPCTL1::SNAK;
    pub use super::OTG_DOEPCTL1::SNPM;
    pub use super::OTG_DOEPCTL1::STALL;
    pub use super::OTG_DOEPCTL1::USBAEP;
}

/// This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.
pub mod OTG_DOEPINT5 {
    pub use super::OTG_DOEPINT0::AHBERR;
    pub use super::OTG_DOEPINT0::B2BSTUP;
    pub use super::OTG_DOEPINT0::BERR;
    pub use super::OTG_DOEPINT0::BNA;
    pub use super::OTG_DOEPINT0::EPDISD;
    pub use super::OTG_DOEPINT0::NAK;
    pub use super::OTG_DOEPINT0::NYET;
    pub use super::OTG_DOEPINT0::OTEPDIS;
    pub use super::OTG_DOEPINT0::OUTPKTERR;
    pub use super::OTG_DOEPINT0::STPKTRX;
    pub use super::OTG_DOEPINT0::STSPHSRX;
    pub use super::OTG_DOEPINT0::STUP;
    pub use super::OTG_DOEPINT0::XFRC;
}

/// The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
pub mod OTG_DOEPTSIZ5 {
    pub use super::OTG_DOEPTSIZ1::PKTCNT;
    pub use super::OTG_DOEPTSIZ1::RXDPID_STUPCNT;
    pub use super::OTG_DOEPTSIZ1::XFRSIZ;
}

/// OTG device OUT endpoint 5 DMA address register
pub mod OTG_DOEPDMA5 {
    pub use super::OTG_HCDMA0::DMAADDR;
}

/// The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
pub mod OTG_DOEPCTL6 {
    pub use super::OTG_DOEPCTL1::CNAK;
    pub use super::OTG_DOEPCTL1::EONUM_DPIP;
    pub use super::OTG_DOEPCTL1::EPDIS;
    pub use super::OTG_DOEPCTL1::EPENA;
    pub use super::OTG_DOEPCTL1::EPTYP;
    pub use super::OTG_DOEPCTL1::MPSIZ;
    pub use super::OTG_DOEPCTL1::NAKSTS;
    pub use super::OTG_DOEPCTL1::SD0PID_SEVNFRM;
    pub use super::OTG_DOEPCTL1::SD1PID_SODDFRM;
    pub use super::OTG_DOEPCTL1::SNAK;
    pub use super::OTG_DOEPCTL1::SNPM;
    pub use super::OTG_DOEPCTL1::STALL;
    pub use super::OTG_DOEPCTL1::USBAEP;
}

/// This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.
pub mod OTG_DOEPINT6 {
    pub use super::OTG_DOEPINT0::AHBERR;
    pub use super::OTG_DOEPINT0::B2BSTUP;
    pub use super::OTG_DOEPINT0::BERR;
    pub use super::OTG_DOEPINT0::BNA;
    pub use super::OTG_DOEPINT0::EPDISD;
    pub use super::OTG_DOEPINT0::NAK;
    pub use super::OTG_DOEPINT0::NYET;
    pub use super::OTG_DOEPINT0::OTEPDIS;
    pub use super::OTG_DOEPINT0::OUTPKTERR;
    pub use super::OTG_DOEPINT0::STPKTRX;
    pub use super::OTG_DOEPINT0::STSPHSRX;
    pub use super::OTG_DOEPINT0::STUP;
    pub use super::OTG_DOEPINT0::XFRC;
}

/// The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
pub mod OTG_DOEPTSIZ6 {
    pub use super::OTG_DOEPTSIZ1::PKTCNT;
    pub use super::OTG_DOEPTSIZ1::RXDPID_STUPCNT;
    pub use super::OTG_DOEPTSIZ1::XFRSIZ;
}

/// OTG device OUT endpoint 6 DMA address register
pub mod OTG_DOEPDMA6 {
    pub use super::OTG_HCDMA0::DMAADDR;
}

/// The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
pub mod OTG_DOEPCTL7 {
    pub use super::OTG_DOEPCTL1::CNAK;
    pub use super::OTG_DOEPCTL1::EONUM_DPIP;
    pub use super::OTG_DOEPCTL1::EPDIS;
    pub use super::OTG_DOEPCTL1::EPENA;
    pub use super::OTG_DOEPCTL1::EPTYP;
    pub use super::OTG_DOEPCTL1::MPSIZ;
    pub use super::OTG_DOEPCTL1::NAKSTS;
    pub use super::OTG_DOEPCTL1::SD0PID_SEVNFRM;
    pub use super::OTG_DOEPCTL1::SD1PID_SODDFRM;
    pub use super::OTG_DOEPCTL1::SNAK;
    pub use super::OTG_DOEPCTL1::SNPM;
    pub use super::OTG_DOEPCTL1::STALL;
    pub use super::OTG_DOEPCTL1::USBAEP;
}

/// This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.
pub mod OTG_DOEPINT7 {
    pub use super::OTG_DOEPINT0::AHBERR;
    pub use super::OTG_DOEPINT0::B2BSTUP;
    pub use super::OTG_DOEPINT0::BERR;
    pub use super::OTG_DOEPINT0::BNA;
    pub use super::OTG_DOEPINT0::EPDISD;
    pub use super::OTG_DOEPINT0::NAK;
    pub use super::OTG_DOEPINT0::NYET;
    pub use super::OTG_DOEPINT0::OTEPDIS;
    pub use super::OTG_DOEPINT0::OUTPKTERR;
    pub use super::OTG_DOEPINT0::STPKTRX;
    pub use super::OTG_DOEPINT0::STSPHSRX;
    pub use super::OTG_DOEPINT0::STUP;
    pub use super::OTG_DOEPINT0::XFRC;
}

/// The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
pub mod OTG_DOEPTSIZ7 {
    pub use super::OTG_DOEPTSIZ1::PKTCNT;
    pub use super::OTG_DOEPTSIZ1::RXDPID_STUPCNT;
    pub use super::OTG_DOEPTSIZ1::XFRSIZ;
}

/// OTG device OUT endpoint 7 DMA address register
pub mod OTG_DOEPDMA7 {
    pub use super::OTG_HCDMA0::DMAADDR;
}

/// The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
pub mod OTG_DOEPCTL8 {
    pub use super::OTG_DOEPCTL1::CNAK;
    pub use super::OTG_DOEPCTL1::EONUM_DPIP;
    pub use super::OTG_DOEPCTL1::EPDIS;
    pub use super::OTG_DOEPCTL1::EPENA;
    pub use super::OTG_DOEPCTL1::EPTYP;
    pub use super::OTG_DOEPCTL1::MPSIZ;
    pub use super::OTG_DOEPCTL1::NAKSTS;
    pub use super::OTG_DOEPCTL1::SD0PID_SEVNFRM;
    pub use super::OTG_DOEPCTL1::SD1PID_SODDFRM;
    pub use super::OTG_DOEPCTL1::SNAK;
    pub use super::OTG_DOEPCTL1::SNPM;
    pub use super::OTG_DOEPCTL1::STALL;
    pub use super::OTG_DOEPCTL1::USBAEP;
}

/// This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.
pub mod OTG_DOEPINT8 {
    pub use super::OTG_DOEPINT0::AHBERR;
    pub use super::OTG_DOEPINT0::B2BSTUP;
    pub use super::OTG_DOEPINT0::BERR;
    pub use super::OTG_DOEPINT0::BNA;
    pub use super::OTG_DOEPINT0::EPDISD;
    pub use super::OTG_DOEPINT0::NAK;
    pub use super::OTG_DOEPINT0::NYET;
    pub use super::OTG_DOEPINT0::OTEPDIS;
    pub use super::OTG_DOEPINT0::OUTPKTERR;
    pub use super::OTG_DOEPINT0::STPKTRX;
    pub use super::OTG_DOEPINT0::STSPHSRX;
    pub use super::OTG_DOEPINT0::STUP;
    pub use super::OTG_DOEPINT0::XFRC;
}

/// The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
pub mod OTG_DOEPTSIZ8 {
    pub use super::OTG_DOEPTSIZ1::PKTCNT;
    pub use super::OTG_DOEPTSIZ1::RXDPID_STUPCNT;
    pub use super::OTG_DOEPTSIZ1::XFRSIZ;
}

/// OTG device OUT endpoint 8 DMA address register
pub mod OTG_DOEPDMA8 {
    pub use super::OTG_HCDMA0::DMAADDR;
}

/// This register is available in host and device modes.
pub mod OTG_PCGCCTL {

    /// STPPCLK
    pub mod STPPCLK {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// GATEHCLK
    pub mod GATEHCLK {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PHYSUSP
    pub mod PHYSUSP {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ENL1GTG
    pub mod ENL1GTG {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PHYSLEEP
    pub mod PHYSLEEP {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// SUSP
    pub mod SUSP {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// The OTG_GOTGCTL register controls the behavior and reflects the status of the OTG function of the core.
    pub OTG_GOTGCTL: RWRegister<u32>,

    /// The application reads this register whenever there is an OTG interrupt and clears the bits in this register to clear the OTG interrupt.
    pub OTG_GOTGINT: RWRegister<u32>,

    /// This register can be used to configure the core after power-on or a change in mode. This register mainly contains AHB system-related configuration parameters. Do not change this register after the initial programming. The application must program this register before starting any transactions on either the AHB or the USB.
    pub OTG_GAHBCFG: RWRegister<u32>,

    /// This register can be used to configure the core after power-on or a changing to host mode or device mode. It contains USB and USB-PHY related configuration parameters. The application must program this register before starting any transactions on either the AHB or the USB. Do not make changes to this register after the initial programming.
    pub OTG_GUSBCFG: RWRegister<u32>,

    /// The application uses this register to reset various hardware features inside the core.
    pub OTG_GRSTCTL: RWRegister<u32>,

    /// This register interrupts the application for system-level events in the current mode (device mode or host mode). Some of the bits in this register are valid only in host mode, while others are valid in device mode only. This register also indicates the current mode. To clear the interrupt status bits of the rc_w1 type, the application must write 1 into the bit. The FIFO status interrupts are read-only; once software reads from or writes to the FIFO while servicing these interrupts, FIFO interrupt conditions are cleared automatically. The application must clear the OTG_GINTSTS register at initialization before unmasking the interrupt bit to avoid any interrupts generated prior to initialization.
    pub OTG_GINTSTS: RWRegister<u32>,

    /// This register works with the core interrupt register to interrupt the application. When an interrupt bit is masked, the interrupt associated with that bit is not generated. However, the core interrupt (OTG_GINTSTS) register bit corresponding to that interrupt is still set.
    pub OTG_GINTMSK: RWRegister<u32>,

    /// This description is for register OTG_GRXSTSR in Device mode. A read to the receive status debug read register returns the contents of the top of the receive FIFO. The core ignores the receive status read when the receive FIFO is empty and returns a value of 0x00000000.
    pub OTG_GRXSTSR: RORegister<u32>,

    /// This description is for register OTG_GRXSTSP in Device mode. Similarly to OTG_GRXSTSR (receive status debug read register) where a read returns the contents of the top of the receive FIFO, a read to OTG_GRXSTSP (receive status read and pop register) additionally pops the top data entry out of the Rx FIFO. The core ignores the receive status pop/read when the receive FIFO is empty and returns a value of 0x00000000. The application must only pop the receive status FIFO when the receive FIFO non-empty bit of the core interrupt register (RXFLVL bit in OTG_GINTSTS) is asserted.
    pub OTG_GRXSTSP: RORegister<u32>,

    /// The application can program the RAM size that must be allocated to the Rx FIFO.
    pub OTG_GRXFSIZ: RWRegister<u32>,

    /// Host mode
    pub OTG_HNPTXFSIZ: RWRegister<u32>,

    /// In device mode, this register is not valid. This read-only register contains the free space information for the non-periodic Tx FIFO and the non-periodic transmit request queue.
    pub OTG_HNPTXSTS: RORegister<u32>,

    _reserved1: [u32; 2],

    /// OTG general core configuration register
    pub OTG_GCCFG: RWRegister<u32>,

    /// This is a register containing the Product ID as reset value.
    pub OTG_CID: RWRegister<u32>,

    _reserved2: [u32; 5],

    /// OTG core LPM configuration register
    pub OTG_GLPMCFG: RWRegister<u32>,

    _reserved3: [u32; 42],

    /// OTG host periodic transmit FIFO size register
    pub OTG_HPTXFSIZ: RWRegister<u32>,

    /// OTG device IN endpoint transmit FIFO 1 size register
    pub OTG_DIEPTXF1: RWRegister<u32>,

    /// OTG device IN endpoint transmit FIFO 2 size register
    pub OTG_DIEPTXF2: RWRegister<u32>,

    /// OTG device IN endpoint transmit FIFO 3 size register
    pub OTG_DIEPTXF3: RWRegister<u32>,

    /// OTG device IN endpoint transmit FIFO 4 size register
    pub OTG_DIEPTXF4: RWRegister<u32>,

    /// OTG device IN endpoint transmit FIFO 5 size register
    pub OTG_DIEPTXF5: RWRegister<u32>,

    /// OTG device IN endpoint transmit FIFO 6 size register
    pub OTG_DIEPTXF6: RWRegister<u32>,

    /// OTG device IN endpoint transmit FIFO 7 size register
    pub OTG_DIEPTXF7: RWRegister<u32>,

    /// OTG device IN endpoint transmit FIFO 8 size register
    pub OTG_DIEPTXF8: RWRegister<u32>,

    _reserved4: [u32; 183],

    /// This register configures the core after power-on. Do not make changes to this register after initializing the host.
    pub OTG_HCFG: RWRegister<u32>,

    /// This register stores the frame interval information for the current speed to which the OTG controller has enumerated.
    pub OTG_HFIR: RWRegister<u32>,

    /// This register indicates the current frame number. It also indicates the time remaining (in terms of the number of PHY clocks) in the current frame.
    pub OTG_HFNUM: RORegister<u32>,

    _reserved5: [u32; 1],

    /// This read-only register contains the free space information for the periodic Tx FIFO and the periodic transmit request queue.
    pub OTG_HPTXSTS: RORegister<u32>,

    /// When a significant event occurs on a channel, the host all channels interrupt register interrupts the application using the host channels interrupt bit of the core interrupt register (HCINT bit in OTG_GINTSTS). This is shown in Figure724. There is one interrupt bit per channel, up to a maximum of 16 bits. Bits in this register are set and cleared when the application sets and clears bits in the corresponding host channel-x interrupt register.
    pub OTG_HAINT: RORegister<u32>,

    /// The host all channel interrupt mask register works with the host all channel interrupt register to interrupt the application when an event occurs on a channel. There is one interrupt mask bit per channel, up to a maximum of 16 bits.
    pub OTG_HAINTMSK: RWRegister<u32>,

    /// This register holds the starting address of the frame list information (scatter/gather mode).
    pub OTG_HFLBADDR: RWRegister<u32>,

    _reserved6: [u32; 8],

    /// This register is available only in host mode. Currently, the OTG host supports only one port. A single register holds USB port-related information such as USB reset, enable, suspend, resume, connect status, and test mode for each port. It is shown in Figure724. The rc_w1 bits in this register can trigger an interrupt to the application through the host port interrupt bit of the core interrupt register (HPRTINT bit in OTG_GINTSTS). On a port interrupt, the application must read this register and clear the bit that caused the interrupt. For the rc_w1 bits, the application must write a 1 to the bit to clear the interrupt.
    pub OTG_HPRT: RWRegister<u32>,

    _reserved7: [u32; 47],

    /// OTG host channel 0 characteristics register
    pub OTG_HCCHAR0: RWRegister<u32>,

    /// OTG host channel 0 split control register
    pub OTG_HCSPLT0: RWRegister<u32>,

    /// This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.
    pub OTG_HCINT0: RWRegister<u32>,

    /// This register reflects the mask for each channel status described in the previous section.
    pub OTG_HCINTMSK0: RWRegister<u32>,

    /// OTG host channel 0 transfer size register
    pub OTG_HCTSIZ0: RWRegister<u32>,

    /// OTG host channel 0 DMA address register in buffer DMA \[alternate\]
    pub OTG_HCDMA0: RWRegister<u32>,

    _reserved8: [u32; 1],

    /// OTG host channel-n DMA address buffer register
    pub OTG_HCDMAB0: RORegister<u32>,

    /// OTG host channel 1 characteristics register
    pub OTG_HCCHAR1: RWRegister<u32>,

    /// OTG host channel 1 split control register
    pub OTG_HCSPLT1: RWRegister<u32>,

    /// This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.
    pub OTG_HCINT1: RWRegister<u32>,

    /// This register reflects the mask for each channel status described in the previous section.
    pub OTG_HCINTMSK1: RWRegister<u32>,

    /// OTG host channel 1 transfer size register
    pub OTG_HCTSIZ1: RWRegister<u32>,

    /// OTG host channel 1 DMA address register in buffer DMA \[alternate\]
    pub OTG_HCDMA1: RWRegister<u32>,

    _reserved9: [u32; 1],

    /// OTG host channel-n DMA address buffer register
    pub OTG_HCDMAB1: RORegister<u32>,

    /// OTG host channel 2 characteristics register
    pub OTG_HCCHAR2: RWRegister<u32>,

    /// OTG host channel 2 split control register
    pub OTG_HCSPLT2: RWRegister<u32>,

    /// This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.
    pub OTG_HCINT2: RWRegister<u32>,

    /// This register reflects the mask for each channel status described in the previous section.
    pub OTG_HCINTMSK2: RWRegister<u32>,

    /// OTG host channel 2 transfer size register
    pub OTG_HCTSIZ2: RWRegister<u32>,

    /// OTG host channel 2 DMA address register in buffer DMA \[alternate\]
    pub OTG_HCDMA2: RWRegister<u32>,

    _reserved10: [u32; 1],

    /// OTG host channel-n DMA address buffer register
    pub OTG_HCDMAB2: RORegister<u32>,

    /// OTG host channel 3 characteristics register
    pub OTG_HCCHAR3: RWRegister<u32>,

    /// OTG host channel 3 split control register
    pub OTG_HCSPLT3: RWRegister<u32>,

    /// This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.
    pub OTG_HCINT3: RWRegister<u32>,

    /// This register reflects the mask for each channel status described in the previous section.
    pub OTG_HCINTMSK3: RWRegister<u32>,

    /// OTG host channel 3 transfer size register
    pub OTG_HCTSIZ3: RWRegister<u32>,

    /// OTG host channel 3 DMA address register in buffer DMA \[alternate\]
    pub OTG_HCDMA3: RWRegister<u32>,

    _reserved11: [u32; 1],

    /// OTG host channel-n DMA address buffer register
    pub OTG_HCDMAB3: RORegister<u32>,

    /// OTG host channel 4 characteristics register
    pub OTG_HCCHAR4: RWRegister<u32>,

    /// OTG host channel 4 split control register
    pub OTG_HCSPLT4: RWRegister<u32>,

    /// This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.
    pub OTG_HCINT4: RWRegister<u32>,

    /// This register reflects the mask for each channel status described in the previous section.
    pub OTG_HCINTMSK4: RWRegister<u32>,

    /// OTG host channel 4 transfer size register
    pub OTG_HCTSIZ4: RWRegister<u32>,

    /// OTG host channel 4 DMA address register in buffer DMA \[alternate\]
    pub OTG_HCDMA4: RWRegister<u32>,

    _reserved12: [u32; 1],

    /// OTG host channel-n DMA address buffer register
    pub OTG_HCDMAB4: RORegister<u32>,

    /// OTG host channel 5 characteristics register
    pub OTG_HCCHAR5: RWRegister<u32>,

    /// OTG host channel 5 split control register
    pub OTG_HCSPLT5: RWRegister<u32>,

    /// This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.
    pub OTG_HCINT5: RWRegister<u32>,

    /// This register reflects the mask for each channel status described in the previous section.
    pub OTG_HCINTMSK5: RWRegister<u32>,

    /// OTG host channel 5 transfer size register
    pub OTG_HCTSIZ5: RWRegister<u32>,

    /// OTG host channel 5 DMA address register in buffer DMA \[alternate\]
    pub OTG_HCDMA5: RWRegister<u32>,

    _reserved13: [u32; 1],

    /// OTG host channel-n DMA address buffer register
    pub OTG_HCDMAB5: RORegister<u32>,

    /// OTG host channel 6 characteristics register
    pub OTG_HCCHAR6: RWRegister<u32>,

    /// OTG host channel 6 split control register
    pub OTG_HCSPLT6: RWRegister<u32>,

    /// This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.
    pub OTG_HCINT6: RWRegister<u32>,

    /// This register reflects the mask for each channel status described in the previous section.
    pub OTG_HCINTMSK6: RWRegister<u32>,

    /// OTG host channel 6 transfer size register
    pub OTG_HCTSIZ6: RWRegister<u32>,

    /// OTG host channel 6 DMA address register in buffer DMA \[alternate\]
    pub OTG_HCDMA6: RWRegister<u32>,

    _reserved14: [u32; 1],

    /// OTG host channel-n DMA address buffer register
    pub OTG_HCDMAB6: RORegister<u32>,

    /// OTG host channel 7 characteristics register
    pub OTG_HCCHAR7: RWRegister<u32>,

    /// OTG host channel 7 split control register
    pub OTG_HCSPLT7: RWRegister<u32>,

    /// This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.
    pub OTG_HCINT7: RWRegister<u32>,

    /// This register reflects the mask for each channel status described in the previous section.
    pub OTG_HCINTMSK7: RWRegister<u32>,

    /// OTG host channel 7 transfer size register
    pub OTG_HCTSIZ7: RWRegister<u32>,

    /// OTG host channel 7 DMA address register in buffer DMA \[alternate\]
    pub OTG_HCDMA7: RWRegister<u32>,

    _reserved15: [u32; 1],

    /// OTG host channel-n DMA address buffer register
    pub OTG_HCDMAB7: RORegister<u32>,

    /// OTG host channel 8 characteristics register
    pub OTG_HCCHAR8: RWRegister<u32>,

    /// OTG host channel 8 split control register
    pub OTG_HCSPLT8: RWRegister<u32>,

    /// This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.
    pub OTG_HCINT8: RWRegister<u32>,

    /// This register reflects the mask for each channel status described in the previous section.
    pub OTG_HCINTMSK8: RWRegister<u32>,

    /// OTG host channel 8 transfer size register
    pub OTG_HCTSIZ8: RWRegister<u32>,

    /// OTG host channel 8 DMA address register in buffer DMA \[alternate\]
    pub OTG_HCDMA8: RWRegister<u32>,

    _reserved16: [u32; 1],

    /// OTG host channel-n DMA address buffer register
    pub OTG_HCDMAB8: RORegister<u32>,

    /// OTG host channel 9 characteristics register
    pub OTG_HCCHAR9: RWRegister<u32>,

    /// OTG host channel 9 split control register
    pub OTG_HCSPLT9: RWRegister<u32>,

    /// This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.
    pub OTG_HCINT9: RWRegister<u32>,

    /// This register reflects the mask for each channel status described in the previous section.
    pub OTG_HCINTMSK9: RWRegister<u32>,

    /// OTG host channel 9 transfer size register
    pub OTG_HCTSIZ9: RWRegister<u32>,

    /// OTG host channel 9 DMA address register in buffer DMA \[alternate\]
    pub OTG_HCDMA9: RWRegister<u32>,

    _reserved17: [u32; 1],

    /// OTG host channel-n DMA address buffer register
    pub OTG_HCDMAB9: RORegister<u32>,

    /// OTG host channel 10 characteristics register
    pub OTG_HCCHAR10: RWRegister<u32>,

    /// OTG host channel 10 split control register
    pub OTG_HCSPLT10: RWRegister<u32>,

    /// This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.
    pub OTG_HCINT10: RWRegister<u32>,

    /// This register reflects the mask for each channel status described in the previous section.
    pub OTG_HCINTMSK10: RWRegister<u32>,

    /// OTG host channel 10 transfer size register
    pub OTG_HCTSIZ10: RWRegister<u32>,

    /// OTG host channel 10 DMA address register in buffer DMA \[alternate\]
    pub OTG_HCDMA10: RWRegister<u32>,

    _reserved18: [u32; 1],

    /// OTG host channel-n DMA address buffer register
    pub OTG_HCDMAB10: RORegister<u32>,

    /// OTG host channel 11 characteristics register
    pub OTG_HCCHAR11: RWRegister<u32>,

    /// OTG host channel 11 split control register
    pub OTG_HCSPLT11: RWRegister<u32>,

    /// This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.
    pub OTG_HCINT11: RWRegister<u32>,

    /// This register reflects the mask for each channel status described in the previous section.
    pub OTG_HCINTMSK11: RWRegister<u32>,

    /// OTG host channel 11 transfer size register
    pub OTG_HCTSIZ11: RWRegister<u32>,

    /// OTG host channel 11 DMA address register in buffer DMA \[alternate\]
    pub OTG_HCDMA11: RWRegister<u32>,

    _reserved19: [u32; 1],

    /// OTG host channel-n DMA address buffer register
    pub OTG_HCDMAB11: RORegister<u32>,

    /// OTG host channel 12 characteristics register
    pub OTG_HCCHAR12: RWRegister<u32>,

    /// OTG host channel 12 split control register
    pub OTG_HCSPLT12: RWRegister<u32>,

    /// This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.
    pub OTG_HCINT12: RWRegister<u32>,

    /// This register reflects the mask for each channel status described in the previous section.
    pub OTG_HCINTMSK12: RWRegister<u32>,

    /// OTG host channel 12 transfer size register
    pub OTG_HCTSIZ12: RWRegister<u32>,

    /// OTG host channel 12 DMA address register in buffer DMA \[alternate\]
    pub OTG_HCDMA12: RWRegister<u32>,

    _reserved20: [u32; 1],

    /// OTG host channel-n DMA address buffer register
    pub OTG_HCDMAB12: RORegister<u32>,

    /// OTG host channel 13 characteristics register
    pub OTG_HCCHAR13: RWRegister<u32>,

    /// OTG host channel 13 split control register
    pub OTG_HCSPLT13: RWRegister<u32>,

    /// This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.
    pub OTG_HCINT13: RWRegister<u32>,

    /// This register reflects the mask for each channel status described in the previous section.
    pub OTG_HCINTMSK13: RWRegister<u32>,

    /// OTG host channel 13 transfer size register
    pub OTG_HCTSIZ13: RWRegister<u32>,

    /// OTG host channel 13 DMA address register in buffer DMA \[alternate\]
    pub OTG_HCDMA13: RWRegister<u32>,

    _reserved21: [u32; 1],

    /// OTG host channel-n DMA address buffer register
    pub OTG_HCDMAB13: RORegister<u32>,

    /// OTG host channel 14 characteristics register
    pub OTG_HCCHAR14: RWRegister<u32>,

    /// OTG host channel 14 split control register
    pub OTG_HCSPLT14: RWRegister<u32>,

    /// This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.
    pub OTG_HCINT14: RWRegister<u32>,

    /// This register reflects the mask for each channel status described in the previous section.
    pub OTG_HCINTMSK14: RWRegister<u32>,

    /// OTG host channel 14 transfer size register
    pub OTG_HCTSIZ14: RWRegister<u32>,

    /// OTG host channel 14 DMA address register in buffer DMA \[alternate\]
    pub OTG_HCDMA14: RWRegister<u32>,

    _reserved22: [u32; 1],

    /// OTG host channel-n DMA address buffer register
    pub OTG_HCDMAB14: RORegister<u32>,

    /// OTG host channel 15 characteristics register
    pub OTG_HCCHAR15: RWRegister<u32>,

    /// OTG host channel 15 split control register
    pub OTG_HCSPLT15: RWRegister<u32>,

    /// This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the host channels interrupt bit in the core interrupt register (HCINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the host all channels interrupt (OTG_HAINT) register to get the exact channel number for the host channel-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_HAINT and OTG_GINTSTS registers.
    pub OTG_HCINT15: RWRegister<u32>,

    /// This register reflects the mask for each channel status described in the previous section.
    pub OTG_HCINTMSK15: RWRegister<u32>,

    /// OTG host channel 15 transfer size register
    pub OTG_HCTSIZ15: RWRegister<u32>,

    /// OTG host channel 15 DMA address register in buffer DMA \[alternate\]
    pub OTG_HCDMA15: RWRegister<u32>,

    _reserved23: [u32; 1],

    /// OTG host channel-n DMA address buffer register
    pub OTG_HCDMAB15: RORegister<u32>,

    _reserved24: [u32; 64],

    /// This register configures the core in device mode after power-on or after certain control commands or enumeration. Do not make changes to this register after initial programming.
    pub OTG_DCFG: RWRegister<u32>,

    /// OTG device control register
    pub OTG_DCTL: RWRegister<u32>,

    /// This register indicates the status of the core with respect to USB-related events. It must be read on interrupts from the device all interrupts (OTG_DAINT) register.
    pub OTG_DSTS: RORegister<u32>,

    _reserved25: [u32; 1],

    /// This register works with each of the OTG_DIEPINTx registers for all endpoints to generate an interrupt per IN endpoint. The IN endpoint interrupt for a specific status in the OTG_DIEPINTx register can be masked by writing to the corresponding bit in this register. Status bits are masked by default.
    pub OTG_DIEPMSK: RWRegister<u32>,

    /// This register works with each of the OTG_DOEPINTx registers for all endpoints to generate an interrupt per OUT endpoint. The OUT endpoint interrupt for a specific status in the OTG_DOEPINTx register can be masked by writing into the corresponding bit in this register. Status bits are masked by default.
    pub OTG_DOEPMSK: RWRegister<u32>,

    /// When a significant event occurs on an endpoint, a OTG_DAINT register interrupts the application using the device OUT endpoints interrupt bit or device IN endpoints interrupt bit of the OTG_GINTSTS register (OEPINT or IEPINT in OTG_GINTSTS, respectively). There is one interrupt bit per endpoint, up to a maximum of 16 bits for OUT endpoints and 16 bits for IN endpoints. For a bidirectional endpoint, the corresponding IN and OUT interrupt bits are used. Bits in this register are set and cleared when the application sets and clears bits in the corresponding device endpoint-x interrupt register (OTG_DIEPINTx/OTG_DOEPINTx).
    pub OTG_DAINT: RORegister<u32>,

    /// The OTG_DAINTMSK register works with the device endpoint interrupt register to interrupt the application when an event occurs on a device endpoint. However, the OTG_DAINT register bit corresponding to that interrupt is still set.
    pub OTG_DAINTMSK: RWRegister<u32>,

    _reserved26: [u32; 2],

    /// This register specifies the VBUS discharge time after VBUS pulsing during SRP.
    pub OTG_DVBUSDIS: RWRegister<u32>,

    /// This register specifies the VBUS pulsing time during SRP.
    pub OTG_DVBUSPULSE: RWRegister<u32>,

    /// OTG device threshold control register
    pub OTG_DTHRCTL: RWRegister<u32>,

    /// This register is used to control the IN endpoint FIFO empty interrupt generation (TXFE_OTG_DIEPINTx).
    pub OTG_DIEPEMPMSK: RWRegister<u32>,

    /// OTG device each endpoint interrupt register
    pub OTG_DEACHINT: RORegister<u32>,

    /// There is one interrupt bit for endpoint 1 IN and one interrupt bit for endpoint 1 OUT.
    pub OTG_DEACHINTMSK: RWRegister<u32>,

    _reserved27: [u32; 1],

    /// This register works with the OTG_DIEPINT1 register to generate a dedicated interrupt OTG_HS_EP1_IN for endpoint #1. The IN endpoint interrupt for a specific status in the OTG_DOEPINT1 register can be masked by writing into the corresponding bit in this register. Status bits are masked by default.
    pub OTG_HS_DIEPEACHMSK1: RWRegister<u32>,

    _reserved28: [u32; 15],

    /// This register works with the OTG_DOEPINT1 register to generate a dedicated interrupt OTG_HS_EP1_OUT for endpoint #1. The OUT endpoint interrupt for a specific status in the OTG_DOEPINT1 register can be masked by writing into the corresponding bit in this register. Status bits are masked by default.
    pub OTG_HS_DOEPEACHMSK1: RWRegister<u32>,

    _reserved29: [u32; 30],

    /// The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
    pub OTG_DIEPCTL0: RWRegister<u32>,

    _reserved30: [u32; 1],

    /// This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.
    pub OTG_DIEPINT0: RWRegister<u32>,

    _reserved31: [u32; 1],

    /// The application must modify this register before enabling endpoint 0.
    pub OTG_DIEPTSIZ0: RWRegister<u32>,

    /// OTG device IN endpoint 0 DMA address register
    pub OTG_DIEPDMA0: RWRegister<u32>,

    /// This read-only register contains the free space information for the device IN endpoint Tx FIFO.
    pub OTG_DTXFSTS0: RORegister<u32>,

    _reserved32: [u32; 1],

    /// The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
    pub OTG_DIEPCTL1: RWRegister<u32>,

    _reserved33: [u32; 1],

    /// This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.
    pub OTG_DIEPINT1: RWRegister<u32>,

    _reserved34: [u32; 1],

    /// The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
    pub OTG_DIEPTSIZ1: RWRegister<u32>,

    /// OTG device IN endpoint 1 DMA address register
    pub OTG_DIEPDMA1: RWRegister<u32>,

    /// This read-only register contains the free space information for the device IN endpoint Tx FIFO.
    pub OTG_DTXFSTS1: RORegister<u32>,

    _reserved35: [u32; 1],

    /// The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
    pub OTG_DIEPCTL2: RWRegister<u32>,

    _reserved36: [u32; 1],

    /// This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.
    pub OTG_DIEPINT2: RWRegister<u32>,

    _reserved37: [u32; 1],

    /// The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
    pub OTG_DIEPTSIZ2: RWRegister<u32>,

    /// OTG device IN endpoint 2 DMA address register
    pub OTG_DIEPDMA2: RWRegister<u32>,

    /// This read-only register contains the free space information for the device IN endpoint Tx FIFO.
    pub OTG_DTXFSTS2: RORegister<u32>,

    _reserved38: [u32; 1],

    /// The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
    pub OTG_DIEPCTL3: RWRegister<u32>,

    _reserved39: [u32; 1],

    /// This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.
    pub OTG_DIEPINT3: RWRegister<u32>,

    _reserved40: [u32; 1],

    /// The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
    pub OTG_DIEPTSIZ3: RWRegister<u32>,

    /// OTG device IN endpoint 3 DMA address register
    pub OTG_DIEPDMA3: RWRegister<u32>,

    /// This read-only register contains the free space information for the device IN endpoint Tx FIFO.
    pub OTG_DTXFSTS3: RORegister<u32>,

    _reserved41: [u32; 1],

    /// The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
    pub OTG_DIEPCTL4: RWRegister<u32>,

    _reserved42: [u32; 1],

    /// This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.
    pub OTG_DIEPINT4: RWRegister<u32>,

    _reserved43: [u32; 1],

    /// The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
    pub OTG_DIEPTSIZ4: RWRegister<u32>,

    /// OTG device IN endpoint 4 DMA address register
    pub OTG_DIEPDMA4: RWRegister<u32>,

    /// This read-only register contains the free space information for the device IN endpoint Tx FIFO.
    pub OTG_DTXFSTS4: RORegister<u32>,

    _reserved44: [u32; 1],

    /// The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
    pub OTG_DIEPCTL5: RWRegister<u32>,

    _reserved45: [u32; 1],

    /// This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.
    pub OTG_DIEPINT5: RWRegister<u32>,

    _reserved46: [u32; 1],

    /// The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
    pub OTG_DIEPTSIZ5: RWRegister<u32>,

    /// OTG device IN endpoint 5 DMA address register
    pub OTG_DIEPDMA5: RWRegister<u32>,

    /// This read-only register contains the free space information for the device IN endpoint Tx FIFO.
    pub OTG_DTXFSTS5: RORegister<u32>,

    _reserved47: [u32; 1],

    /// The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
    pub OTG_DIEPCTL6: RWRegister<u32>,

    _reserved48: [u32; 1],

    /// This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.
    pub OTG_DIEPINT6: RWRegister<u32>,

    _reserved49: [u32; 1],

    /// The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
    pub OTG_DIEPTSIZ6: RWRegister<u32>,

    /// OTG device IN endpoint 6 DMA address register
    pub OTG_DIEPDMA6: RWRegister<u32>,

    /// This read-only register contains the free space information for the device IN endpoint Tx FIFO.
    pub OTG_DTXFSTS6: RORegister<u32>,

    _reserved50: [u32; 1],

    /// The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
    pub OTG_DIEPCTL7: RWRegister<u32>,

    _reserved51: [u32; 1],

    /// This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.
    pub OTG_DIEPINT7: RWRegister<u32>,

    _reserved52: [u32; 1],

    /// The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
    pub OTG_DIEPTSIZ7: RWRegister<u32>,

    /// OTG device IN endpoint 7 DMA address register
    pub OTG_DIEPDMA7: RWRegister<u32>,

    /// This read-only register contains the free space information for the device IN endpoint Tx FIFO.
    pub OTG_DTXFSTS7: RORegister<u32>,

    _reserved53: [u32; 1],

    /// The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
    pub OTG_DIEPCTL8: RWRegister<u32>,

    _reserved54: [u32; 1],

    /// This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the IN endpoints interrupt bit of the core interrupt register (IEPINT in OTG_GINTSTS) is set. Before the application can read this register, it must first read the device all endpoints interrupt (OTG_DAINT) register to get the exact endpoint number for the device endpoint-x interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.
    pub OTG_DIEPINT8: RWRegister<u32>,

    _reserved55: [u32; 1],

    /// The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the OTG_DIEPCTLx registers (EPENA bit in OTG_DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
    pub OTG_DIEPTSIZ8: RWRegister<u32>,

    /// OTG device IN endpoint 8 DMA address register
    pub OTG_DIEPDMA8: RWRegister<u32>,

    /// This read-only register contains the free space information for the device IN endpoint Tx FIFO.
    pub OTG_DTXFSTS8: RORegister<u32>,

    _reserved56: [u32; 57],

    /// This section describes the OTG_DOEPCTL0 register.
    pub OTG_DOEPCTL0: RWRegister<u32>,

    _reserved57: [u32; 1],

    /// This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.
    pub OTG_DOEPINT0: RWRegister<u32>,

    _reserved58: [u32; 1],

    /// The application must modify this register before enabling endpoint 0.
    pub OTG_DOEPTSIZ0: RWRegister<u32>,

    /// OTG device OUT endpoint 0 DMA address register
    pub OTG_DOEPDMA0: RWRegister<u32>,

    _reserved59: [u32; 2],

    /// The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
    pub OTG_DOEPCTL1: RWRegister<u32>,

    _reserved60: [u32; 1],

    /// This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.
    pub OTG_DOEPINT1: RWRegister<u32>,

    _reserved61: [u32; 1],

    /// The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
    pub OTG_DOEPTSIZ1: RWRegister<u32>,

    /// OTG device OUT endpoint 1 DMA address register
    pub OTG_DOEPDMA1: RWRegister<u32>,

    _reserved62: [u32; 2],

    /// The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
    pub OTG_DOEPCTL2: RWRegister<u32>,

    _reserved63: [u32; 1],

    /// This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.
    pub OTG_DOEPINT2: RWRegister<u32>,

    _reserved64: [u32; 1],

    /// The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
    pub OTG_DOEPTSIZ2: RWRegister<u32>,

    /// OTG device OUT endpoint 2 DMA address register
    pub OTG_DOEPDMA2: RWRegister<u32>,

    _reserved65: [u32; 2],

    /// The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
    pub OTG_DOEPCTL3: RWRegister<u32>,

    _reserved66: [u32; 1],

    /// This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.
    pub OTG_DOEPINT3: RWRegister<u32>,

    _reserved67: [u32; 1],

    /// The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
    pub OTG_DOEPTSIZ3: RWRegister<u32>,

    /// OTG device OUT endpoint 3 DMA address register
    pub OTG_DOEPDMA3: RWRegister<u32>,

    _reserved68: [u32; 2],

    /// The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
    pub OTG_DOEPCTL4: RWRegister<u32>,

    _reserved69: [u32; 1],

    /// This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.
    pub OTG_DOEPINT4: RWRegister<u32>,

    _reserved70: [u32; 1],

    /// The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
    pub OTG_DOEPTSIZ4: RWRegister<u32>,

    /// OTG device OUT endpoint 4 DMA address register
    pub OTG_DOEPDMA4: RWRegister<u32>,

    _reserved71: [u32; 2],

    /// The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
    pub OTG_DOEPCTL5: RWRegister<u32>,

    _reserved72: [u32; 1],

    /// This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.
    pub OTG_DOEPINT5: RWRegister<u32>,

    _reserved73: [u32; 1],

    /// The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
    pub OTG_DOEPTSIZ5: RWRegister<u32>,

    /// OTG device OUT endpoint 5 DMA address register
    pub OTG_DOEPDMA5: RWRegister<u32>,

    _reserved74: [u32; 2],

    /// The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
    pub OTG_DOEPCTL6: RWRegister<u32>,

    _reserved75: [u32; 1],

    /// This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.
    pub OTG_DOEPINT6: RWRegister<u32>,

    _reserved76: [u32; 1],

    /// The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
    pub OTG_DOEPTSIZ6: RWRegister<u32>,

    /// OTG device OUT endpoint 6 DMA address register
    pub OTG_DOEPDMA6: RWRegister<u32>,

    _reserved77: [u32; 2],

    /// The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
    pub OTG_DOEPCTL7: RWRegister<u32>,

    _reserved78: [u32; 1],

    /// This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.
    pub OTG_DOEPINT7: RWRegister<u32>,

    _reserved79: [u32; 1],

    /// The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
    pub OTG_DOEPTSIZ7: RWRegister<u32>,

    /// OTG device OUT endpoint 7 DMA address register
    pub OTG_DOEPDMA7: RWRegister<u32>,

    _reserved80: [u32; 2],

    /// The application uses this register to control the behavior of each logical endpoint other than endpoint 0.
    pub OTG_DOEPCTL8: RWRegister<u32>,

    _reserved81: [u32; 1],

    /// This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in Figure724. The application must read this register when the OUT endpoints interrupt bit of the OTG_GINTSTS register (OEPINT bit in OTG_GINTSTS) is set. Before the application can read this register, it must first read the OTG_DAINT register to get the exact endpoint number for the OTG_DOEPINTx register. The application must clear the appropriate bit in this register to clear the corresponding bits in the OTG_DAINT and OTG_GINTSTS registers.
    pub OTG_DOEPINT8: RWRegister<u32>,

    _reserved82: [u32; 1],

    /// The application must modify this register before enabling the endpoint. Once the endpoint is enabled using endpoint enable bit of the OTG_DOEPCTLx registers (EPENA bit in OTG_DOEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.
    pub OTG_DOEPTSIZ8: RWRegister<u32>,

    /// OTG device OUT endpoint 8 DMA address register
    pub OTG_DOEPDMA8: RWRegister<u32>,

    _reserved83: [u32; 122],

    /// This register is available in host and device modes.
    pub OTG_PCGCCTL: RWRegister<u32>,
}
pub struct ResetValues {
    pub OTG_GOTGCTL: u32,
    pub OTG_GOTGINT: u32,
    pub OTG_GAHBCFG: u32,
    pub OTG_GUSBCFG: u32,
    pub OTG_GRSTCTL: u32,
    pub OTG_GINTSTS: u32,
    pub OTG_GINTMSK: u32,
    pub OTG_GRXSTSR: u32,
    pub OTG_GRXSTSP: u32,
    pub OTG_GRXFSIZ: u32,
    pub OTG_HNPTXFSIZ: u32,
    pub OTG_HNPTXSTS: u32,
    pub OTG_GCCFG: u32,
    pub OTG_CID: u32,
    pub OTG_GLPMCFG: u32,
    pub OTG_HPTXFSIZ: u32,
    pub OTG_DIEPTXF1: u32,
    pub OTG_DIEPTXF2: u32,
    pub OTG_DIEPTXF3: u32,
    pub OTG_DIEPTXF4: u32,
    pub OTG_DIEPTXF5: u32,
    pub OTG_DIEPTXF6: u32,
    pub OTG_DIEPTXF7: u32,
    pub OTG_DIEPTXF8: u32,
    pub OTG_HCFG: u32,
    pub OTG_HFIR: u32,
    pub OTG_HFNUM: u32,
    pub OTG_HPTXSTS: u32,
    pub OTG_HAINT: u32,
    pub OTG_HAINTMSK: u32,
    pub OTG_HFLBADDR: u32,
    pub OTG_HPRT: u32,
    pub OTG_HCCHAR0: u32,
    pub OTG_HCSPLT0: u32,
    pub OTG_HCINT0: u32,
    pub OTG_HCINTMSK0: u32,
    pub OTG_HCTSIZ0: u32,
    pub OTG_HCDMA0: u32,
    pub OTG_HCDMAB0: u32,
    pub OTG_HCCHAR1: u32,
    pub OTG_HCSPLT1: u32,
    pub OTG_HCINT1: u32,
    pub OTG_HCINTMSK1: u32,
    pub OTG_HCTSIZ1: u32,
    pub OTG_HCDMA1: u32,
    pub OTG_HCDMAB1: u32,
    pub OTG_HCCHAR2: u32,
    pub OTG_HCSPLT2: u32,
    pub OTG_HCINT2: u32,
    pub OTG_HCINTMSK2: u32,
    pub OTG_HCTSIZ2: u32,
    pub OTG_HCDMA2: u32,
    pub OTG_HCDMAB2: u32,
    pub OTG_HCCHAR3: u32,
    pub OTG_HCSPLT3: u32,
    pub OTG_HCINT3: u32,
    pub OTG_HCINTMSK3: u32,
    pub OTG_HCTSIZ3: u32,
    pub OTG_HCDMA3: u32,
    pub OTG_HCDMAB3: u32,
    pub OTG_HCCHAR4: u32,
    pub OTG_HCSPLT4: u32,
    pub OTG_HCINT4: u32,
    pub OTG_HCINTMSK4: u32,
    pub OTG_HCTSIZ4: u32,
    pub OTG_HCDMA4: u32,
    pub OTG_HCDMAB4: u32,
    pub OTG_HCCHAR5: u32,
    pub OTG_HCSPLT5: u32,
    pub OTG_HCINT5: u32,
    pub OTG_HCINTMSK5: u32,
    pub OTG_HCTSIZ5: u32,
    pub OTG_HCDMA5: u32,
    pub OTG_HCDMAB5: u32,
    pub OTG_HCCHAR6: u32,
    pub OTG_HCSPLT6: u32,
    pub OTG_HCINT6: u32,
    pub OTG_HCINTMSK6: u32,
    pub OTG_HCTSIZ6: u32,
    pub OTG_HCDMA6: u32,
    pub OTG_HCDMAB6: u32,
    pub OTG_HCCHAR7: u32,
    pub OTG_HCSPLT7: u32,
    pub OTG_HCINT7: u32,
    pub OTG_HCINTMSK7: u32,
    pub OTG_HCTSIZ7: u32,
    pub OTG_HCDMA7: u32,
    pub OTG_HCDMAB7: u32,
    pub OTG_HCCHAR8: u32,
    pub OTG_HCSPLT8: u32,
    pub OTG_HCINT8: u32,
    pub OTG_HCINTMSK8: u32,
    pub OTG_HCTSIZ8: u32,
    pub OTG_HCDMA8: u32,
    pub OTG_HCDMAB8: u32,
    pub OTG_HCCHAR9: u32,
    pub OTG_HCSPLT9: u32,
    pub OTG_HCINT9: u32,
    pub OTG_HCINTMSK9: u32,
    pub OTG_HCTSIZ9: u32,
    pub OTG_HCDMA9: u32,
    pub OTG_HCDMAB9: u32,
    pub OTG_HCCHAR10: u32,
    pub OTG_HCSPLT10: u32,
    pub OTG_HCINT10: u32,
    pub OTG_HCINTMSK10: u32,
    pub OTG_HCTSIZ10: u32,
    pub OTG_HCDMA10: u32,
    pub OTG_HCDMAB10: u32,
    pub OTG_HCCHAR11: u32,
    pub OTG_HCSPLT11: u32,
    pub OTG_HCINT11: u32,
    pub OTG_HCINTMSK11: u32,
    pub OTG_HCTSIZ11: u32,
    pub OTG_HCDMA11: u32,
    pub OTG_HCDMAB11: u32,
    pub OTG_HCCHAR12: u32,
    pub OTG_HCSPLT12: u32,
    pub OTG_HCINT12: u32,
    pub OTG_HCINTMSK12: u32,
    pub OTG_HCTSIZ12: u32,
    pub OTG_HCDMA12: u32,
    pub OTG_HCDMAB12: u32,
    pub OTG_HCCHAR13: u32,
    pub OTG_HCSPLT13: u32,
    pub OTG_HCINT13: u32,
    pub OTG_HCINTMSK13: u32,
    pub OTG_HCTSIZ13: u32,
    pub OTG_HCDMA13: u32,
    pub OTG_HCDMAB13: u32,
    pub OTG_HCCHAR14: u32,
    pub OTG_HCSPLT14: u32,
    pub OTG_HCINT14: u32,
    pub OTG_HCINTMSK14: u32,
    pub OTG_HCTSIZ14: u32,
    pub OTG_HCDMA14: u32,
    pub OTG_HCDMAB14: u32,
    pub OTG_HCCHAR15: u32,
    pub OTG_HCSPLT15: u32,
    pub OTG_HCINT15: u32,
    pub OTG_HCINTMSK15: u32,
    pub OTG_HCTSIZ15: u32,
    pub OTG_HCDMA15: u32,
    pub OTG_HCDMAB15: u32,
    pub OTG_DCFG: u32,
    pub OTG_DCTL: u32,
    pub OTG_DSTS: u32,
    pub OTG_DIEPMSK: u32,
    pub OTG_DOEPMSK: u32,
    pub OTG_DAINT: u32,
    pub OTG_DAINTMSK: u32,
    pub OTG_DVBUSDIS: u32,
    pub OTG_DVBUSPULSE: u32,
    pub OTG_DTHRCTL: u32,
    pub OTG_DIEPEMPMSK: u32,
    pub OTG_DEACHINT: u32,
    pub OTG_DEACHINTMSK: u32,
    pub OTG_HS_DIEPEACHMSK1: u32,
    pub OTG_HS_DOEPEACHMSK1: u32,
    pub OTG_DIEPCTL0: u32,
    pub OTG_DIEPINT0: u32,
    pub OTG_DIEPTSIZ0: u32,
    pub OTG_DIEPDMA0: u32,
    pub OTG_DTXFSTS0: u32,
    pub OTG_DIEPCTL1: u32,
    pub OTG_DIEPINT1: u32,
    pub OTG_DIEPTSIZ1: u32,
    pub OTG_DIEPDMA1: u32,
    pub OTG_DTXFSTS1: u32,
    pub OTG_DIEPCTL2: u32,
    pub OTG_DIEPINT2: u32,
    pub OTG_DIEPTSIZ2: u32,
    pub OTG_DIEPDMA2: u32,
    pub OTG_DTXFSTS2: u32,
    pub OTG_DIEPCTL3: u32,
    pub OTG_DIEPINT3: u32,
    pub OTG_DIEPTSIZ3: u32,
    pub OTG_DIEPDMA3: u32,
    pub OTG_DTXFSTS3: u32,
    pub OTG_DIEPCTL4: u32,
    pub OTG_DIEPINT4: u32,
    pub OTG_DIEPTSIZ4: u32,
    pub OTG_DIEPDMA4: u32,
    pub OTG_DTXFSTS4: u32,
    pub OTG_DIEPCTL5: u32,
    pub OTG_DIEPINT5: u32,
    pub OTG_DIEPTSIZ5: u32,
    pub OTG_DIEPDMA5: u32,
    pub OTG_DTXFSTS5: u32,
    pub OTG_DIEPCTL6: u32,
    pub OTG_DIEPINT6: u32,
    pub OTG_DIEPTSIZ6: u32,
    pub OTG_DIEPDMA6: u32,
    pub OTG_DTXFSTS6: u32,
    pub OTG_DIEPCTL7: u32,
    pub OTG_DIEPINT7: u32,
    pub OTG_DIEPTSIZ7: u32,
    pub OTG_DIEPDMA7: u32,
    pub OTG_DTXFSTS7: u32,
    pub OTG_DIEPCTL8: u32,
    pub OTG_DIEPINT8: u32,
    pub OTG_DIEPTSIZ8: u32,
    pub OTG_DIEPDMA8: u32,
    pub OTG_DTXFSTS8: u32,
    pub OTG_DOEPCTL0: u32,
    pub OTG_DOEPINT0: u32,
    pub OTG_DOEPTSIZ0: u32,
    pub OTG_DOEPDMA0: u32,
    pub OTG_DOEPCTL1: u32,
    pub OTG_DOEPINT1: u32,
    pub OTG_DOEPTSIZ1: u32,
    pub OTG_DOEPDMA1: u32,
    pub OTG_DOEPCTL2: u32,
    pub OTG_DOEPINT2: u32,
    pub OTG_DOEPTSIZ2: u32,
    pub OTG_DOEPDMA2: u32,
    pub OTG_DOEPCTL3: u32,
    pub OTG_DOEPINT3: u32,
    pub OTG_DOEPTSIZ3: u32,
    pub OTG_DOEPDMA3: u32,
    pub OTG_DOEPCTL4: u32,
    pub OTG_DOEPINT4: u32,
    pub OTG_DOEPTSIZ4: u32,
    pub OTG_DOEPDMA4: u32,
    pub OTG_DOEPCTL5: u32,
    pub OTG_DOEPINT5: u32,
    pub OTG_DOEPTSIZ5: u32,
    pub OTG_DOEPDMA5: u32,
    pub OTG_DOEPCTL6: u32,
    pub OTG_DOEPINT6: u32,
    pub OTG_DOEPTSIZ6: u32,
    pub OTG_DOEPDMA6: u32,
    pub OTG_DOEPCTL7: u32,
    pub OTG_DOEPINT7: u32,
    pub OTG_DOEPTSIZ7: u32,
    pub OTG_DOEPDMA7: u32,
    pub OTG_DOEPCTL8: u32,
    pub OTG_DOEPINT8: u32,
    pub OTG_DOEPTSIZ8: u32,
    pub OTG_DOEPDMA8: u32,
    pub OTG_PCGCCTL: u32,
}
#[cfg(not(feature = "nosync"))]
pub struct Instance {
    pub(crate) addr: u32,
    pub(crate) _marker: PhantomData<*const RegisterBlock>,
}
#[cfg(not(feature = "nosync"))]
impl ::core::ops::Deref for Instance {
    type Target = RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &RegisterBlock {
        unsafe { &*(self.addr as *const _) }
    }
}
#[cfg(feature = "rtic")]
unsafe impl Send for Instance {}

/// Access functions for the OTG peripheral instance
pub mod OTG {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x49000000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in OTG
    pub const reset: ResetValues = ResetValues {
        OTG_GOTGCTL: 0x00010000,
        OTG_GOTGINT: 0x00000000,
        OTG_GAHBCFG: 0x00000000,
        OTG_GUSBCFG: 0x00001400,
        OTG_GRSTCTL: 0x80000000,
        OTG_GINTSTS: 0x14000020,
        OTG_GINTMSK: 0x00000000,
        OTG_GRXSTSR: 0x00000000,
        OTG_GRXSTSP: 0x00000000,
        OTG_GRXFSIZ: 0x00000400,
        OTG_HNPTXFSIZ: 0x02000200,
        OTG_HNPTXSTS: 0x00080400,
        OTG_GCCFG: 0x00000000,
        OTG_CID: 0x00004000,
        OTG_GLPMCFG: 0x00000000,
        OTG_HPTXFSIZ: 0x02000400,
        OTG_DIEPTXF1: 0x02000400,
        OTG_DIEPTXF2: 0x02000400,
        OTG_DIEPTXF3: 0x02000400,
        OTG_DIEPTXF4: 0x02000400,
        OTG_DIEPTXF5: 0x02000400,
        OTG_DIEPTXF6: 0x02000400,
        OTG_DIEPTXF7: 0x02000400,
        OTG_DIEPTXF8: 0x02000400,
        OTG_HCFG: 0x00000000,
        OTG_HFIR: 0x0000EA60,
        OTG_HFNUM: 0x00003FFF,
        OTG_HPTXSTS: 0x00080100,
        OTG_HAINT: 0x00000000,
        OTG_HAINTMSK: 0x00000000,
        OTG_HFLBADDR: 0x00000000,
        OTG_HPRT: 0x00000000,
        OTG_HCCHAR0: 0x00000000,
        OTG_HCSPLT0: 0x00000000,
        OTG_HCINT0: 0x00000000,
        OTG_HCINTMSK0: 0x00000000,
        OTG_HCTSIZ0: 0x00000000,
        OTG_HCDMA0: 0x00000000,
        OTG_HCDMAB0: 0x00000000,
        OTG_HCCHAR1: 0x00000000,
        OTG_HCSPLT1: 0x00000000,
        OTG_HCINT1: 0x00000000,
        OTG_HCINTMSK1: 0x00000000,
        OTG_HCTSIZ1: 0x00000000,
        OTG_HCDMA1: 0x00000000,
        OTG_HCDMAB1: 0x00000000,
        OTG_HCCHAR2: 0x00000000,
        OTG_HCSPLT2: 0x00000000,
        OTG_HCINT2: 0x00000000,
        OTG_HCINTMSK2: 0x00000000,
        OTG_HCTSIZ2: 0x00000000,
        OTG_HCDMA2: 0x00000000,
        OTG_HCDMAB2: 0x00000000,
        OTG_HCCHAR3: 0x00000000,
        OTG_HCSPLT3: 0x00000000,
        OTG_HCINT3: 0x00000000,
        OTG_HCINTMSK3: 0x00000000,
        OTG_HCTSIZ3: 0x00000000,
        OTG_HCDMA3: 0x00000000,
        OTG_HCDMAB3: 0x00000000,
        OTG_HCCHAR4: 0x00000000,
        OTG_HCSPLT4: 0x00000000,
        OTG_HCINT4: 0x00000000,
        OTG_HCINTMSK4: 0x00000000,
        OTG_HCTSIZ4: 0x00000000,
        OTG_HCDMA4: 0x00000000,
        OTG_HCDMAB4: 0x00000000,
        OTG_HCCHAR5: 0x00000000,
        OTG_HCSPLT5: 0x00000000,
        OTG_HCINT5: 0x00000000,
        OTG_HCINTMSK5: 0x00000000,
        OTG_HCTSIZ5: 0x00000000,
        OTG_HCDMA5: 0x00000000,
        OTG_HCDMAB5: 0x00000000,
        OTG_HCCHAR6: 0x00000000,
        OTG_HCSPLT6: 0x00000000,
        OTG_HCINT6: 0x00000000,
        OTG_HCINTMSK6: 0x00000000,
        OTG_HCTSIZ6: 0x00000000,
        OTG_HCDMA6: 0x00000000,
        OTG_HCDMAB6: 0x00000000,
        OTG_HCCHAR7: 0x00000000,
        OTG_HCSPLT7: 0x00000000,
        OTG_HCINT7: 0x00000000,
        OTG_HCINTMSK7: 0x00000000,
        OTG_HCTSIZ7: 0x00000000,
        OTG_HCDMA7: 0x00000000,
        OTG_HCDMAB7: 0x00000000,
        OTG_HCCHAR8: 0x00000000,
        OTG_HCSPLT8: 0x00000000,
        OTG_HCINT8: 0x00000000,
        OTG_HCINTMSK8: 0x00000000,
        OTG_HCTSIZ8: 0x00000000,
        OTG_HCDMA8: 0x00000000,
        OTG_HCDMAB8: 0x00000000,
        OTG_HCCHAR9: 0x00000000,
        OTG_HCSPLT9: 0x00000000,
        OTG_HCINT9: 0x00000000,
        OTG_HCINTMSK9: 0x00000000,
        OTG_HCTSIZ9: 0x00000000,
        OTG_HCDMA9: 0x00000000,
        OTG_HCDMAB9: 0x00000000,
        OTG_HCCHAR10: 0x00000000,
        OTG_HCSPLT10: 0x00000000,
        OTG_HCINT10: 0x00000000,
        OTG_HCINTMSK10: 0x00000000,
        OTG_HCTSIZ10: 0x00000000,
        OTG_HCDMA10: 0x00000000,
        OTG_HCDMAB10: 0x00000000,
        OTG_HCCHAR11: 0x00000000,
        OTG_HCSPLT11: 0x00000000,
        OTG_HCINT11: 0x00000000,
        OTG_HCINTMSK11: 0x00000000,
        OTG_HCTSIZ11: 0x00000000,
        OTG_HCDMA11: 0x00000000,
        OTG_HCDMAB11: 0x00000000,
        OTG_HCCHAR12: 0x00000000,
        OTG_HCSPLT12: 0x00000000,
        OTG_HCINT12: 0x00000000,
        OTG_HCINTMSK12: 0x00000000,
        OTG_HCTSIZ12: 0x00000000,
        OTG_HCDMA12: 0x00000000,
        OTG_HCDMAB12: 0x00000000,
        OTG_HCCHAR13: 0x00000000,
        OTG_HCSPLT13: 0x00000000,
        OTG_HCINT13: 0x00000000,
        OTG_HCINTMSK13: 0x00000000,
        OTG_HCTSIZ13: 0x00000000,
        OTG_HCDMA13: 0x00000000,
        OTG_HCDMAB13: 0x00000000,
        OTG_HCCHAR14: 0x00000000,
        OTG_HCSPLT14: 0x00000000,
        OTG_HCINT14: 0x00000000,
        OTG_HCINTMSK14: 0x00000000,
        OTG_HCTSIZ14: 0x00000000,
        OTG_HCDMA14: 0x00000000,
        OTG_HCDMAB14: 0x00000000,
        OTG_HCCHAR15: 0x00000000,
        OTG_HCSPLT15: 0x00000000,
        OTG_HCINT15: 0x00000000,
        OTG_HCINTMSK15: 0x00000000,
        OTG_HCTSIZ15: 0x00000000,
        OTG_HCDMA15: 0x00000000,
        OTG_HCDMAB15: 0x00000000,
        OTG_DCFG: 0x02200000,
        OTG_DCTL: 0x00000002,
        OTG_DSTS: 0x00000010,
        OTG_DIEPMSK: 0x00000000,
        OTG_DOEPMSK: 0x00000000,
        OTG_DAINT: 0x00000000,
        OTG_DAINTMSK: 0x00000000,
        OTG_DVBUSDIS: 0x000017D7,
        OTG_DVBUSPULSE: 0x000005B8,
        OTG_DTHRCTL: 0x00000000,
        OTG_DIEPEMPMSK: 0x00000000,
        OTG_DEACHINT: 0x00000000,
        OTG_DEACHINTMSK: 0x00000000,
        OTG_HS_DIEPEACHMSK1: 0x00000000,
        OTG_HS_DOEPEACHMSK1: 0x00000000,
        OTG_DIEPCTL0: 0x00000000,
        OTG_DIEPINT0: 0x00000080,
        OTG_DIEPTSIZ0: 0x00000000,
        OTG_DIEPDMA0: 0x00000000,
        OTG_DTXFSTS0: 0x00000200,
        OTG_DIEPCTL1: 0x00000000,
        OTG_DIEPINT1: 0x00000080,
        OTG_DIEPTSIZ1: 0x00000000,
        OTG_DIEPDMA1: 0x00000000,
        OTG_DTXFSTS1: 0x00000200,
        OTG_DIEPCTL2: 0x00000000,
        OTG_DIEPINT2: 0x00000080,
        OTG_DIEPTSIZ2: 0x00000000,
        OTG_DIEPDMA2: 0x00000000,
        OTG_DTXFSTS2: 0x00000200,
        OTG_DIEPCTL3: 0x00000000,
        OTG_DIEPINT3: 0x00000080,
        OTG_DIEPTSIZ3: 0x00000000,
        OTG_DIEPDMA3: 0x00000000,
        OTG_DTXFSTS3: 0x00000200,
        OTG_DIEPCTL4: 0x00000000,
        OTG_DIEPINT4: 0x00000080,
        OTG_DIEPTSIZ4: 0x00000000,
        OTG_DIEPDMA4: 0x00000000,
        OTG_DTXFSTS4: 0x00000200,
        OTG_DIEPCTL5: 0x00000000,
        OTG_DIEPINT5: 0x00000080,
        OTG_DIEPTSIZ5: 0x00000000,
        OTG_DIEPDMA5: 0x00000000,
        OTG_DTXFSTS5: 0x00000200,
        OTG_DIEPCTL6: 0x00000000,
        OTG_DIEPINT6: 0x00000080,
        OTG_DIEPTSIZ6: 0x00000000,
        OTG_DIEPDMA6: 0x00000000,
        OTG_DTXFSTS6: 0x00000200,
        OTG_DIEPCTL7: 0x00000000,
        OTG_DIEPINT7: 0x00000080,
        OTG_DIEPTSIZ7: 0x00000000,
        OTG_DIEPDMA7: 0x00000000,
        OTG_DTXFSTS7: 0x00000200,
        OTG_DIEPCTL8: 0x00000000,
        OTG_DIEPINT8: 0x00000080,
        OTG_DIEPTSIZ8: 0x00000000,
        OTG_DIEPDMA8: 0x00000000,
        OTG_DTXFSTS8: 0x00000200,
        OTG_DOEPCTL0: 0x00008000,
        OTG_DOEPINT0: 0x00000080,
        OTG_DOEPTSIZ0: 0x00000000,
        OTG_DOEPDMA0: 0x00000000,
        OTG_DOEPCTL1: 0x00000000,
        OTG_DOEPINT1: 0x00000080,
        OTG_DOEPTSIZ1: 0x00000000,
        OTG_DOEPDMA1: 0x00000000,
        OTG_DOEPCTL2: 0x00000000,
        OTG_DOEPINT2: 0x00000080,
        OTG_DOEPTSIZ2: 0x00000000,
        OTG_DOEPDMA2: 0x00000000,
        OTG_DOEPCTL3: 0x00000000,
        OTG_DOEPINT3: 0x00000080,
        OTG_DOEPTSIZ3: 0x00000000,
        OTG_DOEPDMA3: 0x00000000,
        OTG_DOEPCTL4: 0x00000000,
        OTG_DOEPINT4: 0x00000080,
        OTG_DOEPTSIZ4: 0x00000000,
        OTG_DOEPDMA4: 0x00000000,
        OTG_DOEPCTL5: 0x00000000,
        OTG_DOEPINT5: 0x00000080,
        OTG_DOEPTSIZ5: 0x00000000,
        OTG_DOEPDMA5: 0x00000000,
        OTG_DOEPCTL6: 0x00000000,
        OTG_DOEPINT6: 0x00000080,
        OTG_DOEPTSIZ6: 0x00000000,
        OTG_DOEPDMA6: 0x00000000,
        OTG_DOEPCTL7: 0x00000000,
        OTG_DOEPINT7: 0x00000080,
        OTG_DOEPTSIZ7: 0x00000000,
        OTG_DOEPDMA7: 0x00000000,
        OTG_DOEPCTL8: 0x00000000,
        OTG_DOEPINT8: 0x00000080,
        OTG_DOEPTSIZ8: 0x00000000,
        OTG_DOEPDMA8: 0x00000000,
        OTG_PCGCCTL: 0x200B8000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut OTG_TAKEN: bool = false;

    /// Safe access to OTG
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
            if OTG_TAKEN {
                None
            } else {
                OTG_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to OTG
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if OTG_TAKEN && inst.addr == INSTANCE.addr {
                OTG_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal OTG
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        OTG_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to OTG
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const OTG: *const RegisterBlock = 0x49000000 as *const _;
