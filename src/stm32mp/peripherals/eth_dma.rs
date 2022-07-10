#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! ETH_DMA
//!
//! Used by: stm32mp153, stm32mp157

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// DMA mode register
pub mod ETH_DMAMR {

    /// Software Reset
    pub mod SWR {
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

    /// TAA
    pub mod TAA {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (3 bits: 0b111 << 2)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Transmit priority
    pub mod TXPR {
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

    /// Priority ratio
    pub mod PR {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (3 bits: 0b111 << 12)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Interrupt Mode
    pub mod INTM {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (2 bits: 0b11 << 16)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// System bus mode register
pub mod ETH_DMASBMR {

    /// Fixed Burst Length
    pub mod FB {
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

    /// BLEN4
    pub mod BLEN4 {
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

    /// BLEN8
    pub mod BLEN8 {
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

    /// BLEN16
    pub mod BLEN16 {
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

    /// BLEN32
    pub mod BLEN32 {
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

    /// BLEN64
    pub mod BLEN64 {
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

    /// BLEN128
    pub mod BLEN128 {
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

    /// BLEN256
    pub mod BLEN256 {
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

    /// Address-Aligned Beats
    pub mod AAL {
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

    /// ONEKBBE
    pub mod ONEKBBE {
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

    /// RD_OSR_LMT
    pub mod RD_OSR_LMT {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (2 bits: 0b11 << 16)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// WR_OSR_LMT
    pub mod WR_OSR_LMT {
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

    /// LPI_XIT_PKT
    pub mod LPI_XIT_PKT {
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

    /// EN_LPI
    pub mod EN_LPI {
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

/// Interrupt status register
pub mod ETH_DMAISR {

    /// DMA Channel Interrupt Status
    pub mod DC0IS {
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

    /// DC1IS
    pub mod DC1IS {
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

    /// MTL Interrupt Status
    pub mod MTLIS {
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

    /// MAC Interrupt Status
    pub mod MACIS {
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

/// Debug status register
pub mod ETH_DMADSR {

    /// AHB Master Write Channel
    pub mod AXWHSTS {
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

    /// AXRHSTS
    pub mod AXRHSTS {
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

    /// RPS0
    pub mod RPS0 {
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

    /// TPS0
    pub mod TPS0 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RPS1
    pub mod RPS1 {
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

    /// TPS1
    pub mod TPS1 {
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

/// AXI4 transmit channel ACE control register
pub mod ETH_DMAA4TxACR {

    /// TDRC
    pub mod TDRC {
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

    /// TEC
    pub mod TEC {
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

    /// THC
    pub mod THC {
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
}

/// AXI4 receive channel ACE control register
pub mod ETH_DMAA4RxACR {

    /// RDWC
    pub mod RDWC {
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

    /// RPC
    pub mod RPC {
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

    /// RHC
    pub mod RHC {
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

    /// RDC
    pub mod RDC {
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

/// AXI4 descriptor ACE control register
pub mod ETH_DMAA4DACR {

    /// TDWC
    pub mod TDWC {
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

    /// TDWD
    pub mod TDWD {
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

    /// RDRC
    pub mod RDRC {
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

    /// RDP
    pub mod RDP {
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

    /// WRP
    pub mod WRP {
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

/// Channel 0 control register
pub mod ETH_DMAC0CR {

    /// MSS
    pub mod MSS {
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

    /// PBLX8
    pub mod PBLX8 {
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

    /// DSL
    pub mod DSL {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (3 bits: 0b111 << 18)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Channel 1 control register
pub mod ETH_DMAC1CR {
    pub use super::ETH_DMAC0CR::DSL;
    pub use super::ETH_DMAC0CR::MSS;
    pub use super::ETH_DMAC0CR::PBLX8;
}

/// Channel 0 transmit control register
pub mod ETH_DMAC0TxCR {

    /// ST
    pub mod ST {
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

    /// TCW
    pub mod TCW {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (3 bits: 0b111 << 1)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// OSF
    pub mod OSF {
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

    /// TSE
    pub mod TSE {
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

    /// TXPBL
    pub mod TXPBL {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (6 bits: 0x3f << 16)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TQOS
    pub mod TQOS {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Channel 1 transmit control register
pub mod ETH_DMAC1TxCR {
    pub use super::ETH_DMAC0TxCR::OSF;
    pub use super::ETH_DMAC0TxCR::ST;
    pub use super::ETH_DMAC0TxCR::TCW;
    pub use super::ETH_DMAC0TxCR::TQOS;
    pub use super::ETH_DMAC0TxCR::TSE;
    pub use super::ETH_DMAC0TxCR::TXPBL;
}

/// Channel receive control register
pub mod ETH_DMAC0RxCR {

    /// Start or Stop Receive Command
    pub mod SR {
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

    /// Receive Buffer size
    pub mod RBSZ {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (14 bits: 0x3fff << 1)
        pub const mask: u32 = 0x3fff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RXPBL
    pub mod RXPBL {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (6 bits: 0x3f << 16)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// RQOS
    pub mod RQOS {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DMA Rx Channel Packet Flush
    pub mod RPF {
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

/// Channel i Tx descriptor list address register
pub mod ETH_DMAC0TxDLAR {

    /// Start of Transmit List
    pub mod TDESLA {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (29 bits: 0x1fffffff << 3)
        pub const mask: u32 = 0x1fffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Channel i Tx descriptor list address register
pub mod ETH_DMAC1TxDLAR {
    pub use super::ETH_DMAC0TxDLAR::TDESLA;
}

/// Channel Rx descriptor list address register
pub mod ETH_DMAC0RxDLAR {

    /// Start of Receive List
    pub mod RDESLA {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (29 bits: 0x1fffffff << 3)
        pub const mask: u32 = 0x1fffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Channel Tx descriptor tail pointer register
pub mod ETH_DMAC0TxDTPR {

    /// Transmit Descriptor Tail Pointer
    pub mod TDT {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (29 bits: 0x1fffffff << 3)
        pub const mask: u32 = 0x1fffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Channel Tx descriptor tail pointer register
pub mod ETH_DMAC1TxDTPR {
    pub use super::ETH_DMAC0TxDTPR::TDT;
}

/// Channel Rx descriptor tail pointer register
pub mod ETH_DMAC0RxDTPR {

    /// Receive Descriptor Tail Pointer
    pub mod RDT {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (29 bits: 0x1fffffff << 3)
        pub const mask: u32 = 0x1fffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Channel Tx descriptor ring length register
pub mod ETH_DMAC0TxRLR {

    /// Transmit Descriptor Ring Length
    pub mod TDRL {
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

/// Channel Tx descriptor ring length register
pub mod ETH_DMAC1TxRLR {
    pub use super::ETH_DMAC0TxRLR::TDRL;
}

/// Channel Rx descriptor ring length register
pub mod ETH_DMAC0RxRLR {

    /// Receive Descriptor Ring Length
    pub mod RDRL {
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

/// Channel interrupt enable register
pub mod ETH_DMAC0IER {

    /// Transmit Interrupt Enable
    pub mod TIE {
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

    /// Transmit Stopped Enable
    pub mod TXSE {
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

    /// Transmit Buffer Unavailable Enable
    pub mod TBUE {
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

    /// Receive Interrupt Enable
    pub mod RIE {
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

    /// Receive Buffer Unavailable Enable
    pub mod RBUE {
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

    /// Receive Stopped Enable
    pub mod RSE {
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

    /// Receive Watchdog Timeout Enable
    pub mod RWTE {
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

    /// Early Transmit Interrupt Enable
    pub mod ETIE {
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

    /// Early Receive Interrupt Enable
    pub mod ERIE {
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

    /// Fatal Bus Error Enable
    pub mod FBEE {
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

    /// Context Descriptor Error Enable
    pub mod CDEE {
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

    /// Abnormal Interrupt Summary Enable
    pub mod AIE {
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

    /// Normal Interrupt Summary Enable
    pub mod NIE {
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

/// Channel interrupt enable register
pub mod ETH_DMAC1IER {
    pub use super::ETH_DMAC0IER::AIE;
    pub use super::ETH_DMAC0IER::CDEE;
    pub use super::ETH_DMAC0IER::ERIE;
    pub use super::ETH_DMAC0IER::ETIE;
    pub use super::ETH_DMAC0IER::FBEE;
    pub use super::ETH_DMAC0IER::NIE;
    pub use super::ETH_DMAC0IER::RBUE;
    pub use super::ETH_DMAC0IER::RIE;
    pub use super::ETH_DMAC0IER::RSE;
    pub use super::ETH_DMAC0IER::RWTE;
    pub use super::ETH_DMAC0IER::TBUE;
    pub use super::ETH_DMAC0IER::TIE;
    pub use super::ETH_DMAC0IER::TXSE;
}

/// Channel Rx interrupt watchdog timer register
pub mod ETH_DMAC0RxIWTR {

    /// Receive Interrupt Watchdog Timer Count
    pub mod RWT {
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

/// Channel i slot function control status register
pub mod ETH_DMAC0SFCSR {

    /// ESC
    pub mod ESC {
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

    /// ASC
    pub mod ASC {
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

    /// RSN
    pub mod RSN {
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
}

/// Channel i slot function control status register
pub mod ETH_DMAC1SFCSR {
    pub use super::ETH_DMAC0SFCSR::ASC;
    pub use super::ETH_DMAC0SFCSR::ESC;
    pub use super::ETH_DMAC0SFCSR::RSN;
}

/// Channel current application transmit descriptor register
pub mod ETH_DMAC0CATxDR {

    /// Application Transmit Descriptor Address Pointer
    pub mod CURTDESAPTR {
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

/// Channel current application transmit descriptor register
pub mod ETH_DMAC1CATxDR {
    pub use super::ETH_DMAC0CATxDR::CURTDESAPTR;
}

/// Channel 0 current application receive descriptor register
pub mod ETH_DMAC0CARxDR {

    /// Application Transmit Descriptor Address Pointer
    pub mod CURRDESAPTR {
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

/// Channel 0 current application transmit buffer register
pub mod ETH_DMAC0CATxBR {

    /// Application Transmit Buffer Address Pointer
    pub mod CURTBUFAPTR {
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

/// Channel 0 current application transmit buffer register
pub mod ETH_DMAC1CATxBR {
    pub use super::ETH_DMAC0CATxBR::CURTBUFAPTR;
}

/// Channel current application receive buffer register
pub mod ETH_DMAC0CARxBR {

    /// Application Receive Buffer Address Pointer
    pub mod CURRBUFAPTR {
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

/// Channel status register
pub mod ETH_DMAC0SR {

    /// Transmit Interrupt
    pub mod TI {
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

    /// Transmit Process Stopped
    pub mod TPS {
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

    /// Transmit Buffer Unavailable
    pub mod TBU {
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

    /// Receive Interrupt
    pub mod RI {
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

    /// Receive Buffer Unavailable
    pub mod RBU {
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

    /// Receive Process Stopped
    pub mod RPS {
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

    /// Receive Watchdog Timeout
    pub mod RWT {
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

    /// Early Transmit Interrupt
    pub mod ETI {
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

    /// Early Receive Interrupt
    pub mod ERI {
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

    /// Fatal Bus Error
    pub mod FBE {
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

    /// Context Descriptor Error
    pub mod CDE {
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

    /// Abnormal Interrupt Summary
    pub mod AIS {
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

    /// Normal Interrupt Summary
    pub mod NIS {
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

    /// Tx DMA Error Bits
    pub mod TEB {
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

    /// Rx DMA Error Bits
    pub mod REB {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (3 bits: 0b111 << 19)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Channel status register
pub mod ETH_DMAC1SR {
    pub use super::ETH_DMAC0SR::AIS;
    pub use super::ETH_DMAC0SR::CDE;
    pub use super::ETH_DMAC0SR::ERI;
    pub use super::ETH_DMAC0SR::ETI;
    pub use super::ETH_DMAC0SR::FBE;
    pub use super::ETH_DMAC0SR::NIS;
    pub use super::ETH_DMAC0SR::RBU;
    pub use super::ETH_DMAC0SR::REB;
    pub use super::ETH_DMAC0SR::RI;
    pub use super::ETH_DMAC0SR::RPS;
    pub use super::ETH_DMAC0SR::RWT;
    pub use super::ETH_DMAC0SR::TBU;
    pub use super::ETH_DMAC0SR::TEB;
    pub use super::ETH_DMAC0SR::TI;
    pub use super::ETH_DMAC0SR::TPS;
}

/// Channel missed frame count register
pub mod ETH_DMAC0MFCR {

    /// Dropped Packet Counters
    pub mod MFC {
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

    /// Overflow status of the MFC Counter
    pub mod MFCO {
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

/// Channel missed frame count register
pub mod ETH_DMAC1MFCR {
    pub use super::ETH_DMAC0MFCR::MFC;
    pub use super::ETH_DMAC0MFCR::MFCO;
}
#[repr(C)]
pub struct RegisterBlock {
    /// DMA mode register
    pub ETH_DMAMR: RWRegister<u32>,

    /// System bus mode register
    pub ETH_DMASBMR: RWRegister<u32>,

    /// Interrupt status register
    pub ETH_DMAISR: RORegister<u32>,

    /// Debug status register
    pub ETH_DMADSR: RORegister<u32>,

    _reserved1: [u8; 16],

    /// AXI4 transmit channel ACE control register
    pub ETH_DMAA4TxACR: RWRegister<u32>,

    /// AXI4 receive channel ACE control register
    pub ETH_DMAA4RxACR: RWRegister<u32>,

    /// AXI4 descriptor ACE control register
    pub ETH_DMAA4DACR: RWRegister<u32>,

    _reserved2: [u8; 212],

    /// Channel 0 control register
    pub ETH_DMAC0CR: RWRegister<u32>,

    /// Channel 0 transmit control register
    pub ETH_DMAC0TxCR: RWRegister<u32>,

    /// Channel receive control register
    pub ETH_DMAC0RxCR: RWRegister<u32>,

    _reserved3: [u8; 8],

    /// Channel i Tx descriptor list address register
    pub ETH_DMAC0TxDLAR: RWRegister<u32>,

    _reserved4: [u8; 4],

    /// Channel Rx descriptor list address register
    pub ETH_DMAC0RxDLAR: RWRegister<u32>,

    /// Channel Tx descriptor tail pointer register
    pub ETH_DMAC0TxDTPR: RWRegister<u32>,

    _reserved5: [u8; 4],

    /// Channel Rx descriptor tail pointer register
    pub ETH_DMAC0RxDTPR: RWRegister<u32>,

    /// Channel Tx descriptor ring length register
    pub ETH_DMAC0TxRLR: RWRegister<u32>,

    /// Channel Rx descriptor ring length register
    pub ETH_DMAC0RxRLR: RWRegister<u32>,

    /// Channel interrupt enable register
    pub ETH_DMAC0IER: RWRegister<u32>,

    /// Channel Rx interrupt watchdog timer register
    pub ETH_DMAC0RxIWTR: RWRegister<u32>,

    /// Channel i slot function control status register
    pub ETH_DMAC0SFCSR: RWRegister<u32>,

    _reserved6: [u8; 4],

    /// Channel current application transmit descriptor register
    pub ETH_DMAC0CATxDR: RORegister<u32>,

    _reserved7: [u8; 4],

    /// Channel 0 current application receive descriptor register
    pub ETH_DMAC0CARxDR: RORegister<u32>,

    _reserved8: [u8; 4],

    /// Channel 0 current application transmit buffer register
    pub ETH_DMAC0CATxBR: RORegister<u32>,

    _reserved9: [u8; 4],

    /// Channel current application receive buffer register
    pub ETH_DMAC0CARxBR: RORegister<u32>,

    /// Channel status register
    pub ETH_DMAC0SR: RWRegister<u32>,

    _reserved10: [u8; 8],

    /// Channel missed frame count register
    pub ETH_DMAC0MFCR: RORegister<u32>,

    _reserved11: [u8; 16],

    /// Channel 1 control register
    pub ETH_DMAC1CR: RWRegister<u32>,

    /// Channel 1 transmit control register
    pub ETH_DMAC1TxCR: RWRegister<u32>,

    _reserved12: [u8; 12],

    /// Channel i Tx descriptor list address register
    pub ETH_DMAC1TxDLAR: RWRegister<u32>,

    _reserved13: [u8; 8],

    /// Channel Tx descriptor tail pointer register
    pub ETH_DMAC1TxDTPR: RWRegister<u32>,

    _reserved14: [u8; 8],

    /// Channel Tx descriptor ring length register
    pub ETH_DMAC1TxRLR: RWRegister<u32>,

    _reserved15: [u8; 4],

    /// Channel interrupt enable register
    pub ETH_DMAC1IER: RWRegister<u32>,

    _reserved16: [u8; 4],

    /// Channel i slot function control status register
    pub ETH_DMAC1SFCSR: RWRegister<u32>,

    _reserved17: [u8; 4],

    /// Channel current application transmit descriptor register
    pub ETH_DMAC1CATxDR: RORegister<u32>,

    _reserved18: [u8; 12],

    /// Channel 0 current application transmit buffer register
    pub ETH_DMAC1CATxBR: RORegister<u32>,

    _reserved19: [u8; 8],

    /// Channel status register
    pub ETH_DMAC1SR: RWRegister<u32>,

    _reserved20: [u8; 8],

    /// Channel missed frame count register
    pub ETH_DMAC1MFCR: RORegister<u32>,
}
pub struct ResetValues {
    pub ETH_DMAMR: u32,
    pub ETH_DMASBMR: u32,
    pub ETH_DMAISR: u32,
    pub ETH_DMADSR: u32,
    pub ETH_DMAA4TxACR: u32,
    pub ETH_DMAA4RxACR: u32,
    pub ETH_DMAA4DACR: u32,
    pub ETH_DMAC0CR: u32,
    pub ETH_DMAC0TxCR: u32,
    pub ETH_DMAC0RxCR: u32,
    pub ETH_DMAC0TxDLAR: u32,
    pub ETH_DMAC0RxDLAR: u32,
    pub ETH_DMAC0TxDTPR: u32,
    pub ETH_DMAC0RxDTPR: u32,
    pub ETH_DMAC0TxRLR: u32,
    pub ETH_DMAC0RxRLR: u32,
    pub ETH_DMAC0IER: u32,
    pub ETH_DMAC0RxIWTR: u32,
    pub ETH_DMAC0SFCSR: u32,
    pub ETH_DMAC0CATxDR: u32,
    pub ETH_DMAC0CARxDR: u32,
    pub ETH_DMAC0CATxBR: u32,
    pub ETH_DMAC0CARxBR: u32,
    pub ETH_DMAC0SR: u32,
    pub ETH_DMAC0MFCR: u32,
    pub ETH_DMAC1CR: u32,
    pub ETH_DMAC1TxCR: u32,
    pub ETH_DMAC1TxDLAR: u32,
    pub ETH_DMAC1TxDTPR: u32,
    pub ETH_DMAC1TxRLR: u32,
    pub ETH_DMAC1IER: u32,
    pub ETH_DMAC1SFCSR: u32,
    pub ETH_DMAC1CATxDR: u32,
    pub ETH_DMAC1CATxBR: u32,
    pub ETH_DMAC1SR: u32,
    pub ETH_DMAC1MFCR: u32,
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
