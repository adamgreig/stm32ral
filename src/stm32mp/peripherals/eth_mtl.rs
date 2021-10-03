#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! ETH_MTL
//!
//! Used by: stm32mp153, stm32mp157

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// The Operating Mode register establishes the Transmit and Receive operating modes and commands.
pub mod ETH_MTLOMR {

    /// DTXSTS
    pub mod DTXSTS {
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

    /// RAA
    pub mod RAA {
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

    /// SCHALG
    pub mod SCHALG {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (2 bits: 0b11 << 5)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CNTPRST
    pub mod CNTPRST {
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

    /// CNTCLR
    pub mod CNTCLR {
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
}

/// The software driver (application) reads this register during interrupt service routine or polling to determine the interrupt status of MTL queues and the MAC.
pub mod ETH_MTLISR {

    /// Q0IS
    pub mod Q0IS {
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

    /// Q1IS
    pub mod Q1IS {
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
}

/// Tx queue 0 operating mode Register
pub mod ETH_MTLTxQ0OMR {

    /// FTQ
    pub mod FTQ {
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

    /// TSF
    pub mod TSF {
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

    /// TXQEN
    pub mod TXQEN {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TTC
    pub mod TTC {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TQS
    pub mod TQS {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (9 bits: 0x1ff << 16)
        pub const mask: u32 = 0x1ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Tx queue 1 operating mode Register
pub mod ETH_MTLTxQ1OMR {
    pub use super::ETH_MTLTxQ0OMR::FTQ;
    pub use super::ETH_MTLTxQ0OMR::TQS;
    pub use super::ETH_MTLTxQ0OMR::TSF;
    pub use super::ETH_MTLTxQ0OMR::TTC;
    pub use super::ETH_MTLTxQ0OMR::TXQEN;
}

/// Tx queue 0 underflow register
pub mod ETH_MTLTxQ0UR {

    /// UFFRMCNT
    pub mod UFFRMCNT {
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

    /// UFCNTOVF
    pub mod UFCNTOVF {
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

/// Tx queue 1 underflow register
pub mod ETH_MTLTxQ1UR {
    pub use super::ETH_MTLTxQ0UR::UFCNTOVF;
    pub use super::ETH_MTLTxQ0UR::UFFRMCNT;
}

/// Tx queue 0 underflow register
pub mod ETH_MTLTxQ0DR {

    /// TXQPAUSED
    pub mod TXQPAUSED {
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

    /// TRCSTS
    pub mod TRCSTS {
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

    /// TWCSTS
    pub mod TWCSTS {
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

    /// TXQSTS
    pub mod TXQSTS {
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

    /// TXSTSFSTS
    pub mod TXSTSFSTS {
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

    /// PTXQ
    pub mod PTXQ {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (3 bits: 0b111 << 16)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// STXSTSF
    pub mod STXSTSF {
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
}

/// Tx queue 1 underflow register
pub mod ETH_MTLTxQ1DR {
    pub use super::ETH_MTLTxQ0DR::PTXQ;
    pub use super::ETH_MTLTxQ0DR::STXSTSF;
    pub use super::ETH_MTLTxQ0DR::TRCSTS;
    pub use super::ETH_MTLTxQ0DR::TWCSTS;
    pub use super::ETH_MTLTxQ0DR::TXQPAUSED;
    pub use super::ETH_MTLTxQ0DR::TXQSTS;
    pub use super::ETH_MTLTxQ0DR::TXSTSFSTS;
}

/// Tx queue x ETS status Register
pub mod ETH_MTLTxQ0ESR {

    /// ABS
    pub mod ABS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (24 bits: 0xffffff << 0)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Tx queue x ETS status Register
pub mod ETH_MTLTxQ1ESR {
    pub use super::ETH_MTLTxQ0ESR::ABS;
}

/// Queue 0 interrupt control status Register
pub mod ETH_MTLQ0ICSR {

    /// TXUNFIS
    pub mod TXUNFIS {
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

    /// ABPSIS
    pub mod ABPSIS {
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

    /// TXUIE
    pub mod TXUIE {
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

    /// ABPSIE
    pub mod ABPSIE {
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

    /// RXOVFIS
    pub mod RXOVFIS {
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

    /// RXOIE
    pub mod RXOIE {
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
}

/// Queue 1 interrupt control status Register
pub mod ETH_MTLQ1ICSR {
    pub use super::ETH_MTLQ0ICSR::ABPSIE;
    pub use super::ETH_MTLQ0ICSR::ABPSIS;
    pub use super::ETH_MTLQ0ICSR::RXOIE;
    pub use super::ETH_MTLQ0ICSR::RXOVFIS;
    pub use super::ETH_MTLQ0ICSR::TXUIE;
    pub use super::ETH_MTLQ0ICSR::TXUNFIS;
}

/// Rx queue 0 operating mode register
pub mod ETH_MTLRxQ0OMR {

    /// RTC
    pub mod RTC {
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

    /// FUP
    pub mod FUP {
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

    /// FEP
    pub mod FEP {
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

    /// RSF
    pub mod RSF {
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

    /// DIS_TCP_EF
    pub mod DIS_TCP_EF {
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

    /// EHFC
    pub mod EHFC {
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

    /// RFA
    pub mod RFA {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (3 bits: 0b111 << 8)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RFD
    pub mod RFD {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (3 bits: 0b111 << 14)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RQS
    pub mod RQS {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Rx queue 1 operating mode register
pub mod ETH_MTLRxQ1OMR {
    pub use super::ETH_MTLRxQ0OMR::DIS_TCP_EF;
    pub use super::ETH_MTLRxQ0OMR::EHFC;
    pub use super::ETH_MTLRxQ0OMR::FEP;
    pub use super::ETH_MTLRxQ0OMR::FUP;
    pub use super::ETH_MTLRxQ0OMR::RFA;
    pub use super::ETH_MTLRxQ0OMR::RFD;
    pub use super::ETH_MTLRxQ0OMR::RQS;
    pub use super::ETH_MTLRxQ0OMR::RSF;
    pub use super::ETH_MTLRxQ0OMR::RTC;
}

/// Rx queue 0 missed packet and overflow counter register
pub mod ETH_MTLRxQ0MPOCR {

    /// OVFPKTCNT
    pub mod OVFPKTCNT {
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

    /// OVFCNTOVF
    pub mod OVFCNTOVF {
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

    /// MISPKTCNT
    pub mod MISPKTCNT {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (11 bits: 0x7ff << 16)
        pub const mask: u32 = 0x7ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// MISCNTOVF
    pub mod MISCNTOVF {
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

/// Rx queue 1 missed packet and overflow counter register
pub mod ETH_MTLRxQ1MPOCR {
    pub use super::ETH_MTLRxQ0MPOCR::MISCNTOVF;
    pub use super::ETH_MTLRxQ0MPOCR::MISPKTCNT;
    pub use super::ETH_MTLRxQ0MPOCR::OVFCNTOVF;
    pub use super::ETH_MTLRxQ0MPOCR::OVFPKTCNT;
}

/// Rx queue i debug register
pub mod ETH_MTLRxQ0DR {

    /// RWCSTS
    pub mod RWCSTS {
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

    /// RRCSTS
    pub mod RRCSTS {
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

    /// RXQSTS
    pub mod RXQSTS {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// PRXQ
    pub mod PRXQ {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (14 bits: 0x3fff << 16)
        pub const mask: u32 = 0x3fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Rx queue i debug register
pub mod ETH_MTLRxQ1DR {
    pub use super::ETH_MTLRxQ0DR::PRXQ;
    pub use super::ETH_MTLRxQ0DR::RRCSTS;
    pub use super::ETH_MTLRxQ0DR::RWCSTS;
    pub use super::ETH_MTLRxQ0DR::RXQSTS;
}

/// Rx queue 0 control register
pub mod ETH_MTLRxQ0CR {

    /// RXQ_WEGT
    pub mod RXQ_WEGT {
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

    /// RXQ_FRM_ARBIT
    pub mod RXQ_FRM_ARBIT {
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
}

/// Rx queue 1 control register
pub mod ETH_MTLRxQ1CR {
    pub use super::ETH_MTLRxQ0CR::RXQ_FRM_ARBIT;
    pub use super::ETH_MTLRxQ0CR::RXQ_WEGT;
}

/// The Queue ETS Control register controls the enhanced transmission selection operation.
pub mod ETH_MTLTxQ1ECR {

    /// AVALG
    pub mod AVALG {
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

    /// CC
    pub mod CC {
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

    /// SLC
    pub mod SLC {
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
}

/// This register provides the average traffic transmitted on queue 1.
pub mod ETH_MTLTxQ1QWR {

    /// ISCQW
    pub mod ISCQW {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (21 bits: 0x1fffff << 0)
        pub const mask: u32 = 0x1fffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// The sendSlopeCredit register contains the sendSlope credit value required for the credit-based shaper algorithm for the Queue.
pub mod ETH_MTLTxQ1SSCR {

    /// SSC
    pub mod SSC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (14 bits: 0x3fff << 0)
        pub const mask: u32 = 0x3fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// The hiCredit register contains the hiCredit value required for the credit-based shaper algorithm for the Queue.
pub mod ETH_MTLTxQ1HCR {

    /// HC
    pub mod HC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (29 bits: 0x1fffffff << 0)
        pub const mask: u32 = 0x1fffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// The loCredit register contains the loCredit value required for the credit-based shaper algorithm for the Queue.
pub mod ETH_MTLTxQ1LCR {

    /// LC
    pub mod LC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (29 bits: 0x1fffffff << 0)
        pub const mask: u32 = 0x1fffffff << offset;
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
    /// The Operating Mode register establishes the Transmit and Receive operating modes and commands.
    pub ETH_MTLOMR: RWRegister<u32>,

    _reserved1: [u32; 7],

    /// The software driver (application) reads this register during interrupt service routine or polling to determine the interrupt status of MTL queues and the MAC.
    pub ETH_MTLISR: RORegister<u32>,

    _reserved2: [u32; 55],

    /// Tx queue 0 operating mode Register
    pub ETH_MTLTxQ0OMR: RWRegister<u32>,

    /// Tx queue 0 underflow register
    pub ETH_MTLTxQ0UR: RORegister<u32>,

    /// Tx queue 0 underflow register
    pub ETH_MTLTxQ0DR: RORegister<u32>,

    _reserved3: [u32; 2],

    /// Tx queue x ETS status Register
    pub ETH_MTLTxQ0ESR: RORegister<u32>,

    _reserved4: [u32; 5],

    /// Queue 0 interrupt control status Register
    pub ETH_MTLQ0ICSR: RWRegister<u32>,

    /// Rx queue 0 operating mode register
    pub ETH_MTLRxQ0OMR: RWRegister<u32>,

    /// Rx queue 0 missed packet and overflow counter register
    pub ETH_MTLRxQ0MPOCR: RORegister<u32>,

    /// Rx queue i debug register
    pub ETH_MTLRxQ0DR: RORegister<u32>,

    /// Rx queue 0 control register
    pub ETH_MTLRxQ0CR: RWRegister<u32>,

    /// Tx queue 1 operating mode Register
    pub ETH_MTLTxQ1OMR: RWRegister<u32>,

    /// Tx queue 1 underflow register
    pub ETH_MTLTxQ1UR: RORegister<u32>,

    /// Tx queue 1 underflow register
    pub ETH_MTLTxQ1DR: RORegister<u32>,

    _reserved5: [u32; 1],

    /// The Queue ETS Control register controls the enhanced transmission selection operation.
    pub ETH_MTLTxQ1ECR: RWRegister<u32>,

    /// Tx queue x ETS status Register
    pub ETH_MTLTxQ1ESR: RORegister<u32>,

    /// This register provides the average traffic transmitted on queue 1.
    pub ETH_MTLTxQ1QWR: RWRegister<u32>,

    /// The sendSlopeCredit register contains the sendSlope credit value required for the credit-based shaper algorithm for the Queue.
    pub ETH_MTLTxQ1SSCR: RWRegister<u32>,

    /// The hiCredit register contains the hiCredit value required for the credit-based shaper algorithm for the Queue.
    pub ETH_MTLTxQ1HCR: RWRegister<u32>,

    /// The loCredit register contains the loCredit value required for the credit-based shaper algorithm for the Queue.
    pub ETH_MTLTxQ1LCR: RWRegister<u32>,

    _reserved6: [u32; 1],

    /// Queue 1 interrupt control status Register
    pub ETH_MTLQ1ICSR: RWRegister<u32>,

    /// Rx queue 1 operating mode register
    pub ETH_MTLRxQ1OMR: RWRegister<u32>,

    /// Rx queue 1 missed packet and overflow counter register
    pub ETH_MTLRxQ1MPOCR: RORegister<u32>,

    /// Rx queue i debug register
    pub ETH_MTLRxQ1DR: RORegister<u32>,

    /// Rx queue 1 control register
    pub ETH_MTLRxQ1CR: RWRegister<u32>,
}
pub struct ResetValues {
    pub ETH_MTLOMR: u32,
    pub ETH_MTLISR: u32,
    pub ETH_MTLTxQ0OMR: u32,
    pub ETH_MTLTxQ0UR: u32,
    pub ETH_MTLTxQ0DR: u32,
    pub ETH_MTLTxQ0ESR: u32,
    pub ETH_MTLQ0ICSR: u32,
    pub ETH_MTLRxQ0OMR: u32,
    pub ETH_MTLRxQ0MPOCR: u32,
    pub ETH_MTLRxQ0DR: u32,
    pub ETH_MTLRxQ0CR: u32,
    pub ETH_MTLTxQ1OMR: u32,
    pub ETH_MTLTxQ1UR: u32,
    pub ETH_MTLTxQ1DR: u32,
    pub ETH_MTLTxQ1ECR: u32,
    pub ETH_MTLTxQ1ESR: u32,
    pub ETH_MTLTxQ1QWR: u32,
    pub ETH_MTLTxQ1SSCR: u32,
    pub ETH_MTLTxQ1HCR: u32,
    pub ETH_MTLTxQ1LCR: u32,
    pub ETH_MTLQ1ICSR: u32,
    pub ETH_MTLRxQ1OMR: u32,
    pub ETH_MTLRxQ1MPOCR: u32,
    pub ETH_MTLRxQ1DR: u32,
    pub ETH_MTLRxQ1CR: u32,
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
