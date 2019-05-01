#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DMA controller
//!
//! Used by: stm32f0x0, stm32f0x1, stm32f0x2, stm32f0x8

#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;
use {RORegister, RWRegister, WORegister};

/// DMA interrupt status register (DMA_ISR)
pub mod ISR {

    /// Channel 1 Global interrupt flag
    pub mod GIF1 {
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

            /// 0b0: No transfer error, half event, complete event
            pub const NoEvent: u32 = 0b0;

            /// 0b1: A transfer error, half event or complete event has occured
            pub const Event: u32 = 0b1;
        }
    }

    /// Channel 1 Transfer Complete flag
    pub mod TCIF1 {
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

            /// 0b0: No transfer complete event
            pub const NotComplete: u32 = 0b0;

            /// 0b1: A transfer complete event has occured
            pub const Complete: u32 = 0b1;
        }
    }

    /// Channel 1 Half Transfer Complete flag
    pub mod HTIF1 {
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

            /// 0b0: No half transfer event
            pub const NotHalt: u32 = 0b0;

            /// 0b1: A half transfer event has occured
            pub const Half: u32 = 0b1;
        }
    }

    /// Channel 1 Transfer Error flag
    pub mod TEIF1 {
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

            /// 0b0: No transfer error
            pub const NoError: u32 = 0b0;

            /// 0b1: A transfer error has occured
            pub const Error: u32 = 0b1;
        }
    }

    /// Channel 2 Global interrupt flag
    pub mod GIF2 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GIF1::RW;
    }

    /// Channel 2 Transfer Complete flag
    pub mod TCIF2 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TCIF1::RW;
    }

    /// Channel 2 Half Transfer Complete flag
    pub mod HTIF2 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::HTIF1::RW;
    }

    /// Channel 2 Transfer Error flag
    pub mod TEIF2 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TEIF1::RW;
    }

    /// Channel 3 Global interrupt flag
    pub mod GIF3 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GIF1::RW;
    }

    /// Channel 3 Transfer Complete flag
    pub mod TCIF3 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TCIF1::RW;
    }

    /// Channel 3 Half Transfer Complete flag
    pub mod HTIF3 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::HTIF1::RW;
    }

    /// Channel 3 Transfer Error flag
    pub mod TEIF3 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TEIF1::RW;
    }

    /// Channel 4 Global interrupt flag
    pub mod GIF4 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GIF1::RW;
    }

    /// Channel 4 Transfer Complete flag
    pub mod TCIF4 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TCIF1::RW;
    }

    /// Channel 4 Half Transfer Complete flag
    pub mod HTIF4 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::HTIF1::RW;
    }

    /// Channel 4 Transfer Error flag
    pub mod TEIF4 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TEIF1::RW;
    }

    /// Channel 5 Global interrupt flag
    pub mod GIF5 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GIF1::RW;
    }

    /// Channel 5 Transfer Complete flag
    pub mod TCIF5 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TCIF1::RW;
    }

    /// Channel 5 Half Transfer Complete flag
    pub mod HTIF5 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::HTIF1::RW;
    }

    /// Channel 5 Transfer Error flag
    pub mod TEIF5 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TEIF1::RW;
    }

    /// Channel 6 Global interrupt flag
    pub mod GIF6 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GIF1::RW;
    }

    /// Channel 6 Transfer Complete flag
    pub mod TCIF6 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TCIF1::RW;
    }

    /// Channel 6 Half Transfer Complete flag
    pub mod HTIF6 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::HTIF1::RW;
    }

    /// Channel 6 Transfer Error flag
    pub mod TEIF6 {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TEIF1::RW;
    }

    /// Channel 7 Global interrupt flag
    pub mod GIF7 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GIF1::RW;
    }

    /// Channel 7 Transfer Complete flag
    pub mod TCIF7 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TCIF1::RW;
    }

    /// Channel 7 Half Transfer Complete flag
    pub mod HTIF7 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::HTIF1::RW;
    }

    /// Channel 7 Transfer Error flag
    pub mod TEIF7 {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TEIF1::RW;
    }
}

/// DMA interrupt flag clear register (DMA_IFCR)
pub mod IFCR {

    /// Channel 1 Global interrupt clear
    pub mod CGIF1 {
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

            /// 0b1: Clears the GIF, TEIF, HTIF, TCIF flags in the ISR register
            pub const Clear: u32 = 0b1;
        }
    }

    /// Channel 1 Transfer Complete clear
    pub mod CTCIF1 {
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

            /// 0b1: Clears the TCIF flag in the ISR register
            pub const Clear: u32 = 0b1;
        }
    }

    /// Channel 1 Half Transfer clear
    pub mod CHTIF1 {
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

            /// 0b1: Clears the HTIF flag in the ISR register
            pub const Clear: u32 = 0b1;
        }
    }

    /// Channel 1 Transfer Error clear
    pub mod CTEIF1 {
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

            /// 0b1: Clears the TEIF flag in the ISR register
            pub const Clear: u32 = 0b1;
        }
    }

    /// Channel 2 Global interrupt clear
    pub mod CGIF2 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CGIF1::RW;
    }

    /// Channel 2 Transfer Complete clear
    pub mod CTCIF2 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CTCIF1::RW;
    }

    /// Channel 2 Half Transfer clear
    pub mod CHTIF2 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CHTIF1::RW;
    }

    /// Channel 2 Transfer Error clear
    pub mod CTEIF2 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CTEIF1::RW;
    }

    /// Channel 3 Global interrupt clear
    pub mod CGIF3 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CGIF1::RW;
    }

    /// Channel 3 Transfer Complete clear
    pub mod CTCIF3 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CTCIF1::RW;
    }

    /// Channel 3 Half Transfer clear
    pub mod CHTIF3 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CHTIF1::RW;
    }

    /// Channel 3 Transfer Error clear
    pub mod CTEIF3 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CTEIF1::RW;
    }

    /// Channel 4 Global interrupt clear
    pub mod CGIF4 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CGIF1::RW;
    }

    /// Channel 4 Transfer Complete clear
    pub mod CTCIF4 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CTCIF1::RW;
    }

    /// Channel 4 Half Transfer clear
    pub mod CHTIF4 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CHTIF1::RW;
    }

    /// Channel 4 Transfer Error clear
    pub mod CTEIF4 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CTEIF1::RW;
    }

    /// Channel 5 Global interrupt clear
    pub mod CGIF5 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CGIF1::RW;
    }

    /// Channel 5 Transfer Complete clear
    pub mod CTCIF5 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CTCIF1::RW;
    }

    /// Channel 5 Half Transfer clear
    pub mod CHTIF5 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CHTIF1::RW;
    }

    /// Channel 5 Transfer Error clear
    pub mod CTEIF5 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CTEIF1::RW;
    }

    /// Channel 6 Global interrupt clear
    pub mod CGIF6 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CGIF1::RW;
    }

    /// Channel 6 Transfer Complete clear
    pub mod CTCIF6 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CTCIF1::RW;
    }

    /// Channel 6 Half Transfer clear
    pub mod CHTIF6 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CHTIF1::RW;
    }

    /// Channel 6 Transfer Error clear
    pub mod CTEIF6 {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CTEIF1::RW;
    }

    /// Channel 7 Global interrupt clear
    pub mod CGIF7 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CGIF1::RW;
    }

    /// Channel 7 Transfer Complete clear
    pub mod CTCIF7 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CTCIF1::RW;
    }

    /// Channel 7 Half Transfer clear
    pub mod CHTIF7 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CHTIF1::RW;
    }

    /// Channel 7 Transfer Error clear
    pub mod CTEIF7 {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CTEIF1::RW;
    }
}

/// DMA channel configuration register (DMA_CCR)
pub mod CR1 {

    /// Channel enable
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

    /// Transfer complete interrupt enable
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

            /// 0b0: Transfer Complete interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Transfer Complete interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Half Transfer interrupt enable
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

            /// 0b0: Half Transfer interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Half Transfer interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Transfer error interrupt enable
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

            /// 0b0: Transfer Error interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Transfer Error interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Data transfer direction
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

            /// 0b0: Read from peripheral
            pub const FromPeripheral: u32 = 0b0;

            /// 0b1: Read from memory
            pub const FromMemory: u32 = 0b1;
        }
    }

    /// Circular mode
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

            /// 0b0: Circular buffer disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Circular buffer enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Peripheral increment mode
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

            /// 0b0: Increment mode disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Increment mode enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Memory increment mode
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

    /// Peripheral size
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

            /// 0b00: 8-bit size
            pub const Bit8: u32 = 0b00;

            /// 0b01: 16-bit size
            pub const Bit16: u32 = 0b01;

            /// 0b10: 32-bit size
            pub const Bit32: u32 = 0b10;
        }
    }

    /// Memory size
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

    /// Channel Priority level
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

            /// 0b00: Low priority
            pub const Low: u32 = 0b00;

            /// 0b01: Medium priority
            pub const Medium: u32 = 0b01;

            /// 0b10: High priority
            pub const High: u32 = 0b10;

            /// 0b11: Very high priority
            pub const VeryHigh: u32 = 0b11;
        }
    }

    /// Memory to memory mode
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

            /// 0b0: Memory to memory mode disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Memory to memory mode enabled
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// DMA channel 1 number of data register
pub mod NDTR1 {

    /// Number of data to transfer
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

/// DMA channel 1 peripheral address register
pub mod PAR1 {

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

/// DMA channel 1 memory address register
pub mod MAR1 {

    /// Memory address
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

/// DMA channel configuration register (DMA_CCR)
pub mod CR2 {
    pub use super::CR1::CIRC;
    pub use super::CR1::DIR;
    pub use super::CR1::EN;
    pub use super::CR1::HTIE;
    pub use super::CR1::MEM2MEM;
    pub use super::CR1::MINC;
    pub use super::CR1::MSIZE;
    pub use super::CR1::PINC;
    pub use super::CR1::PL;
    pub use super::CR1::PSIZE;
    pub use super::CR1::TCIE;
    pub use super::CR1::TEIE;
}

/// DMA channel 1 number of data register
pub mod NDTR2 {
    pub use super::NDTR1::NDT;
}

/// DMA channel 1 peripheral address register
pub mod PAR2 {
    pub use super::PAR1::PA;
}

/// DMA channel 1 memory address register
pub mod MAR2 {
    pub use super::MAR1::MA;
}

/// DMA channel configuration register (DMA_CCR)
pub mod CR3 {
    pub use super::CR1::CIRC;
    pub use super::CR1::DIR;
    pub use super::CR1::EN;
    pub use super::CR1::HTIE;
    pub use super::CR1::MEM2MEM;
    pub use super::CR1::MINC;
    pub use super::CR1::MSIZE;
    pub use super::CR1::PINC;
    pub use super::CR1::PL;
    pub use super::CR1::PSIZE;
    pub use super::CR1::TCIE;
    pub use super::CR1::TEIE;
}

/// DMA channel 1 number of data register
pub mod NDTR3 {
    pub use super::NDTR1::NDT;
}

/// DMA channel 1 peripheral address register
pub mod PAR3 {
    pub use super::PAR1::PA;
}

/// DMA channel 1 memory address register
pub mod MAR3 {
    pub use super::MAR1::MA;
}

/// DMA channel configuration register (DMA_CCR)
pub mod CR4 {
    pub use super::CR1::CIRC;
    pub use super::CR1::DIR;
    pub use super::CR1::EN;
    pub use super::CR1::HTIE;
    pub use super::CR1::MEM2MEM;
    pub use super::CR1::MINC;
    pub use super::CR1::MSIZE;
    pub use super::CR1::PINC;
    pub use super::CR1::PL;
    pub use super::CR1::PSIZE;
    pub use super::CR1::TCIE;
    pub use super::CR1::TEIE;
}

/// DMA channel 1 number of data register
pub mod NDTR4 {
    pub use super::NDTR1::NDT;
}

/// DMA channel 1 peripheral address register
pub mod PAR4 {
    pub use super::PAR1::PA;
}

/// DMA channel 1 memory address register
pub mod MAR4 {
    pub use super::MAR1::MA;
}

/// DMA channel configuration register (DMA_CCR)
pub mod CR5 {
    pub use super::CR1::CIRC;
    pub use super::CR1::DIR;
    pub use super::CR1::EN;
    pub use super::CR1::HTIE;
    pub use super::CR1::MEM2MEM;
    pub use super::CR1::MINC;
    pub use super::CR1::MSIZE;
    pub use super::CR1::PINC;
    pub use super::CR1::PL;
    pub use super::CR1::PSIZE;
    pub use super::CR1::TCIE;
    pub use super::CR1::TEIE;
}

/// DMA channel 1 number of data register
pub mod NDTR5 {
    pub use super::NDTR1::NDT;
}

/// DMA channel 1 peripheral address register
pub mod PAR5 {
    pub use super::PAR1::PA;
}

/// DMA channel 1 memory address register
pub mod MAR5 {
    pub use super::MAR1::MA;
}

/// DMA channel configuration register (DMA_CCR)
pub mod CR6 {
    pub use super::CR1::CIRC;
    pub use super::CR1::DIR;
    pub use super::CR1::EN;
    pub use super::CR1::HTIE;
    pub use super::CR1::MEM2MEM;
    pub use super::CR1::MINC;
    pub use super::CR1::MSIZE;
    pub use super::CR1::PINC;
    pub use super::CR1::PL;
    pub use super::CR1::PSIZE;
    pub use super::CR1::TCIE;
    pub use super::CR1::TEIE;
}

/// DMA channel 1 number of data register
pub mod NDTR6 {
    pub use super::NDTR1::NDT;
}

/// DMA channel 1 peripheral address register
pub mod PAR6 {
    pub use super::PAR1::PA;
}

/// DMA channel 1 memory address register
pub mod MAR6 {
    pub use super::MAR1::MA;
}

/// DMA channel configuration register (DMA_CCR)
pub mod CR7 {
    pub use super::CR1::CIRC;
    pub use super::CR1::DIR;
    pub use super::CR1::EN;
    pub use super::CR1::HTIE;
    pub use super::CR1::MEM2MEM;
    pub use super::CR1::MINC;
    pub use super::CR1::MSIZE;
    pub use super::CR1::PINC;
    pub use super::CR1::PL;
    pub use super::CR1::PSIZE;
    pub use super::CR1::TCIE;
    pub use super::CR1::TEIE;
}

/// DMA channel 1 number of data register
pub mod NDTR7 {
    pub use super::NDTR1::NDT;
}

/// DMA channel 1 peripheral address register
pub mod PAR7 {
    pub use super::PAR1::PA;
}

/// DMA channel 1 memory address register
pub mod MAR7 {
    pub use super::MAR1::MA;
}
pub struct RegisterBlock {
    /// DMA interrupt status register (DMA_ISR)
    pub ISR: RORegister<u32>,

    /// DMA interrupt flag clear register (DMA_IFCR)
    pub IFCR: WORegister<u32>,

    /// DMA channel configuration register (DMA_CCR)
    pub CR1: RWRegister<u32>,

    /// DMA channel 1 number of data register
    pub NDTR1: RWRegister<u32>,

    /// DMA channel 1 peripheral address register
    pub PAR1: RWRegister<u32>,

    /// DMA channel 1 memory address register
    pub MAR1: RWRegister<u32>,

    _reserved1: [u32; 1],

    /// DMA channel configuration register (DMA_CCR)
    pub CR2: RWRegister<u32>,

    /// DMA channel 1 number of data register
    pub NDTR2: RWRegister<u32>,

    /// DMA channel 1 peripheral address register
    pub PAR2: RWRegister<u32>,

    /// DMA channel 1 memory address register
    pub MAR2: RWRegister<u32>,

    _reserved2: [u32; 1],

    /// DMA channel configuration register (DMA_CCR)
    pub CR3: RWRegister<u32>,

    /// DMA channel 1 number of data register
    pub NDTR3: RWRegister<u32>,

    /// DMA channel 1 peripheral address register
    pub PAR3: RWRegister<u32>,

    /// DMA channel 1 memory address register
    pub MAR3: RWRegister<u32>,

    _reserved3: [u32; 1],

    /// DMA channel configuration register (DMA_CCR)
    pub CR4: RWRegister<u32>,

    /// DMA channel 1 number of data register
    pub NDTR4: RWRegister<u32>,

    /// DMA channel 1 peripheral address register
    pub PAR4: RWRegister<u32>,

    /// DMA channel 1 memory address register
    pub MAR4: RWRegister<u32>,

    _reserved4: [u32; 1],

    /// DMA channel configuration register (DMA_CCR)
    pub CR5: RWRegister<u32>,

    /// DMA channel 1 number of data register
    pub NDTR5: RWRegister<u32>,

    /// DMA channel 1 peripheral address register
    pub PAR5: RWRegister<u32>,

    /// DMA channel 1 memory address register
    pub MAR5: RWRegister<u32>,

    _reserved5: [u32; 1],

    /// DMA channel configuration register (DMA_CCR)
    pub CR6: RWRegister<u32>,

    /// DMA channel 1 number of data register
    pub NDTR6: RWRegister<u32>,

    /// DMA channel 1 peripheral address register
    pub PAR6: RWRegister<u32>,

    /// DMA channel 1 memory address register
    pub MAR6: RWRegister<u32>,

    _reserved6: [u32; 1],

    /// DMA channel configuration register (DMA_CCR)
    pub CR7: RWRegister<u32>,

    /// DMA channel 1 number of data register
    pub NDTR7: RWRegister<u32>,

    /// DMA channel 1 peripheral address register
    pub PAR7: RWRegister<u32>,

    /// DMA channel 1 memory address register
    pub MAR7: RWRegister<u32>,
}
pub struct ResetValues {
    pub ISR: u32,
    pub IFCR: u32,
    pub CR1: u32,
    pub NDTR1: u32,
    pub PAR1: u32,
    pub MAR1: u32,
    pub CR2: u32,
    pub NDTR2: u32,
    pub PAR2: u32,
    pub MAR2: u32,
    pub CR3: u32,
    pub NDTR3: u32,
    pub PAR3: u32,
    pub MAR3: u32,
    pub CR4: u32,
    pub NDTR4: u32,
    pub PAR4: u32,
    pub MAR4: u32,
    pub CR5: u32,
    pub NDTR5: u32,
    pub PAR5: u32,
    pub MAR5: u32,
    pub CR6: u32,
    pub NDTR6: u32,
    pub PAR6: u32,
    pub MAR6: u32,
    pub CR7: u32,
    pub NDTR7: u32,
    pub PAR7: u32,
    pub MAR7: u32,
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
