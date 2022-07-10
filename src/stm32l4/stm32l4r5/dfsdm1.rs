#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Digital filter for sigma delta modulators

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// channel configuration y register
pub mod CH0CFGR1 {

    /// DFSDMEN
    pub mod DFSDMEN {
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

    /// CKOUTSRC
    pub mod CKOUTSRC {
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

    /// CKOUTDIV
    pub mod CKOUTDIV {
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

    /// DATPACK
    pub mod DATPACK {
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

    /// DATMPX
    pub mod DATMPX {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (2 bits: 0b11 << 12)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CHINSEL
    pub mod CHINSEL {
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

    /// CHEN
    pub mod CHEN {
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

    /// CKABEN
    pub mod CKABEN {
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

    /// SCDEN
    pub mod SCDEN {
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

    /// SPICKSEL
    pub mod SPICKSEL {
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

    /// SITP
    pub mod SITP {
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

/// channel configuration y register
pub mod CH0CFGR2 {

    /// OFFSET
    pub mod OFFSET {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (24 bits: 0xffffff << 8)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DTRBS
    pub mod DTRBS {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (5 bits: 0b11111 << 3)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// analog watchdog and short-circuit detector register
pub mod CH0AWSCDR {

    /// AWFORD
    pub mod AWFORD {
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

    /// AWFOSR
    pub mod AWFOSR {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (5 bits: 0b11111 << 16)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// BKSCD
    pub mod BKSCD {
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

    /// SCDT
    pub mod SCDT {
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

/// channel watchdog filter data register
pub mod CH0WDATR {

    /// WDATA
    pub mod WDATA {
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

/// channel data input register
pub mod CH0DATINR {

    /// INDAT1
    pub mod INDAT1 {
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

    /// INDAT0
    pub mod INDAT0 {
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

/// channel y delay register
pub mod CH0DLYR {

    /// PLSSKP
    pub mod PLSSKP {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (6 bits: 0x3f << 0)
        pub const mask: u32 = 0x3f << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// CH1CFGR1
pub mod CH1CFGR1 {

    /// DATPACK
    pub mod DATPACK {
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

    /// DATMPX
    pub mod DATMPX {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (2 bits: 0b11 << 12)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CHINSEL
    pub mod CHINSEL {
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

    /// CHEN
    pub mod CHEN {
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

    /// CKABEN
    pub mod CKABEN {
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

    /// SCDEN
    pub mod SCDEN {
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

    /// SPICKSEL
    pub mod SPICKSEL {
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

    /// SITP
    pub mod SITP {
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

/// CH1CFGR2
pub mod CH1CFGR2 {
    pub use super::CH0CFGR2::DTRBS;
    pub use super::CH0CFGR2::OFFSET;
}

/// CH1AWSCDR
pub mod CH1AWSCDR {
    pub use super::CH0AWSCDR::AWFORD;
    pub use super::CH0AWSCDR::AWFOSR;
    pub use super::CH0AWSCDR::BKSCD;
    pub use super::CH0AWSCDR::SCDT;
}

/// CH1WDATR
pub mod CH1WDATR {
    pub use super::CH0WDATR::WDATA;
}

/// CH1DATINR
pub mod CH1DATINR {
    pub use super::CH0DATINR::INDAT0;
    pub use super::CH0DATINR::INDAT1;
}

/// channel y delay register
pub mod CH1DLYR {
    pub use super::CH0DLYR::PLSSKP;
}

/// CH2CFGR1
pub mod CH2CFGR1 {
    pub use super::CH1CFGR1::CHEN;
    pub use super::CH1CFGR1::CHINSEL;
    pub use super::CH1CFGR1::CKABEN;
    pub use super::CH1CFGR1::DATMPX;
    pub use super::CH1CFGR1::DATPACK;
    pub use super::CH1CFGR1::SCDEN;
    pub use super::CH1CFGR1::SITP;
    pub use super::CH1CFGR1::SPICKSEL;
}

/// CH2CFGR2
pub mod CH2CFGR2 {
    pub use super::CH0CFGR2::DTRBS;
    pub use super::CH0CFGR2::OFFSET;
}

/// CH2AWSCDR
pub mod CH2AWSCDR {
    pub use super::CH0AWSCDR::AWFORD;
    pub use super::CH0AWSCDR::AWFOSR;
    pub use super::CH0AWSCDR::BKSCD;
    pub use super::CH0AWSCDR::SCDT;
}

/// CH2WDATR
pub mod CH2WDATR {
    pub use super::CH0WDATR::WDATA;
}

/// CH2DATINR
pub mod CH2DATINR {
    pub use super::CH0DATINR::INDAT0;
    pub use super::CH0DATINR::INDAT1;
}

/// channel y delay register
pub mod CH2DLYR {
    pub use super::CH0DLYR::PLSSKP;
}

/// CH3CFGR1
pub mod CH3CFGR1 {
    pub use super::CH1CFGR1::CHEN;
    pub use super::CH1CFGR1::CHINSEL;
    pub use super::CH1CFGR1::CKABEN;
    pub use super::CH1CFGR1::DATMPX;
    pub use super::CH1CFGR1::DATPACK;
    pub use super::CH1CFGR1::SCDEN;
    pub use super::CH1CFGR1::SITP;
    pub use super::CH1CFGR1::SPICKSEL;
}

/// CH3CFGR2
pub mod CH3CFGR2 {
    pub use super::CH0CFGR2::DTRBS;
    pub use super::CH0CFGR2::OFFSET;
}

/// CH3AWSCDR
pub mod CH3AWSCDR {
    pub use super::CH0AWSCDR::AWFORD;
    pub use super::CH0AWSCDR::AWFOSR;
    pub use super::CH0AWSCDR::BKSCD;
    pub use super::CH0AWSCDR::SCDT;
}

/// CH3WDATR
pub mod CH3WDATR {
    pub use super::CH0WDATR::WDATA;
}

/// CH3DATINR
pub mod CH3DATINR {
    pub use super::CH0DATINR::INDAT0;
    pub use super::CH0DATINR::INDAT1;
}

/// channel y delay register
pub mod CH3DLYR {
    pub use super::CH0DLYR::PLSSKP;
}

/// CH4CFGR1
pub mod CH4CFGR1 {
    pub use super::CH1CFGR1::CHEN;
    pub use super::CH1CFGR1::CHINSEL;
    pub use super::CH1CFGR1::CKABEN;
    pub use super::CH1CFGR1::DATMPX;
    pub use super::CH1CFGR1::DATPACK;
    pub use super::CH1CFGR1::SCDEN;
    pub use super::CH1CFGR1::SITP;
    pub use super::CH1CFGR1::SPICKSEL;
}

/// CH4CFGR2
pub mod CH4CFGR2 {
    pub use super::CH0CFGR2::DTRBS;
    pub use super::CH0CFGR2::OFFSET;
}

/// CH4AWSCDR
pub mod CH4AWSCDR {
    pub use super::CH0AWSCDR::AWFORD;
    pub use super::CH0AWSCDR::AWFOSR;
    pub use super::CH0AWSCDR::BKSCD;
    pub use super::CH0AWSCDR::SCDT;
}

/// CH4WDATR
pub mod CH4WDATR {
    pub use super::CH0WDATR::WDATA;
}

/// CH4DATINR
pub mod CH4DATINR {
    pub use super::CH0DATINR::INDAT0;
    pub use super::CH0DATINR::INDAT1;
}

/// channel y delay register
pub mod CH4DLYR {
    pub use super::CH0DLYR::PLSSKP;
}

/// CH5CFGR1
pub mod CH5CFGR1 {
    pub use super::CH1CFGR1::CHEN;
    pub use super::CH1CFGR1::CHINSEL;
    pub use super::CH1CFGR1::CKABEN;
    pub use super::CH1CFGR1::DATMPX;
    pub use super::CH1CFGR1::DATPACK;
    pub use super::CH1CFGR1::SCDEN;
    pub use super::CH1CFGR1::SITP;
    pub use super::CH1CFGR1::SPICKSEL;
}

/// CH5CFGR2
pub mod CH5CFGR2 {
    pub use super::CH0CFGR2::DTRBS;
    pub use super::CH0CFGR2::OFFSET;
}

/// CH5AWSCDR
pub mod CH5AWSCDR {
    pub use super::CH0AWSCDR::AWFORD;
    pub use super::CH0AWSCDR::AWFOSR;
    pub use super::CH0AWSCDR::BKSCD;
    pub use super::CH0AWSCDR::SCDT;
}

/// CH5WDATR
pub mod CH5WDATR {
    pub use super::CH0WDATR::WDATA;
}

/// CH5DATINR
pub mod CH5DATINR {
    pub use super::CH0DATINR::INDAT0;
    pub use super::CH0DATINR::INDAT1;
}

/// channel y delay register
pub mod CH5DLYR {
    pub use super::CH0DLYR::PLSSKP;
}

/// CH6CFGR1
pub mod CH6CFGR1 {
    pub use super::CH1CFGR1::CHEN;
    pub use super::CH1CFGR1::CHINSEL;
    pub use super::CH1CFGR1::CKABEN;
    pub use super::CH1CFGR1::DATMPX;
    pub use super::CH1CFGR1::DATPACK;
    pub use super::CH1CFGR1::SCDEN;
    pub use super::CH1CFGR1::SITP;
    pub use super::CH1CFGR1::SPICKSEL;
}

/// CH6CFGR2
pub mod CH6CFGR2 {
    pub use super::CH0CFGR2::DTRBS;
    pub use super::CH0CFGR2::OFFSET;
}

/// CH6AWSCDR
pub mod CH6AWSCDR {
    pub use super::CH0AWSCDR::AWFORD;
    pub use super::CH0AWSCDR::AWFOSR;
    pub use super::CH0AWSCDR::BKSCD;
    pub use super::CH0AWSCDR::SCDT;
}

/// CH6WDATR
pub mod CH6WDATR {
    pub use super::CH0WDATR::WDATA;
}

/// CH6DATINR
pub mod CH6DATINR {
    pub use super::CH0DATINR::INDAT0;
    pub use super::CH0DATINR::INDAT1;
}

/// channel y delay register
pub mod CH6DLYR {
    pub use super::CH0DLYR::PLSSKP;
}

/// CH7CFGR1
pub mod CH7CFGR1 {
    pub use super::CH1CFGR1::CHEN;
    pub use super::CH1CFGR1::CHINSEL;
    pub use super::CH1CFGR1::CKABEN;
    pub use super::CH1CFGR1::DATMPX;
    pub use super::CH1CFGR1::DATPACK;
    pub use super::CH1CFGR1::SCDEN;
    pub use super::CH1CFGR1::SITP;
    pub use super::CH1CFGR1::SPICKSEL;
}

/// CH7CFGR2
pub mod CH7CFGR2 {
    pub use super::CH0CFGR2::DTRBS;
    pub use super::CH0CFGR2::OFFSET;
}

/// CH7AWSCDR
pub mod CH7AWSCDR {
    pub use super::CH0AWSCDR::AWFORD;
    pub use super::CH0AWSCDR::AWFOSR;
    pub use super::CH0AWSCDR::BKSCD;
    pub use super::CH0AWSCDR::SCDT;
}

/// channel y delay register
pub mod CH7DLYR {
    pub use super::CH0DLYR::PLSSKP;
}

/// control register 1
pub mod DFSDM_FLT0CR1 {

    /// Analog watchdog fast mode select
    pub mod AWFSEL {
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

    /// Fast conversion mode selection for regular conversions
    pub mod FAST {
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

    /// Regular channel selection
    pub mod RCH {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (3 bits: 0b111 << 24)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DMA channel enabled to read data for the regular conversion
    pub mod RDMAEN {
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

    /// Launch regular conversion synchronously with DFSDM0
    pub mod RSYNC {
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

    /// Continuous mode selection for regular conversions
    pub mod RCONT {
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

    /// Software start of a conversion on the regular channel
    pub mod RSWSTART {
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

    /// Trigger enable and trigger edge selection for injected conversions
    pub mod JEXTEN {
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

    /// Trigger signal selection for launching injected conversions
    pub mod JEXTSEL {
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

    /// DMA channel enabled to read data for the injected channel group
    pub mod JDMAEN {
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

    /// Scanning conversion mode for injected conversions
    pub mod JSCAN {
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

    /// Launch an injected conversion synchronously with the DFSDM0 JSWSTART trigger
    pub mod JSYNC {
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

    /// Start a conversion of the injected group of channels
    pub mod JSWSTART {
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

    /// DFSDM enable
    pub mod DFEN {
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

/// control register 2
pub mod DFSDM_FLT0CR2 {

    /// Analog watchdog channel selection
    pub mod AWDCH {
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

    /// Extremes detector channel selection
    pub mod EXCH {
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

    /// Clock absence interrupt enable
    pub mod CKABIE {
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

    /// Short-circuit detector interrupt enable
    pub mod SCDIE {
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

    /// Analog watchdog interrupt enable
    pub mod AWDIE {
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

    /// Regular data overrun interrupt enable
    pub mod ROVRIE {
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

    /// Injected data overrun interrupt enable
    pub mod JOVRIE {
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

    /// Regular end of conversion interrupt enable
    pub mod REOCIE {
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

    /// Injected end of conversion interrupt enable
    pub mod JEOCIE {
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

/// interrupt and status register
pub mod DFSDM_FLT0ISR {

    /// short-circuit detector flag
    pub mod SCDF {
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

    /// Clock absence flag
    pub mod CKABF {
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

    /// Regular conversion in progress status
    pub mod RCIP {
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

    /// Injected conversion in progress status
    pub mod JCIP {
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

    /// Analog watchdog
    pub mod AWDF {
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

    /// Regular conversion overrun flag
    pub mod ROVRF {
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

    /// Injected conversion overrun flag
    pub mod JOVRF {
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

    /// End of regular conversion flag
    pub mod REOCF {
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

    /// End of injected conversion flag
    pub mod JEOCF {
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

/// interrupt flag clear register
pub mod DFSDM_FLT0ICR {

    /// Clear the short-circuit detector flag
    pub mod CLRSCDF {
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

    /// Clear the clock absence flag
    pub mod CLRCKABF {
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

    /// Clear the regular conversion overrun flag
    pub mod CLRROVRF {
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

    /// Clear the injected conversion overrun flag
    pub mod CLRJOVRF {
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
}

/// injected channel group selection register
pub mod DFSDM_FLT0JCHGR {

    /// Injected channel group selection
    pub mod JCHG {
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

/// filter control register
pub mod DFSDM_FLT0FCR {

    /// Sinc filter order
    pub mod FORD {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (3 bits: 0b111 << 29)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Sinc filter oversampling ratio (decimation rate)
    pub mod FOSR {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (10 bits: 0x3ff << 16)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Integrator oversampling ratio (averaging length)
    pub mod IOSR {
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

/// data register for injected group
pub mod DFSDM_FLT0JDATAR {

    /// Injected group conversion data
    pub mod JDATA {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (24 bits: 0xffffff << 8)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Injected channel most recently converted
    pub mod JDATACH {
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
}

/// data register for the regular channel
pub mod DFSDM_FLT0RDATAR {

    /// Regular channel conversion data
    pub mod RDATA {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (24 bits: 0xffffff << 8)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Regular channel pending data
    pub mod RPEND {
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

    /// Regular channel most recently converted
    pub mod RDATACH {
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
}

/// analog watchdog high threshold register
pub mod DFSDM_FLT0AWHTR {

    /// Analog watchdog high threshold
    pub mod AWHT {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (24 bits: 0xffffff << 8)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Break signal assignment to analog watchdog high threshold event
    pub mod BKAWH {
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

/// analog watchdog low threshold register
pub mod DFSDM_FLT0AWLTR {

    /// Analog watchdog low threshold
    pub mod AWLT {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (24 bits: 0xffffff << 8)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Break signal assignment to analog watchdog low threshold event
    pub mod BKAWL {
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

/// analog watchdog status register
pub mod DFSDM_FLT0AWSR {

    /// Analog watchdog high threshold flag
    pub mod AWHTF {
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

    /// Analog watchdog low threshold flag
    pub mod AWLTF {
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

/// analog watchdog clear flag register
pub mod DFSDM_FLT0AWCFR {

    /// Clear the analog watchdog high threshold flag
    pub mod CLRAWHTF {
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

    /// Clear the analog watchdog low threshold flag
    pub mod CLRAWLTF {
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

/// Extremes detector maximum register
pub mod DFSDM_FLT0EXMAX {

    /// Extremes detector maximum value
    pub mod EXMAX {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (24 bits: 0xffffff << 8)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Extremes detector maximum data channel
    pub mod EXMAXCH {
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
}

/// Extremes detector minimum register
pub mod DFSDM_FLT0EXMIN {

    /// EXMIN
    pub mod EXMIN {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (24 bits: 0xffffff << 8)
        pub const mask: u32 = 0xffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Extremes detector minimum data channel
    pub mod EXMINCH {
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
}

/// conversion timer register
pub mod DFSDM_FLT0CNVTIMR {

    /// 28-bit timer counting conversion time t = CNVCNT\[27:0\] / fDFSDM_CKIN
    pub mod CNVCNT {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (28 bits: 0xfffffff << 4)
        pub const mask: u32 = 0xfffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// control register 1
pub mod DFSDM_FLT1CR1 {
    pub use super::DFSDM_FLT0CR1::AWFSEL;
    pub use super::DFSDM_FLT0CR1::DFEN;
    pub use super::DFSDM_FLT0CR1::FAST;
    pub use super::DFSDM_FLT0CR1::JDMAEN;
    pub use super::DFSDM_FLT0CR1::JEXTEN;
    pub use super::DFSDM_FLT0CR1::JEXTSEL;
    pub use super::DFSDM_FLT0CR1::JSCAN;
    pub use super::DFSDM_FLT0CR1::JSWSTART;
    pub use super::DFSDM_FLT0CR1::JSYNC;
    pub use super::DFSDM_FLT0CR1::RCH;
    pub use super::DFSDM_FLT0CR1::RCONT;
    pub use super::DFSDM_FLT0CR1::RDMAEN;
    pub use super::DFSDM_FLT0CR1::RSWSTART;
    pub use super::DFSDM_FLT0CR1::RSYNC;
}

/// control register 2
pub mod DFSDM_FLT1CR2 {
    pub use super::DFSDM_FLT0CR2::AWDCH;
    pub use super::DFSDM_FLT0CR2::AWDIE;
    pub use super::DFSDM_FLT0CR2::CKABIE;
    pub use super::DFSDM_FLT0CR2::EXCH;
    pub use super::DFSDM_FLT0CR2::JEOCIE;
    pub use super::DFSDM_FLT0CR2::JOVRIE;
    pub use super::DFSDM_FLT0CR2::REOCIE;
    pub use super::DFSDM_FLT0CR2::ROVRIE;
    pub use super::DFSDM_FLT0CR2::SCDIE;
}

/// interrupt and status register
pub mod DFSDM_FLT1ISR {
    pub use super::DFSDM_FLT0ISR::AWDF;
    pub use super::DFSDM_FLT0ISR::CKABF;
    pub use super::DFSDM_FLT0ISR::JCIP;
    pub use super::DFSDM_FLT0ISR::JEOCF;
    pub use super::DFSDM_FLT0ISR::JOVRF;
    pub use super::DFSDM_FLT0ISR::RCIP;
    pub use super::DFSDM_FLT0ISR::REOCF;
    pub use super::DFSDM_FLT0ISR::ROVRF;
    pub use super::DFSDM_FLT0ISR::SCDF;
}

/// interrupt flag clear register
pub mod DFSDM_FLT1ICR {
    pub use super::DFSDM_FLT0ICR::CLRCKABF;
    pub use super::DFSDM_FLT0ICR::CLRJOVRF;
    pub use super::DFSDM_FLT0ICR::CLRROVRF;
    pub use super::DFSDM_FLT0ICR::CLRSCDF;
}

/// injected channel group selection register
pub mod DFSDM_FLT1CHGR {
    pub use super::DFSDM_FLT0JCHGR::JCHG;
}

/// filter control register
pub mod DFSDM_FLT1FCR {
    pub use super::DFSDM_FLT0FCR::FORD;
    pub use super::DFSDM_FLT0FCR::FOSR;
    pub use super::DFSDM_FLT0FCR::IOSR;
}

/// data register for injected group
pub mod DFSDM_FLT1JDATAR {
    pub use super::DFSDM_FLT0JDATAR::JDATA;
    pub use super::DFSDM_FLT0JDATAR::JDATACH;
}

/// data register for the regular channel
pub mod DFSDM_FLT1RDATAR {
    pub use super::DFSDM_FLT0RDATAR::RDATA;
    pub use super::DFSDM_FLT0RDATAR::RDATACH;
    pub use super::DFSDM_FLT0RDATAR::RPEND;
}

/// analog watchdog high threshold register
pub mod DFSDM_FLT1AWHTR {
    pub use super::DFSDM_FLT0AWHTR::AWHT;
    pub use super::DFSDM_FLT0AWHTR::BKAWH;
}

/// analog watchdog low threshold register
pub mod DFSDM_FLT1AWLTR {
    pub use super::DFSDM_FLT0AWLTR::AWLT;
    pub use super::DFSDM_FLT0AWLTR::BKAWL;
}

/// analog watchdog status register
pub mod DFSDM_FLT1AWSR {
    pub use super::DFSDM_FLT0AWSR::AWHTF;
    pub use super::DFSDM_FLT0AWSR::AWLTF;
}

/// analog watchdog clear flag register
pub mod DFSDM_FLT1AWCFR {
    pub use super::DFSDM_FLT0AWCFR::CLRAWHTF;
    pub use super::DFSDM_FLT0AWCFR::CLRAWLTF;
}

/// Extremes detector maximum register
pub mod DFSDM_FLT1EXMAX {
    pub use super::DFSDM_FLT0EXMAX::EXMAX;
    pub use super::DFSDM_FLT0EXMAX::EXMAXCH;
}

/// Extremes detector minimum register
pub mod DFSDM_FLT1EXMIN {
    pub use super::DFSDM_FLT0EXMIN::EXMIN;
    pub use super::DFSDM_FLT0EXMIN::EXMINCH;
}

/// conversion timer register
pub mod DFSDM_FLT1CNVTIMR {
    pub use super::DFSDM_FLT0CNVTIMR::CNVCNT;
}

/// control register 1
pub mod DFSDM_FLT2CR1 {
    pub use super::DFSDM_FLT0CR1::AWFSEL;
    pub use super::DFSDM_FLT0CR1::DFEN;
    pub use super::DFSDM_FLT0CR1::FAST;
    pub use super::DFSDM_FLT0CR1::JDMAEN;
    pub use super::DFSDM_FLT0CR1::JEXTEN;
    pub use super::DFSDM_FLT0CR1::JEXTSEL;
    pub use super::DFSDM_FLT0CR1::JSCAN;
    pub use super::DFSDM_FLT0CR1::JSWSTART;
    pub use super::DFSDM_FLT0CR1::JSYNC;
    pub use super::DFSDM_FLT0CR1::RCH;
    pub use super::DFSDM_FLT0CR1::RCONT;
    pub use super::DFSDM_FLT0CR1::RDMAEN;
    pub use super::DFSDM_FLT0CR1::RSWSTART;
    pub use super::DFSDM_FLT0CR1::RSYNC;
}

/// control register 2
pub mod DFSDM_FLT2CR2 {
    pub use super::DFSDM_FLT0CR2::AWDCH;
    pub use super::DFSDM_FLT0CR2::AWDIE;
    pub use super::DFSDM_FLT0CR2::CKABIE;
    pub use super::DFSDM_FLT0CR2::EXCH;
    pub use super::DFSDM_FLT0CR2::JEOCIE;
    pub use super::DFSDM_FLT0CR2::JOVRIE;
    pub use super::DFSDM_FLT0CR2::REOCIE;
    pub use super::DFSDM_FLT0CR2::ROVRIE;
    pub use super::DFSDM_FLT0CR2::SCDIE;
}

/// interrupt and status register
pub mod DFSDM_FLT2ISR {
    pub use super::DFSDM_FLT0ISR::AWDF;
    pub use super::DFSDM_FLT0ISR::CKABF;
    pub use super::DFSDM_FLT0ISR::JCIP;
    pub use super::DFSDM_FLT0ISR::JEOCF;
    pub use super::DFSDM_FLT0ISR::JOVRF;
    pub use super::DFSDM_FLT0ISR::RCIP;
    pub use super::DFSDM_FLT0ISR::REOCF;
    pub use super::DFSDM_FLT0ISR::ROVRF;
    pub use super::DFSDM_FLT0ISR::SCDF;
}

/// interrupt flag clear register
pub mod DFSDM_FLT2ICR {
    pub use super::DFSDM_FLT0ICR::CLRCKABF;
    pub use super::DFSDM_FLT0ICR::CLRJOVRF;
    pub use super::DFSDM_FLT0ICR::CLRROVRF;
    pub use super::DFSDM_FLT0ICR::CLRSCDF;
}

/// injected channel group selection register
pub mod DFSDM_FLT2JCHGR {
    pub use super::DFSDM_FLT0JCHGR::JCHG;
}

/// filter control register
pub mod DFSDM_FLT2FCR {
    pub use super::DFSDM_FLT0FCR::FORD;
    pub use super::DFSDM_FLT0FCR::FOSR;
    pub use super::DFSDM_FLT0FCR::IOSR;
}

/// data register for injected group
pub mod DFSDM_FLT2JDATAR {
    pub use super::DFSDM_FLT0JDATAR::JDATA;
    pub use super::DFSDM_FLT0JDATAR::JDATACH;
}

/// data register for the regular channel
pub mod DFSDM_FLT2RDATAR {
    pub use super::DFSDM_FLT0RDATAR::RDATA;
    pub use super::DFSDM_FLT0RDATAR::RDATACH;
    pub use super::DFSDM_FLT0RDATAR::RPEND;
}

/// analog watchdog high threshold register
pub mod DFSDM_FLT2AWHTR {
    pub use super::DFSDM_FLT0AWHTR::AWHT;
    pub use super::DFSDM_FLT0AWHTR::BKAWH;
}

/// analog watchdog low threshold register
pub mod DFSDM_FLT2AWLTR {
    pub use super::DFSDM_FLT0AWLTR::AWLT;
    pub use super::DFSDM_FLT0AWLTR::BKAWL;
}

/// analog watchdog status register
pub mod DFSDM_FLT2AWSR {
    pub use super::DFSDM_FLT0AWSR::AWHTF;
    pub use super::DFSDM_FLT0AWSR::AWLTF;
}

/// analog watchdog clear flag register
pub mod DFSDM_FLT2AWCFR {
    pub use super::DFSDM_FLT0AWCFR::CLRAWHTF;
    pub use super::DFSDM_FLT0AWCFR::CLRAWLTF;
}

/// Extremes detector maximum register
pub mod DFSDM_FLT2EXMAX {
    pub use super::DFSDM_FLT0EXMAX::EXMAX;
    pub use super::DFSDM_FLT0EXMAX::EXMAXCH;
}

/// Extremes detector minimum register
pub mod DFSDM_FLT2EXMIN {
    pub use super::DFSDM_FLT0EXMIN::EXMIN;
    pub use super::DFSDM_FLT0EXMIN::EXMINCH;
}

/// conversion timer register
pub mod DFSDM_FLT2CNVTIMR {
    pub use super::DFSDM_FLT0CNVTIMR::CNVCNT;
}

/// control register 1
pub mod DFSDM_FLT3CR1 {
    pub use super::DFSDM_FLT0CR1::AWFSEL;
    pub use super::DFSDM_FLT0CR1::DFEN;
    pub use super::DFSDM_FLT0CR1::FAST;
    pub use super::DFSDM_FLT0CR1::JDMAEN;
    pub use super::DFSDM_FLT0CR1::JEXTEN;
    pub use super::DFSDM_FLT0CR1::JEXTSEL;
    pub use super::DFSDM_FLT0CR1::JSCAN;
    pub use super::DFSDM_FLT0CR1::JSWSTART;
    pub use super::DFSDM_FLT0CR1::JSYNC;
    pub use super::DFSDM_FLT0CR1::RCH;
    pub use super::DFSDM_FLT0CR1::RCONT;
    pub use super::DFSDM_FLT0CR1::RDMAEN;
    pub use super::DFSDM_FLT0CR1::RSWSTART;
    pub use super::DFSDM_FLT0CR1::RSYNC;
}

/// control register 2
pub mod DFSDM_FLT3CR2 {
    pub use super::DFSDM_FLT0CR2::AWDCH;
    pub use super::DFSDM_FLT0CR2::AWDIE;
    pub use super::DFSDM_FLT0CR2::CKABIE;
    pub use super::DFSDM_FLT0CR2::EXCH;
    pub use super::DFSDM_FLT0CR2::JEOCIE;
    pub use super::DFSDM_FLT0CR2::JOVRIE;
    pub use super::DFSDM_FLT0CR2::REOCIE;
    pub use super::DFSDM_FLT0CR2::ROVRIE;
    pub use super::DFSDM_FLT0CR2::SCDIE;
}

/// interrupt and status register
pub mod DFSDM_FLT3ISR {
    pub use super::DFSDM_FLT0ISR::AWDF;
    pub use super::DFSDM_FLT0ISR::CKABF;
    pub use super::DFSDM_FLT0ISR::JCIP;
    pub use super::DFSDM_FLT0ISR::JEOCF;
    pub use super::DFSDM_FLT0ISR::JOVRF;
    pub use super::DFSDM_FLT0ISR::RCIP;
    pub use super::DFSDM_FLT0ISR::REOCF;
    pub use super::DFSDM_FLT0ISR::ROVRF;
    pub use super::DFSDM_FLT0ISR::SCDF;
}

/// interrupt flag clear register
pub mod DFSDM_FLT3ICR {
    pub use super::DFSDM_FLT0ICR::CLRCKABF;
    pub use super::DFSDM_FLT0ICR::CLRJOVRF;
    pub use super::DFSDM_FLT0ICR::CLRROVRF;
    pub use super::DFSDM_FLT0ICR::CLRSCDF;
}

/// injected channel group selection register
pub mod DFSDM_FLT3JCHGR {
    pub use super::DFSDM_FLT0JCHGR::JCHG;
}

/// filter control register
pub mod DFSDM_FLT3FCR {
    pub use super::DFSDM_FLT0FCR::FORD;
    pub use super::DFSDM_FLT0FCR::FOSR;
    pub use super::DFSDM_FLT0FCR::IOSR;
}

/// data register for injected group
pub mod DFSDM_FLT3JDATAR {
    pub use super::DFSDM_FLT0JDATAR::JDATA;
    pub use super::DFSDM_FLT0JDATAR::JDATACH;
}

/// data register for the regular channel
pub mod DFSDM_FLT3RDATAR {
    pub use super::DFSDM_FLT0RDATAR::RDATA;
    pub use super::DFSDM_FLT0RDATAR::RDATACH;
    pub use super::DFSDM_FLT0RDATAR::RPEND;
}

/// analog watchdog high threshold register
pub mod DFSDM_FLT3AWHTR {
    pub use super::DFSDM_FLT0AWHTR::AWHT;
    pub use super::DFSDM_FLT0AWHTR::BKAWH;
}

/// analog watchdog low threshold register
pub mod DFSDM_FLT3AWLTR {
    pub use super::DFSDM_FLT0AWLTR::AWLT;
    pub use super::DFSDM_FLT0AWLTR::BKAWL;
}

/// analog watchdog status register
pub mod DFSDM_FLT3AWSR {
    pub use super::DFSDM_FLT0AWSR::AWHTF;
    pub use super::DFSDM_FLT0AWSR::AWLTF;
}

/// analog watchdog clear flag register
pub mod DFSDM_FLT3AWCFR {
    pub use super::DFSDM_FLT0AWCFR::CLRAWHTF;
    pub use super::DFSDM_FLT0AWCFR::CLRAWLTF;
}

/// Extremes detector maximum register
pub mod DFSDM_FLT3EXMAX {
    pub use super::DFSDM_FLT0EXMAX::EXMAX;
    pub use super::DFSDM_FLT0EXMAX::EXMAXCH;
}

/// Extremes detector minimum register
pub mod DFSDM_FLT3EXMIN {
    pub use super::DFSDM_FLT0EXMIN::EXMIN;
    pub use super::DFSDM_FLT0EXMIN::EXMINCH;
}

/// conversion timer register
pub mod DFSDM_FLT3CNVTIMR {
    pub use super::DFSDM_FLT0CNVTIMR::CNVCNT;
}

/// CH7WDATR
pub mod CH7WDATR {
    pub use super::CH0WDATR::WDATA;
}

/// CH7DATINR
pub mod CH7DATINR {
    pub use super::CH0DATINR::INDAT0;
    pub use super::CH0DATINR::INDAT1;
}
#[repr(C)]
pub struct RegisterBlock {
    /// channel configuration y register
    pub CH0CFGR1: RWRegister<u32>,

    /// channel configuration y register
    pub CH0CFGR2: RWRegister<u32>,

    /// analog watchdog and short-circuit detector register
    pub CH0AWSCDR: RWRegister<u32>,

    /// channel watchdog filter data register
    pub CH0WDATR: RWRegister<u32>,

    /// channel data input register
    pub CH0DATINR: RWRegister<u32>,

    /// channel y delay register
    pub CH0DLYR: RWRegister<u32>,

    _reserved1: [u8; 8],

    /// CH1CFGR1
    pub CH1CFGR1: RWRegister<u32>,

    /// CH1CFGR2
    pub CH1CFGR2: RWRegister<u32>,

    /// CH1AWSCDR
    pub CH1AWSCDR: RWRegister<u32>,

    /// CH1WDATR
    pub CH1WDATR: RWRegister<u32>,

    /// CH1DATINR
    pub CH1DATINR: RWRegister<u32>,

    /// channel y delay register
    pub CH1DLYR: RWRegister<u32>,

    _reserved2: [u8; 8],

    /// CH2CFGR1
    pub CH2CFGR1: RWRegister<u32>,

    /// CH2CFGR2
    pub CH2CFGR2: RWRegister<u32>,

    /// CH2AWSCDR
    pub CH2AWSCDR: RWRegister<u32>,

    /// CH2WDATR
    pub CH2WDATR: RWRegister<u32>,

    /// CH2DATINR
    pub CH2DATINR: RWRegister<u32>,

    /// channel y delay register
    pub CH2DLYR: RWRegister<u32>,

    _reserved3: [u8; 8],

    /// CH3CFGR1
    pub CH3CFGR1: RWRegister<u32>,

    /// CH3CFGR2
    pub CH3CFGR2: RWRegister<u32>,

    /// CH3AWSCDR
    pub CH3AWSCDR: RWRegister<u32>,

    /// CH3WDATR
    pub CH3WDATR: RWRegister<u32>,

    /// CH3DATINR
    pub CH3DATINR: RWRegister<u32>,

    /// channel y delay register
    pub CH3DLYR: RWRegister<u32>,

    _reserved4: [u8; 8],

    /// CH4CFGR1
    pub CH4CFGR1: RWRegister<u32>,

    /// CH4CFGR2
    pub CH4CFGR2: RWRegister<u32>,

    /// CH4AWSCDR
    pub CH4AWSCDR: RWRegister<u32>,

    /// CH4WDATR
    pub CH4WDATR: RWRegister<u32>,

    /// CH4DATINR
    pub CH4DATINR: RWRegister<u32>,

    /// channel y delay register
    pub CH4DLYR: RWRegister<u32>,

    _reserved5: [u8; 8],

    /// CH5CFGR1
    pub CH5CFGR1: RWRegister<u32>,

    /// CH5CFGR2
    pub CH5CFGR2: RWRegister<u32>,

    /// CH5AWSCDR
    pub CH5AWSCDR: RWRegister<u32>,

    /// CH5WDATR
    pub CH5WDATR: RWRegister<u32>,

    /// CH5DATINR
    pub CH5DATINR: RWRegister<u32>,

    /// channel y delay register
    pub CH5DLYR: RWRegister<u32>,

    _reserved6: [u8; 8],

    /// CH6CFGR1
    pub CH6CFGR1: RWRegister<u32>,

    /// CH6CFGR2
    pub CH6CFGR2: RWRegister<u32>,

    /// CH6AWSCDR
    pub CH6AWSCDR: RWRegister<u32>,

    /// CH6WDATR
    pub CH6WDATR: RWRegister<u32>,

    /// CH6DATINR
    pub CH6DATINR: RWRegister<u32>,

    /// channel y delay register
    pub CH6DLYR: RWRegister<u32>,

    _reserved7: [u8; 8],

    /// CH7CFGR1
    pub CH7CFGR1: RWRegister<u32>,

    /// CH7CFGR2
    pub CH7CFGR2: RWRegister<u32>,

    /// CH7AWSCDR
    pub CH7AWSCDR: RWRegister<u32>,

    /// CH7WDATR
    pub CH7WDATR: RWRegister<u32>,

    /// CH7DATINR
    pub CH7DATINR: RWRegister<u32>,

    /// channel y delay register
    pub CH7DLYR: RWRegister<u32>,

    _reserved8: [u8; 8],

    /// control register 1
    pub DFSDM_FLT0CR1: RWRegister<u32>,

    /// control register 2
    pub DFSDM_FLT0CR2: RWRegister<u32>,

    /// interrupt and status register
    pub DFSDM_FLT0ISR: RORegister<u32>,

    /// interrupt flag clear register
    pub DFSDM_FLT0ICR: RWRegister<u32>,

    /// injected channel group selection register
    pub DFSDM_FLT0JCHGR: RWRegister<u32>,

    /// filter control register
    pub DFSDM_FLT0FCR: RWRegister<u32>,

    /// data register for injected group
    pub DFSDM_FLT0JDATAR: RORegister<u32>,

    /// data register for the regular channel
    pub DFSDM_FLT0RDATAR: RORegister<u32>,

    /// analog watchdog high threshold register
    pub DFSDM_FLT0AWHTR: RWRegister<u32>,

    /// analog watchdog low threshold register
    pub DFSDM_FLT0AWLTR: RWRegister<u32>,

    /// analog watchdog status register
    pub DFSDM_FLT0AWSR: RORegister<u32>,

    /// analog watchdog clear flag register
    pub DFSDM_FLT0AWCFR: RWRegister<u32>,

    /// Extremes detector maximum register
    pub DFSDM_FLT0EXMAX: RORegister<u32>,

    /// Extremes detector minimum register
    pub DFSDM_FLT0EXMIN: RORegister<u32>,

    /// conversion timer register
    pub DFSDM_FLT0CNVTIMR: RORegister<u32>,

    _reserved9: [u8; 68],

    /// control register 1
    pub DFSDM_FLT1CR1: RWRegister<u32>,

    /// control register 2
    pub DFSDM_FLT1CR2: RWRegister<u32>,

    /// interrupt and status register
    pub DFSDM_FLT1ISR: RORegister<u32>,

    /// interrupt flag clear register
    pub DFSDM_FLT1ICR: RWRegister<u32>,

    /// injected channel group selection register
    pub DFSDM_FLT1CHGR: RWRegister<u32>,

    /// filter control register
    pub DFSDM_FLT1FCR: RWRegister<u32>,

    /// data register for injected group
    pub DFSDM_FLT1JDATAR: RORegister<u32>,

    /// data register for the regular channel
    pub DFSDM_FLT1RDATAR: RORegister<u32>,

    /// analog watchdog high threshold register
    pub DFSDM_FLT1AWHTR: RWRegister<u32>,

    /// analog watchdog low threshold register
    pub DFSDM_FLT1AWLTR: RWRegister<u32>,

    /// analog watchdog status register
    pub DFSDM_FLT1AWSR: RORegister<u32>,

    /// analog watchdog clear flag register
    pub DFSDM_FLT1AWCFR: RWRegister<u32>,

    /// Extremes detector maximum register
    pub DFSDM_FLT1EXMAX: RORegister<u32>,

    /// Extremes detector minimum register
    pub DFSDM_FLT1EXMIN: RORegister<u32>,

    /// conversion timer register
    pub DFSDM_FLT1CNVTIMR: RORegister<u32>,

    _reserved10: [u8; 68],

    /// control register 1
    pub DFSDM_FLT2CR1: RWRegister<u32>,

    /// control register 2
    pub DFSDM_FLT2CR2: RWRegister<u32>,

    /// interrupt and status register
    pub DFSDM_FLT2ISR: RORegister<u32>,

    /// interrupt flag clear register
    pub DFSDM_FLT2ICR: RWRegister<u32>,

    /// injected channel group selection register
    pub DFSDM_FLT2JCHGR: RWRegister<u32>,

    /// filter control register
    pub DFSDM_FLT2FCR: RWRegister<u32>,

    /// data register for injected group
    pub DFSDM_FLT2JDATAR: RORegister<u32>,

    /// data register for the regular channel
    pub DFSDM_FLT2RDATAR: RORegister<u32>,

    /// analog watchdog high threshold register
    pub DFSDM_FLT2AWHTR: RWRegister<u32>,

    /// analog watchdog low threshold register
    pub DFSDM_FLT2AWLTR: RWRegister<u32>,

    /// analog watchdog status register
    pub DFSDM_FLT2AWSR: RORegister<u32>,

    /// analog watchdog clear flag register
    pub DFSDM_FLT2AWCFR: RWRegister<u32>,

    /// Extremes detector maximum register
    pub DFSDM_FLT2EXMAX: RORegister<u32>,

    /// Extremes detector minimum register
    pub DFSDM_FLT2EXMIN: RORegister<u32>,

    /// conversion timer register
    pub DFSDM_FLT2CNVTIMR: RORegister<u32>,

    _reserved11: [u8; 68],

    /// control register 1
    pub DFSDM_FLT3CR1: RWRegister<u32>,

    /// control register 2
    pub DFSDM_FLT3CR2: RWRegister<u32>,

    /// interrupt and status register
    pub DFSDM_FLT3ISR: RORegister<u32>,

    /// interrupt flag clear register
    pub DFSDM_FLT3ICR: RWRegister<u32>,

    /// injected channel group selection register
    pub DFSDM_FLT3JCHGR: RWRegister<u32>,

    /// filter control register
    pub DFSDM_FLT3FCR: RWRegister<u32>,

    /// data register for injected group
    pub DFSDM_FLT3JDATAR: RORegister<u32>,

    /// data register for the regular channel
    pub DFSDM_FLT3RDATAR: RORegister<u32>,

    /// analog watchdog high threshold register
    pub DFSDM_FLT3AWHTR: RWRegister<u32>,

    /// analog watchdog low threshold register
    pub DFSDM_FLT3AWLTR: RWRegister<u32>,

    /// analog watchdog status register
    pub DFSDM_FLT3AWSR: RORegister<u32>,

    /// analog watchdog clear flag register
    pub DFSDM_FLT3AWCFR: RWRegister<u32>,

    /// Extremes detector maximum register
    pub DFSDM_FLT3EXMAX: RORegister<u32>,

    /// Extremes detector minimum register
    pub DFSDM_FLT3EXMIN: RORegister<u32>,

    /// conversion timer register
    pub DFSDM_FLT3CNVTIMR: RORegister<u32>,
}
pub struct ResetValues {
    pub CH0CFGR1: u32,
    pub CH0CFGR2: u32,
    pub CH0AWSCDR: u32,
    pub CH0WDATR: u32,
    pub CH0DATINR: u32,
    pub CH0DLYR: u32,
    pub CH1CFGR1: u32,
    pub CH1CFGR2: u32,
    pub CH1AWSCDR: u32,
    pub CH1WDATR: u32,
    pub CH1DATINR: u32,
    pub CH1DLYR: u32,
    pub CH2CFGR1: u32,
    pub CH2CFGR2: u32,
    pub CH2AWSCDR: u32,
    pub CH2WDATR: u32,
    pub CH2DATINR: u32,
    pub CH2DLYR: u32,
    pub CH3CFGR1: u32,
    pub CH3CFGR2: u32,
    pub CH3AWSCDR: u32,
    pub CH3WDATR: u32,
    pub CH3DATINR: u32,
    pub CH3DLYR: u32,
    pub CH4CFGR1: u32,
    pub CH4CFGR2: u32,
    pub CH4AWSCDR: u32,
    pub CH4WDATR: u32,
    pub CH4DATINR: u32,
    pub CH4DLYR: u32,
    pub CH5CFGR1: u32,
    pub CH5CFGR2: u32,
    pub CH5AWSCDR: u32,
    pub CH5WDATR: u32,
    pub CH5DATINR: u32,
    pub CH5DLYR: u32,
    pub CH6CFGR1: u32,
    pub CH6CFGR2: u32,
    pub CH6AWSCDR: u32,
    pub CH6WDATR: u32,
    pub CH6DATINR: u32,
    pub CH6DLYR: u32,
    pub CH7CFGR1: u32,
    pub CH7CFGR2: u32,
    pub CH7AWSCDR: u32,
    pub CH7WDATR: u32,
    pub CH7DATINR: u32,
    pub CH7DLYR: u32,
    pub DFSDM_FLT0CR1: u32,
    pub DFSDM_FLT0CR2: u32,
    pub DFSDM_FLT0ISR: u32,
    pub DFSDM_FLT0ICR: u32,
    pub DFSDM_FLT0JCHGR: u32,
    pub DFSDM_FLT0FCR: u32,
    pub DFSDM_FLT0JDATAR: u32,
    pub DFSDM_FLT0RDATAR: u32,
    pub DFSDM_FLT0AWHTR: u32,
    pub DFSDM_FLT0AWLTR: u32,
    pub DFSDM_FLT0AWSR: u32,
    pub DFSDM_FLT0AWCFR: u32,
    pub DFSDM_FLT0EXMAX: u32,
    pub DFSDM_FLT0EXMIN: u32,
    pub DFSDM_FLT0CNVTIMR: u32,
    pub DFSDM_FLT1CR1: u32,
    pub DFSDM_FLT1CR2: u32,
    pub DFSDM_FLT1ISR: u32,
    pub DFSDM_FLT1ICR: u32,
    pub DFSDM_FLT1CHGR: u32,
    pub DFSDM_FLT1FCR: u32,
    pub DFSDM_FLT1JDATAR: u32,
    pub DFSDM_FLT1RDATAR: u32,
    pub DFSDM_FLT1AWHTR: u32,
    pub DFSDM_FLT1AWLTR: u32,
    pub DFSDM_FLT1AWSR: u32,
    pub DFSDM_FLT1AWCFR: u32,
    pub DFSDM_FLT1EXMAX: u32,
    pub DFSDM_FLT1EXMIN: u32,
    pub DFSDM_FLT1CNVTIMR: u32,
    pub DFSDM_FLT2CR1: u32,
    pub DFSDM_FLT2CR2: u32,
    pub DFSDM_FLT2ISR: u32,
    pub DFSDM_FLT2ICR: u32,
    pub DFSDM_FLT2JCHGR: u32,
    pub DFSDM_FLT2FCR: u32,
    pub DFSDM_FLT2JDATAR: u32,
    pub DFSDM_FLT2RDATAR: u32,
    pub DFSDM_FLT2AWHTR: u32,
    pub DFSDM_FLT2AWLTR: u32,
    pub DFSDM_FLT2AWSR: u32,
    pub DFSDM_FLT2AWCFR: u32,
    pub DFSDM_FLT2EXMAX: u32,
    pub DFSDM_FLT2EXMIN: u32,
    pub DFSDM_FLT2CNVTIMR: u32,
    pub DFSDM_FLT3CR1: u32,
    pub DFSDM_FLT3CR2: u32,
    pub DFSDM_FLT3ISR: u32,
    pub DFSDM_FLT3ICR: u32,
    pub DFSDM_FLT3JCHGR: u32,
    pub DFSDM_FLT3FCR: u32,
    pub DFSDM_FLT3JDATAR: u32,
    pub DFSDM_FLT3RDATAR: u32,
    pub DFSDM_FLT3AWHTR: u32,
    pub DFSDM_FLT3AWLTR: u32,
    pub DFSDM_FLT3AWSR: u32,
    pub DFSDM_FLT3AWCFR: u32,
    pub DFSDM_FLT3EXMAX: u32,
    pub DFSDM_FLT3EXMIN: u32,
    pub DFSDM_FLT3CNVTIMR: u32,
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

/// Access functions for the DFSDM1 peripheral instance
pub mod DFSDM1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40016000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in DFSDM1
    pub const reset: ResetValues = ResetValues {
        CH0CFGR1: 0x00000000,
        CH0CFGR2: 0x00000000,
        CH0AWSCDR: 0x00000000,
        CH0WDATR: 0x00000000,
        CH0DATINR: 0x00000000,
        CH0DLYR: 0x00000000,
        CH1CFGR1: 0x00000000,
        CH1CFGR2: 0x00000000,
        CH1AWSCDR: 0x00000000,
        CH1WDATR: 0x00000000,
        CH1DATINR: 0x00000000,
        CH1DLYR: 0x00000000,
        CH2CFGR1: 0x00000000,
        CH2CFGR2: 0x00000000,
        CH2AWSCDR: 0x00000000,
        CH2WDATR: 0x00000000,
        CH2DATINR: 0x00000000,
        CH2DLYR: 0x00000000,
        CH3CFGR1: 0x00000000,
        CH3CFGR2: 0x00000000,
        CH3AWSCDR: 0x00000000,
        CH3WDATR: 0x00000000,
        CH3DATINR: 0x00000000,
        CH3DLYR: 0x00000000,
        CH4CFGR1: 0x00000000,
        CH4CFGR2: 0x00000000,
        CH4AWSCDR: 0x00000000,
        CH4WDATR: 0x00000000,
        CH4DATINR: 0x00000000,
        CH4DLYR: 0x00000000,
        CH5CFGR1: 0x00000000,
        CH5CFGR2: 0x00000000,
        CH5AWSCDR: 0x00000000,
        CH5WDATR: 0x00000000,
        CH5DATINR: 0x00000000,
        CH5DLYR: 0x00000000,
        CH6CFGR1: 0x00000000,
        CH6CFGR2: 0x00000000,
        CH6AWSCDR: 0x00000000,
        CH6WDATR: 0x00000000,
        CH6DATINR: 0x00000000,
        CH6DLYR: 0x00000000,
        CH7CFGR1: 0x00000000,
        CH7CFGR2: 0x00000000,
        CH7AWSCDR: 0x00000000,
        CH7DLYR: 0x00000000,
        DFSDM_FLT0CR1: 0x00000000,
        DFSDM_FLT0CR2: 0x00000000,
        DFSDM_FLT0ISR: 0x00FF0000,
        DFSDM_FLT0ICR: 0x00000000,
        DFSDM_FLT0JCHGR: 0x00000001,
        DFSDM_FLT0FCR: 0x00000000,
        DFSDM_FLT0JDATAR: 0x00000000,
        DFSDM_FLT0RDATAR: 0x00000000,
        DFSDM_FLT0AWHTR: 0x00000000,
        DFSDM_FLT0AWLTR: 0x00000000,
        DFSDM_FLT0AWSR: 0x00000000,
        DFSDM_FLT0AWCFR: 0x00000000,
        DFSDM_FLT0EXMAX: 0x80000000,
        DFSDM_FLT0EXMIN: 0x7FFFFF00,
        DFSDM_FLT0CNVTIMR: 0x00000000,
        DFSDM_FLT1CR1: 0x00000000,
        DFSDM_FLT1CR2: 0x00000000,
        DFSDM_FLT1ISR: 0x00FF0000,
        DFSDM_FLT1ICR: 0x00000000,
        DFSDM_FLT1CHGR: 0x00000001,
        DFSDM_FLT1FCR: 0x00000000,
        DFSDM_FLT1JDATAR: 0x00000000,
        DFSDM_FLT1RDATAR: 0x00000000,
        DFSDM_FLT1AWHTR: 0x00000000,
        DFSDM_FLT1AWLTR: 0x00000000,
        DFSDM_FLT1AWSR: 0x00000000,
        DFSDM_FLT1AWCFR: 0x00000000,
        DFSDM_FLT1EXMAX: 0x80000000,
        DFSDM_FLT1EXMIN: 0x7FFFFF00,
        DFSDM_FLT1CNVTIMR: 0x00000000,
        DFSDM_FLT2CR1: 0x00000000,
        DFSDM_FLT2CR2: 0x00000000,
        DFSDM_FLT2ISR: 0x00FF0000,
        DFSDM_FLT2ICR: 0x00000000,
        DFSDM_FLT2JCHGR: 0x00000001,
        DFSDM_FLT2FCR: 0x00000000,
        DFSDM_FLT2JDATAR: 0x00000000,
        DFSDM_FLT2RDATAR: 0x00000000,
        DFSDM_FLT2AWHTR: 0x00000000,
        DFSDM_FLT2AWLTR: 0x00000000,
        DFSDM_FLT2AWSR: 0x00000000,
        DFSDM_FLT2AWCFR: 0x00000000,
        DFSDM_FLT2EXMAX: 0x80000000,
        DFSDM_FLT2EXMIN: 0x7FFFFF00,
        DFSDM_FLT2CNVTIMR: 0x00000000,
        DFSDM_FLT3CR1: 0x00000000,
        DFSDM_FLT3CR2: 0x00000000,
        DFSDM_FLT3ISR: 0x00FF0000,
        DFSDM_FLT3ICR: 0x00000000,
        DFSDM_FLT3JCHGR: 0x00000001,
        DFSDM_FLT3FCR: 0x00000000,
        DFSDM_FLT3JDATAR: 0x00000000,
        DFSDM_FLT3RDATAR: 0x00000000,
        DFSDM_FLT3AWHTR: 0x00000000,
        DFSDM_FLT3AWLTR: 0x00000000,
        DFSDM_FLT3AWSR: 0x00000000,
        DFSDM_FLT3AWCFR: 0x00000000,
        DFSDM_FLT3EXMAX: 0x80000000,
        DFSDM_FLT3EXMIN: 0x7FFFFF00,
        DFSDM_FLT3CNVTIMR: 0x00000000,
        CH7WDATR: 0x00000000,
        CH7DATINR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut DFSDM1_TAKEN: bool = false;

    /// Safe access to DFSDM1
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
            if DFSDM1_TAKEN {
                None
            } else {
                DFSDM1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to DFSDM1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if DFSDM1_TAKEN && inst.addr == INSTANCE.addr {
                DFSDM1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal DFSDM1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        DFSDM1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to DFSDM1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const DFSDM1: *const RegisterBlock = 0x40016000 as *const _;
