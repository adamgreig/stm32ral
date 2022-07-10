#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DMA1
//!
//! Used by: stm32mp153, stm32mp157

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// DMA low interrupt status register
pub mod DMA_LISR {

    /// FEIF0
    pub mod FEIF0 {
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

    /// DMEIF0
    pub mod DMEIF0 {
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

    /// TEIF0
    pub mod TEIF0 {
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

    /// HTIF0
    pub mod HTIF0 {
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

    /// TCIF0
    pub mod TCIF0 {
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

    /// FEIF1
    pub mod FEIF1 {
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

    /// DMEIF1
    pub mod DMEIF1 {
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

    /// TEIF1
    pub mod TEIF1 {
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

    /// HTIF1
    pub mod HTIF1 {
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

    /// TCIF1
    pub mod TCIF1 {
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

    /// FEIF2
    pub mod FEIF2 {
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

    /// DMEIF2
    pub mod DMEIF2 {
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

    /// TEIF2
    pub mod TEIF2 {
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

    /// HTIF2
    pub mod HTIF2 {
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

    /// TCIF2
    pub mod TCIF2 {
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

    /// FEIF3
    pub mod FEIF3 {
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

    /// DMEIF3
    pub mod DMEIF3 {
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

    /// TEIF3
    pub mod TEIF3 {
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

    /// HTIF3
    pub mod HTIF3 {
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

    /// TCIF3
    pub mod TCIF3 {
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

/// DMA high interrupt status register
pub mod DMA_HISR {

    /// FEIF4
    pub mod FEIF4 {
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

    /// DMEIF4
    pub mod DMEIF4 {
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

    /// TEIF4
    pub mod TEIF4 {
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

    /// HTIF4
    pub mod HTIF4 {
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

    /// TCIF4
    pub mod TCIF4 {
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

    /// FEIF5
    pub mod FEIF5 {
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

    /// DMEIF5
    pub mod DMEIF5 {
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

    /// TEIF5
    pub mod TEIF5 {
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

    /// HTIF5
    pub mod HTIF5 {
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

    /// TCIF5
    pub mod TCIF5 {
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

    /// FEIF6
    pub mod FEIF6 {
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

    /// DMEIF6
    pub mod DMEIF6 {
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

    /// TEIF6
    pub mod TEIF6 {
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

    /// HTIF6
    pub mod HTIF6 {
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

    /// TCIF6
    pub mod TCIF6 {
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

    /// FEIF7
    pub mod FEIF7 {
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

    /// DMEIF7
    pub mod DMEIF7 {
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

    /// TEIF7
    pub mod TEIF7 {
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

    /// HTIF7
    pub mod HTIF7 {
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

    /// TCIF7
    pub mod TCIF7 {
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

/// DMA low interrupt flag clear register
pub mod DMA_LIFCR {

    /// CFEIF0
    pub mod CFEIF0 {
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

    /// CDMEIF0
    pub mod CDMEIF0 {
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

    /// CTEIF0
    pub mod CTEIF0 {
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

    /// CHTIF0
    pub mod CHTIF0 {
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

    /// CTCIF0
    pub mod CTCIF0 {
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

    /// CFEIF1
    pub mod CFEIF1 {
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

    /// CDMEIF1
    pub mod CDMEIF1 {
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

    /// CTEIF1
    pub mod CTEIF1 {
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

    /// CHTIF1
    pub mod CHTIF1 {
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

    /// CTCIF1
    pub mod CTCIF1 {
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

    /// CFEIF2
    pub mod CFEIF2 {
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

    /// CDMEIF2
    pub mod CDMEIF2 {
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

    /// CTEIF2
    pub mod CTEIF2 {
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

    /// CHTIF2
    pub mod CHTIF2 {
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

    /// CTCIF2
    pub mod CTCIF2 {
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

    /// CFEIF3
    pub mod CFEIF3 {
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

    /// CDMEIF3
    pub mod CDMEIF3 {
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

    /// CTEIF3
    pub mod CTEIF3 {
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

    /// CHTIF3
    pub mod CHTIF3 {
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

    /// CTCIF3
    pub mod CTCIF3 {
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

/// DMA high interrupt flag clear register
pub mod DMA_HIFCR {

    /// CFEIF4
    pub mod CFEIF4 {
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

    /// CDMEIF4
    pub mod CDMEIF4 {
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

    /// CTEIF4
    pub mod CTEIF4 {
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

    /// CHTIF4
    pub mod CHTIF4 {
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

    /// CTCIF4
    pub mod CTCIF4 {
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

    /// CFEIF5
    pub mod CFEIF5 {
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

    /// CDMEIF5
    pub mod CDMEIF5 {
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

    /// CTEIF5
    pub mod CTEIF5 {
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

    /// CHTIF5
    pub mod CHTIF5 {
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

    /// CTCIF5
    pub mod CTCIF5 {
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

    /// CFEIF6
    pub mod CFEIF6 {
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

    /// CDMEIF6
    pub mod CDMEIF6 {
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

    /// CTEIF6
    pub mod CTEIF6 {
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

    /// CHTIF6
    pub mod CHTIF6 {
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

    /// CTCIF6
    pub mod CTCIF6 {
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

    /// CFEIF7
    pub mod CFEIF7 {
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

    /// CDMEIF7
    pub mod CDMEIF7 {
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

    /// CTEIF7
    pub mod CTEIF7 {
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

    /// CHTIF7
    pub mod CHTIF7 {
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

    /// CTCIF7
    pub mod CTCIF7 {
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

/// This register is used to configure the concerned stream.
pub mod DMA_S0CR {

    /// EN
    pub mod EN {
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

    /// DMEIE
    pub mod DMEIE {
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

    /// TEIE
    pub mod TEIE {
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

    /// HTIE
    pub mod HTIE {
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

    /// TCIE
    pub mod TCIE {
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

    /// PFCTRL
    pub mod PFCTRL {
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

    /// DIR
    pub mod DIR {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (2 bits: 0b11 << 6)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CIRC
    pub mod CIRC {
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

    /// PINC
    pub mod PINC {
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

    /// MINC
    pub mod MINC {
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

    /// PSIZE
    pub mod PSIZE {
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

    /// MSIZE
    pub mod MSIZE {
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

    /// PINCOS
    pub mod PINCOS {
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

    /// PL
    pub mod PL {
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

    /// DBM
    pub mod DBM {
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

    /// CT
    pub mod CT {
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

    /// PBURST
    pub mod PBURST {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (2 bits: 0b11 << 21)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// MBURST
    pub mod MBURST {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (2 bits: 0b11 << 23)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DMA stream 0 number of data register
pub mod DMA_S0NDTR {

    /// NDT
    pub mod NDT {
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

/// DMA stream 0 peripheral address register
pub mod DMA_S0PAR {

    /// PAR
    pub mod PAR {
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

/// DMA stream 0 memory 0 address register
pub mod DMA_S0M0AR {

    /// M0A
    pub mod M0A {
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

/// DMA stream 0 memory 1 address register
pub mod DMA_S0M1AR {

    /// M1A
    pub mod M1A {
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

/// DMA stream 0 FIFO control register
pub mod DMA_S0FCR {

    /// FTH
    pub mod FTH {
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

    /// DMDIS
    pub mod DMDIS {
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

    /// FS
    pub mod FS {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (3 bits: 0b111 << 3)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// FEIE
    pub mod FEIE {
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
}

/// This register is used to configure the concerned stream.
pub mod DMA_S1CR {
    pub use super::DMA_S0CR::CIRC;
    pub use super::DMA_S0CR::CT;
    pub use super::DMA_S0CR::DBM;
    pub use super::DMA_S0CR::DIR;
    pub use super::DMA_S0CR::DMEIE;
    pub use super::DMA_S0CR::EN;
    pub use super::DMA_S0CR::HTIE;
    pub use super::DMA_S0CR::MBURST;
    pub use super::DMA_S0CR::MINC;
    pub use super::DMA_S0CR::MSIZE;
    pub use super::DMA_S0CR::PBURST;
    pub use super::DMA_S0CR::PFCTRL;
    pub use super::DMA_S0CR::PINC;
    pub use super::DMA_S0CR::PINCOS;
    pub use super::DMA_S0CR::PL;
    pub use super::DMA_S0CR::PSIZE;
    pub use super::DMA_S0CR::TCIE;
    pub use super::DMA_S0CR::TEIE;
}

/// DMA stream 1 number of data register
pub mod DMA_S1NDTR {
    pub use super::DMA_S0NDTR::NDT;
}

/// DMA stream 1 peripheral address register
pub mod DMA_S1PAR {
    pub use super::DMA_S0PAR::PAR;
}

/// DMA stream 1 memory 0 address register
pub mod DMA_S1M0AR {
    pub use super::DMA_S0M0AR::M0A;
}

/// DMA stream 1 memory 1 address register
pub mod DMA_S1M1AR {
    pub use super::DMA_S0M1AR::M1A;
}

/// DMA stream 1 FIFO control register
pub mod DMA_S1FCR {
    pub use super::DMA_S0FCR::DMDIS;
    pub use super::DMA_S0FCR::FEIE;
    pub use super::DMA_S0FCR::FS;
    pub use super::DMA_S0FCR::FTH;
}

/// This register is used to configure the concerned stream.
pub mod DMA_S2CR {
    pub use super::DMA_S0CR::CIRC;
    pub use super::DMA_S0CR::CT;
    pub use super::DMA_S0CR::DBM;
    pub use super::DMA_S0CR::DIR;
    pub use super::DMA_S0CR::DMEIE;
    pub use super::DMA_S0CR::EN;
    pub use super::DMA_S0CR::HTIE;
    pub use super::DMA_S0CR::MBURST;
    pub use super::DMA_S0CR::MINC;
    pub use super::DMA_S0CR::MSIZE;
    pub use super::DMA_S0CR::PBURST;
    pub use super::DMA_S0CR::PFCTRL;
    pub use super::DMA_S0CR::PINC;
    pub use super::DMA_S0CR::PINCOS;
    pub use super::DMA_S0CR::PL;
    pub use super::DMA_S0CR::PSIZE;
    pub use super::DMA_S0CR::TCIE;
    pub use super::DMA_S0CR::TEIE;
}

/// DMA stream 2 number of data register
pub mod DMA_S2NDTR {
    pub use super::DMA_S0NDTR::NDT;
}

/// DMA stream 2 peripheral address register
pub mod DMA_S2PAR {
    pub use super::DMA_S0PAR::PAR;
}

/// DMA stream 2 memory 0 address register
pub mod DMA_S2M0AR {
    pub use super::DMA_S0M0AR::M0A;
}

/// DMA stream 2 memory 1 address register
pub mod DMA_S2M1AR {
    pub use super::DMA_S0M1AR::M1A;
}

/// DMA stream 2 FIFO control register
pub mod DMA_S2FCR {
    pub use super::DMA_S0FCR::DMDIS;
    pub use super::DMA_S0FCR::FEIE;
    pub use super::DMA_S0FCR::FS;
    pub use super::DMA_S0FCR::FTH;
}

/// This register is used to configure the concerned stream.
pub mod DMA_S3CR {
    pub use super::DMA_S0CR::CIRC;
    pub use super::DMA_S0CR::CT;
    pub use super::DMA_S0CR::DBM;
    pub use super::DMA_S0CR::DIR;
    pub use super::DMA_S0CR::DMEIE;
    pub use super::DMA_S0CR::EN;
    pub use super::DMA_S0CR::HTIE;
    pub use super::DMA_S0CR::MBURST;
    pub use super::DMA_S0CR::MINC;
    pub use super::DMA_S0CR::MSIZE;
    pub use super::DMA_S0CR::PBURST;
    pub use super::DMA_S0CR::PFCTRL;
    pub use super::DMA_S0CR::PINC;
    pub use super::DMA_S0CR::PINCOS;
    pub use super::DMA_S0CR::PL;
    pub use super::DMA_S0CR::PSIZE;
    pub use super::DMA_S0CR::TCIE;
    pub use super::DMA_S0CR::TEIE;
}

/// DMA stream 3 number of data register
pub mod DMA_S3NDTR {
    pub use super::DMA_S0NDTR::NDT;
}

/// DMA stream 3 peripheral address register
pub mod DMA_S3PAR {
    pub use super::DMA_S0PAR::PAR;
}

/// DMA stream 3 memory 0 address register
pub mod DMA_S3M0AR {
    pub use super::DMA_S0M0AR::M0A;
}

/// DMA stream 3 memory 1 address register
pub mod DMA_S3M1AR {
    pub use super::DMA_S0M1AR::M1A;
}

/// DMA stream 3 FIFO control register
pub mod DMA_S3FCR {
    pub use super::DMA_S0FCR::DMDIS;
    pub use super::DMA_S0FCR::FEIE;
    pub use super::DMA_S0FCR::FS;
    pub use super::DMA_S0FCR::FTH;
}

/// This register is used to configure the concerned stream.
pub mod DMA_S4CR {
    pub use super::DMA_S0CR::CIRC;
    pub use super::DMA_S0CR::CT;
    pub use super::DMA_S0CR::DBM;
    pub use super::DMA_S0CR::DIR;
    pub use super::DMA_S0CR::DMEIE;
    pub use super::DMA_S0CR::EN;
    pub use super::DMA_S0CR::HTIE;
    pub use super::DMA_S0CR::MBURST;
    pub use super::DMA_S0CR::MINC;
    pub use super::DMA_S0CR::MSIZE;
    pub use super::DMA_S0CR::PBURST;
    pub use super::DMA_S0CR::PFCTRL;
    pub use super::DMA_S0CR::PINC;
    pub use super::DMA_S0CR::PINCOS;
    pub use super::DMA_S0CR::PL;
    pub use super::DMA_S0CR::PSIZE;
    pub use super::DMA_S0CR::TCIE;
    pub use super::DMA_S0CR::TEIE;
}

/// DMA stream 4 number of data register
pub mod DMA_S4NDTR {
    pub use super::DMA_S0NDTR::NDT;
}

/// DMA stream 4 peripheral address register
pub mod DMA_S4PAR {
    pub use super::DMA_S0PAR::PAR;
}

/// DMA stream 4 memory 0 address register
pub mod DMA_S4M0AR {
    pub use super::DMA_S0M0AR::M0A;
}

/// DMA stream 4 memory 1 address register
pub mod DMA_S4M1AR {
    pub use super::DMA_S0M1AR::M1A;
}

/// DMA stream 4 FIFO control register
pub mod DMA_S4FCR {
    pub use super::DMA_S0FCR::DMDIS;
    pub use super::DMA_S0FCR::FEIE;
    pub use super::DMA_S0FCR::FS;
    pub use super::DMA_S0FCR::FTH;
}

/// This register is used to configure the concerned stream.
pub mod DMA_S5CR {
    pub use super::DMA_S0CR::CIRC;
    pub use super::DMA_S0CR::CT;
    pub use super::DMA_S0CR::DBM;
    pub use super::DMA_S0CR::DIR;
    pub use super::DMA_S0CR::DMEIE;
    pub use super::DMA_S0CR::EN;
    pub use super::DMA_S0CR::HTIE;
    pub use super::DMA_S0CR::MBURST;
    pub use super::DMA_S0CR::MINC;
    pub use super::DMA_S0CR::MSIZE;
    pub use super::DMA_S0CR::PBURST;
    pub use super::DMA_S0CR::PFCTRL;
    pub use super::DMA_S0CR::PINC;
    pub use super::DMA_S0CR::PINCOS;
    pub use super::DMA_S0CR::PL;
    pub use super::DMA_S0CR::PSIZE;
    pub use super::DMA_S0CR::TCIE;
    pub use super::DMA_S0CR::TEIE;
}

/// DMA stream 5 number of data register
pub mod DMA_S5NDTR {
    pub use super::DMA_S0NDTR::NDT;
}

/// DMA stream 5 peripheral address register
pub mod DMA_S5PAR {
    pub use super::DMA_S0PAR::PAR;
}

/// DMA stream 5 memory 0 address register
pub mod DMA_S5M0AR {
    pub use super::DMA_S0M0AR::M0A;
}

/// DMA stream 5 memory 1 address register
pub mod DMA_S5M1AR {
    pub use super::DMA_S0M1AR::M1A;
}

/// DMA stream 5 FIFO control register
pub mod DMA_S5FCR {
    pub use super::DMA_S0FCR::DMDIS;
    pub use super::DMA_S0FCR::FEIE;
    pub use super::DMA_S0FCR::FS;
    pub use super::DMA_S0FCR::FTH;
}

/// This register is used to configure the concerned stream.
pub mod DMA_S6CR {
    pub use super::DMA_S0CR::CIRC;
    pub use super::DMA_S0CR::CT;
    pub use super::DMA_S0CR::DBM;
    pub use super::DMA_S0CR::DIR;
    pub use super::DMA_S0CR::DMEIE;
    pub use super::DMA_S0CR::EN;
    pub use super::DMA_S0CR::HTIE;
    pub use super::DMA_S0CR::MBURST;
    pub use super::DMA_S0CR::MINC;
    pub use super::DMA_S0CR::MSIZE;
    pub use super::DMA_S0CR::PBURST;
    pub use super::DMA_S0CR::PFCTRL;
    pub use super::DMA_S0CR::PINC;
    pub use super::DMA_S0CR::PINCOS;
    pub use super::DMA_S0CR::PL;
    pub use super::DMA_S0CR::PSIZE;
    pub use super::DMA_S0CR::TCIE;
    pub use super::DMA_S0CR::TEIE;
}

/// DMA stream 6 number of data register
pub mod DMA_S6NDTR {
    pub use super::DMA_S0NDTR::NDT;
}

/// DMA stream 6 peripheral address register
pub mod DMA_S6PAR {
    pub use super::DMA_S0PAR::PAR;
}

/// DMA stream 6 memory 0 address register
pub mod DMA_S6M0AR {
    pub use super::DMA_S0M0AR::M0A;
}

/// DMA stream 6 memory 1 address register
pub mod DMA_S6M1AR {
    pub use super::DMA_S0M1AR::M1A;
}

/// DMA stream 6 FIFO control register
pub mod DMA_S6FCR {
    pub use super::DMA_S0FCR::DMDIS;
    pub use super::DMA_S0FCR::FEIE;
    pub use super::DMA_S0FCR::FS;
    pub use super::DMA_S0FCR::FTH;
}

/// This register is used to configure the concerned stream.
pub mod DMA_S7CR {
    pub use super::DMA_S0CR::CIRC;
    pub use super::DMA_S0CR::CT;
    pub use super::DMA_S0CR::DBM;
    pub use super::DMA_S0CR::DIR;
    pub use super::DMA_S0CR::DMEIE;
    pub use super::DMA_S0CR::EN;
    pub use super::DMA_S0CR::HTIE;
    pub use super::DMA_S0CR::MBURST;
    pub use super::DMA_S0CR::MINC;
    pub use super::DMA_S0CR::MSIZE;
    pub use super::DMA_S0CR::PBURST;
    pub use super::DMA_S0CR::PFCTRL;
    pub use super::DMA_S0CR::PINC;
    pub use super::DMA_S0CR::PINCOS;
    pub use super::DMA_S0CR::PL;
    pub use super::DMA_S0CR::PSIZE;
    pub use super::DMA_S0CR::TCIE;
    pub use super::DMA_S0CR::TEIE;
}

/// DMA stream 7 number of data register
pub mod DMA_S7NDTR {
    pub use super::DMA_S0NDTR::NDT;
}

/// DMA stream 7 peripheral address register
pub mod DMA_S7PAR {
    pub use super::DMA_S0PAR::PAR;
}

/// DMA stream 7 memory 0 address register
pub mod DMA_S7M0AR {
    pub use super::DMA_S0M0AR::M0A;
}

/// DMA stream 7 memory 1 address register
pub mod DMA_S7M1AR {
    pub use super::DMA_S0M1AR::M1A;
}

/// DMA stream 7 FIFO control register
pub mod DMA_S7FCR {
    pub use super::DMA_S0FCR::DMDIS;
    pub use super::DMA_S0FCR::FEIE;
    pub use super::DMA_S0FCR::FS;
    pub use super::DMA_S0FCR::FTH;
}

/// DMA hardware configuration 2register
pub mod DMA_HWCFGR2 {

    /// FIFO_SIZE
    pub mod FIFO_SIZE {
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

    /// WRITE_BUFFERABLE
    pub mod WRITE_BUFFERABLE {
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

    /// CHSEL_WIDTH
    pub mod CHSEL_WIDTH {
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
}

/// DMA hardware configuration 1 register
pub mod DMA_HWCFGR1 {

    /// DMA_DEF0
    pub mod DMA_DEF0 {
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

    /// DMA_DEF1
    pub mod DMA_DEF1 {
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

    /// DMA_DEF2
    pub mod DMA_DEF2 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DMA_DEF3
    pub mod DMA_DEF3 {
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

    /// DMA_DEF4
    pub mod DMA_DEF4 {
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

    /// DMA_DEF5
    pub mod DMA_DEF5 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (2 bits: 0b11 << 20)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DMA_DEF6
    pub mod DMA_DEF6 {
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

    /// DMA_DEF7
    pub mod DMA_DEF7 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (2 bits: 0b11 << 28)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// This register identifies the version of the IP.
pub mod DMA_VERR {

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

/// DMA IP identification register
pub mod DMA_IPDR {

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

/// DMA size identification register
pub mod DMA_SIDR {

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
    /// DMA low interrupt status register
    pub DMA_LISR: RORegister<u32>,

    /// DMA high interrupt status register
    pub DMA_HISR: RORegister<u32>,

    /// DMA low interrupt flag clear register
    pub DMA_LIFCR: WORegister<u32>,

    /// DMA high interrupt flag clear register
    pub DMA_HIFCR: WORegister<u32>,

    /// This register is used to configure the concerned stream.
    pub DMA_S0CR: RWRegister<u32>,

    /// DMA stream 0 number of data register
    pub DMA_S0NDTR: RWRegister<u32>,

    /// DMA stream 0 peripheral address register
    pub DMA_S0PAR: RWRegister<u32>,

    /// DMA stream 0 memory 0 address register
    pub DMA_S0M0AR: RWRegister<u32>,

    /// DMA stream 0 memory 1 address register
    pub DMA_S0M1AR: RWRegister<u32>,

    /// DMA stream 0 FIFO control register
    pub DMA_S0FCR: RWRegister<u32>,

    /// This register is used to configure the concerned stream.
    pub DMA_S1CR: RWRegister<u32>,

    /// DMA stream 1 number of data register
    pub DMA_S1NDTR: RWRegister<u32>,

    /// DMA stream 1 peripheral address register
    pub DMA_S1PAR: RWRegister<u32>,

    /// DMA stream 1 memory 0 address register
    pub DMA_S1M0AR: RWRegister<u32>,

    /// DMA stream 1 memory 1 address register
    pub DMA_S1M1AR: RWRegister<u32>,

    /// DMA stream 1 FIFO control register
    pub DMA_S1FCR: RWRegister<u32>,

    /// This register is used to configure the concerned stream.
    pub DMA_S2CR: RWRegister<u32>,

    /// DMA stream 2 number of data register
    pub DMA_S2NDTR: RWRegister<u32>,

    /// DMA stream 2 peripheral address register
    pub DMA_S2PAR: RWRegister<u32>,

    /// DMA stream 2 memory 0 address register
    pub DMA_S2M0AR: RWRegister<u32>,

    /// DMA stream 2 memory 1 address register
    pub DMA_S2M1AR: RWRegister<u32>,

    /// DMA stream 2 FIFO control register
    pub DMA_S2FCR: RWRegister<u32>,

    /// This register is used to configure the concerned stream.
    pub DMA_S3CR: RWRegister<u32>,

    /// DMA stream 3 number of data register
    pub DMA_S3NDTR: RWRegister<u32>,

    /// DMA stream 3 peripheral address register
    pub DMA_S3PAR: RWRegister<u32>,

    /// DMA stream 3 memory 0 address register
    pub DMA_S3M0AR: RWRegister<u32>,

    /// DMA stream 3 memory 1 address register
    pub DMA_S3M1AR: RWRegister<u32>,

    /// DMA stream 3 FIFO control register
    pub DMA_S3FCR: RWRegister<u32>,

    /// This register is used to configure the concerned stream.
    pub DMA_S4CR: RWRegister<u32>,

    /// DMA stream 4 number of data register
    pub DMA_S4NDTR: RWRegister<u32>,

    /// DMA stream 4 peripheral address register
    pub DMA_S4PAR: RWRegister<u32>,

    /// DMA stream 4 memory 0 address register
    pub DMA_S4M0AR: RWRegister<u32>,

    /// DMA stream 4 memory 1 address register
    pub DMA_S4M1AR: RWRegister<u32>,

    /// DMA stream 4 FIFO control register
    pub DMA_S4FCR: RWRegister<u32>,

    /// This register is used to configure the concerned stream.
    pub DMA_S5CR: RWRegister<u32>,

    /// DMA stream 5 number of data register
    pub DMA_S5NDTR: RWRegister<u32>,

    /// DMA stream 5 peripheral address register
    pub DMA_S5PAR: RWRegister<u32>,

    /// DMA stream 5 memory 0 address register
    pub DMA_S5M0AR: RWRegister<u32>,

    /// DMA stream 5 memory 1 address register
    pub DMA_S5M1AR: RWRegister<u32>,

    /// DMA stream 5 FIFO control register
    pub DMA_S5FCR: RWRegister<u32>,

    /// This register is used to configure the concerned stream.
    pub DMA_S6CR: RWRegister<u32>,

    /// DMA stream 6 number of data register
    pub DMA_S6NDTR: RWRegister<u32>,

    /// DMA stream 6 peripheral address register
    pub DMA_S6PAR: RWRegister<u32>,

    /// DMA stream 6 memory 0 address register
    pub DMA_S6M0AR: RWRegister<u32>,

    /// DMA stream 6 memory 1 address register
    pub DMA_S6M1AR: RWRegister<u32>,

    /// DMA stream 6 FIFO control register
    pub DMA_S6FCR: RWRegister<u32>,

    /// This register is used to configure the concerned stream.
    pub DMA_S7CR: RWRegister<u32>,

    /// DMA stream 7 number of data register
    pub DMA_S7NDTR: RWRegister<u32>,

    /// DMA stream 7 peripheral address register
    pub DMA_S7PAR: RWRegister<u32>,

    /// DMA stream 7 memory 0 address register
    pub DMA_S7M0AR: RWRegister<u32>,

    /// DMA stream 7 memory 1 address register
    pub DMA_S7M1AR: RWRegister<u32>,

    /// DMA stream 7 FIFO control register
    pub DMA_S7FCR: RWRegister<u32>,

    _reserved1: [u8; 796],

    /// DMA hardware configuration 2register
    pub DMA_HWCFGR2: RORegister<u32>,

    /// DMA hardware configuration 1 register
    pub DMA_HWCFGR1: RORegister<u32>,

    /// This register identifies the version of the IP.
    pub DMA_VERR: RORegister<u32>,

    /// DMA IP identification register
    pub DMA_IPDR: RORegister<u32>,

    /// DMA size identification register
    pub DMA_SIDR: RORegister<u32>,
}
pub struct ResetValues {
    pub DMA_LISR: u32,
    pub DMA_HISR: u32,
    pub DMA_LIFCR: u32,
    pub DMA_HIFCR: u32,
    pub DMA_S0CR: u32,
    pub DMA_S0NDTR: u32,
    pub DMA_S0PAR: u32,
    pub DMA_S0M0AR: u32,
    pub DMA_S0M1AR: u32,
    pub DMA_S0FCR: u32,
    pub DMA_S1CR: u32,
    pub DMA_S1NDTR: u32,
    pub DMA_S1PAR: u32,
    pub DMA_S1M0AR: u32,
    pub DMA_S1M1AR: u32,
    pub DMA_S1FCR: u32,
    pub DMA_S2CR: u32,
    pub DMA_S2NDTR: u32,
    pub DMA_S2PAR: u32,
    pub DMA_S2M0AR: u32,
    pub DMA_S2M1AR: u32,
    pub DMA_S2FCR: u32,
    pub DMA_S3CR: u32,
    pub DMA_S3NDTR: u32,
    pub DMA_S3PAR: u32,
    pub DMA_S3M0AR: u32,
    pub DMA_S3M1AR: u32,
    pub DMA_S3FCR: u32,
    pub DMA_S4CR: u32,
    pub DMA_S4NDTR: u32,
    pub DMA_S4PAR: u32,
    pub DMA_S4M0AR: u32,
    pub DMA_S4M1AR: u32,
    pub DMA_S4FCR: u32,
    pub DMA_S5CR: u32,
    pub DMA_S5NDTR: u32,
    pub DMA_S5PAR: u32,
    pub DMA_S5M0AR: u32,
    pub DMA_S5M1AR: u32,
    pub DMA_S5FCR: u32,
    pub DMA_S6CR: u32,
    pub DMA_S6NDTR: u32,
    pub DMA_S6PAR: u32,
    pub DMA_S6M0AR: u32,
    pub DMA_S6M1AR: u32,
    pub DMA_S6FCR: u32,
    pub DMA_S7CR: u32,
    pub DMA_S7NDTR: u32,
    pub DMA_S7PAR: u32,
    pub DMA_S7M0AR: u32,
    pub DMA_S7M1AR: u32,
    pub DMA_S7FCR: u32,
    pub DMA_HWCFGR2: u32,
    pub DMA_HWCFGR1: u32,
    pub DMA_VERR: u32,
    pub DMA_IPDR: u32,
    pub DMA_SIDR: u32,
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
