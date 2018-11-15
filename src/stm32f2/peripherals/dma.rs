#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DMA controller
//!
//! Used by: stm32f215, stm32f217

#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;
use {RORegister, RWRegister, UnsafeRWRegister, WORegister};

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
pub mod S0CR {

    /// Channel selection
    pub mod CHSEL {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (3 bits: 0b111 << 25)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

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
pub mod S0NDTR {

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
pub mod S0PAR {

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
pub mod S0M0AR {

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
pub mod S0M1AR {

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
pub mod S0FCR {

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
pub mod S1CR {
    pub use super::S0CR::CHSEL;
    pub use super::S0CR::CIRC;
    pub use super::S0CR::CT;
    pub use super::S0CR::DBM;
    pub use super::S0CR::DIR;
    pub use super::S0CR::DMEIE;
    pub use super::S0CR::EN;
    pub use super::S0CR::HTIE;
    pub use super::S0CR::MBURST;
    pub use super::S0CR::MINC;
    pub use super::S0CR::MSIZE;
    pub use super::S0CR::PBURST;
    pub use super::S0CR::PFCTRL;
    pub use super::S0CR::PINC;
    pub use super::S0CR::PINCOS;
    pub use super::S0CR::PL;
    pub use super::S0CR::PSIZE;
    pub use super::S0CR::TCIE;
    pub use super::S0CR::TEIE;
}

/// stream x number of data register
pub mod S1NDTR {
    pub use super::S0NDTR::NDT;
}

/// stream x peripheral address register
pub mod S1PAR {
    pub use super::S0PAR::PA;
}

/// stream x memory 0 address register
pub mod S1M0AR {
    pub use super::S0M0AR::M0A;
}

/// stream x memory 1 address register
pub mod S1M1AR {
    pub use super::S0M1AR::M1A;
}

/// stream x FIFO control register
pub mod S1FCR {
    pub use super::S0FCR::DMDIS;
    pub use super::S0FCR::FEIE;
    pub use super::S0FCR::FS;
    pub use super::S0FCR::FTH;
}

/// stream x configuration register
pub mod S2CR {
    pub use super::S0CR::CHSEL;
    pub use super::S0CR::CIRC;
    pub use super::S0CR::CT;
    pub use super::S0CR::DBM;
    pub use super::S0CR::DIR;
    pub use super::S0CR::DMEIE;
    pub use super::S0CR::EN;
    pub use super::S0CR::HTIE;
    pub use super::S0CR::MBURST;
    pub use super::S0CR::MINC;
    pub use super::S0CR::MSIZE;
    pub use super::S0CR::PBURST;
    pub use super::S0CR::PFCTRL;
    pub use super::S0CR::PINC;
    pub use super::S0CR::PINCOS;
    pub use super::S0CR::PL;
    pub use super::S0CR::PSIZE;
    pub use super::S0CR::TCIE;
    pub use super::S0CR::TEIE;
}

/// stream x number of data register
pub mod S2NDTR {
    pub use super::S0NDTR::NDT;
}

/// stream x peripheral address register
pub mod S2PAR {
    pub use super::S0PAR::PA;
}

/// stream x memory 0 address register
pub mod S2M0AR {
    pub use super::S0M0AR::M0A;
}

/// stream x memory 1 address register
pub mod S2M1AR {
    pub use super::S0M1AR::M1A;
}

/// stream x FIFO control register
pub mod S2FCR {
    pub use super::S0FCR::DMDIS;
    pub use super::S0FCR::FEIE;
    pub use super::S0FCR::FS;
    pub use super::S0FCR::FTH;
}

/// stream x configuration register
pub mod S3CR {
    pub use super::S0CR::CHSEL;
    pub use super::S0CR::CIRC;
    pub use super::S0CR::CT;
    pub use super::S0CR::DBM;
    pub use super::S0CR::DIR;
    pub use super::S0CR::DMEIE;
    pub use super::S0CR::EN;
    pub use super::S0CR::HTIE;
    pub use super::S0CR::MBURST;
    pub use super::S0CR::MINC;
    pub use super::S0CR::MSIZE;
    pub use super::S0CR::PBURST;
    pub use super::S0CR::PFCTRL;
    pub use super::S0CR::PINC;
    pub use super::S0CR::PINCOS;
    pub use super::S0CR::PL;
    pub use super::S0CR::PSIZE;
    pub use super::S0CR::TCIE;
    pub use super::S0CR::TEIE;
}

/// stream x number of data register
pub mod S3NDTR {
    pub use super::S0NDTR::NDT;
}

/// stream x peripheral address register
pub mod S3PAR {
    pub use super::S0PAR::PA;
}

/// stream x memory 0 address register
pub mod S3M0AR {
    pub use super::S0M0AR::M0A;
}

/// stream x memory 1 address register
pub mod S3M1AR {
    pub use super::S0M1AR::M1A;
}

/// stream x FIFO control register
pub mod S3FCR {
    pub use super::S0FCR::DMDIS;
    pub use super::S0FCR::FEIE;
    pub use super::S0FCR::FS;
    pub use super::S0FCR::FTH;
}

/// stream x configuration register
pub mod S4CR {
    pub use super::S0CR::CHSEL;
    pub use super::S0CR::CIRC;
    pub use super::S0CR::CT;
    pub use super::S0CR::DBM;
    pub use super::S0CR::DIR;
    pub use super::S0CR::DMEIE;
    pub use super::S0CR::EN;
    pub use super::S0CR::HTIE;
    pub use super::S0CR::MBURST;
    pub use super::S0CR::MINC;
    pub use super::S0CR::MSIZE;
    pub use super::S0CR::PBURST;
    pub use super::S0CR::PFCTRL;
    pub use super::S0CR::PINC;
    pub use super::S0CR::PINCOS;
    pub use super::S0CR::PL;
    pub use super::S0CR::PSIZE;
    pub use super::S0CR::TCIE;
    pub use super::S0CR::TEIE;
}

/// stream x number of data register
pub mod S4NDTR {
    pub use super::S0NDTR::NDT;
}

/// stream x peripheral address register
pub mod S4PAR {
    pub use super::S0PAR::PA;
}

/// stream x memory 0 address register
pub mod S4M0AR {
    pub use super::S0M0AR::M0A;
}

/// stream x memory 1 address register
pub mod S4M1AR {
    pub use super::S0M1AR::M1A;
}

/// stream x FIFO control register
pub mod S4FCR {
    pub use super::S0FCR::DMDIS;
    pub use super::S0FCR::FEIE;
    pub use super::S0FCR::FS;
    pub use super::S0FCR::FTH;
}

/// stream x configuration register
pub mod S5CR {
    pub use super::S0CR::CHSEL;
    pub use super::S0CR::CIRC;
    pub use super::S0CR::CT;
    pub use super::S0CR::DBM;
    pub use super::S0CR::DIR;
    pub use super::S0CR::DMEIE;
    pub use super::S0CR::EN;
    pub use super::S0CR::HTIE;
    pub use super::S0CR::MBURST;
    pub use super::S0CR::MINC;
    pub use super::S0CR::MSIZE;
    pub use super::S0CR::PBURST;
    pub use super::S0CR::PFCTRL;
    pub use super::S0CR::PINC;
    pub use super::S0CR::PINCOS;
    pub use super::S0CR::PL;
    pub use super::S0CR::PSIZE;
    pub use super::S0CR::TCIE;
    pub use super::S0CR::TEIE;
}

/// stream x number of data register
pub mod S5NDTR {
    pub use super::S0NDTR::NDT;
}

/// stream x peripheral address register
pub mod S5PAR {
    pub use super::S0PAR::PA;
}

/// stream x memory 0 address register
pub mod S5M0AR {
    pub use super::S0M0AR::M0A;
}

/// stream x memory 1 address register
pub mod S5M1AR {
    pub use super::S0M1AR::M1A;
}

/// stream x FIFO control register
pub mod S5FCR {
    pub use super::S0FCR::DMDIS;
    pub use super::S0FCR::FEIE;
    pub use super::S0FCR::FS;
    pub use super::S0FCR::FTH;
}

/// stream x configuration register
pub mod S6CR {
    pub use super::S0CR::CHSEL;
    pub use super::S0CR::CIRC;
    pub use super::S0CR::CT;
    pub use super::S0CR::DBM;
    pub use super::S0CR::DIR;
    pub use super::S0CR::DMEIE;
    pub use super::S0CR::EN;
    pub use super::S0CR::HTIE;
    pub use super::S0CR::MBURST;
    pub use super::S0CR::MINC;
    pub use super::S0CR::MSIZE;
    pub use super::S0CR::PBURST;
    pub use super::S0CR::PFCTRL;
    pub use super::S0CR::PINC;
    pub use super::S0CR::PINCOS;
    pub use super::S0CR::PL;
    pub use super::S0CR::PSIZE;
    pub use super::S0CR::TCIE;
    pub use super::S0CR::TEIE;
}

/// stream x number of data register
pub mod S6NDTR {
    pub use super::S0NDTR::NDT;
}

/// stream x peripheral address register
pub mod S6PAR {
    pub use super::S0PAR::PA;
}

/// stream x memory 0 address register
pub mod S6M0AR {
    pub use super::S0M0AR::M0A;
}

/// stream x memory 1 address register
pub mod S6M1AR {
    pub use super::S0M1AR::M1A;
}

/// stream x FIFO control register
pub mod S6FCR {
    pub use super::S0FCR::DMDIS;
    pub use super::S0FCR::FEIE;
    pub use super::S0FCR::FS;
    pub use super::S0FCR::FTH;
}

/// stream x configuration register
pub mod S7CR {
    pub use super::S0CR::CHSEL;
    pub use super::S0CR::CIRC;
    pub use super::S0CR::CT;
    pub use super::S0CR::DBM;
    pub use super::S0CR::DIR;
    pub use super::S0CR::DMEIE;
    pub use super::S0CR::EN;
    pub use super::S0CR::HTIE;
    pub use super::S0CR::MBURST;
    pub use super::S0CR::MINC;
    pub use super::S0CR::MSIZE;
    pub use super::S0CR::PBURST;
    pub use super::S0CR::PFCTRL;
    pub use super::S0CR::PINC;
    pub use super::S0CR::PINCOS;
    pub use super::S0CR::PL;
    pub use super::S0CR::PSIZE;
    pub use super::S0CR::TCIE;
    pub use super::S0CR::TEIE;
}

/// stream x number of data register
pub mod S7NDTR {
    pub use super::S0NDTR::NDT;
}

/// stream x peripheral address register
pub mod S7PAR {
    pub use super::S0PAR::PA;
}

/// stream x memory 0 address register
pub mod S7M0AR {
    pub use super::S0M0AR::M0A;
}

/// stream x memory 1 address register
pub mod S7M1AR {
    pub use super::S0M1AR::M1A;
}

/// stream x FIFO control register
pub mod S7FCR {
    pub use super::S0FCR::DMDIS;
    pub use super::S0FCR::FEIE;
    pub use super::S0FCR::FS;
    pub use super::S0FCR::FTH;
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
    pub S0CR: RWRegister<u32>,

    /// stream x number of data register
    pub S0NDTR: RWRegister<u32>,

    /// stream x peripheral address register
    pub S0PAR: UnsafeRWRegister<u32>,

    /// stream x memory 0 address register
    pub S0M0AR: UnsafeRWRegister<u32>,

    /// stream x memory 1 address register
    pub S0M1AR: UnsafeRWRegister<u32>,

    /// stream x FIFO control register
    pub S0FCR: RWRegister<u32>,

    /// stream x configuration register
    pub S1CR: RWRegister<u32>,

    /// stream x number of data register
    pub S1NDTR: RWRegister<u32>,

    /// stream x peripheral address register
    pub S1PAR: UnsafeRWRegister<u32>,

    /// stream x memory 0 address register
    pub S1M0AR: UnsafeRWRegister<u32>,

    /// stream x memory 1 address register
    pub S1M1AR: UnsafeRWRegister<u32>,

    /// stream x FIFO control register
    pub S1FCR: RWRegister<u32>,

    /// stream x configuration register
    pub S2CR: RWRegister<u32>,

    /// stream x number of data register
    pub S2NDTR: RWRegister<u32>,

    /// stream x peripheral address register
    pub S2PAR: UnsafeRWRegister<u32>,

    /// stream x memory 0 address register
    pub S2M0AR: UnsafeRWRegister<u32>,

    /// stream x memory 1 address register
    pub S2M1AR: UnsafeRWRegister<u32>,

    /// stream x FIFO control register
    pub S2FCR: RWRegister<u32>,

    /// stream x configuration register
    pub S3CR: RWRegister<u32>,

    /// stream x number of data register
    pub S3NDTR: RWRegister<u32>,

    /// stream x peripheral address register
    pub S3PAR: UnsafeRWRegister<u32>,

    /// stream x memory 0 address register
    pub S3M0AR: UnsafeRWRegister<u32>,

    /// stream x memory 1 address register
    pub S3M1AR: UnsafeRWRegister<u32>,

    /// stream x FIFO control register
    pub S3FCR: RWRegister<u32>,

    /// stream x configuration register
    pub S4CR: RWRegister<u32>,

    /// stream x number of data register
    pub S4NDTR: RWRegister<u32>,

    /// stream x peripheral address register
    pub S4PAR: UnsafeRWRegister<u32>,

    /// stream x memory 0 address register
    pub S4M0AR: UnsafeRWRegister<u32>,

    /// stream x memory 1 address register
    pub S4M1AR: UnsafeRWRegister<u32>,

    /// stream x FIFO control register
    pub S4FCR: RWRegister<u32>,

    /// stream x configuration register
    pub S5CR: RWRegister<u32>,

    /// stream x number of data register
    pub S5NDTR: RWRegister<u32>,

    /// stream x peripheral address register
    pub S5PAR: UnsafeRWRegister<u32>,

    /// stream x memory 0 address register
    pub S5M0AR: UnsafeRWRegister<u32>,

    /// stream x memory 1 address register
    pub S5M1AR: UnsafeRWRegister<u32>,

    /// stream x FIFO control register
    pub S5FCR: RWRegister<u32>,

    /// stream x configuration register
    pub S6CR: RWRegister<u32>,

    /// stream x number of data register
    pub S6NDTR: RWRegister<u32>,

    /// stream x peripheral address register
    pub S6PAR: UnsafeRWRegister<u32>,

    /// stream x memory 0 address register
    pub S6M0AR: UnsafeRWRegister<u32>,

    /// stream x memory 1 address register
    pub S6M1AR: UnsafeRWRegister<u32>,

    /// stream x FIFO control register
    pub S6FCR: RWRegister<u32>,

    /// stream x configuration register
    pub S7CR: RWRegister<u32>,

    /// stream x number of data register
    pub S7NDTR: RWRegister<u32>,

    /// stream x peripheral address register
    pub S7PAR: UnsafeRWRegister<u32>,

    /// stream x memory 0 address register
    pub S7M0AR: UnsafeRWRegister<u32>,

    /// stream x memory 1 address register
    pub S7M1AR: UnsafeRWRegister<u32>,

    /// stream x FIFO control register
    pub S7FCR: RWRegister<u32>,
}
pub struct ResetValues {
    pub LISR: u32,
    pub HISR: u32,
    pub LIFCR: u32,
    pub HIFCR: u32,
    pub S0CR: u32,
    pub S0NDTR: u32,
    pub S0PAR: u32,
    pub S0M0AR: u32,
    pub S0M1AR: u32,
    pub S0FCR: u32,
    pub S1CR: u32,
    pub S1NDTR: u32,
    pub S1PAR: u32,
    pub S1M0AR: u32,
    pub S1M1AR: u32,
    pub S1FCR: u32,
    pub S2CR: u32,
    pub S2NDTR: u32,
    pub S2PAR: u32,
    pub S2M0AR: u32,
    pub S2M1AR: u32,
    pub S2FCR: u32,
    pub S3CR: u32,
    pub S3NDTR: u32,
    pub S3PAR: u32,
    pub S3M0AR: u32,
    pub S3M1AR: u32,
    pub S3FCR: u32,
    pub S4CR: u32,
    pub S4NDTR: u32,
    pub S4PAR: u32,
    pub S4M0AR: u32,
    pub S4M1AR: u32,
    pub S4FCR: u32,
    pub S5CR: u32,
    pub S5NDTR: u32,
    pub S5PAR: u32,
    pub S5M0AR: u32,
    pub S5M1AR: u32,
    pub S5FCR: u32,
    pub S6CR: u32,
    pub S6NDTR: u32,
    pub S6PAR: u32,
    pub S6M0AR: u32,
    pub S6M1AR: u32,
    pub S6FCR: u32,
    pub S7CR: u32,
    pub S7NDTR: u32,
    pub S7PAR: u32,
    pub S7M0AR: u32,
    pub S7M1AR: u32,
    pub S7FCR: u32,
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
