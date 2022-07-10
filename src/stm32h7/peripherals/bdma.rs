#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! BDMA
//!
//! Used by: stm32h743, stm32h743v, stm32h747cm4, stm32h747cm7, stm32h753, stm32h753v, stm32h7b3

use crate::{RORegister, RWRegister, WORegister};
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
        /// Read-only values
        pub mod R {

            /// 0b0: No TE, HT or TC event on channel x
            pub const NoEvent: u32 = 0b0;

            /// 0b1: A TE, HT or TC event occurred on channel x
            pub const Event: u32 = 0b1;
        }
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
        /// Read-only values
        pub mod R {

            /// 0b0: No transfer complete event on channel x
            pub const NotComplete: u32 = 0b0;

            /// 0b1: A transfer complete event occurred on channel x
            pub const Complete: u32 = 0b1;
        }
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
        /// Read-only values
        pub mod R {

            /// 0b0: No half transfer event on channel x
            pub const NotHalf: u32 = 0b0;

            /// 0b1: A half transfer event occurred on channel x
            pub const Half: u32 = 0b1;
        }
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
        /// Read-only values
        pub mod R {

            /// 0b0: No transfer error on channel x
            pub const NoError: u32 = 0b0;

            /// 0b1: A transfer error occurred on channel x
            pub const Error: u32 = 0b1;
        }
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
        pub use super::GIF1::R;
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
        pub use super::TCIF1::R;
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
        pub use super::HTIF1::R;
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
        pub use super::TEIF1::R;
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
        pub use super::GIF1::R;
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
        pub use super::TCIF1::R;
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
        pub use super::HTIF1::R;
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
        pub use super::TEIF1::R;
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
        pub use super::GIF1::R;
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
        pub use super::TCIF1::R;
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
        pub use super::HTIF1::R;
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
        pub use super::TEIF1::R;
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
        pub use super::GIF1::R;
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
        pub use super::TCIF1::R;
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
        pub use super::HTIF1::R;
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
        pub use super::TEIF1::R;
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
        pub use super::GIF1::R;
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
        pub use super::TCIF1::R;
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
        pub use super::HTIF1::R;
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
        pub use super::TEIF1::R;
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
        pub use super::GIF1::R;
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
        pub use super::TCIF1::R;
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
        pub use super::HTIF1::R;
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
        pub use super::TEIF1::R;
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
        pub use super::GIF1::R;
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
        pub use super::TCIF1::R;
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
        pub use super::HTIF1::R;
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
        pub use super::TEIF1::R;
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
        /// Write-only values
        pub mod W {

            /// 0b1: Clear the corresponding CGIFx flag
            pub const Clear: u32 = 0b1;
        }
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
        /// Write-only values
        pub mod W {

            /// 0b1: Clear the corresponding TCIFx flag
            pub const Clear: u32 = 0b1;
        }
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
        /// Write-only values
        pub mod W {

            /// 0b1: Clear the corresponding HTIFx flag
            pub const Clear: u32 = 0b1;
        }
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
        /// Write-only values
        pub mod W {

            /// 0b1: Clear the corresponding TEIFx flag
            pub const Clear: u32 = 0b1;
        }
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
        pub use super::CGIF1::W;
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
        pub use super::CTCIF1::W;
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
        pub use super::CHTIF1::W;
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
        pub use super::CTEIF1::W;
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
        pub use super::CGIF1::W;
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
        pub use super::CTCIF1::W;
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
        pub use super::CHTIF1::W;
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
        pub use super::CTEIF1::W;
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
        pub use super::CGIF1::W;
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
        pub use super::CTCIF1::W;
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
        pub use super::CHTIF1::W;
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
        pub use super::CTEIF1::W;
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
        pub use super::CGIF1::W;
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
        pub use super::CTCIF1::W;
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
        pub use super::CHTIF1::W;
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
        pub use super::CTEIF1::W;
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
        pub use super::CGIF1::W;
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
        pub use super::CTCIF1::W;
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
        pub use super::CHTIF1::W;
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
        pub use super::CTEIF1::W;
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
        pub use super::CGIF1::W;
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
        pub use super::CTCIF1::W;
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
        pub use super::CHTIF1::W;
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
        pub use super::CTEIF1::W;
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
        pub use super::CGIF1::W;
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
        pub use super::CTCIF1::W;
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
        pub use super::CHTIF1::W;
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
        pub use super::CTEIF1::W;
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DMA channel x configuration register
pub mod CR0 {

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
        /// Read-write values
        pub mod RW {

            /// 0b0: Channel disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Channel enabled
            pub const Enabled: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: TC interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: TC interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: HT interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: HT interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: TE interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: TE interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: Peripheral-to-memory
            pub const PeripheralToMemory: u32 = 0b0;

            /// 0b1: Memory-to-peripheral
            pub const MemoryToPeripheral: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: Circular mode disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Circular mode enabled
            pub const Enabled: u32 = 0b1;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: Address pointer is fixed
            pub const Fixed: u32 = 0b0;

            /// 0b1: Address pointer is incremented after each data transfer
            pub const Incremented: u32 = 0b1;
        }
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
        pub use super::PINC::RW;
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
        /// Read-write values
        pub mod RW {

            /// 0b00: Byte (8-bit)
            pub const Bits8: u32 = 0b00;

            /// 0b01: Half-word (16-bit)
            pub const Bits16: u32 = 0b01;

            /// 0b10: Word (32-bit)
            pub const Bits32: u32 = 0b10;
        }
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
        pub use super::PSIZE::RW;
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
        /// Read-write values
        pub mod RW {

            /// 0b00: Low
            pub const Low: u32 = 0b00;

            /// 0b01: Medium
            pub const Medium: u32 = 0b01;

            /// 0b10: High
            pub const High: u32 = 0b10;

            /// 0b11: Very high
            pub const VeryHigh: u32 = 0b11;
        }
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
        /// Read-write values
        pub mod RW {

            /// 0b0: Memory-to-memory mode disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Memory-to-memory mode enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Current target memory in double-buffer mode
    pub mod CT {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: The current target memory is Memory 0
            pub const Memory0: u32 = 0b0;

            /// 0b1: The current target memory is Memory 1
            pub const Memory1: u32 = 0b1;
        }
    }

    /// Double-buffer mode
    pub mod DBM {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No buffer switching at the end of transfer
            pub const Disabled: u32 = 0b0;

            /// 0b1: Memory target switched at the end of the DMA transfer
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// DMA channel x number of data register
pub mod NDTR0 {

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
pub mod PAR0 {

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
pub mod M0AR0 {

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

/// Channel x memory 1 address register
pub mod M1AR0 {
    pub use super::M0AR0::MA;
}

/// DMA channel x configuration register
pub mod CR1 {
    pub use super::CR0::CIRC;
    pub use super::CR0::CT;
    pub use super::CR0::DBM;
    pub use super::CR0::DIR;
    pub use super::CR0::EN;
    pub use super::CR0::HTIE;
    pub use super::CR0::MEM2MEM;
    pub use super::CR0::MINC;
    pub use super::CR0::MSIZE;
    pub use super::CR0::PINC;
    pub use super::CR0::PL;
    pub use super::CR0::PSIZE;
    pub use super::CR0::TCIE;
    pub use super::CR0::TEIE;
}

/// DMA channel x number of data register
pub mod NDTR1 {
    pub use super::NDTR0::NDT;
}

/// This register must not be written when the channel is enabled.
pub mod PAR1 {
    pub use super::PAR0::PA;
}

/// This register must not be written when the channel is enabled.
pub mod M0AR1 {
    pub use super::M0AR0::MA;
}

/// Channel x memory 1 address register
pub mod M1AR1 {
    pub use super::M0AR0::MA;
}

/// DMA channel x configuration register
pub mod CR2 {
    pub use super::CR0::CIRC;
    pub use super::CR0::CT;
    pub use super::CR0::DBM;
    pub use super::CR0::DIR;
    pub use super::CR0::EN;
    pub use super::CR0::HTIE;
    pub use super::CR0::MEM2MEM;
    pub use super::CR0::MINC;
    pub use super::CR0::MSIZE;
    pub use super::CR0::PINC;
    pub use super::CR0::PL;
    pub use super::CR0::PSIZE;
    pub use super::CR0::TCIE;
    pub use super::CR0::TEIE;
}

/// DMA channel x number of data register
pub mod NDTR2 {
    pub use super::NDTR0::NDT;
}

/// This register must not be written when the channel is enabled.
pub mod PAR2 {
    pub use super::PAR0::PA;
}

/// This register must not be written when the channel is enabled.
pub mod M0AR2 {
    pub use super::M0AR0::MA;
}

/// Channel x memory 1 address register
pub mod M1AR2 {
    pub use super::M0AR0::MA;
}

/// DMA channel x configuration register
pub mod CR3 {
    pub use super::CR0::CIRC;
    pub use super::CR0::CT;
    pub use super::CR0::DBM;
    pub use super::CR0::DIR;
    pub use super::CR0::EN;
    pub use super::CR0::HTIE;
    pub use super::CR0::MEM2MEM;
    pub use super::CR0::MINC;
    pub use super::CR0::MSIZE;
    pub use super::CR0::PINC;
    pub use super::CR0::PL;
    pub use super::CR0::PSIZE;
    pub use super::CR0::TCIE;
    pub use super::CR0::TEIE;
}

/// DMA channel x number of data register
pub mod NDTR3 {
    pub use super::NDTR0::NDT;
}

/// This register must not be written when the channel is enabled.
pub mod PAR3 {
    pub use super::PAR0::PA;
}

/// This register must not be written when the channel is enabled.
pub mod M0AR3 {
    pub use super::M0AR0::MA;
}

/// Channel x memory 1 address register
pub mod M1AR3 {
    pub use super::M0AR0::MA;
}

/// DMA channel x configuration register
pub mod CR4 {
    pub use super::CR0::CIRC;
    pub use super::CR0::CT;
    pub use super::CR0::DBM;
    pub use super::CR0::DIR;
    pub use super::CR0::EN;
    pub use super::CR0::HTIE;
    pub use super::CR0::MEM2MEM;
    pub use super::CR0::MINC;
    pub use super::CR0::MSIZE;
    pub use super::CR0::PINC;
    pub use super::CR0::PL;
    pub use super::CR0::PSIZE;
    pub use super::CR0::TCIE;
    pub use super::CR0::TEIE;
}

/// DMA channel x number of data register
pub mod NDTR4 {
    pub use super::NDTR0::NDT;
}

/// This register must not be written when the channel is enabled.
pub mod PAR4 {
    pub use super::PAR0::PA;
}

/// This register must not be written when the channel is enabled.
pub mod M0AR4 {
    pub use super::M0AR0::MA;
}

/// Channel x memory 1 address register
pub mod M1AR4 {
    pub use super::M0AR0::MA;
}

/// DMA channel x configuration register
pub mod CR5 {
    pub use super::CR0::CIRC;
    pub use super::CR0::CT;
    pub use super::CR0::DBM;
    pub use super::CR0::DIR;
    pub use super::CR0::EN;
    pub use super::CR0::HTIE;
    pub use super::CR0::MEM2MEM;
    pub use super::CR0::MINC;
    pub use super::CR0::MSIZE;
    pub use super::CR0::PINC;
    pub use super::CR0::PL;
    pub use super::CR0::PSIZE;
    pub use super::CR0::TCIE;
    pub use super::CR0::TEIE;
}

/// DMA channel x number of data register
pub mod NDTR5 {
    pub use super::NDTR0::NDT;
}

/// This register must not be written when the channel is enabled.
pub mod PAR5 {
    pub use super::PAR0::PA;
}

/// This register must not be written when the channel is enabled.
pub mod M0AR5 {
    pub use super::M0AR0::MA;
}

/// Channel x memory 1 address register
pub mod M1AR5 {
    pub use super::M0AR0::MA;
}

/// DMA channel x configuration register
pub mod CR6 {
    pub use super::CR0::CIRC;
    pub use super::CR0::CT;
    pub use super::CR0::DBM;
    pub use super::CR0::DIR;
    pub use super::CR0::EN;
    pub use super::CR0::HTIE;
    pub use super::CR0::MEM2MEM;
    pub use super::CR0::MINC;
    pub use super::CR0::MSIZE;
    pub use super::CR0::PINC;
    pub use super::CR0::PL;
    pub use super::CR0::PSIZE;
    pub use super::CR0::TCIE;
    pub use super::CR0::TEIE;
}

/// DMA channel x number of data register
pub mod NDTR6 {
    pub use super::NDTR0::NDT;
}

/// This register must not be written when the channel is enabled.
pub mod PAR6 {
    pub use super::PAR0::PA;
}

/// This register must not be written when the channel is enabled.
pub mod M0AR6 {
    pub use super::M0AR0::MA;
}

/// Channel x memory 1 address register
pub mod M1AR6 {
    pub use super::M0AR0::MA;
}

/// DMA channel x configuration register
pub mod CR7 {
    pub use super::CR0::CIRC;
    pub use super::CR0::CT;
    pub use super::CR0::DBM;
    pub use super::CR0::DIR;
    pub use super::CR0::EN;
    pub use super::CR0::HTIE;
    pub use super::CR0::MEM2MEM;
    pub use super::CR0::MINC;
    pub use super::CR0::MSIZE;
    pub use super::CR0::PINC;
    pub use super::CR0::PL;
    pub use super::CR0::PSIZE;
    pub use super::CR0::TCIE;
    pub use super::CR0::TEIE;
}

/// DMA channel x number of data register
pub mod NDTR7 {
    pub use super::NDTR0::NDT;
}

/// This register must not be written when the channel is enabled.
pub mod PAR7 {
    pub use super::PAR0::PA;
}

/// This register must not be written when the channel is enabled.
pub mod M0AR7 {
    pub use super::M0AR0::MA;
}

/// Channel x memory 1 address register
pub mod M1AR7 {
    pub use super::M0AR0::MA;
}
#[repr(C)]
pub struct RegisterBlock {
    /// DMA interrupt status register
    pub ISR: RORegister<u32>,

    /// DMA interrupt flag clear register
    pub IFCR: WORegister<u32>,

    /// DMA channel x configuration register
    pub CR0: RWRegister<u32>,

    /// DMA channel x number of data register
    pub NDTR0: RWRegister<u32>,

    /// This register must not be written when the channel is enabled.
    pub PAR0: RWRegister<u32>,

    /// This register must not be written when the channel is enabled.
    pub M0AR0: RWRegister<u32>,

    /// Channel x memory 1 address register
    pub M1AR0: RWRegister<u32>,

    /// DMA channel x configuration register
    pub CR1: RWRegister<u32>,

    /// DMA channel x number of data register
    pub NDTR1: RWRegister<u32>,

    /// This register must not be written when the channel is enabled.
    pub PAR1: RWRegister<u32>,

    /// This register must not be written when the channel is enabled.
    pub M0AR1: RWRegister<u32>,

    /// Channel x memory 1 address register
    pub M1AR1: RWRegister<u32>,

    /// DMA channel x configuration register
    pub CR2: RWRegister<u32>,

    /// DMA channel x number of data register
    pub NDTR2: RWRegister<u32>,

    /// This register must not be written when the channel is enabled.
    pub PAR2: RWRegister<u32>,

    /// This register must not be written when the channel is enabled.
    pub M0AR2: RWRegister<u32>,

    /// Channel x memory 1 address register
    pub M1AR2: RWRegister<u32>,

    /// DMA channel x configuration register
    pub CR3: RWRegister<u32>,

    /// DMA channel x number of data register
    pub NDTR3: RWRegister<u32>,

    /// This register must not be written when the channel is enabled.
    pub PAR3: RWRegister<u32>,

    /// This register must not be written when the channel is enabled.
    pub M0AR3: RWRegister<u32>,

    /// Channel x memory 1 address register
    pub M1AR3: RWRegister<u32>,

    /// DMA channel x configuration register
    pub CR4: RWRegister<u32>,

    /// DMA channel x number of data register
    pub NDTR4: RWRegister<u32>,

    /// This register must not be written when the channel is enabled.
    pub PAR4: RWRegister<u32>,

    /// This register must not be written when the channel is enabled.
    pub M0AR4: RWRegister<u32>,

    /// Channel x memory 1 address register
    pub M1AR4: RWRegister<u32>,

    /// DMA channel x configuration register
    pub CR5: RWRegister<u32>,

    /// DMA channel x number of data register
    pub NDTR5: RWRegister<u32>,

    /// This register must not be written when the channel is enabled.
    pub PAR5: RWRegister<u32>,

    /// This register must not be written when the channel is enabled.
    pub M0AR5: RWRegister<u32>,

    /// Channel x memory 1 address register
    pub M1AR5: RWRegister<u32>,

    /// DMA channel x configuration register
    pub CR6: RWRegister<u32>,

    /// DMA channel x number of data register
    pub NDTR6: RWRegister<u32>,

    /// This register must not be written when the channel is enabled.
    pub PAR6: RWRegister<u32>,

    /// This register must not be written when the channel is enabled.
    pub M0AR6: RWRegister<u32>,

    /// Channel x memory 1 address register
    pub M1AR6: RWRegister<u32>,

    /// DMA channel x configuration register
    pub CR7: RWRegister<u32>,

    /// DMA channel x number of data register
    pub NDTR7: RWRegister<u32>,

    /// This register must not be written when the channel is enabled.
    pub PAR7: RWRegister<u32>,

    /// This register must not be written when the channel is enabled.
    pub M0AR7: RWRegister<u32>,

    /// Channel x memory 1 address register
    pub M1AR7: RWRegister<u32>,
}
pub struct ResetValues {
    pub ISR: u32,
    pub IFCR: u32,
    pub CR0: u32,
    pub NDTR0: u32,
    pub PAR0: u32,
    pub M0AR0: u32,
    pub M1AR0: u32,
    pub CR1: u32,
    pub NDTR1: u32,
    pub PAR1: u32,
    pub M0AR1: u32,
    pub M1AR1: u32,
    pub CR2: u32,
    pub NDTR2: u32,
    pub PAR2: u32,
    pub M0AR2: u32,
    pub M1AR2: u32,
    pub CR3: u32,
    pub NDTR3: u32,
    pub PAR3: u32,
    pub M0AR3: u32,
    pub M1AR3: u32,
    pub CR4: u32,
    pub NDTR4: u32,
    pub PAR4: u32,
    pub M0AR4: u32,
    pub M1AR4: u32,
    pub CR5: u32,
    pub NDTR5: u32,
    pub PAR5: u32,
    pub M0AR5: u32,
    pub M1AR5: u32,
    pub CR6: u32,
    pub NDTR6: u32,
    pub PAR6: u32,
    pub M0AR6: u32,
    pub M1AR6: u32,
    pub CR7: u32,
    pub NDTR7: u32,
    pub PAR7: u32,
    pub M0AR7: u32,
    pub M1AR7: u32,
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
