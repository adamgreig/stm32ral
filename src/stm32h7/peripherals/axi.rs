#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! AXI interconnect registers
//!
//! Used by: stm32h735, stm32h743, stm32h743v, stm32h747cm7, stm32h753, stm32h753v, stm32h7b3

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// AXI interconnect - peripheral ID4 register
pub mod PERIPH_ID_4 {

    /// JEP106 continuation code
    pub mod JEP106CON {
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

    /// Register file size
    pub mod KCOUNT4 {
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
}

/// AXI interconnect - peripheral ID0 register
pub mod PERIPH_ID_0 {

    /// Peripheral part number bits 0 to 7
    pub mod PARTNUM {
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

/// AXI interconnect - peripheral ID1 register
pub mod PERIPH_ID_1 {

    /// Peripheral part number bits 8 to 11
    pub mod PARTNUM {
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

    /// JEP106 identity bits 0 to 3
    pub mod JEP106I {
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
}

/// AXI interconnect - peripheral ID2 register
pub mod PERIPH_ID_2 {

    /// JEP106 Identity bits 4 to 6
    pub mod JEP106ID {
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

    /// JEP106 code flag
    pub mod JEDEC {
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

    /// Peripheral revision number
    pub mod REVISION {
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
}

/// AXI interconnect - peripheral ID3 register
pub mod PERIPH_ID_3 {

    /// Customer modification
    pub mod CUST_MOD_NUM {
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

    /// Customer version
    pub mod REV_AND {
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
}

/// AXI interconnect - component ID0 register
pub mod COMP_ID_0 {

    /// Preamble bits 0 to 7
    pub mod PREAMBLE {
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

/// AXI interconnect - component ID1 register
pub mod COMP_ID_1 {

    /// Preamble bits 8 to 11
    pub mod PREAMBLE {
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

    /// Component class
    pub mod CLASS {
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
}

/// AXI interconnect - component ID2 register
pub mod COMP_ID_2 {
    pub use super::COMP_ID_0::PREAMBLE;
}

/// AXI interconnect - component ID3 register
pub mod COMP_ID_3 {
    pub use super::COMP_ID_0::PREAMBLE;
}

/// AXI interconnect - TARG x bus matrix issuing functionality register
pub mod TARG1_FN_MOD_ISS_BM {

    /// READ_ISS_OVERRIDE
    pub mod READ_ISS_OVERRIDE {
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

    /// Switch matrix write issuing override for target
    pub mod WRITE_ISS_OVERRIDE {
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

/// AXI interconnect - TARG x bus matrix issuing functionality register
pub mod TARG2_FN_MOD_ISS_BM {
    pub use super::TARG1_FN_MOD_ISS_BM::READ_ISS_OVERRIDE;
    pub use super::TARG1_FN_MOD_ISS_BM::WRITE_ISS_OVERRIDE;
}

/// AXI interconnect - TARG x bus matrix issuing functionality register
pub mod TARG3_FN_MOD_ISS_BM {
    pub use super::TARG1_FN_MOD_ISS_BM::READ_ISS_OVERRIDE;
    pub use super::TARG1_FN_MOD_ISS_BM::WRITE_ISS_OVERRIDE;
}

/// AXI interconnect - TARG x bus matrix issuing functionality register
pub mod TARG4_FN_MOD_ISS_BM {
    pub use super::TARG1_FN_MOD_ISS_BM::READ_ISS_OVERRIDE;
    pub use super::TARG1_FN_MOD_ISS_BM::WRITE_ISS_OVERRIDE;
}

/// AXI interconnect - TARG x bus matrix issuing functionality register
pub mod TARG5_FN_MOD_ISS_BM {
    pub use super::TARG1_FN_MOD_ISS_BM::READ_ISS_OVERRIDE;
    pub use super::TARG1_FN_MOD_ISS_BM::WRITE_ISS_OVERRIDE;
}

/// AXI interconnect - TARG x bus matrix issuing functionality register
pub mod TARG6_FN_MOD_ISS_BM {
    pub use super::TARG1_FN_MOD_ISS_BM::READ_ISS_OVERRIDE;
    pub use super::TARG1_FN_MOD_ISS_BM::WRITE_ISS_OVERRIDE;
}

/// AXI interconnect - TARG x bus matrix issuing functionality register
pub mod TARG7_FN_MOD_ISS_BM {
    pub use super::TARG1_FN_MOD_ISS_BM::READ_ISS_OVERRIDE;
    pub use super::TARG1_FN_MOD_ISS_BM::WRITE_ISS_OVERRIDE;
}

/// AXI interconnect - TARG x bus matrix functionality 2 register
pub mod TARG1_FN_MOD2 {

    /// Disable packing of beats to match the output data width
    pub mod BYPASS_MERGE {
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

/// AXI interconnect - TARG x bus matrix functionality 2 register
pub mod TARG2_FN_MOD2 {
    pub use super::TARG1_FN_MOD2::BYPASS_MERGE;
}

/// AXI interconnect - TARG x bus matrix functionality 2 register
pub mod TARG7_FN_MOD2 {
    pub use super::TARG1_FN_MOD2::BYPASS_MERGE;
}

/// AXI interconnect - TARG x long burst functionality modification
pub mod TARG1_FN_MOD_LB {

    /// Controls burst breaking of long bursts
    pub mod FN_MOD_LB {
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

/// AXI interconnect - TARG x long burst functionality modification
pub mod TARG2_FN_MOD_LB {
    pub use super::TARG1_FN_MOD_LB::FN_MOD_LB;
}

/// AXI interconnect - TARG x long burst functionality modification
pub mod TARG1_FN_MOD {
    pub use super::TARG1_FN_MOD_ISS_BM::READ_ISS_OVERRIDE;
    pub use super::TARG1_FN_MOD_ISS_BM::WRITE_ISS_OVERRIDE;
}

/// AXI interconnect - TARG x long burst functionality modification
pub mod TARG2_FN_MOD {
    pub use super::TARG1_FN_MOD_ISS_BM::READ_ISS_OVERRIDE;
    pub use super::TARG1_FN_MOD_ISS_BM::WRITE_ISS_OVERRIDE;
}

/// AXI interconnect - TARG x long burst functionality modification
pub mod TARG7_FN_MOD {
    pub use super::TARG1_FN_MOD_ISS_BM::READ_ISS_OVERRIDE;
    pub use super::TARG1_FN_MOD_ISS_BM::WRITE_ISS_OVERRIDE;
}

/// AXI interconnect - INI x functionality modification 2 register
pub mod INI1_FN_MOD2 {
    pub use super::TARG1_FN_MOD2::BYPASS_MERGE;
}

/// AXI interconnect - INI x functionality modification 2 register
pub mod INI3_FN_MOD2 {
    pub use super::TARG1_FN_MOD2::BYPASS_MERGE;
}

/// AXI interconnect - INI x AHB functionality modification register
pub mod INI1_FN_MOD_AHB {

    /// Converts all AHB-Lite write transactions to a series of single beat AXI
    pub mod RD_INC_OVERRIDE {
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

    /// Converts all AHB-Lite read transactions to a series of single beat AXI
    pub mod WR_INC_OVERRIDE {
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

/// AXI interconnect - INI x AHB functionality modification register
pub mod INI3_FN_MOD_AHB {
    pub use super::INI1_FN_MOD_AHB::RD_INC_OVERRIDE;
    pub use super::INI1_FN_MOD_AHB::WR_INC_OVERRIDE;
}

/// AXI interconnect - INI x read QoS register
pub mod INI1_READ_QOS {

    /// Read channel QoS setting
    pub mod AR_QOS {
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

/// AXI interconnect - INI x read QoS register
pub mod INI2_READ_QOS {
    pub use super::INI1_READ_QOS::AR_QOS;
}

/// AXI interconnect - INI x read QoS register
pub mod INI3_READ_QOS {
    pub use super::INI1_READ_QOS::AR_QOS;
}

/// AXI interconnect - INI x read QoS register
pub mod INI4_READ_QOS {
    pub use super::INI1_READ_QOS::AR_QOS;
}

/// AXI interconnect - INI x read QoS register
pub mod INI5_READ_QOS {
    pub use super::INI1_READ_QOS::AR_QOS;
}

/// AXI interconnect - INI x read QoS register
pub mod INI6_READ_QOS {
    pub use super::INI1_READ_QOS::AR_QOS;
}

/// AXI interconnect - INI x write QoS register
pub mod INI1_WRITE_QOS {

    /// Write channel QoS setting
    pub mod AW_QOS {
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

/// AXI interconnect - INI x write QoS register
pub mod INI2_WRITE_QOS {
    pub use super::INI1_WRITE_QOS::AW_QOS;
}

/// AXI interconnect - INI x write QoS register
pub mod INI3_WRITE_QOS {
    pub use super::INI1_WRITE_QOS::AW_QOS;
}

/// AXI interconnect - INI x write QoS register
pub mod INI4_WRITE_QOS {
    pub use super::INI1_WRITE_QOS::AW_QOS;
}

/// AXI interconnect - INI x write QoS register
pub mod INI5_WRITE_QOS {
    pub use super::INI1_WRITE_QOS::AW_QOS;
}

/// AXI interconnect - INI x write QoS register
pub mod INI6_WRITE_QOS {
    pub use super::INI1_WRITE_QOS::AW_QOS;
}

/// AXI interconnect - INI x issuing functionality modification register
pub mod INI1_FN_MOD {

    /// Override ASIB read issuing capability
    pub mod READ_ISS_OVERRIDE {
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

            /// 0b0: Normal ASIB read issuing capability
            pub const Normal: u32 = 0b0;

            /// 0b1: Force ASIB read issuing capability to 1
            pub const Force1: u32 = 0b1;
        }
    }

    /// Override ASIB write issuing capability
    pub mod WRITE_ISS_OVERRIDE {
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

            /// 0b0: Normal ASIB write issuing capability
            pub const Normal: u32 = 0b0;

            /// 0b1: Force ASIB write issuing capability to 1
            pub const Force1: u32 = 0b1;
        }
    }
}

/// AXI interconnect - INI x issuing functionality modification register
pub mod INI2_FN_MOD {
    pub use super::INI1_FN_MOD::READ_ISS_OVERRIDE;
    pub use super::INI1_FN_MOD::WRITE_ISS_OVERRIDE;
}

/// AXI interconnect - INI x issuing functionality modification register
pub mod INI3_FN_MOD {
    pub use super::INI1_FN_MOD::READ_ISS_OVERRIDE;
    pub use super::INI1_FN_MOD::WRITE_ISS_OVERRIDE;
}

/// AXI interconnect - INI x issuing functionality modification register
pub mod INI4_FN_MOD {
    pub use super::INI1_FN_MOD::READ_ISS_OVERRIDE;
    pub use super::INI1_FN_MOD::WRITE_ISS_OVERRIDE;
}

/// AXI interconnect - INI x issuing functionality modification register
pub mod INI5_FN_MOD {
    pub use super::INI1_FN_MOD::READ_ISS_OVERRIDE;
    pub use super::INI1_FN_MOD::WRITE_ISS_OVERRIDE;
}

/// AXI interconnect - INI x issuing functionality modification register
pub mod INI6_FN_MOD {
    pub use super::INI1_FN_MOD::READ_ISS_OVERRIDE;
    pub use super::INI1_FN_MOD::WRITE_ISS_OVERRIDE;
}
#[repr(C)]
pub struct RegisterBlock {
    _reserved1: [u32; 2036],

    /// AXI interconnect - peripheral ID4 register
    pub PERIPH_ID_4: RORegister<u32>,

    _reserved2: [u32; 3],

    /// AXI interconnect - peripheral ID0 register
    pub PERIPH_ID_0: RORegister<u32>,

    /// AXI interconnect - peripheral ID1 register
    pub PERIPH_ID_1: RORegister<u32>,

    /// AXI interconnect - peripheral ID2 register
    pub PERIPH_ID_2: RORegister<u32>,

    /// AXI interconnect - peripheral ID3 register
    pub PERIPH_ID_3: RORegister<u32>,

    /// AXI interconnect - component ID0 register
    pub COMP_ID_0: RORegister<u32>,

    /// AXI interconnect - component ID1 register
    pub COMP_ID_1: RORegister<u32>,

    /// AXI interconnect - component ID2 register
    pub COMP_ID_2: RORegister<u32>,

    /// AXI interconnect - component ID3 register
    pub COMP_ID_3: RORegister<u32>,

    _reserved3: [u32; 2],

    /// AXI interconnect - TARG x bus matrix issuing functionality register
    pub TARG1_FN_MOD_ISS_BM: RWRegister<u32>,

    _reserved4: [u32; 6],

    /// AXI interconnect - TARG x bus matrix functionality 2 register
    pub TARG1_FN_MOD2: RWRegister<u32>,

    _reserved5: [u32; 1],

    /// AXI interconnect - TARG x long burst functionality modification
    pub TARG1_FN_MOD_LB: RWRegister<u32>,

    _reserved6: [u32; 54],

    /// AXI interconnect - TARG x long burst functionality modification
    pub TARG1_FN_MOD: RWRegister<u32>,

    _reserved7: [u32; 959],

    /// AXI interconnect - TARG x bus matrix issuing functionality register
    pub TARG2_FN_MOD_ISS_BM: RWRegister<u32>,

    _reserved8: [u32; 6],

    /// AXI interconnect - TARG x bus matrix functionality 2 register
    pub TARG2_FN_MOD2: RWRegister<u32>,

    _reserved9: [u32; 1],

    /// AXI interconnect - TARG x long burst functionality modification
    pub TARG2_FN_MOD_LB: RWRegister<u32>,

    _reserved10: [u32; 54],

    /// AXI interconnect - TARG x long burst functionality modification
    pub TARG2_FN_MOD: RWRegister<u32>,

    _reserved11: [u32; 959],

    /// AXI interconnect - TARG x bus matrix issuing functionality register
    pub TARG3_FN_MOD_ISS_BM: RWRegister<u32>,

    _reserved12: [u32; 1023],

    /// AXI interconnect - TARG x bus matrix issuing functionality register
    pub TARG4_FN_MOD_ISS_BM: RWRegister<u32>,

    _reserved13: [u32; 1023],

    /// AXI interconnect - TARG x bus matrix issuing functionality register
    pub TARG5_FN_MOD_ISS_BM: RWRegister<u32>,

    _reserved14: [u32; 1023],

    /// AXI interconnect - TARG x bus matrix issuing functionality register
    pub TARG6_FN_MOD_ISS_BM: RWRegister<u32>,

    _reserved15: [u32; 1024],

    /// AXI interconnect - TARG x bus matrix issuing functionality register
    pub TARG7_FN_MOD_ISS_BM: RWRegister<u32>,

    _reserved16: [u32; 5],

    /// AXI interconnect - TARG x bus matrix functionality 2 register
    pub TARG7_FN_MOD2: RWRegister<u32>,

    _reserved17: [u32; 56],

    /// AXI interconnect - TARG x long burst functionality modification
    pub TARG7_FN_MOD: RWRegister<u32>,

    _reserved18: [u32; 59334],

    /// AXI interconnect - INI x functionality modification 2 register
    pub INI1_FN_MOD2: RWRegister<u32>,

    /// AXI interconnect - INI x AHB functionality modification register
    pub INI1_FN_MOD_AHB: RWRegister<u32>,

    _reserved19: [u32; 53],

    /// AXI interconnect - INI x read QoS register
    pub INI1_READ_QOS: RWRegister<u32>,

    /// AXI interconnect - INI x write QoS register
    pub INI1_WRITE_QOS: RWRegister<u32>,

    /// AXI interconnect - INI x issuing functionality modification register
    pub INI1_FN_MOD: RWRegister<u32>,

    _reserved20: [u32; 1021],

    /// AXI interconnect - INI x read QoS register
    pub INI2_READ_QOS: RWRegister<u32>,

    /// AXI interconnect - INI x write QoS register
    pub INI2_WRITE_QOS: RWRegister<u32>,

    /// AXI interconnect - INI x issuing functionality modification register
    pub INI2_FN_MOD: RWRegister<u32>,

    _reserved21: [u32; 966],

    /// AXI interconnect - INI x functionality modification 2 register
    pub INI3_FN_MOD2: RWRegister<u32>,

    /// AXI interconnect - INI x AHB functionality modification register
    pub INI3_FN_MOD_AHB: RWRegister<u32>,

    _reserved22: [u32; 53],

    /// AXI interconnect - INI x read QoS register
    pub INI3_READ_QOS: RWRegister<u32>,

    /// AXI interconnect - INI x write QoS register
    pub INI3_WRITE_QOS: RWRegister<u32>,

    /// AXI interconnect - INI x issuing functionality modification register
    pub INI3_FN_MOD: RWRegister<u32>,

    _reserved23: [u32; 1021],

    /// AXI interconnect - INI x read QoS register
    pub INI4_READ_QOS: RWRegister<u32>,

    /// AXI interconnect - INI x write QoS register
    pub INI4_WRITE_QOS: RWRegister<u32>,

    /// AXI interconnect - INI x issuing functionality modification register
    pub INI4_FN_MOD: RWRegister<u32>,

    _reserved24: [u32; 1021],

    /// AXI interconnect - INI x read QoS register
    pub INI5_READ_QOS: RWRegister<u32>,

    /// AXI interconnect - INI x write QoS register
    pub INI5_WRITE_QOS: RWRegister<u32>,

    /// AXI interconnect - INI x issuing functionality modification register
    pub INI5_FN_MOD: RWRegister<u32>,

    _reserved25: [u32; 1021],

    /// AXI interconnect - INI x read QoS register
    pub INI6_READ_QOS: RWRegister<u32>,

    /// AXI interconnect - INI x write QoS register
    pub INI6_WRITE_QOS: RWRegister<u32>,

    /// AXI interconnect - INI x issuing functionality modification register
    pub INI6_FN_MOD: RWRegister<u32>,
}
pub struct ResetValues {
    pub PERIPH_ID_4: u32,
    pub PERIPH_ID_0: u32,
    pub PERIPH_ID_1: u32,
    pub PERIPH_ID_2: u32,
    pub PERIPH_ID_3: u32,
    pub COMP_ID_0: u32,
    pub COMP_ID_1: u32,
    pub COMP_ID_2: u32,
    pub COMP_ID_3: u32,
    pub TARG1_FN_MOD_ISS_BM: u32,
    pub TARG1_FN_MOD2: u32,
    pub TARG1_FN_MOD_LB: u32,
    pub TARG1_FN_MOD: u32,
    pub TARG2_FN_MOD_ISS_BM: u32,
    pub TARG2_FN_MOD2: u32,
    pub TARG2_FN_MOD_LB: u32,
    pub TARG2_FN_MOD: u32,
    pub TARG3_FN_MOD_ISS_BM: u32,
    pub TARG4_FN_MOD_ISS_BM: u32,
    pub TARG5_FN_MOD_ISS_BM: u32,
    pub TARG6_FN_MOD_ISS_BM: u32,
    pub TARG7_FN_MOD_ISS_BM: u32,
    pub TARG7_FN_MOD2: u32,
    pub TARG7_FN_MOD: u32,
    pub INI1_FN_MOD2: u32,
    pub INI1_FN_MOD_AHB: u32,
    pub INI1_READ_QOS: u32,
    pub INI1_WRITE_QOS: u32,
    pub INI1_FN_MOD: u32,
    pub INI2_READ_QOS: u32,
    pub INI2_WRITE_QOS: u32,
    pub INI2_FN_MOD: u32,
    pub INI3_FN_MOD2: u32,
    pub INI3_FN_MOD_AHB: u32,
    pub INI3_READ_QOS: u32,
    pub INI3_WRITE_QOS: u32,
    pub INI3_FN_MOD: u32,
    pub INI4_READ_QOS: u32,
    pub INI4_WRITE_QOS: u32,
    pub INI4_FN_MOD: u32,
    pub INI5_READ_QOS: u32,
    pub INI5_WRITE_QOS: u32,
    pub INI5_FN_MOD: u32,
    pub INI6_READ_QOS: u32,
    pub INI6_WRITE_QOS: u32,
    pub INI6_FN_MOD: u32,
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
