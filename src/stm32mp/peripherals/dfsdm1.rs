#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DFSDM1
//!
//! Used by: stm32mp153, stm32mp157

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// This register specifies the parameters used by channel y.
pub mod DFSDM_CH0CFGR1 {

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
}

/// This register specifies the parameters used by channel y.
pub mod DFSDM_CH0CFGR2 {

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
}

/// Short-circuit detector and analog watchdog settings for channel y.
pub mod DFSDM_CH0AWSCDR {

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
}

/// This register contains the data resulting from the analog watchdog filter associated to the input channel y.
pub mod DFSDM_CH0WDATR {

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

/// This register contains 16-bit input data to be processed by DFSDM filter module.
pub mod DFSDM_CH0DATINR {

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
}

/// DFSDM channel 0 delay register
pub mod DFSDM_CH0DLYR {

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

/// This register specifies the parameters used by channel y.
pub mod DFSDM_CH1CFGR1 {
    pub use super::DFSDM_CH0CFGR1::CHEN;
    pub use super::DFSDM_CH0CFGR1::CHINSEL;
    pub use super::DFSDM_CH0CFGR1::CKABEN;
    pub use super::DFSDM_CH0CFGR1::CKOUTDIV;
    pub use super::DFSDM_CH0CFGR1::CKOUTSRC;
    pub use super::DFSDM_CH0CFGR1::DATMPX;
    pub use super::DFSDM_CH0CFGR1::DATPACK;
    pub use super::DFSDM_CH0CFGR1::DFSDMEN;
    pub use super::DFSDM_CH0CFGR1::SCDEN;
    pub use super::DFSDM_CH0CFGR1::SITP;
    pub use super::DFSDM_CH0CFGR1::SPICKSEL;
}

/// This register specifies the parameters used by channel y.
pub mod DFSDM_CH1CFGR2 {
    pub use super::DFSDM_CH0CFGR2::DTRBS;
    pub use super::DFSDM_CH0CFGR2::OFFSET;
}

/// Short-circuit detector and analog watchdog settings for channel y.
pub mod DFSDM_CH1AWSCDR {
    pub use super::DFSDM_CH0AWSCDR::AWFORD;
    pub use super::DFSDM_CH0AWSCDR::AWFOSR;
    pub use super::DFSDM_CH0AWSCDR::BKSCD;
    pub use super::DFSDM_CH0AWSCDR::SCDT;
}

/// This register contains the data resulting from the analog watchdog filter associated to the input channel y.
pub mod DFSDM_CH1WDATR {
    pub use super::DFSDM_CH0WDATR::WDATA;
}

/// This register contains 16-bit input data to be processed by DFSDM filter module.
pub mod DFSDM_CH1DATINR {
    pub use super::DFSDM_CH0DATINR::INDAT0;
    pub use super::DFSDM_CH0DATINR::INDAT1;
}

/// DFSDM channel 1 delay register
pub mod DFSDM_CH1DLYR {
    pub use super::DFSDM_CH0DLYR::PLSSKP;
}

/// This register specifies the parameters used by channel y.
pub mod DFSDM_CH2CFGR1 {
    pub use super::DFSDM_CH0CFGR1::CHEN;
    pub use super::DFSDM_CH0CFGR1::CHINSEL;
    pub use super::DFSDM_CH0CFGR1::CKABEN;
    pub use super::DFSDM_CH0CFGR1::CKOUTDIV;
    pub use super::DFSDM_CH0CFGR1::CKOUTSRC;
    pub use super::DFSDM_CH0CFGR1::DATMPX;
    pub use super::DFSDM_CH0CFGR1::DATPACK;
    pub use super::DFSDM_CH0CFGR1::DFSDMEN;
    pub use super::DFSDM_CH0CFGR1::SCDEN;
    pub use super::DFSDM_CH0CFGR1::SITP;
    pub use super::DFSDM_CH0CFGR1::SPICKSEL;
}

/// This register specifies the parameters used by channel y.
pub mod DFSDM_CH2CFGR2 {
    pub use super::DFSDM_CH0CFGR2::DTRBS;
    pub use super::DFSDM_CH0CFGR2::OFFSET;
}

/// Short-circuit detector and analog watchdog settings for channel y.
pub mod DFSDM_CH2AWSCDR {
    pub use super::DFSDM_CH0AWSCDR::AWFORD;
    pub use super::DFSDM_CH0AWSCDR::AWFOSR;
    pub use super::DFSDM_CH0AWSCDR::BKSCD;
    pub use super::DFSDM_CH0AWSCDR::SCDT;
}

/// This register contains the data resulting from the analog watchdog filter associated to the input channel y.
pub mod DFSDM_CH2WDATR {
    pub use super::DFSDM_CH0WDATR::WDATA;
}

/// This register contains 16-bit input data to be processed by DFSDM filter module.
pub mod DFSDM_CH2DATINR {
    pub use super::DFSDM_CH0DATINR::INDAT0;
    pub use super::DFSDM_CH0DATINR::INDAT1;
}

/// DFSDM channel 2 delay register
pub mod DFSDM_CH2DLYR {
    pub use super::DFSDM_CH0DLYR::PLSSKP;
}

/// This register specifies the parameters used by channel y.
pub mod DFSDM_CH3CFGR1 {
    pub use super::DFSDM_CH0CFGR1::CHEN;
    pub use super::DFSDM_CH0CFGR1::CHINSEL;
    pub use super::DFSDM_CH0CFGR1::CKABEN;
    pub use super::DFSDM_CH0CFGR1::CKOUTDIV;
    pub use super::DFSDM_CH0CFGR1::CKOUTSRC;
    pub use super::DFSDM_CH0CFGR1::DATMPX;
    pub use super::DFSDM_CH0CFGR1::DATPACK;
    pub use super::DFSDM_CH0CFGR1::DFSDMEN;
    pub use super::DFSDM_CH0CFGR1::SCDEN;
    pub use super::DFSDM_CH0CFGR1::SITP;
    pub use super::DFSDM_CH0CFGR1::SPICKSEL;
}

/// This register specifies the parameters used by channel y.
pub mod DFSDM_CH3CFGR2 {
    pub use super::DFSDM_CH0CFGR2::DTRBS;
    pub use super::DFSDM_CH0CFGR2::OFFSET;
}

/// Short-circuit detector and analog watchdog settings for channel y.
pub mod DFSDM_CH3AWSCDR {
    pub use super::DFSDM_CH0AWSCDR::AWFORD;
    pub use super::DFSDM_CH0AWSCDR::AWFOSR;
    pub use super::DFSDM_CH0AWSCDR::BKSCD;
    pub use super::DFSDM_CH0AWSCDR::SCDT;
}

/// This register contains the data resulting from the analog watchdog filter associated to the input channel y.
pub mod DFSDM_CH3WDATR {
    pub use super::DFSDM_CH0WDATR::WDATA;
}

/// This register contains 16-bit input data to be processed by DFSDM filter module.
pub mod DFSDM_CH3DATINR {
    pub use super::DFSDM_CH0DATINR::INDAT0;
    pub use super::DFSDM_CH0DATINR::INDAT1;
}

/// DFSDM channel 3 delay register
pub mod DFSDM_CH3DLYR {
    pub use super::DFSDM_CH0DLYR::PLSSKP;
}

/// This register specifies the parameters used by channel y.
pub mod DFSDM_CH4CFGR1 {
    pub use super::DFSDM_CH0CFGR1::CHEN;
    pub use super::DFSDM_CH0CFGR1::CHINSEL;
    pub use super::DFSDM_CH0CFGR1::CKABEN;
    pub use super::DFSDM_CH0CFGR1::CKOUTDIV;
    pub use super::DFSDM_CH0CFGR1::CKOUTSRC;
    pub use super::DFSDM_CH0CFGR1::DATMPX;
    pub use super::DFSDM_CH0CFGR1::DATPACK;
    pub use super::DFSDM_CH0CFGR1::DFSDMEN;
    pub use super::DFSDM_CH0CFGR1::SCDEN;
    pub use super::DFSDM_CH0CFGR1::SITP;
    pub use super::DFSDM_CH0CFGR1::SPICKSEL;
}

/// This register specifies the parameters used by channel y.
pub mod DFSDM_CH4CFGR2 {
    pub use super::DFSDM_CH0CFGR2::DTRBS;
    pub use super::DFSDM_CH0CFGR2::OFFSET;
}

/// Short-circuit detector and analog watchdog settings for channel y.
pub mod DFSDM_CH4AWSCDR {
    pub use super::DFSDM_CH0AWSCDR::AWFORD;
    pub use super::DFSDM_CH0AWSCDR::AWFOSR;
    pub use super::DFSDM_CH0AWSCDR::BKSCD;
    pub use super::DFSDM_CH0AWSCDR::SCDT;
}

/// This register contains the data resulting from the analog watchdog filter associated to the input channel y.
pub mod DFSDM_CH4WDATR {
    pub use super::DFSDM_CH0WDATR::WDATA;
}

/// This register contains 16-bit input data to be processed by DFSDM filter module.
pub mod DFSDM_CH4DATINR {
    pub use super::DFSDM_CH0DATINR::INDAT0;
    pub use super::DFSDM_CH0DATINR::INDAT1;
}

/// DFSDM channel 4 delay register
pub mod DFSDM_CH4DLYR {
    pub use super::DFSDM_CH0DLYR::PLSSKP;
}

/// This register specifies the parameters used by channel y.
pub mod DFSDM_CH5CFGR1 {
    pub use super::DFSDM_CH0CFGR1::CHEN;
    pub use super::DFSDM_CH0CFGR1::CHINSEL;
    pub use super::DFSDM_CH0CFGR1::CKABEN;
    pub use super::DFSDM_CH0CFGR1::CKOUTDIV;
    pub use super::DFSDM_CH0CFGR1::CKOUTSRC;
    pub use super::DFSDM_CH0CFGR1::DATMPX;
    pub use super::DFSDM_CH0CFGR1::DATPACK;
    pub use super::DFSDM_CH0CFGR1::DFSDMEN;
    pub use super::DFSDM_CH0CFGR1::SCDEN;
    pub use super::DFSDM_CH0CFGR1::SITP;
    pub use super::DFSDM_CH0CFGR1::SPICKSEL;
}

/// This register specifies the parameters used by channel y.
pub mod DFSDM_CH5CFGR2 {
    pub use super::DFSDM_CH0CFGR2::DTRBS;
    pub use super::DFSDM_CH0CFGR2::OFFSET;
}

/// Short-circuit detector and analog watchdog settings for channel y.
pub mod DFSDM_CH5AWSCDR {
    pub use super::DFSDM_CH0AWSCDR::AWFORD;
    pub use super::DFSDM_CH0AWSCDR::AWFOSR;
    pub use super::DFSDM_CH0AWSCDR::BKSCD;
    pub use super::DFSDM_CH0AWSCDR::SCDT;
}

/// This register contains the data resulting from the analog watchdog filter associated to the input channel y.
pub mod DFSDM_CH5WDATR {
    pub use super::DFSDM_CH0WDATR::WDATA;
}

/// This register contains 16-bit input data to be processed by DFSDM filter module.
pub mod DFSDM_CH5DATINR {
    pub use super::DFSDM_CH0DATINR::INDAT0;
    pub use super::DFSDM_CH0DATINR::INDAT1;
}

/// DFSDM channel 5 delay register
pub mod DFSDM_CH5DLYR {
    pub use super::DFSDM_CH0DLYR::PLSSKP;
}

/// This register specifies the parameters used by channel y.
pub mod DFSDM_CH6CFGR1 {
    pub use super::DFSDM_CH0CFGR1::CHEN;
    pub use super::DFSDM_CH0CFGR1::CHINSEL;
    pub use super::DFSDM_CH0CFGR1::CKABEN;
    pub use super::DFSDM_CH0CFGR1::CKOUTDIV;
    pub use super::DFSDM_CH0CFGR1::CKOUTSRC;
    pub use super::DFSDM_CH0CFGR1::DATMPX;
    pub use super::DFSDM_CH0CFGR1::DATPACK;
    pub use super::DFSDM_CH0CFGR1::DFSDMEN;
    pub use super::DFSDM_CH0CFGR1::SCDEN;
    pub use super::DFSDM_CH0CFGR1::SITP;
    pub use super::DFSDM_CH0CFGR1::SPICKSEL;
}

/// This register specifies the parameters used by channel y.
pub mod DFSDM_CH6CFGR2 {
    pub use super::DFSDM_CH0CFGR2::DTRBS;
    pub use super::DFSDM_CH0CFGR2::OFFSET;
}

/// Short-circuit detector and analog watchdog settings for channel y.
pub mod DFSDM_CH6AWSCDR {
    pub use super::DFSDM_CH0AWSCDR::AWFORD;
    pub use super::DFSDM_CH0AWSCDR::AWFOSR;
    pub use super::DFSDM_CH0AWSCDR::BKSCD;
    pub use super::DFSDM_CH0AWSCDR::SCDT;
}

/// This register contains the data resulting from the analog watchdog filter associated to the input channel y.
pub mod DFSDM_CH6WDATR {
    pub use super::DFSDM_CH0WDATR::WDATA;
}

/// This register contains 16-bit input data to be processed by DFSDM filter module.
pub mod DFSDM_CH6DATINR {
    pub use super::DFSDM_CH0DATINR::INDAT0;
    pub use super::DFSDM_CH0DATINR::INDAT1;
}

/// DFSDM channel 6 delay register
pub mod DFSDM_CH6DLYR {
    pub use super::DFSDM_CH0DLYR::PLSSKP;
}

/// This register specifies the parameters used by channel y.
pub mod DFSDM_CH7CFGR1 {
    pub use super::DFSDM_CH0CFGR1::CHEN;
    pub use super::DFSDM_CH0CFGR1::CHINSEL;
    pub use super::DFSDM_CH0CFGR1::CKABEN;
    pub use super::DFSDM_CH0CFGR1::CKOUTDIV;
    pub use super::DFSDM_CH0CFGR1::CKOUTSRC;
    pub use super::DFSDM_CH0CFGR1::DATMPX;
    pub use super::DFSDM_CH0CFGR1::DATPACK;
    pub use super::DFSDM_CH0CFGR1::DFSDMEN;
    pub use super::DFSDM_CH0CFGR1::SCDEN;
    pub use super::DFSDM_CH0CFGR1::SITP;
    pub use super::DFSDM_CH0CFGR1::SPICKSEL;
}

/// This register specifies the parameters used by channel y.
pub mod DFSDM_CH7CFGR2 {
    pub use super::DFSDM_CH0CFGR2::DTRBS;
    pub use super::DFSDM_CH0CFGR2::OFFSET;
}

/// Short-circuit detector and analog watchdog settings for channel y.
pub mod DFSDM_CH7AWSCDR {
    pub use super::DFSDM_CH0AWSCDR::AWFORD;
    pub use super::DFSDM_CH0AWSCDR::AWFOSR;
    pub use super::DFSDM_CH0AWSCDR::BKSCD;
    pub use super::DFSDM_CH0AWSCDR::SCDT;
}

/// This register contains the data resulting from the analog watchdog filter associated to the input channel y.
pub mod DFSDM_CH7WDATR {
    pub use super::DFSDM_CH0WDATR::WDATA;
}

/// This register contains 16-bit input data to be processed by DFSDM filter module.
pub mod DFSDM_CH7DATINR {
    pub use super::DFSDM_CH0DATINR::INDAT0;
    pub use super::DFSDM_CH0DATINR::INDAT1;
}

/// DFSDM channel 7 delay register
pub mod DFSDM_CH7DLYR {
    pub use super::DFSDM_CH0DLYR::PLSSKP;
}

/// DFSDM filter 0 control register 1
pub mod DFSDM_FLT0CR1 {

    /// DFEN
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

    /// JSWSTART
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

    /// JSYNC
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

    /// JSCAN
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

    /// JDMAEN
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

    /// JEXTSEL
    pub mod JEXTSEL {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (5 bits: 0b11111 << 8)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// JEXTEN
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

    /// RSWSTART
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

    /// RCONT
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

    /// RSYNC
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

    /// RDMAEN
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

    /// RCH
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

    /// FAST
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

    /// AWFSEL
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
}

/// DFSDM filter 0 control register 2
pub mod DFSDM_FLT0CR2 {

    /// JEOCIE
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

    /// REOCIE
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

    /// JOVRIE
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

    /// ROVRIE
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

    /// AWDIE
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

    /// SCDIE
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

    /// CKABIE
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

    /// EXCH
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

    /// AWDCH
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
}

/// DFSDM filter 0 interrupt and status register
pub mod DFSDM_FLT0ISR {

    /// JEOCF
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

    /// REOCF
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

    /// JOVRF
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

    /// ROVRF
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

    /// AWDF
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

    /// JCIP
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

    /// RCIP
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

    /// CKABF
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

    /// SCDF
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
}

/// DFSDM filter 0 interrupt flag clear register
pub mod DFSDM_FLT0ICR {

    /// CLRJOVRF
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

    /// CLRROVRF
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

    /// CLRCKABF
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

    /// CLRSCDF
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
}

/// DFSDM filter 0 injected channel group selection register
pub mod DFSDM_FLT0JCHGR {

    /// JCHG
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

/// DFSDM filter 0 control register
pub mod DFSDM_FLT0FCR {

    /// IOSR
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

    /// FOSR
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

    /// FORD
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
}

/// DFSDM filter 0 data register for injected group
pub mod DFSDM_FLT0JDATAR {

    /// JDATACH
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

    /// JDATA
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
}

/// DFSDM filter 0 data register for the regular channel
pub mod DFSDM_FLT0RDATAR {

    /// RDATACH
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

    /// RPEND
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

    /// RDATA
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
}

/// DFSDM filter 0 analog watchdog high threshold register
pub mod DFSDM_FLT0AWHTR {

    /// BKAWH
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

    /// AWHT
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
}

/// DFSDM filter 0 analog watchdog low threshold register
pub mod DFSDM_FLT0AWLTR {

    /// BKAWL
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

    /// AWLT
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
}

/// DFSDM filter 0 analog watchdog status register
pub mod DFSDM_FLT0AWSR {

    /// AWLTF
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

    /// AWHTF
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
}

/// DFSDM filter 0 analog watchdog clear flag register
pub mod DFSDM_FLT0AWCFR {

    /// CLRAWLTF
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

    /// CLRAWHTF
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
}

/// DFSDM filter 0 extremes detector maximum register
pub mod DFSDM_FLT0EXMAX {

    /// EXMAXCH
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

    /// EXMAX
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
}

/// DFSDM filter 0 extremes detector minimum register
pub mod DFSDM_FLT0EXMIN {

    /// EXMINCH
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
}

/// DFSDM filter 0 conversion timer register
pub mod DFSDM_FLT0CNVTIMR {

    /// CNVCNT
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

/// DFSDM filter 1 control register 1
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

/// DFSDM filter 1 control register 2
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

/// DFSDM filter 1 interrupt and status register
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

/// DFSDM filter 1 interrupt flag clear register
pub mod DFSDM_FLT1ICR {
    pub use super::DFSDM_FLT0ICR::CLRCKABF;
    pub use super::DFSDM_FLT0ICR::CLRJOVRF;
    pub use super::DFSDM_FLT0ICR::CLRROVRF;
    pub use super::DFSDM_FLT0ICR::CLRSCDF;
}

/// DFSDM filter 1 injected channel group selection register
pub mod DFSDM_FLT1JCHGR {
    pub use super::DFSDM_FLT0JCHGR::JCHG;
}

/// DFSDM filter 1 control register
pub mod DFSDM_FLT1FCR {
    pub use super::DFSDM_FLT0FCR::FORD;
    pub use super::DFSDM_FLT0FCR::FOSR;
    pub use super::DFSDM_FLT0FCR::IOSR;
}

/// DFSDM filter 1 data register for injected group
pub mod DFSDM_FLT1JDATAR {
    pub use super::DFSDM_FLT0JDATAR::JDATA;
    pub use super::DFSDM_FLT0JDATAR::JDATACH;
}

/// DFSDM filter 1 data register for the regular channel
pub mod DFSDM_FLT1RDATAR {
    pub use super::DFSDM_FLT0RDATAR::RDATA;
    pub use super::DFSDM_FLT0RDATAR::RDATACH;
    pub use super::DFSDM_FLT0RDATAR::RPEND;
}

/// DFSDM filter 1 analog watchdog high threshold register
pub mod DFSDM_FLT1AWHTR {
    pub use super::DFSDM_FLT0AWHTR::AWHT;
    pub use super::DFSDM_FLT0AWHTR::BKAWH;
}

/// DFSDM filter 1 analog watchdog low threshold register
pub mod DFSDM_FLT1AWLTR {
    pub use super::DFSDM_FLT0AWLTR::AWLT;
    pub use super::DFSDM_FLT0AWLTR::BKAWL;
}

/// DFSDM filter 1 analog watchdog status register
pub mod DFSDM_FLT1AWSR {
    pub use super::DFSDM_FLT0AWSR::AWHTF;
    pub use super::DFSDM_FLT0AWSR::AWLTF;
}

/// DFSDM filter 1 analog watchdog clear flag register
pub mod DFSDM_FLT1AWCFR {
    pub use super::DFSDM_FLT0AWCFR::CLRAWHTF;
    pub use super::DFSDM_FLT0AWCFR::CLRAWLTF;
}

/// DFSDM filter 1 extremes detector maximum register
pub mod DFSDM_FLT1EXMAX {
    pub use super::DFSDM_FLT0EXMAX::EXMAX;
    pub use super::DFSDM_FLT0EXMAX::EXMAXCH;
}

/// DFSDM filter 1 extremes detector minimum register
pub mod DFSDM_FLT1EXMIN {
    pub use super::DFSDM_FLT0EXMIN::EXMIN;
    pub use super::DFSDM_FLT0EXMIN::EXMINCH;
}

/// DFSDM filter 1 conversion timer register
pub mod DFSDM_FLT1CNVTIMR {
    pub use super::DFSDM_FLT0CNVTIMR::CNVCNT;
}

/// DFSDM filter 2 control register 1
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

/// DFSDM filter 2 control register 2
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

/// DFSDM filter 2 interrupt and status register
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

/// DFSDM filter 2 interrupt flag clear register
pub mod DFSDM_FLT2ICR {
    pub use super::DFSDM_FLT0ICR::CLRCKABF;
    pub use super::DFSDM_FLT0ICR::CLRJOVRF;
    pub use super::DFSDM_FLT0ICR::CLRROVRF;
    pub use super::DFSDM_FLT0ICR::CLRSCDF;
}

/// DFSDM filter 2 injected channel group selection register
pub mod DFSDM_FLT2JCHGR {
    pub use super::DFSDM_FLT0JCHGR::JCHG;
}

/// DFSDM filter 2 control register
pub mod DFSDM_FLT2FCR {
    pub use super::DFSDM_FLT0FCR::FORD;
    pub use super::DFSDM_FLT0FCR::FOSR;
    pub use super::DFSDM_FLT0FCR::IOSR;
}

/// DFSDM filter 2 data register for injected group
pub mod DFSDM_FLT2JDATAR {
    pub use super::DFSDM_FLT0JDATAR::JDATA;
    pub use super::DFSDM_FLT0JDATAR::JDATACH;
}

/// DFSDM filter 2 data register for the regular channel
pub mod DFSDM_FLT2RDATAR {
    pub use super::DFSDM_FLT0RDATAR::RDATA;
    pub use super::DFSDM_FLT0RDATAR::RDATACH;
    pub use super::DFSDM_FLT0RDATAR::RPEND;
}

/// DFSDM filter 2 analog watchdog high threshold register
pub mod DFSDM_FLT2AWHTR {
    pub use super::DFSDM_FLT0AWHTR::AWHT;
    pub use super::DFSDM_FLT0AWHTR::BKAWH;
}

/// DFSDM filter 2 analog watchdog low threshold register
pub mod DFSDM_FLT2AWLTR {
    pub use super::DFSDM_FLT0AWLTR::AWLT;
    pub use super::DFSDM_FLT0AWLTR::BKAWL;
}

/// DFSDM filter 2 analog watchdog status register
pub mod DFSDM_FLT2AWSR {
    pub use super::DFSDM_FLT0AWSR::AWHTF;
    pub use super::DFSDM_FLT0AWSR::AWLTF;
}

/// DFSDM filter 2 analog watchdog clear flag register
pub mod DFSDM_FLT2AWCFR {
    pub use super::DFSDM_FLT0AWCFR::CLRAWHTF;
    pub use super::DFSDM_FLT0AWCFR::CLRAWLTF;
}

/// DFSDM filter 2 extremes detector maximum register
pub mod DFSDM_FLT2EXMAX {
    pub use super::DFSDM_FLT0EXMAX::EXMAX;
    pub use super::DFSDM_FLT0EXMAX::EXMAXCH;
}

/// DFSDM filter 2 extremes detector minimum register
pub mod DFSDM_FLT2EXMIN {
    pub use super::DFSDM_FLT0EXMIN::EXMIN;
    pub use super::DFSDM_FLT0EXMIN::EXMINCH;
}

/// DFSDM filter 2 conversion timer register
pub mod DFSDM_FLT2CNVTIMR {
    pub use super::DFSDM_FLT0CNVTIMR::CNVCNT;
}

/// DFSDM filter 3 control register 1
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

/// DFSDM filter 3 control register 2
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

/// DFSDM filter 3 interrupt and status register
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

/// DFSDM filter 3 interrupt flag clear register
pub mod DFSDM_FLT3ICR {
    pub use super::DFSDM_FLT0ICR::CLRCKABF;
    pub use super::DFSDM_FLT0ICR::CLRJOVRF;
    pub use super::DFSDM_FLT0ICR::CLRROVRF;
    pub use super::DFSDM_FLT0ICR::CLRSCDF;
}

/// DFSDM filter 3 injected channel group selection register
pub mod DFSDM_FLT3JCHGR {
    pub use super::DFSDM_FLT0JCHGR::JCHG;
}

/// DFSDM filter 3 control register
pub mod DFSDM_FLT3FCR {
    pub use super::DFSDM_FLT0FCR::FORD;
    pub use super::DFSDM_FLT0FCR::FOSR;
    pub use super::DFSDM_FLT0FCR::IOSR;
}

/// DFSDM filter 3 data register for injected group
pub mod DFSDM_FLT3JDATAR {
    pub use super::DFSDM_FLT0JDATAR::JDATA;
    pub use super::DFSDM_FLT0JDATAR::JDATACH;
}

/// DFSDM filter 3 data register for the regular channel
pub mod DFSDM_FLT3RDATAR {
    pub use super::DFSDM_FLT0RDATAR::RDATA;
    pub use super::DFSDM_FLT0RDATAR::RDATACH;
    pub use super::DFSDM_FLT0RDATAR::RPEND;
}

/// DFSDM filter 3 analog watchdog high threshold register
pub mod DFSDM_FLT3AWHTR {
    pub use super::DFSDM_FLT0AWHTR::AWHT;
    pub use super::DFSDM_FLT0AWHTR::BKAWH;
}

/// DFSDM filter 3 analog watchdog low threshold register
pub mod DFSDM_FLT3AWLTR {
    pub use super::DFSDM_FLT0AWLTR::AWLT;
    pub use super::DFSDM_FLT0AWLTR::BKAWL;
}

/// DFSDM filter 3 analog watchdog status register
pub mod DFSDM_FLT3AWSR {
    pub use super::DFSDM_FLT0AWSR::AWHTF;
    pub use super::DFSDM_FLT0AWSR::AWLTF;
}

/// DFSDM filter 3 analog watchdog clear flag register
pub mod DFSDM_FLT3AWCFR {
    pub use super::DFSDM_FLT0AWCFR::CLRAWHTF;
    pub use super::DFSDM_FLT0AWCFR::CLRAWLTF;
}

/// DFSDM filter 3 extremes detector maximum register
pub mod DFSDM_FLT3EXMAX {
    pub use super::DFSDM_FLT0EXMAX::EXMAX;
    pub use super::DFSDM_FLT0EXMAX::EXMAXCH;
}

/// DFSDM filter 3 extremes detector minimum register
pub mod DFSDM_FLT3EXMIN {
    pub use super::DFSDM_FLT0EXMIN::EXMIN;
    pub use super::DFSDM_FLT0EXMIN::EXMINCH;
}

/// DFSDM filter 3 conversion timer register
pub mod DFSDM_FLT3CNVTIMR {
    pub use super::DFSDM_FLT0CNVTIMR::CNVCNT;
}

/// DFSDM filter 4 control register 1
pub mod DFSDM_FLT4CR1 {
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

/// DFSDM filter 4 control register 2
pub mod DFSDM_FLT4CR2 {
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

/// DFSDM filter 4 interrupt and status register
pub mod DFSDM_FLT4ISR {
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

/// DFSDM filter 4 interrupt flag clear register
pub mod DFSDM_FLT4ICR {
    pub use super::DFSDM_FLT0ICR::CLRCKABF;
    pub use super::DFSDM_FLT0ICR::CLRJOVRF;
    pub use super::DFSDM_FLT0ICR::CLRROVRF;
    pub use super::DFSDM_FLT0ICR::CLRSCDF;
}

/// DFSDM filter 4 injected channel group selection register
pub mod DFSDM_FLT4JCHGR {
    pub use super::DFSDM_FLT0JCHGR::JCHG;
}

/// DFSDM filter 4 control register
pub mod DFSDM_FLT4FCR {
    pub use super::DFSDM_FLT0FCR::FORD;
    pub use super::DFSDM_FLT0FCR::FOSR;
    pub use super::DFSDM_FLT0FCR::IOSR;
}

/// DFSDM filter 4 data register for injected group
pub mod DFSDM_FLT4JDATAR {
    pub use super::DFSDM_FLT0JDATAR::JDATA;
    pub use super::DFSDM_FLT0JDATAR::JDATACH;
}

/// DFSDM filter 4 data register for the regular channel
pub mod DFSDM_FLT4RDATAR {
    pub use super::DFSDM_FLT0RDATAR::RDATA;
    pub use super::DFSDM_FLT0RDATAR::RDATACH;
    pub use super::DFSDM_FLT0RDATAR::RPEND;
}

/// DFSDM filter 4 analog watchdog high threshold register
pub mod DFSDM_FLT4AWHTR {
    pub use super::DFSDM_FLT0AWHTR::AWHT;
    pub use super::DFSDM_FLT0AWHTR::BKAWH;
}

/// DFSDM filter 4 analog watchdog low threshold register
pub mod DFSDM_FLT4AWLTR {
    pub use super::DFSDM_FLT0AWLTR::AWLT;
    pub use super::DFSDM_FLT0AWLTR::BKAWL;
}

/// DFSDM filter 4 analog watchdog status register
pub mod DFSDM_FLT4AWSR {
    pub use super::DFSDM_FLT0AWSR::AWHTF;
    pub use super::DFSDM_FLT0AWSR::AWLTF;
}

/// DFSDM filter 4 analog watchdog clear flag register
pub mod DFSDM_FLT4AWCFR {
    pub use super::DFSDM_FLT0AWCFR::CLRAWHTF;
    pub use super::DFSDM_FLT0AWCFR::CLRAWLTF;
}

/// DFSDM filter 4 extremes detector maximum register
pub mod DFSDM_FLT4EXMAX {
    pub use super::DFSDM_FLT0EXMAX::EXMAX;
    pub use super::DFSDM_FLT0EXMAX::EXMAXCH;
}

/// DFSDM filter 4 extremes detector minimum register
pub mod DFSDM_FLT4EXMIN {
    pub use super::DFSDM_FLT0EXMIN::EXMIN;
    pub use super::DFSDM_FLT0EXMIN::EXMINCH;
}

/// DFSDM filter 4 conversion timer register
pub mod DFSDM_FLT4CNVTIMR {
    pub use super::DFSDM_FLT0CNVTIMR::CNVCNT;
}

/// DFSDM filter 5 control register 1
pub mod DFSDM_FLT5CR1 {
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

/// DFSDM filter 5 control register 2
pub mod DFSDM_FLT5CR2 {
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

/// DFSDM filter 5 interrupt and status register
pub mod DFSDM_FLT5ISR {
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

/// DFSDM filter 5 interrupt flag clear register
pub mod DFSDM_FLT5ICR {
    pub use super::DFSDM_FLT0ICR::CLRCKABF;
    pub use super::DFSDM_FLT0ICR::CLRJOVRF;
    pub use super::DFSDM_FLT0ICR::CLRROVRF;
    pub use super::DFSDM_FLT0ICR::CLRSCDF;
}

/// DFSDM filter 5 injected channel group selection register
pub mod DFSDM_FLT5JCHGR {
    pub use super::DFSDM_FLT0JCHGR::JCHG;
}

/// DFSDM filter 5 control register
pub mod DFSDM_FLT5FCR {
    pub use super::DFSDM_FLT0FCR::FORD;
    pub use super::DFSDM_FLT0FCR::FOSR;
    pub use super::DFSDM_FLT0FCR::IOSR;
}

/// DFSDM filter 5 data register for injected group
pub mod DFSDM_FLT5JDATAR {
    pub use super::DFSDM_FLT0JDATAR::JDATA;
    pub use super::DFSDM_FLT0JDATAR::JDATACH;
}

/// DFSDM filter 5 data register for the regular channel
pub mod DFSDM_FLT5RDATAR {
    pub use super::DFSDM_FLT0RDATAR::RDATA;
    pub use super::DFSDM_FLT0RDATAR::RDATACH;
    pub use super::DFSDM_FLT0RDATAR::RPEND;
}

/// DFSDM filter 5 analog watchdog high threshold register
pub mod DFSDM_FLT5AWHTR {
    pub use super::DFSDM_FLT0AWHTR::AWHT;
    pub use super::DFSDM_FLT0AWHTR::BKAWH;
}

/// DFSDM filter 5 analog watchdog low threshold register
pub mod DFSDM_FLT5AWLTR {
    pub use super::DFSDM_FLT0AWLTR::AWLT;
    pub use super::DFSDM_FLT0AWLTR::BKAWL;
}

/// DFSDM filter 5 analog watchdog status register
pub mod DFSDM_FLT5AWSR {
    pub use super::DFSDM_FLT0AWSR::AWHTF;
    pub use super::DFSDM_FLT0AWSR::AWLTF;
}

/// DFSDM filter 5 analog watchdog clear flag register
pub mod DFSDM_FLT5AWCFR {
    pub use super::DFSDM_FLT0AWCFR::CLRAWHTF;
    pub use super::DFSDM_FLT0AWCFR::CLRAWLTF;
}

/// DFSDM filter 5 extremes detector maximum register
pub mod DFSDM_FLT5EXMAX {
    pub use super::DFSDM_FLT0EXMAX::EXMAX;
    pub use super::DFSDM_FLT0EXMAX::EXMAXCH;
}

/// DFSDM filter 5 extremes detector minimum register
pub mod DFSDM_FLT5EXMIN {
    pub use super::DFSDM_FLT0EXMIN::EXMIN;
    pub use super::DFSDM_FLT0EXMIN::EXMINCH;
}

/// DFSDM filter 5 conversion timer register
pub mod DFSDM_FLT5CNVTIMR {
    pub use super::DFSDM_FLT0CNVTIMR::CNVCNT;
}

/// This register specifies the hardware configuration of DFSDM peripheral.
pub mod DFSDM_HWCFGR {

    /// NBT
    pub mod NBT {
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

    /// NBF
    pub mod NBF {
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
}

/// This register specifies the version of DFSDM peripheral.
pub mod DFSDM_VERR {

    /// MINREV
    pub mod MINREV {
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

    /// MAJREV
    pub mod MAJREV {
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

/// This register specifies the identification of DFSDM peripheral.
pub mod DFSDM_IPIDR {

    /// ID
    pub mod ID {
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

/// This register specifies the size allocated to DFSDM registers.
pub mod DFSDM_SIDR {

    /// SID
    pub mod SID {
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
#[repr(C)]
pub struct RegisterBlock {
    /// This register specifies the parameters used by channel y.
    pub DFSDM_CH0CFGR1: RWRegister<u32>,

    /// This register specifies the parameters used by channel y.
    pub DFSDM_CH0CFGR2: RWRegister<u32>,

    /// Short-circuit detector and analog watchdog settings for channel y.
    pub DFSDM_CH0AWSCDR: RWRegister<u32>,

    /// This register contains the data resulting from the analog watchdog filter associated to the input channel y.
    pub DFSDM_CH0WDATR: RORegister<u32>,

    /// This register contains 16-bit input data to be processed by DFSDM filter module.
    pub DFSDM_CH0DATINR: RWRegister<u32>,

    /// DFSDM channel 0 delay register
    pub DFSDM_CH0DLYR: RWRegister<u32>,

    _reserved1: [u8; 8],

    /// This register specifies the parameters used by channel y.
    pub DFSDM_CH1CFGR1: RWRegister<u32>,

    /// This register specifies the parameters used by channel y.
    pub DFSDM_CH1CFGR2: RWRegister<u32>,

    /// Short-circuit detector and analog watchdog settings for channel y.
    pub DFSDM_CH1AWSCDR: RWRegister<u32>,

    /// This register contains the data resulting from the analog watchdog filter associated to the input channel y.
    pub DFSDM_CH1WDATR: RORegister<u32>,

    /// This register contains 16-bit input data to be processed by DFSDM filter module.
    pub DFSDM_CH1DATINR: RWRegister<u32>,

    /// DFSDM channel 1 delay register
    pub DFSDM_CH1DLYR: RWRegister<u32>,

    _reserved2: [u8; 8],

    /// This register specifies the parameters used by channel y.
    pub DFSDM_CH2CFGR1: RWRegister<u32>,

    /// This register specifies the parameters used by channel y.
    pub DFSDM_CH2CFGR2: RWRegister<u32>,

    /// Short-circuit detector and analog watchdog settings for channel y.
    pub DFSDM_CH2AWSCDR: RWRegister<u32>,

    /// This register contains the data resulting from the analog watchdog filter associated to the input channel y.
    pub DFSDM_CH2WDATR: RORegister<u32>,

    /// This register contains 16-bit input data to be processed by DFSDM filter module.
    pub DFSDM_CH2DATINR: RWRegister<u32>,

    /// DFSDM channel 2 delay register
    pub DFSDM_CH2DLYR: RWRegister<u32>,

    _reserved3: [u8; 8],

    /// This register specifies the parameters used by channel y.
    pub DFSDM_CH3CFGR1: RWRegister<u32>,

    /// This register specifies the parameters used by channel y.
    pub DFSDM_CH3CFGR2: RWRegister<u32>,

    /// Short-circuit detector and analog watchdog settings for channel y.
    pub DFSDM_CH3AWSCDR: RWRegister<u32>,

    /// This register contains the data resulting from the analog watchdog filter associated to the input channel y.
    pub DFSDM_CH3WDATR: RORegister<u32>,

    /// This register contains 16-bit input data to be processed by DFSDM filter module.
    pub DFSDM_CH3DATINR: RWRegister<u32>,

    /// DFSDM channel 3 delay register
    pub DFSDM_CH3DLYR: RWRegister<u32>,

    _reserved4: [u8; 8],

    /// This register specifies the parameters used by channel y.
    pub DFSDM_CH4CFGR1: RWRegister<u32>,

    /// This register specifies the parameters used by channel y.
    pub DFSDM_CH4CFGR2: RWRegister<u32>,

    /// Short-circuit detector and analog watchdog settings for channel y.
    pub DFSDM_CH4AWSCDR: RWRegister<u32>,

    /// This register contains the data resulting from the analog watchdog filter associated to the input channel y.
    pub DFSDM_CH4WDATR: RORegister<u32>,

    /// This register contains 16-bit input data to be processed by DFSDM filter module.
    pub DFSDM_CH4DATINR: RWRegister<u32>,

    /// DFSDM channel 4 delay register
    pub DFSDM_CH4DLYR: RWRegister<u32>,

    _reserved5: [u8; 8],

    /// This register specifies the parameters used by channel y.
    pub DFSDM_CH5CFGR1: RWRegister<u32>,

    /// This register specifies the parameters used by channel y.
    pub DFSDM_CH5CFGR2: RWRegister<u32>,

    /// Short-circuit detector and analog watchdog settings for channel y.
    pub DFSDM_CH5AWSCDR: RWRegister<u32>,

    /// This register contains the data resulting from the analog watchdog filter associated to the input channel y.
    pub DFSDM_CH5WDATR: RORegister<u32>,

    /// This register contains 16-bit input data to be processed by DFSDM filter module.
    pub DFSDM_CH5DATINR: RWRegister<u32>,

    /// DFSDM channel 5 delay register
    pub DFSDM_CH5DLYR: RWRegister<u32>,

    _reserved6: [u8; 8],

    /// This register specifies the parameters used by channel y.
    pub DFSDM_CH6CFGR1: RWRegister<u32>,

    /// This register specifies the parameters used by channel y.
    pub DFSDM_CH6CFGR2: RWRegister<u32>,

    /// Short-circuit detector and analog watchdog settings for channel y.
    pub DFSDM_CH6AWSCDR: RWRegister<u32>,

    /// This register contains the data resulting from the analog watchdog filter associated to the input channel y.
    pub DFSDM_CH6WDATR: RORegister<u32>,

    /// This register contains 16-bit input data to be processed by DFSDM filter module.
    pub DFSDM_CH6DATINR: RWRegister<u32>,

    /// DFSDM channel 6 delay register
    pub DFSDM_CH6DLYR: RWRegister<u32>,

    _reserved7: [u8; 8],

    /// This register specifies the parameters used by channel y.
    pub DFSDM_CH7CFGR1: RWRegister<u32>,

    /// This register specifies the parameters used by channel y.
    pub DFSDM_CH7CFGR2: RWRegister<u32>,

    /// Short-circuit detector and analog watchdog settings for channel y.
    pub DFSDM_CH7AWSCDR: RWRegister<u32>,

    /// This register contains the data resulting from the analog watchdog filter associated to the input channel y.
    pub DFSDM_CH7WDATR: RORegister<u32>,

    /// This register contains 16-bit input data to be processed by DFSDM filter module.
    pub DFSDM_CH7DATINR: RWRegister<u32>,

    /// DFSDM channel 7 delay register
    pub DFSDM_CH7DLYR: RWRegister<u32>,

    _reserved8: [u8; 8],

    /// DFSDM filter 0 control register 1
    pub DFSDM_FLT0CR1: RWRegister<u32>,

    /// DFSDM filter 0 control register 2
    pub DFSDM_FLT0CR2: RWRegister<u32>,

    /// DFSDM filter 0 interrupt and status register
    pub DFSDM_FLT0ISR: RORegister<u32>,

    /// DFSDM filter 0 interrupt flag clear register
    pub DFSDM_FLT0ICR: RWRegister<u32>,

    /// DFSDM filter 0 injected channel group selection register
    pub DFSDM_FLT0JCHGR: RWRegister<u32>,

    /// DFSDM filter 0 control register
    pub DFSDM_FLT0FCR: RWRegister<u32>,

    /// DFSDM filter 0 data register for injected group
    pub DFSDM_FLT0JDATAR: RORegister<u32>,

    /// DFSDM filter 0 data register for the regular channel
    pub DFSDM_FLT0RDATAR: RORegister<u32>,

    /// DFSDM filter 0 analog watchdog high threshold register
    pub DFSDM_FLT0AWHTR: RWRegister<u32>,

    /// DFSDM filter 0 analog watchdog low threshold register
    pub DFSDM_FLT0AWLTR: RWRegister<u32>,

    /// DFSDM filter 0 analog watchdog status register
    pub DFSDM_FLT0AWSR: RORegister<u32>,

    /// DFSDM filter 0 analog watchdog clear flag register
    pub DFSDM_FLT0AWCFR: RWRegister<u32>,

    /// DFSDM filter 0 extremes detector maximum register
    pub DFSDM_FLT0EXMAX: RORegister<u32>,

    /// DFSDM filter 0 extremes detector minimum register
    pub DFSDM_FLT0EXMIN: RWRegister<u32>,

    /// DFSDM filter 0 conversion timer register
    pub DFSDM_FLT0CNVTIMR: RORegister<u32>,

    _reserved9: [u8; 68],

    /// DFSDM filter 1 control register 1
    pub DFSDM_FLT1CR1: RWRegister<u32>,

    /// DFSDM filter 1 control register 2
    pub DFSDM_FLT1CR2: RWRegister<u32>,

    /// DFSDM filter 1 interrupt and status register
    pub DFSDM_FLT1ISR: RORegister<u32>,

    /// DFSDM filter 1 interrupt flag clear register
    pub DFSDM_FLT1ICR: RWRegister<u32>,

    /// DFSDM filter 1 injected channel group selection register
    pub DFSDM_FLT1JCHGR: RWRegister<u32>,

    /// DFSDM filter 1 control register
    pub DFSDM_FLT1FCR: RWRegister<u32>,

    /// DFSDM filter 1 data register for injected group
    pub DFSDM_FLT1JDATAR: RORegister<u32>,

    /// DFSDM filter 1 data register for the regular channel
    pub DFSDM_FLT1RDATAR: RORegister<u32>,

    /// DFSDM filter 1 analog watchdog high threshold register
    pub DFSDM_FLT1AWHTR: RWRegister<u32>,

    /// DFSDM filter 1 analog watchdog low threshold register
    pub DFSDM_FLT1AWLTR: RWRegister<u32>,

    /// DFSDM filter 1 analog watchdog status register
    pub DFSDM_FLT1AWSR: RORegister<u32>,

    /// DFSDM filter 1 analog watchdog clear flag register
    pub DFSDM_FLT1AWCFR: RWRegister<u32>,

    /// DFSDM filter 1 extremes detector maximum register
    pub DFSDM_FLT1EXMAX: RORegister<u32>,

    /// DFSDM filter 1 extremes detector minimum register
    pub DFSDM_FLT1EXMIN: RWRegister<u32>,

    /// DFSDM filter 1 conversion timer register
    pub DFSDM_FLT1CNVTIMR: RORegister<u32>,

    _reserved10: [u8; 68],

    /// DFSDM filter 2 control register 1
    pub DFSDM_FLT2CR1: RWRegister<u32>,

    /// DFSDM filter 2 control register 2
    pub DFSDM_FLT2CR2: RWRegister<u32>,

    /// DFSDM filter 2 interrupt and status register
    pub DFSDM_FLT2ISR: RORegister<u32>,

    /// DFSDM filter 2 interrupt flag clear register
    pub DFSDM_FLT2ICR: RWRegister<u32>,

    /// DFSDM filter 2 injected channel group selection register
    pub DFSDM_FLT2JCHGR: RWRegister<u32>,

    /// DFSDM filter 2 control register
    pub DFSDM_FLT2FCR: RWRegister<u32>,

    /// DFSDM filter 2 data register for injected group
    pub DFSDM_FLT2JDATAR: RORegister<u32>,

    /// DFSDM filter 2 data register for the regular channel
    pub DFSDM_FLT2RDATAR: RORegister<u32>,

    /// DFSDM filter 2 analog watchdog high threshold register
    pub DFSDM_FLT2AWHTR: RWRegister<u32>,

    /// DFSDM filter 2 analog watchdog low threshold register
    pub DFSDM_FLT2AWLTR: RWRegister<u32>,

    /// DFSDM filter 2 analog watchdog status register
    pub DFSDM_FLT2AWSR: RORegister<u32>,

    /// DFSDM filter 2 analog watchdog clear flag register
    pub DFSDM_FLT2AWCFR: RWRegister<u32>,

    /// DFSDM filter 2 extremes detector maximum register
    pub DFSDM_FLT2EXMAX: RORegister<u32>,

    /// DFSDM filter 2 extremes detector minimum register
    pub DFSDM_FLT2EXMIN: RWRegister<u32>,

    /// DFSDM filter 2 conversion timer register
    pub DFSDM_FLT2CNVTIMR: RORegister<u32>,

    _reserved11: [u8; 68],

    /// DFSDM filter 3 control register 1
    pub DFSDM_FLT3CR1: RWRegister<u32>,

    /// DFSDM filter 3 control register 2
    pub DFSDM_FLT3CR2: RWRegister<u32>,

    /// DFSDM filter 3 interrupt and status register
    pub DFSDM_FLT3ISR: RORegister<u32>,

    /// DFSDM filter 3 interrupt flag clear register
    pub DFSDM_FLT3ICR: RWRegister<u32>,

    /// DFSDM filter 3 injected channel group selection register
    pub DFSDM_FLT3JCHGR: RWRegister<u32>,

    /// DFSDM filter 3 control register
    pub DFSDM_FLT3FCR: RWRegister<u32>,

    /// DFSDM filter 3 data register for injected group
    pub DFSDM_FLT3JDATAR: RORegister<u32>,

    /// DFSDM filter 3 data register for the regular channel
    pub DFSDM_FLT3RDATAR: RORegister<u32>,

    /// DFSDM filter 3 analog watchdog high threshold register
    pub DFSDM_FLT3AWHTR: RWRegister<u32>,

    /// DFSDM filter 3 analog watchdog low threshold register
    pub DFSDM_FLT3AWLTR: RWRegister<u32>,

    /// DFSDM filter 3 analog watchdog status register
    pub DFSDM_FLT3AWSR: RORegister<u32>,

    /// DFSDM filter 3 analog watchdog clear flag register
    pub DFSDM_FLT3AWCFR: RWRegister<u32>,

    /// DFSDM filter 3 extremes detector maximum register
    pub DFSDM_FLT3EXMAX: RORegister<u32>,

    /// DFSDM filter 3 extremes detector minimum register
    pub DFSDM_FLT3EXMIN: RWRegister<u32>,

    /// DFSDM filter 3 conversion timer register
    pub DFSDM_FLT3CNVTIMR: RORegister<u32>,

    _reserved12: [u8; 68],

    /// DFSDM filter 4 control register 1
    pub DFSDM_FLT4CR1: RWRegister<u32>,

    /// DFSDM filter 4 control register 2
    pub DFSDM_FLT4CR2: RWRegister<u32>,

    /// DFSDM filter 4 interrupt and status register
    pub DFSDM_FLT4ISR: RORegister<u32>,

    /// DFSDM filter 4 interrupt flag clear register
    pub DFSDM_FLT4ICR: RWRegister<u32>,

    /// DFSDM filter 4 injected channel group selection register
    pub DFSDM_FLT4JCHGR: RWRegister<u32>,

    /// DFSDM filter 4 control register
    pub DFSDM_FLT4FCR: RWRegister<u32>,

    /// DFSDM filter 4 data register for injected group
    pub DFSDM_FLT4JDATAR: RORegister<u32>,

    /// DFSDM filter 4 data register for the regular channel
    pub DFSDM_FLT4RDATAR: RORegister<u32>,

    /// DFSDM filter 4 analog watchdog high threshold register
    pub DFSDM_FLT4AWHTR: RWRegister<u32>,

    /// DFSDM filter 4 analog watchdog low threshold register
    pub DFSDM_FLT4AWLTR: RWRegister<u32>,

    /// DFSDM filter 4 analog watchdog status register
    pub DFSDM_FLT4AWSR: RORegister<u32>,

    /// DFSDM filter 4 analog watchdog clear flag register
    pub DFSDM_FLT4AWCFR: RWRegister<u32>,

    /// DFSDM filter 4 extremes detector maximum register
    pub DFSDM_FLT4EXMAX: RORegister<u32>,

    /// DFSDM filter 4 extremes detector minimum register
    pub DFSDM_FLT4EXMIN: RWRegister<u32>,

    /// DFSDM filter 4 conversion timer register
    pub DFSDM_FLT4CNVTIMR: RORegister<u32>,

    _reserved13: [u8; 68],

    /// DFSDM filter 5 control register 1
    pub DFSDM_FLT5CR1: RWRegister<u32>,

    /// DFSDM filter 5 control register 2
    pub DFSDM_FLT5CR2: RWRegister<u32>,

    /// DFSDM filter 5 interrupt and status register
    pub DFSDM_FLT5ISR: RORegister<u32>,

    /// DFSDM filter 5 interrupt flag clear register
    pub DFSDM_FLT5ICR: RWRegister<u32>,

    /// DFSDM filter 5 injected channel group selection register
    pub DFSDM_FLT5JCHGR: RWRegister<u32>,

    /// DFSDM filter 5 control register
    pub DFSDM_FLT5FCR: RWRegister<u32>,

    /// DFSDM filter 5 data register for injected group
    pub DFSDM_FLT5JDATAR: RORegister<u32>,

    /// DFSDM filter 5 data register for the regular channel
    pub DFSDM_FLT5RDATAR: RORegister<u32>,

    /// DFSDM filter 5 analog watchdog high threshold register
    pub DFSDM_FLT5AWHTR: RWRegister<u32>,

    /// DFSDM filter 5 analog watchdog low threshold register
    pub DFSDM_FLT5AWLTR: RWRegister<u32>,

    /// DFSDM filter 5 analog watchdog status register
    pub DFSDM_FLT5AWSR: RORegister<u32>,

    /// DFSDM filter 5 analog watchdog clear flag register
    pub DFSDM_FLT5AWCFR: RWRegister<u32>,

    /// DFSDM filter 5 extremes detector maximum register
    pub DFSDM_FLT5EXMAX: RORegister<u32>,

    /// DFSDM filter 5 extremes detector minimum register
    pub DFSDM_FLT5EXMIN: RWRegister<u32>,

    /// DFSDM filter 5 conversion timer register
    pub DFSDM_FLT5CNVTIMR: RORegister<u32>,

    _reserved14: [u8; 1076],

    /// This register specifies the hardware configuration of DFSDM peripheral.
    pub DFSDM_HWCFGR: RORegister<u32>,

    /// This register specifies the version of DFSDM peripheral.
    pub DFSDM_VERR: RORegister<u32>,

    /// This register specifies the identification of DFSDM peripheral.
    pub DFSDM_IPIDR: RORegister<u32>,

    /// This register specifies the size allocated to DFSDM registers.
    pub DFSDM_SIDR: RORegister<u32>,
}
pub struct ResetValues {
    pub DFSDM_CH0CFGR1: u32,
    pub DFSDM_CH0CFGR2: u32,
    pub DFSDM_CH0AWSCDR: u32,
    pub DFSDM_CH0WDATR: u32,
    pub DFSDM_CH0DATINR: u32,
    pub DFSDM_CH0DLYR: u32,
    pub DFSDM_CH1CFGR1: u32,
    pub DFSDM_CH1CFGR2: u32,
    pub DFSDM_CH1AWSCDR: u32,
    pub DFSDM_CH1WDATR: u32,
    pub DFSDM_CH1DATINR: u32,
    pub DFSDM_CH1DLYR: u32,
    pub DFSDM_CH2CFGR1: u32,
    pub DFSDM_CH2CFGR2: u32,
    pub DFSDM_CH2AWSCDR: u32,
    pub DFSDM_CH2WDATR: u32,
    pub DFSDM_CH2DATINR: u32,
    pub DFSDM_CH2DLYR: u32,
    pub DFSDM_CH3CFGR1: u32,
    pub DFSDM_CH3CFGR2: u32,
    pub DFSDM_CH3AWSCDR: u32,
    pub DFSDM_CH3WDATR: u32,
    pub DFSDM_CH3DATINR: u32,
    pub DFSDM_CH3DLYR: u32,
    pub DFSDM_CH4CFGR1: u32,
    pub DFSDM_CH4CFGR2: u32,
    pub DFSDM_CH4AWSCDR: u32,
    pub DFSDM_CH4WDATR: u32,
    pub DFSDM_CH4DATINR: u32,
    pub DFSDM_CH4DLYR: u32,
    pub DFSDM_CH5CFGR1: u32,
    pub DFSDM_CH5CFGR2: u32,
    pub DFSDM_CH5AWSCDR: u32,
    pub DFSDM_CH5WDATR: u32,
    pub DFSDM_CH5DATINR: u32,
    pub DFSDM_CH5DLYR: u32,
    pub DFSDM_CH6CFGR1: u32,
    pub DFSDM_CH6CFGR2: u32,
    pub DFSDM_CH6AWSCDR: u32,
    pub DFSDM_CH6WDATR: u32,
    pub DFSDM_CH6DATINR: u32,
    pub DFSDM_CH6DLYR: u32,
    pub DFSDM_CH7CFGR1: u32,
    pub DFSDM_CH7CFGR2: u32,
    pub DFSDM_CH7AWSCDR: u32,
    pub DFSDM_CH7WDATR: u32,
    pub DFSDM_CH7DATINR: u32,
    pub DFSDM_CH7DLYR: u32,
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
    pub DFSDM_FLT1JCHGR: u32,
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
    pub DFSDM_FLT4CR1: u32,
    pub DFSDM_FLT4CR2: u32,
    pub DFSDM_FLT4ISR: u32,
    pub DFSDM_FLT4ICR: u32,
    pub DFSDM_FLT4JCHGR: u32,
    pub DFSDM_FLT4FCR: u32,
    pub DFSDM_FLT4JDATAR: u32,
    pub DFSDM_FLT4RDATAR: u32,
    pub DFSDM_FLT4AWHTR: u32,
    pub DFSDM_FLT4AWLTR: u32,
    pub DFSDM_FLT4AWSR: u32,
    pub DFSDM_FLT4AWCFR: u32,
    pub DFSDM_FLT4EXMAX: u32,
    pub DFSDM_FLT4EXMIN: u32,
    pub DFSDM_FLT4CNVTIMR: u32,
    pub DFSDM_FLT5CR1: u32,
    pub DFSDM_FLT5CR2: u32,
    pub DFSDM_FLT5ISR: u32,
    pub DFSDM_FLT5ICR: u32,
    pub DFSDM_FLT5JCHGR: u32,
    pub DFSDM_FLT5FCR: u32,
    pub DFSDM_FLT5JDATAR: u32,
    pub DFSDM_FLT5RDATAR: u32,
    pub DFSDM_FLT5AWHTR: u32,
    pub DFSDM_FLT5AWLTR: u32,
    pub DFSDM_FLT5AWSR: u32,
    pub DFSDM_FLT5AWCFR: u32,
    pub DFSDM_FLT5EXMAX: u32,
    pub DFSDM_FLT5EXMIN: u32,
    pub DFSDM_FLT5CNVTIMR: u32,
    pub DFSDM_HWCFGR: u32,
    pub DFSDM_VERR: u32,
    pub DFSDM_IPIDR: u32,
    pub DFSDM_SIDR: u32,
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
