#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Universal serial bus full-speed device interface
//!
//! Used by: stm32l0x2, stm32l0x3

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// endpoint register
pub mod EP0R {

    /// CTR_RX
    pub mod CTR_RX {
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

    /// DTOG_RX
    pub mod DTOG_RX {
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

    /// STAT_RX
    pub mod STAT_RX {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (2 bits: 0b11 << 12)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: all reception requests addressed to this endpoint are ignored
            pub const Disabled: u32 = 0b00;

            /// 0b01: the endpoint is stalled and all reception requests result in a STALL handshake
            pub const Stall: u32 = 0b01;

            /// 0b10: the endpoint is naked and all reception requests result in a NAK handshake
            pub const Nak: u32 = 0b10;

            /// 0b11: this endpoint is enabled for reception
            pub const Valid: u32 = 0b11;
        }
    }

    /// SETUP
    pub mod SETUP {
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

    /// EPTYPE
    pub mod EP_TYPE {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (2 bits: 0b11 << 9)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Bulk endpoint
            pub const Bulk: u32 = 0b00;

            /// 0b01: Control endpoint
            pub const Control: u32 = 0b01;

            /// 0b10: Iso endpoint
            pub const Iso: u32 = 0b10;

            /// 0b11: Interrupt endpoint
            pub const Interrupt: u32 = 0b11;
        }
    }

    /// EP_KIND
    pub mod EP_KIND {
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

    /// CTR_TX
    pub mod CTR_TX {
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

    /// DTOG_TX
    pub mod DTOG_TX {
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

    /// STAT_TX
    pub mod STAT_TX {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: all transmission requests addressed to this endpoint are ignored
            pub const Disabled: u32 = 0b00;

            /// 0b01: the endpoint is stalled and all transmission requests result in a STALL handshake
            pub const Stall: u32 = 0b01;

            /// 0b10: the endpoint is naked and all transmission requests result in a NAK handshake
            pub const Nak: u32 = 0b10;

            /// 0b11: this endpoint is enabled for transmission
            pub const Valid: u32 = 0b11;
        }
    }

    /// EA
    pub mod EA {
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
}

/// endpoint register
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

/// endpoint register
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

/// endpoint register
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

/// endpoint register
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

/// endpoint register
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

/// endpoint register
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

/// endpoint register
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

    /// CTRM
    pub mod CTRM {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Correct Transfer (CTR) Interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: CTR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set
            pub const Enabled: u32 = 0b1;
        }
    }

    /// PMAOVRM
    pub mod PMAOVRM {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: PMAOVR Interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: PMAOVR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set
            pub const Enabled: u32 = 0b1;
        }
    }

    /// ERRM
    pub mod ERRM {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: ERR Interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: ERR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set
            pub const Enabled: u32 = 0b1;
        }
    }

    /// WKUPM
    pub mod WKUPM {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: WKUP Interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: WKUP Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set
            pub const Enabled: u32 = 0b1;
        }
    }

    /// SUSPM
    pub mod SUSPM {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Suspend Mode Request SUSP Interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: SUSP Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set
            pub const Enabled: u32 = 0b1;
        }
    }

    /// RESETM
    pub mod RESETM {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: RESET Interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: RESET Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set
            pub const Enabled: u32 = 0b1;
        }
    }

    /// SOFM
    pub mod SOFM {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: SOF Interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: SOF Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set
            pub const Enabled: u32 = 0b1;
        }
    }

    /// ESOFM
    pub mod ESOFM {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: ESOF Interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: ESOF Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set
            pub const Enabled: u32 = 0b1;
        }
    }

    /// L1REQM
    pub mod L1REQM {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: L1REQ Interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: L1REQ Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set
            pub const Enabled: u32 = 0b1;
        }
    }

    /// L1RESUME
    pub mod L1RESUME {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b1: LPM L1 request requested
            pub const Requested: u32 = 0b1;
        }
    }

    /// RESUME
    pub mod RESUME {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b1: Resume requested
            pub const Requested: u32 = 0b1;
        }
    }

    /// FSUSP
    pub mod FSUSP {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No effect
            pub const NoEffect: u32 = 0b0;

            /// 0b1: Enter suspend mode. Clocks and static power dissipation in the analog transceiver are left unaffected
            pub const Suspend: u32 = 0b1;
        }
    }

    /// LPMODE
    pub mod LPMODE {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No low-power mode
            pub const Disabled: u32 = 0b0;

            /// 0b1: Enter low-power mode
            pub const Enabled: u32 = 0b1;
        }
    }

    /// PDWN
    pub mod PDWN {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No power down
            pub const Disabled: u32 = 0b0;

            /// 0b1: Enter power down mode
            pub const Enabled: u32 = 0b1;
        }
    }

    /// FRES
    pub mod FRES {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Clear USB reset
            pub const NoReset: u32 = 0b0;

            /// 0b1: Force a reset of the USB peripheral, exactly like a RESET signaling on the USB
            pub const Reset: u32 = 0b1;
        }
    }
}

/// interrupt status register
pub mod ISTR {

    /// CTR
    pub mod CTR {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b1: endpoint has successfully completed a transaction
            pub const Completed: u32 = 0b1;
        }
    }

    /// PMAOVR
    pub mod PMAOVR {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b1: microcontroller has not been able to respond in time to an USB memory request
            pub const Overrun: u32 = 0b1;
        }
    }

    /// ERR
    pub mod ERR {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b1: One of No ANSwer, Cyclic Redundancy Check, Bit Stuffing or Framing format Violation error occurred
            pub const Error: u32 = 0b1;
        }
    }

    /// WKUP
    pub mod WKUP {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b1: activity is detected that wakes up the USB peripheral
            pub const Wakeup: u32 = 0b1;
        }
    }

    /// SUSP
    pub mod SUSP {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b1: no traffic has been received for 3 ms, indicating a suspend mode request from the USB bus
            pub const Suspend: u32 = 0b1;
        }
    }

    /// RESET
    pub mod RESET {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b1: peripheral detects an active USB RESET signal at its inputs
            pub const Reset: u32 = 0b1;
        }
    }

    /// SOF
    pub mod SOF {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b1: beginning of a new USB frame and it is set when a SOF packet arrives through the USB bus
            pub const StartOfFrame: u32 = 0b1;
        }
    }

    /// ESOF
    pub mod ESOF {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b1: an SOF packet is expected but not received
            pub const ExpectedStartOfFrame: u32 = 0b1;
        }
    }

    /// L1REQ
    pub mod L1REQ {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b1: LPM command to enter the L1 state is successfully received and acknowledged
            pub const Received: u32 = 0b1;
        }
    }

    /// DIR
    pub mod DIR {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: data transmitted by the USB peripheral to the host PC
            pub const To: u32 = 0b0;

            /// 0b1: data received by the USB peripheral from the host PC
            pub const From: u32 = 0b1;
        }
    }

    /// EP_ID
    pub mod EP_ID {
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
}

/// frame number register
pub mod FNR {

    /// RXDP
    pub mod RXDP {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b1: received data plus upstream port data line
            pub const Received: u32 = 0b1;
        }
    }

    /// RXDM
    pub mod RXDM {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b1: received data minus upstream port data line
            pub const Received: u32 = 0b1;
        }
    }

    /// LCK
    pub mod LCK {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b1: the frame timer remains in this state until an USB reset or USB suspend event occurs
            pub const Locked: u32 = 0b1;
        }
    }

    /// LSOF
    pub mod LSOF {
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

    /// FN
    pub mod FN {
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

/// device address
pub mod DADDR {

    /// EF
    pub mod EF {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: USB device disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: USB device enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// ADD
    pub mod ADD {
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

/// Buffer table address
pub mod BTABLE {

    /// BTABLE
    pub mod BTABLE {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (13 bits: 0x1fff << 3)
        pub const mask: u32 = 0x1fff << offset;
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

    /// BESL
    pub mod BESL {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
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
        /// Read-write values
        pub mod RW {

            /// 0b0: the valid LPM Token will be NYET
            pub const Nyet: u32 = 0b0;

            /// 0b1: the valid LPM Token will be ACK
            pub const Ack: u32 = 0b1;
        }
    }

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
        /// Read-write values
        pub mod RW {

            /// 0b0: enable the LPM support within the USB device
            pub const Disabled: u32 = 0b0;

            /// 0b1: no LPM transactions are handled
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// Battery charging detector
pub mod BCDR {

    /// DPPU
    pub mod DPPU {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: signalize disconnect to the host when needed by the user software
            pub const Disabled: u32 = 0b0;

            /// 0b1: enable the embedded pull-up on the DP line
            pub const Enabled: u32 = 0b1;
        }
    }

    /// PS2DET
    pub mod PS2DET {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Normal port detected
            pub const Normal: u32 = 0b0;

            /// 0b1: PS2 port or proprietary charger detected
            pub const PS2: u32 = 0b1;
        }
    }

    /// SDET
    pub mod SDET {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: CDP detected
            pub const CDP: u32 = 0b0;

            /// 0b1: DCP detected
            pub const DCP: u32 = 0b1;
        }
    }

    /// PDET
    pub mod PDET {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: no BCD support detected
            pub const NoBCD: u32 = 0b0;

            /// 0b1: BCD support detected
            pub const BCD: u32 = 0b1;
        }
    }

    /// DCDET
    pub mod DCDET {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: data lines contact not detected
            pub const NotDetected: u32 = 0b0;

            /// 0b1: data lines contact detected
            pub const Detected: u32 = 0b1;
        }
    }

    /// SDEN
    pub mod SDEN {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Secondary detection (SD) mode disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Secondary detection (SD) mode enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// PDEN
    pub mod PDEN {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Primary detection (PD) mode disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Primary detection (PD) mode enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// DCDEN
    pub mod DCDEN {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Data contact detection (DCD) mode disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Data contact detection (DCD) mode enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// BCDEN
    pub mod BCDEN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: disable the BCD support
            pub const Disabled: u32 = 0b0;

            /// 0b1: enable the BCD support within the USB device
            pub const Enabled: u32 = 0b1;
        }
    }
}
pub struct RegisterBlock {
    /// endpoint register
    pub EP0R: RWRegister<u32>,

    /// endpoint register
    pub EP1R: RWRegister<u32>,

    /// endpoint register
    pub EP2R: RWRegister<u32>,

    /// endpoint register
    pub EP3R: RWRegister<u32>,

    /// endpoint register
    pub EP4R: RWRegister<u32>,

    /// endpoint register
    pub EP5R: RWRegister<u32>,

    /// endpoint register
    pub EP6R: RWRegister<u32>,

    /// endpoint register
    pub EP7R: RWRegister<u32>,

    _reserved1: [u32; 8],

    /// control register
    pub CNTR: RWRegister<u32>,

    /// interrupt status register
    pub ISTR: RWRegister<u32>,

    /// frame number register
    pub FNR: RORegister<u32>,

    /// device address
    pub DADDR: RWRegister<u32>,

    /// Buffer table address
    pub BTABLE: RWRegister<u32>,

    /// LPM control and status register
    pub LPMCSR: RWRegister<u32>,

    /// Battery charging detector
    pub BCDR: RWRegister<u32>,
}
pub struct ResetValues {
    pub EP0R: u32,
    pub EP1R: u32,
    pub EP2R: u32,
    pub EP3R: u32,
    pub EP4R: u32,
    pub EP5R: u32,
    pub EP6R: u32,
    pub EP7R: u32,
    pub CNTR: u32,
    pub ISTR: u32,
    pub FNR: u32,
    pub DADDR: u32,
    pub BTABLE: u32,
    pub LPMCSR: u32,
    pub BCDR: u32,
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
