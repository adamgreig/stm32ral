#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DMA controller
//!
//! Used by: stm32h747cm4, stm32h757cm7

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// low interrupt status register
pub mod LISR {

    /// Stream x transfer complete interrupt flag (x = 3..0)
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

    /// Stream x half transfer interrupt flag (x=3..0)
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

    /// Stream x transfer error interrupt flag (x=3..0)
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

    /// Stream x direct mode error interrupt flag (x=3..0)
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

    /// Stream x FIFO error interrupt flag (x=3..0)
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

    /// Stream x transfer complete interrupt flag (x = 3..0)
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

    /// Stream x half transfer interrupt flag (x=3..0)
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

    /// Stream x transfer error interrupt flag (x=3..0)
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

    /// Stream x direct mode error interrupt flag (x=3..0)
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

    /// Stream x FIFO error interrupt flag (x=3..0)
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

    /// Stream x transfer complete interrupt flag (x = 3..0)
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

    /// Stream x half transfer interrupt flag (x=3..0)
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

    /// Stream x transfer error interrupt flag (x=3..0)
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

    /// Stream x direct mode error interrupt flag (x=3..0)
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

    /// Stream x FIFO error interrupt flag (x=3..0)
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

    /// Stream x transfer complete interrupt flag (x = 3..0)
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

    /// Stream x half transfer interrupt flag (x=3..0)
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

    /// Stream x transfer error interrupt flag (x=3..0)
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

    /// Stream x direct mode error interrupt flag (x=3..0)
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

    /// Stream x FIFO error interrupt flag (x=3..0)
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
}

/// high interrupt status register
pub mod HISR {

    /// Stream x transfer complete interrupt flag (x=7..4)
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

    /// Stream x half transfer interrupt flag (x=7..4)
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

    /// Stream x transfer error interrupt flag (x=7..4)
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

    /// Stream x direct mode error interrupt flag (x=7..4)
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

    /// Stream x FIFO error interrupt flag (x=7..4)
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

    /// Stream x transfer complete interrupt flag (x=7..4)
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

    /// Stream x half transfer interrupt flag (x=7..4)
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

    /// Stream x transfer error interrupt flag (x=7..4)
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

    /// Stream x direct mode error interrupt flag (x=7..4)
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

    /// Stream x FIFO error interrupt flag (x=7..4)
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

    /// Stream x transfer complete interrupt flag (x=7..4)
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

    /// Stream x half transfer interrupt flag (x=7..4)
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

    /// Stream x transfer error interrupt flag (x=7..4)
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

    /// Stream x direct mode error interrupt flag (x=7..4)
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

    /// Stream x FIFO error interrupt flag (x=7..4)
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

    /// Stream x transfer complete interrupt flag (x=7..4)
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

    /// Stream x half transfer interrupt flag (x=7..4)
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

    /// Stream x transfer error interrupt flag (x=7..4)
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

    /// Stream x direct mode error interrupt flag (x=7..4)
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

    /// Stream x FIFO error interrupt flag (x=7..4)
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
}

/// low interrupt flag clear register
pub mod LIFCR {

    /// Stream x clear transfer complete interrupt flag (x = 3..0)
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

    /// Stream x clear half transfer interrupt flag (x = 3..0)
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

    /// Stream x clear transfer error interrupt flag (x = 3..0)
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

    /// Stream x clear direct mode error interrupt flag (x = 3..0)
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

    /// Stream x clear FIFO error interrupt flag (x = 3..0)
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

    /// Stream x clear transfer complete interrupt flag (x = 3..0)
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

    /// Stream x clear half transfer interrupt flag (x = 3..0)
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

    /// Stream x clear transfer error interrupt flag (x = 3..0)
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

    /// Stream x clear direct mode error interrupt flag (x = 3..0)
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

    /// Stream x clear FIFO error interrupt flag (x = 3..0)
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

    /// Stream x clear transfer complete interrupt flag (x = 3..0)
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

    /// Stream x clear half transfer interrupt flag (x = 3..0)
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

    /// Stream x clear transfer error interrupt flag (x = 3..0)
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

    /// Stream x clear direct mode error interrupt flag (x = 3..0)
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

    /// Stream x clear FIFO error interrupt flag (x = 3..0)
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

    /// Stream x clear transfer complete interrupt flag (x = 3..0)
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

    /// Stream x clear half transfer interrupt flag (x = 3..0)
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

    /// Stream x clear transfer error interrupt flag (x = 3..0)
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

    /// Stream x clear direct mode error interrupt flag (x = 3..0)
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

    /// Stream x clear FIFO error interrupt flag (x = 3..0)
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
}

/// high interrupt flag clear register
pub mod HIFCR {

    /// Stream x clear transfer complete interrupt flag (x = 7..4)
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

    /// Stream x clear half transfer interrupt flag (x = 7..4)
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

    /// Stream x clear transfer error interrupt flag (x = 7..4)
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

    /// Stream x clear direct mode error interrupt flag (x = 7..4)
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

    /// Stream x clear FIFO error interrupt flag (x = 7..4)
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

    /// Stream x clear transfer complete interrupt flag (x = 7..4)
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

    /// Stream x clear half transfer interrupt flag (x = 7..4)
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

    /// Stream x clear transfer error interrupt flag (x = 7..4)
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

    /// Stream x clear direct mode error interrupt flag (x = 7..4)
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

    /// Stream x clear FIFO error interrupt flag (x = 7..4)
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

    /// Stream x clear transfer complete interrupt flag (x = 7..4)
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

    /// Stream x clear half transfer interrupt flag (x = 7..4)
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

    /// Stream x clear transfer error interrupt flag (x = 7..4)
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

    /// Stream x clear direct mode error interrupt flag (x = 7..4)
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

    /// Stream x clear FIFO error interrupt flag (x = 7..4)
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

    /// Stream x clear transfer complete interrupt flag (x = 7..4)
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

    /// Stream x clear half transfer interrupt flag (x = 7..4)
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

    /// Stream x clear transfer error interrupt flag (x = 7..4)
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

    /// Stream x clear direct mode error interrupt flag (x = 7..4)
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

    /// Stream x clear FIFO error interrupt flag (x = 7..4)
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
}

/// stream x configuration register
pub mod CR0 {

    /// Memory burst transfer configuration
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

    /// Peripheral burst transfer configuration
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

    /// Current target (only in double buffer mode)
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

    /// Double buffer mode
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

    /// Priority level
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

    /// Peripheral increment offset size
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

    /// Memory data size
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

    /// Peripheral data size
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

    /// Memory increment mode
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

    /// Peripheral increment mode
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

    /// Circular mode
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

    /// Data transfer direction
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

    /// Peripheral flow controller
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

    /// Transfer complete interrupt enable
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

    /// Half transfer interrupt enable
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

    /// Transfer error interrupt enable
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

    /// Direct mode error interrupt enable
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

    /// Stream enable / flag stream ready when read low
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
}

/// stream x number of data register
pub mod NDTR0 {

    /// Number of data items to transfer
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

/// stream x peripheral address register
pub mod PAR0 {

    /// Peripheral address
    pub mod PA {
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

/// stream x memory 0 address register
pub mod M0AR0 {

    /// Memory 0 address
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

/// stream x memory 1 address register
pub mod M1AR0 {

    /// Memory 1 address (used in case of Double buffer mode)
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

/// stream x FIFO control register
pub mod FCR0 {

    /// FIFO error interrupt enable
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

    /// FIFO status
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

    /// Direct mode disable
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

    /// FIFO threshold selection
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
}

/// stream x configuration register
pub mod CR1 {
    pub use super::CR0::CIRC;
    pub use super::CR0::CT;
    pub use super::CR0::DBM;
    pub use super::CR0::DIR;
    pub use super::CR0::DMEIE;
    pub use super::CR0::EN;
    pub use super::CR0::HTIE;
    pub use super::CR0::MBURST;
    pub use super::CR0::MINC;
    pub use super::CR0::MSIZE;
    pub use super::CR0::PBURST;
    pub use super::CR0::PFCTRL;
    pub use super::CR0::PINC;
    pub use super::CR0::PINCOS;
    pub use super::CR0::PL;
    pub use super::CR0::PSIZE;
    pub use super::CR0::TCIE;
    pub use super::CR0::TEIE;
}

/// stream x number of data register
pub mod NDTR1 {
    pub use super::NDTR0::NDT;
}

/// stream x peripheral address register
pub mod PAR1 {
    pub use super::PAR0::PA;
}

/// stream x memory 0 address register
pub mod M0AR1 {
    pub use super::M0AR0::M0A;
}

/// stream x memory 1 address register
pub mod M1AR1 {
    pub use super::M1AR0::M1A;
}

/// stream x FIFO control register
pub mod FCR1 {
    pub use super::FCR0::DMDIS;
    pub use super::FCR0::FEIE;
    pub use super::FCR0::FS;
    pub use super::FCR0::FTH;
}

/// stream x configuration register
pub mod CR2 {
    pub use super::CR0::CIRC;
    pub use super::CR0::CT;
    pub use super::CR0::DBM;
    pub use super::CR0::DIR;
    pub use super::CR0::DMEIE;
    pub use super::CR0::EN;
    pub use super::CR0::HTIE;
    pub use super::CR0::MBURST;
    pub use super::CR0::MINC;
    pub use super::CR0::MSIZE;
    pub use super::CR0::PBURST;
    pub use super::CR0::PFCTRL;
    pub use super::CR0::PINC;
    pub use super::CR0::PINCOS;
    pub use super::CR0::PL;
    pub use super::CR0::PSIZE;
    pub use super::CR0::TCIE;
    pub use super::CR0::TEIE;
}

/// stream x number of data register
pub mod NDTR2 {
    pub use super::NDTR0::NDT;
}

/// stream x peripheral address register
pub mod PAR2 {
    pub use super::PAR0::PA;
}

/// stream x memory 0 address register
pub mod M0AR2 {
    pub use super::M0AR0::M0A;
}

/// stream x memory 1 address register
pub mod M1AR2 {
    pub use super::M1AR0::M1A;
}

/// stream x FIFO control register
pub mod FCR2 {
    pub use super::FCR0::DMDIS;
    pub use super::FCR0::FEIE;
    pub use super::FCR0::FS;
    pub use super::FCR0::FTH;
}

/// stream x configuration register
pub mod CR3 {
    pub use super::CR0::CIRC;
    pub use super::CR0::CT;
    pub use super::CR0::DBM;
    pub use super::CR0::DIR;
    pub use super::CR0::DMEIE;
    pub use super::CR0::EN;
    pub use super::CR0::HTIE;
    pub use super::CR0::MBURST;
    pub use super::CR0::MINC;
    pub use super::CR0::MSIZE;
    pub use super::CR0::PBURST;
    pub use super::CR0::PFCTRL;
    pub use super::CR0::PINC;
    pub use super::CR0::PINCOS;
    pub use super::CR0::PL;
    pub use super::CR0::PSIZE;
    pub use super::CR0::TCIE;
    pub use super::CR0::TEIE;
}

/// stream x number of data register
pub mod NDTR3 {
    pub use super::NDTR0::NDT;
}

/// stream x peripheral address register
pub mod PAR3 {
    pub use super::PAR0::PA;
}

/// stream x memory 0 address register
pub mod M0AR3 {
    pub use super::M0AR0::M0A;
}

/// stream x memory 1 address register
pub mod M1AR3 {
    pub use super::M1AR0::M1A;
}

/// stream x FIFO control register
pub mod FCR3 {
    pub use super::FCR0::DMDIS;
    pub use super::FCR0::FEIE;
    pub use super::FCR0::FS;
    pub use super::FCR0::FTH;
}

/// stream x configuration register
pub mod CR4 {
    pub use super::CR0::CIRC;
    pub use super::CR0::CT;
    pub use super::CR0::DBM;
    pub use super::CR0::DIR;
    pub use super::CR0::DMEIE;
    pub use super::CR0::EN;
    pub use super::CR0::HTIE;
    pub use super::CR0::MBURST;
    pub use super::CR0::MINC;
    pub use super::CR0::MSIZE;
    pub use super::CR0::PBURST;
    pub use super::CR0::PFCTRL;
    pub use super::CR0::PINC;
    pub use super::CR0::PINCOS;
    pub use super::CR0::PL;
    pub use super::CR0::PSIZE;
    pub use super::CR0::TCIE;
    pub use super::CR0::TEIE;
}

/// stream x number of data register
pub mod NDTR4 {
    pub use super::NDTR0::NDT;
}

/// stream x peripheral address register
pub mod PAR4 {
    pub use super::PAR0::PA;
}

/// stream x memory 0 address register
pub mod M0AR4 {
    pub use super::M0AR0::M0A;
}

/// stream x memory 1 address register
pub mod M1AR4 {
    pub use super::M1AR0::M1A;
}

/// stream x FIFO control register
pub mod FCR4 {
    pub use super::FCR0::DMDIS;
    pub use super::FCR0::FEIE;
    pub use super::FCR0::FS;
    pub use super::FCR0::FTH;
}

/// stream x configuration register
pub mod CR5 {
    pub use super::CR0::CIRC;
    pub use super::CR0::CT;
    pub use super::CR0::DBM;
    pub use super::CR0::DIR;
    pub use super::CR0::DMEIE;
    pub use super::CR0::EN;
    pub use super::CR0::HTIE;
    pub use super::CR0::MBURST;
    pub use super::CR0::MINC;
    pub use super::CR0::MSIZE;
    pub use super::CR0::PBURST;
    pub use super::CR0::PFCTRL;
    pub use super::CR0::PINC;
    pub use super::CR0::PINCOS;
    pub use super::CR0::PL;
    pub use super::CR0::PSIZE;
    pub use super::CR0::TCIE;
    pub use super::CR0::TEIE;
}

/// stream x number of data register
pub mod NDTR5 {
    pub use super::NDTR0::NDT;
}

/// stream x peripheral address register
pub mod PAR5 {
    pub use super::PAR0::PA;
}

/// stream x memory 0 address register
pub mod M0AR5 {
    pub use super::M0AR0::M0A;
}

/// stream x memory 1 address register
pub mod M1AR5 {
    pub use super::M1AR0::M1A;
}

/// stream x FIFO control register
pub mod FCR5 {
    pub use super::FCR0::DMDIS;
    pub use super::FCR0::FEIE;
    pub use super::FCR0::FS;
    pub use super::FCR0::FTH;
}

/// stream x configuration register
pub mod CR6 {
    pub use super::CR0::CIRC;
    pub use super::CR0::CT;
    pub use super::CR0::DBM;
    pub use super::CR0::DIR;
    pub use super::CR0::DMEIE;
    pub use super::CR0::EN;
    pub use super::CR0::HTIE;
    pub use super::CR0::MBURST;
    pub use super::CR0::MINC;
    pub use super::CR0::MSIZE;
    pub use super::CR0::PBURST;
    pub use super::CR0::PFCTRL;
    pub use super::CR0::PINC;
    pub use super::CR0::PINCOS;
    pub use super::CR0::PL;
    pub use super::CR0::PSIZE;
    pub use super::CR0::TCIE;
    pub use super::CR0::TEIE;
}

/// stream x number of data register
pub mod NDTR6 {
    pub use super::NDTR0::NDT;
}

/// stream x peripheral address register
pub mod PAR6 {
    pub use super::PAR0::PA;
}

/// stream x memory 0 address register
pub mod M0AR6 {
    pub use super::M0AR0::M0A;
}

/// stream x memory 1 address register
pub mod M1AR6 {
    pub use super::M1AR0::M1A;
}

/// stream x FIFO control register
pub mod FCR6 {
    pub use super::FCR0::DMDIS;
    pub use super::FCR0::FEIE;
    pub use super::FCR0::FS;
    pub use super::FCR0::FTH;
}

/// stream x configuration register
pub mod CR7 {
    pub use super::CR0::CIRC;
    pub use super::CR0::CT;
    pub use super::CR0::DBM;
    pub use super::CR0::DIR;
    pub use super::CR0::DMEIE;
    pub use super::CR0::EN;
    pub use super::CR0::HTIE;
    pub use super::CR0::MBURST;
    pub use super::CR0::MINC;
    pub use super::CR0::MSIZE;
    pub use super::CR0::PBURST;
    pub use super::CR0::PFCTRL;
    pub use super::CR0::PINC;
    pub use super::CR0::PINCOS;
    pub use super::CR0::PL;
    pub use super::CR0::PSIZE;
    pub use super::CR0::TCIE;
    pub use super::CR0::TEIE;
}

/// stream x number of data register
pub mod NDTR7 {
    pub use super::NDTR0::NDT;
}

/// stream x peripheral address register
pub mod PAR7 {
    pub use super::PAR0::PA;
}

/// stream x memory 0 address register
pub mod M0AR7 {
    pub use super::M0AR0::M0A;
}

/// stream x memory 1 address register
pub mod M1AR7 {
    pub use super::M1AR0::M1A;
}

/// stream x FIFO control register
pub mod FCR7 {
    pub use super::FCR0::DMDIS;
    pub use super::FCR0::FEIE;
    pub use super::FCR0::FS;
    pub use super::FCR0::FTH;
}
pub struct RegisterBlock {
    /// low interrupt status register
    pub LISR: RORegister<u32>,

    /// high interrupt status register
    pub HISR: RORegister<u32>,

    /// low interrupt flag clear register
    pub LIFCR: WORegister<u32>,

    /// high interrupt flag clear register
    pub HIFCR: WORegister<u32>,

    /// stream x configuration register
    pub CR0: RWRegister<u32>,

    /// stream x number of data register
    pub NDTR0: RWRegister<u32>,

    /// stream x peripheral address register
    pub PAR0: RWRegister<u32>,

    /// stream x memory 0 address register
    pub M0AR0: RWRegister<u32>,

    /// stream x memory 1 address register
    pub M1AR0: RWRegister<u32>,

    /// stream x FIFO control register
    pub FCR0: RWRegister<u32>,

    /// stream x configuration register
    pub CR1: RWRegister<u32>,

    /// stream x number of data register
    pub NDTR1: RWRegister<u32>,

    /// stream x peripheral address register
    pub PAR1: RWRegister<u32>,

    /// stream x memory 0 address register
    pub M0AR1: RWRegister<u32>,

    /// stream x memory 1 address register
    pub M1AR1: RWRegister<u32>,

    /// stream x FIFO control register
    pub FCR1: RWRegister<u32>,

    /// stream x configuration register
    pub CR2: RWRegister<u32>,

    /// stream x number of data register
    pub NDTR2: RWRegister<u32>,

    /// stream x peripheral address register
    pub PAR2: RWRegister<u32>,

    /// stream x memory 0 address register
    pub M0AR2: RWRegister<u32>,

    /// stream x memory 1 address register
    pub M1AR2: RWRegister<u32>,

    /// stream x FIFO control register
    pub FCR2: RWRegister<u32>,

    /// stream x configuration register
    pub CR3: RWRegister<u32>,

    /// stream x number of data register
    pub NDTR3: RWRegister<u32>,

    /// stream x peripheral address register
    pub PAR3: RWRegister<u32>,

    /// stream x memory 0 address register
    pub M0AR3: RWRegister<u32>,

    /// stream x memory 1 address register
    pub M1AR3: RWRegister<u32>,

    /// stream x FIFO control register
    pub FCR3: RWRegister<u32>,

    /// stream x configuration register
    pub CR4: RWRegister<u32>,

    /// stream x number of data register
    pub NDTR4: RWRegister<u32>,

    /// stream x peripheral address register
    pub PAR4: RWRegister<u32>,

    /// stream x memory 0 address register
    pub M0AR4: RWRegister<u32>,

    /// stream x memory 1 address register
    pub M1AR4: RWRegister<u32>,

    /// stream x FIFO control register
    pub FCR4: RWRegister<u32>,

    /// stream x configuration register
    pub CR5: RWRegister<u32>,

    /// stream x number of data register
    pub NDTR5: RWRegister<u32>,

    /// stream x peripheral address register
    pub PAR5: RWRegister<u32>,

    /// stream x memory 0 address register
    pub M0AR5: RWRegister<u32>,

    /// stream x memory 1 address register
    pub M1AR5: RWRegister<u32>,

    /// stream x FIFO control register
    pub FCR5: RWRegister<u32>,

    /// stream x configuration register
    pub CR6: RWRegister<u32>,

    /// stream x number of data register
    pub NDTR6: RWRegister<u32>,

    /// stream x peripheral address register
    pub PAR6: RWRegister<u32>,

    /// stream x memory 0 address register
    pub M0AR6: RWRegister<u32>,

    /// stream x memory 1 address register
    pub M1AR6: RWRegister<u32>,

    /// stream x FIFO control register
    pub FCR6: RWRegister<u32>,

    /// stream x configuration register
    pub CR7: RWRegister<u32>,

    /// stream x number of data register
    pub NDTR7: RWRegister<u32>,

    /// stream x peripheral address register
    pub PAR7: RWRegister<u32>,

    /// stream x memory 0 address register
    pub M0AR7: RWRegister<u32>,

    /// stream x memory 1 address register
    pub M1AR7: RWRegister<u32>,

    /// stream x FIFO control register
    pub FCR7: RWRegister<u32>,
}
pub struct ResetValues {
    pub LISR: u32,
    pub HISR: u32,
    pub LIFCR: u32,
    pub HIFCR: u32,
    pub CR0: u32,
    pub NDTR0: u32,
    pub PAR0: u32,
    pub M0AR0: u32,
    pub M1AR0: u32,
    pub FCR0: u32,
    pub CR1: u32,
    pub NDTR1: u32,
    pub PAR1: u32,
    pub M0AR1: u32,
    pub M1AR1: u32,
    pub FCR1: u32,
    pub CR2: u32,
    pub NDTR2: u32,
    pub PAR2: u32,
    pub M0AR2: u32,
    pub M1AR2: u32,
    pub FCR2: u32,
    pub CR3: u32,
    pub NDTR3: u32,
    pub PAR3: u32,
    pub M0AR3: u32,
    pub M1AR3: u32,
    pub FCR3: u32,
    pub CR4: u32,
    pub NDTR4: u32,
    pub PAR4: u32,
    pub M0AR4: u32,
    pub M1AR4: u32,
    pub FCR4: u32,
    pub CR5: u32,
    pub NDTR5: u32,
    pub PAR5: u32,
    pub M0AR5: u32,
    pub M1AR5: u32,
    pub FCR5: u32,
    pub CR6: u32,
    pub NDTR6: u32,
    pub PAR6: u32,
    pub M0AR6: u32,
    pub M1AR6: u32,
    pub FCR6: u32,
    pub CR7: u32,
    pub NDTR7: u32,
    pub PAR7: u32,
    pub M0AR7: u32,
    pub M1AR7: u32,
    pub FCR7: u32,
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
