#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! USB on the go full speed
//!
//! Used by: stm32f730, stm32f7x2, stm32f7x3

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// OTG_FS device configuration register (OTG_FS_DCFG)
pub mod OTG_FS_DCFG {

    /// Device speed
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

    /// Non-zero-length status OUT handshake
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

    /// Device address
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

    /// Periodic frame interval
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
}

/// OTG_FS device control register (OTG_FS_DCTL)
pub mod OTG_FS_DCTL {

    /// Remote wakeup signaling
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

    /// Soft disconnect
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

    /// Global IN NAK status
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

    /// Global OUT NAK status
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

    /// Test control
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

    /// Set global IN NAK
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

    /// Clear global IN NAK
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

    /// Set global OUT NAK
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

    /// Clear global OUT NAK
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

    /// Power-on programming done
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
}

/// OTG_FS device status register (OTG_FS_DSTS)
pub mod OTG_FS_DSTS {

    /// Suspend status
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

    /// Enumerated speed
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

    /// Erratic error
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

    /// Frame number of the received SOF
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
}

/// OTG_FS device IN endpoint common interrupt mask register (OTG_FS_DIEPMSK)
pub mod OTG_FS_DIEPMSK {

    /// Transfer completed interrupt mask
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

    /// Endpoint disabled interrupt mask
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

    /// Timeout condition mask (Non-isochronous endpoints)
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

    /// IN token received when TxFIFO empty mask
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

    /// IN token received with EP mismatch mask
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

    /// IN endpoint NAK effective mask
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
}

/// OTG_FS device OUT endpoint common interrupt mask register (OTG_FS_DOEPMSK)
pub mod OTG_FS_DOEPMSK {

    /// Transfer completed interrupt mask
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

    /// Endpoint disabled interrupt mask
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

    /// SETUP phase done mask
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

    /// OUT token received when endpoint disabled mask
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
}

/// OTG_FS device all endpoints interrupt register (OTG_FS_DAINT)
pub mod OTG_FS_DAINT {

    /// IN endpoint interrupt bits
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

    /// OUT endpoint interrupt bits
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

/// OTG_FS all endpoints interrupt mask register (OTG_FS_DAINTMSK)
pub mod OTG_FS_DAINTMSK {

    /// IN EP interrupt mask bits
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

    /// OUT endpoint interrupt bits
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

/// OTG_FS device VBUS discharge time register
pub mod OTG_FS_DVBUSDIS {

    /// Device VBUS discharge time
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

/// OTG_FS device VBUS pulsing time register
pub mod OTG_FS_DVBUSPULSE {

    /// Device VBUS pulsing time
    pub mod DVBUSP {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (12 bits: 0xfff << 0)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// OTG_FS device IN endpoint FIFO empty interrupt mask register
pub mod OTG_FS_DIEPEMPMSK {

    /// IN EP Tx FIFO empty interrupt mask bits
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

/// OTG_FS device control IN endpoint 0 control register (OTG_FS_DIEPCTL0)
pub mod OTG_FS_DIEPCTL0 {

    /// Maximum packet size
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

    /// USB active endpoint
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

    /// NAK status
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

    /// Endpoint type
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

    /// STALL handshake
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

    /// TxFIFO number
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

    /// Clear NAK
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

    /// Set NAK
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

    /// Endpoint disable
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

    /// Endpoint enable
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

/// OTG device endpoint-1 control register
pub mod OTG_FS_DIEPCTL1 {

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

    /// SODDFRM/SD1PID
    pub mod SODDFRM_SD1PID {
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

    /// SD0PID/SEVNFRM
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

    /// Stall
    pub mod Stall {
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

    /// EONUM/DPID
    pub mod EONUM_DPID {
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
}

/// OTG device endpoint-2 control register
pub mod OTG_FS_DIEPCTL2 {

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

    /// SD0PID/SEVNFRM
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

    /// Stall
    pub mod Stall {
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

    /// EONUM/DPID
    pub mod EONUM_DPID {
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
}

/// OTG device endpoint-3 control register
pub mod OTG_FS_DIEPCTL3 {
    pub use super::OTG_FS_DIEPCTL2::Stall;
    pub use super::OTG_FS_DIEPCTL2::CNAK;
    pub use super::OTG_FS_DIEPCTL2::EONUM_DPID;
    pub use super::OTG_FS_DIEPCTL2::EPDIS;
    pub use super::OTG_FS_DIEPCTL2::EPENA;
    pub use super::OTG_FS_DIEPCTL2::EPTYP;
    pub use super::OTG_FS_DIEPCTL2::MPSIZ;
    pub use super::OTG_FS_DIEPCTL2::NAKSTS;
    pub use super::OTG_FS_DIEPCTL2::SD0PID_SEVNFRM;
    pub use super::OTG_FS_DIEPCTL2::SNAK;
    pub use super::OTG_FS_DIEPCTL2::SODDFRM;
    pub use super::OTG_FS_DIEPCTL2::TXFNUM;
    pub use super::OTG_FS_DIEPCTL2::USBAEP;
}

/// device endpoint-0 control register
pub mod OTG_FS_DOEPCTL0 {

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

    /// Stall
    pub mod Stall {
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
}

/// device endpoint-1 control register
pub mod OTG_FS_DOEPCTL1 {

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

    /// SD0PID/SEVNFRM
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

    /// Stall
    pub mod Stall {
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

    /// EONUM/DPID
    pub mod EONUM_DPID {
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
}

/// device endpoint-2 control register
pub mod OTG_FS_DOEPCTL2 {
    pub use super::OTG_FS_DOEPCTL1::Stall;
    pub use super::OTG_FS_DOEPCTL1::CNAK;
    pub use super::OTG_FS_DOEPCTL1::EONUM_DPID;
    pub use super::OTG_FS_DOEPCTL1::EPDIS;
    pub use super::OTG_FS_DOEPCTL1::EPENA;
    pub use super::OTG_FS_DOEPCTL1::EPTYP;
    pub use super::OTG_FS_DOEPCTL1::MPSIZ;
    pub use super::OTG_FS_DOEPCTL1::NAKSTS;
    pub use super::OTG_FS_DOEPCTL1::SD0PID_SEVNFRM;
    pub use super::OTG_FS_DOEPCTL1::SNAK;
    pub use super::OTG_FS_DOEPCTL1::SNPM;
    pub use super::OTG_FS_DOEPCTL1::SODDFRM;
    pub use super::OTG_FS_DOEPCTL1::USBAEP;
}

/// device endpoint-3 control register
pub mod OTG_FS_DOEPCTL3 {
    pub use super::OTG_FS_DOEPCTL1::Stall;
    pub use super::OTG_FS_DOEPCTL1::CNAK;
    pub use super::OTG_FS_DOEPCTL1::EONUM_DPID;
    pub use super::OTG_FS_DOEPCTL1::EPDIS;
    pub use super::OTG_FS_DOEPCTL1::EPENA;
    pub use super::OTG_FS_DOEPCTL1::EPTYP;
    pub use super::OTG_FS_DOEPCTL1::MPSIZ;
    pub use super::OTG_FS_DOEPCTL1::NAKSTS;
    pub use super::OTG_FS_DOEPCTL1::SD0PID_SEVNFRM;
    pub use super::OTG_FS_DOEPCTL1::SNAK;
    pub use super::OTG_FS_DOEPCTL1::SNPM;
    pub use super::OTG_FS_DOEPCTL1::SODDFRM;
    pub use super::OTG_FS_DOEPCTL1::USBAEP;
}

/// device endpoint-x interrupt register
pub mod OTG_FS_DIEPINT0 {

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
}

/// device endpoint-1 interrupt register
pub mod OTG_FS_DIEPINT1 {
    pub use super::OTG_FS_DIEPINT0::EPDISD;
    pub use super::OTG_FS_DIEPINT0::INEPNE;
    pub use super::OTG_FS_DIEPINT0::ITTXFE;
    pub use super::OTG_FS_DIEPINT0::TOC;
    pub use super::OTG_FS_DIEPINT0::TXFE;
    pub use super::OTG_FS_DIEPINT0::XFRC;
}

/// device endpoint-2 interrupt register
pub mod OTG_FS_DIEPINT2 {
    pub use super::OTG_FS_DIEPINT0::EPDISD;
    pub use super::OTG_FS_DIEPINT0::INEPNE;
    pub use super::OTG_FS_DIEPINT0::ITTXFE;
    pub use super::OTG_FS_DIEPINT0::TOC;
    pub use super::OTG_FS_DIEPINT0::TXFE;
    pub use super::OTG_FS_DIEPINT0::XFRC;
}

/// device endpoint-3 interrupt register
pub mod OTG_FS_DIEPINT3 {
    pub use super::OTG_FS_DIEPINT0::EPDISD;
    pub use super::OTG_FS_DIEPINT0::INEPNE;
    pub use super::OTG_FS_DIEPINT0::ITTXFE;
    pub use super::OTG_FS_DIEPINT0::TOC;
    pub use super::OTG_FS_DIEPINT0::TXFE;
    pub use super::OTG_FS_DIEPINT0::XFRC;
}

/// device endpoint-0 interrupt register
pub mod OTG_FS_DOEPINT0 {

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
}

/// device endpoint-1 interrupt register
pub mod OTG_FS_DOEPINT1 {
    pub use super::OTG_FS_DOEPINT0::B2BSTUP;
    pub use super::OTG_FS_DOEPINT0::EPDISD;
    pub use super::OTG_FS_DOEPINT0::OTEPDIS;
    pub use super::OTG_FS_DOEPINT0::STUP;
    pub use super::OTG_FS_DOEPINT0::XFRC;
}

/// device endpoint-2 interrupt register
pub mod OTG_FS_DOEPINT2 {
    pub use super::OTG_FS_DOEPINT0::B2BSTUP;
    pub use super::OTG_FS_DOEPINT0::EPDISD;
    pub use super::OTG_FS_DOEPINT0::OTEPDIS;
    pub use super::OTG_FS_DOEPINT0::STUP;
    pub use super::OTG_FS_DOEPINT0::XFRC;
}

/// device endpoint-3 interrupt register
pub mod OTG_FS_DOEPINT3 {
    pub use super::OTG_FS_DOEPINT0::B2BSTUP;
    pub use super::OTG_FS_DOEPINT0::EPDISD;
    pub use super::OTG_FS_DOEPINT0::OTEPDIS;
    pub use super::OTG_FS_DOEPINT0::STUP;
    pub use super::OTG_FS_DOEPINT0::XFRC;
}

/// device endpoint-0 transfer size register
pub mod OTG_FS_DIEPTSIZ0 {

    /// Packet count
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

    /// Transfer size
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
}

/// device OUT endpoint-0 transfer size register
pub mod OTG_FS_DOEPTSIZ0 {

    /// SETUP packet count
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

    /// Packet count
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

    /// Transfer size
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
}

/// device endpoint-1 transfer size register
pub mod OTG_FS_DIEPTSIZ1 {

    /// Multi count
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

    /// Packet count
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

    /// Transfer size
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
}

/// device endpoint-2 transfer size register
pub mod OTG_FS_DIEPTSIZ2 {
    pub use super::OTG_FS_DIEPTSIZ1::MCNT;
    pub use super::OTG_FS_DIEPTSIZ1::PKTCNT;
    pub use super::OTG_FS_DIEPTSIZ1::XFRSIZ;
}

/// device endpoint-3 transfer size register
pub mod OTG_FS_DIEPTSIZ3 {
    pub use super::OTG_FS_DIEPTSIZ1::MCNT;
    pub use super::OTG_FS_DIEPTSIZ1::PKTCNT;
    pub use super::OTG_FS_DIEPTSIZ1::XFRSIZ;
}

/// OTG_FS device IN endpoint transmit FIFO status register
pub mod OTG_FS_DTXFSTS0 {

    /// IN endpoint TxFIFO space available
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

/// OTG_FS device IN endpoint transmit FIFO status register
pub mod OTG_FS_DTXFSTS1 {
    pub use super::OTG_FS_DTXFSTS0::INEPTFSAV;
}

/// OTG_FS device IN endpoint transmit FIFO status register
pub mod OTG_FS_DTXFSTS2 {
    pub use super::OTG_FS_DTXFSTS0::INEPTFSAV;
}

/// OTG_FS device IN endpoint transmit FIFO status register
pub mod OTG_FS_DTXFSTS3 {
    pub use super::OTG_FS_DTXFSTS0::INEPTFSAV;
}

/// device OUT endpoint-1 transfer size register
pub mod OTG_FS_DOEPTSIZ1 {

    /// Received data PID/SETUP packet count
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

    /// Packet count
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

    /// Transfer size
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
}

/// device OUT endpoint-2 transfer size register
pub mod OTG_FS_DOEPTSIZ2 {
    pub use super::OTG_FS_DOEPTSIZ1::PKTCNT;
    pub use super::OTG_FS_DOEPTSIZ1::RXDPID_STUPCNT;
    pub use super::OTG_FS_DOEPTSIZ1::XFRSIZ;
}

/// device OUT endpoint-3 transfer size register
pub mod OTG_FS_DOEPTSIZ3 {
    pub use super::OTG_FS_DOEPTSIZ1::PKTCNT;
    pub use super::OTG_FS_DOEPTSIZ1::RXDPID_STUPCNT;
    pub use super::OTG_FS_DOEPTSIZ1::XFRSIZ;
}

/// OTG device endpoint-4 control register
pub mod OTG_FS_DIEPCTL4 {
    pub use super::OTG_FS_DIEPCTL2::Stall;
    pub use super::OTG_FS_DIEPCTL2::CNAK;
    pub use super::OTG_FS_DIEPCTL2::EONUM_DPID;
    pub use super::OTG_FS_DIEPCTL2::EPDIS;
    pub use super::OTG_FS_DIEPCTL2::EPENA;
    pub use super::OTG_FS_DIEPCTL2::EPTYP;
    pub use super::OTG_FS_DIEPCTL2::MPSIZ;
    pub use super::OTG_FS_DIEPCTL2::NAKSTS;
    pub use super::OTG_FS_DIEPCTL2::SD0PID_SEVNFRM;
    pub use super::OTG_FS_DIEPCTL2::SNAK;
    pub use super::OTG_FS_DIEPCTL2::SODDFRM;
    pub use super::OTG_FS_DIEPCTL2::TXFNUM;
    pub use super::OTG_FS_DIEPCTL2::USBAEP;
}

/// device endpoint-4 interrupt register
pub mod OTG_FS_DIEPINT4 {
    pub use super::OTG_FS_DIEPINT0::EPDISD;
    pub use super::OTG_FS_DIEPINT0::INEPNE;
    pub use super::OTG_FS_DIEPINT0::ITTXFE;
    pub use super::OTG_FS_DIEPINT0::TOC;
    pub use super::OTG_FS_DIEPINT0::TXFE;
    pub use super::OTG_FS_DIEPINT0::XFRC;
}

/// device endpoint-4 transfer size register
pub mod OTG_FS_DIEPTSIZ4 {
    pub use super::OTG_FS_DIEPTSIZ1::MCNT;
    pub use super::OTG_FS_DIEPTSIZ1::PKTCNT;
    pub use super::OTG_FS_DIEPTSIZ1::XFRSIZ;
}

/// OTG_FS device IN endpoint transmit FIFO status register
pub mod OTG_FS_DTXFSTS4 {

    /// IN endpoint TxFIFO space available
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

/// OTG device endpoint-5 control register
pub mod OTG_FS_DIEPCTL5 {
    pub use super::OTG_FS_DIEPCTL2::Stall;
    pub use super::OTG_FS_DIEPCTL2::CNAK;
    pub use super::OTG_FS_DIEPCTL2::EONUM_DPID;
    pub use super::OTG_FS_DIEPCTL2::EPDIS;
    pub use super::OTG_FS_DIEPCTL2::EPENA;
    pub use super::OTG_FS_DIEPCTL2::EPTYP;
    pub use super::OTG_FS_DIEPCTL2::MPSIZ;
    pub use super::OTG_FS_DIEPCTL2::NAKSTS;
    pub use super::OTG_FS_DIEPCTL2::SD0PID_SEVNFRM;
    pub use super::OTG_FS_DIEPCTL2::SNAK;
    pub use super::OTG_FS_DIEPCTL2::SODDFRM;
    pub use super::OTG_FS_DIEPCTL2::TXFNUM;
    pub use super::OTG_FS_DIEPCTL2::USBAEP;
}

/// device endpoint-5 interrupt register
pub mod OTG_FS_DIEPINT5 {
    pub use super::OTG_FS_DIEPINT0::EPDISD;
    pub use super::OTG_FS_DIEPINT0::INEPNE;
    pub use super::OTG_FS_DIEPINT0::ITTXFE;
    pub use super::OTG_FS_DIEPINT0::TOC;
    pub use super::OTG_FS_DIEPINT0::TXFE;
    pub use super::OTG_FS_DIEPINT0::XFRC;
}

/// device endpoint-5 transfer size register
pub mod OTG_FS_DIEPTSIZ5 {
    pub use super::OTG_FS_DIEPTSIZ1::MCNT;
    pub use super::OTG_FS_DIEPTSIZ1::PKTCNT;
    pub use super::OTG_FS_DIEPTSIZ1::XFRSIZ;
}

/// OTG_FS device IN endpoint transmit FIFO status register
pub mod OTG_FS_DTXFSTS5 {
    pub use super::OTG_FS_DTXFSTS4::INEPTFSAV;
}

/// device endpoint-4 control register
pub mod OTG_FS_DOEPCTL4 {
    pub use super::OTG_FS_DOEPCTL1::Stall;
    pub use super::OTG_FS_DOEPCTL1::CNAK;
    pub use super::OTG_FS_DOEPCTL1::EONUM_DPID;
    pub use super::OTG_FS_DOEPCTL1::EPDIS;
    pub use super::OTG_FS_DOEPCTL1::EPENA;
    pub use super::OTG_FS_DOEPCTL1::EPTYP;
    pub use super::OTG_FS_DOEPCTL1::MPSIZ;
    pub use super::OTG_FS_DOEPCTL1::NAKSTS;
    pub use super::OTG_FS_DOEPCTL1::SD0PID_SEVNFRM;
    pub use super::OTG_FS_DOEPCTL1::SNAK;
    pub use super::OTG_FS_DOEPCTL1::SNPM;
    pub use super::OTG_FS_DOEPCTL1::SODDFRM;
    pub use super::OTG_FS_DOEPCTL1::USBAEP;
}

/// device endpoint-4 interrupt register
pub mod OTG_FS_DOEPINT4 {
    pub use super::OTG_FS_DOEPINT0::B2BSTUP;
    pub use super::OTG_FS_DOEPINT0::EPDISD;
    pub use super::OTG_FS_DOEPINT0::OTEPDIS;
    pub use super::OTG_FS_DOEPINT0::STUP;
    pub use super::OTG_FS_DOEPINT0::XFRC;
}

/// device OUT endpoint-4 transfer size register
pub mod OTG_FS_DOEPTSIZ4 {
    pub use super::OTG_FS_DOEPTSIZ1::PKTCNT;
    pub use super::OTG_FS_DOEPTSIZ1::RXDPID_STUPCNT;
    pub use super::OTG_FS_DOEPTSIZ1::XFRSIZ;
}

/// device endpoint-5 control register
pub mod OTG_FS_DOEPCTL5 {
    pub use super::OTG_FS_DOEPCTL1::Stall;
    pub use super::OTG_FS_DOEPCTL1::CNAK;
    pub use super::OTG_FS_DOEPCTL1::EONUM_DPID;
    pub use super::OTG_FS_DOEPCTL1::EPDIS;
    pub use super::OTG_FS_DOEPCTL1::EPENA;
    pub use super::OTG_FS_DOEPCTL1::EPTYP;
    pub use super::OTG_FS_DOEPCTL1::MPSIZ;
    pub use super::OTG_FS_DOEPCTL1::NAKSTS;
    pub use super::OTG_FS_DOEPCTL1::SD0PID_SEVNFRM;
    pub use super::OTG_FS_DOEPCTL1::SNAK;
    pub use super::OTG_FS_DOEPCTL1::SNPM;
    pub use super::OTG_FS_DOEPCTL1::SODDFRM;
    pub use super::OTG_FS_DOEPCTL1::USBAEP;
}

/// device endpoint-5 interrupt register
pub mod OTG_FS_DOEPINT5 {
    pub use super::OTG_FS_DOEPINT0::B2BSTUP;
    pub use super::OTG_FS_DOEPINT0::EPDISD;
    pub use super::OTG_FS_DOEPINT0::OTEPDIS;
    pub use super::OTG_FS_DOEPINT0::STUP;
    pub use super::OTG_FS_DOEPINT0::XFRC;
}

/// device OUT endpoint-5 transfer size register
pub mod OTG_FS_DOEPTSIZ5 {
    pub use super::OTG_FS_DOEPTSIZ1::PKTCNT;
    pub use super::OTG_FS_DOEPTSIZ1::RXDPID_STUPCNT;
    pub use super::OTG_FS_DOEPTSIZ1::XFRSIZ;
}
pub struct RegisterBlock {
    /// OTG_FS device configuration register (OTG_FS_DCFG)
    pub OTG_FS_DCFG: RWRegister<u32>,

    /// OTG_FS device control register (OTG_FS_DCTL)
    pub OTG_FS_DCTL: RWRegister<u32>,

    /// OTG_FS device status register (OTG_FS_DSTS)
    pub OTG_FS_DSTS: RORegister<u32>,

    _reserved1: [u32; 1],

    /// OTG_FS device IN endpoint common interrupt mask register (OTG_FS_DIEPMSK)
    pub OTG_FS_DIEPMSK: RWRegister<u32>,

    /// OTG_FS device OUT endpoint common interrupt mask register (OTG_FS_DOEPMSK)
    pub OTG_FS_DOEPMSK: RWRegister<u32>,

    /// OTG_FS device all endpoints interrupt register (OTG_FS_DAINT)
    pub OTG_FS_DAINT: RORegister<u32>,

    /// OTG_FS all endpoints interrupt mask register (OTG_FS_DAINTMSK)
    pub OTG_FS_DAINTMSK: RWRegister<u32>,

    _reserved2: [u32; 2],

    /// OTG_FS device VBUS discharge time register
    pub OTG_FS_DVBUSDIS: RWRegister<u32>,

    /// OTG_FS device VBUS pulsing time register
    pub OTG_FS_DVBUSPULSE: RWRegister<u32>,

    _reserved3: [u32; 1],

    /// OTG_FS device IN endpoint FIFO empty interrupt mask register
    pub OTG_FS_DIEPEMPMSK: RWRegister<u32>,

    _reserved4: [u32; 50],

    /// OTG_FS device control IN endpoint 0 control register (OTG_FS_DIEPCTL0)
    pub OTG_FS_DIEPCTL0: RWRegister<u32>,

    _reserved5: [u32; 1],

    /// device endpoint-x interrupt register
    pub OTG_FS_DIEPINT0: RWRegister<u32>,

    _reserved6: [u32; 1],

    /// device endpoint-0 transfer size register
    pub OTG_FS_DIEPTSIZ0: RWRegister<u32>,

    _reserved7: [u32; 1],

    /// OTG_FS device IN endpoint transmit FIFO status register
    pub OTG_FS_DTXFSTS0: RORegister<u32>,

    _reserved8: [u32; 1],

    /// OTG device endpoint-1 control register
    pub OTG_FS_DIEPCTL1: RWRegister<u32>,

    _reserved9: [u32; 1],

    /// device endpoint-1 interrupt register
    pub OTG_FS_DIEPINT1: RWRegister<u32>,

    _reserved10: [u32; 1],

    /// device endpoint-1 transfer size register
    pub OTG_FS_DIEPTSIZ1: RWRegister<u32>,

    _reserved11: [u32; 1],

    /// OTG_FS device IN endpoint transmit FIFO status register
    pub OTG_FS_DTXFSTS1: RORegister<u32>,

    _reserved12: [u32; 1],

    /// OTG device endpoint-2 control register
    pub OTG_FS_DIEPCTL2: RWRegister<u32>,

    _reserved13: [u32; 1],

    /// device endpoint-2 interrupt register
    pub OTG_FS_DIEPINT2: RWRegister<u32>,

    _reserved14: [u32; 1],

    /// device endpoint-2 transfer size register
    pub OTG_FS_DIEPTSIZ2: RWRegister<u32>,

    _reserved15: [u32; 1],

    /// OTG_FS device IN endpoint transmit FIFO status register
    pub OTG_FS_DTXFSTS2: RORegister<u32>,

    _reserved16: [u32; 1],

    /// OTG device endpoint-3 control register
    pub OTG_FS_DIEPCTL3: RWRegister<u32>,

    _reserved17: [u32; 1],

    /// device endpoint-3 interrupt register
    pub OTG_FS_DIEPINT3: RWRegister<u32>,

    _reserved18: [u32; 1],

    /// device endpoint-3 transfer size register
    pub OTG_FS_DIEPTSIZ3: RWRegister<u32>,

    _reserved19: [u32; 1],

    /// OTG_FS device IN endpoint transmit FIFO status register
    pub OTG_FS_DTXFSTS3: RORegister<u32>,

    _reserved20: [u32; 1],

    /// OTG device endpoint-4 control register
    pub OTG_FS_DIEPCTL4: RWRegister<u32>,

    _reserved21: [u32; 1],

    /// device endpoint-4 interrupt register
    pub OTG_FS_DIEPINT4: RWRegister<u32>,

    _reserved22: [u32; 1],

    /// device endpoint-4 transfer size register
    pub OTG_FS_DIEPTSIZ4: RWRegister<u32>,

    _reserved23: [u32; 1],

    /// OTG_FS device IN endpoint transmit FIFO status register
    pub OTG_FS_DTXFSTS4: RWRegister<u32>,

    _reserved24: [u32; 1],

    /// OTG device endpoint-5 control register
    pub OTG_FS_DIEPCTL5: RWRegister<u32>,

    _reserved25: [u32; 1],

    /// device endpoint-5 interrupt register
    pub OTG_FS_DIEPINT5: RWRegister<u32>,

    _reserved26: [u32; 1],

    /// device endpoint-5 transfer size register
    pub OTG_FS_DIEPTSIZ5: RWRegister<u32>,

    _reserved27: [u32; 1],

    /// OTG_FS device IN endpoint transmit FIFO status register
    pub OTG_FS_DTXFSTS5: RWRegister<u32>,

    _reserved28: [u32; 81],

    /// device endpoint-0 control register
    pub OTG_FS_DOEPCTL0: RWRegister<u32>,

    _reserved29: [u32; 1],

    /// device endpoint-0 interrupt register
    pub OTG_FS_DOEPINT0: RWRegister<u32>,

    _reserved30: [u32; 1],

    /// device OUT endpoint-0 transfer size register
    pub OTG_FS_DOEPTSIZ0: RWRegister<u32>,

    _reserved31: [u32; 3],

    /// device endpoint-1 control register
    pub OTG_FS_DOEPCTL1: RWRegister<u32>,

    _reserved32: [u32; 1],

    /// device endpoint-1 interrupt register
    pub OTG_FS_DOEPINT1: RWRegister<u32>,

    _reserved33: [u32; 1],

    /// device OUT endpoint-1 transfer size register
    pub OTG_FS_DOEPTSIZ1: RWRegister<u32>,

    _reserved34: [u32; 3],

    /// device endpoint-2 control register
    pub OTG_FS_DOEPCTL2: RWRegister<u32>,

    _reserved35: [u32; 1],

    /// device endpoint-2 interrupt register
    pub OTG_FS_DOEPINT2: RWRegister<u32>,

    _reserved36: [u32; 1],

    /// device OUT endpoint-2 transfer size register
    pub OTG_FS_DOEPTSIZ2: RWRegister<u32>,

    _reserved37: [u32; 3],

    /// device endpoint-3 control register
    pub OTG_FS_DOEPCTL3: RWRegister<u32>,

    _reserved38: [u32; 1],

    /// device endpoint-3 interrupt register
    pub OTG_FS_DOEPINT3: RWRegister<u32>,

    _reserved39: [u32; 1],

    /// device OUT endpoint-3 transfer size register
    pub OTG_FS_DOEPTSIZ3: RWRegister<u32>,

    _reserved40: [u32; 3],

    /// device endpoint-4 control register
    pub OTG_FS_DOEPCTL4: RWRegister<u32>,

    _reserved41: [u32; 1],

    /// device endpoint-4 interrupt register
    pub OTG_FS_DOEPINT4: RWRegister<u32>,

    _reserved42: [u32; 1],

    /// device OUT endpoint-4 transfer size register
    pub OTG_FS_DOEPTSIZ4: RWRegister<u32>,

    _reserved43: [u32; 3],

    /// device endpoint-5 control register
    pub OTG_FS_DOEPCTL5: RWRegister<u32>,

    _reserved44: [u32; 1],

    /// device endpoint-5 interrupt register
    pub OTG_FS_DOEPINT5: RWRegister<u32>,

    _reserved45: [u32; 1],

    /// device OUT endpoint-5 transfer size register
    pub OTG_FS_DOEPTSIZ5: RWRegister<u32>,
}
pub struct ResetValues {
    pub OTG_FS_DCFG: u32,
    pub OTG_FS_DCTL: u32,
    pub OTG_FS_DSTS: u32,
    pub OTG_FS_DIEPMSK: u32,
    pub OTG_FS_DOEPMSK: u32,
    pub OTG_FS_DAINT: u32,
    pub OTG_FS_DAINTMSK: u32,
    pub OTG_FS_DVBUSDIS: u32,
    pub OTG_FS_DVBUSPULSE: u32,
    pub OTG_FS_DIEPEMPMSK: u32,
    pub OTG_FS_DIEPCTL0: u32,
    pub OTG_FS_DIEPINT0: u32,
    pub OTG_FS_DIEPTSIZ0: u32,
    pub OTG_FS_DTXFSTS0: u32,
    pub OTG_FS_DIEPCTL1: u32,
    pub OTG_FS_DIEPINT1: u32,
    pub OTG_FS_DIEPTSIZ1: u32,
    pub OTG_FS_DTXFSTS1: u32,
    pub OTG_FS_DIEPCTL2: u32,
    pub OTG_FS_DIEPINT2: u32,
    pub OTG_FS_DIEPTSIZ2: u32,
    pub OTG_FS_DTXFSTS2: u32,
    pub OTG_FS_DIEPCTL3: u32,
    pub OTG_FS_DIEPINT3: u32,
    pub OTG_FS_DIEPTSIZ3: u32,
    pub OTG_FS_DTXFSTS3: u32,
    pub OTG_FS_DIEPCTL4: u32,
    pub OTG_FS_DIEPINT4: u32,
    pub OTG_FS_DIEPTSIZ4: u32,
    pub OTG_FS_DTXFSTS4: u32,
    pub OTG_FS_DIEPCTL5: u32,
    pub OTG_FS_DIEPINT5: u32,
    pub OTG_FS_DIEPTSIZ5: u32,
    pub OTG_FS_DTXFSTS5: u32,
    pub OTG_FS_DOEPCTL0: u32,
    pub OTG_FS_DOEPINT0: u32,
    pub OTG_FS_DOEPTSIZ0: u32,
    pub OTG_FS_DOEPCTL1: u32,
    pub OTG_FS_DOEPINT1: u32,
    pub OTG_FS_DOEPTSIZ1: u32,
    pub OTG_FS_DOEPCTL2: u32,
    pub OTG_FS_DOEPINT2: u32,
    pub OTG_FS_DOEPTSIZ2: u32,
    pub OTG_FS_DOEPCTL3: u32,
    pub OTG_FS_DOEPINT3: u32,
    pub OTG_FS_DOEPTSIZ3: u32,
    pub OTG_FS_DOEPCTL4: u32,
    pub OTG_FS_DOEPINT4: u32,
    pub OTG_FS_DOEPTSIZ4: u32,
    pub OTG_FS_DOEPCTL5: u32,
    pub OTG_FS_DOEPINT5: u32,
    pub OTG_FS_DOEPTSIZ5: u32,
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
#[cfg(feature = "rtfm")]
unsafe impl Send for Instance {}
