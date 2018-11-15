#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! BDMA

#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;
use {RORegister, RWRegister, WORegister};

/// DMA interrupt status register
pub mod BDMA_ISR {

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
pub mod BDMA_IFCR {

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
pub mod BDMA_CCR1 {

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
pub mod BDMA_CNDTR1 {

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
pub mod BDMA_CPAR1 {

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
pub mod BDMA_CMAR1 {

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
pub mod BDMA_CCR2 {
    pub use super::BDMA_CCR1::CIRC;
    pub use super::BDMA_CCR1::DIR;
    pub use super::BDMA_CCR1::EN;
    pub use super::BDMA_CCR1::HTIE;
    pub use super::BDMA_CCR1::MEM2MEM;
    pub use super::BDMA_CCR1::MINC;
    pub use super::BDMA_CCR1::MSIZE;
    pub use super::BDMA_CCR1::PINC;
    pub use super::BDMA_CCR1::PL;
    pub use super::BDMA_CCR1::PSIZE;
    pub use super::BDMA_CCR1::TCIE;
    pub use super::BDMA_CCR1::TEIE;
}

/// DMA channel x number of data register
pub mod BDMA_CNDTR2 {
    pub use super::BDMA_CNDTR1::NDT;
}

/// This register must not be written when the channel is enabled.
pub mod BDMA_CPAR2 {
    pub use super::BDMA_CPAR1::PA;
}

/// This register must not be written when the channel is enabled.
pub mod BDMA_CMAR2 {
    pub use super::BDMA_CMAR1::MA;
}

/// DMA channel x configuration register
pub mod BDMA_CCR3 {
    pub use super::BDMA_CCR1::CIRC;
    pub use super::BDMA_CCR1::DIR;
    pub use super::BDMA_CCR1::EN;
    pub use super::BDMA_CCR1::HTIE;
    pub use super::BDMA_CCR1::MEM2MEM;
    pub use super::BDMA_CCR1::MINC;
    pub use super::BDMA_CCR1::MSIZE;
    pub use super::BDMA_CCR1::PINC;
    pub use super::BDMA_CCR1::PL;
    pub use super::BDMA_CCR1::PSIZE;
    pub use super::BDMA_CCR1::TCIE;
    pub use super::BDMA_CCR1::TEIE;
}

/// DMA channel x number of data register
pub mod BDMA_CNDTR3 {
    pub use super::BDMA_CNDTR1::NDT;
}

/// This register must not be written when the channel is enabled.
pub mod BDMA_CPAR3 {
    pub use super::BDMA_CPAR1::PA;
}

/// This register must not be written when the channel is enabled.
pub mod BDMA_CMAR3 {
    pub use super::BDMA_CMAR1::MA;
}

/// DMA channel x configuration register
pub mod BDMA_CCR4 {
    pub use super::BDMA_CCR1::CIRC;
    pub use super::BDMA_CCR1::DIR;
    pub use super::BDMA_CCR1::EN;
    pub use super::BDMA_CCR1::HTIE;
    pub use super::BDMA_CCR1::MEM2MEM;
    pub use super::BDMA_CCR1::MINC;
    pub use super::BDMA_CCR1::MSIZE;
    pub use super::BDMA_CCR1::PINC;
    pub use super::BDMA_CCR1::PL;
    pub use super::BDMA_CCR1::PSIZE;
    pub use super::BDMA_CCR1::TCIE;
    pub use super::BDMA_CCR1::TEIE;
}

/// DMA channel x number of data register
pub mod BDMA_CNDTR4 {
    pub use super::BDMA_CNDTR1::NDT;
}

/// This register must not be written when the channel is enabled.
pub mod BDMA_CPAR4 {
    pub use super::BDMA_CPAR1::PA;
}

/// This register must not be written when the channel is enabled.
pub mod BDMA_CMAR4 {
    pub use super::BDMA_CMAR1::MA;
}

/// DMA channel x configuration register
pub mod BDMA_CCR5 {
    pub use super::BDMA_CCR1::CIRC;
    pub use super::BDMA_CCR1::DIR;
    pub use super::BDMA_CCR1::EN;
    pub use super::BDMA_CCR1::HTIE;
    pub use super::BDMA_CCR1::MEM2MEM;
    pub use super::BDMA_CCR1::MINC;
    pub use super::BDMA_CCR1::MSIZE;
    pub use super::BDMA_CCR1::PINC;
    pub use super::BDMA_CCR1::PL;
    pub use super::BDMA_CCR1::PSIZE;
    pub use super::BDMA_CCR1::TCIE;
    pub use super::BDMA_CCR1::TEIE;
}

/// DMA channel x number of data register
pub mod BDMA_CNDTR5 {
    pub use super::BDMA_CNDTR1::NDT;
}

/// This register must not be written when the channel is enabled.
pub mod BDMA_CPAR5 {
    pub use super::BDMA_CPAR1::PA;
}

/// This register must not be written when the channel is enabled.
pub mod BDMA_CMAR5 {
    pub use super::BDMA_CMAR1::MA;
}

/// DMA channel x configuration register
pub mod BDMA_CCR6 {
    pub use super::BDMA_CCR1::CIRC;
    pub use super::BDMA_CCR1::DIR;
    pub use super::BDMA_CCR1::EN;
    pub use super::BDMA_CCR1::HTIE;
    pub use super::BDMA_CCR1::MEM2MEM;
    pub use super::BDMA_CCR1::MINC;
    pub use super::BDMA_CCR1::MSIZE;
    pub use super::BDMA_CCR1::PINC;
    pub use super::BDMA_CCR1::PL;
    pub use super::BDMA_CCR1::PSIZE;
    pub use super::BDMA_CCR1::TCIE;
    pub use super::BDMA_CCR1::TEIE;
}

/// DMA channel x number of data register
pub mod BDMA_CNDTR6 {
    pub use super::BDMA_CNDTR1::NDT;
}

/// This register must not be written when the channel is enabled.
pub mod BDMA_CPAR6 {
    pub use super::BDMA_CPAR1::PA;
}

/// This register must not be written when the channel is enabled.
pub mod BDMA_CMAR6 {
    pub use super::BDMA_CMAR1::MA;
}

/// DMA channel x configuration register
pub mod BDMA_CCR7 {
    pub use super::BDMA_CCR1::CIRC;
    pub use super::BDMA_CCR1::DIR;
    pub use super::BDMA_CCR1::EN;
    pub use super::BDMA_CCR1::HTIE;
    pub use super::BDMA_CCR1::MEM2MEM;
    pub use super::BDMA_CCR1::MINC;
    pub use super::BDMA_CCR1::MSIZE;
    pub use super::BDMA_CCR1::PINC;
    pub use super::BDMA_CCR1::PL;
    pub use super::BDMA_CCR1::PSIZE;
    pub use super::BDMA_CCR1::TCIE;
    pub use super::BDMA_CCR1::TEIE;
}

/// DMA channel x number of data register
pub mod BDMA_CNDTR7 {
    pub use super::BDMA_CNDTR1::NDT;
}

/// This register must not be written when the channel is enabled.
pub mod BDMA_CPAR7 {
    pub use super::BDMA_CPAR1::PA;
}

/// This register must not be written when the channel is enabled.
pub mod BDMA_CMAR7 {
    pub use super::BDMA_CMAR1::MA;
}

/// DMA channel x configuration register
pub mod BDMA_CCR8 {
    pub use super::BDMA_CCR1::CIRC;
    pub use super::BDMA_CCR1::DIR;
    pub use super::BDMA_CCR1::EN;
    pub use super::BDMA_CCR1::HTIE;
    pub use super::BDMA_CCR1::MEM2MEM;
    pub use super::BDMA_CCR1::MINC;
    pub use super::BDMA_CCR1::MSIZE;
    pub use super::BDMA_CCR1::PINC;
    pub use super::BDMA_CCR1::PL;
    pub use super::BDMA_CCR1::PSIZE;
    pub use super::BDMA_CCR1::TCIE;
    pub use super::BDMA_CCR1::TEIE;
}

/// DMA channel x number of data register
pub mod BDMA_CNDTR8 {
    pub use super::BDMA_CNDTR1::NDT;
}

/// This register must not be written when the channel is enabled.
pub mod BDMA_CPAR8 {
    pub use super::BDMA_CPAR1::PA;
}

/// This register must not be written when the channel is enabled.
pub mod BDMA_CMAR8 {
    pub use super::BDMA_CMAR1::MA;
}
pub struct RegisterBlock {
    /// DMA interrupt status register
    pub BDMA_ISR: RORegister<u32>,

    /// DMA interrupt flag clear register
    pub BDMA_IFCR: WORegister<u32>,

    /// DMA channel x configuration register
    pub BDMA_CCR1: RWRegister<u32>,

    /// DMA channel x number of data register
    pub BDMA_CNDTR1: RWRegister<u32>,

    /// This register must not be written when the channel is enabled.
    pub BDMA_CPAR1: RWRegister<u32>,

    /// This register must not be written when the channel is enabled.
    pub BDMA_CMAR1: RWRegister<u32>,

    _reserved1: [u32; 1],

    /// DMA channel x configuration register
    pub BDMA_CCR2: RWRegister<u32>,

    /// DMA channel x number of data register
    pub BDMA_CNDTR2: RWRegister<u32>,

    /// This register must not be written when the channel is enabled.
    pub BDMA_CPAR2: RWRegister<u32>,

    /// This register must not be written when the channel is enabled.
    pub BDMA_CMAR2: RWRegister<u32>,

    _reserved2: [u32; 1],

    /// DMA channel x configuration register
    pub BDMA_CCR3: RWRegister<u32>,

    /// DMA channel x number of data register
    pub BDMA_CNDTR3: RWRegister<u32>,

    /// This register must not be written when the channel is enabled.
    pub BDMA_CPAR3: RWRegister<u32>,

    /// This register must not be written when the channel is enabled.
    pub BDMA_CMAR3: RWRegister<u32>,

    _reserved3: [u32; 1],

    /// DMA channel x configuration register
    pub BDMA_CCR4: RWRegister<u32>,

    /// DMA channel x number of data register
    pub BDMA_CNDTR4: RWRegister<u32>,

    /// This register must not be written when the channel is enabled.
    pub BDMA_CPAR4: RWRegister<u32>,

    /// This register must not be written when the channel is enabled.
    pub BDMA_CMAR4: RWRegister<u32>,

    _reserved4: [u32; 1],

    /// DMA channel x configuration register
    pub BDMA_CCR5: RWRegister<u32>,

    /// DMA channel x number of data register
    pub BDMA_CNDTR5: RWRegister<u32>,

    /// This register must not be written when the channel is enabled.
    pub BDMA_CPAR5: RWRegister<u32>,

    /// This register must not be written when the channel is enabled.
    pub BDMA_CMAR5: RWRegister<u32>,

    _reserved5: [u32; 1],

    /// DMA channel x configuration register
    pub BDMA_CCR6: RWRegister<u32>,

    /// DMA channel x number of data register
    pub BDMA_CNDTR6: RWRegister<u32>,

    /// This register must not be written when the channel is enabled.
    pub BDMA_CPAR6: RWRegister<u32>,

    /// This register must not be written when the channel is enabled.
    pub BDMA_CMAR6: RWRegister<u32>,

    _reserved6: [u32; 1],

    /// DMA channel x configuration register
    pub BDMA_CCR7: RWRegister<u32>,

    /// DMA channel x number of data register
    pub BDMA_CNDTR7: RWRegister<u32>,

    /// This register must not be written when the channel is enabled.
    pub BDMA_CPAR7: RWRegister<u32>,

    /// This register must not be written when the channel is enabled.
    pub BDMA_CMAR7: RWRegister<u32>,

    _reserved7: [u32; 1],

    /// DMA channel x configuration register
    pub BDMA_CCR8: RWRegister<u32>,

    /// DMA channel x number of data register
    pub BDMA_CNDTR8: RWRegister<u32>,

    /// This register must not be written when the channel is enabled.
    pub BDMA_CPAR8: RWRegister<u32>,

    /// This register must not be written when the channel is enabled.
    pub BDMA_CMAR8: RWRegister<u32>,
}
pub struct ResetValues {
    pub BDMA_ISR: u32,
    pub BDMA_IFCR: u32,
    pub BDMA_CCR1: u32,
    pub BDMA_CNDTR1: u32,
    pub BDMA_CPAR1: u32,
    pub BDMA_CMAR1: u32,
    pub BDMA_CCR2: u32,
    pub BDMA_CNDTR2: u32,
    pub BDMA_CPAR2: u32,
    pub BDMA_CMAR2: u32,
    pub BDMA_CCR3: u32,
    pub BDMA_CNDTR3: u32,
    pub BDMA_CPAR3: u32,
    pub BDMA_CMAR3: u32,
    pub BDMA_CCR4: u32,
    pub BDMA_CNDTR4: u32,
    pub BDMA_CPAR4: u32,
    pub BDMA_CMAR4: u32,
    pub BDMA_CCR5: u32,
    pub BDMA_CNDTR5: u32,
    pub BDMA_CPAR5: u32,
    pub BDMA_CMAR5: u32,
    pub BDMA_CCR6: u32,
    pub BDMA_CNDTR6: u32,
    pub BDMA_CPAR6: u32,
    pub BDMA_CMAR6: u32,
    pub BDMA_CCR7: u32,
    pub BDMA_CNDTR7: u32,
    pub BDMA_CPAR7: u32,
    pub BDMA_CMAR7: u32,
    pub BDMA_CCR8: u32,
    pub BDMA_CNDTR8: u32,
    pub BDMA_CPAR8: u32,
    pub BDMA_CMAR8: u32,
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

/// Access functions for the BDMA peripheral instance
pub mod BDMA {
    #[cfg(not(feature = "nosync"))]
    use external_cortex_m;

    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x58025400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in BDMA
    pub const reset: ResetValues = ResetValues {
        BDMA_ISR: 0x00000000,
        BDMA_IFCR: 0x00000000,
        BDMA_CCR1: 0x00000000,
        BDMA_CNDTR1: 0x00000000,
        BDMA_CPAR1: 0x00000000,
        BDMA_CMAR1: 0x00000000,
        BDMA_CCR2: 0x00000000,
        BDMA_CNDTR2: 0x00000000,
        BDMA_CPAR2: 0x00000000,
        BDMA_CMAR2: 0x00000000,
        BDMA_CCR3: 0x00000000,
        BDMA_CNDTR3: 0x00000000,
        BDMA_CPAR3: 0x00000000,
        BDMA_CMAR3: 0x00000000,
        BDMA_CCR4: 0x00000000,
        BDMA_CNDTR4: 0x00000000,
        BDMA_CPAR4: 0x00000000,
        BDMA_CMAR4: 0x00000000,
        BDMA_CCR5: 0x00000000,
        BDMA_CNDTR5: 0x00000000,
        BDMA_CPAR5: 0x00000000,
        BDMA_CMAR5: 0x00000000,
        BDMA_CCR6: 0x00000000,
        BDMA_CNDTR6: 0x00000000,
        BDMA_CPAR6: 0x00000000,
        BDMA_CMAR6: 0x00000000,
        BDMA_CCR7: 0x00000000,
        BDMA_CNDTR7: 0x00000000,
        BDMA_CPAR7: 0x00000000,
        BDMA_CMAR7: 0x00000000,
        BDMA_CCR8: 0x00000000,
        BDMA_CNDTR8: 0x00000000,
        BDMA_CPAR8: 0x00000000,
        BDMA_CMAR8: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[no_mangle]
    static mut BDMA_TAKEN: bool = false;

    /// Safe access to BDMA
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
            if BDMA_TAKEN {
                None
            } else {
                BDMA_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to BDMA
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if BDMA_TAKEN && inst.addr == INSTANCE.addr {
                BDMA_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }
}

/// Raw pointer to BDMA
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const BDMA: *const RegisterBlock = 0x58025400 as *const _;
