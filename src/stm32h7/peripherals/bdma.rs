#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! BDMA
//!
//! Used by: stm32h743, stm32h743v, stm32h747cm4, stm32h747cm7, stm32h753, stm32h753v

use crate::{RORegister, RWRegister, UnsafeRWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// DMA interrupt status register
pub mod ISR {

    /// Channel x global interrupt flag (x = 1..8) This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register.
    pub mod GIF1 {
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

    /// Channel x transfer complete flag (x = 1..8) This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register.
    pub mod TCIF1 {
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

    /// Channel x half transfer flag (x = 1..8) This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register.
    pub mod HTIF1 {
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

    /// Channel x transfer error flag (x = 1..8) This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register.
    pub mod TEIF1 {
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

    /// Channel x global interrupt flag (x = 1..8) This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register.
    pub mod GIF2 {
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

    /// Channel x transfer complete flag (x = 1..8) This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register.
    pub mod TCIF2 {
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

    /// Channel x half transfer flag (x = 1..8) This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register.
    pub mod HTIF2 {
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

    /// Channel x transfer error flag (x = 1..8) This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register.
    pub mod TEIF2 {
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

    /// Channel x global interrupt flag (x = 1..8) This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register.
    pub mod GIF3 {
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

    /// Channel x transfer complete flag (x = 1..8) This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register.
    pub mod TCIF3 {
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

    /// Channel x half transfer flag (x = 1..8) This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register.
    pub mod HTIF3 {
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

    /// Channel x transfer error flag (x = 1..8) This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register.
    pub mod TEIF3 {
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

    /// Channel x global interrupt flag (x = 1..8) This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register.
    pub mod GIF4 {
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

    /// Channel x transfer complete flag (x = 1..8) This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register.
    pub mod TCIF4 {
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

    /// Channel x half transfer flag (x = 1..8) This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register.
    pub mod HTIF4 {
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

    /// Channel x transfer error flag (x = 1..8) This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register.
    pub mod TEIF4 {
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

    /// Channel x global interrupt flag (x = 1..8) This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register.
    pub mod GIF5 {
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

    /// Channel x transfer complete flag (x = 1..8) This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register.
    pub mod TCIF5 {
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

    /// Channel x half transfer flag (x = 1..8) This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register.
    pub mod HTIF5 {
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

    /// Channel x transfer error flag (x = 1..8) This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register.
    pub mod TEIF5 {
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

    /// Channel x global interrupt flag (x = 1..8) This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register.
    pub mod GIF6 {
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

    /// Channel x transfer complete flag (x = 1..8) This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register.
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

    /// Channel x half transfer flag (x = 1..8) This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register.
    pub mod HTIF6 {
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

    /// Channel x transfer error flag (x = 1..8) This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register.
    pub mod TEIF6 {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Channel x global interrupt flag (x = 1..8) This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register.
    pub mod GIF7 {
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

    /// Channel x transfer complete flag (x = 1..8) This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register.
    pub mod TCIF7 {
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

    /// Channel x half transfer flag (x = 1..8) This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register.
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

    /// Channel x transfer error flag (x = 1..8) This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register.
    pub mod TEIF7 {
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

    /// Channel x global interrupt flag (x = 1..8) This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register.
    pub mod GIF8 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Channel x transfer complete flag (x = 1..8) This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register.
    pub mod TCIF8 {
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

    /// Channel x half transfer flag (x = 1..8) This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register.
    pub mod HTIF8 {
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

    /// Channel x transfer error flag (x = 1..8) This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCR register.
    pub mod TEIF8 {
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

/// DMA interrupt flag clear register
pub mod IFCR {

    /// Channel x global interrupt clear This bit is set and cleared by software.
    pub mod CGIF1 {
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

    /// Channel x transfer complete clear This bit is set and cleared by software.
    pub mod CTCIF1 {
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

    /// Channel x half transfer clear This bit is set and cleared by software.
    pub mod CHTIF1 {
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

    /// Channel x transfer error clear This bit is set and cleared by software.
    pub mod CTEIF1 {
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

    /// Channel x global interrupt clear This bit is set and cleared by software.
    pub mod CGIF2 {
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

    /// Channel x transfer complete clear This bit is set and cleared by software.
    pub mod CTCIF2 {
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

    /// Channel x half transfer clear This bit is set and cleared by software.
    pub mod CHTIF2 {
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

    /// Channel x transfer error clear This bit is set and cleared by software.
    pub mod CTEIF2 {
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

    /// Channel x global interrupt clear This bit is set and cleared by software.
    pub mod CGIF3 {
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

    /// Channel x transfer complete clear This bit is set and cleared by software.
    pub mod CTCIF3 {
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

    /// Channel x half transfer clear This bit is set and cleared by software.
    pub mod CHTIF3 {
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

    /// Channel x transfer error clear This bit is set and cleared by software.
    pub mod CTEIF3 {
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

    /// Channel x global interrupt clear This bit is set and cleared by software.
    pub mod CGIF4 {
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

    /// Channel x transfer complete clear This bit is set and cleared by software.
    pub mod CTCIF4 {
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

    /// Channel x half transfer clear This bit is set and cleared by software.
    pub mod CHTIF4 {
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

    /// Channel x transfer error clear This bit is set and cleared by software.
    pub mod CTEIF4 {
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

    /// Channel x global interrupt clear This bit is set and cleared by software.
    pub mod CGIF5 {
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

    /// Channel x transfer complete clear This bit is set and cleared by software.
    pub mod CTCIF5 {
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

    /// Channel x half transfer clear This bit is set and cleared by software.
    pub mod CHTIF5 {
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

    /// Channel x transfer error clear This bit is set and cleared by software.
    pub mod CTEIF5 {
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

    /// Channel x global interrupt clear This bit is set and cleared by software.
    pub mod CGIF6 {
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

    /// Channel x transfer complete clear This bit is set and cleared by software.
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

    /// Channel x half transfer clear This bit is set and cleared by software.
    pub mod CHTIF6 {
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

    /// Channel x transfer error clear This bit is set and cleared by software.
    pub mod CTEIF6 {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Channel x global interrupt clear This bit is set and cleared by software.
    pub mod CGIF7 {
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

    /// Channel x transfer complete clear This bit is set and cleared by software.
    pub mod CTCIF7 {
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

    /// Channel x half transfer clear This bit is set and cleared by software.
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

    /// Channel x transfer error clear This bit is set and cleared by software.
    pub mod CTEIF7 {
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

    /// Channel x global interrupt clear This bit is set and cleared by software.
    pub mod CGIF8 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Channel x transfer complete clear This bit is set and cleared by software.
    pub mod CTCIF8 {
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

    /// Channel x half transfer clear This bit is set and cleared by software.
    pub mod CHTIF8 {
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

    /// Channel x transfer error clear This bit is set and cleared by software.
    pub mod CTEIF8 {
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

/// DMA channel x configuration register
pub mod CCR1 {

    /// Channel enable This bit is set and cleared by software.
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

    /// Transfer complete interrupt enable This bit is set and cleared by software.
    pub mod TCIE {
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

    /// Half transfer interrupt enable This bit is set and cleared by software.
    pub mod HTIE {
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

    /// Transfer error interrupt enable This bit is set and cleared by software.
    pub mod TEIE {
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

    /// Data transfer direction This bit is set and cleared by software.
    pub mod DIR {
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

    /// Circular mode This bit is set and cleared by software.
    pub mod CIRC {
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

    /// Peripheral increment mode This bit is set and cleared by software.
    pub mod PINC {
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

    /// Memory increment mode This bit is set and cleared by software.
    pub mod MINC {
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

    /// Peripheral size These bits are set and cleared by software.
    pub mod PSIZE {
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

    /// Memory size These bits are set and cleared by software.
    pub mod MSIZE {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (2 bits: 0b11 << 10)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Channel priority level These bits are set and cleared by software.
    pub mod PL {
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

    /// Memory to memory mode This bit is set and cleared by software.
    pub mod MEM2MEM {
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
}

/// DMA channel x number of data register
pub mod CNDTR1 {

    /// Number of data to transfer Number of data to be transferred (0 up to 65535). This register can only be written when the channel is disabled. Once the channel is enabled, this register is read-only, indicating the remaining bytes to be transmitted. This register decrements after each DMA transfer. Once the transfer is completed, this register can either stay at zero or be reloaded automatically by the value previously programmed if the channel is configured in auto-reload mode. If this register is zero, no transaction can be served whether the channel is enabled or not.
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

/// This register must not be written when the channel is enabled.
pub mod CPAR1 {

    /// Peripheral address Base address of the peripheral data register from/to which the data will be read/written. When PSIZE is 01 (16-bit), the PA\[0\] bit is ignored. Access is automatically aligned to a half-word address. When PSIZE is 10 (32-bit), PA\[1:0\] are ignored. Access is automatically aligned to a word address.
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

/// This register must not be written when the channel is enabled.
pub mod CMAR1 {

    /// Memory address Base address of the memory area from/to which the data will be read/written. When MSIZE is 01 (16-bit), the MA\[0\] bit is ignored. Access is automatically aligned to a half-word address. When MSIZE is 10 (32-bit), MA\[1:0\] are ignored. Access is automatically aligned to a word address.
    pub mod MA {
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

/// DMA channel x configuration register
pub mod CCR2 {
    pub use super::CCR1::CIRC;
    pub use super::CCR1::DIR;
    pub use super::CCR1::EN;
    pub use super::CCR1::HTIE;
    pub use super::CCR1::MEM2MEM;
    pub use super::CCR1::MINC;
    pub use super::CCR1::MSIZE;
    pub use super::CCR1::PINC;
    pub use super::CCR1::PL;
    pub use super::CCR1::PSIZE;
    pub use super::CCR1::TCIE;
    pub use super::CCR1::TEIE;
}

/// DMA channel x number of data register
pub mod CNDTR2 {
    pub use super::CNDTR1::NDT;
}

/// This register must not be written when the channel is enabled.
pub mod CPAR2 {
    pub use super::CPAR1::PA;
}

/// This register must not be written when the channel is enabled.
pub mod CMAR2 {
    pub use super::CMAR1::MA;
}

/// DMA channel x configuration register
pub mod CCR3 {
    pub use super::CCR1::CIRC;
    pub use super::CCR1::DIR;
    pub use super::CCR1::EN;
    pub use super::CCR1::HTIE;
    pub use super::CCR1::MEM2MEM;
    pub use super::CCR1::MINC;
    pub use super::CCR1::MSIZE;
    pub use super::CCR1::PINC;
    pub use super::CCR1::PL;
    pub use super::CCR1::PSIZE;
    pub use super::CCR1::TCIE;
    pub use super::CCR1::TEIE;
}

/// DMA channel x number of data register
pub mod CNDTR3 {
    pub use super::CNDTR1::NDT;
}

/// This register must not be written when the channel is enabled.
pub mod CPAR3 {
    pub use super::CPAR1::PA;
}

/// This register must not be written when the channel is enabled.
pub mod CMAR3 {
    pub use super::CMAR1::MA;
}

/// DMA channel x configuration register
pub mod CCR4 {
    pub use super::CCR1::CIRC;
    pub use super::CCR1::DIR;
    pub use super::CCR1::EN;
    pub use super::CCR1::HTIE;
    pub use super::CCR1::MEM2MEM;
    pub use super::CCR1::MINC;
    pub use super::CCR1::MSIZE;
    pub use super::CCR1::PINC;
    pub use super::CCR1::PL;
    pub use super::CCR1::PSIZE;
    pub use super::CCR1::TCIE;
    pub use super::CCR1::TEIE;
}

/// DMA channel x number of data register
pub mod CNDTR4 {
    pub use super::CNDTR1::NDT;
}

/// This register must not be written when the channel is enabled.
pub mod CPAR4 {
    pub use super::CPAR1::PA;
}

/// This register must not be written when the channel is enabled.
pub mod CMAR4 {
    pub use super::CMAR1::MA;
}

/// DMA channel x configuration register
pub mod CCR5 {
    pub use super::CCR1::CIRC;
    pub use super::CCR1::DIR;
    pub use super::CCR1::EN;
    pub use super::CCR1::HTIE;
    pub use super::CCR1::MEM2MEM;
    pub use super::CCR1::MINC;
    pub use super::CCR1::MSIZE;
    pub use super::CCR1::PINC;
    pub use super::CCR1::PL;
    pub use super::CCR1::PSIZE;
    pub use super::CCR1::TCIE;
    pub use super::CCR1::TEIE;
}

/// DMA channel x number of data register
pub mod CNDTR5 {
    pub use super::CNDTR1::NDT;
}

/// This register must not be written when the channel is enabled.
pub mod CPAR5 {
    pub use super::CPAR1::PA;
}

/// This register must not be written when the channel is enabled.
pub mod CMAR5 {
    pub use super::CMAR1::MA;
}

/// DMA channel x configuration register
pub mod CCR6 {
    pub use super::CCR1::CIRC;
    pub use super::CCR1::DIR;
    pub use super::CCR1::EN;
    pub use super::CCR1::HTIE;
    pub use super::CCR1::MEM2MEM;
    pub use super::CCR1::MINC;
    pub use super::CCR1::MSIZE;
    pub use super::CCR1::PINC;
    pub use super::CCR1::PL;
    pub use super::CCR1::PSIZE;
    pub use super::CCR1::TCIE;
    pub use super::CCR1::TEIE;
}

/// DMA channel x number of data register
pub mod CNDTR6 {
    pub use super::CNDTR1::NDT;
}

/// This register must not be written when the channel is enabled.
pub mod CPAR6 {
    pub use super::CPAR1::PA;
}

/// This register must not be written when the channel is enabled.
pub mod CMAR6 {
    pub use super::CMAR1::MA;
}

/// DMA channel x configuration register
pub mod CCR7 {
    pub use super::CCR1::CIRC;
    pub use super::CCR1::DIR;
    pub use super::CCR1::EN;
    pub use super::CCR1::HTIE;
    pub use super::CCR1::MEM2MEM;
    pub use super::CCR1::MINC;
    pub use super::CCR1::MSIZE;
    pub use super::CCR1::PINC;
    pub use super::CCR1::PL;
    pub use super::CCR1::PSIZE;
    pub use super::CCR1::TCIE;
    pub use super::CCR1::TEIE;
}

/// DMA channel x number of data register
pub mod CNDTR7 {
    pub use super::CNDTR1::NDT;
}

/// This register must not be written when the channel is enabled.
pub mod CPAR7 {
    pub use super::CPAR1::PA;
}

/// This register must not be written when the channel is enabled.
pub mod CMAR7 {
    pub use super::CMAR1::MA;
}

/// DMA channel x configuration register
pub mod CCR8 {
    pub use super::CCR1::CIRC;
    pub use super::CCR1::DIR;
    pub use super::CCR1::EN;
    pub use super::CCR1::HTIE;
    pub use super::CCR1::MEM2MEM;
    pub use super::CCR1::MINC;
    pub use super::CCR1::MSIZE;
    pub use super::CCR1::PINC;
    pub use super::CCR1::PL;
    pub use super::CCR1::PSIZE;
    pub use super::CCR1::TCIE;
    pub use super::CCR1::TEIE;
}

/// DMA channel x number of data register
pub mod CNDTR8 {
    pub use super::CNDTR1::NDT;
}

/// This register must not be written when the channel is enabled.
pub mod CPAR8 {
    pub use super::CPAR1::PA;
}

/// This register must not be written when the channel is enabled.
pub mod CMAR8 {
    pub use super::CMAR1::MA;
}
#[repr(C)]
pub struct RegisterBlock {
    /// DMA interrupt status register
    pub ISR: RORegister<u32>,

    /// DMA interrupt flag clear register
    pub IFCR: WORegister<u32>,

    /// DMA channel x configuration register
    pub CCR1: RWRegister<u32>,

    /// DMA channel x number of data register
    pub CNDTR1: RWRegister<u32>,

    /// This register must not be written when the channel is enabled.
    pub CPAR1: UnsafeRWRegister<u32>,

    /// This register must not be written when the channel is enabled.
    pub CMAR1: UnsafeRWRegister<u32>,

    _reserved1: [u32; 1],

    /// DMA channel x configuration register
    pub CCR2: RWRegister<u32>,

    /// DMA channel x number of data register
    pub CNDTR2: RWRegister<u32>,

    /// This register must not be written when the channel is enabled.
    pub CPAR2: UnsafeRWRegister<u32>,

    /// This register must not be written when the channel is enabled.
    pub CMAR2: UnsafeRWRegister<u32>,

    _reserved2: [u32; 1],

    /// DMA channel x configuration register
    pub CCR3: RWRegister<u32>,

    /// DMA channel x number of data register
    pub CNDTR3: RWRegister<u32>,

    /// This register must not be written when the channel is enabled.
    pub CPAR3: UnsafeRWRegister<u32>,

    /// This register must not be written when the channel is enabled.
    pub CMAR3: UnsafeRWRegister<u32>,

    _reserved3: [u32; 1],

    /// DMA channel x configuration register
    pub CCR4: RWRegister<u32>,

    /// DMA channel x number of data register
    pub CNDTR4: RWRegister<u32>,

    /// This register must not be written when the channel is enabled.
    pub CPAR4: UnsafeRWRegister<u32>,

    /// This register must not be written when the channel is enabled.
    pub CMAR4: UnsafeRWRegister<u32>,

    _reserved4: [u32; 1],

    /// DMA channel x configuration register
    pub CCR5: RWRegister<u32>,

    /// DMA channel x number of data register
    pub CNDTR5: RWRegister<u32>,

    /// This register must not be written when the channel is enabled.
    pub CPAR5: UnsafeRWRegister<u32>,

    /// This register must not be written when the channel is enabled.
    pub CMAR5: UnsafeRWRegister<u32>,

    _reserved5: [u32; 1],

    /// DMA channel x configuration register
    pub CCR6: RWRegister<u32>,

    /// DMA channel x number of data register
    pub CNDTR6: RWRegister<u32>,

    /// This register must not be written when the channel is enabled.
    pub CPAR6: UnsafeRWRegister<u32>,

    /// This register must not be written when the channel is enabled.
    pub CMAR6: UnsafeRWRegister<u32>,

    _reserved6: [u32; 1],

    /// DMA channel x configuration register
    pub CCR7: RWRegister<u32>,

    /// DMA channel x number of data register
    pub CNDTR7: RWRegister<u32>,

    /// This register must not be written when the channel is enabled.
    pub CPAR7: UnsafeRWRegister<u32>,

    /// This register must not be written when the channel is enabled.
    pub CMAR7: UnsafeRWRegister<u32>,

    _reserved7: [u32; 1],

    /// DMA channel x configuration register
    pub CCR8: RWRegister<u32>,

    /// DMA channel x number of data register
    pub CNDTR8: RWRegister<u32>,

    /// This register must not be written when the channel is enabled.
    pub CPAR8: UnsafeRWRegister<u32>,

    /// This register must not be written when the channel is enabled.
    pub CMAR8: UnsafeRWRegister<u32>,
}
pub struct ResetValues {
    pub ISR: u32,
    pub IFCR: u32,
    pub CCR1: u32,
    pub CNDTR1: u32,
    pub CPAR1: u32,
    pub CMAR1: u32,
    pub CCR2: u32,
    pub CNDTR2: u32,
    pub CPAR2: u32,
    pub CMAR2: u32,
    pub CCR3: u32,
    pub CNDTR3: u32,
    pub CPAR3: u32,
    pub CMAR3: u32,
    pub CCR4: u32,
    pub CNDTR4: u32,
    pub CPAR4: u32,
    pub CMAR4: u32,
    pub CCR5: u32,
    pub CNDTR5: u32,
    pub CPAR5: u32,
    pub CMAR5: u32,
    pub CCR6: u32,
    pub CNDTR6: u32,
    pub CPAR6: u32,
    pub CMAR6: u32,
    pub CCR7: u32,
    pub CNDTR7: u32,
    pub CPAR7: u32,
    pub CMAR7: u32,
    pub CCR8: u32,
    pub CNDTR8: u32,
    pub CPAR8: u32,
    pub CMAR8: u32,
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
