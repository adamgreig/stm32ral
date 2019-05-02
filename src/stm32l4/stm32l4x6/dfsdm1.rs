#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Digital filter for sigma delta modulators

#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;
use {RORegister, RWRegister};

/// channel configuration y register
pub mod CHCFG0R1 {

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
pub mod CHCFG0R2 {

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
pub mod AWSCD0R {

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
pub mod CHWDAT0R {

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
pub mod CHDATIN0R {

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

/// CHCFG1R1
pub mod CHCFG1R1 {

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

/// CHCFG1R2
pub mod CHCFG1R2 {
    pub use super::CHCFG0R2::DTRBS;
    pub use super::CHCFG0R2::OFFSET;
}

/// AWSCD1R
pub mod AWSCD1R {
    pub use super::AWSCD0R::AWFORD;
    pub use super::AWSCD0R::AWFOSR;
    pub use super::AWSCD0R::BKSCD;
    pub use super::AWSCD0R::SCDT;
}

/// CHWDAT1R
pub mod CHWDAT1R {
    pub use super::CHWDAT0R::WDATA;
}

/// CHDATIN1R
pub mod CHDATIN1R {
    pub use super::CHDATIN0R::INDAT0;
    pub use super::CHDATIN0R::INDAT1;
}

/// CHCFG2R1
pub mod CHCFG2R1 {
    pub use super::CHCFG1R1::CHEN;
    pub use super::CHCFG1R1::CHINSEL;
    pub use super::CHCFG1R1::CKABEN;
    pub use super::CHCFG1R1::DATMPX;
    pub use super::CHCFG1R1::DATPACK;
    pub use super::CHCFG1R1::SCDEN;
    pub use super::CHCFG1R1::SITP;
    pub use super::CHCFG1R1::SPICKSEL;
}

/// CHCFG2R2
pub mod CHCFG2R2 {
    pub use super::CHCFG0R2::DTRBS;
    pub use super::CHCFG0R2::OFFSET;
}

/// AWSCD2R
pub mod AWSCD2R {
    pub use super::AWSCD0R::AWFORD;
    pub use super::AWSCD0R::AWFOSR;
    pub use super::AWSCD0R::BKSCD;
    pub use super::AWSCD0R::SCDT;
}

/// CHWDAT2R
pub mod CHWDAT2R {
    pub use super::CHWDAT0R::WDATA;
}

/// CHDATIN2R
pub mod CHDATIN2R {
    pub use super::CHDATIN0R::INDAT0;
    pub use super::CHDATIN0R::INDAT1;
}

/// CHCFG3R1
pub mod CHCFG3R1 {
    pub use super::CHCFG1R1::CHEN;
    pub use super::CHCFG1R1::CHINSEL;
    pub use super::CHCFG1R1::CKABEN;
    pub use super::CHCFG1R1::DATMPX;
    pub use super::CHCFG1R1::DATPACK;
    pub use super::CHCFG1R1::SCDEN;
    pub use super::CHCFG1R1::SITP;
    pub use super::CHCFG1R1::SPICKSEL;
}

/// CHCFG3R2
pub mod CHCFG3R2 {
    pub use super::CHCFG0R2::DTRBS;
    pub use super::CHCFG0R2::OFFSET;
}

/// AWSCD3R
pub mod AWSCD3R {
    pub use super::AWSCD0R::AWFORD;
    pub use super::AWSCD0R::AWFOSR;
    pub use super::AWSCD0R::BKSCD;
    pub use super::AWSCD0R::SCDT;
}

/// CHWDAT3R
pub mod CHWDAT3R {
    pub use super::CHWDAT0R::WDATA;
}

/// CHDATIN3R
pub mod CHDATIN3R {
    pub use super::CHDATIN0R::INDAT0;
    pub use super::CHDATIN0R::INDAT1;
}

/// CHCFG4R1
pub mod CHCFG4R1 {
    pub use super::CHCFG1R1::CHEN;
    pub use super::CHCFG1R1::CHINSEL;
    pub use super::CHCFG1R1::CKABEN;
    pub use super::CHCFG1R1::DATMPX;
    pub use super::CHCFG1R1::DATPACK;
    pub use super::CHCFG1R1::SCDEN;
    pub use super::CHCFG1R1::SITP;
    pub use super::CHCFG1R1::SPICKSEL;
}

/// CHCFG4R2
pub mod CHCFG4R2 {
    pub use super::CHCFG0R2::DTRBS;
    pub use super::CHCFG0R2::OFFSET;
}

/// AWSCD4R
pub mod AWSCD4R {
    pub use super::AWSCD0R::AWFORD;
    pub use super::AWSCD0R::AWFOSR;
    pub use super::AWSCD0R::BKSCD;
    pub use super::AWSCD0R::SCDT;
}

/// CHWDAT4R
pub mod CHWDAT4R {
    pub use super::CHWDAT0R::WDATA;
}

/// CHDATIN4R
pub mod CHDATIN4R {
    pub use super::CHDATIN0R::INDAT0;
    pub use super::CHDATIN0R::INDAT1;
}

/// CHCFG5R1
pub mod CHCFG5R1 {
    pub use super::CHCFG1R1::CHEN;
    pub use super::CHCFG1R1::CHINSEL;
    pub use super::CHCFG1R1::CKABEN;
    pub use super::CHCFG1R1::DATMPX;
    pub use super::CHCFG1R1::DATPACK;
    pub use super::CHCFG1R1::SCDEN;
    pub use super::CHCFG1R1::SITP;
    pub use super::CHCFG1R1::SPICKSEL;
}

/// CHCFG5R2
pub mod CHCFG5R2 {
    pub use super::CHCFG0R2::DTRBS;
    pub use super::CHCFG0R2::OFFSET;
}

/// AWSCD5R
pub mod AWSCD5R {
    pub use super::AWSCD0R::AWFORD;
    pub use super::AWSCD0R::AWFOSR;
    pub use super::AWSCD0R::BKSCD;
    pub use super::AWSCD0R::SCDT;
}

/// CHWDAT5R
pub mod CHWDAT5R {
    pub use super::CHWDAT0R::WDATA;
}

/// CHDATIN5R
pub mod CHDATIN5R {
    pub use super::CHDATIN0R::INDAT0;
    pub use super::CHDATIN0R::INDAT1;
}

/// CHCFG6R1
pub mod CHCFG6R1 {
    pub use super::CHCFG1R1::CHEN;
    pub use super::CHCFG1R1::CHINSEL;
    pub use super::CHCFG1R1::CKABEN;
    pub use super::CHCFG1R1::DATMPX;
    pub use super::CHCFG1R1::DATPACK;
    pub use super::CHCFG1R1::SCDEN;
    pub use super::CHCFG1R1::SITP;
    pub use super::CHCFG1R1::SPICKSEL;
}

/// CHCFG6R2
pub mod CHCFG6R2 {
    pub use super::CHCFG0R2::DTRBS;
    pub use super::CHCFG0R2::OFFSET;
}

/// AWSCD6R
pub mod AWSCD6R {
    pub use super::AWSCD0R::AWFORD;
    pub use super::AWSCD0R::AWFOSR;
    pub use super::AWSCD0R::BKSCD;
    pub use super::AWSCD0R::SCDT;
}

/// CHWDAT6R
pub mod CHWDAT6R {
    pub use super::CHWDAT0R::WDATA;
}

/// CHDATIN6R
pub mod CHDATIN6R {
    pub use super::CHDATIN0R::INDAT0;
    pub use super::CHDATIN0R::INDAT1;
}

/// CHCFG7R1
pub mod CHCFG7R1 {
    pub use super::CHCFG1R1::CHEN;
    pub use super::CHCFG1R1::CHINSEL;
    pub use super::CHCFG1R1::CKABEN;
    pub use super::CHCFG1R1::DATMPX;
    pub use super::CHCFG1R1::DATPACK;
    pub use super::CHCFG1R1::SCDEN;
    pub use super::CHCFG1R1::SITP;
    pub use super::CHCFG1R1::SPICKSEL;
}

/// CHCFG7R2
pub mod CHCFG7R2 {
    pub use super::CHCFG0R2::DTRBS;
    pub use super::CHCFG0R2::OFFSET;
}

/// AWSCD7R
pub mod AWSCD7R {
    pub use super::AWSCD0R::AWFORD;
    pub use super::AWSCD0R::AWFOSR;
    pub use super::AWSCD0R::BKSCD;
    pub use super::AWSCD0R::SCDT;
}

/// CHWDAT7R
pub mod CHWDAT7R {
    pub use super::CHWDAT0R::WDATA;
}

/// CHDATIN7R
pub mod CHDATIN7R {
    pub use super::CHDATIN0R::INDAT0;
    pub use super::CHDATIN0R::INDAT1;
}

/// control register 1
pub mod DFSDM0_CR1 {

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
pub mod DFSDM0_CR2 {

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
pub mod DFSDM0_ISR {

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
pub mod DFSDM0_ICR {

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
pub mod DFSDM0_JCHGR {

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
pub mod DFSDM0_FCR {

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
pub mod DFSDM0_JDATAR {

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
pub mod DFSDM0_RDATAR {

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
pub mod DFSDM0_AWHTR {

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
pub mod DFSDM0_AWLTR {

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
pub mod DFSDM0_AWSR {

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
pub mod DFSDM0_AWCFR {

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
pub mod DFSDM0_EXMAX {

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
pub mod DFSDM0_EXMIN {

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
pub mod DFSDM0_CNVTIMR {

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
pub mod DFSDM1_CR1 {
    pub use super::DFSDM0_CR1::AWFSEL;
    pub use super::DFSDM0_CR1::DFEN;
    pub use super::DFSDM0_CR1::FAST;
    pub use super::DFSDM0_CR1::JDMAEN;
    pub use super::DFSDM0_CR1::JEXTEN;
    pub use super::DFSDM0_CR1::JEXTSEL;
    pub use super::DFSDM0_CR1::JSCAN;
    pub use super::DFSDM0_CR1::JSWSTART;
    pub use super::DFSDM0_CR1::JSYNC;
    pub use super::DFSDM0_CR1::RCH;
    pub use super::DFSDM0_CR1::RCONT;
    pub use super::DFSDM0_CR1::RDMAEN;
    pub use super::DFSDM0_CR1::RSWSTART;
    pub use super::DFSDM0_CR1::RSYNC;
}

/// control register 2
pub mod DFSDM1_CR2 {
    pub use super::DFSDM0_CR2::AWDCH;
    pub use super::DFSDM0_CR2::AWDIE;
    pub use super::DFSDM0_CR2::CKABIE;
    pub use super::DFSDM0_CR2::EXCH;
    pub use super::DFSDM0_CR2::JEOCIE;
    pub use super::DFSDM0_CR2::JOVRIE;
    pub use super::DFSDM0_CR2::REOCIE;
    pub use super::DFSDM0_CR2::ROVRIE;
    pub use super::DFSDM0_CR2::SCDIE;
}

/// interrupt and status register
pub mod DFSDM1_ISR {
    pub use super::DFSDM0_ISR::AWDF;
    pub use super::DFSDM0_ISR::CKABF;
    pub use super::DFSDM0_ISR::JCIP;
    pub use super::DFSDM0_ISR::JEOCF;
    pub use super::DFSDM0_ISR::JOVRF;
    pub use super::DFSDM0_ISR::RCIP;
    pub use super::DFSDM0_ISR::REOCF;
    pub use super::DFSDM0_ISR::ROVRF;
    pub use super::DFSDM0_ISR::SCDF;
}

/// interrupt flag clear register
pub mod DFSDM1_ICR {
    pub use super::DFSDM0_ICR::CLRCKABF;
    pub use super::DFSDM0_ICR::CLRJOVRF;
    pub use super::DFSDM0_ICR::CLRROVRF;
    pub use super::DFSDM0_ICR::CLRSCDF;
}

/// injected channel group selection register
pub mod DFSDM1_JCHGR {
    pub use super::DFSDM0_JCHGR::JCHG;
}

/// filter control register
pub mod DFSDM1_FCR {
    pub use super::DFSDM0_FCR::FORD;
    pub use super::DFSDM0_FCR::FOSR;
    pub use super::DFSDM0_FCR::IOSR;
}

/// data register for injected group
pub mod DFSDM1_JDATAR {
    pub use super::DFSDM0_JDATAR::JDATA;
    pub use super::DFSDM0_JDATAR::JDATACH;
}

/// data register for the regular channel
pub mod DFSDM1_RDATAR {
    pub use super::DFSDM0_RDATAR::RDATA;
    pub use super::DFSDM0_RDATAR::RDATACH;
    pub use super::DFSDM0_RDATAR::RPEND;
}

/// analog watchdog high threshold register
pub mod DFSDM1_AWHTR {
    pub use super::DFSDM0_AWHTR::AWHT;
    pub use super::DFSDM0_AWHTR::BKAWH;
}

/// analog watchdog low threshold register
pub mod DFSDM1_AWLTR {
    pub use super::DFSDM0_AWLTR::AWLT;
    pub use super::DFSDM0_AWLTR::BKAWL;
}

/// analog watchdog status register
pub mod DFSDM1_AWSR {
    pub use super::DFSDM0_AWSR::AWHTF;
    pub use super::DFSDM0_AWSR::AWLTF;
}

/// analog watchdog clear flag register
pub mod DFSDM1_AWCFR {
    pub use super::DFSDM0_AWCFR::CLRAWHTF;
    pub use super::DFSDM0_AWCFR::CLRAWLTF;
}

/// Extremes detector maximum register
pub mod DFSDM1_EXMAX {
    pub use super::DFSDM0_EXMAX::EXMAX;
    pub use super::DFSDM0_EXMAX::EXMAXCH;
}

/// Extremes detector minimum register
pub mod DFSDM1_EXMIN {
    pub use super::DFSDM0_EXMIN::EXMIN;
    pub use super::DFSDM0_EXMIN::EXMINCH;
}

/// conversion timer register
pub mod DFSDM1_CNVTIMR {
    pub use super::DFSDM0_CNVTIMR::CNVCNT;
}

/// control register 1
pub mod DFSDM2_CR1 {
    pub use super::DFSDM0_CR1::AWFSEL;
    pub use super::DFSDM0_CR1::DFEN;
    pub use super::DFSDM0_CR1::FAST;
    pub use super::DFSDM0_CR1::JDMAEN;
    pub use super::DFSDM0_CR1::JEXTEN;
    pub use super::DFSDM0_CR1::JEXTSEL;
    pub use super::DFSDM0_CR1::JSCAN;
    pub use super::DFSDM0_CR1::JSWSTART;
    pub use super::DFSDM0_CR1::JSYNC;
    pub use super::DFSDM0_CR1::RCH;
    pub use super::DFSDM0_CR1::RCONT;
    pub use super::DFSDM0_CR1::RDMAEN;
    pub use super::DFSDM0_CR1::RSWSTART;
    pub use super::DFSDM0_CR1::RSYNC;
}

/// control register 2
pub mod DFSDM2_CR2 {
    pub use super::DFSDM0_CR2::AWDCH;
    pub use super::DFSDM0_CR2::AWDIE;
    pub use super::DFSDM0_CR2::CKABIE;
    pub use super::DFSDM0_CR2::EXCH;
    pub use super::DFSDM0_CR2::JEOCIE;
    pub use super::DFSDM0_CR2::JOVRIE;
    pub use super::DFSDM0_CR2::REOCIE;
    pub use super::DFSDM0_CR2::ROVRIE;
    pub use super::DFSDM0_CR2::SCDIE;
}

/// interrupt and status register
pub mod DFSDM2_ISR {
    pub use super::DFSDM0_ISR::AWDF;
    pub use super::DFSDM0_ISR::CKABF;
    pub use super::DFSDM0_ISR::JCIP;
    pub use super::DFSDM0_ISR::JEOCF;
    pub use super::DFSDM0_ISR::JOVRF;
    pub use super::DFSDM0_ISR::RCIP;
    pub use super::DFSDM0_ISR::REOCF;
    pub use super::DFSDM0_ISR::ROVRF;
    pub use super::DFSDM0_ISR::SCDF;
}

/// interrupt flag clear register
pub mod DFSDM2_ICR {
    pub use super::DFSDM0_ICR::CLRCKABF;
    pub use super::DFSDM0_ICR::CLRJOVRF;
    pub use super::DFSDM0_ICR::CLRROVRF;
    pub use super::DFSDM0_ICR::CLRSCDF;
}

/// injected channel group selection register
pub mod DFSDM2_JCHGR {
    pub use super::DFSDM0_JCHGR::JCHG;
}

/// filter control register
pub mod DFSDM2_FCR {
    pub use super::DFSDM0_FCR::FORD;
    pub use super::DFSDM0_FCR::FOSR;
    pub use super::DFSDM0_FCR::IOSR;
}

/// data register for injected group
pub mod DFSDM2_JDATAR {
    pub use super::DFSDM0_JDATAR::JDATA;
    pub use super::DFSDM0_JDATAR::JDATACH;
}

/// data register for the regular channel
pub mod DFSDM2_RDATAR {
    pub use super::DFSDM0_RDATAR::RDATA;
    pub use super::DFSDM0_RDATAR::RDATACH;
    pub use super::DFSDM0_RDATAR::RPEND;
}

/// analog watchdog high threshold register
pub mod DFSDM2_AWHTR {
    pub use super::DFSDM0_AWHTR::AWHT;
    pub use super::DFSDM0_AWHTR::BKAWH;
}

/// analog watchdog low threshold register
pub mod DFSDM2_AWLTR {
    pub use super::DFSDM0_AWLTR::AWLT;
    pub use super::DFSDM0_AWLTR::BKAWL;
}

/// analog watchdog status register
pub mod DFSDM2_AWSR {
    pub use super::DFSDM0_AWSR::AWHTF;
    pub use super::DFSDM0_AWSR::AWLTF;
}

/// analog watchdog clear flag register
pub mod DFSDM2_AWCFR {
    pub use super::DFSDM0_AWCFR::CLRAWHTF;
    pub use super::DFSDM0_AWCFR::CLRAWLTF;
}

/// Extremes detector maximum register
pub mod DFSDM2_EXMAX {
    pub use super::DFSDM0_EXMAX::EXMAX;
    pub use super::DFSDM0_EXMAX::EXMAXCH;
}

/// Extremes detector minimum register
pub mod DFSDM2_EXMIN {
    pub use super::DFSDM0_EXMIN::EXMIN;
    pub use super::DFSDM0_EXMIN::EXMINCH;
}

/// conversion timer register
pub mod DFSDM2_CNVTIMR {
    pub use super::DFSDM0_CNVTIMR::CNVCNT;
}

/// control register 1
pub mod DFSDM3_CR1 {
    pub use super::DFSDM0_CR1::AWFSEL;
    pub use super::DFSDM0_CR1::DFEN;
    pub use super::DFSDM0_CR1::FAST;
    pub use super::DFSDM0_CR1::JDMAEN;
    pub use super::DFSDM0_CR1::JEXTEN;
    pub use super::DFSDM0_CR1::JEXTSEL;
    pub use super::DFSDM0_CR1::JSCAN;
    pub use super::DFSDM0_CR1::JSWSTART;
    pub use super::DFSDM0_CR1::JSYNC;
    pub use super::DFSDM0_CR1::RCH;
    pub use super::DFSDM0_CR1::RCONT;
    pub use super::DFSDM0_CR1::RDMAEN;
    pub use super::DFSDM0_CR1::RSWSTART;
    pub use super::DFSDM0_CR1::RSYNC;
}

/// control register 2
pub mod DFSDM3_CR2 {
    pub use super::DFSDM0_CR2::AWDCH;
    pub use super::DFSDM0_CR2::AWDIE;
    pub use super::DFSDM0_CR2::CKABIE;
    pub use super::DFSDM0_CR2::EXCH;
    pub use super::DFSDM0_CR2::JEOCIE;
    pub use super::DFSDM0_CR2::JOVRIE;
    pub use super::DFSDM0_CR2::REOCIE;
    pub use super::DFSDM0_CR2::ROVRIE;
    pub use super::DFSDM0_CR2::SCDIE;
}

/// interrupt and status register
pub mod DFSDM3_ISR {
    pub use super::DFSDM0_ISR::AWDF;
    pub use super::DFSDM0_ISR::CKABF;
    pub use super::DFSDM0_ISR::JCIP;
    pub use super::DFSDM0_ISR::JEOCF;
    pub use super::DFSDM0_ISR::JOVRF;
    pub use super::DFSDM0_ISR::RCIP;
    pub use super::DFSDM0_ISR::REOCF;
    pub use super::DFSDM0_ISR::ROVRF;
    pub use super::DFSDM0_ISR::SCDF;
}

/// interrupt flag clear register
pub mod DFSDM3_ICR {
    pub use super::DFSDM0_ICR::CLRCKABF;
    pub use super::DFSDM0_ICR::CLRJOVRF;
    pub use super::DFSDM0_ICR::CLRROVRF;
    pub use super::DFSDM0_ICR::CLRSCDF;
}

/// injected channel group selection register
pub mod DFSDM3_JCHGR {
    pub use super::DFSDM0_JCHGR::JCHG;
}

/// filter control register
pub mod DFSDM3_FCR {
    pub use super::DFSDM0_FCR::FORD;
    pub use super::DFSDM0_FCR::FOSR;
    pub use super::DFSDM0_FCR::IOSR;
}

/// data register for injected group
pub mod DFSDM3_JDATAR {
    pub use super::DFSDM0_JDATAR::JDATA;
    pub use super::DFSDM0_JDATAR::JDATACH;
}

/// data register for the regular channel
pub mod DFSDM3_RDATAR {
    pub use super::DFSDM0_RDATAR::RDATA;
    pub use super::DFSDM0_RDATAR::RDATACH;
    pub use super::DFSDM0_RDATAR::RPEND;
}

/// analog watchdog high threshold register
pub mod DFSDM3_AWHTR {
    pub use super::DFSDM0_AWHTR::AWHT;
    pub use super::DFSDM0_AWHTR::BKAWH;
}

/// analog watchdog low threshold register
pub mod DFSDM3_AWLTR {
    pub use super::DFSDM0_AWLTR::AWLT;
    pub use super::DFSDM0_AWLTR::BKAWL;
}

/// analog watchdog status register
pub mod DFSDM3_AWSR {
    pub use super::DFSDM0_AWSR::AWHTF;
    pub use super::DFSDM0_AWSR::AWLTF;
}

/// analog watchdog clear flag register
pub mod DFSDM3_AWCFR {
    pub use super::DFSDM0_AWCFR::CLRAWHTF;
    pub use super::DFSDM0_AWCFR::CLRAWLTF;
}

/// Extremes detector maximum register
pub mod DFSDM3_EXMAX {
    pub use super::DFSDM0_EXMAX::EXMAX;
    pub use super::DFSDM0_EXMAX::EXMAXCH;
}

/// Extremes detector minimum register
pub mod DFSDM3_EXMIN {
    pub use super::DFSDM0_EXMIN::EXMIN;
    pub use super::DFSDM0_EXMIN::EXMINCH;
}

/// conversion timer register
pub mod DFSDM3_CNVTIMR {
    pub use super::DFSDM0_CNVTIMR::CNVCNT;
}
pub struct RegisterBlock {
    /// channel configuration y register
    pub CHCFG0R1: RWRegister<u32>,

    /// channel configuration y register
    pub CHCFG0R2: RWRegister<u32>,

    /// analog watchdog and short-circuit detector register
    pub AWSCD0R: RWRegister<u32>,

    /// channel watchdog filter data register
    pub CHWDAT0R: RWRegister<u32>,

    /// channel data input register
    pub CHDATIN0R: RWRegister<u32>,

    _reserved1: [u32; 3],

    /// CHCFG1R1
    pub CHCFG1R1: RWRegister<u32>,

    /// CHCFG1R2
    pub CHCFG1R2: RWRegister<u32>,

    /// AWSCD1R
    pub AWSCD1R: RWRegister<u32>,

    /// CHWDAT1R
    pub CHWDAT1R: RWRegister<u32>,

    /// CHDATIN1R
    pub CHDATIN1R: RWRegister<u32>,

    _reserved2: [u32; 3],

    /// CHCFG2R1
    pub CHCFG2R1: RWRegister<u32>,

    /// CHCFG2R2
    pub CHCFG2R2: RWRegister<u32>,

    /// AWSCD2R
    pub AWSCD2R: RWRegister<u32>,

    /// CHWDAT2R
    pub CHWDAT2R: RWRegister<u32>,

    /// CHDATIN2R
    pub CHDATIN2R: RWRegister<u32>,

    _reserved3: [u32; 3],

    /// CHCFG3R1
    pub CHCFG3R1: RWRegister<u32>,

    /// CHCFG3R2
    pub CHCFG3R2: RWRegister<u32>,

    /// AWSCD3R
    pub AWSCD3R: RWRegister<u32>,

    /// CHWDAT3R
    pub CHWDAT3R: RWRegister<u32>,

    /// CHDATIN3R
    pub CHDATIN3R: RWRegister<u32>,

    _reserved4: [u32; 3],

    /// CHCFG4R1
    pub CHCFG4R1: RWRegister<u32>,

    /// CHCFG4R2
    pub CHCFG4R2: RWRegister<u32>,

    /// AWSCD4R
    pub AWSCD4R: RWRegister<u32>,

    /// CHWDAT4R
    pub CHWDAT4R: RWRegister<u32>,

    /// CHDATIN4R
    pub CHDATIN4R: RWRegister<u32>,

    _reserved5: [u32; 3],

    /// CHCFG5R1
    pub CHCFG5R1: RWRegister<u32>,

    /// CHCFG5R2
    pub CHCFG5R2: RWRegister<u32>,

    /// AWSCD5R
    pub AWSCD5R: RWRegister<u32>,

    /// CHWDAT5R
    pub CHWDAT5R: RWRegister<u32>,

    /// CHDATIN5R
    pub CHDATIN5R: RWRegister<u32>,

    _reserved6: [u32; 3],

    /// CHCFG6R1
    pub CHCFG6R1: RWRegister<u32>,

    /// CHCFG6R2
    pub CHCFG6R2: RWRegister<u32>,

    /// AWSCD6R
    pub AWSCD6R: RWRegister<u32>,

    /// CHWDAT6R
    pub CHWDAT6R: RWRegister<u32>,

    /// CHDATIN6R
    pub CHDATIN6R: RWRegister<u32>,

    _reserved7: [u32; 3],

    /// CHCFG7R1
    pub CHCFG7R1: RWRegister<u32>,

    /// CHCFG7R2
    pub CHCFG7R2: RWRegister<u32>,

    /// AWSCD7R
    pub AWSCD7R: RWRegister<u32>,

    /// CHWDAT7R
    pub CHWDAT7R: RWRegister<u32>,

    /// CHDATIN7R
    pub CHDATIN7R: RWRegister<u32>,

    _reserved8: [u32; 3],

    /// control register 1
    pub DFSDM0_CR1: RWRegister<u32>,

    /// control register 2
    pub DFSDM0_CR2: RWRegister<u32>,

    /// interrupt and status register
    pub DFSDM0_ISR: RORegister<u32>,

    /// interrupt flag clear register
    pub DFSDM0_ICR: RWRegister<u32>,

    /// injected channel group selection register
    pub DFSDM0_JCHGR: RWRegister<u32>,

    /// filter control register
    pub DFSDM0_FCR: RWRegister<u32>,

    /// data register for injected group
    pub DFSDM0_JDATAR: RORegister<u32>,

    /// data register for the regular channel
    pub DFSDM0_RDATAR: RORegister<u32>,

    /// analog watchdog high threshold register
    pub DFSDM0_AWHTR: RWRegister<u32>,

    /// analog watchdog low threshold register
    pub DFSDM0_AWLTR: RWRegister<u32>,

    /// analog watchdog status register
    pub DFSDM0_AWSR: RORegister<u32>,

    /// analog watchdog clear flag register
    pub DFSDM0_AWCFR: RWRegister<u32>,

    /// Extremes detector maximum register
    pub DFSDM0_EXMAX: RORegister<u32>,

    /// Extremes detector minimum register
    pub DFSDM0_EXMIN: RORegister<u32>,

    /// conversion timer register
    pub DFSDM0_CNVTIMR: RORegister<u32>,

    _reserved9: [u32; 49],

    /// control register 1
    pub DFSDM1_CR1: RWRegister<u32>,

    /// control register 2
    pub DFSDM1_CR2: RWRegister<u32>,

    /// interrupt and status register
    pub DFSDM1_ISR: RORegister<u32>,

    /// interrupt flag clear register
    pub DFSDM1_ICR: RWRegister<u32>,

    /// injected channel group selection register
    pub DFSDM1_JCHGR: RWRegister<u32>,

    /// filter control register
    pub DFSDM1_FCR: RWRegister<u32>,

    /// data register for injected group
    pub DFSDM1_JDATAR: RORegister<u32>,

    /// data register for the regular channel
    pub DFSDM1_RDATAR: RORegister<u32>,

    /// analog watchdog high threshold register
    pub DFSDM1_AWHTR: RWRegister<u32>,

    /// analog watchdog low threshold register
    pub DFSDM1_AWLTR: RWRegister<u32>,

    /// analog watchdog status register
    pub DFSDM1_AWSR: RORegister<u32>,

    /// analog watchdog clear flag register
    pub DFSDM1_AWCFR: RWRegister<u32>,

    /// Extremes detector maximum register
    pub DFSDM1_EXMAX: RORegister<u32>,

    /// Extremes detector minimum register
    pub DFSDM1_EXMIN: RORegister<u32>,

    /// conversion timer register
    pub DFSDM1_CNVTIMR: RORegister<u32>,

    _reserved10: [u32; 49],

    /// control register 1
    pub DFSDM2_CR1: RWRegister<u32>,

    /// control register 2
    pub DFSDM2_CR2: RWRegister<u32>,

    /// interrupt and status register
    pub DFSDM2_ISR: RORegister<u32>,

    /// interrupt flag clear register
    pub DFSDM2_ICR: RWRegister<u32>,

    /// injected channel group selection register
    pub DFSDM2_JCHGR: RWRegister<u32>,

    /// filter control register
    pub DFSDM2_FCR: RWRegister<u32>,

    /// data register for injected group
    pub DFSDM2_JDATAR: RORegister<u32>,

    /// data register for the regular channel
    pub DFSDM2_RDATAR: RORegister<u32>,

    /// analog watchdog high threshold register
    pub DFSDM2_AWHTR: RWRegister<u32>,

    /// analog watchdog low threshold register
    pub DFSDM2_AWLTR: RWRegister<u32>,

    /// analog watchdog status register
    pub DFSDM2_AWSR: RORegister<u32>,

    /// analog watchdog clear flag register
    pub DFSDM2_AWCFR: RWRegister<u32>,

    /// Extremes detector maximum register
    pub DFSDM2_EXMAX: RORegister<u32>,

    /// Extremes detector minimum register
    pub DFSDM2_EXMIN: RORegister<u32>,

    /// conversion timer register
    pub DFSDM2_CNVTIMR: RORegister<u32>,

    _reserved11: [u32; 49],

    /// control register 1
    pub DFSDM3_CR1: RWRegister<u32>,

    /// control register 2
    pub DFSDM3_CR2: RWRegister<u32>,

    /// interrupt and status register
    pub DFSDM3_ISR: RORegister<u32>,

    /// interrupt flag clear register
    pub DFSDM3_ICR: RWRegister<u32>,

    /// injected channel group selection register
    pub DFSDM3_JCHGR: RWRegister<u32>,

    /// filter control register
    pub DFSDM3_FCR: RWRegister<u32>,

    /// data register for injected group
    pub DFSDM3_JDATAR: RORegister<u32>,

    /// data register for the regular channel
    pub DFSDM3_RDATAR: RORegister<u32>,

    /// analog watchdog high threshold register
    pub DFSDM3_AWHTR: RWRegister<u32>,

    /// analog watchdog low threshold register
    pub DFSDM3_AWLTR: RWRegister<u32>,

    /// analog watchdog status register
    pub DFSDM3_AWSR: RORegister<u32>,

    /// analog watchdog clear flag register
    pub DFSDM3_AWCFR: RWRegister<u32>,

    /// Extremes detector maximum register
    pub DFSDM3_EXMAX: RORegister<u32>,

    /// Extremes detector minimum register
    pub DFSDM3_EXMIN: RORegister<u32>,

    /// conversion timer register
    pub DFSDM3_CNVTIMR: RORegister<u32>,
}
pub struct ResetValues {
    pub CHCFG0R1: u32,
    pub CHCFG0R2: u32,
    pub AWSCD0R: u32,
    pub CHWDAT0R: u32,
    pub CHDATIN0R: u32,
    pub CHCFG1R1: u32,
    pub CHCFG1R2: u32,
    pub AWSCD1R: u32,
    pub CHWDAT1R: u32,
    pub CHDATIN1R: u32,
    pub CHCFG2R1: u32,
    pub CHCFG2R2: u32,
    pub AWSCD2R: u32,
    pub CHWDAT2R: u32,
    pub CHDATIN2R: u32,
    pub CHCFG3R1: u32,
    pub CHCFG3R2: u32,
    pub AWSCD3R: u32,
    pub CHWDAT3R: u32,
    pub CHDATIN3R: u32,
    pub CHCFG4R1: u32,
    pub CHCFG4R2: u32,
    pub AWSCD4R: u32,
    pub CHWDAT4R: u32,
    pub CHDATIN4R: u32,
    pub CHCFG5R1: u32,
    pub CHCFG5R2: u32,
    pub AWSCD5R: u32,
    pub CHWDAT5R: u32,
    pub CHDATIN5R: u32,
    pub CHCFG6R1: u32,
    pub CHCFG6R2: u32,
    pub AWSCD6R: u32,
    pub CHWDAT6R: u32,
    pub CHDATIN6R: u32,
    pub CHCFG7R1: u32,
    pub CHCFG7R2: u32,
    pub AWSCD7R: u32,
    pub CHWDAT7R: u32,
    pub CHDATIN7R: u32,
    pub DFSDM0_CR1: u32,
    pub DFSDM0_CR2: u32,
    pub DFSDM0_ISR: u32,
    pub DFSDM0_ICR: u32,
    pub DFSDM0_JCHGR: u32,
    pub DFSDM0_FCR: u32,
    pub DFSDM0_JDATAR: u32,
    pub DFSDM0_RDATAR: u32,
    pub DFSDM0_AWHTR: u32,
    pub DFSDM0_AWLTR: u32,
    pub DFSDM0_AWSR: u32,
    pub DFSDM0_AWCFR: u32,
    pub DFSDM0_EXMAX: u32,
    pub DFSDM0_EXMIN: u32,
    pub DFSDM0_CNVTIMR: u32,
    pub DFSDM1_CR1: u32,
    pub DFSDM1_CR2: u32,
    pub DFSDM1_ISR: u32,
    pub DFSDM1_ICR: u32,
    pub DFSDM1_JCHGR: u32,
    pub DFSDM1_FCR: u32,
    pub DFSDM1_JDATAR: u32,
    pub DFSDM1_RDATAR: u32,
    pub DFSDM1_AWHTR: u32,
    pub DFSDM1_AWLTR: u32,
    pub DFSDM1_AWSR: u32,
    pub DFSDM1_AWCFR: u32,
    pub DFSDM1_EXMAX: u32,
    pub DFSDM1_EXMIN: u32,
    pub DFSDM1_CNVTIMR: u32,
    pub DFSDM2_CR1: u32,
    pub DFSDM2_CR2: u32,
    pub DFSDM2_ISR: u32,
    pub DFSDM2_ICR: u32,
    pub DFSDM2_JCHGR: u32,
    pub DFSDM2_FCR: u32,
    pub DFSDM2_JDATAR: u32,
    pub DFSDM2_RDATAR: u32,
    pub DFSDM2_AWHTR: u32,
    pub DFSDM2_AWLTR: u32,
    pub DFSDM2_AWSR: u32,
    pub DFSDM2_AWCFR: u32,
    pub DFSDM2_EXMAX: u32,
    pub DFSDM2_EXMIN: u32,
    pub DFSDM2_CNVTIMR: u32,
    pub DFSDM3_CR1: u32,
    pub DFSDM3_CR2: u32,
    pub DFSDM3_ISR: u32,
    pub DFSDM3_ICR: u32,
    pub DFSDM3_JCHGR: u32,
    pub DFSDM3_FCR: u32,
    pub DFSDM3_JDATAR: u32,
    pub DFSDM3_RDATAR: u32,
    pub DFSDM3_AWHTR: u32,
    pub DFSDM3_AWLTR: u32,
    pub DFSDM3_AWSR: u32,
    pub DFSDM3_AWCFR: u32,
    pub DFSDM3_EXMAX: u32,
    pub DFSDM3_EXMIN: u32,
    pub DFSDM3_CNVTIMR: u32,
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

/// Access functions for the DFSDM1 peripheral instance
pub mod DFSDM1 {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

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
        CHCFG0R1: 0x00000000,
        CHCFG0R2: 0x00000000,
        AWSCD0R: 0x00000000,
        CHWDAT0R: 0x00000000,
        CHDATIN0R: 0x00000000,
        CHCFG1R1: 0x00000000,
        CHCFG1R2: 0x00000000,
        AWSCD1R: 0x00000000,
        CHWDAT1R: 0x00000000,
        CHDATIN1R: 0x00000000,
        CHCFG2R1: 0x00000000,
        CHCFG2R2: 0x00000000,
        AWSCD2R: 0x00000000,
        CHWDAT2R: 0x00000000,
        CHDATIN2R: 0x00000000,
        CHCFG3R1: 0x00000000,
        CHCFG3R2: 0x00000000,
        AWSCD3R: 0x00000000,
        CHWDAT3R: 0x00000000,
        CHDATIN3R: 0x00000000,
        CHCFG4R1: 0x00000000,
        CHCFG4R2: 0x00000000,
        AWSCD4R: 0x00000000,
        CHWDAT4R: 0x00000000,
        CHDATIN4R: 0x00000000,
        CHCFG5R1: 0x00000000,
        CHCFG5R2: 0x00000000,
        AWSCD5R: 0x00000000,
        CHWDAT5R: 0x00000000,
        CHDATIN5R: 0x00000000,
        CHCFG6R1: 0x00000000,
        CHCFG6R2: 0x00000000,
        AWSCD6R: 0x00000000,
        CHWDAT6R: 0x00000000,
        CHDATIN6R: 0x00000000,
        CHCFG7R1: 0x00000000,
        CHCFG7R2: 0x00000000,
        AWSCD7R: 0x00000000,
        CHWDAT7R: 0x00000000,
        CHDATIN7R: 0x00000000,
        DFSDM0_CR1: 0x00000000,
        DFSDM0_CR2: 0x00000000,
        DFSDM0_ISR: 0x00FF0000,
        DFSDM0_ICR: 0x00000000,
        DFSDM0_JCHGR: 0x00000001,
        DFSDM0_FCR: 0x00000000,
        DFSDM0_JDATAR: 0x00000000,
        DFSDM0_RDATAR: 0x00000000,
        DFSDM0_AWHTR: 0x00000000,
        DFSDM0_AWLTR: 0x00000000,
        DFSDM0_AWSR: 0x00000000,
        DFSDM0_AWCFR: 0x00000000,
        DFSDM0_EXMAX: 0x80000000,
        DFSDM0_EXMIN: 0x7FFFFF00,
        DFSDM0_CNVTIMR: 0x00000000,
        DFSDM1_CR1: 0x00000000,
        DFSDM1_CR2: 0x00000000,
        DFSDM1_ISR: 0x00FF0000,
        DFSDM1_ICR: 0x00000000,
        DFSDM1_JCHGR: 0x00000001,
        DFSDM1_FCR: 0x00000000,
        DFSDM1_JDATAR: 0x00000000,
        DFSDM1_RDATAR: 0x00000000,
        DFSDM1_AWHTR: 0x00000000,
        DFSDM1_AWLTR: 0x00000000,
        DFSDM1_AWSR: 0x00000000,
        DFSDM1_AWCFR: 0x00000000,
        DFSDM1_EXMAX: 0x80000000,
        DFSDM1_EXMIN: 0x7FFFFF00,
        DFSDM1_CNVTIMR: 0x00000000,
        DFSDM2_CR1: 0x00000000,
        DFSDM2_CR2: 0x00000000,
        DFSDM2_ISR: 0x00FF0000,
        DFSDM2_ICR: 0x00000000,
        DFSDM2_JCHGR: 0x00000001,
        DFSDM2_FCR: 0x00000000,
        DFSDM2_JDATAR: 0x00000000,
        DFSDM2_RDATAR: 0x00000000,
        DFSDM2_AWHTR: 0x00000000,
        DFSDM2_AWLTR: 0x00000000,
        DFSDM2_AWSR: 0x00000000,
        DFSDM2_AWCFR: 0x00000000,
        DFSDM2_EXMAX: 0x80000000,
        DFSDM2_EXMIN: 0x7FFFFF00,
        DFSDM2_CNVTIMR: 0x00000000,
        DFSDM3_CR1: 0x00000000,
        DFSDM3_CR2: 0x00000000,
        DFSDM3_ISR: 0x00FF0000,
        DFSDM3_ICR: 0x00000000,
        DFSDM3_JCHGR: 0x00000001,
        DFSDM3_FCR: 0x00000000,
        DFSDM3_JDATAR: 0x00000000,
        DFSDM3_RDATAR: 0x00000000,
        DFSDM3_AWHTR: 0x00000000,
        DFSDM3_AWLTR: 0x00000000,
        DFSDM3_AWSR: 0x00000000,
        DFSDM3_AWCFR: 0x00000000,
        DFSDM3_EXMAX: 0x80000000,
        DFSDM3_EXMIN: 0x7FFFFF00,
        DFSDM3_CNVTIMR: 0x00000000,
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
