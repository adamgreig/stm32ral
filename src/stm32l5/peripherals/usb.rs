#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Universal serial bus full-speed device interface
//!
//! Used by: stm32l552, stm32l562

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// endpoint 0 register
pub mod EP0R {

    /// Endpoint address
    pub mod EA {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u16 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Status bits, for transmission transfers
    pub mod STAT_TX {
        /// Offset (4 bits)
        pub const offset: u16 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u16 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Data Toggle, for transmission transfers
    pub mod DTOG_TX {
        /// Offset (6 bits)
        pub const offset: u16 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Correct Transfer for transmission
    pub mod CTR_TX {
        /// Offset (7 bits)
        pub const offset: u16 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Endpoint kind
    pub mod EP_KIND {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Endpoint type
    pub mod EP_TYPE {
        /// Offset (9 bits)
        pub const offset: u16 = 9;
        /// Mask (2 bits: 0b11 << 9)
        pub const mask: u16 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Setup transaction completed
    pub mod SETUP {
        /// Offset (11 bits)
        pub const offset: u16 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Status bits, for reception transfers
    pub mod STAT_RX {
        /// Offset (12 bits)
        pub const offset: u16 = 12;
        /// Mask (2 bits: 0b11 << 12)
        pub const mask: u16 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Data Toggle, for reception transfers
    pub mod DTOG_RX {
        /// Offset (14 bits)
        pub const offset: u16 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Correct transfer for reception
    pub mod CTR_RX {
        /// Offset (15 bits)
        pub const offset: u16 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// endpoint 1 register
pub mod EP1R {
    pub use super::EP0R::CTR_RX;
    pub use super::EP0R::CTR_TX;
    pub use super::EP0R::DTOG_RX;
    pub use super::EP0R::DTOG_TX;
    pub use super::EP0R::EA;
    pub use super::EP0R::EP_KIND;
    pub use super::EP0R::EP_TYPE;
    pub use super::EP0R::SETUP;
    pub use super::EP0R::STAT_RX;
    pub use super::EP0R::STAT_TX;
}

/// endpoint 2 register
pub mod EP2R {
    pub use super::EP0R::CTR_RX;
    pub use super::EP0R::CTR_TX;
    pub use super::EP0R::DTOG_RX;
    pub use super::EP0R::DTOG_TX;
    pub use super::EP0R::EA;
    pub use super::EP0R::EP_KIND;
    pub use super::EP0R::EP_TYPE;
    pub use super::EP0R::SETUP;
    pub use super::EP0R::STAT_RX;
    pub use super::EP0R::STAT_TX;
}

/// endpoint 3 register
pub mod EP3R {
    pub use super::EP0R::CTR_RX;
    pub use super::EP0R::CTR_TX;
    pub use super::EP0R::DTOG_RX;
    pub use super::EP0R::DTOG_TX;
    pub use super::EP0R::EA;
    pub use super::EP0R::EP_KIND;
    pub use super::EP0R::EP_TYPE;
    pub use super::EP0R::SETUP;
    pub use super::EP0R::STAT_RX;
    pub use super::EP0R::STAT_TX;
}

/// endpoint 4 register
pub mod EP4R {
    pub use super::EP0R::CTR_RX;
    pub use super::EP0R::CTR_TX;
    pub use super::EP0R::DTOG_RX;
    pub use super::EP0R::DTOG_TX;
    pub use super::EP0R::EA;
    pub use super::EP0R::EP_KIND;
    pub use super::EP0R::EP_TYPE;
    pub use super::EP0R::SETUP;
    pub use super::EP0R::STAT_RX;
    pub use super::EP0R::STAT_TX;
}

/// endpoint 5 register
pub mod EP5R {
    pub use super::EP0R::CTR_RX;
    pub use super::EP0R::CTR_TX;
    pub use super::EP0R::DTOG_RX;
    pub use super::EP0R::DTOG_TX;
    pub use super::EP0R::EA;
    pub use super::EP0R::EP_KIND;
    pub use super::EP0R::EP_TYPE;
    pub use super::EP0R::SETUP;
    pub use super::EP0R::STAT_RX;
    pub use super::EP0R::STAT_TX;
}

/// endpoint 6 register
pub mod EP6R {
    pub use super::EP0R::CTR_RX;
    pub use super::EP0R::CTR_TX;
    pub use super::EP0R::DTOG_RX;
    pub use super::EP0R::DTOG_TX;
    pub use super::EP0R::EA;
    pub use super::EP0R::EP_KIND;
    pub use super::EP0R::EP_TYPE;
    pub use super::EP0R::SETUP;
    pub use super::EP0R::STAT_RX;
    pub use super::EP0R::STAT_TX;
}

/// endpoint 7 register
pub mod EP7R {
    pub use super::EP0R::CTR_RX;
    pub use super::EP0R::CTR_TX;
    pub use super::EP0R::DTOG_RX;
    pub use super::EP0R::DTOG_TX;
    pub use super::EP0R::EA;
    pub use super::EP0R::EP_KIND;
    pub use super::EP0R::EP_TYPE;
    pub use super::EP0R::SETUP;
    pub use super::EP0R::STAT_RX;
    pub use super::EP0R::STAT_TX;
}

/// control register
pub mod CNTR {

    /// Force USB Reset
    pub mod FRES {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Power down
    pub mod PDWN {
        /// Offset (1 bits)
        pub const offset: u16 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Low-power mode
    pub mod LPMODE {
        /// Offset (2 bits)
        pub const offset: u16 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Force suspend
    pub mod FSUSP {
        /// Offset (3 bits)
        pub const offset: u16 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Resume request
    pub mod RESUME {
        /// Offset (4 bits)
        pub const offset: u16 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LPM L1 Resume request
    pub mod L1RESUME {
        /// Offset (5 bits)
        pub const offset: u16 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LPM L1 state request interrupt mask
    pub mod L1REQM {
        /// Offset (7 bits)
        pub const offset: u16 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Expected start of frame interrupt mask
    pub mod ESOFM {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Start of frame interrupt mask
    pub mod SOFM {
        /// Offset (9 bits)
        pub const offset: u16 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// USB reset interrupt mask
    pub mod RESETM {
        /// Offset (10 bits)
        pub const offset: u16 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Suspend mode interrupt mask
    pub mod SUSPM {
        /// Offset (11 bits)
        pub const offset: u16 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Wakeup interrupt mask
    pub mod WKUPM {
        /// Offset (12 bits)
        pub const offset: u16 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Error interrupt mask
    pub mod ERRM {
        /// Offset (13 bits)
        pub const offset: u16 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Packet memory area over / underrun interrupt mask
    pub mod PMAOVRM {
        /// Offset (14 bits)
        pub const offset: u16 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Correct transfer interrupt mask
    pub mod CTRM {
        /// Offset (15 bits)
        pub const offset: u16 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// interrupt status register
pub mod ISTR {

    /// Endpoint Identifier
    pub mod EP_ID {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u16 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Direction of transaction
    pub mod DIR {
        /// Offset (4 bits)
        pub const offset: u16 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LPM L1 state request
    pub mod L1REQ {
        /// Offset (7 bits)
        pub const offset: u16 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Expected start frame
    pub mod ESOF {
        /// Offset (8 bits)
        pub const offset: u16 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// start of frame
    pub mod SOF {
        /// Offset (9 bits)
        pub const offset: u16 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// reset request
    pub mod RESET {
        /// Offset (10 bits)
        pub const offset: u16 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Suspend mode request
    pub mod SUSP {
        /// Offset (11 bits)
        pub const offset: u16 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Wakeup
    pub mod WKUP {
        /// Offset (12 bits)
        pub const offset: u16 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Error
    pub mod ERR {
        /// Offset (13 bits)
        pub const offset: u16 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Packet memory area over / underrun
    pub mod PMAOVR {
        /// Offset (14 bits)
        pub const offset: u16 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Correct transfer
    pub mod CTR {
        /// Offset (15 bits)
        pub const offset: u16 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// frame number register
pub mod FNR {

    /// Frame number
    pub mod FN {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (11 bits: 0x7ff << 0)
        pub const mask: u16 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Lost SOF
    pub mod LSOF {
        /// Offset (11 bits)
        pub const offset: u16 = 11;
        /// Mask (2 bits: 0b11 << 11)
        pub const mask: u16 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Locked
    pub mod LCK {
        /// Offset (13 bits)
        pub const offset: u16 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Receive data - line status
    pub mod RXDM {
        /// Offset (14 bits)
        pub const offset: u16 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Receive data + line status
    pub mod RXDP {
        /// Offset (15 bits)
        pub const offset: u16 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// device address
pub mod DADDR {

    /// Device address
    pub mod ADD {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (7 bits: 0x7f << 0)
        pub const mask: u16 = 0x7f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Enable function
    pub mod EF {
        /// Offset (7 bits)
        pub const offset: u16 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Buffer table address
pub mod BTABLE {

    /// Buffer table
    pub mod BTABLE {
        /// Offset (3 bits)
        pub const offset: u16 = 3;
        /// Mask (13 bits: 0x1fff << 3)
        pub const mask: u16 = 0x1fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// LPM control and status register
pub mod LPMCSR {

    /// LPM support enable
    pub mod LPMEN {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// LPM Token acknowledge enable
    pub mod LPMACK {
        /// Offset (1 bits)
        pub const offset: u16 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RemoteWake value
    pub mod REMWAKE {
        /// Offset (3 bits)
        pub const offset: u16 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// BESL value
    pub mod BESL {
        /// Offset (4 bits)
        pub const offset: u16 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u16 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Battery charging detector
pub mod BCDR {

    /// Battery charging detector (BCD) enable
    pub mod BCDEN {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Data contact detection (DCD) mode enable
    pub mod DCDEN {
        /// Offset (1 bits)
        pub const offset: u16 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Primary detection (PD) mode enable
    pub mod PDEN {
        /// Offset (2 bits)
        pub const offset: u16 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Secondary detection (SD) mode enable
    pub mod SDEN {
        /// Offset (3 bits)
        pub const offset: u16 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Data contact detection (DCD) status
    pub mod DCDET {
        /// Offset (4 bits)
        pub const offset: u16 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Primary detection (PD) status
    pub mod PDET {
        /// Offset (5 bits)
        pub const offset: u16 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Secondary detection (SD) status
    pub mod SDET {
        /// Offset (6 bits)
        pub const offset: u16 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DM pull-up detection status
    pub mod PS2DET {
        /// Offset (7 bits)
        pub const offset: u16 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DP pull-up control
    pub mod DPPU {
        /// Offset (15 bits)
        pub const offset: u16 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Transmission byte count 0
pub mod COUNT0_TX {

    /// Transmission byte count
    pub mod COUNT0_TX {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (10 bits: 0x3ff << 0)
        pub const mask: u16 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Transmission byte count 0
pub mod COUNT1_TX {

    /// Transmission byte count
    pub mod COUNT1_TX {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (10 bits: 0x3ff << 0)
        pub const mask: u16 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Transmission byte count 0
pub mod COUNT2_TX {

    /// Transmission byte count
    pub mod COUNT2_TX {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (10 bits: 0x3ff << 0)
        pub const mask: u16 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Transmission byte count 0
pub mod COUNT3_TX {

    /// Transmission byte count
    pub mod COUNT3_TX {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (10 bits: 0x3ff << 0)
        pub const mask: u16 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Transmission byte count 0
pub mod COUNT4_TX {

    /// Transmission byte count
    pub mod COUNT4_TX {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (10 bits: 0x3ff << 0)
        pub const mask: u16 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Transmission byte count 0
pub mod COUNT5_TX {

    /// Transmission byte count
    pub mod COUNT5_TX {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (10 bits: 0x3ff << 0)
        pub const mask: u16 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Transmission byte count 0
pub mod COUNT6_TX {

    /// Transmission byte count
    pub mod COUNT6_TX {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (10 bits: 0x3ff << 0)
        pub const mask: u16 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Transmission byte count 0
pub mod COUNT7_TX {

    /// Transmission byte count
    pub mod COUNT7_TX {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (10 bits: 0x3ff << 0)
        pub const mask: u16 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Reception buffer address 0
pub mod ADDR0_RX {

    /// Reception buffer address
    pub mod ADDR0_RX {
        /// Offset (1 bits)
        pub const offset: u16 = 1;
        /// Mask (15 bits: 0x7fff << 1)
        pub const mask: u16 = 0x7fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Reception buffer address 0
pub mod ADDR1_RX {

    /// Reception buffer address
    pub mod ADDR1_RX {
        /// Offset (1 bits)
        pub const offset: u16 = 1;
        /// Mask (15 bits: 0x7fff << 1)
        pub const mask: u16 = 0x7fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Reception buffer address 0
pub mod ADDR2_RX {

    /// Reception buffer address
    pub mod ADDR2_RX {
        /// Offset (1 bits)
        pub const offset: u16 = 1;
        /// Mask (15 bits: 0x7fff << 1)
        pub const mask: u16 = 0x7fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Reception buffer address 0
pub mod ADDR3_RX {

    /// Reception buffer address
    pub mod ADDR3_RX {
        /// Offset (1 bits)
        pub const offset: u16 = 1;
        /// Mask (15 bits: 0x7fff << 1)
        pub const mask: u16 = 0x7fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Reception buffer address 0
pub mod ADDR4_RX {

    /// Reception buffer address
    pub mod ADDR4_RX {
        /// Offset (1 bits)
        pub const offset: u16 = 1;
        /// Mask (15 bits: 0x7fff << 1)
        pub const mask: u16 = 0x7fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Reception buffer address 0
pub mod ADDR5_RX {

    /// Reception buffer address
    pub mod ADDR5_RX {
        /// Offset (1 bits)
        pub const offset: u16 = 1;
        /// Mask (15 bits: 0x7fff << 1)
        pub const mask: u16 = 0x7fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Reception buffer address 0
pub mod ADDR6_RX {

    /// Reception buffer address
    pub mod ADDR6_RX {
        /// Offset (1 bits)
        pub const offset: u16 = 1;
        /// Mask (15 bits: 0x7fff << 1)
        pub const mask: u16 = 0x7fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Reception buffer address 0
pub mod ADDR7_RX {

    /// Reception buffer address
    pub mod ADDR7_RX {
        /// Offset (1 bits)
        pub const offset: u16 = 1;
        /// Mask (15 bits: 0x7fff << 1)
        pub const mask: u16 = 0x7fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Reception byte count 0
pub mod COUNT0_RX {

    /// Reception byte count
    pub mod COUNT0_RX {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (10 bits: 0x3ff << 0)
        pub const mask: u16 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Number of blocks
    pub mod NUM_BLOCK {
        /// Offset (10 bits)
        pub const offset: u16 = 10;
        /// Mask (5 bits: 0b11111 << 10)
        pub const mask: u16 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Block size
    pub mod BL_SIZE {
        /// Offset (15 bits)
        pub const offset: u16 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Reception byte count 0
pub mod COUNT1_RX {

    /// Reception byte count
    pub mod COUNT1_RX {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (10 bits: 0x3ff << 0)
        pub const mask: u16 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Number of blocks
    pub mod NUM_BLOCK {
        /// Offset (10 bits)
        pub const offset: u16 = 10;
        /// Mask (5 bits: 0b11111 << 10)
        pub const mask: u16 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Block size
    pub mod BL_SIZE {
        /// Offset (15 bits)
        pub const offset: u16 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Reception byte count 0
pub mod COUNT2_RX {

    /// Reception byte count
    pub mod COUNT2_RX {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (10 bits: 0x3ff << 0)
        pub const mask: u16 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Number of blocks
    pub mod NUM_BLOCK {
        /// Offset (10 bits)
        pub const offset: u16 = 10;
        /// Mask (5 bits: 0b11111 << 10)
        pub const mask: u16 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Block size
    pub mod BL_SIZE {
        /// Offset (15 bits)
        pub const offset: u16 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Reception byte count 0
pub mod COUNT3_RX {

    /// Reception byte count
    pub mod COUNT3_RX {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (10 bits: 0x3ff << 0)
        pub const mask: u16 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Number of blocks
    pub mod NUM_BLOCK {
        /// Offset (10 bits)
        pub const offset: u16 = 10;
        /// Mask (5 bits: 0b11111 << 10)
        pub const mask: u16 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Block size
    pub mod BL_SIZE {
        /// Offset (15 bits)
        pub const offset: u16 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Reception byte count 0
pub mod COUNT4_RX {

    /// Reception byte count
    pub mod COUNT4_RX {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (10 bits: 0x3ff << 0)
        pub const mask: u16 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Number of blocks
    pub mod NUM_BLOCK {
        /// Offset (10 bits)
        pub const offset: u16 = 10;
        /// Mask (5 bits: 0b11111 << 10)
        pub const mask: u16 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Block size
    pub mod BL_SIZE {
        /// Offset (15 bits)
        pub const offset: u16 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Reception byte count 0
pub mod COUNT5_RX {

    /// Reception byte count
    pub mod COUNT5_RX {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (10 bits: 0x3ff << 0)
        pub const mask: u16 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Number of blocks
    pub mod NUM_BLOCK {
        /// Offset (10 bits)
        pub const offset: u16 = 10;
        /// Mask (5 bits: 0b11111 << 10)
        pub const mask: u16 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Block size
    pub mod BL_SIZE {
        /// Offset (15 bits)
        pub const offset: u16 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Reception byte count 0
pub mod COUNT6_RX {

    /// Reception byte count
    pub mod COUNT6_RX {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (10 bits: 0x3ff << 0)
        pub const mask: u16 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Number of blocks
    pub mod NUM_BLOCK {
        /// Offset (10 bits)
        pub const offset: u16 = 10;
        /// Mask (5 bits: 0b11111 << 10)
        pub const mask: u16 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Block size
    pub mod BL_SIZE {
        /// Offset (15 bits)
        pub const offset: u16 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u16 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Reception byte count 0
pub mod COUNT7_RX {

    /// Reception byte count
    pub mod COUNT7_RX {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (10 bits: 0x3ff << 0)
        pub const mask: u16 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Number of blocks
    pub mod NUM_BLOCK {
        /// Offset (10 bits)
        pub const offset: u16 = 10;
        /// Mask (5 bits: 0b11111 << 10)
        pub const mask: u16 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Block size
    pub mod BL_SIZE {
        /// Offset (15 bits)
        pub const offset: u16 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u16 = 1 << offset;
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
    /// endpoint 0 register
    pub EP0R: RWRegister<u16>,

    _reserved1: [u16; 1],

    /// endpoint 1 register
    pub EP1R: RWRegister<u16>,

    _reserved2: [u16; 1],

    /// endpoint 2 register
    pub EP2R: RWRegister<u16>,

    _reserved3: [u16; 1],

    /// endpoint 3 register
    pub EP3R: RWRegister<u16>,

    _reserved4: [u16; 1],

    /// endpoint 4 register
    pub EP4R: RWRegister<u16>,

    _reserved5: [u16; 1],

    /// endpoint 5 register
    pub EP5R: RWRegister<u16>,

    _reserved6: [u16; 1],

    /// endpoint 6 register
    pub EP6R: RWRegister<u16>,

    _reserved7: [u16; 1],

    /// endpoint 7 register
    pub EP7R: RWRegister<u16>,

    _reserved8: [u32; 8],
    _reserved9: [u16; 1],

    /// control register
    pub CNTR: RWRegister<u16>,

    _reserved10: [u16; 1],

    /// interrupt status register
    pub ISTR: RWRegister<u16>,

    _reserved11: [u16; 1],

    /// frame number register
    pub FNR: RORegister<u16>,

    _reserved12: [u16; 1],

    /// device address
    pub DADDR: RWRegister<u16>,

    _reserved13: [u16; 1],

    /// Buffer table address
    pub BTABLE: RWRegister<u16>,

    /// Transmission byte count 0
    pub COUNT0_TX: RWRegister<u16>,

    /// LPM control and status register
    pub LPMCSR: RWRegister<u16>,

    /// Reception byte count 0
    pub COUNT0_RX: RWRegister<u16>,

    /// Battery charging detector
    pub BCDR: RWRegister<u16>,

    /// Transmission byte count 0
    pub COUNT1_TX: RWRegister<u16>,

    /// Reception buffer address 0
    pub ADDR0_RX: RWRegister<u16>,

    /// Reception byte count 0
    pub COUNT1_RX: RWRegister<u16>,

    _reserved14: [u16; 1],

    /// Transmission byte count 0
    pub COUNT2_TX: RWRegister<u16>,

    /// Reception buffer address 0
    pub ADDR1_RX: RWRegister<u16>,

    /// Reception byte count 0
    pub COUNT2_RX: RWRegister<u16>,

    _reserved15: [u16; 1],

    /// Transmission byte count 0
    pub COUNT3_TX: RWRegister<u16>,

    /// Reception buffer address 0
    pub ADDR2_RX: RWRegister<u16>,

    /// Reception byte count 0
    pub COUNT3_RX: RWRegister<u16>,

    _reserved16: [u16; 1],

    /// Transmission byte count 0
    pub COUNT4_TX: RWRegister<u16>,

    /// Reception buffer address 0
    pub ADDR3_RX: RWRegister<u16>,

    /// Reception byte count 0
    pub COUNT4_RX: RWRegister<u16>,

    _reserved17: [u16; 1],

    /// Transmission byte count 0
    pub COUNT5_TX: RWRegister<u16>,

    /// Reception buffer address 0
    pub ADDR4_RX: RWRegister<u16>,

    /// Reception byte count 0
    pub COUNT5_RX: RWRegister<u16>,

    _reserved18: [u16; 1],

    /// Transmission byte count 0
    pub COUNT6_TX: RWRegister<u16>,

    /// Reception buffer address 0
    pub ADDR5_RX: RWRegister<u16>,

    /// Reception byte count 0
    pub COUNT6_RX: RWRegister<u16>,

    _reserved19: [u16; 1],

    /// Transmission byte count 0
    pub COUNT7_TX: RWRegister<u16>,

    /// Reception buffer address 0
    pub ADDR6_RX: RWRegister<u16>,

    /// Reception byte count 0
    pub COUNT7_RX: RWRegister<u16>,

    _reserved20: [u32; 1],

    /// Reception buffer address 0
    pub ADDR7_RX: RWRegister<u16>,
}
pub struct ResetValues {
    pub EP0R: u16,
    pub EP1R: u16,
    pub EP2R: u16,
    pub EP3R: u16,
    pub EP4R: u16,
    pub EP5R: u16,
    pub EP6R: u16,
    pub EP7R: u16,
    pub CNTR: u16,
    pub ISTR: u16,
    pub FNR: u16,
    pub DADDR: u16,
    pub BTABLE: u16,
    pub COUNT0_TX: u16,
    pub LPMCSR: u16,
    pub COUNT0_RX: u16,
    pub BCDR: u16,
    pub COUNT1_TX: u16,
    pub ADDR0_RX: u16,
    pub COUNT1_RX: u16,
    pub COUNT2_TX: u16,
    pub ADDR1_RX: u16,
    pub COUNT2_RX: u16,
    pub COUNT3_TX: u16,
    pub ADDR2_RX: u16,
    pub COUNT3_RX: u16,
    pub COUNT4_TX: u16,
    pub ADDR3_RX: u16,
    pub COUNT4_RX: u16,
    pub COUNT5_TX: u16,
    pub ADDR4_RX: u16,
    pub COUNT5_RX: u16,
    pub COUNT6_TX: u16,
    pub ADDR5_RX: u16,
    pub COUNT6_RX: u16,
    pub COUNT7_TX: u16,
    pub ADDR6_RX: u16,
    pub COUNT7_RX: u16,
    pub ADDR7_RX: u16,
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
