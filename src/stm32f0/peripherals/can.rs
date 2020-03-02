#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Controller area network
//!
//! Used by: stm32f0x1, stm32f0x2, stm32f0x8

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// CAN_MCR
pub mod MCR {

    /// DBF
    pub mod DBF {
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

    /// RESET
    pub mod RESET {
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

    /// TTCM
    pub mod TTCM {
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

    /// ABOM
    pub mod ABOM {
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

    /// AWUM
    pub mod AWUM {
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

    /// NART
    pub mod NART {
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

    /// RFLM
    pub mod RFLM {
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

    /// TXFP
    pub mod TXFP {
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

    /// SLEEP
    pub mod SLEEP {
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

    /// INRQ
    pub mod INRQ {
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

/// CAN_MSR
pub mod MSR {

    /// RX
    pub mod RX {
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

    /// SAMP
    pub mod SAMP {
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

    /// RXM
    pub mod RXM {
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

    /// TXM
    pub mod TXM {
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

    /// SLAKI
    pub mod SLAKI {
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

    /// WKUI
    pub mod WKUI {
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

    /// ERRI
    pub mod ERRI {
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

    /// SLAK
    pub mod SLAK {
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

    /// INAK
    pub mod INAK {
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

/// CAN_TSR
pub mod TSR {

    /// Lowest priority flag for mailbox 2
    pub mod LOW2 {
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

    /// Lowest priority flag for mailbox 1
    pub mod LOW1 {
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

    /// Lowest priority flag for mailbox 0
    pub mod LOW0 {
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

    /// Lowest priority flag for mailbox 2
    pub mod TME2 {
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

    /// Lowest priority flag for mailbox 1
    pub mod TME1 {
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

    /// Lowest priority flag for mailbox 0
    pub mod TME0 {
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

    /// CODE
    pub mod CODE {
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

    /// ABRQ2
    pub mod ABRQ2 {
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

    /// TERR2
    pub mod TERR2 {
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

    /// ALST2
    pub mod ALST2 {
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

    /// TXOK2
    pub mod TXOK2 {
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

    /// RQCP2
    pub mod RQCP2 {
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

    /// ABRQ1
    pub mod ABRQ1 {
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

    /// TERR1
    pub mod TERR1 {
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

    /// ALST1
    pub mod ALST1 {
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

    /// TXOK1
    pub mod TXOK1 {
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

    /// RQCP1
    pub mod RQCP1 {
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

    /// ABRQ0
    pub mod ABRQ0 {
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

    /// TERR0
    pub mod TERR0 {
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

    /// ALST0
    pub mod ALST0 {
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

    /// TXOK0
    pub mod TXOK0 {
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

    /// RQCP0
    pub mod RQCP0 {
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

/// CAN_RF0R
pub mod RF0R {

    /// RFOM0
    pub mod RFOM {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: Set by software to release the output mailbox of the FIFO
            pub const Release: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// FOVR0
    pub mod FOVR {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No FIFO x overrun
            pub const NoOverrun: u32 = 0b0;

            /// 0b1: FIFO x overrun
            pub const Overrun: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b1: Clear flag
            pub const Clear: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// FULL0
    pub mod FULL {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: FIFO x is not full
            pub const NotFull: u32 = 0b0;

            /// 0b1: FIFO x is full
            pub const Full: u32 = 0b1;
        }
        pub use super::FOVR::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// FMP0
    pub mod FMP {
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

/// CAN_RF0R
pub mod RF1R {
    pub use super::RF0R::FMP;
    pub use super::RF0R::FOVR;
    pub use super::RF0R::FULL;
    pub use super::RF0R::RFOM;
}

/// CAN_IER
pub mod IER {

    /// SLKIE
    pub mod SLKIE {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No interrupt when SLAKI bit is set
            pub const Disabled: u32 = 0b0;

            /// 0b1: Interrupt generated when SLAKI bit is set
            pub const Enabled: u32 = 0b1;
        }
    }

    /// WKUIE
    pub mod WKUIE {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No interrupt when WKUI is set
            pub const Disabled: u32 = 0b0;

            /// 0b1: Interrupt generated when WKUI bit is set
            pub const Enabled: u32 = 0b1;
        }
    }

    /// ERRIE
    pub mod ERRIE {
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

            /// 0b0: No interrupt will be generated when an error condition is pending in the CAN_ESR
            pub const Disabled: u32 = 0b0;

            /// 0b1: An interrupt will be generation when an error condition is pending in the CAN_ESR
            pub const Enabled: u32 = 0b1;
        }
    }

    /// LECIE
    pub mod LECIE {
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

            /// 0b0: ERRI bit will not be set when the error code in LEC\[2:0\] is set by hardware on error detection
            pub const Disabled: u32 = 0b0;

            /// 0b1: ERRI bit will be set when the error code in LEC\[2:0\] is set by hardware on error detection
            pub const Enabled: u32 = 0b1;
        }
    }

    /// BOFIE
    pub mod BOFIE {
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

            /// 0b0: ERRI bit will not be set when BOFF is set
            pub const Disabled: u32 = 0b0;

            /// 0b1: ERRI bit will be set when BOFF is set
            pub const Enabled: u32 = 0b1;
        }
    }

    /// EPVIE
    pub mod EPVIE {
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

            /// 0b0: ERRI bit will not be set when EPVF is set
            pub const Disabled: u32 = 0b0;

            /// 0b1: ERRI bit will be set when EPVF is set
            pub const Enabled: u32 = 0b1;
        }
    }

    /// EWGIE
    pub mod EWGIE {
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

            /// 0b0: ERRI bit will not be set when EWGF is set
            pub const Disabled: u32 = 0b0;

            /// 0b1: ERRI bit will be set when EWGF is set
            pub const Enabled: u32 = 0b1;
        }
    }

    /// FOVIE1
    pub mod FOVIE1 {
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

            /// 0b0: No interrupt when FOVR is set
            pub const Disabled: u32 = 0b0;

            /// 0b1: Interrupt generation when FOVR is set
            pub const Enabled: u32 = 0b1;
        }
    }

    /// FFIE1
    pub mod FFIE1 {
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

            /// 0b0: No interrupt when FULL bit is set
            pub const Disabled: u32 = 0b0;

            /// 0b1: Interrupt generated when FULL bit is set
            pub const Enabled: u32 = 0b1;
        }
    }

    /// FMPIE1
    pub mod FMPIE1 {
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

            /// 0b0: No interrupt generated when state of FMP\[1:0\] bits are not 00b
            pub const Disabled: u32 = 0b0;

            /// 0b1: Interrupt generated when state of FMP\[1:0\] bits are not 00b
            pub const Enabled: u32 = 0b1;
        }
    }

    /// FOVIE0
    pub mod FOVIE0 {
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

            /// 0b0: No interrupt when FOVR bit is set
            pub const Disabled: u32 = 0b0;

            /// 0b1: Interrupt generated when FOVR bit is set
            pub const Enabled: u32 = 0b1;
        }
    }

    /// FFIE0
    pub mod FFIE0 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::FFIE1::RW;
    }

    /// FMPIE0
    pub mod FMPIE0 {
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

            /// 0b0: No interrupt generated when state of FMP\[1:0\] bits are not 00
            pub const Disabled: u32 = 0b0;

            /// 0b1: Interrupt generated when state of FMP\[1:0\] bits are not 00b
            pub const Enabled: u32 = 0b1;
        }
    }

    /// TMEIE
    pub mod TMEIE {
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

            /// 0b0: No interrupt when RQCPx bit is set
            pub const Disabled: u32 = 0b0;

            /// 0b1: Interrupt generated when RQCPx bit is set
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// CAN_ESR
pub mod ESR {

    /// REC
    pub mod REC {
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

    /// TEC
    pub mod TEC {
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

    /// LEC
    pub mod LEC {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (3 bits: 0b111 << 4)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: No Error
            pub const NoError: u32 = 0b000;

            /// 0b001: Stuff Error
            pub const Stuff: u32 = 0b001;

            /// 0b010: Form Error
            pub const Form: u32 = 0b010;

            /// 0b011: Acknowledgment Error
            pub const Ack: u32 = 0b011;

            /// 0b100: Bit recessive Error
            pub const BitRecessive: u32 = 0b100;

            /// 0b101: Bit dominant Error
            pub const BitDominant: u32 = 0b101;

            /// 0b110: CRC Error
            pub const Crc: u32 = 0b110;

            /// 0b111: Set by software
            pub const Custom: u32 = 0b111;
        }
    }

    /// BOFF
    pub mod BOFF {
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

    /// EPVF
    pub mod EPVF {
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

    /// EWGF
    pub mod EWGF {
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

/// CAN BTR
pub mod BTR {

    /// SILM
    pub mod SILM {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Normal operation
            pub const Normal: u32 = 0b0;

            /// 0b1: Silent Mode
            pub const Silent: u32 = 0b1;
        }
    }

    /// LBKM
    pub mod LBKM {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Loop Back Mode disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Loop Back Mode enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// SJW
    pub mod SJW {
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

    /// TS2
    pub mod TS2 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (3 bits: 0b111 << 20)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TS1
    pub mod TS1 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (4 bits: 0b1111 << 16)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// BRP
    pub mod BRP {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (10 bits: 0x3ff << 0)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// CAN_FMR
pub mod FMR {

    /// CAN2SB
    pub mod CAN2SB {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (6 bits: 0x3f << 8)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// FINIT
    pub mod FINIT {
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

/// CAN_FM1R
pub mod FM1R {

    /// Filter mode
    pub mod FBM0 {
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

    /// Filter mode
    pub mod FBM1 {
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

    /// Filter mode
    pub mod FBM2 {
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

    /// Filter mode
    pub mod FBM3 {
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

    /// Filter mode
    pub mod FBM4 {
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

    /// Filter mode
    pub mod FBM5 {
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

    /// Filter mode
    pub mod FBM6 {
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

    /// Filter mode
    pub mod FBM7 {
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

    /// Filter mode
    pub mod FBM8 {
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

    /// Filter mode
    pub mod FBM9 {
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

    /// Filter mode
    pub mod FBM10 {
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

    /// Filter mode
    pub mod FBM11 {
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

    /// Filter mode
    pub mod FBM12 {
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

    /// Filter mode
    pub mod FBM13 {
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

    /// Filter mode
    pub mod FBM14 {
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

    /// Filter mode
    pub mod FBM15 {
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

    /// Filter mode
    pub mod FBM16 {
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

    /// Filter mode
    pub mod FBM17 {
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

    /// Filter mode
    pub mod FBM18 {
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

    /// Filter mode
    pub mod FBM19 {
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

    /// Filter mode
    pub mod FBM20 {
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

    /// Filter mode
    pub mod FBM21 {
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

    /// Filter mode
    pub mod FBM22 {
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

    /// Filter mode
    pub mod FBM23 {
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

    /// Filter mode
    pub mod FBM24 {
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

    /// Filter mode
    pub mod FBM25 {
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

    /// Filter mode
    pub mod FBM26 {
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

    /// Filter mode
    pub mod FBM27 {
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

/// CAN_FS1R
pub mod FS1R {

    /// Filter scale configuration
    pub mod FSC0 {
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

    /// Filter scale configuration
    pub mod FSC1 {
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

    /// Filter scale configuration
    pub mod FSC2 {
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

    /// Filter scale configuration
    pub mod FSC3 {
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

    /// Filter scale configuration
    pub mod FSC4 {
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

    /// Filter scale configuration
    pub mod FSC5 {
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

    /// Filter scale configuration
    pub mod FSC6 {
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

    /// Filter scale configuration
    pub mod FSC7 {
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

    /// Filter scale configuration
    pub mod FSC8 {
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

    /// Filter scale configuration
    pub mod FSC9 {
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

    /// Filter scale configuration
    pub mod FSC10 {
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

    /// Filter scale configuration
    pub mod FSC11 {
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

    /// Filter scale configuration
    pub mod FSC12 {
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

    /// Filter scale configuration
    pub mod FSC13 {
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

    /// Filter scale configuration
    pub mod FSC14 {
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

    /// Filter scale configuration
    pub mod FSC15 {
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

    /// Filter scale configuration
    pub mod FSC16 {
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

    /// Filter scale configuration
    pub mod FSC17 {
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

    /// Filter scale configuration
    pub mod FSC18 {
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

    /// Filter scale configuration
    pub mod FSC19 {
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

    /// Filter scale configuration
    pub mod FSC20 {
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

    /// Filter scale configuration
    pub mod FSC21 {
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

    /// Filter scale configuration
    pub mod FSC22 {
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

    /// Filter scale configuration
    pub mod FSC23 {
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

    /// Filter scale configuration
    pub mod FSC24 {
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

    /// Filter scale configuration
    pub mod FSC25 {
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

    /// Filter scale configuration
    pub mod FSC26 {
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

    /// Filter scale configuration
    pub mod FSC27 {
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

/// CAN_FFA1R
pub mod FFA1R {

    /// Filter FIFO assignment for filter 0
    pub mod FFA0 {
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

    /// Filter FIFO assignment for filter 1
    pub mod FFA1 {
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

    /// Filter FIFO assignment for filter 2
    pub mod FFA2 {
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

    /// Filter FIFO assignment for filter 3
    pub mod FFA3 {
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

    /// Filter FIFO assignment for filter 4
    pub mod FFA4 {
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

    /// Filter FIFO assignment for filter 5
    pub mod FFA5 {
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

    /// Filter FIFO assignment for filter 6
    pub mod FFA6 {
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

    /// Filter FIFO assignment for filter 7
    pub mod FFA7 {
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

    /// Filter FIFO assignment for filter 8
    pub mod FFA8 {
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

    /// Filter FIFO assignment for filter 9
    pub mod FFA9 {
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

    /// Filter FIFO assignment for filter 10
    pub mod FFA10 {
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

    /// Filter FIFO assignment for filter 11
    pub mod FFA11 {
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

    /// Filter FIFO assignment for filter 12
    pub mod FFA12 {
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

    /// Filter FIFO assignment for filter 13
    pub mod FFA13 {
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

    /// Filter FIFO assignment for filter 14
    pub mod FFA14 {
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

    /// Filter FIFO assignment for filter 15
    pub mod FFA15 {
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

    /// Filter FIFO assignment for filter 16
    pub mod FFA16 {
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

    /// Filter FIFO assignment for filter 17
    pub mod FFA17 {
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

    /// Filter FIFO assignment for filter 18
    pub mod FFA18 {
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

    /// Filter FIFO assignment for filter 19
    pub mod FFA19 {
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

    /// Filter FIFO assignment for filter 20
    pub mod FFA20 {
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

    /// Filter FIFO assignment for filter 21
    pub mod FFA21 {
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

    /// Filter FIFO assignment for filter 22
    pub mod FFA22 {
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

    /// Filter FIFO assignment for filter 23
    pub mod FFA23 {
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

    /// Filter FIFO assignment for filter 24
    pub mod FFA24 {
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

    /// Filter FIFO assignment for filter 25
    pub mod FFA25 {
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

    /// Filter FIFO assignment for filter 26
    pub mod FFA26 {
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

    /// Filter FIFO assignment for filter 27
    pub mod FFA27 {
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

/// CAN_FA1R
pub mod FA1R {

    /// Filter active
    pub mod FACT0 {
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

    /// Filter active
    pub mod FACT1 {
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

    /// Filter active
    pub mod FACT2 {
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

    /// Filter active
    pub mod FACT3 {
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

    /// Filter active
    pub mod FACT4 {
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

    /// Filter active
    pub mod FACT5 {
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

    /// Filter active
    pub mod FACT6 {
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

    /// Filter active
    pub mod FACT7 {
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

    /// Filter active
    pub mod FACT8 {
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

    /// Filter active
    pub mod FACT9 {
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

    /// Filter active
    pub mod FACT10 {
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

    /// Filter active
    pub mod FACT11 {
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

    /// Filter active
    pub mod FACT12 {
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

    /// Filter active
    pub mod FACT13 {
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

    /// Filter active
    pub mod FACT14 {
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

    /// Filter active
    pub mod FACT15 {
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

    /// Filter active
    pub mod FACT16 {
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

    /// Filter active
    pub mod FACT17 {
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

    /// Filter active
    pub mod FACT18 {
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

    /// Filter active
    pub mod FACT19 {
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

    /// Filter active
    pub mod FACT20 {
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

    /// Filter active
    pub mod FACT21 {
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

    /// Filter active
    pub mod FACT22 {
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

    /// Filter active
    pub mod FACT23 {
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

    /// Filter active
    pub mod FACT24 {
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

    /// Filter active
    pub mod FACT25 {
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

    /// Filter active
    pub mod FACT26 {
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

    /// Filter active
    pub mod FACT27 {
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

/// CAN_TI0R
pub mod TIR0 {

    /// STID
    pub mod STID {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (11 bits: 0x7ff << 21)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// EXID
    pub mod EXID {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (18 bits: 0x3ffff << 3)
        pub const mask: u32 = 0x3ffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// IDE
    pub mod IDE {
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

            /// 0b0: Standard identifier
            pub const Standard: u32 = 0b0;

            /// 0b1: Extended identifier
            pub const Extended: u32 = 0b1;
        }
    }

    /// RTR
    pub mod RTR {
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

            /// 0b0: Data frame
            pub const Data: u32 = 0b0;

            /// 0b1: Remote frame
            pub const Remote: u32 = 0b1;
        }
    }

    /// TXRQ
    pub mod TXRQ {
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

/// CAN_TDT0R
pub mod TDTR0 {

    /// TIME
    pub mod TIME {
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

    /// TGT
    pub mod TGT {
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

    /// DLC
    pub mod DLC {
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

/// CAN_TDL0R
pub mod TDLR0 {

    /// DATA3
    pub mod DATA3 {
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

    /// DATA2
    pub mod DATA2 {
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

    /// DATA1
    pub mod DATA1 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DATA0
    pub mod DATA0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// CAN_TDH0R
pub mod TDHR0 {

    /// DATA7
    pub mod DATA7 {
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

    /// DATA6
    pub mod DATA6 {
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

    /// DATA5
    pub mod DATA5 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DATA4
    pub mod DATA4 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// CAN_TI0R
pub mod TIR1 {
    pub use super::TIR0::EXID;
    pub use super::TIR0::IDE;
    pub use super::TIR0::RTR;
    pub use super::TIR0::STID;
    pub use super::TIR0::TXRQ;
}

/// CAN_TDT0R
pub mod TDTR1 {
    pub use super::TDTR0::DLC;
    pub use super::TDTR0::TGT;
    pub use super::TDTR0::TIME;
}

/// CAN_TDL0R
pub mod TDLR1 {
    pub use super::TDLR0::DATA0;
    pub use super::TDLR0::DATA1;
    pub use super::TDLR0::DATA2;
    pub use super::TDLR0::DATA3;
}

/// CAN_TDH0R
pub mod TDHR1 {
    pub use super::TDHR0::DATA4;
    pub use super::TDHR0::DATA5;
    pub use super::TDHR0::DATA6;
    pub use super::TDHR0::DATA7;
}

/// CAN_TI0R
pub mod TIR2 {
    pub use super::TIR0::EXID;
    pub use super::TIR0::IDE;
    pub use super::TIR0::RTR;
    pub use super::TIR0::STID;
    pub use super::TIR0::TXRQ;
}

/// CAN_TDT0R
pub mod TDTR2 {
    pub use super::TDTR0::DLC;
    pub use super::TDTR0::TGT;
    pub use super::TDTR0::TIME;
}

/// CAN_TDL0R
pub mod TDLR2 {
    pub use super::TDLR0::DATA0;
    pub use super::TDLR0::DATA1;
    pub use super::TDLR0::DATA2;
    pub use super::TDLR0::DATA3;
}

/// CAN_TDH0R
pub mod TDHR2 {
    pub use super::TDHR0::DATA4;
    pub use super::TDHR0::DATA5;
    pub use super::TDHR0::DATA6;
    pub use super::TDHR0::DATA7;
}

/// CAN_RI0R
pub mod RIR0 {

    /// STID
    pub mod STID {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (11 bits: 0x7ff << 21)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// EXID
    pub mod EXID {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (18 bits: 0x3ffff << 3)
        pub const mask: u32 = 0x3ffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// IDE
    pub mod IDE {
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

            /// 0b0: Standard identifier
            pub const Standard: u32 = 0b0;

            /// 0b1: Extended identifier
            pub const Extended: u32 = 0b1;
        }
    }

    /// RTR
    pub mod RTR {
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

            /// 0b0: Data frame
            pub const Data: u32 = 0b0;

            /// 0b1: Remote frame
            pub const Remote: u32 = 0b1;
        }
    }
}

/// CAN_RDT0R
pub mod RDTR0 {

    /// TIME
    pub mod TIME {
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

    /// FMI
    pub mod FMI {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DLC
    pub mod DLC {
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

/// CAN_RDL0R
pub mod RDLR0 {

    /// DATA3
    pub mod DATA3 {
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

    /// DATA2
    pub mod DATA2 {
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

    /// DATA1
    pub mod DATA1 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DATA0
    pub mod DATA0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// CAN_RDH0R
pub mod RDHR0 {

    /// DATA7
    pub mod DATA7 {
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

    /// DATA6
    pub mod DATA6 {
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

    /// DATA5
    pub mod DATA5 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DATA4
    pub mod DATA4 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// CAN_RI0R
pub mod RIR1 {
    pub use super::RIR0::EXID;
    pub use super::RIR0::IDE;
    pub use super::RIR0::RTR;
    pub use super::RIR0::STID;
}

/// CAN_RDT0R
pub mod RDTR1 {
    pub use super::RDTR0::DLC;
    pub use super::RDTR0::FMI;
    pub use super::RDTR0::TIME;
}

/// CAN_RDL0R
pub mod RDLR1 {
    pub use super::RDLR0::DATA0;
    pub use super::RDLR0::DATA1;
    pub use super::RDLR0::DATA2;
    pub use super::RDLR0::DATA3;
}

/// CAN_RDH0R
pub mod RDHR1 {
    pub use super::RDHR0::DATA4;
    pub use super::RDHR0::DATA5;
    pub use super::RDHR0::DATA6;
    pub use super::RDHR0::DATA7;
}

/// Filter bank 0 register 1
pub mod FR10 {

    /// Filter bits
    pub mod FB0 {
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

    /// Filter bits
    pub mod FB1 {
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

    /// Filter bits
    pub mod FB2 {
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

    /// Filter bits
    pub mod FB3 {
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

    /// Filter bits
    pub mod FB4 {
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

    /// Filter bits
    pub mod FB5 {
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

    /// Filter bits
    pub mod FB6 {
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

    /// Filter bits
    pub mod FB7 {
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

    /// Filter bits
    pub mod FB8 {
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

    /// Filter bits
    pub mod FB9 {
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

    /// Filter bits
    pub mod FB10 {
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

    /// Filter bits
    pub mod FB11 {
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

    /// Filter bits
    pub mod FB12 {
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

    /// Filter bits
    pub mod FB13 {
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

    /// Filter bits
    pub mod FB14 {
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

    /// Filter bits
    pub mod FB15 {
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

    /// Filter bits
    pub mod FB16 {
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

    /// Filter bits
    pub mod FB17 {
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

    /// Filter bits
    pub mod FB18 {
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

    /// Filter bits
    pub mod FB19 {
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

    /// Filter bits
    pub mod FB20 {
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

    /// Filter bits
    pub mod FB21 {
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

    /// Filter bits
    pub mod FB22 {
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

    /// Filter bits
    pub mod FB23 {
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

    /// Filter bits
    pub mod FB24 {
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

    /// Filter bits
    pub mod FB25 {
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

    /// Filter bits
    pub mod FB26 {
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

    /// Filter bits
    pub mod FB27 {
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

    /// Filter bits
    pub mod FB28 {
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

    /// Filter bits
    pub mod FB29 {
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

    /// Filter bits
    pub mod FB30 {
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

    /// Filter bits
    pub mod FB31 {
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

/// Filter bank 0 register 2
pub mod FR20 {
    pub use super::FR10::FB0;
    pub use super::FR10::FB1;
    pub use super::FR10::FB10;
    pub use super::FR10::FB11;
    pub use super::FR10::FB12;
    pub use super::FR10::FB13;
    pub use super::FR10::FB14;
    pub use super::FR10::FB15;
    pub use super::FR10::FB16;
    pub use super::FR10::FB17;
    pub use super::FR10::FB18;
    pub use super::FR10::FB19;
    pub use super::FR10::FB2;
    pub use super::FR10::FB20;
    pub use super::FR10::FB21;
    pub use super::FR10::FB22;
    pub use super::FR10::FB23;
    pub use super::FR10::FB24;
    pub use super::FR10::FB25;
    pub use super::FR10::FB26;
    pub use super::FR10::FB27;
    pub use super::FR10::FB28;
    pub use super::FR10::FB29;
    pub use super::FR10::FB3;
    pub use super::FR10::FB30;
    pub use super::FR10::FB31;
    pub use super::FR10::FB4;
    pub use super::FR10::FB5;
    pub use super::FR10::FB6;
    pub use super::FR10::FB7;
    pub use super::FR10::FB8;
    pub use super::FR10::FB9;
}

/// Filter bank 0 register 1
pub mod FR11 {
    pub use super::FR10::FB0;
    pub use super::FR10::FB1;
    pub use super::FR10::FB10;
    pub use super::FR10::FB11;
    pub use super::FR10::FB12;
    pub use super::FR10::FB13;
    pub use super::FR10::FB14;
    pub use super::FR10::FB15;
    pub use super::FR10::FB16;
    pub use super::FR10::FB17;
    pub use super::FR10::FB18;
    pub use super::FR10::FB19;
    pub use super::FR10::FB2;
    pub use super::FR10::FB20;
    pub use super::FR10::FB21;
    pub use super::FR10::FB22;
    pub use super::FR10::FB23;
    pub use super::FR10::FB24;
    pub use super::FR10::FB25;
    pub use super::FR10::FB26;
    pub use super::FR10::FB27;
    pub use super::FR10::FB28;
    pub use super::FR10::FB29;
    pub use super::FR10::FB3;
    pub use super::FR10::FB30;
    pub use super::FR10::FB31;
    pub use super::FR10::FB4;
    pub use super::FR10::FB5;
    pub use super::FR10::FB6;
    pub use super::FR10::FB7;
    pub use super::FR10::FB8;
    pub use super::FR10::FB9;
}

/// Filter bank 0 register 2
pub mod FR21 {
    pub use super::FR10::FB0;
    pub use super::FR10::FB1;
    pub use super::FR10::FB10;
    pub use super::FR10::FB11;
    pub use super::FR10::FB12;
    pub use super::FR10::FB13;
    pub use super::FR10::FB14;
    pub use super::FR10::FB15;
    pub use super::FR10::FB16;
    pub use super::FR10::FB17;
    pub use super::FR10::FB18;
    pub use super::FR10::FB19;
    pub use super::FR10::FB2;
    pub use super::FR10::FB20;
    pub use super::FR10::FB21;
    pub use super::FR10::FB22;
    pub use super::FR10::FB23;
    pub use super::FR10::FB24;
    pub use super::FR10::FB25;
    pub use super::FR10::FB26;
    pub use super::FR10::FB27;
    pub use super::FR10::FB28;
    pub use super::FR10::FB29;
    pub use super::FR10::FB3;
    pub use super::FR10::FB30;
    pub use super::FR10::FB31;
    pub use super::FR10::FB4;
    pub use super::FR10::FB5;
    pub use super::FR10::FB6;
    pub use super::FR10::FB7;
    pub use super::FR10::FB8;
    pub use super::FR10::FB9;
}

/// Filter bank 0 register 1
pub mod FR12 {
    pub use super::FR10::FB0;
    pub use super::FR10::FB1;
    pub use super::FR10::FB10;
    pub use super::FR10::FB11;
    pub use super::FR10::FB12;
    pub use super::FR10::FB13;
    pub use super::FR10::FB14;
    pub use super::FR10::FB15;
    pub use super::FR10::FB16;
    pub use super::FR10::FB17;
    pub use super::FR10::FB18;
    pub use super::FR10::FB19;
    pub use super::FR10::FB2;
    pub use super::FR10::FB20;
    pub use super::FR10::FB21;
    pub use super::FR10::FB22;
    pub use super::FR10::FB23;
    pub use super::FR10::FB24;
    pub use super::FR10::FB25;
    pub use super::FR10::FB26;
    pub use super::FR10::FB27;
    pub use super::FR10::FB28;
    pub use super::FR10::FB29;
    pub use super::FR10::FB3;
    pub use super::FR10::FB30;
    pub use super::FR10::FB31;
    pub use super::FR10::FB4;
    pub use super::FR10::FB5;
    pub use super::FR10::FB6;
    pub use super::FR10::FB7;
    pub use super::FR10::FB8;
    pub use super::FR10::FB9;
}

/// Filter bank 0 register 2
pub mod FR22 {
    pub use super::FR10::FB0;
    pub use super::FR10::FB1;
    pub use super::FR10::FB10;
    pub use super::FR10::FB11;
    pub use super::FR10::FB12;
    pub use super::FR10::FB13;
    pub use super::FR10::FB14;
    pub use super::FR10::FB15;
    pub use super::FR10::FB16;
    pub use super::FR10::FB17;
    pub use super::FR10::FB18;
    pub use super::FR10::FB19;
    pub use super::FR10::FB2;
    pub use super::FR10::FB20;
    pub use super::FR10::FB21;
    pub use super::FR10::FB22;
    pub use super::FR10::FB23;
    pub use super::FR10::FB24;
    pub use super::FR10::FB25;
    pub use super::FR10::FB26;
    pub use super::FR10::FB27;
    pub use super::FR10::FB28;
    pub use super::FR10::FB29;
    pub use super::FR10::FB3;
    pub use super::FR10::FB30;
    pub use super::FR10::FB31;
    pub use super::FR10::FB4;
    pub use super::FR10::FB5;
    pub use super::FR10::FB6;
    pub use super::FR10::FB7;
    pub use super::FR10::FB8;
    pub use super::FR10::FB9;
}

/// Filter bank 0 register 1
pub mod FR13 {
    pub use super::FR10::FB0;
    pub use super::FR10::FB1;
    pub use super::FR10::FB10;
    pub use super::FR10::FB11;
    pub use super::FR10::FB12;
    pub use super::FR10::FB13;
    pub use super::FR10::FB14;
    pub use super::FR10::FB15;
    pub use super::FR10::FB16;
    pub use super::FR10::FB17;
    pub use super::FR10::FB18;
    pub use super::FR10::FB19;
    pub use super::FR10::FB2;
    pub use super::FR10::FB20;
    pub use super::FR10::FB21;
    pub use super::FR10::FB22;
    pub use super::FR10::FB23;
    pub use super::FR10::FB24;
    pub use super::FR10::FB25;
    pub use super::FR10::FB26;
    pub use super::FR10::FB27;
    pub use super::FR10::FB28;
    pub use super::FR10::FB29;
    pub use super::FR10::FB3;
    pub use super::FR10::FB30;
    pub use super::FR10::FB31;
    pub use super::FR10::FB4;
    pub use super::FR10::FB5;
    pub use super::FR10::FB6;
    pub use super::FR10::FB7;
    pub use super::FR10::FB8;
    pub use super::FR10::FB9;
}

/// Filter bank 0 register 2
pub mod FR23 {
    pub use super::FR10::FB0;
    pub use super::FR10::FB1;
    pub use super::FR10::FB10;
    pub use super::FR10::FB11;
    pub use super::FR10::FB12;
    pub use super::FR10::FB13;
    pub use super::FR10::FB14;
    pub use super::FR10::FB15;
    pub use super::FR10::FB16;
    pub use super::FR10::FB17;
    pub use super::FR10::FB18;
    pub use super::FR10::FB19;
    pub use super::FR10::FB2;
    pub use super::FR10::FB20;
    pub use super::FR10::FB21;
    pub use super::FR10::FB22;
    pub use super::FR10::FB23;
    pub use super::FR10::FB24;
    pub use super::FR10::FB25;
    pub use super::FR10::FB26;
    pub use super::FR10::FB27;
    pub use super::FR10::FB28;
    pub use super::FR10::FB29;
    pub use super::FR10::FB3;
    pub use super::FR10::FB30;
    pub use super::FR10::FB31;
    pub use super::FR10::FB4;
    pub use super::FR10::FB5;
    pub use super::FR10::FB6;
    pub use super::FR10::FB7;
    pub use super::FR10::FB8;
    pub use super::FR10::FB9;
}

/// Filter bank 0 register 1
pub mod FR14 {
    pub use super::FR10::FB0;
    pub use super::FR10::FB1;
    pub use super::FR10::FB10;
    pub use super::FR10::FB11;
    pub use super::FR10::FB12;
    pub use super::FR10::FB13;
    pub use super::FR10::FB14;
    pub use super::FR10::FB15;
    pub use super::FR10::FB16;
    pub use super::FR10::FB17;
    pub use super::FR10::FB18;
    pub use super::FR10::FB19;
    pub use super::FR10::FB2;
    pub use super::FR10::FB20;
    pub use super::FR10::FB21;
    pub use super::FR10::FB22;
    pub use super::FR10::FB23;
    pub use super::FR10::FB24;
    pub use super::FR10::FB25;
    pub use super::FR10::FB26;
    pub use super::FR10::FB27;
    pub use super::FR10::FB28;
    pub use super::FR10::FB29;
    pub use super::FR10::FB3;
    pub use super::FR10::FB30;
    pub use super::FR10::FB31;
    pub use super::FR10::FB4;
    pub use super::FR10::FB5;
    pub use super::FR10::FB6;
    pub use super::FR10::FB7;
    pub use super::FR10::FB8;
    pub use super::FR10::FB9;
}

/// Filter bank 0 register 2
pub mod FR24 {
    pub use super::FR10::FB0;
    pub use super::FR10::FB1;
    pub use super::FR10::FB10;
    pub use super::FR10::FB11;
    pub use super::FR10::FB12;
    pub use super::FR10::FB13;
    pub use super::FR10::FB14;
    pub use super::FR10::FB15;
    pub use super::FR10::FB16;
    pub use super::FR10::FB17;
    pub use super::FR10::FB18;
    pub use super::FR10::FB19;
    pub use super::FR10::FB2;
    pub use super::FR10::FB20;
    pub use super::FR10::FB21;
    pub use super::FR10::FB22;
    pub use super::FR10::FB23;
    pub use super::FR10::FB24;
    pub use super::FR10::FB25;
    pub use super::FR10::FB26;
    pub use super::FR10::FB27;
    pub use super::FR10::FB28;
    pub use super::FR10::FB29;
    pub use super::FR10::FB3;
    pub use super::FR10::FB30;
    pub use super::FR10::FB31;
    pub use super::FR10::FB4;
    pub use super::FR10::FB5;
    pub use super::FR10::FB6;
    pub use super::FR10::FB7;
    pub use super::FR10::FB8;
    pub use super::FR10::FB9;
}

/// Filter bank 0 register 1
pub mod FR15 {
    pub use super::FR10::FB0;
    pub use super::FR10::FB1;
    pub use super::FR10::FB10;
    pub use super::FR10::FB11;
    pub use super::FR10::FB12;
    pub use super::FR10::FB13;
    pub use super::FR10::FB14;
    pub use super::FR10::FB15;
    pub use super::FR10::FB16;
    pub use super::FR10::FB17;
    pub use super::FR10::FB18;
    pub use super::FR10::FB19;
    pub use super::FR10::FB2;
    pub use super::FR10::FB20;
    pub use super::FR10::FB21;
    pub use super::FR10::FB22;
    pub use super::FR10::FB23;
    pub use super::FR10::FB24;
    pub use super::FR10::FB25;
    pub use super::FR10::FB26;
    pub use super::FR10::FB27;
    pub use super::FR10::FB28;
    pub use super::FR10::FB29;
    pub use super::FR10::FB3;
    pub use super::FR10::FB30;
    pub use super::FR10::FB31;
    pub use super::FR10::FB4;
    pub use super::FR10::FB5;
    pub use super::FR10::FB6;
    pub use super::FR10::FB7;
    pub use super::FR10::FB8;
    pub use super::FR10::FB9;
}

/// Filter bank 0 register 2
pub mod FR25 {
    pub use super::FR10::FB0;
    pub use super::FR10::FB1;
    pub use super::FR10::FB10;
    pub use super::FR10::FB11;
    pub use super::FR10::FB12;
    pub use super::FR10::FB13;
    pub use super::FR10::FB14;
    pub use super::FR10::FB15;
    pub use super::FR10::FB16;
    pub use super::FR10::FB17;
    pub use super::FR10::FB18;
    pub use super::FR10::FB19;
    pub use super::FR10::FB2;
    pub use super::FR10::FB20;
    pub use super::FR10::FB21;
    pub use super::FR10::FB22;
    pub use super::FR10::FB23;
    pub use super::FR10::FB24;
    pub use super::FR10::FB25;
    pub use super::FR10::FB26;
    pub use super::FR10::FB27;
    pub use super::FR10::FB28;
    pub use super::FR10::FB29;
    pub use super::FR10::FB3;
    pub use super::FR10::FB30;
    pub use super::FR10::FB31;
    pub use super::FR10::FB4;
    pub use super::FR10::FB5;
    pub use super::FR10::FB6;
    pub use super::FR10::FB7;
    pub use super::FR10::FB8;
    pub use super::FR10::FB9;
}

/// Filter bank 0 register 1
pub mod FR16 {
    pub use super::FR10::FB0;
    pub use super::FR10::FB1;
    pub use super::FR10::FB10;
    pub use super::FR10::FB11;
    pub use super::FR10::FB12;
    pub use super::FR10::FB13;
    pub use super::FR10::FB14;
    pub use super::FR10::FB15;
    pub use super::FR10::FB16;
    pub use super::FR10::FB17;
    pub use super::FR10::FB18;
    pub use super::FR10::FB19;
    pub use super::FR10::FB2;
    pub use super::FR10::FB20;
    pub use super::FR10::FB21;
    pub use super::FR10::FB22;
    pub use super::FR10::FB23;
    pub use super::FR10::FB24;
    pub use super::FR10::FB25;
    pub use super::FR10::FB26;
    pub use super::FR10::FB27;
    pub use super::FR10::FB28;
    pub use super::FR10::FB29;
    pub use super::FR10::FB3;
    pub use super::FR10::FB30;
    pub use super::FR10::FB31;
    pub use super::FR10::FB4;
    pub use super::FR10::FB5;
    pub use super::FR10::FB6;
    pub use super::FR10::FB7;
    pub use super::FR10::FB8;
    pub use super::FR10::FB9;
}

/// Filter bank 0 register 2
pub mod FR26 {
    pub use super::FR10::FB0;
    pub use super::FR10::FB1;
    pub use super::FR10::FB10;
    pub use super::FR10::FB11;
    pub use super::FR10::FB12;
    pub use super::FR10::FB13;
    pub use super::FR10::FB14;
    pub use super::FR10::FB15;
    pub use super::FR10::FB16;
    pub use super::FR10::FB17;
    pub use super::FR10::FB18;
    pub use super::FR10::FB19;
    pub use super::FR10::FB2;
    pub use super::FR10::FB20;
    pub use super::FR10::FB21;
    pub use super::FR10::FB22;
    pub use super::FR10::FB23;
    pub use super::FR10::FB24;
    pub use super::FR10::FB25;
    pub use super::FR10::FB26;
    pub use super::FR10::FB27;
    pub use super::FR10::FB28;
    pub use super::FR10::FB29;
    pub use super::FR10::FB3;
    pub use super::FR10::FB30;
    pub use super::FR10::FB31;
    pub use super::FR10::FB4;
    pub use super::FR10::FB5;
    pub use super::FR10::FB6;
    pub use super::FR10::FB7;
    pub use super::FR10::FB8;
    pub use super::FR10::FB9;
}

/// Filter bank 0 register 1
pub mod FR17 {
    pub use super::FR10::FB0;
    pub use super::FR10::FB1;
    pub use super::FR10::FB10;
    pub use super::FR10::FB11;
    pub use super::FR10::FB12;
    pub use super::FR10::FB13;
    pub use super::FR10::FB14;
    pub use super::FR10::FB15;
    pub use super::FR10::FB16;
    pub use super::FR10::FB17;
    pub use super::FR10::FB18;
    pub use super::FR10::FB19;
    pub use super::FR10::FB2;
    pub use super::FR10::FB20;
    pub use super::FR10::FB21;
    pub use super::FR10::FB22;
    pub use super::FR10::FB23;
    pub use super::FR10::FB24;
    pub use super::FR10::FB25;
    pub use super::FR10::FB26;
    pub use super::FR10::FB27;
    pub use super::FR10::FB28;
    pub use super::FR10::FB29;
    pub use super::FR10::FB3;
    pub use super::FR10::FB30;
    pub use super::FR10::FB31;
    pub use super::FR10::FB4;
    pub use super::FR10::FB5;
    pub use super::FR10::FB6;
    pub use super::FR10::FB7;
    pub use super::FR10::FB8;
    pub use super::FR10::FB9;
}

/// Filter bank 0 register 2
pub mod FR27 {
    pub use super::FR10::FB0;
    pub use super::FR10::FB1;
    pub use super::FR10::FB10;
    pub use super::FR10::FB11;
    pub use super::FR10::FB12;
    pub use super::FR10::FB13;
    pub use super::FR10::FB14;
    pub use super::FR10::FB15;
    pub use super::FR10::FB16;
    pub use super::FR10::FB17;
    pub use super::FR10::FB18;
    pub use super::FR10::FB19;
    pub use super::FR10::FB2;
    pub use super::FR10::FB20;
    pub use super::FR10::FB21;
    pub use super::FR10::FB22;
    pub use super::FR10::FB23;
    pub use super::FR10::FB24;
    pub use super::FR10::FB25;
    pub use super::FR10::FB26;
    pub use super::FR10::FB27;
    pub use super::FR10::FB28;
    pub use super::FR10::FB29;
    pub use super::FR10::FB3;
    pub use super::FR10::FB30;
    pub use super::FR10::FB31;
    pub use super::FR10::FB4;
    pub use super::FR10::FB5;
    pub use super::FR10::FB6;
    pub use super::FR10::FB7;
    pub use super::FR10::FB8;
    pub use super::FR10::FB9;
}

/// Filter bank 0 register 1
pub mod FR18 {
    pub use super::FR10::FB0;
    pub use super::FR10::FB1;
    pub use super::FR10::FB10;
    pub use super::FR10::FB11;
    pub use super::FR10::FB12;
    pub use super::FR10::FB13;
    pub use super::FR10::FB14;
    pub use super::FR10::FB15;
    pub use super::FR10::FB16;
    pub use super::FR10::FB17;
    pub use super::FR10::FB18;
    pub use super::FR10::FB19;
    pub use super::FR10::FB2;
    pub use super::FR10::FB20;
    pub use super::FR10::FB21;
    pub use super::FR10::FB22;
    pub use super::FR10::FB23;
    pub use super::FR10::FB24;
    pub use super::FR10::FB25;
    pub use super::FR10::FB26;
    pub use super::FR10::FB27;
    pub use super::FR10::FB28;
    pub use super::FR10::FB29;
    pub use super::FR10::FB3;
    pub use super::FR10::FB30;
    pub use super::FR10::FB31;
    pub use super::FR10::FB4;
    pub use super::FR10::FB5;
    pub use super::FR10::FB6;
    pub use super::FR10::FB7;
    pub use super::FR10::FB8;
    pub use super::FR10::FB9;
}

/// Filter bank 0 register 2
pub mod FR28 {
    pub use super::FR10::FB0;
    pub use super::FR10::FB1;
    pub use super::FR10::FB10;
    pub use super::FR10::FB11;
    pub use super::FR10::FB12;
    pub use super::FR10::FB13;
    pub use super::FR10::FB14;
    pub use super::FR10::FB15;
    pub use super::FR10::FB16;
    pub use super::FR10::FB17;
    pub use super::FR10::FB18;
    pub use super::FR10::FB19;
    pub use super::FR10::FB2;
    pub use super::FR10::FB20;
    pub use super::FR10::FB21;
    pub use super::FR10::FB22;
    pub use super::FR10::FB23;
    pub use super::FR10::FB24;
    pub use super::FR10::FB25;
    pub use super::FR10::FB26;
    pub use super::FR10::FB27;
    pub use super::FR10::FB28;
    pub use super::FR10::FB29;
    pub use super::FR10::FB3;
    pub use super::FR10::FB30;
    pub use super::FR10::FB31;
    pub use super::FR10::FB4;
    pub use super::FR10::FB5;
    pub use super::FR10::FB6;
    pub use super::FR10::FB7;
    pub use super::FR10::FB8;
    pub use super::FR10::FB9;
}

/// Filter bank 0 register 1
pub mod FR19 {
    pub use super::FR10::FB0;
    pub use super::FR10::FB1;
    pub use super::FR10::FB10;
    pub use super::FR10::FB11;
    pub use super::FR10::FB12;
    pub use super::FR10::FB13;
    pub use super::FR10::FB14;
    pub use super::FR10::FB15;
    pub use super::FR10::FB16;
    pub use super::FR10::FB17;
    pub use super::FR10::FB18;
    pub use super::FR10::FB19;
    pub use super::FR10::FB2;
    pub use super::FR10::FB20;
    pub use super::FR10::FB21;
    pub use super::FR10::FB22;
    pub use super::FR10::FB23;
    pub use super::FR10::FB24;
    pub use super::FR10::FB25;
    pub use super::FR10::FB26;
    pub use super::FR10::FB27;
    pub use super::FR10::FB28;
    pub use super::FR10::FB29;
    pub use super::FR10::FB3;
    pub use super::FR10::FB30;
    pub use super::FR10::FB31;
    pub use super::FR10::FB4;
    pub use super::FR10::FB5;
    pub use super::FR10::FB6;
    pub use super::FR10::FB7;
    pub use super::FR10::FB8;
    pub use super::FR10::FB9;
}

/// Filter bank 0 register 2
pub mod FR29 {
    pub use super::FR10::FB0;
    pub use super::FR10::FB1;
    pub use super::FR10::FB10;
    pub use super::FR10::FB11;
    pub use super::FR10::FB12;
    pub use super::FR10::FB13;
    pub use super::FR10::FB14;
    pub use super::FR10::FB15;
    pub use super::FR10::FB16;
    pub use super::FR10::FB17;
    pub use super::FR10::FB18;
    pub use super::FR10::FB19;
    pub use super::FR10::FB2;
    pub use super::FR10::FB20;
    pub use super::FR10::FB21;
    pub use super::FR10::FB22;
    pub use super::FR10::FB23;
    pub use super::FR10::FB24;
    pub use super::FR10::FB25;
    pub use super::FR10::FB26;
    pub use super::FR10::FB27;
    pub use super::FR10::FB28;
    pub use super::FR10::FB29;
    pub use super::FR10::FB3;
    pub use super::FR10::FB30;
    pub use super::FR10::FB31;
    pub use super::FR10::FB4;
    pub use super::FR10::FB5;
    pub use super::FR10::FB6;
    pub use super::FR10::FB7;
    pub use super::FR10::FB8;
    pub use super::FR10::FB9;
}

/// Filter bank 0 register 1
pub mod FR110 {
    pub use super::FR10::FB0;
    pub use super::FR10::FB1;
    pub use super::FR10::FB10;
    pub use super::FR10::FB11;
    pub use super::FR10::FB12;
    pub use super::FR10::FB13;
    pub use super::FR10::FB14;
    pub use super::FR10::FB15;
    pub use super::FR10::FB16;
    pub use super::FR10::FB17;
    pub use super::FR10::FB18;
    pub use super::FR10::FB19;
    pub use super::FR10::FB2;
    pub use super::FR10::FB20;
    pub use super::FR10::FB21;
    pub use super::FR10::FB22;
    pub use super::FR10::FB23;
    pub use super::FR10::FB24;
    pub use super::FR10::FB25;
    pub use super::FR10::FB26;
    pub use super::FR10::FB27;
    pub use super::FR10::FB28;
    pub use super::FR10::FB29;
    pub use super::FR10::FB3;
    pub use super::FR10::FB30;
    pub use super::FR10::FB31;
    pub use super::FR10::FB4;
    pub use super::FR10::FB5;
    pub use super::FR10::FB6;
    pub use super::FR10::FB7;
    pub use super::FR10::FB8;
    pub use super::FR10::FB9;
}

/// Filter bank 0 register 2
pub mod FR210 {
    pub use super::FR10::FB0;
    pub use super::FR10::FB1;
    pub use super::FR10::FB10;
    pub use super::FR10::FB11;
    pub use super::FR10::FB12;
    pub use super::FR10::FB13;
    pub use super::FR10::FB14;
    pub use super::FR10::FB15;
    pub use super::FR10::FB16;
    pub use super::FR10::FB17;
    pub use super::FR10::FB18;
    pub use super::FR10::FB19;
    pub use super::FR10::FB2;
    pub use super::FR10::FB20;
    pub use super::FR10::FB21;
    pub use super::FR10::FB22;
    pub use super::FR10::FB23;
    pub use super::FR10::FB24;
    pub use super::FR10::FB25;
    pub use super::FR10::FB26;
    pub use super::FR10::FB27;
    pub use super::FR10::FB28;
    pub use super::FR10::FB29;
    pub use super::FR10::FB3;
    pub use super::FR10::FB30;
    pub use super::FR10::FB31;
    pub use super::FR10::FB4;
    pub use super::FR10::FB5;
    pub use super::FR10::FB6;
    pub use super::FR10::FB7;
    pub use super::FR10::FB8;
    pub use super::FR10::FB9;
}

/// Filter bank 0 register 1
pub mod FR111 {
    pub use super::FR10::FB0;
    pub use super::FR10::FB1;
    pub use super::FR10::FB10;
    pub use super::FR10::FB11;
    pub use super::FR10::FB12;
    pub use super::FR10::FB13;
    pub use super::FR10::FB14;
    pub use super::FR10::FB15;
    pub use super::FR10::FB16;
    pub use super::FR10::FB17;
    pub use super::FR10::FB18;
    pub use super::FR10::FB19;
    pub use super::FR10::FB2;
    pub use super::FR10::FB20;
    pub use super::FR10::FB21;
    pub use super::FR10::FB22;
    pub use super::FR10::FB23;
    pub use super::FR10::FB24;
    pub use super::FR10::FB25;
    pub use super::FR10::FB26;
    pub use super::FR10::FB27;
    pub use super::FR10::FB28;
    pub use super::FR10::FB29;
    pub use super::FR10::FB3;
    pub use super::FR10::FB30;
    pub use super::FR10::FB31;
    pub use super::FR10::FB4;
    pub use super::FR10::FB5;
    pub use super::FR10::FB6;
    pub use super::FR10::FB7;
    pub use super::FR10::FB8;
    pub use super::FR10::FB9;
}

/// Filter bank 0 register 2
pub mod FR211 {
    pub use super::FR10::FB0;
    pub use super::FR10::FB1;
    pub use super::FR10::FB10;
    pub use super::FR10::FB11;
    pub use super::FR10::FB12;
    pub use super::FR10::FB13;
    pub use super::FR10::FB14;
    pub use super::FR10::FB15;
    pub use super::FR10::FB16;
    pub use super::FR10::FB17;
    pub use super::FR10::FB18;
    pub use super::FR10::FB19;
    pub use super::FR10::FB2;
    pub use super::FR10::FB20;
    pub use super::FR10::FB21;
    pub use super::FR10::FB22;
    pub use super::FR10::FB23;
    pub use super::FR10::FB24;
    pub use super::FR10::FB25;
    pub use super::FR10::FB26;
    pub use super::FR10::FB27;
    pub use super::FR10::FB28;
    pub use super::FR10::FB29;
    pub use super::FR10::FB3;
    pub use super::FR10::FB30;
    pub use super::FR10::FB31;
    pub use super::FR10::FB4;
    pub use super::FR10::FB5;
    pub use super::FR10::FB6;
    pub use super::FR10::FB7;
    pub use super::FR10::FB8;
    pub use super::FR10::FB9;
}

/// Filter bank 0 register 1
pub mod FR112 {
    pub use super::FR10::FB0;
    pub use super::FR10::FB1;
    pub use super::FR10::FB10;
    pub use super::FR10::FB11;
    pub use super::FR10::FB12;
    pub use super::FR10::FB13;
    pub use super::FR10::FB14;
    pub use super::FR10::FB15;
    pub use super::FR10::FB16;
    pub use super::FR10::FB17;
    pub use super::FR10::FB18;
    pub use super::FR10::FB19;
    pub use super::FR10::FB2;
    pub use super::FR10::FB20;
    pub use super::FR10::FB21;
    pub use super::FR10::FB22;
    pub use super::FR10::FB23;
    pub use super::FR10::FB24;
    pub use super::FR10::FB25;
    pub use super::FR10::FB26;
    pub use super::FR10::FB27;
    pub use super::FR10::FB28;
    pub use super::FR10::FB29;
    pub use super::FR10::FB3;
    pub use super::FR10::FB30;
    pub use super::FR10::FB31;
    pub use super::FR10::FB4;
    pub use super::FR10::FB5;
    pub use super::FR10::FB6;
    pub use super::FR10::FB7;
    pub use super::FR10::FB8;
    pub use super::FR10::FB9;
}

/// Filter bank 0 register 2
pub mod FR212 {
    pub use super::FR10::FB0;
    pub use super::FR10::FB1;
    pub use super::FR10::FB10;
    pub use super::FR10::FB11;
    pub use super::FR10::FB12;
    pub use super::FR10::FB13;
    pub use super::FR10::FB14;
    pub use super::FR10::FB15;
    pub use super::FR10::FB16;
    pub use super::FR10::FB17;
    pub use super::FR10::FB18;
    pub use super::FR10::FB19;
    pub use super::FR10::FB2;
    pub use super::FR10::FB20;
    pub use super::FR10::FB21;
    pub use super::FR10::FB22;
    pub use super::FR10::FB23;
    pub use super::FR10::FB24;
    pub use super::FR10::FB25;
    pub use super::FR10::FB26;
    pub use super::FR10::FB27;
    pub use super::FR10::FB28;
    pub use super::FR10::FB29;
    pub use super::FR10::FB3;
    pub use super::FR10::FB30;
    pub use super::FR10::FB31;
    pub use super::FR10::FB4;
    pub use super::FR10::FB5;
    pub use super::FR10::FB6;
    pub use super::FR10::FB7;
    pub use super::FR10::FB8;
    pub use super::FR10::FB9;
}

/// Filter bank 0 register 1
pub mod FR113 {
    pub use super::FR10::FB0;
    pub use super::FR10::FB1;
    pub use super::FR10::FB10;
    pub use super::FR10::FB11;
    pub use super::FR10::FB12;
    pub use super::FR10::FB13;
    pub use super::FR10::FB14;
    pub use super::FR10::FB15;
    pub use super::FR10::FB16;
    pub use super::FR10::FB17;
    pub use super::FR10::FB18;
    pub use super::FR10::FB19;
    pub use super::FR10::FB2;
    pub use super::FR10::FB20;
    pub use super::FR10::FB21;
    pub use super::FR10::FB22;
    pub use super::FR10::FB23;
    pub use super::FR10::FB24;
    pub use super::FR10::FB25;
    pub use super::FR10::FB26;
    pub use super::FR10::FB27;
    pub use super::FR10::FB28;
    pub use super::FR10::FB29;
    pub use super::FR10::FB3;
    pub use super::FR10::FB30;
    pub use super::FR10::FB31;
    pub use super::FR10::FB4;
    pub use super::FR10::FB5;
    pub use super::FR10::FB6;
    pub use super::FR10::FB7;
    pub use super::FR10::FB8;
    pub use super::FR10::FB9;
}

/// Filter bank 0 register 2
pub mod FR213 {
    pub use super::FR10::FB0;
    pub use super::FR10::FB1;
    pub use super::FR10::FB10;
    pub use super::FR10::FB11;
    pub use super::FR10::FB12;
    pub use super::FR10::FB13;
    pub use super::FR10::FB14;
    pub use super::FR10::FB15;
    pub use super::FR10::FB16;
    pub use super::FR10::FB17;
    pub use super::FR10::FB18;
    pub use super::FR10::FB19;
    pub use super::FR10::FB2;
    pub use super::FR10::FB20;
    pub use super::FR10::FB21;
    pub use super::FR10::FB22;
    pub use super::FR10::FB23;
    pub use super::FR10::FB24;
    pub use super::FR10::FB25;
    pub use super::FR10::FB26;
    pub use super::FR10::FB27;
    pub use super::FR10::FB28;
    pub use super::FR10::FB29;
    pub use super::FR10::FB3;
    pub use super::FR10::FB30;
    pub use super::FR10::FB31;
    pub use super::FR10::FB4;
    pub use super::FR10::FB5;
    pub use super::FR10::FB6;
    pub use super::FR10::FB7;
    pub use super::FR10::FB8;
    pub use super::FR10::FB9;
}

/// Filter bank 0 register 1
pub mod FR114 {
    pub use super::FR10::FB0;
    pub use super::FR10::FB1;
    pub use super::FR10::FB10;
    pub use super::FR10::FB11;
    pub use super::FR10::FB12;
    pub use super::FR10::FB13;
    pub use super::FR10::FB14;
    pub use super::FR10::FB15;
    pub use super::FR10::FB16;
    pub use super::FR10::FB17;
    pub use super::FR10::FB18;
    pub use super::FR10::FB19;
    pub use super::FR10::FB2;
    pub use super::FR10::FB20;
    pub use super::FR10::FB21;
    pub use super::FR10::FB22;
    pub use super::FR10::FB23;
    pub use super::FR10::FB24;
    pub use super::FR10::FB25;
    pub use super::FR10::FB26;
    pub use super::FR10::FB27;
    pub use super::FR10::FB28;
    pub use super::FR10::FB29;
    pub use super::FR10::FB3;
    pub use super::FR10::FB30;
    pub use super::FR10::FB31;
    pub use super::FR10::FB4;
    pub use super::FR10::FB5;
    pub use super::FR10::FB6;
    pub use super::FR10::FB7;
    pub use super::FR10::FB8;
    pub use super::FR10::FB9;
}

/// Filter bank 0 register 2
pub mod FR214 {
    pub use super::FR10::FB0;
    pub use super::FR10::FB1;
    pub use super::FR10::FB10;
    pub use super::FR10::FB11;
    pub use super::FR10::FB12;
    pub use super::FR10::FB13;
    pub use super::FR10::FB14;
    pub use super::FR10::FB15;
    pub use super::FR10::FB16;
    pub use super::FR10::FB17;
    pub use super::FR10::FB18;
    pub use super::FR10::FB19;
    pub use super::FR10::FB2;
    pub use super::FR10::FB20;
    pub use super::FR10::FB21;
    pub use super::FR10::FB22;
    pub use super::FR10::FB23;
    pub use super::FR10::FB24;
    pub use super::FR10::FB25;
    pub use super::FR10::FB26;
    pub use super::FR10::FB27;
    pub use super::FR10::FB28;
    pub use super::FR10::FB29;
    pub use super::FR10::FB3;
    pub use super::FR10::FB30;
    pub use super::FR10::FB31;
    pub use super::FR10::FB4;
    pub use super::FR10::FB5;
    pub use super::FR10::FB6;
    pub use super::FR10::FB7;
    pub use super::FR10::FB8;
    pub use super::FR10::FB9;
}

/// Filter bank 0 register 1
pub mod FR115 {
    pub use super::FR10::FB0;
    pub use super::FR10::FB1;
    pub use super::FR10::FB10;
    pub use super::FR10::FB11;
    pub use super::FR10::FB12;
    pub use super::FR10::FB13;
    pub use super::FR10::FB14;
    pub use super::FR10::FB15;
    pub use super::FR10::FB16;
    pub use super::FR10::FB17;
    pub use super::FR10::FB18;
    pub use super::FR10::FB19;
    pub use super::FR10::FB2;
    pub use super::FR10::FB20;
    pub use super::FR10::FB21;
    pub use super::FR10::FB22;
    pub use super::FR10::FB23;
    pub use super::FR10::FB24;
    pub use super::FR10::FB25;
    pub use super::FR10::FB26;
    pub use super::FR10::FB27;
    pub use super::FR10::FB28;
    pub use super::FR10::FB29;
    pub use super::FR10::FB3;
    pub use super::FR10::FB30;
    pub use super::FR10::FB31;
    pub use super::FR10::FB4;
    pub use super::FR10::FB5;
    pub use super::FR10::FB6;
    pub use super::FR10::FB7;
    pub use super::FR10::FB8;
    pub use super::FR10::FB9;
}

/// Filter bank 0 register 2
pub mod FR215 {
    pub use super::FR10::FB0;
    pub use super::FR10::FB1;
    pub use super::FR10::FB10;
    pub use super::FR10::FB11;
    pub use super::FR10::FB12;
    pub use super::FR10::FB13;
    pub use super::FR10::FB14;
    pub use super::FR10::FB15;
    pub use super::FR10::FB16;
    pub use super::FR10::FB17;
    pub use super::FR10::FB18;
    pub use super::FR10::FB19;
    pub use super::FR10::FB2;
    pub use super::FR10::FB20;
    pub use super::FR10::FB21;
    pub use super::FR10::FB22;
    pub use super::FR10::FB23;
    pub use super::FR10::FB24;
    pub use super::FR10::FB25;
    pub use super::FR10::FB26;
    pub use super::FR10::FB27;
    pub use super::FR10::FB28;
    pub use super::FR10::FB29;
    pub use super::FR10::FB3;
    pub use super::FR10::FB30;
    pub use super::FR10::FB31;
    pub use super::FR10::FB4;
    pub use super::FR10::FB5;
    pub use super::FR10::FB6;
    pub use super::FR10::FB7;
    pub use super::FR10::FB8;
    pub use super::FR10::FB9;
}

/// Filter bank 0 register 1
pub mod FR116 {
    pub use super::FR10::FB0;
    pub use super::FR10::FB1;
    pub use super::FR10::FB10;
    pub use super::FR10::FB11;
    pub use super::FR10::FB12;
    pub use super::FR10::FB13;
    pub use super::FR10::FB14;
    pub use super::FR10::FB15;
    pub use super::FR10::FB16;
    pub use super::FR10::FB17;
    pub use super::FR10::FB18;
    pub use super::FR10::FB19;
    pub use super::FR10::FB2;
    pub use super::FR10::FB20;
    pub use super::FR10::FB21;
    pub use super::FR10::FB22;
    pub use super::FR10::FB23;
    pub use super::FR10::FB24;
    pub use super::FR10::FB25;
    pub use super::FR10::FB26;
    pub use super::FR10::FB27;
    pub use super::FR10::FB28;
    pub use super::FR10::FB29;
    pub use super::FR10::FB3;
    pub use super::FR10::FB30;
    pub use super::FR10::FB31;
    pub use super::FR10::FB4;
    pub use super::FR10::FB5;
    pub use super::FR10::FB6;
    pub use super::FR10::FB7;
    pub use super::FR10::FB8;
    pub use super::FR10::FB9;
}

/// Filter bank 0 register 2
pub mod FR216 {
    pub use super::FR10::FB0;
    pub use super::FR10::FB1;
    pub use super::FR10::FB10;
    pub use super::FR10::FB11;
    pub use super::FR10::FB12;
    pub use super::FR10::FB13;
    pub use super::FR10::FB14;
    pub use super::FR10::FB15;
    pub use super::FR10::FB16;
    pub use super::FR10::FB17;
    pub use super::FR10::FB18;
    pub use super::FR10::FB19;
    pub use super::FR10::FB2;
    pub use super::FR10::FB20;
    pub use super::FR10::FB21;
    pub use super::FR10::FB22;
    pub use super::FR10::FB23;
    pub use super::FR10::FB24;
    pub use super::FR10::FB25;
    pub use super::FR10::FB26;
    pub use super::FR10::FB27;
    pub use super::FR10::FB28;
    pub use super::FR10::FB29;
    pub use super::FR10::FB3;
    pub use super::FR10::FB30;
    pub use super::FR10::FB31;
    pub use super::FR10::FB4;
    pub use super::FR10::FB5;
    pub use super::FR10::FB6;
    pub use super::FR10::FB7;
    pub use super::FR10::FB8;
    pub use super::FR10::FB9;
}

/// Filter bank 0 register 1
pub mod FR117 {
    pub use super::FR10::FB0;
    pub use super::FR10::FB1;
    pub use super::FR10::FB10;
    pub use super::FR10::FB11;
    pub use super::FR10::FB12;
    pub use super::FR10::FB13;
    pub use super::FR10::FB14;
    pub use super::FR10::FB15;
    pub use super::FR10::FB16;
    pub use super::FR10::FB17;
    pub use super::FR10::FB18;
    pub use super::FR10::FB19;
    pub use super::FR10::FB2;
    pub use super::FR10::FB20;
    pub use super::FR10::FB21;
    pub use super::FR10::FB22;
    pub use super::FR10::FB23;
    pub use super::FR10::FB24;
    pub use super::FR10::FB25;
    pub use super::FR10::FB26;
    pub use super::FR10::FB27;
    pub use super::FR10::FB28;
    pub use super::FR10::FB29;
    pub use super::FR10::FB3;
    pub use super::FR10::FB30;
    pub use super::FR10::FB31;
    pub use super::FR10::FB4;
    pub use super::FR10::FB5;
    pub use super::FR10::FB6;
    pub use super::FR10::FB7;
    pub use super::FR10::FB8;
    pub use super::FR10::FB9;
}

/// Filter bank 0 register 2
pub mod FR217 {
    pub use super::FR10::FB0;
    pub use super::FR10::FB1;
    pub use super::FR10::FB10;
    pub use super::FR10::FB11;
    pub use super::FR10::FB12;
    pub use super::FR10::FB13;
    pub use super::FR10::FB14;
    pub use super::FR10::FB15;
    pub use super::FR10::FB16;
    pub use super::FR10::FB17;
    pub use super::FR10::FB18;
    pub use super::FR10::FB19;
    pub use super::FR10::FB2;
    pub use super::FR10::FB20;
    pub use super::FR10::FB21;
    pub use super::FR10::FB22;
    pub use super::FR10::FB23;
    pub use super::FR10::FB24;
    pub use super::FR10::FB25;
    pub use super::FR10::FB26;
    pub use super::FR10::FB27;
    pub use super::FR10::FB28;
    pub use super::FR10::FB29;
    pub use super::FR10::FB3;
    pub use super::FR10::FB30;
    pub use super::FR10::FB31;
    pub use super::FR10::FB4;
    pub use super::FR10::FB5;
    pub use super::FR10::FB6;
    pub use super::FR10::FB7;
    pub use super::FR10::FB8;
    pub use super::FR10::FB9;
}

/// Filter bank 0 register 1
pub mod FR118 {
    pub use super::FR10::FB0;
    pub use super::FR10::FB1;
    pub use super::FR10::FB10;
    pub use super::FR10::FB11;
    pub use super::FR10::FB12;
    pub use super::FR10::FB13;
    pub use super::FR10::FB14;
    pub use super::FR10::FB15;
    pub use super::FR10::FB16;
    pub use super::FR10::FB17;
    pub use super::FR10::FB18;
    pub use super::FR10::FB19;
    pub use super::FR10::FB2;
    pub use super::FR10::FB20;
    pub use super::FR10::FB21;
    pub use super::FR10::FB22;
    pub use super::FR10::FB23;
    pub use super::FR10::FB24;
    pub use super::FR10::FB25;
    pub use super::FR10::FB26;
    pub use super::FR10::FB27;
    pub use super::FR10::FB28;
    pub use super::FR10::FB29;
    pub use super::FR10::FB3;
    pub use super::FR10::FB30;
    pub use super::FR10::FB31;
    pub use super::FR10::FB4;
    pub use super::FR10::FB5;
    pub use super::FR10::FB6;
    pub use super::FR10::FB7;
    pub use super::FR10::FB8;
    pub use super::FR10::FB9;
}

/// Filter bank 0 register 2
pub mod FR218 {
    pub use super::FR10::FB0;
    pub use super::FR10::FB1;
    pub use super::FR10::FB10;
    pub use super::FR10::FB11;
    pub use super::FR10::FB12;
    pub use super::FR10::FB13;
    pub use super::FR10::FB14;
    pub use super::FR10::FB15;
    pub use super::FR10::FB16;
    pub use super::FR10::FB17;
    pub use super::FR10::FB18;
    pub use super::FR10::FB19;
    pub use super::FR10::FB2;
    pub use super::FR10::FB20;
    pub use super::FR10::FB21;
    pub use super::FR10::FB22;
    pub use super::FR10::FB23;
    pub use super::FR10::FB24;
    pub use super::FR10::FB25;
    pub use super::FR10::FB26;
    pub use super::FR10::FB27;
    pub use super::FR10::FB28;
    pub use super::FR10::FB29;
    pub use super::FR10::FB3;
    pub use super::FR10::FB30;
    pub use super::FR10::FB31;
    pub use super::FR10::FB4;
    pub use super::FR10::FB5;
    pub use super::FR10::FB6;
    pub use super::FR10::FB7;
    pub use super::FR10::FB8;
    pub use super::FR10::FB9;
}

/// Filter bank 0 register 1
pub mod FR119 {
    pub use super::FR10::FB0;
    pub use super::FR10::FB1;
    pub use super::FR10::FB10;
    pub use super::FR10::FB11;
    pub use super::FR10::FB12;
    pub use super::FR10::FB13;
    pub use super::FR10::FB14;
    pub use super::FR10::FB15;
    pub use super::FR10::FB16;
    pub use super::FR10::FB17;
    pub use super::FR10::FB18;
    pub use super::FR10::FB19;
    pub use super::FR10::FB2;
    pub use super::FR10::FB20;
    pub use super::FR10::FB21;
    pub use super::FR10::FB22;
    pub use super::FR10::FB23;
    pub use super::FR10::FB24;
    pub use super::FR10::FB25;
    pub use super::FR10::FB26;
    pub use super::FR10::FB27;
    pub use super::FR10::FB28;
    pub use super::FR10::FB29;
    pub use super::FR10::FB3;
    pub use super::FR10::FB30;
    pub use super::FR10::FB31;
    pub use super::FR10::FB4;
    pub use super::FR10::FB5;
    pub use super::FR10::FB6;
    pub use super::FR10::FB7;
    pub use super::FR10::FB8;
    pub use super::FR10::FB9;
}

/// Filter bank 0 register 2
pub mod FR219 {
    pub use super::FR10::FB0;
    pub use super::FR10::FB1;
    pub use super::FR10::FB10;
    pub use super::FR10::FB11;
    pub use super::FR10::FB12;
    pub use super::FR10::FB13;
    pub use super::FR10::FB14;
    pub use super::FR10::FB15;
    pub use super::FR10::FB16;
    pub use super::FR10::FB17;
    pub use super::FR10::FB18;
    pub use super::FR10::FB19;
    pub use super::FR10::FB2;
    pub use super::FR10::FB20;
    pub use super::FR10::FB21;
    pub use super::FR10::FB22;
    pub use super::FR10::FB23;
    pub use super::FR10::FB24;
    pub use super::FR10::FB25;
    pub use super::FR10::FB26;
    pub use super::FR10::FB27;
    pub use super::FR10::FB28;
    pub use super::FR10::FB29;
    pub use super::FR10::FB3;
    pub use super::FR10::FB30;
    pub use super::FR10::FB31;
    pub use super::FR10::FB4;
    pub use super::FR10::FB5;
    pub use super::FR10::FB6;
    pub use super::FR10::FB7;
    pub use super::FR10::FB8;
    pub use super::FR10::FB9;
}

/// Filter bank 0 register 1
pub mod FR120 {
    pub use super::FR10::FB0;
    pub use super::FR10::FB1;
    pub use super::FR10::FB10;
    pub use super::FR10::FB11;
    pub use super::FR10::FB12;
    pub use super::FR10::FB13;
    pub use super::FR10::FB14;
    pub use super::FR10::FB15;
    pub use super::FR10::FB16;
    pub use super::FR10::FB17;
    pub use super::FR10::FB18;
    pub use super::FR10::FB19;
    pub use super::FR10::FB2;
    pub use super::FR10::FB20;
    pub use super::FR10::FB21;
    pub use super::FR10::FB22;
    pub use super::FR10::FB23;
    pub use super::FR10::FB24;
    pub use super::FR10::FB25;
    pub use super::FR10::FB26;
    pub use super::FR10::FB27;
    pub use super::FR10::FB28;
    pub use super::FR10::FB29;
    pub use super::FR10::FB3;
    pub use super::FR10::FB30;
    pub use super::FR10::FB31;
    pub use super::FR10::FB4;
    pub use super::FR10::FB5;
    pub use super::FR10::FB6;
    pub use super::FR10::FB7;
    pub use super::FR10::FB8;
    pub use super::FR10::FB9;
}

/// Filter bank 0 register 2
pub mod FR220 {
    pub use super::FR10::FB0;
    pub use super::FR10::FB1;
    pub use super::FR10::FB10;
    pub use super::FR10::FB11;
    pub use super::FR10::FB12;
    pub use super::FR10::FB13;
    pub use super::FR10::FB14;
    pub use super::FR10::FB15;
    pub use super::FR10::FB16;
    pub use super::FR10::FB17;
    pub use super::FR10::FB18;
    pub use super::FR10::FB19;
    pub use super::FR10::FB2;
    pub use super::FR10::FB20;
    pub use super::FR10::FB21;
    pub use super::FR10::FB22;
    pub use super::FR10::FB23;
    pub use super::FR10::FB24;
    pub use super::FR10::FB25;
    pub use super::FR10::FB26;
    pub use super::FR10::FB27;
    pub use super::FR10::FB28;
    pub use super::FR10::FB29;
    pub use super::FR10::FB3;
    pub use super::FR10::FB30;
    pub use super::FR10::FB31;
    pub use super::FR10::FB4;
    pub use super::FR10::FB5;
    pub use super::FR10::FB6;
    pub use super::FR10::FB7;
    pub use super::FR10::FB8;
    pub use super::FR10::FB9;
}

/// Filter bank 0 register 1
pub mod FR121 {
    pub use super::FR10::FB0;
    pub use super::FR10::FB1;
    pub use super::FR10::FB10;
    pub use super::FR10::FB11;
    pub use super::FR10::FB12;
    pub use super::FR10::FB13;
    pub use super::FR10::FB14;
    pub use super::FR10::FB15;
    pub use super::FR10::FB16;
    pub use super::FR10::FB17;
    pub use super::FR10::FB18;
    pub use super::FR10::FB19;
    pub use super::FR10::FB2;
    pub use super::FR10::FB20;
    pub use super::FR10::FB21;
    pub use super::FR10::FB22;
    pub use super::FR10::FB23;
    pub use super::FR10::FB24;
    pub use super::FR10::FB25;
    pub use super::FR10::FB26;
    pub use super::FR10::FB27;
    pub use super::FR10::FB28;
    pub use super::FR10::FB29;
    pub use super::FR10::FB3;
    pub use super::FR10::FB30;
    pub use super::FR10::FB31;
    pub use super::FR10::FB4;
    pub use super::FR10::FB5;
    pub use super::FR10::FB6;
    pub use super::FR10::FB7;
    pub use super::FR10::FB8;
    pub use super::FR10::FB9;
}

/// Filter bank 0 register 2
pub mod FR221 {
    pub use super::FR10::FB0;
    pub use super::FR10::FB1;
    pub use super::FR10::FB10;
    pub use super::FR10::FB11;
    pub use super::FR10::FB12;
    pub use super::FR10::FB13;
    pub use super::FR10::FB14;
    pub use super::FR10::FB15;
    pub use super::FR10::FB16;
    pub use super::FR10::FB17;
    pub use super::FR10::FB18;
    pub use super::FR10::FB19;
    pub use super::FR10::FB2;
    pub use super::FR10::FB20;
    pub use super::FR10::FB21;
    pub use super::FR10::FB22;
    pub use super::FR10::FB23;
    pub use super::FR10::FB24;
    pub use super::FR10::FB25;
    pub use super::FR10::FB26;
    pub use super::FR10::FB27;
    pub use super::FR10::FB28;
    pub use super::FR10::FB29;
    pub use super::FR10::FB3;
    pub use super::FR10::FB30;
    pub use super::FR10::FB31;
    pub use super::FR10::FB4;
    pub use super::FR10::FB5;
    pub use super::FR10::FB6;
    pub use super::FR10::FB7;
    pub use super::FR10::FB8;
    pub use super::FR10::FB9;
}

/// Filter bank 0 register 1
pub mod FR122 {
    pub use super::FR10::FB0;
    pub use super::FR10::FB1;
    pub use super::FR10::FB10;
    pub use super::FR10::FB11;
    pub use super::FR10::FB12;
    pub use super::FR10::FB13;
    pub use super::FR10::FB14;
    pub use super::FR10::FB15;
    pub use super::FR10::FB16;
    pub use super::FR10::FB17;
    pub use super::FR10::FB18;
    pub use super::FR10::FB19;
    pub use super::FR10::FB2;
    pub use super::FR10::FB20;
    pub use super::FR10::FB21;
    pub use super::FR10::FB22;
    pub use super::FR10::FB23;
    pub use super::FR10::FB24;
    pub use super::FR10::FB25;
    pub use super::FR10::FB26;
    pub use super::FR10::FB27;
    pub use super::FR10::FB28;
    pub use super::FR10::FB29;
    pub use super::FR10::FB3;
    pub use super::FR10::FB30;
    pub use super::FR10::FB31;
    pub use super::FR10::FB4;
    pub use super::FR10::FB5;
    pub use super::FR10::FB6;
    pub use super::FR10::FB7;
    pub use super::FR10::FB8;
    pub use super::FR10::FB9;
}

/// Filter bank 0 register 2
pub mod FR222 {
    pub use super::FR10::FB0;
    pub use super::FR10::FB1;
    pub use super::FR10::FB10;
    pub use super::FR10::FB11;
    pub use super::FR10::FB12;
    pub use super::FR10::FB13;
    pub use super::FR10::FB14;
    pub use super::FR10::FB15;
    pub use super::FR10::FB16;
    pub use super::FR10::FB17;
    pub use super::FR10::FB18;
    pub use super::FR10::FB19;
    pub use super::FR10::FB2;
    pub use super::FR10::FB20;
    pub use super::FR10::FB21;
    pub use super::FR10::FB22;
    pub use super::FR10::FB23;
    pub use super::FR10::FB24;
    pub use super::FR10::FB25;
    pub use super::FR10::FB26;
    pub use super::FR10::FB27;
    pub use super::FR10::FB28;
    pub use super::FR10::FB29;
    pub use super::FR10::FB3;
    pub use super::FR10::FB30;
    pub use super::FR10::FB31;
    pub use super::FR10::FB4;
    pub use super::FR10::FB5;
    pub use super::FR10::FB6;
    pub use super::FR10::FB7;
    pub use super::FR10::FB8;
    pub use super::FR10::FB9;
}

/// Filter bank 0 register 1
pub mod FR123 {
    pub use super::FR10::FB0;
    pub use super::FR10::FB1;
    pub use super::FR10::FB10;
    pub use super::FR10::FB11;
    pub use super::FR10::FB12;
    pub use super::FR10::FB13;
    pub use super::FR10::FB14;
    pub use super::FR10::FB15;
    pub use super::FR10::FB16;
    pub use super::FR10::FB17;
    pub use super::FR10::FB18;
    pub use super::FR10::FB19;
    pub use super::FR10::FB2;
    pub use super::FR10::FB20;
    pub use super::FR10::FB21;
    pub use super::FR10::FB22;
    pub use super::FR10::FB23;
    pub use super::FR10::FB24;
    pub use super::FR10::FB25;
    pub use super::FR10::FB26;
    pub use super::FR10::FB27;
    pub use super::FR10::FB28;
    pub use super::FR10::FB29;
    pub use super::FR10::FB3;
    pub use super::FR10::FB30;
    pub use super::FR10::FB31;
    pub use super::FR10::FB4;
    pub use super::FR10::FB5;
    pub use super::FR10::FB6;
    pub use super::FR10::FB7;
    pub use super::FR10::FB8;
    pub use super::FR10::FB9;
}

/// Filter bank 0 register 2
pub mod FR223 {
    pub use super::FR10::FB0;
    pub use super::FR10::FB1;
    pub use super::FR10::FB10;
    pub use super::FR10::FB11;
    pub use super::FR10::FB12;
    pub use super::FR10::FB13;
    pub use super::FR10::FB14;
    pub use super::FR10::FB15;
    pub use super::FR10::FB16;
    pub use super::FR10::FB17;
    pub use super::FR10::FB18;
    pub use super::FR10::FB19;
    pub use super::FR10::FB2;
    pub use super::FR10::FB20;
    pub use super::FR10::FB21;
    pub use super::FR10::FB22;
    pub use super::FR10::FB23;
    pub use super::FR10::FB24;
    pub use super::FR10::FB25;
    pub use super::FR10::FB26;
    pub use super::FR10::FB27;
    pub use super::FR10::FB28;
    pub use super::FR10::FB29;
    pub use super::FR10::FB3;
    pub use super::FR10::FB30;
    pub use super::FR10::FB31;
    pub use super::FR10::FB4;
    pub use super::FR10::FB5;
    pub use super::FR10::FB6;
    pub use super::FR10::FB7;
    pub use super::FR10::FB8;
    pub use super::FR10::FB9;
}

/// Filter bank 0 register 1
pub mod FR124 {
    pub use super::FR10::FB0;
    pub use super::FR10::FB1;
    pub use super::FR10::FB10;
    pub use super::FR10::FB11;
    pub use super::FR10::FB12;
    pub use super::FR10::FB13;
    pub use super::FR10::FB14;
    pub use super::FR10::FB15;
    pub use super::FR10::FB16;
    pub use super::FR10::FB17;
    pub use super::FR10::FB18;
    pub use super::FR10::FB19;
    pub use super::FR10::FB2;
    pub use super::FR10::FB20;
    pub use super::FR10::FB21;
    pub use super::FR10::FB22;
    pub use super::FR10::FB23;
    pub use super::FR10::FB24;
    pub use super::FR10::FB25;
    pub use super::FR10::FB26;
    pub use super::FR10::FB27;
    pub use super::FR10::FB28;
    pub use super::FR10::FB29;
    pub use super::FR10::FB3;
    pub use super::FR10::FB30;
    pub use super::FR10::FB31;
    pub use super::FR10::FB4;
    pub use super::FR10::FB5;
    pub use super::FR10::FB6;
    pub use super::FR10::FB7;
    pub use super::FR10::FB8;
    pub use super::FR10::FB9;
}

/// Filter bank 0 register 2
pub mod FR224 {
    pub use super::FR10::FB0;
    pub use super::FR10::FB1;
    pub use super::FR10::FB10;
    pub use super::FR10::FB11;
    pub use super::FR10::FB12;
    pub use super::FR10::FB13;
    pub use super::FR10::FB14;
    pub use super::FR10::FB15;
    pub use super::FR10::FB16;
    pub use super::FR10::FB17;
    pub use super::FR10::FB18;
    pub use super::FR10::FB19;
    pub use super::FR10::FB2;
    pub use super::FR10::FB20;
    pub use super::FR10::FB21;
    pub use super::FR10::FB22;
    pub use super::FR10::FB23;
    pub use super::FR10::FB24;
    pub use super::FR10::FB25;
    pub use super::FR10::FB26;
    pub use super::FR10::FB27;
    pub use super::FR10::FB28;
    pub use super::FR10::FB29;
    pub use super::FR10::FB3;
    pub use super::FR10::FB30;
    pub use super::FR10::FB31;
    pub use super::FR10::FB4;
    pub use super::FR10::FB5;
    pub use super::FR10::FB6;
    pub use super::FR10::FB7;
    pub use super::FR10::FB8;
    pub use super::FR10::FB9;
}

/// Filter bank 0 register 1
pub mod FR125 {
    pub use super::FR10::FB0;
    pub use super::FR10::FB1;
    pub use super::FR10::FB10;
    pub use super::FR10::FB11;
    pub use super::FR10::FB12;
    pub use super::FR10::FB13;
    pub use super::FR10::FB14;
    pub use super::FR10::FB15;
    pub use super::FR10::FB16;
    pub use super::FR10::FB17;
    pub use super::FR10::FB18;
    pub use super::FR10::FB19;
    pub use super::FR10::FB2;
    pub use super::FR10::FB20;
    pub use super::FR10::FB21;
    pub use super::FR10::FB22;
    pub use super::FR10::FB23;
    pub use super::FR10::FB24;
    pub use super::FR10::FB25;
    pub use super::FR10::FB26;
    pub use super::FR10::FB27;
    pub use super::FR10::FB28;
    pub use super::FR10::FB29;
    pub use super::FR10::FB3;
    pub use super::FR10::FB30;
    pub use super::FR10::FB31;
    pub use super::FR10::FB4;
    pub use super::FR10::FB5;
    pub use super::FR10::FB6;
    pub use super::FR10::FB7;
    pub use super::FR10::FB8;
    pub use super::FR10::FB9;
}

/// Filter bank 0 register 2
pub mod FR225 {
    pub use super::FR10::FB0;
    pub use super::FR10::FB1;
    pub use super::FR10::FB10;
    pub use super::FR10::FB11;
    pub use super::FR10::FB12;
    pub use super::FR10::FB13;
    pub use super::FR10::FB14;
    pub use super::FR10::FB15;
    pub use super::FR10::FB16;
    pub use super::FR10::FB17;
    pub use super::FR10::FB18;
    pub use super::FR10::FB19;
    pub use super::FR10::FB2;
    pub use super::FR10::FB20;
    pub use super::FR10::FB21;
    pub use super::FR10::FB22;
    pub use super::FR10::FB23;
    pub use super::FR10::FB24;
    pub use super::FR10::FB25;
    pub use super::FR10::FB26;
    pub use super::FR10::FB27;
    pub use super::FR10::FB28;
    pub use super::FR10::FB29;
    pub use super::FR10::FB3;
    pub use super::FR10::FB30;
    pub use super::FR10::FB31;
    pub use super::FR10::FB4;
    pub use super::FR10::FB5;
    pub use super::FR10::FB6;
    pub use super::FR10::FB7;
    pub use super::FR10::FB8;
    pub use super::FR10::FB9;
}

/// Filter bank 0 register 1
pub mod FR126 {
    pub use super::FR10::FB0;
    pub use super::FR10::FB1;
    pub use super::FR10::FB10;
    pub use super::FR10::FB11;
    pub use super::FR10::FB12;
    pub use super::FR10::FB13;
    pub use super::FR10::FB14;
    pub use super::FR10::FB15;
    pub use super::FR10::FB16;
    pub use super::FR10::FB17;
    pub use super::FR10::FB18;
    pub use super::FR10::FB19;
    pub use super::FR10::FB2;
    pub use super::FR10::FB20;
    pub use super::FR10::FB21;
    pub use super::FR10::FB22;
    pub use super::FR10::FB23;
    pub use super::FR10::FB24;
    pub use super::FR10::FB25;
    pub use super::FR10::FB26;
    pub use super::FR10::FB27;
    pub use super::FR10::FB28;
    pub use super::FR10::FB29;
    pub use super::FR10::FB3;
    pub use super::FR10::FB30;
    pub use super::FR10::FB31;
    pub use super::FR10::FB4;
    pub use super::FR10::FB5;
    pub use super::FR10::FB6;
    pub use super::FR10::FB7;
    pub use super::FR10::FB8;
    pub use super::FR10::FB9;
}

/// Filter bank 0 register 2
pub mod FR226 {
    pub use super::FR10::FB0;
    pub use super::FR10::FB1;
    pub use super::FR10::FB10;
    pub use super::FR10::FB11;
    pub use super::FR10::FB12;
    pub use super::FR10::FB13;
    pub use super::FR10::FB14;
    pub use super::FR10::FB15;
    pub use super::FR10::FB16;
    pub use super::FR10::FB17;
    pub use super::FR10::FB18;
    pub use super::FR10::FB19;
    pub use super::FR10::FB2;
    pub use super::FR10::FB20;
    pub use super::FR10::FB21;
    pub use super::FR10::FB22;
    pub use super::FR10::FB23;
    pub use super::FR10::FB24;
    pub use super::FR10::FB25;
    pub use super::FR10::FB26;
    pub use super::FR10::FB27;
    pub use super::FR10::FB28;
    pub use super::FR10::FB29;
    pub use super::FR10::FB3;
    pub use super::FR10::FB30;
    pub use super::FR10::FB31;
    pub use super::FR10::FB4;
    pub use super::FR10::FB5;
    pub use super::FR10::FB6;
    pub use super::FR10::FB7;
    pub use super::FR10::FB8;
    pub use super::FR10::FB9;
}

/// Filter bank 0 register 1
pub mod FR127 {
    pub use super::FR10::FB0;
    pub use super::FR10::FB1;
    pub use super::FR10::FB10;
    pub use super::FR10::FB11;
    pub use super::FR10::FB12;
    pub use super::FR10::FB13;
    pub use super::FR10::FB14;
    pub use super::FR10::FB15;
    pub use super::FR10::FB16;
    pub use super::FR10::FB17;
    pub use super::FR10::FB18;
    pub use super::FR10::FB19;
    pub use super::FR10::FB2;
    pub use super::FR10::FB20;
    pub use super::FR10::FB21;
    pub use super::FR10::FB22;
    pub use super::FR10::FB23;
    pub use super::FR10::FB24;
    pub use super::FR10::FB25;
    pub use super::FR10::FB26;
    pub use super::FR10::FB27;
    pub use super::FR10::FB28;
    pub use super::FR10::FB29;
    pub use super::FR10::FB3;
    pub use super::FR10::FB30;
    pub use super::FR10::FB31;
    pub use super::FR10::FB4;
    pub use super::FR10::FB5;
    pub use super::FR10::FB6;
    pub use super::FR10::FB7;
    pub use super::FR10::FB8;
    pub use super::FR10::FB9;
}

/// Filter bank 0 register 2
pub mod FR227 {
    pub use super::FR10::FB0;
    pub use super::FR10::FB1;
    pub use super::FR10::FB10;
    pub use super::FR10::FB11;
    pub use super::FR10::FB12;
    pub use super::FR10::FB13;
    pub use super::FR10::FB14;
    pub use super::FR10::FB15;
    pub use super::FR10::FB16;
    pub use super::FR10::FB17;
    pub use super::FR10::FB18;
    pub use super::FR10::FB19;
    pub use super::FR10::FB2;
    pub use super::FR10::FB20;
    pub use super::FR10::FB21;
    pub use super::FR10::FB22;
    pub use super::FR10::FB23;
    pub use super::FR10::FB24;
    pub use super::FR10::FB25;
    pub use super::FR10::FB26;
    pub use super::FR10::FB27;
    pub use super::FR10::FB28;
    pub use super::FR10::FB29;
    pub use super::FR10::FB3;
    pub use super::FR10::FB30;
    pub use super::FR10::FB31;
    pub use super::FR10::FB4;
    pub use super::FR10::FB5;
    pub use super::FR10::FB6;
    pub use super::FR10::FB7;
    pub use super::FR10::FB8;
    pub use super::FR10::FB9;
}
#[repr(C)]
pub struct RegisterBlock {
    /// CAN_MCR
    pub MCR: RWRegister<u32>,

    /// CAN_MSR
    pub MSR: RWRegister<u32>,

    /// CAN_TSR
    pub TSR: RWRegister<u32>,

    /// CAN_RF0R
    pub RF0R: RWRegister<u32>,

    /// CAN_RF0R
    pub RF1R: RWRegister<u32>,

    /// CAN_IER
    pub IER: RWRegister<u32>,

    /// CAN_ESR
    pub ESR: RWRegister<u32>,

    /// CAN BTR
    pub BTR: RWRegister<u32>,

    _reserved1: [u32; 88],

    /// CAN_TI0R
    pub TIR0: RWRegister<u32>,

    /// CAN_TDT0R
    pub TDTR0: RWRegister<u32>,

    /// CAN_TDL0R
    pub TDLR0: RWRegister<u32>,

    /// CAN_TDH0R
    pub TDHR0: RWRegister<u32>,

    /// CAN_TI0R
    pub TIR1: RWRegister<u32>,

    /// CAN_TDT0R
    pub TDTR1: RWRegister<u32>,

    /// CAN_TDL0R
    pub TDLR1: RWRegister<u32>,

    /// CAN_TDH0R
    pub TDHR1: RWRegister<u32>,

    /// CAN_TI0R
    pub TIR2: RWRegister<u32>,

    /// CAN_TDT0R
    pub TDTR2: RWRegister<u32>,

    /// CAN_TDL0R
    pub TDLR2: RWRegister<u32>,

    /// CAN_TDH0R
    pub TDHR2: RWRegister<u32>,

    /// CAN_RI0R
    pub RIR0: RORegister<u32>,

    /// CAN_RDT0R
    pub RDTR0: RORegister<u32>,

    /// CAN_RDL0R
    pub RDLR0: RORegister<u32>,

    /// CAN_RDH0R
    pub RDHR0: RORegister<u32>,

    /// CAN_RI0R
    pub RIR1: RORegister<u32>,

    /// CAN_RDT0R
    pub RDTR1: RORegister<u32>,

    /// CAN_RDL0R
    pub RDLR1: RORegister<u32>,

    /// CAN_RDH0R
    pub RDHR1: RORegister<u32>,

    _reserved2: [u32; 12],

    /// CAN_FMR
    pub FMR: RWRegister<u32>,

    /// CAN_FM1R
    pub FM1R: RWRegister<u32>,

    _reserved3: [u32; 1],

    /// CAN_FS1R
    pub FS1R: RWRegister<u32>,

    _reserved4: [u32; 1],

    /// CAN_FFA1R
    pub FFA1R: RWRegister<u32>,

    _reserved5: [u32; 1],

    /// CAN_FA1R
    pub FA1R: RWRegister<u32>,

    _reserved6: [u32; 8],

    /// Filter bank 0 register 1
    pub FR10: RWRegister<u32>,

    /// Filter bank 0 register 2
    pub FR20: RWRegister<u32>,

    /// Filter bank 0 register 1
    pub FR11: RWRegister<u32>,

    /// Filter bank 0 register 2
    pub FR21: RWRegister<u32>,

    /// Filter bank 0 register 1
    pub FR12: RWRegister<u32>,

    /// Filter bank 0 register 2
    pub FR22: RWRegister<u32>,

    /// Filter bank 0 register 1
    pub FR13: RWRegister<u32>,

    /// Filter bank 0 register 2
    pub FR23: RWRegister<u32>,

    /// Filter bank 0 register 1
    pub FR14: RWRegister<u32>,

    /// Filter bank 0 register 2
    pub FR24: RWRegister<u32>,

    /// Filter bank 0 register 1
    pub FR15: RWRegister<u32>,

    /// Filter bank 0 register 2
    pub FR25: RWRegister<u32>,

    /// Filter bank 0 register 1
    pub FR16: RWRegister<u32>,

    /// Filter bank 0 register 2
    pub FR26: RWRegister<u32>,

    /// Filter bank 0 register 1
    pub FR17: RWRegister<u32>,

    /// Filter bank 0 register 2
    pub FR27: RWRegister<u32>,

    /// Filter bank 0 register 1
    pub FR18: RWRegister<u32>,

    /// Filter bank 0 register 2
    pub FR28: RWRegister<u32>,

    /// Filter bank 0 register 1
    pub FR19: RWRegister<u32>,

    /// Filter bank 0 register 2
    pub FR29: RWRegister<u32>,

    /// Filter bank 0 register 1
    pub FR110: RWRegister<u32>,

    /// Filter bank 0 register 2
    pub FR210: RWRegister<u32>,

    /// Filter bank 0 register 1
    pub FR111: RWRegister<u32>,

    /// Filter bank 0 register 2
    pub FR211: RWRegister<u32>,

    /// Filter bank 0 register 1
    pub FR112: RWRegister<u32>,

    /// Filter bank 0 register 2
    pub FR212: RWRegister<u32>,

    /// Filter bank 0 register 1
    pub FR113: RWRegister<u32>,

    /// Filter bank 0 register 2
    pub FR213: RWRegister<u32>,

    /// Filter bank 0 register 1
    pub FR114: RWRegister<u32>,

    /// Filter bank 0 register 2
    pub FR214: RWRegister<u32>,

    /// Filter bank 0 register 1
    pub FR115: RWRegister<u32>,

    /// Filter bank 0 register 2
    pub FR215: RWRegister<u32>,

    /// Filter bank 0 register 1
    pub FR116: RWRegister<u32>,

    /// Filter bank 0 register 2
    pub FR216: RWRegister<u32>,

    /// Filter bank 0 register 1
    pub FR117: RWRegister<u32>,

    /// Filter bank 0 register 2
    pub FR217: RWRegister<u32>,

    /// Filter bank 0 register 1
    pub FR118: RWRegister<u32>,

    /// Filter bank 0 register 2
    pub FR218: RWRegister<u32>,

    /// Filter bank 0 register 1
    pub FR119: RWRegister<u32>,

    /// Filter bank 0 register 2
    pub FR219: RWRegister<u32>,

    /// Filter bank 0 register 1
    pub FR120: RWRegister<u32>,

    /// Filter bank 0 register 2
    pub FR220: RWRegister<u32>,

    /// Filter bank 0 register 1
    pub FR121: RWRegister<u32>,

    /// Filter bank 0 register 2
    pub FR221: RWRegister<u32>,

    /// Filter bank 0 register 1
    pub FR122: RWRegister<u32>,

    /// Filter bank 0 register 2
    pub FR222: RWRegister<u32>,

    /// Filter bank 0 register 1
    pub FR123: RWRegister<u32>,

    /// Filter bank 0 register 2
    pub FR223: RWRegister<u32>,

    /// Filter bank 0 register 1
    pub FR124: RWRegister<u32>,

    /// Filter bank 0 register 2
    pub FR224: RWRegister<u32>,

    /// Filter bank 0 register 1
    pub FR125: RWRegister<u32>,

    /// Filter bank 0 register 2
    pub FR225: RWRegister<u32>,

    /// Filter bank 0 register 1
    pub FR126: RWRegister<u32>,

    /// Filter bank 0 register 2
    pub FR226: RWRegister<u32>,

    /// Filter bank 0 register 1
    pub FR127: RWRegister<u32>,

    /// Filter bank 0 register 2
    pub FR227: RWRegister<u32>,
}
pub struct ResetValues {
    pub MCR: u32,
    pub MSR: u32,
    pub TSR: u32,
    pub RF0R: u32,
    pub RF1R: u32,
    pub IER: u32,
    pub ESR: u32,
    pub BTR: u32,
    pub TIR0: u32,
    pub TDTR0: u32,
    pub TDLR0: u32,
    pub TDHR0: u32,
    pub TIR1: u32,
    pub TDTR1: u32,
    pub TDLR1: u32,
    pub TDHR1: u32,
    pub TIR2: u32,
    pub TDTR2: u32,
    pub TDLR2: u32,
    pub TDHR2: u32,
    pub RIR0: u32,
    pub RDTR0: u32,
    pub RDLR0: u32,
    pub RDHR0: u32,
    pub RIR1: u32,
    pub RDTR1: u32,
    pub RDLR1: u32,
    pub RDHR1: u32,
    pub FMR: u32,
    pub FM1R: u32,
    pub FS1R: u32,
    pub FFA1R: u32,
    pub FA1R: u32,
    pub FR10: u32,
    pub FR20: u32,
    pub FR11: u32,
    pub FR21: u32,
    pub FR12: u32,
    pub FR22: u32,
    pub FR13: u32,
    pub FR23: u32,
    pub FR14: u32,
    pub FR24: u32,
    pub FR15: u32,
    pub FR25: u32,
    pub FR16: u32,
    pub FR26: u32,
    pub FR17: u32,
    pub FR27: u32,
    pub FR18: u32,
    pub FR28: u32,
    pub FR19: u32,
    pub FR29: u32,
    pub FR110: u32,
    pub FR210: u32,
    pub FR111: u32,
    pub FR211: u32,
    pub FR112: u32,
    pub FR212: u32,
    pub FR113: u32,
    pub FR213: u32,
    pub FR114: u32,
    pub FR214: u32,
    pub FR115: u32,
    pub FR215: u32,
    pub FR116: u32,
    pub FR216: u32,
    pub FR117: u32,
    pub FR217: u32,
    pub FR118: u32,
    pub FR218: u32,
    pub FR119: u32,
    pub FR219: u32,
    pub FR120: u32,
    pub FR220: u32,
    pub FR121: u32,
    pub FR221: u32,
    pub FR122: u32,
    pub FR222: u32,
    pub FR123: u32,
    pub FR223: u32,
    pub FR124: u32,
    pub FR224: u32,
    pub FR125: u32,
    pub FR225: u32,
    pub FR126: u32,
    pub FR226: u32,
    pub FR127: u32,
    pub FR227: u32,
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
