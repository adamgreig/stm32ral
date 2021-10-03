#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Digital filter for sigma delta modulators
//!
//! Used by: stm32l412, stm32l4x1, stm32l4x2, stm32l4x5

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// channel configuration y register
pub mod CFGR10 {

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
pub mod CFGR20 {

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
pub mod AWSCDR0 {

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
pub mod WDATR0 {

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
pub mod DATINR0 {

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

/// channel configuration y register
pub mod CFGR11 {
    pub use super::CFGR10::CHEN;
    pub use super::CFGR10::CHINSEL;
    pub use super::CFGR10::CKABEN;
    pub use super::CFGR10::CKOUTDIV;
    pub use super::CFGR10::CKOUTSRC;
    pub use super::CFGR10::DATMPX;
    pub use super::CFGR10::DATPACK;
    pub use super::CFGR10::DFSDMEN;
    pub use super::CFGR10::SCDEN;
    pub use super::CFGR10::SITP;
    pub use super::CFGR10::SPICKSEL;
}

/// channel configuration y register
pub mod CFGR21 {
    pub use super::CFGR20::DTRBS;
    pub use super::CFGR20::OFFSET;
}

/// analog watchdog and short-circuit detector register
pub mod AWSCDR1 {
    pub use super::AWSCDR0::AWFORD;
    pub use super::AWSCDR0::AWFOSR;
    pub use super::AWSCDR0::BKSCD;
    pub use super::AWSCDR0::SCDT;
}

/// channel watchdog filter data register
pub mod WDATR1 {
    pub use super::WDATR0::WDATA;
}

/// channel data input register
pub mod DATINR1 {
    pub use super::DATINR0::INDAT0;
    pub use super::DATINR0::INDAT1;
}

/// channel configuration y register
pub mod CFGR12 {
    pub use super::CFGR10::CHEN;
    pub use super::CFGR10::CHINSEL;
    pub use super::CFGR10::CKABEN;
    pub use super::CFGR10::CKOUTDIV;
    pub use super::CFGR10::CKOUTSRC;
    pub use super::CFGR10::DATMPX;
    pub use super::CFGR10::DATPACK;
    pub use super::CFGR10::DFSDMEN;
    pub use super::CFGR10::SCDEN;
    pub use super::CFGR10::SITP;
    pub use super::CFGR10::SPICKSEL;
}

/// channel configuration y register
pub mod CFGR22 {
    pub use super::CFGR20::DTRBS;
    pub use super::CFGR20::OFFSET;
}

/// analog watchdog and short-circuit detector register
pub mod AWSCDR2 {
    pub use super::AWSCDR0::AWFORD;
    pub use super::AWSCDR0::AWFOSR;
    pub use super::AWSCDR0::BKSCD;
    pub use super::AWSCDR0::SCDT;
}

/// channel watchdog filter data register
pub mod WDATR2 {
    pub use super::WDATR0::WDATA;
}

/// channel data input register
pub mod DATINR2 {
    pub use super::DATINR0::INDAT0;
    pub use super::DATINR0::INDAT1;
}

/// channel configuration y register
pub mod CFGR13 {
    pub use super::CFGR10::CHEN;
    pub use super::CFGR10::CHINSEL;
    pub use super::CFGR10::CKABEN;
    pub use super::CFGR10::CKOUTDIV;
    pub use super::CFGR10::CKOUTSRC;
    pub use super::CFGR10::DATMPX;
    pub use super::CFGR10::DATPACK;
    pub use super::CFGR10::DFSDMEN;
    pub use super::CFGR10::SCDEN;
    pub use super::CFGR10::SITP;
    pub use super::CFGR10::SPICKSEL;
}

/// channel configuration y register
pub mod CFGR23 {
    pub use super::CFGR20::DTRBS;
    pub use super::CFGR20::OFFSET;
}

/// analog watchdog and short-circuit detector register
pub mod AWSCDR3 {
    pub use super::AWSCDR0::AWFORD;
    pub use super::AWSCDR0::AWFOSR;
    pub use super::AWSCDR0::BKSCD;
    pub use super::AWSCDR0::SCDT;
}

/// channel watchdog filter data register
pub mod WDATR3 {
    pub use super::WDATR0::WDATA;
}

/// channel data input register
pub mod DATINR3 {
    pub use super::DATINR0::INDAT0;
    pub use super::DATINR0::INDAT1;
}

/// channel configuration y register
pub mod CFGR14 {
    pub use super::CFGR10::CHEN;
    pub use super::CFGR10::CHINSEL;
    pub use super::CFGR10::CKABEN;
    pub use super::CFGR10::CKOUTDIV;
    pub use super::CFGR10::CKOUTSRC;
    pub use super::CFGR10::DATMPX;
    pub use super::CFGR10::DATPACK;
    pub use super::CFGR10::DFSDMEN;
    pub use super::CFGR10::SCDEN;
    pub use super::CFGR10::SITP;
    pub use super::CFGR10::SPICKSEL;
}

/// channel configuration y register
pub mod CFGR24 {
    pub use super::CFGR20::DTRBS;
    pub use super::CFGR20::OFFSET;
}

/// analog watchdog and short-circuit detector register
pub mod AWSCDR4 {
    pub use super::AWSCDR0::AWFORD;
    pub use super::AWSCDR0::AWFOSR;
    pub use super::AWSCDR0::BKSCD;
    pub use super::AWSCDR0::SCDT;
}

/// channel watchdog filter data register
pub mod WDATR4 {
    pub use super::WDATR0::WDATA;
}

/// channel data input register
pub mod DATINR4 {
    pub use super::DATINR0::INDAT0;
    pub use super::DATINR0::INDAT1;
}

/// channel configuration y register
pub mod CFGR15 {
    pub use super::CFGR10::CHEN;
    pub use super::CFGR10::CHINSEL;
    pub use super::CFGR10::CKABEN;
    pub use super::CFGR10::CKOUTDIV;
    pub use super::CFGR10::CKOUTSRC;
    pub use super::CFGR10::DATMPX;
    pub use super::CFGR10::DATPACK;
    pub use super::CFGR10::DFSDMEN;
    pub use super::CFGR10::SCDEN;
    pub use super::CFGR10::SITP;
    pub use super::CFGR10::SPICKSEL;
}

/// channel configuration y register
pub mod CFGR25 {
    pub use super::CFGR20::DTRBS;
    pub use super::CFGR20::OFFSET;
}

/// analog watchdog and short-circuit detector register
pub mod AWSCDR5 {
    pub use super::AWSCDR0::AWFORD;
    pub use super::AWSCDR0::AWFOSR;
    pub use super::AWSCDR0::BKSCD;
    pub use super::AWSCDR0::SCDT;
}

/// channel watchdog filter data register
pub mod WDATR5 {
    pub use super::WDATR0::WDATA;
}

/// channel data input register
pub mod DATINR5 {
    pub use super::DATINR0::INDAT0;
    pub use super::DATINR0::INDAT1;
}

/// channel configuration y register
pub mod CFGR16 {
    pub use super::CFGR10::CHEN;
    pub use super::CFGR10::CHINSEL;
    pub use super::CFGR10::CKABEN;
    pub use super::CFGR10::CKOUTDIV;
    pub use super::CFGR10::CKOUTSRC;
    pub use super::CFGR10::DATMPX;
    pub use super::CFGR10::DATPACK;
    pub use super::CFGR10::DFSDMEN;
    pub use super::CFGR10::SCDEN;
    pub use super::CFGR10::SITP;
    pub use super::CFGR10::SPICKSEL;
}

/// channel configuration y register
pub mod CFGR26 {
    pub use super::CFGR20::DTRBS;
    pub use super::CFGR20::OFFSET;
}

/// analog watchdog and short-circuit detector register
pub mod AWSCDR6 {
    pub use super::AWSCDR0::AWFORD;
    pub use super::AWSCDR0::AWFOSR;
    pub use super::AWSCDR0::BKSCD;
    pub use super::AWSCDR0::SCDT;
}

/// channel watchdog filter data register
pub mod WDATR6 {
    pub use super::WDATR0::WDATA;
}

/// channel data input register
pub mod DATINR6 {
    pub use super::DATINR0::INDAT0;
    pub use super::DATINR0::INDAT1;
}

/// channel configuration y register
pub mod CFGR17 {
    pub use super::CFGR10::CHEN;
    pub use super::CFGR10::CHINSEL;
    pub use super::CFGR10::CKABEN;
    pub use super::CFGR10::CKOUTDIV;
    pub use super::CFGR10::CKOUTSRC;
    pub use super::CFGR10::DATMPX;
    pub use super::CFGR10::DATPACK;
    pub use super::CFGR10::DFSDMEN;
    pub use super::CFGR10::SCDEN;
    pub use super::CFGR10::SITP;
    pub use super::CFGR10::SPICKSEL;
}

/// channel configuration y register
pub mod CFGR27 {
    pub use super::CFGR20::DTRBS;
    pub use super::CFGR20::OFFSET;
}

/// analog watchdog and short-circuit detector register
pub mod AWSCDR7 {
    pub use super::AWSCDR0::AWFORD;
    pub use super::AWSCDR0::AWFOSR;
    pub use super::AWSCDR0::BKSCD;
    pub use super::AWSCDR0::SCDT;
}

/// channel watchdog filter data register
pub mod WDATR7 {
    pub use super::WDATR0::WDATA;
}

/// channel data input register
pub mod DATINR7 {
    pub use super::DATINR0::INDAT0;
    pub use super::DATINR0::INDAT1;
}

/// control register 1
pub mod CR10 {

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
pub mod CR20 {

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
pub mod ISR0 {

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
pub mod ICR0 {

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
pub mod JCHGR0 {

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
pub mod FCR0 {

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
pub mod JDATAR0 {

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
pub mod RDATAR0 {

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
pub mod AWHTR0 {

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
pub mod AWLTR0 {

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
pub mod AWSR0 {

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
pub mod AWCFR0 {

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
pub mod EXMAX0 {

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
pub mod EXMIN0 {

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
pub mod CNVTIMR0 {

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
pub mod CR11 {
    pub use super::CR10::AWFSEL;
    pub use super::CR10::DFEN;
    pub use super::CR10::FAST;
    pub use super::CR10::JDMAEN;
    pub use super::CR10::JEXTEN;
    pub use super::CR10::JEXTSEL;
    pub use super::CR10::JSCAN;
    pub use super::CR10::JSWSTART;
    pub use super::CR10::JSYNC;
    pub use super::CR10::RCH;
    pub use super::CR10::RCONT;
    pub use super::CR10::RDMAEN;
    pub use super::CR10::RSWSTART;
    pub use super::CR10::RSYNC;
}

/// control register 2
pub mod CR21 {
    pub use super::CR20::AWDCH;
    pub use super::CR20::AWDIE;
    pub use super::CR20::CKABIE;
    pub use super::CR20::EXCH;
    pub use super::CR20::JEOCIE;
    pub use super::CR20::JOVRIE;
    pub use super::CR20::REOCIE;
    pub use super::CR20::ROVRIE;
    pub use super::CR20::SCDIE;
}

/// interrupt and status register
pub mod ISR1 {
    pub use super::ISR0::AWDF;
    pub use super::ISR0::CKABF;
    pub use super::ISR0::JCIP;
    pub use super::ISR0::JEOCF;
    pub use super::ISR0::JOVRF;
    pub use super::ISR0::RCIP;
    pub use super::ISR0::REOCF;
    pub use super::ISR0::ROVRF;
    pub use super::ISR0::SCDF;
}

/// interrupt flag clear register
pub mod ICR1 {
    pub use super::ICR0::CLRCKABF;
    pub use super::ICR0::CLRJOVRF;
    pub use super::ICR0::CLRROVRF;
    pub use super::ICR0::CLRSCDF;
}

/// injected channel group selection register
pub mod JCHGR1 {
    pub use super::JCHGR0::JCHG;
}

/// filter control register
pub mod FCR1 {
    pub use super::FCR0::FORD;
    pub use super::FCR0::FOSR;
    pub use super::FCR0::IOSR;
}

/// data register for injected group
pub mod JDATAR1 {
    pub use super::JDATAR0::JDATA;
    pub use super::JDATAR0::JDATACH;
}

/// data register for the regular channel
pub mod RDATAR1 {
    pub use super::RDATAR0::RDATA;
    pub use super::RDATAR0::RDATACH;
    pub use super::RDATAR0::RPEND;
}

/// analog watchdog high threshold register
pub mod AWHTR1 {
    pub use super::AWHTR0::AWHT;
    pub use super::AWHTR0::BKAWH;
}

/// analog watchdog low threshold register
pub mod AWLTR1 {
    pub use super::AWLTR0::AWLT;
    pub use super::AWLTR0::BKAWL;
}

/// analog watchdog status register
pub mod AWSR1 {
    pub use super::AWSR0::AWHTF;
    pub use super::AWSR0::AWLTF;
}

/// analog watchdog clear flag register
pub mod AWCFR1 {
    pub use super::AWCFR0::CLRAWHTF;
    pub use super::AWCFR0::CLRAWLTF;
}

/// Extremes detector maximum register
pub mod EXMAX1 {
    pub use super::EXMAX0::EXMAX;
    pub use super::EXMAX0::EXMAXCH;
}

/// Extremes detector minimum register
pub mod EXMIN1 {
    pub use super::EXMIN0::EXMIN;
    pub use super::EXMIN0::EXMINCH;
}

/// conversion timer register
pub mod CNVTIMR1 {
    pub use super::CNVTIMR0::CNVCNT;
}

/// control register 1
pub mod CR12 {
    pub use super::CR10::AWFSEL;
    pub use super::CR10::DFEN;
    pub use super::CR10::FAST;
    pub use super::CR10::JDMAEN;
    pub use super::CR10::JEXTEN;
    pub use super::CR10::JEXTSEL;
    pub use super::CR10::JSCAN;
    pub use super::CR10::JSWSTART;
    pub use super::CR10::JSYNC;
    pub use super::CR10::RCH;
    pub use super::CR10::RCONT;
    pub use super::CR10::RDMAEN;
    pub use super::CR10::RSWSTART;
    pub use super::CR10::RSYNC;
}

/// control register 2
pub mod CR22 {
    pub use super::CR20::AWDCH;
    pub use super::CR20::AWDIE;
    pub use super::CR20::CKABIE;
    pub use super::CR20::EXCH;
    pub use super::CR20::JEOCIE;
    pub use super::CR20::JOVRIE;
    pub use super::CR20::REOCIE;
    pub use super::CR20::ROVRIE;
    pub use super::CR20::SCDIE;
}

/// interrupt and status register
pub mod ISR2 {
    pub use super::ISR0::AWDF;
    pub use super::ISR0::CKABF;
    pub use super::ISR0::JCIP;
    pub use super::ISR0::JEOCF;
    pub use super::ISR0::JOVRF;
    pub use super::ISR0::RCIP;
    pub use super::ISR0::REOCF;
    pub use super::ISR0::ROVRF;
    pub use super::ISR0::SCDF;
}

/// interrupt flag clear register
pub mod ICR2 {
    pub use super::ICR0::CLRCKABF;
    pub use super::ICR0::CLRJOVRF;
    pub use super::ICR0::CLRROVRF;
    pub use super::ICR0::CLRSCDF;
}

/// injected channel group selection register
pub mod JCHGR2 {
    pub use super::JCHGR0::JCHG;
}

/// filter control register
pub mod FCR2 {
    pub use super::FCR0::FORD;
    pub use super::FCR0::FOSR;
    pub use super::FCR0::IOSR;
}

/// data register for injected group
pub mod JDATAR2 {
    pub use super::JDATAR0::JDATA;
    pub use super::JDATAR0::JDATACH;
}

/// data register for the regular channel
pub mod RDATAR2 {
    pub use super::RDATAR0::RDATA;
    pub use super::RDATAR0::RDATACH;
    pub use super::RDATAR0::RPEND;
}

/// analog watchdog high threshold register
pub mod AWHTR2 {
    pub use super::AWHTR0::AWHT;
    pub use super::AWHTR0::BKAWH;
}

/// analog watchdog low threshold register
pub mod AWLTR2 {
    pub use super::AWLTR0::AWLT;
    pub use super::AWLTR0::BKAWL;
}

/// analog watchdog status register
pub mod AWSR2 {
    pub use super::AWSR0::AWHTF;
    pub use super::AWSR0::AWLTF;
}

/// analog watchdog clear flag register
pub mod AWCFR2 {
    pub use super::AWCFR0::CLRAWHTF;
    pub use super::AWCFR0::CLRAWLTF;
}

/// Extremes detector maximum register
pub mod EXMAX2 {
    pub use super::EXMAX0::EXMAX;
    pub use super::EXMAX0::EXMAXCH;
}

/// Extremes detector minimum register
pub mod EXMIN2 {
    pub use super::EXMIN0::EXMIN;
    pub use super::EXMIN0::EXMINCH;
}

/// conversion timer register
pub mod CNVTIMR2 {
    pub use super::CNVTIMR0::CNVCNT;
}

/// control register 1
pub mod CR13 {
    pub use super::CR10::AWFSEL;
    pub use super::CR10::DFEN;
    pub use super::CR10::FAST;
    pub use super::CR10::JDMAEN;
    pub use super::CR10::JEXTEN;
    pub use super::CR10::JEXTSEL;
    pub use super::CR10::JSCAN;
    pub use super::CR10::JSWSTART;
    pub use super::CR10::JSYNC;
    pub use super::CR10::RCH;
    pub use super::CR10::RCONT;
    pub use super::CR10::RDMAEN;
    pub use super::CR10::RSWSTART;
    pub use super::CR10::RSYNC;
}

/// control register 2
pub mod CR23 {
    pub use super::CR20::AWDCH;
    pub use super::CR20::AWDIE;
    pub use super::CR20::CKABIE;
    pub use super::CR20::EXCH;
    pub use super::CR20::JEOCIE;
    pub use super::CR20::JOVRIE;
    pub use super::CR20::REOCIE;
    pub use super::CR20::ROVRIE;
    pub use super::CR20::SCDIE;
}

/// interrupt and status register
pub mod ISR3 {
    pub use super::ISR0::AWDF;
    pub use super::ISR0::CKABF;
    pub use super::ISR0::JCIP;
    pub use super::ISR0::JEOCF;
    pub use super::ISR0::JOVRF;
    pub use super::ISR0::RCIP;
    pub use super::ISR0::REOCF;
    pub use super::ISR0::ROVRF;
    pub use super::ISR0::SCDF;
}

/// interrupt flag clear register
pub mod ICR3 {
    pub use super::ICR0::CLRCKABF;
    pub use super::ICR0::CLRJOVRF;
    pub use super::ICR0::CLRROVRF;
    pub use super::ICR0::CLRSCDF;
}

/// injected channel group selection register
pub mod JCHGR3 {
    pub use super::JCHGR0::JCHG;
}

/// filter control register
pub mod FCR3 {
    pub use super::FCR0::FORD;
    pub use super::FCR0::FOSR;
    pub use super::FCR0::IOSR;
}

/// data register for injected group
pub mod JDATAR3 {
    pub use super::JDATAR0::JDATA;
    pub use super::JDATAR0::JDATACH;
}

/// data register for the regular channel
pub mod RDATAR3 {
    pub use super::RDATAR0::RDATA;
    pub use super::RDATAR0::RDATACH;
    pub use super::RDATAR0::RPEND;
}

/// analog watchdog high threshold register
pub mod AWHTR3 {
    pub use super::AWHTR0::AWHT;
    pub use super::AWHTR0::BKAWH;
}

/// analog watchdog low threshold register
pub mod AWLTR3 {
    pub use super::AWLTR0::AWLT;
    pub use super::AWLTR0::BKAWL;
}

/// analog watchdog status register
pub mod AWSR3 {
    pub use super::AWSR0::AWHTF;
    pub use super::AWSR0::AWLTF;
}

/// analog watchdog clear flag register
pub mod AWCFR3 {
    pub use super::AWCFR0::CLRAWHTF;
    pub use super::AWCFR0::CLRAWLTF;
}

/// Extremes detector maximum register
pub mod EXMAX3 {
    pub use super::EXMAX0::EXMAX;
    pub use super::EXMAX0::EXMAXCH;
}

/// Extremes detector minimum register
pub mod EXMIN3 {
    pub use super::EXMIN0::EXMIN;
    pub use super::EXMIN0::EXMINCH;
}

/// conversion timer register
pub mod CNVTIMR3 {
    pub use super::CNVTIMR0::CNVCNT;
}
#[repr(C)]
pub struct RegisterBlock {
    /// channel configuration y register
    pub CFGR10: RWRegister<u32>,

    /// channel configuration y register
    pub CFGR20: RWRegister<u32>,

    /// analog watchdog and short-circuit detector register
    pub AWSCDR0: RWRegister<u32>,

    /// channel watchdog filter data register
    pub WDATR0: RWRegister<u32>,

    /// channel data input register
    pub DATINR0: RWRegister<u32>,

    _reserved1: [u32; 3],

    /// channel configuration y register
    pub CFGR11: RWRegister<u32>,

    /// channel configuration y register
    pub CFGR21: RWRegister<u32>,

    /// analog watchdog and short-circuit detector register
    pub AWSCDR1: RWRegister<u32>,

    /// channel watchdog filter data register
    pub WDATR1: RWRegister<u32>,

    /// channel data input register
    pub DATINR1: RWRegister<u32>,

    _reserved2: [u32; 3],

    /// channel configuration y register
    pub CFGR12: RWRegister<u32>,

    /// channel configuration y register
    pub CFGR22: RWRegister<u32>,

    /// analog watchdog and short-circuit detector register
    pub AWSCDR2: RWRegister<u32>,

    /// channel watchdog filter data register
    pub WDATR2: RWRegister<u32>,

    /// channel data input register
    pub DATINR2: RWRegister<u32>,

    _reserved3: [u32; 3],

    /// channel configuration y register
    pub CFGR13: RWRegister<u32>,

    /// channel configuration y register
    pub CFGR23: RWRegister<u32>,

    /// analog watchdog and short-circuit detector register
    pub AWSCDR3: RWRegister<u32>,

    /// channel watchdog filter data register
    pub WDATR3: RWRegister<u32>,

    /// channel data input register
    pub DATINR3: RWRegister<u32>,

    _reserved4: [u32; 3],

    /// channel configuration y register
    pub CFGR14: RWRegister<u32>,

    /// channel configuration y register
    pub CFGR24: RWRegister<u32>,

    /// analog watchdog and short-circuit detector register
    pub AWSCDR4: RWRegister<u32>,

    /// channel watchdog filter data register
    pub WDATR4: RWRegister<u32>,

    /// channel data input register
    pub DATINR4: RWRegister<u32>,

    _reserved5: [u32; 3],

    /// channel configuration y register
    pub CFGR15: RWRegister<u32>,

    /// channel configuration y register
    pub CFGR25: RWRegister<u32>,

    /// analog watchdog and short-circuit detector register
    pub AWSCDR5: RWRegister<u32>,

    /// channel watchdog filter data register
    pub WDATR5: RWRegister<u32>,

    /// channel data input register
    pub DATINR5: RWRegister<u32>,

    _reserved6: [u32; 3],

    /// channel configuration y register
    pub CFGR16: RWRegister<u32>,

    /// channel configuration y register
    pub CFGR26: RWRegister<u32>,

    /// analog watchdog and short-circuit detector register
    pub AWSCDR6: RWRegister<u32>,

    /// channel watchdog filter data register
    pub WDATR6: RWRegister<u32>,

    /// channel data input register
    pub DATINR6: RWRegister<u32>,

    _reserved7: [u32; 3],

    /// channel configuration y register
    pub CFGR17: RWRegister<u32>,

    /// channel configuration y register
    pub CFGR27: RWRegister<u32>,

    /// analog watchdog and short-circuit detector register
    pub AWSCDR7: RWRegister<u32>,

    /// channel watchdog filter data register
    pub WDATR7: RWRegister<u32>,

    /// channel data input register
    pub DATINR7: RWRegister<u32>,

    _reserved8: [u32; 3],

    /// control register 1
    pub CR10: RWRegister<u32>,

    /// control register 2
    pub CR20: RWRegister<u32>,

    /// interrupt and status register
    pub ISR0: RORegister<u32>,

    /// interrupt flag clear register
    pub ICR0: RWRegister<u32>,

    /// injected channel group selection register
    pub JCHGR0: RWRegister<u32>,

    /// filter control register
    pub FCR0: RWRegister<u32>,

    /// data register for injected group
    pub JDATAR0: RORegister<u32>,

    /// data register for the regular channel
    pub RDATAR0: RORegister<u32>,

    /// analog watchdog high threshold register
    pub AWHTR0: RWRegister<u32>,

    /// analog watchdog low threshold register
    pub AWLTR0: RWRegister<u32>,

    /// analog watchdog status register
    pub AWSR0: RORegister<u32>,

    /// analog watchdog clear flag register
    pub AWCFR0: RWRegister<u32>,

    /// Extremes detector maximum register
    pub EXMAX0: RORegister<u32>,

    /// Extremes detector minimum register
    pub EXMIN0: RORegister<u32>,

    /// conversion timer register
    pub CNVTIMR0: RORegister<u32>,

    _reserved9: [u32; 49],

    /// control register 1
    pub CR11: RWRegister<u32>,

    /// control register 2
    pub CR21: RWRegister<u32>,

    /// interrupt and status register
    pub ISR1: RORegister<u32>,

    /// interrupt flag clear register
    pub ICR1: RWRegister<u32>,

    /// injected channel group selection register
    pub JCHGR1: RWRegister<u32>,

    /// filter control register
    pub FCR1: RWRegister<u32>,

    /// data register for injected group
    pub JDATAR1: RORegister<u32>,

    /// data register for the regular channel
    pub RDATAR1: RORegister<u32>,

    /// analog watchdog high threshold register
    pub AWHTR1: RWRegister<u32>,

    /// analog watchdog low threshold register
    pub AWLTR1: RWRegister<u32>,

    /// analog watchdog status register
    pub AWSR1: RORegister<u32>,

    /// analog watchdog clear flag register
    pub AWCFR1: RWRegister<u32>,

    /// Extremes detector maximum register
    pub EXMAX1: RORegister<u32>,

    /// Extremes detector minimum register
    pub EXMIN1: RORegister<u32>,

    /// conversion timer register
    pub CNVTIMR1: RORegister<u32>,

    _reserved10: [u32; 49],

    /// control register 1
    pub CR12: RWRegister<u32>,

    /// control register 2
    pub CR22: RWRegister<u32>,

    /// interrupt and status register
    pub ISR2: RORegister<u32>,

    /// interrupt flag clear register
    pub ICR2: RWRegister<u32>,

    /// injected channel group selection register
    pub JCHGR2: RWRegister<u32>,

    /// filter control register
    pub FCR2: RWRegister<u32>,

    /// data register for injected group
    pub JDATAR2: RORegister<u32>,

    /// data register for the regular channel
    pub RDATAR2: RORegister<u32>,

    /// analog watchdog high threshold register
    pub AWHTR2: RWRegister<u32>,

    /// analog watchdog low threshold register
    pub AWLTR2: RWRegister<u32>,

    /// analog watchdog status register
    pub AWSR2: RORegister<u32>,

    /// analog watchdog clear flag register
    pub AWCFR2: RWRegister<u32>,

    /// Extremes detector maximum register
    pub EXMAX2: RORegister<u32>,

    /// Extremes detector minimum register
    pub EXMIN2: RORegister<u32>,

    /// conversion timer register
    pub CNVTIMR2: RORegister<u32>,

    _reserved11: [u32; 49],

    /// control register 1
    pub CR13: RWRegister<u32>,

    /// control register 2
    pub CR23: RWRegister<u32>,

    /// interrupt and status register
    pub ISR3: RORegister<u32>,

    /// interrupt flag clear register
    pub ICR3: RWRegister<u32>,

    /// injected channel group selection register
    pub JCHGR3: RWRegister<u32>,

    /// filter control register
    pub FCR3: RWRegister<u32>,

    /// data register for injected group
    pub JDATAR3: RORegister<u32>,

    /// data register for the regular channel
    pub RDATAR3: RORegister<u32>,

    /// analog watchdog high threshold register
    pub AWHTR3: RWRegister<u32>,

    /// analog watchdog low threshold register
    pub AWLTR3: RWRegister<u32>,

    /// analog watchdog status register
    pub AWSR3: RORegister<u32>,

    /// analog watchdog clear flag register
    pub AWCFR3: RWRegister<u32>,

    /// Extremes detector maximum register
    pub EXMAX3: RORegister<u32>,

    /// Extremes detector minimum register
    pub EXMIN3: RORegister<u32>,

    /// conversion timer register
    pub CNVTIMR3: RORegister<u32>,
}
pub struct ResetValues {
    pub CFGR10: u32,
    pub CFGR20: u32,
    pub AWSCDR0: u32,
    pub WDATR0: u32,
    pub DATINR0: u32,
    pub CFGR11: u32,
    pub CFGR21: u32,
    pub AWSCDR1: u32,
    pub WDATR1: u32,
    pub DATINR1: u32,
    pub CFGR12: u32,
    pub CFGR22: u32,
    pub AWSCDR2: u32,
    pub WDATR2: u32,
    pub DATINR2: u32,
    pub CFGR13: u32,
    pub CFGR23: u32,
    pub AWSCDR3: u32,
    pub WDATR3: u32,
    pub DATINR3: u32,
    pub CFGR14: u32,
    pub CFGR24: u32,
    pub AWSCDR4: u32,
    pub WDATR4: u32,
    pub DATINR4: u32,
    pub CFGR15: u32,
    pub CFGR25: u32,
    pub AWSCDR5: u32,
    pub WDATR5: u32,
    pub DATINR5: u32,
    pub CFGR16: u32,
    pub CFGR26: u32,
    pub AWSCDR6: u32,
    pub WDATR6: u32,
    pub DATINR6: u32,
    pub CFGR17: u32,
    pub CFGR27: u32,
    pub AWSCDR7: u32,
    pub WDATR7: u32,
    pub DATINR7: u32,
    pub CR10: u32,
    pub CR20: u32,
    pub ISR0: u32,
    pub ICR0: u32,
    pub JCHGR0: u32,
    pub FCR0: u32,
    pub JDATAR0: u32,
    pub RDATAR0: u32,
    pub AWHTR0: u32,
    pub AWLTR0: u32,
    pub AWSR0: u32,
    pub AWCFR0: u32,
    pub EXMAX0: u32,
    pub EXMIN0: u32,
    pub CNVTIMR0: u32,
    pub CR11: u32,
    pub CR21: u32,
    pub ISR1: u32,
    pub ICR1: u32,
    pub JCHGR1: u32,
    pub FCR1: u32,
    pub JDATAR1: u32,
    pub RDATAR1: u32,
    pub AWHTR1: u32,
    pub AWLTR1: u32,
    pub AWSR1: u32,
    pub AWCFR1: u32,
    pub EXMAX1: u32,
    pub EXMIN1: u32,
    pub CNVTIMR1: u32,
    pub CR12: u32,
    pub CR22: u32,
    pub ISR2: u32,
    pub ICR2: u32,
    pub JCHGR2: u32,
    pub FCR2: u32,
    pub JDATAR2: u32,
    pub RDATAR2: u32,
    pub AWHTR2: u32,
    pub AWLTR2: u32,
    pub AWSR2: u32,
    pub AWCFR2: u32,
    pub EXMAX2: u32,
    pub EXMIN2: u32,
    pub CNVTIMR2: u32,
    pub CR13: u32,
    pub CR23: u32,
    pub ISR3: u32,
    pub ICR3: u32,
    pub JCHGR3: u32,
    pub FCR3: u32,
    pub JDATAR3: u32,
    pub RDATAR3: u32,
    pub AWHTR3: u32,
    pub AWLTR3: u32,
    pub AWSR3: u32,
    pub AWCFR3: u32,
    pub EXMAX3: u32,
    pub EXMIN3: u32,
    pub CNVTIMR3: u32,
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
